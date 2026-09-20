#![cfg(target_os = "linux")]
#![forbid(unsafe_code)]

use intent_contracts::{AccountId, CancellationId, CapabilityId, ContentHash, TaskId};
use intent_local_transport::WorkerRole;
use intent_supervisor::{
    AdmissionLimits, ExecutableImage, ExecutionBoundary, HealthPolicy, Priority, ProcessLimits,
    RestartPolicy, SchedulerLimits, Supervisor, WorkerConfig, WorkerFailure, WorkerScope,
    WorkerState,
};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs::DirBuilder,
    os::unix::fs::DirBuilderExt,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use uuid::Uuid;

type TestResult = Result<(), Box<dyn Error>>;
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, Eq, PartialEq)]
enum StartupOutcome {
    Admitted,
    Expired,
    Revoked,
    Disconnected,
}

impl StartupOutcome {
    fn failure(self) -> Option<WorkerFailure> {
        match self {
            Self::Admitted | Self::Revoked => None,
            Self::Expired => Some(WorkerFailure::HandshakeTimeout),
            Self::Disconnected => Some(WorkerFailure::ControlClosed),
        }
    }
}

struct Markers(PathBuf);
impl Drop for Markers {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn gated_startup(gate_at: &str, expected: StartupOutcome) -> TestResult {
    let markers = Markers(std::env::temp_dir().join(format!("startup-{}", Uuid::new_v4())));
    DirBuilder::new().mode(0o700).create(&markers.0)?;
    let mut supervisor = Supervisor::new(
        &std::env::temp_dir(),
        AdmissionLimits::new(1, 128 * 1024 * 1024, 100, [0; 2], [0; 2], [0; 2])?,
        SchedulerLimits::default(),
    )?;
    let scope = WorkerScope {
        account_id: AccountId::from_uuid(Uuid::new_v4()),
        task_id: TaskId::from_uuid(Uuid::new_v4()),
    };
    let config = WorkerConfig::new(
        scope,
        WorkerRole::FixtureWorker,
        [CapabilityId::from_uuid(Uuid::new_v4())],
        Priority::Background,
        ProcessLimits::new(128 * 1024 * 1024, 10, 64, 100)?,
        HealthPolicy {
            handshake_timeout: HANDSHAKE_TIMEOUT,
            heartbeat_timeout: Duration::from_secs(10),
            work_progress_timeout: Duration::from_secs(10),
            ..HealthPolicy::default()
        },
        Duration::from_secs(20),
        RestartPolicy::never(),
        ExecutionBoundary::CooperativeLocal,
    )?;
    let path = Path::new(env!("CARGO_BIN_EXE_intent-fixture-worker"));
    let hash = ContentHash::from_bytes(Sha256::digest(std::fs::read(path)?).into());
    let image = ExecutableImage::load(path, hash)?;
    let id = supervisor.launch(
        image,
        config,
        &[
            "startup-gate".to_owned(),
            markers.0.to_str().ok_or("non-UTF8 marker path")?.to_owned(),
            gate_at.to_owned(),
        ],
    )?;
    let launched = Instant::now();
    while !markers.0.join("waiting").try_exists()? {
        supervisor.poll();
        let snapshot = supervisor.snapshot(id)?;
        assert_eq!(snapshot.state, WorkerState::Starting, "{snapshot:?}");
        if launched.elapsed() >= HANDSHAKE_TIMEOUT {
            return Err("fixture did not authenticate within startup interval".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let pid: u32 = std::fs::read_to_string(markers.0.join("waiting"))?.parse()?;
    assert_eq!(pid, supervisor.snapshot(id)?.process_id);
    assert_ne!(pid, std::process::id());
    assert_eq!(supervisor.snapshot(id)?.state, WorkerState::Starting);
    assert!(supervisor.lease(id).is_err());
    assert_eq!(
        markers.0.join("control-authenticated").try_exists()?,
        gate_at != "control"
    );
    assert_eq!(
        markers.0.join("progress-authenticated").try_exists()?,
        gate_at == "ready"
    );
    if expected == StartupOutcome::Expired {
        let release_at = launched + HANDSHAKE_TIMEOUT + Duration::from_millis(30);
        std::thread::sleep(release_at.saturating_duration_since(Instant::now()));
    }
    if expected == StartupOutcome::Revoked {
        supervisor.cancel(id, CancellationId::from_uuid(Uuid::new_v4()))?;
        assert_eq!(supervisor.snapshot(id)?.state, WorkerState::Draining);
        assert!(supervisor.lease(id).is_err());
    }
    std::fs::write(markers.0.join("release"), b"release")?;
    let sent_deadline = Instant::now() + Duration::from_secs(2);
    while !markers.0.join("completed").try_exists()? {
        if Instant::now() >= sent_deadline {
            return Err("fixture did not complete gated step after release".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let completed = std::fs::read(markers.0.join("completed"))?;
    assert!(
        completed == b"sent"
            || (completed == b"closed"
                && matches!(
                    expected,
                    StartupOutcome::Revoked | StartupOutcome::Disconnected
                )),
        "unexpected gated send outcome: {completed:?}"
    );
    supervisor.poll();
    if expected == StartupOutcome::Admitted {
        while supervisor.snapshot(id)?.state == WorkerState::Starting {
            if Instant::now() >= sent_deadline {
                return Err("on-time worker did not become ready".into());
            }
            supervisor.poll();
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    let snapshot = supervisor.snapshot(id)?;
    if expected != StartupOutcome::Admitted {
        assert_eq!(snapshot.failure, expected.failure(), "{snapshot:?}");
        assert_ne!(snapshot.state, WorkerState::Ready);
        assert!(supervisor.lease(id).is_err());
    } else {
        assert_eq!(snapshot.state, WorkerState::Ready, "{snapshot:?}");
        let lease = supervisor.lease(id)?;
        assert!(!lease.is_revoked());
        supervisor.cancel(id, CancellationId::from_uuid(Uuid::new_v4()))?;
        assert!(lease.is_revoked());
    }
    let cleanup_deadline = Instant::now() + Duration::from_secs(5);
    loop {
        supervisor.poll();
        let snapshot = supervisor.snapshot(id)?;
        assert_ne!(snapshot.state, WorkerState::Ready);
        assert!(supervisor.lease(id).is_err());
        if matches!(snapshot.state, WorkerState::Stopped | WorkerState::Failed) {
            assert_eq!(snapshot.failure, expected.failure());
            break;
        }
        if Instant::now() >= cleanup_deadline {
            return Err(format!("startup cleanup did not reap child: {snapshot:?}").into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(supervisor.retire(id)?.unresolved_requests.is_empty());
    Ok(())
}

#[test]
fn ready_sent_after_startup_expiry_never_gains_a_lease() -> TestResult {
    gated_startup("ready", StartupOutcome::Expired)
}

#[test]
fn same_gated_worker_admitted_before_expiry_gets_a_revocable_lease() -> TestResult {
    for gate_at in ["control", "progress", "ready"] {
        gated_startup(gate_at, StartupOutcome::Admitted)?;
    }
    Ok(())
}

#[test]
fn first_hello_after_expiry_never_gains_a_lease() -> TestResult {
    gated_startup("control", StartupOutcome::Expired)
}

#[test]
fn control_welcome_does_not_extend_the_progress_hello_deadline() -> TestResult {
    gated_startup("progress", StartupOutcome::Expired)
}

#[test]
fn revoked_startup_cannot_complete_with_valid_hello_or_ready() -> TestResult {
    for gate_at in ["control", "progress", "ready"] {
        gated_startup(gate_at, StartupOutcome::Revoked)?;
    }
    Ok(())
}

#[test]
fn disconnect_after_control_welcome_cannot_gain_authority() -> TestResult {
    gated_startup("disconnect", StartupOutcome::Disconnected)
}
