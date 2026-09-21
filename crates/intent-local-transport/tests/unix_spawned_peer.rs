#![cfg(any(target_os = "linux", target_vendor = "apple"))]

use intent_contracts::WorkerInstanceId;
use intent_local_transport::{
    AuthenticationError, ExpectedPeer, MessageFamily, WorkerHello, WorkerRole,
    create_private_unix_listener, issue_worker_authentication,
};
use nix::unistd::{getegid, geteuid};
use std::error::Error;
use std::fs::DirBuilder;
use std::io::{self, Read, Write};
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

type TestResult = Result<(), Box<dyn Error>>;
const ENDPOINT_ENV: &str = "INTENT_TEST_UNIX_PEER_ENDPOINT";
const TIMEOUT: Duration = Duration::from_secs(15);
const MAX_HELLO: usize = 4096;

fn instance() -> Result<WorkerInstanceId, Box<dyn Error>> {
    Ok("018f47f7-5a86-7c00-8000-000000000501".parse()?)
}

fn remaining(deadline: Instant) -> io::Result<Duration> {
    deadline
        .checked_duration_since(Instant::now())
        .filter(|remaining| !remaining.is_zero())
        .ok_or_else(|| io::ErrorKind::TimedOut.into())
}

fn pause(deadline: Instant) -> io::Result<()> {
    std::thread::sleep(remaining(deadline)?.min(Duration::from_millis(2)));
    Ok(())
}

struct Endpoint {
    directory: PathBuf,
    path: PathBuf,
}

impl Endpoint {
    fn new() -> Result<Self, Box<dyn Error>> {
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).map_err(|error| format!("endpoint entropy: {error}"))?;
        // A short private path also fits macOS's sockaddr_un limit.
        let directory = PathBuf::from("/tmp").join(format!(
            "intent-peer-{}-{:032x}",
            std::process::id(),
            u128::from_ne_bytes(nonce)
        ));
        DirBuilder::new().mode(0o700).create(&directory)?;
        Ok(Self {
            path: directory.join("peer"),
            directory,
        })
    }
}

impl Drop for Endpoint {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
        let _ = std::fs::remove_dir(&self.directory);
    }
}

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn accept(listener: &UnixListener, deadline: Instant) -> io::Result<UnixStream> {
    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                stream.set_nonblocking(false)?;
                return Ok(stream);
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => pause(deadline)?,
            Err(error) => return Err(error),
        }
    }
}

fn read_exact(stream: &mut UnixStream, mut bytes: &mut [u8], deadline: Instant) -> io::Result<()> {
    while !bytes.is_empty() {
        stream.set_read_timeout(Some(remaining(deadline)?))?;
        match stream.read(bytes) {
            Ok(0) => return Err(io::ErrorKind::UnexpectedEof.into()),
            Ok(count) => bytes = &mut bytes[count..],
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

fn write_all(stream: &mut UnixStream, mut bytes: &[u8], deadline: Instant) -> io::Result<()> {
    while !bytes.is_empty() {
        stream.set_write_timeout(Some(remaining(deadline)?))?;
        match stream.write(bytes) {
            Ok(0) => return Err(io::ErrorKind::WriteZero.into()),
            Ok(count) => bytes = &bytes[count..],
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

#[test]
#[ignore = "launched by the spawned peer cases with private bootstrap stdin"]
fn unix_peer_child() -> TestResult {
    let deadline = Instant::now() + TIMEOUT;
    let mut bootstrap = Vec::new();
    std::io::stdin()
        .take((MAX_HELLO + 1) as u64)
        .read_to_end(&mut bootstrap)?;
    if bootstrap.len() > MAX_HELLO {
        return Err("oversized fixture bootstrap".into());
    }
    let hello: WorkerHello = serde_json::from_slice(&bootstrap)?;
    let mut stream =
        UnixStream::connect(std::env::var_os(ENDPOINT_ENV).ok_or("missing endpoint")?)?;
    let bytes = serde_json::to_vec(&hello)?;
    write_all(
        &mut stream,
        &u16::try_from(bytes.len())?.to_be_bytes(),
        deadline,
    )?;
    write_all(&mut stream, &bytes, deadline)?;
    let mut ack = [0];
    read_exact(&mut stream, &mut ack, deadline)?;
    assert_eq!(ack, [1]);
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HelloCase {
    Valid,
    WrongToken,
    WrongRole,
    WrongInstance,
}

fn authentication_case(case: HelloCase) -> TestResult {
    let deadline = Instant::now() + TIMEOUT;
    let endpoint = Endpoint::new()?;
    let listener = create_private_unix_listener(&endpoint.path)?;
    assert_eq!(
        std::fs::symlink_metadata(&endpoint.path)?
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    listener.set_nonblocking(true)?;
    let (token, pending) = issue_worker_authentication(instance()?, WorkerRole::BrowserWorker)
        .map_err(|error| format!("bootstrap entropy: {error}"))?;
    let valid = WorkerHello::new(instance()?, WorkerRole::BrowserWorker, token.clone());
    let supplied = match case {
        HelloCase::Valid => valid.clone(),
        HelloCase::WrongToken => {
            let (other, _) = issue_worker_authentication(instance()?, WorkerRole::BrowserWorker)
                .map_err(|error| format!("bootstrap entropy: {error}"))?;
            WorkerHello::new(instance()?, WorkerRole::BrowserWorker, other)
        }
        HelloCase::WrongRole => WorkerHello::new(instance()?, WorkerRole::PolicyBroker, token),
        HelloCase::WrongInstance => WorkerHello::new(
            "018f47f7-5a86-7c00-8000-000000000502".parse()?,
            WorkerRole::BrowserWorker,
            token,
        ),
    };
    let mut child = ChildGuard(
        Command::new(std::env::current_exe()?)
            .args(["--exact", "unix_peer_child", "--ignored", "--nocapture"])
            .env(ENDPOINT_ENV, &endpoint.path)
            .stdin(Stdio::piped())
            .spawn()?,
    );
    assert_ne!(child.0.id(), std::process::id());
    let mut verifier = pending.bind(ExpectedPeer::unix_process(
        child.0.id(),
        geteuid().as_raw(),
        getegid().as_raw(),
    )?);
    if case == HelloCase::Valid {
        let foreign = UnixStream::connect(&endpoint.path)?;
        let foreign_connection = accept(&listener, deadline)?;
        assert_eq!(
            verifier.check_unix_peer(&foreign_connection),
            Err(AuthenticationError::PeerCredentialMismatch)
        );
        assert_eq!(
            verifier.authenticate_unix(&valid, &foreign_connection),
            Err(AuthenticationError::PeerCredentialMismatch)
        );
        drop(foreign_connection);
        drop(foreign);
    }
    child
        .0
        .stdin
        .take()
        .ok_or("missing bootstrap pipe")?
        .write_all(&serde_json::to_vec(&supplied)?)?;
    let mut stream = accept(&listener, deadline)?;
    verifier.check_unix_peer(&stream)?;
    if case == HelloCase::Valid {
        for (uid, gid) in [
            (geteuid().as_raw() ^ 1, getegid().as_raw()),
            (geteuid().as_raw(), getegid().as_raw() ^ 1),
        ] {
            let (_, pending) = issue_worker_authentication(instance()?, WorkerRole::BrowserWorker)
                .map_err(|error| format!("bootstrap entropy: {error}"))?;
            let wrong_principal = pending.bind(ExpectedPeer::unix_process(child.0.id(), uid, gid)?);
            assert_eq!(
                wrong_principal.check_unix_peer(&stream),
                Err(AuthenticationError::PeerCredentialMismatch)
            );
        }
    }
    let mut size = [0; 2];
    read_exact(&mut stream, &mut size, deadline)?;
    let size = usize::from(u16::from_be_bytes(size));
    if size > MAX_HELLO {
        return Err("oversized fixture hello".into());
    }
    let mut bytes = vec![0; size];
    read_exact(&mut stream, &mut bytes, deadline)?;
    let hello: WorkerHello = serde_json::from_slice(&bytes)?;
    assert_eq!(hello, supplied);
    let result = verifier.authenticate_unix(&hello, &stream);
    if case == HelloCase::Valid {
        let identity = result?;
        assert_eq!(identity.instance_id(), instance()?);
        assert_eq!(identity.role(), WorkerRole::BrowserWorker);
        assert!(identity.allows(MessageFamily::BrowserObservation));
        assert!(!identity.allows(MessageFamily::PolicyDecision));
    } else {
        assert_eq!(result, Err(AuthenticationError::LaunchIdentityMismatch));
    }
    assert_eq!(
        verifier.authenticate_unix(&valid, &stream),
        Err(AuthenticationError::AlreadyConsumed)
    );
    write_all(&mut stream, &[1], deadline)?;
    loop {
        if let Some(status) = child.0.try_wait()? {
            assert!(
                status.success(),
                "child authentication fixture failed: {status}"
            );
            break;
        }
        pause(deadline)?;
    }
    Ok(())
}

#[test]
fn spawned_peer_rejects_foreign_connection_then_authenticates() -> TestResult {
    authentication_case(HelloCase::Valid)
}

#[test]
fn spawned_peer_wrong_token_consumes_credential() -> TestResult {
    authentication_case(HelloCase::WrongToken)
}

#[test]
fn spawned_peer_wrong_role_consumes_credential() -> TestResult {
    authentication_case(HelloCase::WrongRole)
}

#[test]
fn spawned_peer_wrong_instance_consumes_credential() -> TestResult {
    authentication_case(HelloCase::WrongInstance)
}
