#![cfg(windows)]

use intent_contracts::WorkerInstanceId;
use intent_local_transport::{WorkerChannel, WorkerHello, WorkerRole, verify_named_pipe_server};
use intent_supervisor::{
    HealthPolicy, RestartDecision, RestartPolicy, SupervisorError, WindowsBootstrapPacket,
    WindowsPendingWorker, WindowsRestartLifecycle,
};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    process::{Command, Stdio},
    thread,
    time::Duration,
};

type TestResult = Result<(), Box<dyn Error>>;

fn instance() -> Result<WorkerInstanceId, Box<dyn Error>> {
    Ok("018f47f7-5a86-7c00-8000-000000000505".parse()?)
}

fn fresh_instance() -> Result<WorkerInstanceId, Box<dyn Error>> {
    Ok("018f47f7-5a86-7c00-8000-000000000506".parse()?)
}

fn third_instance() -> Result<WorkerInstanceId, Box<dyn Error>> {
    Ok("018f47f7-5a86-7c00-8000-000000000507".parse()?)
}

fn child_command() -> Result<Command, Box<dyn Error>> {
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
    Ok(command)
}

#[test]
#[ignore = "launched only by Windows supervisor subprocess regressions"]
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
    if std::env::var_os("INTENT_WINDOWS_STALE_GENERATION").is_some() {
        let mut stale = serde_json::to_value(&packet.control)?;
        stale["instance_id"] = serde_json::to_value(instance()?)?;
        let stale: WorkerHello = serde_json::from_value(stale)?;
        write_hello(&mut control, &stale)?;
    } else {
        write_hello(&mut control, &packet.control)?;
    }
    write_hello(&mut progress, &packet.progress)?;

    if std::env::var_os("INTENT_WINDOWS_COOPERATIVE_STOP").is_some() {
        let mut byte = [0_u8; 1];
        return match control.read(&mut byte) {
            Ok(0) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
            Ok(_) => Err("unexpected control byte during cooperative stop".into()),
            Err(error) => Err(error.into()),
        };
    }
    if std::env::var_os("INTENT_WINDOWS_IGNORE_STOP").is_some() {
        thread::sleep(Duration::from_secs(30));
        return Ok(());
    }

    let mut ack = [0_u8; 1];
    control.read_exact(&mut ack)?;
    progress.read_exact(&mut ack)?;
    Ok(())
}

fn write_hello(pipe: &mut File, hello: &WorkerHello) -> TestResult {
    let mut bytes = serde_json::to_vec(hello)?;
    bytes.push(b'\n');
    pipe.write_all(&bytes)?;
    pipe.flush()?;
    Ok(())
}

fn finish_authenticated(
    authenticated: intent_supervisor::WindowsAuthenticatedWorker,
) -> TestResult {
    let (mut child, mut control, mut progress, _, _) = authenticated.into_parts()?;
    control.write_all(&[1])?;
    progress.write_all(&[1])?;
    assert!(child.wait()?.success());
    Ok(())
}

#[test]
fn windows_supervisor_binds_spawned_worker_channels() -> TestResult {
    let mut command = child_command()?;
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
    finish_authenticated(authenticated)
}

#[test]
fn windows_supervisor_rejects_previous_generation() -> TestResult {
    let mut command = child_command()?;
    command.env("INTENT_WINDOWS_STALE_GENERATION", "1");
    let pending = WindowsPendingWorker::spawn(
        &mut command,
        fresh_instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?;
    assert!(matches!(
        pending.authenticate(),
        Err(SupervisorError::Protocol)
    ));
    Ok(())
}

#[test]
fn windows_supervisor_revokes_before_auth_then_restarts_fresh() -> TestResult {
    let mut first = child_command()?;
    let pending = WindowsPendingWorker::spawn(
        &mut first,
        instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?;
    assert!(!pending.revoke_before_auth()?.success());

    let mut second = child_command()?;
    let pending = WindowsPendingWorker::spawn(
        &mut second,
        fresh_instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?;
    let authenticated = pending.authenticate()?;
    assert_eq!(
        authenticated.control_identity().instance_id(),
        fresh_instance()?
    );
    assert_eq!(
        authenticated.progress_identity().instance_id(),
        fresh_instance()?
    );
    finish_authenticated(authenticated)
}

#[test]
fn windows_supervisor_revokes_after_auth_then_restarts_fresh() -> TestResult {
    let mut first = child_command()?;
    let pending = WindowsPendingWorker::spawn(
        &mut first,
        instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?;
    let authenticated = pending.authenticate()?;
    assert!(!authenticated.revoke_after_auth()?.success());

    let mut second = child_command()?;
    let pending = WindowsPendingWorker::spawn(
        &mut second,
        fresh_instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?;
    let authenticated = pending.authenticate()?;
    assert_eq!(
        authenticated.control_identity().instance_id(),
        fresh_instance()?
    );
    assert_eq!(
        authenticated.progress_identity().instance_id(),
        fresh_instance()?
    );
    finish_authenticated(authenticated)
}

#[test]
fn windows_supervisor_restart_budget_survives_authenticated_generations() -> TestResult {
    let policy = RestartPolicy::bounded(1, Duration::from_millis(100), Duration::from_millis(100))?;
    let mut lifecycle = WindowsRestartLifecycle::new(policy, Duration::from_secs(5))?;

    let mut first = child_command()?;
    let first = WindowsPendingWorker::spawn(
        &mut first,
        instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?
    .authenticate()?;
    assert!(!lifecycle.revoke_after_auth(first)?.success());
    assert!(matches!(
        lifecycle.decision(),
        RestartDecision::BackoffUntil(_)
    ));

    thread::sleep(Duration::from_millis(125));
    assert_eq!(lifecycle.decision(), RestartDecision::Eligible);

    let mut second = child_command()?;
    let second = lifecycle
        .restart(
            &mut second,
            fresh_instance()?,
            WorkerRole::BrowserWorker,
            Duration::from_secs(15),
        )?
        .authenticate()?;
    assert_eq!(second.control_identity().instance_id(), fresh_instance()?);
    assert_eq!(second.progress_identity().instance_id(), fresh_instance()?);
    assert!(!lifecycle.revoke_after_auth(second)?.success());
    assert_eq!(lifecycle.decision(), RestartDecision::CircuitOpen);

    let mut denied = child_command()?;
    assert!(matches!(
        lifecycle.restart(
            &mut denied,
            third_instance()?,
            WorkerRole::BrowserWorker,
            Duration::from_secs(15),
        ),
        Err(SupervisorError::RestartDenied)
    ));
    Ok(())
}

#[test]
fn windows_supervisor_cooperative_stop_uses_shared_health_grace() -> TestResult {
    let policy = RestartPolicy::bounded(1, Duration::from_millis(100), Duration::from_millis(100))?;
    let mut lifecycle = WindowsRestartLifecycle::new(policy, Duration::from_secs(5))?;
    let mut command = child_command()?;
    command.env("INTENT_WINDOWS_COOPERATIVE_STOP", "1");
    let authenticated = WindowsPendingWorker::spawn(
        &mut command,
        instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?
    .authenticate()?;

    let report = lifecycle.stop_after_auth(
        authenticated,
        HealthPolicy {
            stop_grace: Duration::from_secs(1),
            ..HealthPolicy::default()
        },
    )?;
    assert!(!report.escalated());
    assert!(report.status().success());
    assert_eq!(lifecycle.decision(), RestartDecision::Eligible);
    Ok(())
}

#[test]
fn windows_supervisor_escalates_uncooperative_stop_and_records_failure() -> TestResult {
    let policy = RestartPolicy::bounded(1, Duration::from_millis(100), Duration::from_millis(100))?;
    let mut lifecycle = WindowsRestartLifecycle::new(policy, Duration::from_secs(5))?;
    let mut command = child_command()?;
    command.env("INTENT_WINDOWS_IGNORE_STOP", "1");
    let authenticated = WindowsPendingWorker::spawn(
        &mut command,
        instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?
    .authenticate()?;

    let report = lifecycle.stop_after_auth(
        authenticated,
        HealthPolicy {
            stop_grace: Duration::from_millis(50),
            ..HealthPolicy::default()
        },
    )?;
    assert!(report.escalated());
    assert!(!report.status().success());
    assert!(matches!(
        lifecycle.decision(),
        RestartDecision::BackoffUntil(_)
    ));
    Ok(())
}
