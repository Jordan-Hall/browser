#![cfg(all(target_os = "linux", target_env = "gnu"))]
#![forbid(unsafe_code)]

use intent_contracts::{
    ActionProposalId, BoundedText, CapabilityId, ContentHash, OperationId, OutboxMessageId,
    SchemaVersion, Task, TaskId, UnixTimestampMicros, Workspace, WorkspaceId,
};
use intent_recovery::RecoveryEffect;
use intent_state::{
    ArtifactScope, AuthorityUpdate, DurableOperationState, NewDurableOperation, RecoverableAction,
    RuntimeOwner, TransportObservation, WorkerRegistration, WorkspaceGraph,
};
use sha2::{Digest, Sha256};
use std::{error::Error, fs, os::unix::fs::DirBuilderExt, path::PathBuf};
use uuid::Uuid;

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

struct Profile(PathBuf);
impl Profile {
    fn new() -> Result<Self> {
        let path = std::env::temp_dir().join(format!("intent-revocation-{}", Uuid::new_v4()));
        fs::DirBuilder::new().mode(0o700).create(&path)?;
        Ok(Self(path))
    }
}
impl Drop for Profile {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn id<T: std::str::FromStr>(n: u64) -> Result<T>
where
    T::Err: Error + 'static,
{
    Ok(format!("018f47f7-5a86-7c00-8000-{n:012x}").parse()?)
}
fn t(n: i64) -> Result<UnixTimestampMicros> {
    Ok(UnixTimestampMicros::try_new(n)?)
}
fn digest(bytes: &[u8]) -> ContentHash {
    ContentHash::from_bytes(Sha256::digest(bytes).into())
}
fn scope() -> Result<ArtifactScope> {
    Ok(ArtifactScope::try_new("revocation-scope")?)
}
fn graph() -> Result<WorkspaceGraph> {
    Ok(WorkspaceGraph {
        schema_version: SchemaVersion::V1,
        workspace: Workspace::new(
            id::<WorkspaceId>(1)?,
            BoundedText::try_new("workspace")?,
            t(1)?,
        ),
        goals: vec![],
        tasks: vec![Task::new(
            id::<TaskId>(2)?,
            id(1)?,
            BoundedText::try_new("task")?,
            t(1)?,
        )],
        dependencies: vec![],
        retained_artifacts: vec![],
        cursors: vec![],
        provider_references: vec![],
        worker_instances: vec![],
    })
}
fn owner(profile: &Profile) -> Result<RuntimeOwner> {
    let mut owner = RuntimeOwner::open_profile(&profile.0, t(100)?)?;
    owner
        .state_mut()
        .save_workspace_graph(&scope()?, 0, &graph()?, t(100)?)?;
    owner.activate_after_planning(t(100)?)?;
    owner.update_authority(
        AuthorityUpdate {
            account_id: id(12)?,
            capability_id: id::<CapabilityId>(13)?,
            expected_revision: 0,
            effect: RecoveryEffect::ExternalWrite,
            enabled: true,
            source_revision: digest(b"source-v1"),
            valid_until: t(1_000_000)?,
            evidence_key_id: digest(b"evidence-key"),
        },
        t(100)?,
    )?;
    owner.register_worker(worker(20)?, t(100)?)?;
    Ok(owner)
}
fn worker(n: u64) -> Result<WorkerRegistration> {
    Ok(WorkerRegistration {
        worker_id: id(n)?,
        task_id: id(2)?,
        account_id: id(12)?,
        capabilities: vec![id(13)?],
        expires_at: t(1_000_000)?,
    })
}
fn action(n: u64) -> Result<RecoverableAction> {
    let payload = b"exact approved action".to_vec();
    let mut action = RecoverableAction {
        operation: NewDurableOperation {
            binding: None,
            operation_id: id(n)?,
            task_id: id(2)?,
            action_proposal_id: ActionProposalId::from_uuid(Uuid::new_v4()),
            account_id: id(12)?,
            capability_id: id(13)?,
            arguments_hash: digest(&payload),
            source_schema: SchemaVersion::V1,
            created_at: t(100)?,
        },
        scope: scope()?,
        effect: RecoveryEffect::ExternalWrite,

        destination: BoundedText::try_new("fixture://external")?,
        message_kind: BoundedText::try_new("create")?,
        payload,
        deadline: t(900_000)?,
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
            source_revision: digest(b"source-v1"),
            canonicalization: intent_contracts::CanonicalizationVersion::ExactBytesV1,
        },
        effect_class: intent_contracts::CapabilityEffectClass::IrreversibleOrUncertain,
        approval_requirement: intent_contracts::ApprovalRequirement::Always,
        expires_at: Some(action.deadline),
    });
    Ok(action)
}
fn pending(owner: &mut RuntimeOwner, n: u64) -> Result<OutboxMessageId> {
    owner.prepare_action(action(n)?)?;
    owner.approve_action(id(n)?, 0, t(800_000)?, t(100)?)?;
    Ok(owner.enqueue_action(id(n)?, 1, t(100)?)?)
}

#[test]
fn worker_revocation_cancels_its_unstarted_claim_before_notification() -> Result {
    let profile = Profile::new()?;
    let mut owner = owner(&profile)?;
    let outbox = pending(&mut owner, 30)?;
    let lease = owner.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;

    owner.revoke_worker(id(20)?, t(101)?)?;

    assert!(owner.begin_authorized_dispatch(lease, t(101)?).is_err());
    let operation = owner
        .state()
        .load_operation(id::<OperationId>(30)?)?
        .ok_or("operation")?;
    assert_eq!(operation.state(), DurableOperationState::DispatchPending);
    assert_eq!(operation.revision(), 2);
    assert!(
        owner
            .claim_dispatch(outbox, id(20)?, t(200)?, t(101)?)
            .is_err()
    );
    owner.register_worker(worker(21)?, t(101)?)?;
    assert!(
        owner
            .claim_dispatch(outbox, id(21)?, t(200)?, t(101)?)
            .is_err(),
        "retired claim must require a fresh approval/outbox before replacement dispatch"
    );
    owner.revoke_worker(id(20)?, t(102)?)?;
    assert_eq!(
        owner
            .state()
            .load_operation(id::<OperationId>(30)?)?
            .ok_or("operation")?
            .revision(),
        2
    );
    Ok(())
}
#[test]
fn worker_revocation_only_retires_claims_bound_to_that_generation() -> Result {
    let profile = Profile::new()?;
    let mut owner = owner(&profile)?;
    let unrelated = pending(&mut owner, 30)?;
    let claimed = pending(&mut owner, 31)?;
    let _lease = owner.claim_dispatch(claimed, id(20)?, t(200)?, t(100)?)?;

    owner.revoke_worker(id(20)?, t(101)?)?;

    assert_eq!(
        owner
            .state()
            .load_operation(id::<OperationId>(30)?)?
            .ok_or("unrelated operation")?
            .state(),
        DurableOperationState::DispatchPending
    );
    assert_eq!(
        owner
            .state()
            .load_operation(id::<OperationId>(31)?)?
            .ok_or("claimed operation")?
            .state(),
        DurableOperationState::DispatchPending
    );

    owner.register_worker(worker(21)?, t(101)?)?;
    assert!(
        owner
            .claim_dispatch(claimed, id(21)?, t(200)?, t(101)?)
            .is_err()
    );
    let replacement = owner.claim_dispatch(unrelated, id(21)?, t(200)?, t(101)?)?;
    let _attempt = owner.begin_authorized_dispatch(replacement, t(101)?)?;
    assert_eq!(
        owner
            .state()
            .load_operation(id::<OperationId>(30)?)?
            .ok_or("unrelated operation")?
            .state(),
        DurableOperationState::Attempting
    );
    Ok(())
}

#[test]
fn started_attempt_records_cancellation_intent_before_transport_settlement() -> Result {
    let profile = Profile::new()?;
    let mut owner = owner(&profile)?;
    let outbox = pending(&mut owner, 40)?;
    let lease = owner.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
    let attempt = owner.begin_authorized_dispatch(lease, t(100)?)?;
    let before = owner
        .state()
        .load_operation(id::<OperationId>(40)?)?
        .ok_or("operation")?;
    assert_eq!(before.state(), DurableOperationState::Attempting);
    assert_eq!(before.revision(), 3);

    owner.revoke_worker(id(20)?, t(101)?)?;

    let intent = owner
        .state()
        .load_operation(id::<OperationId>(40)?)?
        .ok_or("operation")?;
    assert_eq!(intent.state(), DurableOperationState::Attempting);
    assert_eq!(
        intent.revision(),
        4,
        "revocation must durably journal cancellation intent before notification"
    );
    owner.revoke_worker(id(20)?, t(101)?)?;
    assert_eq!(
        owner
            .state()
            .load_operation(id::<OperationId>(40)?)?
            .ok_or("operation")?
            .revision(),
        4,
        "duplicate revocation must not append duplicate cancellation intent"
    );

    owner.record_transport_observation(attempt, TransportObservation::OutcomeUnknown, t(102)?)?;
    let settled = owner
        .state()
        .load_operation(id::<OperationId>(40)?)?
        .ok_or("operation")?;
    assert_eq!(settled.state(), DurableOperationState::NeedsReconciliation);
    assert_eq!(settled.revision(), 5);
    Ok(())
}

#[test]
fn restart_after_started_cancellation_intent_never_revives_the_attempt() -> Result {
    let profile = Profile::new()?;
    {
        let mut owner = owner(&profile)?;
        let outbox = pending(&mut owner, 41)?;
        let lease = owner.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
        let _attempt = owner.begin_authorized_dispatch(lease, t(100)?)?;
        owner.revoke_worker(id(20)?, t(101)?)?;
        let operation = owner
            .state()
            .load_operation(id::<OperationId>(41)?)?
            .ok_or("operation")?;
        assert_eq!(operation.state(), DurableOperationState::Attempting);
        assert_eq!(operation.revision(), 4);
    }

    let mut next = RuntimeOwner::open_profile(&profile.0, t(102)?)?;
    let batch = next.plan_startup(t(102)?, 128)?;
    assert!(!batch.plans.is_empty());
    let recovered = next
        .state()
        .load_operation(id::<OperationId>(41)?)?
        .ok_or("operation")?;
    assert_eq!(
        recovered.state(),
        DurableOperationState::NeedsReconciliation
    );
    assert!(next.activate_after_planning(t(102)?).is_ok());
    Ok(())
}
