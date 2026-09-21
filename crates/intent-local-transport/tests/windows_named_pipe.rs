#![cfg(windows)]
#![allow(unsafe_code)]

use intent_contracts::WorkerInstanceId;
use intent_local_transport::{
    AuthenticationError, ExpectedPeer, MessageFamily, PeerCredentialError, WorkerHello, WorkerRole,
    create_current_user_named_pipe, issue_worker_authentication, verify_named_pipe_server,
};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::windows::io::AsRawHandle;
use std::process::{Child, Command, Stdio};
use std::ptr::{null, null_mut};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use windows_sys::Win32::Foundation::{
    ERROR_NO_DATA, ERROR_PIPE_BUSY, ERROR_PIPE_CONNECTED, ERROR_PIPE_LISTENING,
};
use windows_sys::Win32::System::Pipes::{
    ConnectNamedPipe, DisconnectNamedPipe, PIPE_NOWAIT, PIPE_REJECT_REMOTE_CLIENTS,
    SetNamedPipeHandleState,
};

type TestResult = Result<(), Box<dyn Error>>;
const PIPE_ENV: &str = "INTENT_PEER_TEST_PIPE";
const PARENT_ENV: &str = "INTENT_PEER_TEST_PARENT";
const TIMEOUT: Duration = Duration::from_secs(15);

fn instance() -> Result<WorkerInstanceId, Box<dyn Error>> {
    Ok("018f47f7-5a86-7c00-8000-000000000501".parse()?)
}

fn pause(deadline: Instant) -> io::Result<()> {
    if Instant::now() >= deadline {
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "named pipe test timed out",
        ));
    }
    thread::sleep(Duration::from_millis(10));
    Ok(())
}

fn read_line(pipe: &mut File, deadline: Instant) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    loop {
        let mut byte = [0];
        match pipe.read(&mut byte) {
            Ok(1) if byte[0] == b'\n' => return Ok(bytes),
            Ok(1) => bytes.push(byte[0]),
            // std::fs::File maps ERROR_NO_DATA on a nonblocking pipe to an empty read.
            Ok(_) => pause(deadline)?,
            Err(error) if error.raw_os_error() == Some(ERROR_NO_DATA as i32) => pause(deadline)?,
            Err(error) => return Err(error),
        }
        if bytes.len() > 4096 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "oversized test handshake",
            ));
        }
        if Instant::now() >= deadline {
            return Err(io::ErrorKind::TimedOut.into());
        }
    }
}

fn write_line(pipe: &mut File, bytes: &[u8], deadline: Instant) -> io::Result<()> {
    let mut frame = bytes.to_vec();
    frame.push(b'\n');
    let mut remaining = frame.as_slice();
    while !remaining.is_empty() {
        match pipe.write(remaining) {
            Ok(0) => pause(deadline)?,
            Ok(count) => remaining = &remaining[count..],
            Err(error) if error.raw_os_error() == Some(ERROR_NO_DATA as i32) => pause(deadline)?,
            Err(error) => return Err(error),
        }
        if Instant::now() >= deadline {
            return Err(io::ErrorKind::TimedOut.into());
        }
    }
    Ok(())
}

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        // Kill before waiting so an assertion failure cannot leave the helper waiting for an ACK.
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
#[ignore = "launched only by named_pipe_binds_spawned_process_and_launch_identity"]
fn named_pipe_child() -> TestResult {
    let deadline = Instant::now() + TIMEOUT;
    let name = std::env::var(PIPE_ENV)?;
    let parent: u32 = std::env::var(PARENT_ENV)?.parse()?;
    let mut bootstrap = Vec::new();
    std::io::stdin().take(4097).read_to_end(&mut bootstrap)?;
    if bootstrap.len() > 4096 {
        return Err("oversized fixture bootstrap".into());
    }
    let hello: WorkerHello = serde_json::from_slice(&bootstrap)?;
    let mut pipe = loop {
        match OpenOptions::new().read(true).write(true).open(&name) {
            Ok(pipe) => break pipe,
            Err(error) if error.raw_os_error() == Some(ERROR_PIPE_BUSY as i32) => pause(deadline)?,
            Err(error) => return Err(error.into()),
        }
    };
    let mode = PIPE_NOWAIT;
    // SAFETY: pipe owns a live handle and mode points to initialized storage. Optional pointers
    // are null, and neither the handle nor the mode pointer is retained by the OS call.
    if unsafe { SetNamedPipeHandleState(pipe.as_raw_handle(), &mode, null(), null()) } == 0 {
        return Err(io::Error::last_os_error().into());
    }
    verify_named_pipe_server(&pipe, parent)?;
    assert_eq!(
        verify_named_pipe_server(&pipe, std::process::id()),
        Err(PeerCredentialError::Mismatch)
    );
    // The server waits for our hello, so this connected pipe has no incoming bytes yet.
    let Err(timeout) = read_line(&mut pipe, Instant::now() + Duration::from_millis(30)) else {
        return Err("idle pipe read did not time out".into());
    };
    assert_eq!(timeout.kind(), io::ErrorKind::TimedOut);
    write_line(&mut pipe, &serde_json::to_vec(&hello)?, deadline)?;
    assert_eq!(read_line(&mut pipe, deadline)?, b"checked");
    Ok(())
}

fn server_pipe() -> Result<(String, File), Box<dyn Error>> {
    let name = format!(
        r"\\.\pipe\intent-peer-test-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    );
    // The production transport primitive installs a protected DACL granting this principal only,
    // rejects remote clients, disables inheritance, and bounds the pipe buffers.
    let pipe = create_current_user_named_pipe(name.as_ref(), 4096)?;
    Ok((name, pipe))
}

#[test]
fn named_pipe_factory_rejects_unbounded_or_nonlocal_endpoints() {
    assert!(matches!(
        create_current_user_named_pipe(r"\\.\pipe\intent-invalid".as_ref(), 0),
        Err(PeerCredentialError::InvalidPipeBufferSize)
    ));
    assert!(matches!(
        create_current_user_named_pipe(r"\\.\pipe\intent-invalid".as_ref(), 1024 * 1024 + 1),
        Err(PeerCredentialError::InvalidPipeBufferSize)
    ));
    assert!(matches!(
        create_current_user_named_pipe(r"C:\intent-invalid".as_ref(), 4096),
        Err(PeerCredentialError::InvalidPipeName)
    ));
}

fn connected(pipe: &File, deadline: Instant) -> io::Result<()> {
    loop {
        // SAFETY: pipe owns a live synchronous nonblocking handle. No OVERLAPPED is used.
        let connected = unsafe { ConnectNamedPipe(pipe.as_raw_handle(), null_mut()) };
        if connected != 0 {
            pause(deadline)?;
            continue;
        }
        let error = io::Error::last_os_error();
        match error.raw_os_error() {
            Some(code) if code == ERROR_PIPE_CONNECTED as i32 => return Ok(()),
            Some(code) if code == ERROR_PIPE_LISTENING as i32 => pause(deadline)?,
            _ => return Err(error),
        }
    }
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
    let (name, mut pipe) = server_pipe()?;
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
            .args(["--exact", "named_pipe_child", "--ignored", "--nocapture"])
            .env(PIPE_ENV, &name)
            .env(PARENT_ENV, std::process::id().to_string())
            .stdin(Stdio::piped())
            .spawn()?,
    );
    assert_ne!(child.0.id(), std::process::id());
    let mut verifier = pending.bind(ExpectedPeer::windows_process(child.0.id())?);
    if case == HelloCase::Valid {
        // SAFETY: pipe owns a live nonblocking server handle. No OVERLAPPED is used.
        if unsafe { ConnectNamedPipe(pipe.as_raw_handle(), null_mut()) } == 0
            && io::Error::last_os_error().raw_os_error() != Some(ERROR_PIPE_LISTENING as i32)
        {
            return Err(io::Error::last_os_error().into());
        }
        let foreign = OpenOptions::new().read(true).write(true).open(&name)?;
        connected(&pipe, deadline)?;
        assert_eq!(
            verifier.check_named_pipe_client(&pipe),
            Err(AuthenticationError::PeerCredentialMismatch)
        );
        assert_eq!(
            verifier.authenticate_named_pipe_client(&valid, &pipe),
            Err(AuthenticationError::PeerCredentialMismatch)
        );
        drop(foreign);
        // SAFETY: pipe still owns the server handle; disconnect permits the intended child to connect next.
        if unsafe { DisconnectNamedPipe(pipe.as_raw_handle()) } == 0 {
            return Err(io::Error::last_os_error().into());
        }
    }
    child
        .0
        .stdin
        .take()
        .ok_or("missing bootstrap pipe")?
        .write_all(&serde_json::to_vec(&supplied)?)?;
    connected(&pipe, deadline)?;
    verifier.check_named_pipe_client(&pipe)?;
    let hello: WorkerHello = serde_json::from_slice(&read_line(&mut pipe, deadline)?)?;
    assert_eq!(hello, supplied);
    let result = verifier.authenticate_named_pipe_client(&hello, &pipe);
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
        verifier.authenticate_named_pipe_client(&valid, &pipe),
        Err(AuthenticationError::AlreadyConsumed)
    );
    write_line(&mut pipe, b"checked", deadline)?;
    loop {
        if let Some(status) = child.0.try_wait()? {
            assert!(status.success(), "child peer verification failed: {status}");
            break;
        }
        pause(deadline)?;
    }
    Ok(())
}

#[test]
fn named_pipe_binds_spawned_process_and_launch_identity() -> TestResult {
    for case in [
        HelloCase::Valid,
        HelloCase::WrongToken,
        HelloCase::WrongRole,
        HelloCase::WrongInstance,
    ] {
        authentication_case(case)?;
    }
    Ok(())
}
