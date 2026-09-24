#![cfg(windows)]

use intent_contracts::{ContentHash, WorkerInstanceId};
use intent_local_transport::{WorkerHello, WorkerRole, verify_named_pipe_server};
use intent_supervisor::{
    HealthPolicy, LifecycleControl, RestartDecision, RestartPolicy, WindowsBootstrapPacket,
    WindowsExecutableIdentity, WindowsPendingWorker, WindowsRestartLifecycle,
};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    process::{Command, Stdio},
    thread,
    time::Duration,
};

type TestResult = Result<(), Box<dyn Error>>;

fn instance() -> Result<WorkerInstanceId, Box<dyn Error>> {
    Ok("018f47f7-5a86-7c00-8000-000000000605".parse()?)
}

fn fresh_instance() -> Result<WorkerInstanceId, Box<dyn Error>> {
    Ok("018f47f7-5a86-7c00-8000-000000000606".parse()?)
}

fn child_command() -> Result<(Command, WindowsExecutableIdentity), Box<dyn Error>> {
    let executable_path = std::env::current_exe()?;
    let expected = ContentHash::from_bytes(Sha256::digest(fs::read(&executable_path)?).into());
    let executable = WindowsExecutableIdentity::load(&executable_path, expected)?;
    let mut command = Command::new(executable.path());
    command
        .args([
            "--exact",
            "windows_lifecycle_control_child",
            "--ignored",
            "--nocapture",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    Ok((command, executable))
}

#[test]
#[ignore = "launched only by Windows lifecycle-control subprocess regressions"]
fn windows_lifecycle_control_child() -> TestResult {
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

    if std::env::var_os("INTENT_WINDOWS_IGNORE_LIFECYCLE_CONTROL").is_some() {
        thread::sleep(Duration::from_secs(30));
        return Ok(());
    }

    let expected_generation = packet.control.instance_id();
    let control_message: LifecycleControl = serde_json::from_slice(&read_line(&mut control)?)?;
    match control_message {
        LifecycleControl::Cancel { generation, .. } if generation == expected_generation => Ok(()),
        _ => Err("unexpected Windows lifecycle control message".into()),
    }
}

fn write_hello(pipe: &mut File, hello: &WorkerHello) -> TestResult {
    let mut bytes = serde_json::to_vec(hello)?;
    bytes.push(b'\n');
    pipe.write_all(&bytes)?;
    pipe.flush()?;
    Ok(())
}

fn read_line(pipe: &mut File) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes = Vec::new();
    loop {
        if bytes.len() >= 4096 {
            return Err("oversized lifecycle control message".into());
        }
        let mut byte = [0_u8; 1];
        pipe.read_exact(&mut byte)?;
        if byte[0] == b'\n' {
            return Ok(bytes);
        }
        bytes.push(byte[0]);
    }
}

#[test]
fn windows_controlled_worker_delivers_cancel_before_exit() -> TestResult {
    let (mut command, executable) = child_command()?;
    let controlled = WindowsPendingWorker::spawn_pinned(
        &mut command,
        executable,
        instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?
    .authenticate()?
    .into_controlled()?;

    let report = controlled.stop(HealthPolicy {
        stop_grace: Duration::from_secs(2),
        terminate_grace: Duration::from_secs(5),
        ..HealthPolicy::default()
    })?;
    assert!(report.cancel_sent());
    assert!(!report.escalated());
    assert!(report.status().success());
    Ok(())
}

#[test]
fn windows_controlled_worker_escalates_after_cancel_grace() -> TestResult {
    let (mut command, executable) = child_command()?;
    command.env("INTENT_WINDOWS_IGNORE_LIFECYCLE_CONTROL", "1");
    let controlled = WindowsPendingWorker::spawn_pinned(
        &mut command,
        executable,
        instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?
    .authenticate()?
    .into_controlled()?;

    let report = controlled.stop(HealthPolicy {
        stop_grace: Duration::from_millis(50),
        terminate_grace: Duration::from_secs(5),
        ..HealthPolicy::default()
    })?;
    assert!(report.cancel_sent());
    assert!(report.escalated());
    assert!(!report.status().success());
    Ok(())
}

#[test]
fn windows_restart_lifecycle_routes_controlled_stop_and_restart_budget() -> TestResult {
    let policy = RestartPolicy::bounded(2, Duration::from_millis(50), Duration::from_millis(50))?;
    let mut lifecycle = WindowsRestartLifecycle::new(policy, Duration::from_secs(5))?;

    let (mut first, first_executable) = child_command()?;
    first.env("INTENT_WINDOWS_IGNORE_LIFECYCLE_CONTROL", "1");
    let first = WindowsPendingWorker::spawn_pinned(
        &mut first,
        first_executable,
        instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?
    .authenticate()?
    .into_controlled()?;

    let first_report = lifecycle.stop_controlled_after_auth(
        first,
        HealthPolicy {
            stop_grace: Duration::from_millis(50),
            terminate_grace: Duration::from_secs(5),
            ..HealthPolicy::default()
        },
    )?;
    assert!(first_report.cancel_sent());
    assert!(first_report.escalated());
    assert!(!first_report.status().success());
    assert!(matches!(
        lifecycle.decision(),
        RestartDecision::BackoffUntil(_)
    ));

    thread::sleep(Duration::from_millis(75));
    assert_eq!(lifecycle.decision(), RestartDecision::Eligible);

    let (mut second, second_executable) = child_command()?;
    let second = lifecycle
        .restart_pinned(
            &mut second,
            second_executable,
            fresh_instance()?,
            WorkerRole::BrowserWorker,
            Duration::from_secs(15),
        )?
        .authenticate()?
        .into_controlled()?;
    assert_eq!(second.control_identity().instance_id(), fresh_instance()?);
    assert_eq!(second.progress_identity().instance_id(), fresh_instance()?);

    let second_report = lifecycle.stop_controlled_after_auth(
        second,
        HealthPolicy {
            stop_grace: Duration::from_secs(2),
            terminate_grace: Duration::from_secs(5),
            ..HealthPolicy::default()
        },
    )?;
    assert!(second_report.cancel_sent());
    assert!(!second_report.escalated());
    assert!(second_report.status().success());
    assert_eq!(lifecycle.decision(), RestartDecision::CircuitOpen);
    Ok(())
}
