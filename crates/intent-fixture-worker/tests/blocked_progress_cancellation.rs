#![cfg(target_os = "linux")]
#![forbid(unsafe_code)]

use intent_contracts::{
    AccountId, BoundedText, CancellationId, CapabilityId, ContentHash, RequestId, TaskId,
    WorkerInstanceId,
};
use intent_local_transport::{MessageFamily, WorkerRole};
use intent_supervisor::{
    AdmissionLimits, ExecutableImage, ExecutionBoundary, HealthPolicy, Priority, ProcessLimits,
    RestartPolicy, SchedulerLimits, Supervisor, SupervisorError, WorkerConfig, WorkerScope,
    WorkerSnapshot, WorkerState,
};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    path::Path,
    time::{Duration, Instant},
};
use uuid::Uuid;

type TestResult = Result<(), Box<dyn Error>>;

fn scope() -> WorkerScope {
    WorkerScope {
        account_id: AccountId::from_uuid(Uuid::new_v4()),
        task_id: TaskId::from_uuid(Uuid::new_v4()),
    }
}

fn capability() -> CapabilityId {
    CapabilityId::from_uuid(Uuid::new_v4())
}

fn image() -> Result<ExecutableImage, Box<dyn Error>> {
    let path = Path::new(env!("CARGO_BIN_EXE_intent-blocked-progress-worker"));
    let hash = ContentHash::from_bytes(Sha256::digest(std::fs::read(path)?).into());
    Ok(ExecutableImage::load(path, hash)?)
}

fn config(
    scope: WorkerScope,
    capability: CapabilityId,
    health: HealthPolicy,
) -> Result<WorkerConfig, SupervisorError> {
    WorkerConfig::new(
        scope,
        WorkerRole::FixtureWorker,
        [capability],
        Priority::Background,
        ProcessLimits::new(128 * 1024 * 1024, 10, 64, 100)?,
        health,
        Duration::from_secs(10),
        RestartPolicy::never(),
        ExecutionBoundary::CooperativeLocal,
    )
}

fn supervisor() -> Result<Supervisor, SupervisorError> {
    Supervisor::new(
        &std::env::temp_dir(),
        AdmissionLimits::new(1, 128 * 1024 * 1024, 100, [0; 2], [0; 2], [0; 2])?,
        SchedulerLimits::default(),
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

#[test]
fn cancellation_crosses_control_while_real_progress_transport_is_backpressured() -> TestResult {
    let marker =
        std::env::temp_dir().join(format!("intent-progress-backpressure-{}", Uuid::new_v4()));
    let health = HealthPolicy {
        stop_grace: Duration::from_millis(750),
        terminate_grace: Duration::from_millis(750),
        ..HealthPolicy::default()
    };
    let scope = scope();
    let capability = capability();
    let mut supervisor = supervisor()?;
    let id = supervisor.launch(
        image()?,
        config(scope, capability, health)?,
        &[marker.to_string_lossy().into_owned()],
    )?;
    let ready = until(&mut supervisor, id, Duration::from_secs(5), |snapshot| {
        matches!(snapshot.state, WorkerState::Ready | WorkerState::Failed)
    })?;
    if ready.state != WorkerState::Ready {
        return Err(format!("worker did not become ready: {ready:?}").into());
    }

    let lease = supervisor.lease(id)?;
    let request = RequestId::from_uuid(Uuid::new_v4());
    let permit = lease.admit(
        request,
        lease.scope(),
        capability,
        MessageFamily::LifecycleControl,
        Instant::now() + Duration::from_secs(5),
    )?;
    supervisor.submit_immediate(permit, BoundedText::try_new("start progress flood")?)?;

    let blocked_deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match std::fs::read(&marker) {
            Ok(contents) if contents == b"progress transport backpressured" => break,
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        if Instant::now() >= blocked_deadline {
            return Err("worker progress socket marker did not publish complete backpressure state".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }

    let started = Instant::now();
    let receipt = supervisor.cancel(id, CancellationId::from_uuid(Uuid::new_v4()))?;
    assert!(receipt.newly_revoked);
    let stopped = until(&mut supervisor, id, Duration::from_secs(5), |snapshot| {
        matches!(snapshot.state, WorkerState::Stopped | WorkerState::Failed)
    })?;
    let elapsed = started.elapsed();
    assert!(stopped.cancellation_acknowledged);
    assert!(!stopped.stop_escalated);
    assert!(elapsed < health.stop_grace);
    assert_eq!(
        std::fs::read(&marker)?,
        b"cancel received while progress backpressured"
    );
    assert_eq!(supervisor.retire(id)?.unresolved_requests, vec![request]);
    std::fs::remove_file(&marker)?;

    println!(
        "{}",
        serde_json::json!({
            "scenario": "real_progress_socket_backpressure_with_independent_control_cancel",
            "cancel_to_reap_microseconds": elapsed.as_micros(),
            "cancel_acknowledged": true,
            "signal_escalation": false,
            "production_latency_qualification": false
        })
    );
    Ok(())
}
