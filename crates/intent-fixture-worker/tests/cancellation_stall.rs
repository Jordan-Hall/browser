#![cfg(all(target_os = "linux", target_env = "gnu"))]
#![forbid(unsafe_code)]

use hmac::{Hmac, Mac};
use intent_broker::{DispatchTicket, RuntimeBroker};
use intent_contracts::{
    AccountId, ActionProposalId, BoundedText, CapabilityId, ContentHash, OperationId,
    OutboxMessageId, SchemaVersion, Task, TaskId, UnixTimestampMicros, WorkerInstanceId, Workspace,
    WorkspaceId,
};
use intent_local_transport::WorkerRole;
use intent_recovery::{AttemptBinding, RecoveryEffect};
use intent_state::{
    ArtifactScope, AuthorityUpdate, DurableOperationState, EvidenceVerifier, NewDurableOperation,
    ReadOnlyAttestation, ReconciliationVerdict, RecoverableAction, RuntimeOwner,
    VerifiedReadOnlyEvidence, WorkspaceGraph,
};
use intent_supervisor::{
    AdmissionLimits, ExecutableImage, ExecutionBoundary, HealthPolicy, Priority, ProcessLimits,
    RestartPolicy, SchedulerLimits, WorkerConfig, WorkerScope, WorkerState,
};
use rusqlite::Connection;
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs,
    os::unix::fs::DirBuilderExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;
const KEY: [u8; 32] = [79; 32];
const PAYLOAD: &[u8] = b"\x00approved exact bytes\xff\n";
const CANCELLATION_DETAIL: &str =
    "worker cancellation requested after dispatch start; outcome retained for reconciliation";

fn cancellation_busy_handler(_attempt: i32) -> bool {
    if let Ok(root) = std::env::var("INTENT_CANCEL_STALL_PROFILE") {
        let _ = fs::write(Path::new(&root).join("cancel-write-blocked"), b"blocked");
    }
    true
}

fn hash(bytes: &[u8]) -> ContentHash {
    ContentHash::from_bytes(Sha256::digest(bytes).into())
}

fn id<T: std::str::FromStr>(n: u64) -> Result<T>
where
    T::Err: Error + 'static,
{
    Ok(format!("018f47f7-5a86-7c00-8000-{n:012x}").parse()?)
}

fn now() -> Result<UnixTimestampMicros> {
    Ok(UnixTimestampMicros::try_new(i64::try_from(
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_micros(),
    )?)?)
}

fn future() -> Result<UnixTimestampMicros> {
    Ok(UnixTimestampMicros::try_new(now()?.get() + 60_000_000)?)
}

struct Profile {
    root: PathBuf,
    ledger: PathBuf,
}

impl Profile {
    fn new() -> Result<Self> {
        let root = std::env::temp_dir().join(format!("intent-cancel-stall-{}", Uuid::new_v4()));
        fs::DirBuilder::new().mode(0o700).create(&root)?;
        Ok(Self {
            ledger: root.with_extension("external-ledger"),
            root,
        })
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
        let _ = fs::remove_file(&self.ledger);
    }
}

fn graph() -> Result<WorkspaceGraph> {
    Ok(WorkspaceGraph {
        schema_version: SchemaVersion::V1,
        workspace: Workspace::new(
            id::<WorkspaceId>(1)?,
            BoundedText::try_new("cancellation stall workspace")?,
            now()?,
        ),
        goals: vec![],
        tasks: vec![Task::new(
            id::<TaskId>(2)?,
            id(1)?,
            BoundedText::try_new("cancellation stall task")?,
            now()?,
        )],
        dependencies: vec![],
        retained_artifacts: vec![],
        cursors: vec![],
        provider_references: vec![],
        worker_instances: vec![],
    })
}

fn authority(revision: u64) -> Result<AuthorityUpdate> {
    Ok(AuthorityUpdate {
        account_id: id(12)?,
        capability_id: id(13)?,
        expected_revision: revision,
        effect: RecoveryEffect::ExternalWrite,
        enabled: true,
        source_revision: hash(b"source-v1"),
        valid_until: future()?,
        evidence_key_id: EvidenceVerifier::new(KEY)?.key_id(),
    })
}

fn from_owner(owner: RuntimeOwner) -> Result<RuntimeBroker> {
    Ok(RuntimeBroker::new(
        owner,
        &std::env::temp_dir(),
        AdmissionLimits::new(4, 512 * 1024 * 1024, 400, [0; 2], [0; 2], [0; 2])?,
        SchedulerLimits::default(),
    )?)
}

fn broker(profile: &Profile) -> Result<RuntimeBroker> {
    let mut owner = RuntimeOwner::open_profile(&profile.root, now()?)?;
    owner.state_mut().save_workspace_graph(
        &ArtifactScope::try_new("cancel-stall-scope")?,
        0,
        &graph()?,
        now()?,
    )?;
    let mut broker = from_owner(owner)?;
    broker.plan_startup(128)?;
    broker.activate_after_planning()?;
    broker.update_authority(authority(0)?)?;
    Ok(broker)
}

fn action() -> Result<RecoverableAction> {
    let mut action = RecoverableAction {
        operation: NewDurableOperation {
            binding: None,
            operation_id: id(30)?,
            task_id: id(2)?,
            action_proposal_id: ActionProposalId::from_uuid(Uuid::new_v4()),
            account_id: id(12)?,
            capability_id: id(13)?,
            arguments_hash: hash(PAYLOAD),
            source_schema: SchemaVersion::V1,
            created_at: now()?,
        },
        scope: ArtifactScope::try_new("cancel-stall-scope")?,
        effect: RecoveryEffect::ExternalWrite,

        destination: BoundedText::try_new("fixture://external-ledger")?,
        message_kind: BoundedText::try_new("append")?,
        payload: PAYLOAD.to_vec(),
        deadline: future()?,
        compensation: None,
    };
    action.operation.binding = Some(intent_contracts::ActionBinding {
        task_id: action.operation.task_id,
        account_id: action.operation.account_id,
        capability_id: action.operation.capability_id,
        target_resource: None,
        canonical_arguments: intent_contracts::ArtifactReference::new(
            intent_contracts::ArtifactId::from_uuid(action.operation.action_proposal_id.as_uuid()),
            action.operation.arguments_hash,
            intent_contracts::ByteSize::from_bytes(action.payload.len() as u64),
            BoundedText::try_new("application/octet-stream")?,
        ),
        context: intent_contracts::ActionContext {
            source_revision: hash(b"source-v1"),
            canonicalization: intent_contracts::CanonicalizationVersion::ExactBytesV1,
        },
        effect_class: intent_contracts::CapabilityEffectClass::IrreversibleOrUncertain,
        approval_requirement: intent_contracts::ApprovalRequirement::Always,
        expires_at: Some(action.deadline),
    });
    Ok(action)
}

fn stage(broker: &mut RuntimeBroker) -> Result<OutboxMessageId> {
    let operation = broker.prepare_action(action()?)?;
    broker.approve_action(
        operation,
        0,
        UnixTimestampMicros::try_new(now()?.get() + 30_000_000)?,
    )?;
    Ok(broker.enqueue_action(operation, 1)?)
}

fn launch(broker: &mut RuntimeBroker, profile: &Profile) -> Result<WorkerInstanceId> {
    let path = Path::new(env!("CARGO_BIN_EXE_intent-fixture-worker"));
    let bytes = fs::read(path)?;
    let image = ExecutableImage::load(path, hash(&bytes))?;
    let config = WorkerConfig::new(
        WorkerScope {
            task_id: id(2)?,
            account_id: id::<AccountId>(12)?,
        },
        WorkerRole::ConnectorHost,
        [id::<CapabilityId>(13)?],
        Priority::Interactive,
        ProcessLimits::new(128 * 1024 * 1024, 10, 64, 100)?,
        HealthPolicy::default(),
        Duration::from_secs(20),
        RestartPolicy::never(),
        ExecutionBoundary::CooperativeLocal,
    )?;
    Ok(broker.launch(
        image,
        config,
        &[
            "broker-effect-lost-ack".to_owned(),
            profile.ledger.to_string_lossy().into_owned(),
        ],
    )?)
}

fn ready(broker: &mut RuntimeBroker, worker: WorkerInstanceId) -> Result {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        broker.poll()?;
        if broker
            .worker_snapshot(worker)
            .is_ok_and(|snapshot| snapshot.state == WorkerState::Ready)
        {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    Err("worker readiness timed out".into())
}

fn state(broker: &RuntimeBroker) -> Result<DurableOperationState> {
    Ok(broker
        .state()
        .load_operation(id::<OperationId>(30)?)?
        .ok_or("missing operation")?
        .state())
}

fn revision(broker: &RuntimeBroker) -> Result<u64> {
    Ok(broker
        .state()
        .load_operation(id::<OperationId>(30)?)?
        .ok_or("missing operation")?
        .revision())
}

fn rows(profile: &Profile) -> Result<Vec<serde_json::Value>> {
    match fs::read_to_string(&profile.ledger) {
        Ok(text) => text
            .lines()
            .map(|line| Ok(serde_json::from_str(line)?))
            .collect(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(vec![]),
        Err(error) => Err(error.into()),
    }
}

fn evidence(profile: &Profile, ticket: DispatchTicket) -> Result<VerifiedReadOnlyEvidence> {
    let records = rows(profile)?;
    assert_eq!(records.len(), 1);
    let binding: AttemptBinding = serde_json::from_value(records[0]["binding"].clone())?;
    assert_eq!(binding.operation_id, ticket.operation_id);
    assert_eq!(binding.attempt_id, ticket.attempt_id);
    assert_eq!(binding.arguments_hash, hash(PAYLOAD));
    let value = ReadOnlyAttestation {
        schema_version: 1,
        evidence_id: Uuid::new_v4(),
        binding,
        verdict: ReconciliationVerdict::Committed {
            receipt: hash(&fs::read(&profile.ledger)?),
        },
        observed_at: now()?,
        valid_until: future()?,
    };
    let bytes = serde_json::to_vec(&value)?;
    let mut mac = Hmac::<Sha256>::new_from_slice(&KEY)?;
    mac.update(b"intent.read-only-reconciliation.v1\0");
    mac.update(&bytes);
    let tag = mac.finalize().into_bytes().into();
    Ok(EvidenceVerifier::new(KEY)?.verify(&bytes, &tag, now()?)?)
}

#[test]
fn cancellation_stall_child() -> Result {
    let Ok(root) = std::env::var("INTENT_CANCEL_STALL_PROFILE") else {
        return Ok(());
    };
    let root = PathBuf::from(root);
    let profile = Profile {
        ledger: root.with_extension("external-ledger"),
        root,
    };
    let mut broker = broker(&profile)?;
    let worker = launch(&mut broker, &profile)?;
    ready(&mut broker, worker)?;
    let outbox = stage(&mut broker)?;
    broker.dispatch(outbox, worker, Duration::from_secs(2))?;
    let effect_deadline = Instant::now() + Duration::from_secs(3);
    while rows(&profile)?.is_empty() {
        if Instant::now() >= effect_deadline {
            return Err("child external effect did not arrive".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(state(&broker)?, DurableOperationState::Attempting);
    assert_eq!(revision(&broker)?, 3);
    broker
        .state()
        .install_busy_handler_for_integration_test(Some(cancellation_busy_handler))?;
    fs::write(
        profile.root.join("cancel-ready"),
        format!("{worker}\n{outbox}"),
    )?;
    let lock_deadline = Instant::now() + Duration::from_secs(8);
    while !profile.root.join("cancel-lock-ready").is_file() {
        if Instant::now() >= lock_deadline {
            return Err("parent never installed the cancellation journal stall".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    fs::write(profile.root.join("cancel-entered"), b"entered")?;
    let result = broker.cancel_worker(worker);
    fs::write(profile.root.join("cancel-returned"), format!("{result:?}"))?;
    Err("cancel_worker returned while the journal writer lock was held".into())
}

#[test]
fn process_death_while_cancellation_persistence_is_stalled_recovers_without_resend() -> Result {
    struct ChildGuard(std::process::Child);
    impl Drop for ChildGuard {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }

    let profile = Profile::new()?;
    let mut child = ChildGuard(
        Command::new(std::env::current_exe()?)
            .args(["--exact", "cancellation_stall_child", "--nocapture"])
            .env("INTENT_CANCEL_STALL_PROFILE", &profile.root)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()?,
    );
    let ready_deadline = Instant::now() + Duration::from_secs(8);
    while !profile.root.join("cancel-ready").is_file() {
        if let Some(status) = child.0.try_wait()? {
            return Err(format!("broker child exited before cancellation stall: {status}").into());
        }
        if Instant::now() >= ready_deadline {
            return Err("broker cancellation-stall child timed out".into());
        }
        std::thread::sleep(Duration::from_millis(2));
    }
    let marker = fs::read_to_string(profile.root.join("cancel-ready"))?;
    let mut marker_lines = marker.lines();
    let old_worker: WorkerInstanceId = marker_lines
        .next()
        .ok_or("missing old worker marker")?
        .parse()?;
    let old_outbox: OutboxMessageId = marker_lines
        .next()
        .ok_or("missing old outbox marker")?
        .parse()?;
    if marker_lines.next().is_some() {
        return Err("unexpected cancellation marker data".into());
    }

    let db = Connection::open(profile.root.join("state.sqlite3"))?;
    db.execute_batch("BEGIN IMMEDIATE")?;
    fs::write(profile.root.join("cancel-lock-ready"), b"locked")?;
    let entered_deadline = Instant::now() + Duration::from_secs(3);
    while !profile.root.join("cancel-entered").is_file() {
        if let Some(status) = child.0.try_wait()? {
            return Err(
                format!("broker child exited before entering cancellation: {status}").into(),
            );
        }
        if Instant::now() >= entered_deadline {
            return Err("broker child never entered stalled cancellation".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let blocked_deadline = Instant::now() + Duration::from_secs(3);
    while !profile.root.join("cancel-write-blocked").is_file() {
        if let Some(status) = child.0.try_wait()? {
            return Err(format!(
                "broker child exited before cancellation reached the blocked SQLite writer: {status}"
            )
            .into());
        }
        if Instant::now() >= blocked_deadline {
            return Err("cancellation never reached the blocked SQLite writer".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(
        child.0.try_wait()?.is_none(),
        "stalled cancellation unexpectedly returned before process death"
    );
    assert!(
        !profile.root.join("cancel-returned").exists(),
        "cancel_worker returned while the database writer remained stalled"
    );
    child.0.kill()?;
    child.0.wait()?;
    db.execute_batch("ROLLBACK")?;
    let persisted_cancellation_rows: i64 = db.query_row(
        "SELECT COUNT(*) FROM operation_journal WHERE state_detail=?1",
        [CANCELLATION_DETAIL],
        |row| row.get(0),
    )?;
    assert_eq!(
        persisted_cancellation_rows, 0,
        "process death happened before cancellation intent could persist"
    );
    drop(db);

    let mut next = from_owner(RuntimeOwner::open_profile(&profile.root, now()?)?)?;
    assert!(!next.state().dispatch_status()?.enabled);
    assert!(next.worker_snapshot(old_worker).is_err());
    assert!(
        next.dispatch(old_outbox, old_worker, Duration::from_secs(1))
            .is_err()
    );
    assert!(!next.plan_startup(128)?.remaining);
    assert_eq!(state(&next)?, DurableOperationState::NeedsReconciliation);
    next.activate_after_planning()?;
    assert!(
        next.dispatch(old_outbox, old_worker, Duration::from_secs(1))
            .is_err()
    );
    let binding: AttemptBinding = serde_json::from_value(rows(&profile)?[0]["binding"].clone())?;
    let ticket = DispatchTicket {
        operation_id: binding.operation_id,
        attempt_id: binding.attempt_id,
        request_id: id(60)?,
        worker_id: old_worker,
    };
    let current_revision = revision(&next)?;
    next.reconcile(evidence(&profile, ticket)?, current_revision)?;
    assert_eq!(state(&next)?, DurableOperationState::Verified);
    assert_eq!(
        rows(&profile)?.len(),
        1,
        "restart reconciliation must not resend the already-observed external effect"
    );
    assert!(!next.is_fenced());
    Ok(())
}
