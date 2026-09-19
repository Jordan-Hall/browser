#![cfg(windows)]
#![allow(unsafe_code)]

use intent_contracts::WorkerInstanceId;
use intent_local_transport::{
    AuthenticationError, BootstrapToken, MessageFamily, OneShotAuthenticator,
    PeerCredentialEvidence, PeerExpectation, WorkerHello, WorkerLaunchRecord, WorkerRole,
    named_pipe_client_credentials, named_pipe_server_credentials,
};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use std::process::{Child, Command, Stdio};
use std::ptr::{null, null_mut};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use windows_sys::Win32::Foundation::{
    ERROR_NO_DATA, ERROR_PIPE_BUSY, ERROR_PIPE_CONNECTED, ERROR_PIPE_LISTENING,
    INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Storage::FileSystem::PIPE_ACCESS_DUPLEX;
use windows_sys::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, PIPE_NOWAIT, PIPE_REJECT_REMOTE_CLIENTS,
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
    assert_eq!(
        named_pipe_server_credentials(&pipe)?,
        PeerCredentialEvidence::Windows { process_id: parent }
    );
    // The server waits for our hello, so this connected pipe has no incoming bytes yet.
    let Err(timeout) = read_line(&mut pipe, Instant::now() + Duration::from_millis(30)) else {
        return Err("idle pipe read did not time out".into());
    };
    assert_eq!(timeout.kind(), io::ErrorKind::TimedOut);
    let hello = WorkerHello::new(
        instance()?,
        WorkerRole::BrowserWorker,
        BootstrapToken::from_bytes([42; 32]),
    );
    write_line(&mut pipe, &serde_json::to_vec(&hello)?, deadline)?;
    assert_eq!(read_line(&mut pipe, deadline)?, b"authenticated");
    Ok(())
}

#[test]
fn named_pipe_binds_spawned_process_and_launch_identity() -> TestResult {
    let deadline = Instant::now() + TIMEOUT;
    let name = format!(
        r"\\.\pipe\intent-peer-test-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    );
    let wide_name: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
    // This fixture deliberately polls synchronous nonblocking handles to bound every I/O call.
    // SAFETY: wide_name is terminated and lives through the call; null security attributes use
    // the default descriptor and disable inheritance. The returned handle is checked then owned.
    let handle = unsafe {
        CreateNamedPipeW(
            wide_name.as_ptr(),
            PIPE_ACCESS_DUPLEX,
            PIPE_NOWAIT | PIPE_REJECT_REMOTE_CLIENTS,
            1,
            4096,
            4096,
            0,
            null(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error().into());
    }
    // SAFETY: CreateNamedPipeW returned a fresh valid handle, transferred once into File.
    let mut pipe = unsafe { File::from_raw_handle(handle) };
    let mut child = ChildGuard(
        Command::new(std::env::current_exe()?)
            .args(["--exact", "named_pipe_child", "--ignored", "--nocapture"])
            .env(PIPE_ENV, &name)
            .env(PARENT_ENV, std::process::id().to_string())
            .stdin(Stdio::null())
            .spawn()?,
    );
    loop {
        // SAFETY: pipe owns a live synchronous nonblocking handle. No OVERLAPPED is used.
        let connected = unsafe { ConnectNamedPipe(pipe.as_raw_handle(), null_mut()) };
        if connected != 0 {
            // In nonblocking mode success can mean the pipe has just entered listening state.
            pause(deadline)?;
            continue;
        }
        let error = io::Error::last_os_error();
        match error.raw_os_error() {
            Some(code) if code == ERROR_PIPE_CONNECTED as i32 => break,
            Some(code) if code == ERROR_PIPE_LISTENING as i32 => pause(deadline)?,
            _ => return Err(error.into()),
        }
    }
    let peer = named_pipe_client_credentials(&pipe)?;
    assert_eq!(
        peer,
        PeerCredentialEvidence::Windows {
            process_id: child.0.id()
        }
    );
    assert_ne!(child.0.id(), std::process::id());
    let hello: WorkerHello = serde_json::from_slice(&read_line(&mut pipe, deadline)?)?;
    let authenticate = |expected_peer, role, token| -> Result<_, Box<dyn Error>> {
        let launch = WorkerLaunchRecord::new(
            instance()?,
            role,
            BootstrapToken::from_bytes(token),
            PeerExpectation::exact(expected_peer),
        );
        Ok(OneShotAuthenticator::new(launch).authenticate(&hello, peer))
    };
    assert_eq!(
        authenticate(
            PeerCredentialEvidence::Windows {
                process_id: std::process::id()
            },
            WorkerRole::BrowserWorker,
            [42; 32]
        )?,
        Err(AuthenticationError::PeerCredentialMismatch)
    );
    assert_eq!(
        authenticate(peer, WorkerRole::BrowserWorker, [43; 32])?,
        Err(AuthenticationError::LaunchIdentityMismatch)
    );
    assert_eq!(
        authenticate(peer, WorkerRole::PolicyBroker, [42; 32])?,
        Err(AuthenticationError::LaunchIdentityMismatch)
    );
    let wrong_instance = WorkerLaunchRecord::new(
        "018f47f7-5a86-7c00-8000-000000000502".parse()?,
        WorkerRole::BrowserWorker,
        BootstrapToken::from_bytes([42; 32]),
        PeerExpectation::exact(peer),
    );
    assert_eq!(
        OneShotAuthenticator::new(wrong_instance).authenticate(&hello, peer),
        Err(AuthenticationError::LaunchIdentityMismatch)
    );
    let identity = authenticate(peer, WorkerRole::BrowserWorker, [42; 32])??;
    assert_eq!(identity.peer(), peer);
    assert_eq!(identity.instance_id(), instance()?);
    assert_eq!(identity.role(), WorkerRole::BrowserWorker);
    assert!(identity.allows(MessageFamily::BrowserObservation));
    assert!(!identity.allows(MessageFamily::PolicyDecision));
    write_line(&mut pipe, b"authenticated", deadline)?;
    loop {
        if let Some(status) = child.0.try_wait()? {
            assert!(status.success(), "child peer verification failed: {status}");
            break;
        }
        pause(deadline)?;
    }
    Ok(())
}
