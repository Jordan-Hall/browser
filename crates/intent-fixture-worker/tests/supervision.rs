#![cfg(target_os = "linux")]
#![forbid(unsafe_code)]

use intent_contracts::{
    AccountId, BoundedText, CancellationId, CapabilityId, ContentHash, RequestId, TaskId,
    WorkerInstanceId,
};
use intent_local_transport::{MessageFamily, WorkerRole};
use intent_supervisor::{
    AdmissionLimits, ExecutableImage, ExecutionBoundary, HealthPolicy, Priority, ProcessLimits,
    RestartDecision, RestartPolicy, SchedulerLimits, Supervisor, SupervisorError, WorkerConfig,
    WorkerFailure, WorkerScope, WorkerSnapshot, WorkerState,
};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    path::Path,
    time::{Duration, Instant},
};
use uuid::Uuid;

type TestResult = Result<(), Box<dyn Error>>;
struct Markers(std::path::PathBuf);
impl Drop for Markers {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn scope() -> WorkerScope {
    WorkerScope {
        account_id: AccountId::from_uuid(Uuid::new_v4()),
        task_id: TaskId::from_uuid(Uuid::new_v4()),
    }
}
fn cap() -> CapabilityId {
    CapabilityId::from_uuid(Uuid::new_v4())
}
fn request() -> RequestId {
    RequestId::from_uuid(Uuid::new_v4())
}
fn cancel_id() -> CancellationId {
    CancellationId::from_uuid(Uuid::new_v4())
}
fn image() -> Result<ExecutableImage, Box<dyn Error>> {
    let path = Path::new(env!("CARGO_BIN_EXE_intent-fixture-worker"));
    let hash = ContentHash::from_bytes(Sha256::digest(std::fs::read(path)?).into());
    Ok(ExecutableImage::load(path, hash)?)
}
fn limits() -> Result<ProcessLimits, SupervisorError> {
    ProcessLimits::new(128 * 1024 * 1024, 10, 64, 100)
}
fn config(scope: WorkerScope, capability: CapabilityId) -> Result<WorkerConfig, SupervisorError> {
    WorkerConfig::new(
        scope,
        WorkerRole::FixtureWorker,
        [capability],
        Priority::Background,
        limits()?,
        HealthPolicy::default(),
        Duration::from_secs(20),
        RestartPolicy::never(),
        ExecutionBoundary::CooperativeLocal,
    )
}
fn supervisor(workers: usize) -> Result<Supervisor, SupervisorError> {
    Supervisor::new(
        &std::env::temp_dir(),
        AdmissionLimits::new(
            workers,
            workers as u64 * 128 * 1024 * 1024,
            workers as u64 * 100,
            [0; 2],
            [0; 2],
            [0; 2],
        )?,
        SchedulerLimits::default(),
    )
}
fn observed_stop_config(
    scope: WorkerScope,
    capability: CapabilityId,
) -> Result<WorkerConfig, SupervisorError> {
    WorkerConfig::new(
        scope,
        WorkerRole::FixtureWorker,
        [capability],
        Priority::Background,
        limits()?,
        HealthPolicy {
            stop_grace: Duration::from_secs(5),
            ..HealthPolicy::default()
        },
        Duration::from_secs(20),
        RestartPolicy::never(),
        ExecutionBoundary::CooperativeLocal,
    )
}
fn until(
    supervisor: &mut Supervisor,
    id: WorkerInstanceId,
    timeout: Duration,
    predicate: impl Fn(&WorkerSnapshot) -> bool,
) -> Result<WorkerSnapshot, Box<dyn Error>> {
    let deadline = Instant::now() + timeout;
    loop {
        supervisor.poll();
        let snapshot = supervisor.snapshot(id)?;
        if predicate(&snapshot) {
            return Ok(snapshot);
        }
        if Instant::now() >= deadline {
            return Err(format!("worker wait expired: {snapshot:?}").into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn ready(
    supervisor: &mut Supervisor,
    id: WorkerInstanceId,
) -> Result<WorkerSnapshot, Box<dyn Error>> {
    let snapshot = until(supervisor, id, Duration::from_secs(5), |s| {
        matches!(s.state, WorkerState::Ready | WorkerState::Failed)
    })?;
    if snapshot.state != WorkerState::Ready {
        return Err(format!("not ready: {snapshot:?}").into());
    }
    Ok(snapshot)
}
fn terminal(
    supervisor: &mut Supervisor,
    id: WorkerInstanceId,
) -> Result<WorkerSnapshot, Box<dyn Error>> {
    until(supervisor, id, Duration::from_secs(5), |s| {
        matches!(s.state, WorkerState::Stopped | WorkerState::Failed)
    })
}
fn submit(
    supervisor: &mut Supervisor,
    id: WorkerInstanceId,
    capability: CapabilityId,
    ttl: Duration,
) -> Result<RequestId, Box<dyn Error>> {
    let lease = supervisor.lease(id)?;
    let request = request();
    let permit = lease.admit(
        request,
        lease.scope(),
        capability,
        MessageFamily::LifecycleControl,
        Instant::now() + ttl,
    )?;
    supervisor.submit(permit, BoundedText::try_new("fixture observation only")?)?;
    Ok(request)
}

#[test]
fn actual_child_negotiates_readiness_and_roundtrips_scoped_deadlined_work() -> TestResult {
    let mut supervisor = supervisor(1)?;
    let capability = cap();
    let id = supervisor.launch(image()?, config(scope(), capability)?, &[])?;
    assert!(supervisor.lease(id).is_err());
    let snapshot = ready(&mut supervisor, id)?;
    assert_ne!(snapshot.process_id, std::process::id());
    assert_eq!(
        snapshot.selected_schema,
        Some(intent_contracts::SchemaVersion::V1)
    );
    let request = submit(&mut supervisor, id, capability, Duration::from_secs(2))?;
    until(&mut supervisor, id, Duration::from_secs(3), |s| {
        s.retained_observations == 1
    })?;
    let result = supervisor
        .take_result(id, request)?
        .ok_or("missing observation")?;
    assert_eq!(result.request_id, request);
    assert_eq!(result.generation, id);
    supervisor.cancel(id, cancel_id())?;
    terminal(&mut supervisor, id)?;
    assert!(supervisor.retire(id)?.unresolved_requests.is_empty());
    Ok(())
}

#[test]
fn wrong_bootstrap_role_generation_and_replayed_hello_never_gain_authority() -> TestResult {
    for mode in ["wrong-token", "wrong-role", "wrong-generation", "replay"] {
        let mut supervisor = supervisor(1)?;
        let id = supervisor.launch(image()?, config(scope(), cap())?, &[mode.to_owned()])?;
        let terminal = terminal(&mut supervisor, id)?;
        assert_eq!(terminal.state, WorkerState::Failed, "{mode}");
        assert!(supervisor.lease(id).is_err());
        assert_eq!(terminal.pending_requests, 0);
    }
    Ok(())
}

#[test]
fn a_different_child_cannot_use_the_intended_childs_bootstrap() -> TestResult {
    let mut supervisor = supervisor(1)?;
    let capability = cap();
    let health = HealthPolicy {
        handshake_timeout: Duration::from_millis(250),
        ..HealthPolicy::default()
    };
    let cfg = WorkerConfig::new(
        scope(),
        WorkerRole::FixtureWorker,
        [capability],
        Priority::Background,
        limits()?,
        health,
        Duration::from_secs(10),
        RestartPolicy::never(),
        ExecutionBoundary::CooperativeLocal,
    )?;
    let id = supervisor.launch(image()?, cfg, &["wrong-peer".to_owned()])?;
    let snapshot = terminal(&mut supervisor, id)?;
    assert_eq!(snapshot.failure, Some(WorkerFailure::HandshakeTimeout));
    assert!(snapshot.rejected_peers >= 1);
    assert!(supervisor.lease(id).is_err());
    Ok(())
}

fn rejected_foreign_peer(mode: &str) -> TestResult {
    use std::os::unix::fs::DirBuilderExt;

    let markers = Markers(std::env::temp_dir().join(format!("foreign-peer-{}", Uuid::new_v4())));
    std::fs::DirBuilder::new().mode(0o700).create(&markers.0)?;
    let foreign_pid_path = markers.0.join("foreign-pid");
    let mut supervisor = supervisor(1)?;
    let capability = cap();
    let id = supervisor.launch(
        image()?,
        config(scope(), capability)?,
        &[
            mode.to_owned(),
            foreign_pid_path
                .to_str()
                .ok_or("non-UTF8 marker path")?
                .to_owned(),
        ],
    )?;
    assert!(supervisor.lease(id).is_err());
    let snapshot = ready(&mut supervisor, id)?;
    let foreign_pid: u32 = std::fs::read_to_string(&foreign_pid_path)?.parse()?;
    assert_ne!(foreign_pid, snapshot.process_id);
    assert_ne!(snapshot.process_id, std::process::id());
    assert!(snapshot.rejected_peers >= 1);
    let lease = supervisor.lease(id)?;
    let request = submit(&mut supervisor, id, capability, Duration::from_secs(2))?;
    until(&mut supervisor, id, Duration::from_secs(3), |snapshot| {
        snapshot.retained_observations == 1
    })?;
    let result = supervisor
        .take_result(id, request)?
        .ok_or("missing scoped result")?;
    assert_eq!(result.generation, id);
    assert_eq!(result.request_id, request);
    supervisor.cancel(id, cancel_id())?;
    assert!(lease.is_revoked());
    terminal(&mut supervisor, id)?;
    assert!(supervisor.retire(id)?.unresolved_requests.is_empty());
    Ok(())
}

#[test]
fn a_rejected_foreign_peer_does_not_consume_the_intended_childs_bootstrap() -> TestResult {
    rejected_foreign_peer("wrong-peer-then-owner")
}

#[test]
fn a_silent_foreign_peer_is_rejected_before_reading_hello() -> TestResult {
    rejected_foreign_peer("silent-peer-then-owner")
}

fn foreign_worker_message(mode: &str) -> TestResult {
    let mut supervisor = supervisor(2)?;
    let capability = cap();
    let healthy = supervisor.launch(image()?, config(scope(), capability)?, &[])?;
    ready(&mut supervisor, healthy)?;
    let healthy_lease = supervisor.lease(healthy)?;
    let hostile = supervisor.launch(
        image()?,
        config(scope(), cap())?,
        &[mode.to_owned(), healthy.to_string()],
    )?;
    let failed = terminal(&mut supervisor, hostile)?;
    assert_eq!(failed.failure, Some(WorkerFailure::ProtocolViolation));
    assert!(supervisor.lease(hostile).is_err());
    assert!(!healthy_lease.is_revoked());
    assert_eq!(supervisor.snapshot(healthy)?.state, WorkerState::Ready);
    let request = submit(&mut supervisor, healthy, capability, Duration::from_secs(2))?;
    until(
        &mut supervisor,
        healthy,
        Duration::from_secs(3),
        |snapshot| snapshot.retained_observations == 1,
    )?;
    let result = supervisor
        .take_result(healthy, request)?
        .ok_or("missing healthy result")?;
    assert_eq!(result.generation, healthy);
    assert_eq!(result.request_id, request);
    supervisor.cancel(healthy, cancel_id())?;
    assert!(healthy_lease.is_revoked());
    terminal(&mut supervisor, healthy)?;
    assert!(supervisor.retire(hostile)?.unresolved_requests.is_empty());
    assert!(supervisor.retire(healthy)?.unresolved_requests.is_empty());
    Ok(())
}

#[test]
fn a_worker_cannot_cancel_an_independent_generation() -> TestResult {
    foreign_worker_message("foreign-lifecycle")
}

#[test]
fn a_worker_cannot_send_a_heartbeat_for_an_independent_generation() -> TestResult {
    foreign_worker_message("foreign-heartbeat")
}

#[test]
fn executable_replacement_cannot_change_a_sealed_approved_launch() -> TestResult {
    let bytes = std::fs::read(env!("CARGO_BIN_EXE_intent-fixture-worker"))?;
    let path = std::env::temp_dir().join(format!("intent-image-{}", Uuid::new_v4()));
    std::fs::write(&path, &bytes)?;
    let hash = ContentHash::from_bytes(Sha256::digest(&bytes).into());
    let approved = ExecutableImage::load(&path, hash)?;
    std::fs::write(&path, b"replacement is not the approved ELF")?;
    assert!(matches!(
        ExecutableImage::load(&path, hash),
        Err(SupervisorError::ExecutableMismatch)
    ));
    std::fs::remove_file(path)?;
    let mut supervisor = supervisor(1)?;
    let id = supervisor.launch(approved, config(scope(), cap())?, &[])?;
    assert_eq!(ready(&mut supervisor, id)?.executable_hash, hash);
    supervisor.cancel(id, cancel_id())?;
    terminal(&mut supervisor, id)?;
    Ok(())
}

#[test]
fn launch_timeout_and_missed_heartbeat_are_distinct_from_os_exit() -> TestResult {
    for (mode, reason) in [
        ("silent", WorkerFailure::HandshakeTimeout),
        ("silent-after-ready", WorkerFailure::HeartbeatTimeout),
    ] {
        let health = HealthPolicy {
            handshake_timeout: Duration::from_millis(300),
            heartbeat_timeout: Duration::from_millis(100),
            ..HealthPolicy::default()
        };
        let cfg = WorkerConfig::new(
            scope(),
            WorkerRole::FixtureWorker,
            [cap()],
            Priority::Background,
            limits()?,
            health,
            Duration::from_secs(10),
            RestartPolicy::never(),
            ExecutionBoundary::CooperativeLocal,
        )?;
        let mut supervisor = supervisor(1)?;
        let id = supervisor.launch(image()?, cfg, &[mode.to_owned()])?;
        assert_eq!(terminal(&mut supervisor, id)?.failure, Some(reason));
    }
    let mut supervisor = supervisor(1)?;
    let id = supervisor.launch(image()?, config(scope(), cap())?, &["exit".to_owned()])?;
    let exited = terminal(&mut supervisor, id)?;
    assert!(matches!(
        exited.failure,
        Some(WorkerFailure::OsExit | WorkerFailure::ControlClosed)
    ));
    assert_eq!(exited.exit_code, Some(23));
    Ok(())
}

#[test]
fn heartbeats_do_not_mask_stalled_work() -> TestResult {
    let health = HealthPolicy {
        work_progress_timeout: Duration::from_millis(100),
        heartbeat_timeout: Duration::from_secs(2),
        ..HealthPolicy::default()
    };
    let capability = cap();
    let cfg = WorkerConfig::new(
        scope(),
        WorkerRole::FixtureWorker,
        [capability],
        Priority::Background,
        limits()?,
        health,
        Duration::from_secs(10),
        RestartPolicy::never(),
        ExecutionBoundary::CooperativeLocal,
    )?;
    let mut supervisor = supervisor(1)?;
    let id = supervisor.launch(image()?, cfg, &["stalled-work".to_owned()])?;
    ready(&mut supervisor, id)?;
    let request = submit(&mut supervisor, id, capability, Duration::from_secs(3))?;
    assert_eq!(
        terminal(&mut supervisor, id)?.failure,
        Some(WorkerFailure::ProgressTimeout)
    );
    assert_eq!(supervisor.retire(id)?.unresolved_requests, vec![request]);
    Ok(())
}

#[test]
fn stop_revokes_both_worker_generations_without_waiting_for_saturated_progress() -> TestResult {
    let markers =
        Markers(std::env::temp_dir().join(format!("intent-flood-stop-{}", Uuid::new_v4())));
    std::fs::create_dir(&markers.0)?;
    let release = markers.0.join("release");
    let args = [
        "flood".to_owned(),
        release.to_str().ok_or("non-UTF8 release path")?.to_owned(),
    ];
    let mut supervisor = supervisor(3)?;
    let shared = scope();
    let capability = cap();
    let approved = image()?;
    let first = supervisor.launch(
        approved.clone(),
        observed_stop_config(shared, capability)?,
        &args,
    )?;
    let second = supervisor.launch(
        approved.clone(),
        observed_stop_config(shared, capability)?,
        &args,
    )?;
    let other = supervisor.launch(approved, config(scope(), capability)?, &[])?;
    ready(&mut supervisor, first)?;
    ready(&mut supervisor, second)?;
    ready(&mut supervisor, other)?;
    let first_lease = supervisor.lease(first)?;
    let second_lease = supervisor.lease(second)?;
    std::thread::sleep(Duration::from_millis(50));
    let started = Instant::now();
    let receipts = supervisor.cancel_task(shared.task_id, cancel_id());
    let revocation_elapsed = started.elapsed();
    assert!(revocation_elapsed < Duration::from_millis(250));
    assert_eq!(receipts.len(), 2);
    assert!(first_lease.is_revoked());
    assert!(second_lease.is_revoked());
    assert!(!supervisor.lease(other)?.is_revoked());
    for worker in [first, second] {
        until(
            &mut supervisor,
            worker,
            Duration::from_secs(2),
            |snapshot| snapshot.cancellation_acknowledged,
        )?;
    }
    std::fs::write(release, [])?;
    let first_snapshot = terminal(&mut supervisor, first)?;
    let second_snapshot = terminal(&mut supervisor, second)?;
    assert!(first_snapshot.cancellation_acknowledged);
    assert!(second_snapshot.cancellation_acknowledged);
    println!(
        "{}",
        serde_json::json!({
            "scenario": "two_progress_floods_and_an_independent_ready_worker",
            "local_revoke_microseconds": revocation_elapsed.as_micros(),
            "both_reaped_microseconds": started.elapsed().as_micros(),
            "both_cancel_acknowledged": true,
            "production_latency_qualification": false
        })
    );
    assert_eq!(supervisor.snapshot(other)?.state, WorkerState::Ready);
    supervisor.cancel(other, cancel_id())?;
    terminal(&mut supervisor, other)?;
    Ok(())
}

#[test]
fn cancellation_acknowledgements_precede_draining_measured_progress_backlogs() -> TestResult {
    const MAX_LOCALLY_QUEUED_PROGRESS_FRAMES: u64 = 1;
    let markers =
        Markers(std::env::temp_dir().join(format!("intent-progress-pressure-{}", Uuid::new_v4())));
    std::fs::create_dir(&markers.0)?;
    let start = markers.0.join("start");
    let reports = [markers.0.join("first"), markers.0.join("second")];
    let mut supervisor = supervisor(3)?;
    let shared = scope();
    let capability = cap();
    let approved = image()?;
    let pressure_config = observed_stop_config(shared, capability)?;
    let mut workers = Vec::new();
    for report in &reports {
        workers.push(supervisor.launch(
            approved.clone(),
            pressure_config.clone(),
            &[
                "progress-pressure".to_owned(),
                start.to_str().ok_or("non-UTF8 start path")?.to_owned(),
                report.to_str().ok_or("non-UTF8 report path")?.to_owned(),
            ],
        )?);
    }
    let other = supervisor.launch(approved, config(scope(), capability)?, &[])?;
    for &worker in workers.iter().chain(std::iter::once(&other)) {
        ready(&mut supervisor, worker)?;
    }
    let leases = workers
        .iter()
        .map(|&worker| supervisor.lease(worker))
        .collect::<Result<Vec<_>, _>>()?;
    std::fs::write(&start, [])?;
    let pressure_deadline = Instant::now() + Duration::from_secs(3);
    while !reports.iter().all(|report| report.is_file()) {
        if Instant::now() >= pressure_deadline {
            return Err("workers did not report progress backpressure".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let admitted = reports
        .iter()
        .map(|report| Ok(std::fs::read_to_string(report)?.parse::<u64>()?))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    assert!(admitted.iter().all(|&count| count > 1));

    let started = Instant::now();
    let receipts = supervisor.cancel_task(shared.task_id, cancel_id());
    let revoked_after = started.elapsed();
    assert_eq!(receipts.len(), 2);
    assert!(leases.iter().all(|lease| lease.is_revoked()));
    assert!(!supervisor.lease(other)?.is_revoked());
    assert!(revoked_after < Duration::from_millis(250));

    supervisor.poll();
    std::thread::sleep(Duration::from_millis(30));
    let mut acknowledgements = [None, None];
    while acknowledgements.iter().any(Option::is_none) {
        supervisor.poll();
        for (index, &worker) in workers.iter().enumerate() {
            let snapshot = supervisor.snapshot(worker)?;
            if !snapshot.cancellation_acknowledged
                && matches!(snapshot.state, WorkerState::Stopped | WorkerState::Failed)
            {
                return Err(format!(
                    "worker exited before acknowledgement was observed: {snapshot:?}"
                )
                .into());
            }
            if acknowledgements[index].is_none() && snapshot.cancellation_acknowledged {
                assert!(
                    snapshot.late_messages + MAX_LOCALLY_QUEUED_PROGRESS_FRAMES < admitted[index],
                    "worker drained progress before acknowledging cancellation: {snapshot:?}; admitted={}",
                    admitted[index]
                );
                acknowledgements[index] = Some(snapshot.late_messages);
                std::fs::write(reports[index].with_extension("release"), [])?;
            }
        }
        if started.elapsed() >= Duration::from_secs(2) {
            return Err(
                "cancellation acknowledgements timed out under measured backpressure".into(),
            );
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let acknowledged_after = started.elapsed();
    for &worker in &workers {
        let stopped = terminal(&mut supervisor, worker)?;
        assert_eq!(stopped.state, WorkerState::Stopped);
        assert_eq!(stopped.exit_code, Some(0));
        assert!(!stopped.stop_escalated);
    }
    let request = submit(&mut supervisor, other, capability, Duration::from_secs(2))?;
    until(&mut supervisor, other, Duration::from_secs(3), |snapshot| {
        snapshot.retained_observations == 1
    })?;
    let result = supervisor
        .take_result(other, request)?
        .ok_or("missing independent result")?;
    assert_eq!(result.request_id, request);
    assert_eq!(result.generation, other);
    supervisor.cancel(other, cancel_id())?;
    terminal(&mut supervisor, other)?;
    println!(
        "{}",
        serde_json::json!({
            "scenario": "two_measured_progress_backlogs",
            "admitted_frames": admitted,
            "drained_frames_at_ack": acknowledgements,
            "local_revoke_microseconds": revoked_after.as_micros(),
            "both_acknowledged_microseconds": acknowledged_after.as_micros(),
            "independent_worker_completed": true,
            "production_latency_qualification": false
        })
    );
    Ok(())
}

#[test]
fn queued_work_and_late_results_cannot_cross_revocation() -> TestResult {
    let mut supervisor = supervisor(1)?;
    let capability = cap();
    let id = supervisor.launch(
        image()?,
        config(scope(), capability)?,
        &["late-result".to_owned()],
    )?;
    ready(&mut supervisor, id)?;
    let request = submit(&mut supervisor, id, capability, Duration::from_secs(2))?;
    supervisor.poll();
    std::thread::sleep(Duration::from_millis(30));
    let receipt = supervisor.cancel(id, cancel_id())?;
    assert!(receipt.newly_revoked);
    assert!(!supervisor.cancel(id, cancel_id())?.newly_revoked);
    let stopped = terminal(&mut supervisor, id)?;
    assert!(stopped.late_messages >= 1);
    assert_eq!(stopped.retained_observations, 0);
    assert_eq!(supervisor.retire(id)?.unresolved_requests, vec![request]);
    Ok(())
}

#[test]
fn expired_queued_request_is_never_sent() -> TestResult {
    let mut supervisor = supervisor(1)?;
    let capability = cap();
    let id = supervisor.launch(image()?, config(scope(), capability)?, &[])?;
    ready(&mut supervisor, id)?;
    let request = submit(&mut supervisor, id, capability, Duration::from_millis(10))?;
    std::thread::sleep(Duration::from_millis(20));
    let expired = terminal(&mut supervisor, id)?;
    assert_eq!(expired.failure, Some(WorkerFailure::DeadlineExpired));
    assert_eq!(expired.retained_observations, 0);
    assert_eq!(supervisor.retire(id)?.unresolved_requests, vec![request]);
    Ok(())
}

#[test]
fn malformed_response_does_not_discharge_an_unresolved_request() -> TestResult {
    let mut supervisor = supervisor(1)?;
    let capability = cap();
    let id = supervisor.launch(
        image()?,
        config(scope(), capability)?,
        &["bad-response".to_owned()],
    )?;
    ready(&mut supervisor, id)?;
    let request = submit(&mut supervisor, id, capability, Duration::from_secs(2))?;
    assert_eq!(
        terminal(&mut supervisor, id)?.failure,
        Some(WorkerFailure::ProtocolViolation)
    );
    assert_eq!(supervisor.retire(id)?.unresolved_requests, vec![request]);
    Ok(())
}

#[test]
fn restart_is_bounded_uses_a_fresh_epoch_and_preserves_revocation() -> TestResult {
    let capability = cap();
    let policy = RestartPolicy::bounded(1, Duration::from_millis(20), Duration::from_millis(20))?;
    let cfg = WorkerConfig::new(
        scope(),
        WorkerRole::FixtureWorker,
        [capability],
        Priority::Background,
        limits()?,
        HealthPolicy::default(),
        Duration::from_secs(10),
        policy,
        ExecutionBoundary::CooperativeLocal,
    )?;
    let mut supervisor = supervisor(1)?;
    let first = supervisor.launch(image()?, cfg, &["exit-delayed".to_owned()])?;
    ready(&mut supervisor, first)?;
    let old_lease = supervisor.lease(first)?;
    terminal(&mut supervisor, first)?;
    assert!(matches!(
        supervisor.restart(first),
        Err(SupervisorError::RestartDenied)
    ));
    std::thread::sleep(Duration::from_millis(25));
    let second = supervisor.restart(first)?;
    assert_ne!(first, second);
    ready(&mut supervisor, second)?;
    assert!(old_lease.is_revoked());
    assert!(matches!(
        old_lease.admit(
            request(),
            old_lease.scope(),
            capability,
            MessageFamily::LifecycleControl,
            Instant::now() + Duration::from_secs(1)
        ),
        Err(SupervisorError::Revoked)
    ));
    terminal(&mut supervisor, second)?;
    assert_eq!(
        supervisor.restart_decision(second)?,
        RestartDecision::CircuitOpen
    );
    assert!(supervisor.restart(second).is_err());
    Ok(())
}

#[test]
fn retired_generations_do_not_exhaust_a_long_running_registry() -> TestResult {
    let mut supervisor = supervisor(1)?;
    let image = image()?;
    let cfg = config(scope(), cap())?;
    for _ in 0..12 {
        let id = supervisor.launch(image.clone(), cfg.clone(), &[])?;
        ready(&mut supervisor, id)?;
        let lease = supervisor.lease(id)?;
        supervisor.cancel(id, cancel_id())?;
        terminal(&mut supervisor, id)?;
        supervisor.retire(id)?;
        assert!(lease.is_revoked());
        assert!(supervisor.snapshot(id).is_err());
        assert_eq!(supervisor.poll().active_workers, 0);
    }
    Ok(())
}

#[test]
fn address_space_and_descriptor_limits_are_installed_before_worker_code() -> TestResult {
    for mode in ["memory-probe", "fd-probe", "environment-probe"] {
        let mut supervisor = supervisor(1)?;
        let capability = cap();
        let id = supervisor.launch(image()?, config(scope(), capability)?, &[mode.to_owned()])?;
        let snapshot = ready(&mut supervisor, id)?;
        let status = std::fs::read_to_string(format!("/proc/{}/status", snapshot.process_id))?;
        assert!(status.lines().any(|line| line == "NoNewPrivs:\t1"));
        let os_limits = std::fs::read_to_string(format!("/proc/{}/limits", snapshot.process_id))?;
        assert!(
            os_limits
                .lines()
                .any(|line| line.starts_with("Max address space") && line.contains("134217728"))
        );
        assert!(
            os_limits
                .lines()
                .any(|line| line.starts_with("Max open files") && line.contains("64"))
        );
        let request = submit(&mut supervisor, id, capability, Duration::from_secs(2))?;
        until(&mut supervisor, id, Duration::from_secs(3), |s| {
            s.retained_observations == 1
        })?;
        assert!(supervisor.take_result(id, request)?.is_some());
        supervisor.cancel(id, cancel_id())?;
        terminal(&mut supervisor, id)?;
    }
    Ok(())
}

#[test]
fn cpu_abuse_is_terminated_by_the_hard_kernel_limit_not_heartbeat_policy() -> TestResult {
    let health = HealthPolicy {
        heartbeat_timeout: Duration::from_secs(60),
        ..HealthPolicy::default()
    };
    let cpu_limit = ProcessLimits::new(128 * 1024 * 1024, 1, 64, 100)?;
    let cfg = WorkerConfig::new(
        scope(),
        WorkerRole::FixtureWorker,
        [cap()],
        Priority::Background,
        cpu_limit,
        health,
        Duration::from_secs(10),
        RestartPolicy::never(),
        ExecutionBoundary::CooperativeLocal,
    )?;
    let mut supervisor = supervisor(1)?;
    let id = supervisor.launch(image()?, cfg, &["cpu-hog".to_owned()])?;
    ready(&mut supervisor, id)?;
    let stopped = terminal(&mut supervisor, id)?;
    assert_eq!(stopped.exit_signal, Some(9));
    assert!(!stopped.stop_escalated);
    Ok(())
}

#[test]
fn yield_is_coalesced_and_resource_samples_are_content_free() -> TestResult {
    let mut supervisor = supervisor(1)?;
    let id = supervisor.launch(image()?, config(scope(), cap())?, &[])?;
    ready(&mut supervisor, id)?;
    assert!(supervisor.request_yield(id)?);
    assert!(!supervisor.request_yield(id)?);
    until(&mut supervisor, id, Duration::from_secs(2), |s| {
        s.yield_acknowledged
    })?;
    let deadline = Instant::now() + Duration::from_secs(2);
    while supervisor.observation(id).is_none() {
        supervisor.poll();
        if Instant::now() > deadline {
            return Err("no resource sample".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(
        supervisor
            .observation(id)
            .is_some_and(|sample| sample.resident_bytes > 0
                && sample.virtual_bytes.is_some_and(|bytes| bytes > 0))
    );
    supervisor.cancel(id, cancel_id())?;
    terminal(&mut supervisor, id)?;
    Ok(())
}

#[test]
fn unqualified_unattended_containment_is_rejected_instead_of_silently_downgraded() -> TestResult {
    let result = WorkerConfig::new(
        scope(),
        WorkerRole::ConnectorHost,
        [cap()],
        Priority::Background,
        limits()?,
        HealthPolicy::default(),
        Duration::from_secs(10),
        RestartPolicy::never(),
        ExecutionBoundary::UnattendedUntrusted,
    );
    assert!(matches!(result, Err(SupervisorError::UnsupportedBoundary)));
    assert!(ProcessLimits::new(0, 1, 64, 100).is_err());
    assert!(AdmissionLimits::new(1, 100, 100, [1, 1], [0, 0], [0, 0]).is_err());
    Ok(())
}

#[test]
fn cancellation_reaps_the_leader_and_terminates_cooperative_descendants() -> TestResult {
    let pid_file = std::env::temp_dir().join(format!("intent-descendant-{}", Uuid::new_v4()));
    let mut supervisor = supervisor(1)?;
    let id = supervisor.launch(
        image()?,
        config(scope(), cap())?,
        &["descendant".to_owned(), pid_file.display().to_string()],
    )?;
    ready(&mut supervisor, id)?;
    let deadline = Instant::now() + Duration::from_secs(2);
    while !pid_file.exists() {
        supervisor.poll();
        if Instant::now() >= deadline {
            return Err("missing descendant PID".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let pid: u32 = std::fs::read_to_string(&pid_file)?.parse()?;
    supervisor.cancel(id, cancel_id())?;
    let stopped = terminal(&mut supervisor, id)?;
    assert!(!Path::new(&format!("/proc/{}", stopped.process_id)).exists());
    assert!(!Path::new(&format!("/proc/{pid}")).exists());
    std::fs::remove_file(pid_file)?;
    Ok(())
}

#[test]
fn an_unresponsive_stop_escalates_without_blocking_other_workers() -> TestResult {
    let mut supervisor = supervisor(2)?;
    let capability = cap();
    let image = image()?;
    let stalled = supervisor.launch(
        image.clone(),
        config(scope(), capability)?,
        &["ignore-cancel".to_owned()],
    )?;
    let healthy = supervisor.launch(image, config(scope(), capability)?, &[])?;
    ready(&mut supervisor, stalled)?;
    ready(&mut supervisor, healthy)?;
    let lease = supervisor.lease(stalled)?;
    supervisor.cancel(stalled, cancel_id())?;
    assert!(lease.is_revoked());
    let request = submit(&mut supervisor, healthy, capability, Duration::from_secs(2))?;
    until(&mut supervisor, healthy, Duration::from_secs(3), |s| {
        s.retained_observations == 1
    })?;
    assert!(supervisor.take_result(healthy, request)?.is_some());
    let stopped = terminal(&mut supervisor, stalled)?;
    assert!(stopped.stop_escalated);
    assert!(matches!(stopped.exit_signal, Some(15 | 9)));
    assert!(!stopped.cancellation_acknowledged);
    supervisor.cancel(healthy, cancel_id())?;
    terminal(&mut supervisor, healthy)?;
    Ok(())
}

#[test]
fn suspended_worker_loses_its_lease_before_it_can_resume_work() -> TestResult {
    use nix::{
        sys::signal::{Signal, kill},
        unistd::Pid,
    };
    let capability = cap();
    let health = HealthPolicy {
        heartbeat_timeout: Duration::from_millis(100),
        stop_grace: Duration::from_millis(250),
        ..HealthPolicy::default()
    };
    let cfg = WorkerConfig::new(
        scope(),
        WorkerRole::FixtureWorker,
        [capability],
        Priority::Background,
        limits()?,
        health,
        Duration::from_secs(10),
        RestartPolicy::never(),
        ExecutionBoundary::CooperativeLocal,
    )?;
    let mut supervisor = supervisor(1)?;
    let id = supervisor.launch(image()?, cfg, &[])?;
    let snapshot = ready(&mut supervisor, id)?;
    let lease = supervisor.lease(id)?;
    kill(Pid::from_raw(snapshot.process_id as i32), Signal::SIGSTOP)?;
    until(&mut supervisor, id, Duration::from_secs(2), |s| {
        s.state == WorkerState::Draining
    })?;
    assert!(lease.is_revoked());
    kill(Pid::from_raw(snapshot.process_id as i32), Signal::SIGCONT)?;
    assert!(
        lease
            .admit(
                request(),
                lease.scope(),
                capability,
                MessageFamily::LifecycleControl,
                Instant::now() + Duration::from_secs(1)
            )
            .is_err()
    );
    assert_eq!(
        terminal(&mut supervisor, id)?.failure,
        Some(WorkerFailure::HeartbeatTimeout)
    );
    Ok(())
}

#[test]
fn embedding_application_descriptors_do_not_leak_across_exec() -> TestResult {
    use nix::fcntl::{FcntlArg, FdFlag, fcntl};
    let path = std::env::temp_dir().join(format!("intent-fd-probe-{}", Uuid::new_v4()));
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    fcntl(&file, FcntlArg::F_SETFD(FdFlag::empty()))?;
    let outcome = (|| -> TestResult {
        let mut supervisor = supervisor(1)?;
        let capability = cap();
        let id = supervisor.launch(
            image()?,
            config(scope(), capability)?,
            &[
                "inherited-fd-probe".to_owned(),
                path.to_string_lossy().into_owned(),
            ],
        )?;
        ready(&mut supervisor, id)?;
        let request = submit(&mut supervisor, id, capability, Duration::from_secs(2))?;
        until(&mut supervisor, id, Duration::from_secs(3), |s| {
            s.retained_observations == 1
        })?;
        assert!(supervisor.take_result(id, request)?.is_some());
        supervisor.cancel(id, cancel_id())?;
        terminal(&mut supervisor, id)?;
        Ok(())
    })();
    drop(file);
    std::fs::remove_file(path)?;
    outcome
}
