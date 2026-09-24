#![cfg(target_os = "linux")]
#![forbid(unsafe_code)]

use intent_contracts::{AccountId, CancellationId, CapabilityId, ContentHash, TaskId, WorkerInstanceId};
use intent_local_transport::WorkerRole;
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

const STOP_GRACE: Duration = Duration::from_secs(1);

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

fn capability() -> CapabilityId {
    CapabilityId::from_uuid(Uuid::new_v4())
}

fn cancellation() -> CancellationId {
    CancellationId::from_uuid(Uuid::new_v4())
}

fn image() -> Result<ExecutableImage, Box<dyn Error>> {
    let path = Path::new(env!("CARGO_BIN_EXE_intent-fixture-worker"));
    let bytes = std::fs::read(path)?;
    let hash = ContentHash::from_bytes(Sha256::digest(bytes).into());
    Ok(ExecutableImage::load(path, hash)?)
}

fn process_limits() -> Result<ProcessLimits, SupervisorError> {
    ProcessLimits::new(128 * 1024 * 1024, 10, 64, 100)
}

fn config(scope: WorkerScope, capability: CapabilityId) -> Result<WorkerConfig, SupervisorError> {
    WorkerConfig::new(
        scope,
        WorkerRole::FixtureWorker,
        [capability],
        Priority::Background,
        process_limits()?,
        HealthPolicy {
            stop_grace: STOP_GRACE,
            terminate_grace: Duration::from_secs(1),
            ..HealthPolicy::default()
        },
        Duration::from_secs(20),
        RestartPolicy::never(),
        ExecutionBoundary::CooperativeLocal,
    )
}

fn supervisor() -> Result<Supervisor, SupervisorError> {
    Supervisor::new(
        &std::env::temp_dir(),
        AdmissionLimits::new(2, 2 * 128 * 1024 * 1024, 200, [0; 2], [0; 2], [0; 2])?,
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

fn ready(supervisor: &mut Supervisor, id: WorkerInstanceId) -> Result<(), Box<dyn Error>> {
    let snapshot = until(supervisor, id, Duration::from_secs(5), |snapshot| {
        matches!(snapshot.state, WorkerState::Ready | WorkerState::Failed)
    })?;
    if snapshot.state != WorkerState::Ready {
        return Err(format!("worker not ready: {snapshot:?}").into());
    }
    Ok(())
}

#[test]
fn task_stop_control_is_acknowledged_within_configured_grace_under_progress_backpressure()
-> TestResult {
    let markers = Markers(
        std::env::temp_dir().join(format!("intent-stop-control-latency-{}", Uuid::new_v4())),
    );
    std::fs::create_dir(&markers.0)?;
    let start = markers.0.join("start");
    let reports = [markers.0.join("first"), markers.0.join("second")];

    let mut supervisor = supervisor()?;
    let shared_scope = scope();
    let capability = capability();
    let approved = image()?;
    let mut workers = Vec::new();
    for report in &reports {
        workers.push(supervisor.launch(
            approved.clone(),
            config(shared_scope, capability)?,
            &[
                "progress-pressure".to_owned(),
                start.to_str().ok_or("non-UTF8 start path")?.to_owned(),
                report.to_str().ok_or("non-UTF8 report path")?.to_owned(),
            ],
        )?);
    }
    for &worker in &workers {
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
    let receipts = supervisor.cancel_task(shared_scope.task_id, cancellation());
    let revoked_after = started.elapsed();
    assert_eq!(receipts.len(), workers.len());
    assert!(leases.iter().all(|lease| lease.is_revoked()));
    assert!(revoked_after < Duration::from_millis(250));

    let acknowledgement_deadline = started + STOP_GRACE;
    let mut acknowledged = vec![false; workers.len()];
    while acknowledged.iter().any(|value| !value) {
        supervisor.poll();
        for (index, &worker) in workers.iter().enumerate() {
            if acknowledged[index] {
                continue;
            }
            let snapshot = supervisor.snapshot(worker)?;
            if snapshot.cancellation_acknowledged {
                acknowledged[index] = true;
                std::fs::write(reports[index].with_extension("release"), [])?;
            } else if matches!(snapshot.state, WorkerState::Stopped | WorkerState::Failed) {
                return Err(format!(
                    "worker became terminal before cancellation acknowledgement: {snapshot:?}"
                )
                .into());
            }
        }
        if acknowledged.iter().any(|value| !value) && Instant::now() >= acknowledgement_deadline {
            return Err(format!(
                "cancellation acknowledgement exceeded configured stop grace of {:?}",
                STOP_GRACE
            )
            .into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let acknowledged_after = started.elapsed();
    assert!(acknowledged_after < STOP_GRACE);

    for &worker in &workers {
        let stopped = until(&mut supervisor, worker, Duration::from_secs(5), |snapshot| {
            matches!(snapshot.state, WorkerState::Stopped | WorkerState::Failed)
        })?;
        assert_eq!(stopped.state, WorkerState::Stopped);
        assert_eq!(stopped.exit_code, Some(0));
        assert!(!stopped.stop_escalated);
    }

    println!(
        "{}",
        serde_json::json!({
            "scenario": "two_real_workers_under_measured_progress_backpressure",
            "configured_stop_grace_micros": STOP_GRACE.as_micros(),
            "local_revocation_micros": revoked_after.as_micros(),
            "both_cancel_acknowledged_micros": acknowledged_after.as_micros(),
            "admitted_progress_frames": admitted,
            "cooperative_stop_without_escalation": true,
            "scheduler_tick_aggregation_qualified": false,
            "wall_clock_suspend_qualified": false,
            "external_effect_reconciliation_qualified": false
        })
    );
    Ok(())
}
