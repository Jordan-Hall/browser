#![cfg(windows)]

use intent_contracts::WorkerInstanceId;
use intent_local_transport::{WorkerChannel, WorkerRole, verify_named_pipe_server};
use intent_supervisor::{WindowsBootstrapPacket, WindowsPendingWorker};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
    process::{Command, Stdio},
    time::Duration,
};

type TestResult = Result<(), Box<dyn Error>>;

fn instance() -> Result<WorkerInstanceId, Box<dyn Error>> {
    Ok("018f47f7-5a86-7c00-8000-000000000505".parse()?)
}

#[test]
#[ignore = "launched only by windows_supervisor_binds_spawned_worker_channels"]
fn windows_supervisor_child() -> TestResult {
    let mut bootstrap = Vec::new();
    std::io::stdin().take(4097).read_to_end(&mut bootstrap)?;
    if bootstrap.len() > 4096 {
        return Err("oversized supervisor bootstrap".into());
    }
    let packet: WindowsBootstrapPacket = serde_json::from_slice(&bootstrap)?;
    let mut control = OpenOptions::new()
        .read(true)
        .write(true)
        .open(packet.control_endpoint.as_str())?;
    let mut progress = OpenOptions::new()
        .read(true)
        .write(true)
        .open(packet.progress_endpoint.as_str())?;
    verify_named_pipe_server(&control, packet.supervisor_process_id)?;
    verify_named_pipe_server(&progress, packet.supervisor_process_id)?;
    write_hello(&mut control, &packet.control)?;
    write_hello(&mut progress, &packet.progress)?;
    let mut ack = [0_u8; 1];
    control.read_exact(&mut ack)?;
    progress.read_exact(&mut ack)?;
    Ok(())
}

fn write_hello(pipe: &mut File, hello: &intent_local_transport::WorkerHello) -> TestResult {
    let mut bytes = serde_json::to_vec(hello)?;
    bytes.push(b'\n');
    pipe.write_all(&bytes)?;
    pipe.flush()?;
    Ok(())
}

#[test]
fn windows_supervisor_binds_spawned_worker_channels() -> TestResult {
    let mut command = Command::new(std::env::current_exe()?);
    command
        .args([
            "--exact",
            "windows_supervisor_child",
            "--ignored",
            "--nocapture",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let pending = WindowsPendingWorker::spawn(
        &mut command,
        instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?;
    assert_ne!(pending.child_id()?, std::process::id());
    let authenticated = pending.authenticate()?;
    assert_eq!(authenticated.control_identity().instance_id(), instance()?);
    assert_eq!(
        authenticated.control_identity().role(),
        WorkerRole::BrowserWorker
    );
    assert_eq!(
        authenticated.control_identity().channel(),
        Some(WorkerChannel::Control)
    );
    assert_eq!(authenticated.progress_identity().instance_id(), instance()?);
    assert_eq!(
        authenticated.progress_identity().role(),
        WorkerRole::BrowserWorker
    );
    assert_eq!(
        authenticated.progress_identity().channel(),
        Some(WorkerChannel::Progress)
    );
    let (mut child, mut control, mut progress, _, _) = authenticated.into_parts()?;
    control.write_all(&[1])?;
    progress.write_all(&[1])?;
    assert!(child.wait()?.success());
    Ok(())
}
