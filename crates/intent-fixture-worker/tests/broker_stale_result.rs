#![cfg(all(target_os = "linux", target_env = "gnu"))]
#![forbid(unsafe_code)]

use intent_broker::RuntimeBroker;
use intent_contracts::{
    ActionProposalId, BoundedText, CapabilityId, ContentHash, OperationId, OutboxMessageId,
    SchemaVersion, Task, TaskId, UnixTimestampMicros, WorkerInstanceId, Workspace, WorkspaceId,
};
use intent_local_transport::WorkerRole;
use intent_recovery::RecoveryEffect;
use intent_state::{
    ArtifactScope, AuthorityUpdate, DurableOperationState, EvidenceVerifier, NewDurableOperation,
    RecoverableAction, RuntimeOwner, WorkspaceGraph,
};
use intent_supervisor::{
    AdmissionLimits, ExecutableImage, ExecutionBoundary, HealthPolicy, Priority, ProcessLimits,
    RestartPolicy, SchedulerLimits, WorkerConfig, WorkerScope, WorkerState,
};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs,
    os::unix::fs::DirBuilderExt,
    path::{Path, PathBuf},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;
const PAYLOAD: &[u8] = b"stale generation payload";

struct Profile {
    root: PathBuf,
}
impl Profile {
    fn new() -> TestResult<Self> {
        let root = std::env::temp_dir().join(format!("intent-stale-result-{}", Uuid::new_v4()));
        fs::DirBuilder::new().mode(0o700).create(&root)?;
        Ok(Self { root })
    }
}
impl Drop for Profile {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn hash(bytes: &[u8]) -> ContentHash {
    ContentHash::from_bytes(Sha256::digest(bytes).into())
}
fn id<T: std::str::FromStr>(n: u64) -> TestResult<T>
where
    T::Err: Error + 'static,
{
    Ok(format!("018f47f7-5a86-7c00-8000-{n:012x}").parse()?)
}
fn now() -> TestResult<UnixTimestampMicros> {
    Ok(UnixTimestampMicros::try_new(i64::try_from(
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_micros(),
    )?)?)
}
fn future() -> TestResult<UnixTimestampMicros> {
    Ok(UnixTimestampMicros::try_new(now()?.get() + 60_000_000)?)
}
fn graph() -> TestResult<WorkspaceGraph> {
    Ok(WorkspaceGraph {
        schema_version: SchemaVersion::V1,
        workspace: Workspace::new(
            id::<WorkspaceId>(1)?,
            BoundedText::try_new("stale result workspace")?,
            now()?,
        ),
        goals: vec![],
        tasks: vec![Task::new(
            id::<TaskId>(2)?,
            id(1)?,
            BoundedText::try_new("stale result task")?,
            now()?,
        )],
        dependencies: vec![],
        retained_artifacts: vec![],
        cursors: vec![],
        provider_references: vec![],
        worker_instances: vec![],
    })
}
fn authority(revision: u64) -> TestResult<AuthorityUpdate> {
    Ok(AuthorityUpdate {
        account_id: id(12)?,
        capability_id: id(13)?,
        expected_revision: revision,
        effect: RecoveryEffect::ExternalWrite,
        enabled: true,
        source_revision: hash(b"source-v1"),
        valid_until: future()?,
        evidence_key_id: EvidenceVerifier::new([79; 32])?.key_id(),
    })
}
fn broker(profile: &Profile) -> TestResult<RuntimeBroker> {
    let mut owner = RuntimeOwner::open_profile(&profile.root, now()?)?;
    owner.state_mut().save_workspace_graph(
        &ArtifactScope::try_new("stale-result-scope")?,
        0,
        &graph()?,
        now()?,
    )?;
    let mut broker = RuntimeBroker::new(
        owner,
        &std::env::temp_dir(),
        AdmissionLimits::new(2, 256 * 1024 * 1024, 200, [0; 2], [0; 2], [0; 2])?,
        SchedulerLimits::default(),
    )?;
    broker.plan_startup(128)?;
    broker.activate_after_planning()?;
    broker.update_authority(authority(0)?)?;
    Ok(broker)
}
fn action() -> TestResult<RecoverableAction> {
    Ok(RecoverableAction {
        operation: NewDurableOperation {
            operation_id: id(30)?,
            task_id: id(2)?,
            action_proposal_id: ActionProposalId::from_uuid(Uuid::new_v4()),
            account_id: id(12)?,
            capability_id: id(13)?,
            arguments_hash: hash(PAYLOAD),
            source_schema: SchemaVersion::V1,
            created_at: now()?,
        },
        scope: ArtifactScope::try_new("stale-result-scope")?,
        effect: RecoveryEffect::ExternalWrite,
        source_revision: hash(b"source-v1"),
        destination: BoundedText::try_new("fixture://stale-result")?,
        message_kind: BoundedText::try_new("append")?,
        payload: PAYLOAD.to_vec(),
        deadline: future()?,
        compensation: None,
    })
}
fn stage(broker: &mut RuntimeBroker) -> TestResult<OutboxMessageId> {
    let operation = broker.prepare_action(action()?)?;
    broker.approve_action(
        operation,
        0,
        UnixTimestampMicros::try_new(now()?.get() + 30_000_000)?,
    )?;
    Ok(broker.enqueue_action(operation, 1)?)
}
fn launch(
    broker: &mut RuntimeBroker,
    executable: &Path,
    args: &[String],
) -> TestResult<WorkerInstanceId> {
    let image = ExecutableImage::load(executable, hash(&fs::read(executable)?))?;
    let config = WorkerConfig::new(
        WorkerScope {
            task_id: id(2)?,
            account_id: id(12)?,
        },
        WorkerRole::ConnectorHost,
        [id::<CapabilityId>(13)?],
        Priority::Interactive,
        ProcessLimits::new(128 * 1024 * 1024, 10, 64, 100)?,
        HealthPolicy {
            stop_grace: Duration::from_secs(2),
            terminate_grace: Duration::from_secs(2),
            ..HealthPolicy::default()
        },
        Duration::from_secs(20),
        RestartPolicy::never(),
        ExecutionBoundary::CooperativeLocal,
    )?;
    Ok(broker.launch(image, config, args)?)
}
fn until(broker: &mut RuntimeBroker, predicate: impl Fn(&RuntimeBroker) -> bool) -> TestResult {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        broker.poll()?;
        if predicate(broker) {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err("broker wait expired".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn ready(broker: &mut RuntimeBroker, worker: WorkerInstanceId) -> TestResult {
    until(broker, |broker| {
        broker
            .worker_snapshot(worker)
            .is_ok_and(|snapshot| snapshot.state == WorkerState::Ready)
    })
}
fn state(broker: &RuntimeBroker) -> TestResult<DurableOperationState> {
    Ok(broker
        .state()
        .load_operation(id::<OperationId>(30)?)?
        .ok_or("missing operation")?
        .state())
}

#[test]
fn late_result_from_revoked_worker_cannot_authorize_replacement_dispatch() -> TestResult {
    let profile = Profile::new()?;
    let mut broker = broker(&profile)?;
    let replacement_ready = profile.root.join("replacement-ready");
    let old_path = Path::new(env!("CARGO_BIN_EXE_intent-stale-result-worker"));
    let old = launch(
        &mut broker,
        old_path,
        &[replacement_ready.to_string_lossy().into_owned()],
    )?;
    ready(&mut broker, old)?;
    let outbox = stage(&mut broker)?;
    broker.dispatch(outbox, old, Duration::from_secs(2))?;
    assert_eq!(state(&broker)?, DurableOperationState::Attempting);
    let outcomes = broker.cancel_worker(old)?;
    assert_eq!(outcomes.len(), 1);
    assert_eq!(state(&broker)?, DurableOperationState::NeedsReconciliation);
    assert_eq!(broker.inflight_count(), 0);
    let replacement_path = Path::new(env!("CARGO_BIN_EXE_intent-fixture-worker"));
    let replacement = launch(&mut broker, replacement_path, &["normal".to_owned()])?;
    ready(&mut broker, replacement)?;
    assert_eq!(
        broker.worker_snapshot(old)?.state,
        WorkerState::Draining,
        "the revoked worker must still be waiting to release its late result"
    );
    fs::write(&replacement_ready, b"replacement ready")?;
    until(&mut broker, |broker| {
        broker.worker_snapshot(old).is_ok_and(|snapshot| {
            matches!(snapshot.state, WorkerState::Stopped | WorkerState::Failed)
        })
    })?;
    assert_eq!(state(&broker)?, DurableOperationState::NeedsReconciliation);
    assert!(
        broker
            .dispatch(outbox, replacement, Duration::from_secs(1))
            .is_err(),
        "a late response from the revoked generation must not reopen dispatch"
    );
    assert_eq!(state(&broker)?, DurableOperationState::NeedsReconciliation);
    broker.retire(old)?;
    Ok(())
}
