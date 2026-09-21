use super::*;
use std::error::Error;

thread_local! {
    static DISPOSALS: std::cell::RefCell<Vec<bool>> = const { std::cell::RefCell::new(Vec::new()) };
}

pub(super) fn observe_disposal(bytes: &[u8]) {
    DISPOSALS.with(|events| {
        events
            .borrow_mut()
            .push(bytes.iter().all(|byte| *byte == 0))
    });
}

fn take_disposals() -> Vec<bool> {
    DISPOSALS.with(|events| std::mem::take(&mut *events.borrow_mut()))
}

#[test]
fn consumed_and_abandoned_verifiers_erase_their_owned_token() -> Result<(), Box<dyn Error>> {
    for valid in [true, false] {
        let (mut hello, mut verifier) = fixture()?;
        if !valid {
            hello.role = WorkerRole::PolicyBroker;
        }
        take_disposals();
        assert_eq!(verifier.consume_hello(&hello).is_ok(), valid);
        assert_eq!(take_disposals(), vec![true]);
        drop(verifier);
        assert!(take_disposals().is_empty());
        drop(hello);
        assert_eq!(take_disposals(), vec![true]);
    }
    let (proof, pending) = issue_worker_authentication(instance()?, WorkerRole::BrowserWorker)
        .map_err(|error| format!("bootstrap entropy: {error}"))?;
    take_disposals();
    drop(pending);
    assert_eq!(take_disposals(), vec![true]);
    drop(proof);
    assert_eq!(take_disposals(), vec![true]);
    Ok(())
}

#[test]
fn observation_failure_retains_the_live_secret_until_owner_disposal() -> Result<(), Box<dyn Error>>
{
    let (hello, verifier) = fixture()?;
    take_disposals();
    assert!(
        verifier
            .check_observed(Err(PeerCredentialError::InvalidProcessId))
            .is_err()
    );
    assert!(take_disposals().is_empty());
    assert!(verifier.launch.borrow().is_some());
    drop(verifier);
    assert_eq!(take_disposals(), vec![true]);
    drop(hello);
    Ok(())
}

#[test]
fn malformed_token_arrays_dispose_of_partially_decoded_storage() {
    for text in [
        "[171,172]".to_owned(),
        format!("[{}]", vec!["171"; 33].join(",")),
        "[171,256]".to_owned(),
    ] {
        take_disposals();
        assert!(serde_json::from_str::<BootstrapToken>(&text).is_err());
        assert_eq!(take_disposals(), vec![true]);
    }
}

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
        expires_at: Instant::now() + MAX_BOOTSTRAP_LIFETIME,
    }
    .bind(expected()?);
    Ok((
        WorkerHello::new(instance()?, WorkerRole::BrowserWorker, token),
        verifier,
    ))
}

#[test]
fn expiration_consumes_and_erases_the_verifier_secret() -> Result<(), Box<dyn Error>> {
    let token = BootstrapToken([171; 32]);
    let hello = WorkerHello::new(instance()?, WorkerRole::BrowserWorker, token.clone());
    let mut verifier = WorkerVerifier {
        launch: RefCell::new(Some(WorkerLaunch {
            instance_id: instance()?,
            role: WorkerRole::BrowserWorker,
            bootstrap_token: token,
        })),
        expected: expected()?,
        expires_at: Instant::now(),
    };
    take_disposals();
    assert_eq!(
        verifier.consume_hello(&hello),
        Err(AuthenticationError::Expired)
    );
    assert_eq!(take_disposals(), vec![true]);
    assert_eq!(
        verifier.consume_hello(&hello),
        Err(AuthenticationError::AlreadyConsumed)
    );
    drop(hello);
    assert_eq!(take_disposals(), vec![true]);
    Ok(())
}

#[test]
fn issued_verifier_has_a_finite_hard_lifetime() -> Result<(), Box<dyn Error>> {
    let before = Instant::now();
    let (_, pending) = issue_worker_authentication(instance()?, WorkerRole::BrowserWorker)
        .map_err(|error| format!("bootstrap entropy: {error}"))?;
    assert!(pending.expires_at > before);
    assert!(pending.expires_at <= Instant::now() + MAX_BOOTSTRAP_LIFETIME);
    Ok(())
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
