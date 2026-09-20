use super::*;
use std::error::Error;

fn instance() -> Result<WorkerInstanceId, Box<dyn Error>> {
    Ok("018f47f7-5a86-7c00-8000-000000000501".parse()?)
}

fn expected() -> Result<ExpectedPeer, PeerCredentialError> {
    #[cfg(unix)]
    {
        ExpectedPeer::unix_process(123, 1000, 1000)
    }
    #[cfg(windows)]
    {
        ExpectedPeer::windows_process(123)
    }
}

fn fixture() -> Result<(WorkerHello, WorkerVerifier), Box<dyn Error>> {
    let token = BootstrapToken([171; 32]);
    let verifier = UnboundWorkerVerifier {
        launch: WorkerLaunch {
            instance_id: instance()?,
            role: WorkerRole::BrowserWorker,
            bootstrap_token: token.clone(),
        },
    }
    .bind(expected()?);
    Ok((
        WorkerHello::new(instance()?, WorkerRole::BrowserWorker, token),
        verifier,
    ))
}

#[test]
fn debug_output_never_contains_bootstrap_secret() -> Result<(), Box<dyn Error>> {
    let (hello, verifier) = fixture()?;
    assert_eq!(
        format!("{:?}", hello.bootstrap_token),
        "BootstrapToken([REDACTED])"
    );
    assert!(!format!("{verifier:?} {hello:?}").contains("171"));
    Ok(())
}

#[test]
fn matching_peer_hello_failures_consume_the_verifier() -> Result<(), Box<dyn Error>> {
    for field in ["schema", "token", "instance", "role"] {
        let (valid, mut verifier) = fixture()?;
        let mut invalid = valid.clone();
        match field {
            "schema" => invalid.schema_version = SchemaVersion::try_new(2, 0)?,
            "token" => invalid.bootstrap_token = BootstrapToken([172; 32]),
            "instance" => invalid.instance_id = "018f47f7-5a86-7c00-8000-000000000502".parse()?,
            "role" => invalid.role = WorkerRole::PolicyBroker,
            _ => return Err("unknown test field".into()),
        }
        assert_eq!(
            verifier.consume_hello(&invalid),
            Err(if field == "schema" {
                AuthenticationError::UnsupportedSchema
            } else {
                AuthenticationError::LaunchIdentityMismatch
            })
        );
        assert_eq!(
            verifier.consume_hello(&valid),
            Err(AuthenticationError::AlreadyConsumed)
        );
    }
    Ok(())
}

#[test]
fn successful_authentication_binds_role_and_instance_once() -> Result<(), Box<dyn Error>> {
    let (hello, mut verifier) = fixture()?;
    let identity = verifier.consume_hello(&hello)?;
    assert_eq!(identity.instance_id(), instance()?);
    assert_eq!(identity.role(), WorkerRole::BrowserWorker);
    assert!(identity.allows(MessageFamily::BrowserObservation));
    assert!(!identity.allows(MessageFamily::PolicyDecision));
    assert_eq!(
        verifier.consume_hello(&hello),
        Err(AuthenticationError::AlreadyConsumed)
    );
    Ok(())
}

#[test]
fn wire_hello_keeps_the_existing_v1_representation() -> Result<(), Box<dyn Error>> {
    let golden = r#"{"schema_version":{"major":1,"minor":0},"instance_id":"018f47f7-5a86-7c00-8000-000000000501","role":"browser_worker","bootstrap_token":[171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171,171]}"#;
    let (hello, _) = fixture()?;
    assert_eq!(serde_json::to_string(&hello)?, golden);
    assert_eq!(serde_json::from_str::<WorkerHello>(golden)?, hello);
    Ok(())
}

#[cfg(unix)]
#[test]
fn connected_peer_check_preserves_one_use_authentication() -> Result<(), Box<dyn Error>> {
    let (socket, _other) = std::os::unix::net::UnixStream::pair()?;
    let (token, pending) = issue_worker_authentication(instance()?, WorkerRole::BrowserWorker)
        .map_err(|error| format!("bootstrap entropy: {error}"))?;
    let mut verifier = pending.bind(ExpectedPeer::unix_process(
        std::process::id(),
        nix::unistd::geteuid().as_raw(),
        nix::unistd::getegid().as_raw(),
    )?);
    verifier.check_unix_peer(&socket)?;
    verifier.check_unix_peer(&socket)?;
    let hello = WorkerHello::new(instance()?, WorkerRole::BrowserWorker, token);
    assert_eq!(
        verifier.authenticate_unix(&hello, &socket)?.instance_id(),
        instance()?
    );
    assert_eq!(
        verifier.authenticate_unix(&hello, &socket),
        Err(AuthenticationError::AlreadyConsumed)
    );
    assert_eq!(
        verifier.check_unix_peer(&socket),
        Err(AuthenticationError::AlreadyConsumed)
    );
    Ok(())
}

#[test]
fn rejected_observation_leaves_the_issued_credential_available() -> Result<(), Box<dyn Error>> {
    let (hello, mut verifier) = fixture()?;
    let wrong = {
        #[cfg(unix)]
        {
            ObservedPeer::Unix {
                pid: 124,
                uid: 1000,
                gid: 1000,
            }
        }
        #[cfg(windows)]
        {
            ObservedPeer::Windows { process_id: 124 }
        }
    };
    assert_eq!(
        verifier.check_observed(Ok(wrong)),
        Err(AuthenticationError::PeerCredentialMismatch)
    );
    assert_eq!(
        verifier.check_observed(Err(PeerCredentialError::Os(5))),
        Err(AuthenticationError::PeerObservation(
            PeerCredentialError::Os(5)
        ))
    );
    assert_eq!(verifier.consume_hello(&hello)?.instance_id(), instance()?);
    Ok(())
}
