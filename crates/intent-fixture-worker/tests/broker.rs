#![cfg(all(target_os = "linux", target_env = "gnu"))]
#![forbid(unsafe_code)]
use hmac::{Hmac, Mac};
use intent_broker::{
    BrokerError, CancellationFailure, CancellationPersistenceStatus, CancellationTerminationStatus,
    DispatchTicket, RuntimeBroker,
};
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
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;
type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;
const KEY: [u8; 32] = [79; 32];
const PAYLOAD: &[u8] = b"\x00approved exact bytes\xff\n";
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
        let root = std::env::temp_dir().join(format!("intent-broker-{}", Uuid::new_v4()));
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
            BoundedText::try_new("broker workspace")?,
            now()?,
        ),
        goals: vec![],
        tasks: vec![Task::new(
            id::<TaskId>(2)?,
            id(1)?,
            BoundedText::try_new("broker task")?,
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
fn broker(profile: &Profile) -> Result<RuntimeBroker> {
    let mut owner = RuntimeOwner::open_profile(&profile.root, now()?)?;
    owner.state_mut().save_workspace_graph(
        &ArtifactScope::try_new("broker-scope")?,
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
fn from_owner(owner: RuntimeOwner) -> Result<RuntimeBroker> {
    Ok(RuntimeBroker::new(
        owner,
        &std::env::temp_dir(),
        AdmissionLimits::new(4, 512 * 1024 * 1024, 400, [0; 2], [0; 2], [0; 2])?,
        SchedulerLimits::default(),
    )?)
}
fn action(n: u64) -> Result<RecoverableAction> {
    Ok(RecoverableAction {
        operation: NewDurableOperation {
            operation_id: id(n)?,
            task_id: id(2)?,
            action_proposal_id: ActionProposalId::from_uuid(Uuid::new_v4()),
            account_id: id(12)?,
            capability_id: id(13)?,
            arguments_hash: hash(PAYLOAD),
            source_schema: SchemaVersion::V1,
            created_at: now()?,
        },
        scope: ArtifactScope::try_new("broker-scope")?,
        effect: RecoveryEffect::ExternalWrite,
        source_revision: hash(b"source-v1"),
        destination: BoundedText::try_new("fixture://external-ledger")?,
        message_kind: BoundedText::try_new("append")?,
        payload: PAYLOAD.to_vec(),
        deadline: future()?,
        compensation: None,
    })
}
fn stage(broker: &mut RuntimeBroker, n: u64) -> Result<OutboxMessageId> {
    let op = broker.prepare_action(action(n)?)?;
    broker.approve_action(
        op,
        0,
        UnixTimestampMicros::try_new(now()?.get() + 30_000_000)?,
    )?;
    Ok(broker.enqueue_action(op, 1)?)
}
fn launch(
    broker: &mut RuntimeBroker,
    profile: &Profile,
    mode: &str,
    account: AccountId,
    cap: CapabilityId,
) -> Result<WorkerInstanceId> {
    let path = Path::new(env!("CARGO_BIN_EXE_intent-fixture-worker"));
    let started = Instant::now();
    let bytes = fs::read(path)?;
    eprintln!(
        "broker fixture image: {} bytes read in {:?}",
        bytes.len(),
        started.elapsed()
    );
    let expected = hash(&bytes);
    eprintln!("broker fixture image: hashed in {:?}", started.elapsed());
    let image = ExecutableImage::load(path, expected)?;
    eprintln!("broker fixture image: sealed in {:?}", started.elapsed());
    let config = WorkerConfig::new(
        WorkerScope {
            task_id: id(2)?,
            account_id: account,
        },
        WorkerRole::ConnectorHost,
        [cap],
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
            mode.to_owned(),
            profile.ledger.to_string_lossy().into_owned(),
        ],
    )?)
}
fn until(broker: &mut RuntimeBroker, predicate: impl Fn(&RuntimeBroker) -> bool) -> Result {
    let end = Instant::now() + Duration::from_secs(5);
    while Instant::now() < end {
        broker.poll()?;
        if predicate(broker) {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    Err("broker predicate timed out".into())
}
fn ready(broker: &mut RuntimeBroker, worker: WorkerInstanceId) -> Result {
    until(broker, |b| {
        b.worker_snapshot(worker)
            .is_ok_and(|s| s.state == WorkerState::Ready)
    })
}

fn wait_for_child_marker(
    child: &mut std::process::Child,
    marker: &Path,
    timeout: Duration,
) -> Result {
    let end = Instant::now() + timeout;
    while !marker.is_file() {
        if let Some(status) = child.try_wait()? {
            return Err(format!("broker child exited early: {status}").into());
        }
        if Instant::now() >= end {
            return Err(format!("broker child timed out waiting for {}", marker.display()).into());
        }
        std::thread::sleep(Duration::from_millis(2));
    }
    Ok(())
}
fn state(b: &RuntimeBroker, n: u64) -> Result<DurableOperationState> {
    Ok(b.state()
        .load_operation(id(n)?)?
        .ok_or("missing operation")?
        .state())
}
fn revision(b: &RuntimeBroker, n: u64) -> Result<u64> {
    Ok(b.state()
        .load_operation(id(n)?)?
        .ok_or("missing operation")?
        .revision())
}
fn rows(profile: &Profile) -> Result<Vec<serde_json::Value>> {
    match fs::read_to_string(&profile.ledger) {
        Ok(text) => text
            .lines()
            .map(|line| Ok(serde_json::from_str(line)?))
            .collect(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(vec![]),
        Err(e) => Err(e.into()),
    }
}
/// The parent observes a ledger outside the runtime profile. The worker never gets this MAC key.
fn evidence(profile: &Profile, ticket: DispatchTicket) -> Result<VerifiedReadOnlyEvidence> {
    let records = rows(profile)?;
    assert_eq!(
        records.len(),
        1,
        "independent effect ledger detects duplicate external execution"
    );
    let binding: AttemptBinding = serde_json::from_value(records[0]["binding"].clone())?;
    assert_eq!(binding.operation_id, ticket.operation_id);
    assert_eq!(binding.attempt_id, ticket.attempt_id);
    assert_eq!(binding.arguments_hash, hash(PAYLOAD));
    assert_eq!(records[0]["payload"], serde_json::json!(PAYLOAD));
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
fn real_worker_effect_is_accepted_but_only_independent_evidence_verifies_it() -> Result {
    let p = Profile::new()?;
    let mut b = broker(&p)?;
    let w = launch(&mut b, &p, "broker-effect", id(12)?, id(13)?)?;
    ready(&mut b, w)?;
    let out = stage(&mut b, 30)?;
    let ticket = b.dispatch(out, w, Duration::from_secs(2))?;
    assert_eq!(state(&b, 30)?, DurableOperationState::Attempting);
    until(&mut b, |b| b.inflight_count() == 0)?;
    assert_eq!(state(&b, 30)?, DurableOperationState::Accepted);
    assert!(b.dispatch(out, w, Duration::from_secs(2)).is_err());
    let rev = revision(&b, 30)?;
    b.reconcile(evidence(&p, ticket)?, rev)?;
    assert_eq!(state(&b, 30)?, DurableOperationState::Verified);
    assert_eq!(rows(&p)?.len(), 1);
    Ok(())
}
#[test]
fn worker_ack_without_an_effect_never_becomes_a_verified_receipt() -> Result {
    let p = Profile::new()?;
    let mut b = broker(&p)?;
    let w = launch(&mut b, &p, "broker-ack-only", id(12)?, id(13)?)?;
    ready(&mut b, w)?;
    let out = stage(&mut b, 30)?;
    b.dispatch(out, w, Duration::from_secs(2))?;
    until(&mut b, |b| b.inflight_count() == 0)?;
    assert_eq!(state(&b, 30)?, DurableOperationState::Accepted);
    assert!(rows(&p)?.is_empty());
    Ok(())
}
#[test]
fn lost_acknowledgement_and_restart_preserve_attempt_without_resending() -> Result {
    let p = Profile::new()?;
    let mut b = broker(&p)?;
    let w = launch(&mut b, &p, "broker-effect-lost-ack", id(12)?, id(13)?)?;
    ready(&mut b, w)?;
    let out = stage(&mut b, 30)?;
    let ticket = b.dispatch(out, w, Duration::from_secs(2))?;
    until(&mut b, |b| b.inflight_count() == 0)?;
    assert_eq!(state(&b, 30)?, DurableOperationState::NeedsReconciliation);
    let epoch = b.epoch();
    drop(b);
    let mut b = from_owner(RuntimeOwner::open_profile(&p.root, now()?)?)?;
    assert_ne!(b.epoch(), epoch);
    assert!(!b.state().dispatch_status()?.enabled);
    assert!(!b.plan_startup(128)?.remaining);
    b.activate_after_planning()?;
    assert!(b.dispatch(out, w, Duration::from_secs(2)).is_err());
    let rev = revision(&b, 30)?;
    b.reconcile(evidence(&p, ticket)?, rev)?;
    assert_eq!(state(&b, 30)?, DurableOperationState::Verified);
    assert_eq!(rows(&p)?.len(), 1);
    Ok(())
}
#[test]
fn owner_drop_after_effect_before_poll_recovers_the_same_attempt() -> Result {
    let p = Profile::new()?;
    let mut b = broker(&p)?;
    let w = launch(&mut b, &p, "broker-effect-lost-ack", id(12)?, id(13)?)?;
    ready(&mut b, w)?;
    let out = stage(&mut b, 30)?;
    let ticket = b.dispatch(out, w, Duration::from_secs(2))?;
    let end = Instant::now() + Duration::from_secs(3);
    while rows(&p)?.is_empty() {
        if Instant::now() >= end {
            return Err("missing effect".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(state(&b, 30)?, DurableOperationState::Attempting);
    drop(b);
    let mut b = from_owner(RuntimeOwner::open_profile(&p.root, now()?)?)?;
    b.plan_startup(128)?;
    b.activate_after_planning()?;
    assert_eq!(state(&b, 30)?, DurableOperationState::NeedsReconciliation);
    let rev = revision(&b, 30)?;
    b.reconcile(evidence(&p, ticket)?, rev)?;
    assert_eq!(rows(&p)?.len(), 1);
    Ok(())
}
#[test]
fn cancellation_before_dispatch_preserves_unsent_state_and_sends_no_effect() -> Result {
    let p = Profile::new()?;
    let mut b = broker(&p)?;
    let w = launch(&mut b, &p, "broker-effect", id(12)?, id(13)?)?;
    ready(&mut b, w)?;
    let out = stage(&mut b, 30)?;
    b.cancel_worker(w)?;
    assert!(b.dispatch(out, w, Duration::from_secs(1)).is_err());
    assert_eq!(state(&b, 30)?, DurableOperationState::DispatchPending);
    assert!(rows(&p)?.is_empty());
    until(&mut b, |b| {
        b.worker_snapshot(w)
            .is_ok_and(|s| matches!(s.state, WorkerState::Stopped | WorkerState::Failed))
    })?;
    b.retire(w)?;
    assert!(b.worker_snapshot(w).is_err());
    Ok(())
}
#[test]
fn authority_or_source_revision_change_revokes_workers_and_stale_approval() -> Result {
    for disable in [false, true] {
        let p = Profile::new()?;
        let mut b = broker(&p)?;
        let w = launch(&mut b, &p, "broker-effect", id(12)?, id(13)?)?;
        ready(&mut b, w)?;
        let out = stage(&mut b, 30)?;
        let mut update = authority(1)?;
        if disable {
            update.enabled = false;
        } else {
            update.source_revision = hash(b"source-v2");
        }
        b.update_authority(update)?;
        assert!(b.dispatch(out, w, Duration::from_secs(1)).is_err());
        let next = launch(&mut b, &p, "broker-effect", id(12)?, id(13)?)?;
        ready(&mut b, next)?;
        assert!(b.dispatch(out, next, Duration::from_secs(1)).is_err());
        assert_eq!(state(&b, 30)?, DurableOperationState::DispatchPending);
        assert!(rows(&p)?.is_empty());
    }
    Ok(())
}
#[test]
fn expired_approval_is_denied_before_the_socket_or_attempt_start() -> Result {
    let p = Profile::new()?;
    let mut b = broker(&p)?;
    let w = launch(&mut b, &p, "broker-effect", id(12)?, id(13)?)?;
    ready(&mut b, w)?;
    let op = b.prepare_action(action(30)?)?;
    b.approve_action(op, 0, UnixTimestampMicros::try_new(now()?.get() + 100_000)?)?;
    let out = b.enqueue_action(op, 1)?;
    std::thread::sleep(Duration::from_millis(120));
    assert!(b.dispatch(out, w, Duration::from_secs(1)).is_err());
    assert_eq!(state(&b, 30)?, DurableOperationState::DispatchPending);
    assert!(rows(&p)?.is_empty());
    Ok(())
}
#[test]
fn authenticated_wrong_scope_or_capability_cannot_dispatch_an_action() -> Result {
    for (account, cap) in [(14, 13), (12, 14)] {
        let p = Profile::new()?;
        let mut b = broker(&p)?;
        let w = launch(&mut b, &p, "broker-effect", id(account)?, id(cap)?)?;
        ready(&mut b, w)?;
        let out = stage(&mut b, 30)?;
        assert!(b.dispatch(out, w, Duration::from_secs(1)).is_err());
        assert_eq!(state(&b, 30)?, DurableOperationState::DispatchPending);
        assert!(rows(&p)?.is_empty());
    }
    Ok(())
}
#[test]
fn unready_worker_never_receives_durable_dispatch_authority() -> Result {
    let p = Profile::new()?;
    let mut b = broker(&p)?;
    let w = launch(&mut b, &p, "wrong-token", id(12)?, id(13)?)?;
    let out = stage(&mut b, 30)?;
    assert!(b.dispatch(out, w, Duration::from_secs(1)).is_err());
    until(&mut b, |b| {
        b.worker_snapshot(w)
            .is_ok_and(|s| s.state == WorkerState::Failed)
    })?;
    assert!(b.dispatch(out, w, Duration::from_secs(1)).is_err());
    assert!(rows(&p)?.is_empty());
    Ok(())
}
#[test]
fn oversized_or_escaping_action_is_rejected_before_persistence() -> Result {
    let p = Profile::new()?;
    let mut b = broker(&p)?;
    let mut a = action(30)?;
    a.payload = vec![0; 1025];
    a.operation.arguments_hash = hash(&a.payload);
    assert!(b.prepare_action(a).is_err());
    assert!(b.state().load_operation(id::<OperationId>(30)?)?.is_none());
    let mut a = action(31)?;
    a.payload = vec![0; 1024];
    a.operation.arguments_hash = hash(&a.payload);
    a.destination = BoundedText::try_new("\u{1}".repeat(512))?;
    assert!(b.prepare_action(a).is_err());
    assert!(b.state().load_operation(id::<OperationId>(31)?)?.is_none());
    Ok(())
}
#[test]
fn dead_socket_after_preflight_never_resets_a_committed_attempt_to_pending() -> Result {
    let p = Profile::new()?;
    let mut b = broker(&p)?;
    let w = launch(&mut b, &p, "broker-effect", id(12)?, id(13)?)?;
    ready(&mut b, w)?;
    let pid = b.worker_snapshot(w)?.process_id;
    let out = stage(&mut b, 30)?;
    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(i32::try_from(pid)?),
        nix::sys::signal::Signal::SIGKILL,
    )?;
    std::thread::sleep(Duration::from_millis(50));
    let result = b.dispatch(out, w, Duration::from_secs(1));
    assert!(matches!(result, Err(BrokerError::Uncertain { .. })));
    assert_eq!(state(&b, 30)?, DurableOperationState::NeedsReconciliation);
    assert!(rows(&p)?.is_empty());
    assert!(b.dispatch(out, w, Duration::from_secs(1)).is_err());
    Ok(())
}

#[test]
fn broker_process_child() -> Result {
    let Ok(root) = std::env::var("INTENT_BROKER_TEST_PROFILE") else {
        return Ok(());
    };
    let phase = std::env::var("INTENT_BROKER_TEST_PHASE")?;
    let root = PathBuf::from(root);
    let p = Profile {
        ledger: root.with_extension("external-ledger"),
        root,
    };
    eprintln!("broker crash child: opening broker");
    let mut b = broker(&p)?;
    eprintln!("broker crash child: launching worker");
    let w = launch(&mut b, &p, "broker-effect-lost-ack", id(12)?, id(13)?)?;
    eprintln!("broker crash child: waiting for ready");
    ready(&mut b, w)?;
    fs::write(p.root.join("ready-marker"), [])?;
    let start_deadline = Instant::now() + Duration::from_secs(8);
    while !p.root.join("start-marker").is_file() {
        if Instant::now() >= start_deadline {
            return Err("parent did not start the broker crash scenario".into());
        }
        b.poll()?;
        std::thread::sleep(Duration::from_millis(1));
    }
    eprintln!("broker crash child: staging action");
    let outbox = stage(&mut b, 30)?;
    if phase == "after-effect" {
        b.dispatch(outbox, w, Duration::from_secs(2))?;
        let end = Instant::now() + Duration::from_secs(3);
        while rows(&p)?.is_empty() {
            if Instant::now() >= end {
                return Err("child fixture effect did not arrive".into());
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    fs::write(p.root.join("kill-marker"), phase.as_bytes())?;
    loop {
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn actual_broker_process_death_preserves_unsent_and_accepted_unknown_boundaries() -> Result {
    use std::process::{Command, Stdio};
    struct ChildGuard(std::process::Child);
    impl Drop for ChildGuard {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    for phase in ["before-send", "after-effect"] {
        let p = Profile::new()?;
        let mut child = ChildGuard(
            Command::new(std::env::current_exe()?)
                .args(["--exact", "broker_process_child", "--nocapture"])
                .env("INTENT_BROKER_TEST_PROFILE", &p.root)
                .env("INTENT_BROKER_TEST_PHASE", phase)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::inherit())
                .spawn()?,
        );
        wait_for_child_marker(
            &mut child.0,
            &p.root.join("ready-marker"),
            Duration::from_secs(30),
        )?;
        fs::write(p.root.join("start-marker"), [])?;
        wait_for_child_marker(
            &mut child.0,
            &p.root.join("kill-marker"),
            Duration::from_secs(8),
        )?;
        child.0.kill()?;
        child.0.wait()?;
        let mut b = from_owner(RuntimeOwner::open_profile(&p.root, now()?)?)?;
        assert!(!b.state().dispatch_status()?.enabled);
        b.plan_startup(128)?;
        b.activate_after_planning()?;
        if phase == "before-send" {
            assert_eq!(state(&b, 30)?, DurableOperationState::DispatchPending);
            assert!(rows(&p)?.is_empty());
            assert!(b.enqueue_action(id(30)?, revision(&b, 30)?).is_err());
        } else {
            assert_eq!(state(&b, 30)?, DurableOperationState::NeedsReconciliation);
            let binding: AttemptBinding = serde_json::from_value(rows(&p)?[0]["binding"].clone())?;
            let op = b
                .state()
                .load_operation(id(30)?)?
                .ok_or("missing crashed operation")?;
            assert_eq!(op.attempt_identity(), Some(binding.attempt_id));
            let ticket = DispatchTicket {
                operation_id: binding.operation_id,
                attempt_id: binding.attempt_id,
                request_id: id(60)?,
                worker_id: id(61)?,
            };
            let rev = op.revision();
            b.reconcile(evidence(&p, ticket)?, rev)?;
            assert_eq!(state(&b, 30)?, DurableOperationState::Verified);
            assert_eq!(rows(&p)?.len(), 1);
        }
    }
    Ok(())
}

#[test]
fn cancellation_persistence_failure_is_explicit_fenced_and_recoverable() -> Result {
    let p = Profile::new()?;
    let mut b = broker(&p)?;
    let w = launch(&mut b, &p, "broker-effect-lost-ack", id(12)?, id(13)?)?;
    ready(&mut b, w)?;
    let out = stage(&mut b, 32)?;
    let ticket = b.dispatch(out, w, Duration::from_secs(2))?;
    let effect_deadline = Instant::now() + Duration::from_secs(3);
    while rows(&p)?.is_empty() {
        if Instant::now() >= effect_deadline {
            return Err("cancellation fixture effect did not arrive".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(state(&b, 32)?, DurableOperationState::Attempting);
    assert_eq!(revision(&b, 32)?, 3);

    let db = Connection::open(p.root.join("state.sqlite3"))?;
    db.execute_batch(
        "CREATE TRIGGER fail_worker_cancel_journal
         BEFORE INSERT ON operation_journal
         WHEN NEW.state_detail='worker cancellation requested after dispatch start; outcome retained for reconciliation'
         BEGIN
           SELECT RAISE(ABORT, 'injected cancellation journal failure');
         END;",
    )?;
    drop(db);

    let error = match b.cancel_worker(w) {
        Ok(_) => return Err("journal failure did not surface".into()),
        Err(error) => error,
    };
    match error {
        BrokerError::Cancellation {
            status:
                CancellationFailure {
                    worker_id,
                    persistence,
                    termination,
                },
            ..
        } => {
            assert_eq!(worker_id, w);
            assert_eq!(persistence, CancellationPersistenceStatus::Failed);
            assert_eq!(termination, CancellationTerminationStatus::Requested);
        }
        other => return Err(format!("unexpected cancellation error: {other}").into()),
    }
    assert!(b.is_fenced());
    assert!(matches!(
        b.dispatch(out, w, Duration::from_secs(1)),
        Err(BrokerError::Blocked)
    ));
    assert_eq!(
        state(&b, 32)?,
        DurableOperationState::Attempting,
        "failed cancellation transaction must not claim durable completion"
    );
    assert_eq!(revision(&b, 32)?, 3);

    drop(b);
    let mut next = from_owner(RuntimeOwner::open_profile(&p.root, now()?)?)?;
    next.plan_startup(128)?;
    assert_eq!(
        state(&next, 32)?,
        DurableOperationState::NeedsReconciliation,
        "a stale pre-cancellation database must recover the started effect as uncertain"
    );
    next.activate_after_planning()?;
    let rev = revision(&next, 32)?;
    next.reconcile(evidence(&p, ticket)?, rev)?;
    assert_eq!(
        rows(&p)?.len(),
        1,
        "reconciliation must not resend the effect"
    );
    assert!(!next.is_fenced());
    Ok(())
}

#[test]
fn authority_update_failure_fences_future_dispatch_instead_of_leaving_old_access_live() -> Result {
    let p = Profile::new()?;
    let mut b = broker(&p)?;
    let w = launch(&mut b, &p, "broker-effect", id(12)?, id(13)?)?;
    ready(&mut b, w)?;
    let out = stage(&mut b, 30)?;
    assert!(b.update_authority(authority(999)?).is_err());
    assert!(b.is_fenced());
    assert!(matches!(
        b.dispatch(out, w, Duration::from_secs(1)),
        Err(BrokerError::Blocked)
    ));
    assert!(rows(&p)?.is_empty());
    Ok(())
}

#[test]
fn cancellation_after_effect_preserves_uncertainty_and_allows_exact_read_only_reconciliation()
-> Result {
    let p = Profile::new()?;
    let mut b = broker(&p)?;
    let w = launch(&mut b, &p, "broker-effect-lost-ack", id(12)?, id(13)?)?;
    ready(&mut b, w)?;
    let out = stage(&mut b, 30)?;
    let ticket = b.dispatch(out, w, Duration::from_secs(2))?;
    let end = Instant::now() + Duration::from_secs(3);
    while rows(&p)?.is_empty() {
        if Instant::now() >= end {
            return Err("missing fixture effect".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let rev = revision(&b, 30)?;
    assert!(b.reconcile(evidence(&p, ticket)?, rev).is_err());
    assert_eq!(state(&b, 30)?, DurableOperationState::Attempting);
    let outcomes = b.cancel_worker(w)?;
    assert_eq!(outcomes.len(), 1);
    assert_eq!(outcomes[0].ticket, ticket);
    assert_eq!(state(&b, 30)?, DurableOperationState::NeedsReconciliation);
    let rev = revision(&b, 30)?;
    b.reconcile(evidence(&p, ticket)?, rev)?;
    assert_eq!(state(&b, 30)?, DurableOperationState::Verified);
    assert_eq!(rows(&p)?.len(), 1);
    assert!(b.dispatch(out, w, Duration::from_secs(1)).is_err());
    Ok(())
}
