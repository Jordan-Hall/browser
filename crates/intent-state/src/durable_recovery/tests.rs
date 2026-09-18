use super::*;
use crate::{ArtifactScope, NewDurableOperation, OperationTransition, WorkspaceGraph};
use hmac::{Hmac, Mac};
use intent_contracts::{
    AccountId, ActionProposalId, BoundedText, CapabilityId, OperationAttemptId, OutboxMessageId,
    SchemaVersion, Task, TaskId, Workspace, WorkspaceId,
};
use intent_recovery::{AttemptBinding, RecoveryDisposition, RecoveryEffect};
use sha2::Sha256;
use std::{
    error::Error,
    fs,
    os::unix::fs::{DirBuilderExt, PermissionsExt},
    path::PathBuf,
};
type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;
const KEY: [u8; 32] = [83; 32];
struct Profile(PathBuf);
impl Profile {
    fn new() -> Result<Self> {
        let path = std::env::temp_dir().join(format!("intent-owner-{}", Uuid::new_v4()));
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
fn scope() -> Result<ArtifactScope> {
    Ok(ArtifactScope::try_new("owner-scope")?)
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
    owner.update_authority(authority(0)?, t(100)?)?;
    owner.register_worker(worker(20)?, t(100)?)?;
    Ok(owner)
}
fn authority(rev: u64) -> Result<AuthorityUpdate> {
    Ok(AuthorityUpdate {
        account_id: id(12)?,
        capability_id: id(13)?,
        expected_revision: rev,
        effect: RecoveryEffect::ExternalWrite,
        enabled: true,
        source_revision: digest(b"source-v1"),
        valid_until: t(1_000_000)?,
        evidence_key_id: EvidenceVerifier::new(KEY)?.key_id(),
    })
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
    Ok(RecoverableAction {
        operation: NewDurableOperation {
            operation_id: id(n)?,
            task_id: id(2)?,
            action_proposal_id: ActionProposalId::from_uuid(Uuid::new_v4()),
            account_id: id(12)?,
            capability_id: id(13)?,
            arguments_hash: digest(b"exact approved secret action"),
            source_schema: SchemaVersion::V1,
            created_at: t(100)?,
        },
        scope: scope()?,
        effect: RecoveryEffect::ExternalWrite,
        source_revision: digest(b"source-v1"),
        destination: BoundedText::try_new("fixture://external-ledger")?,
        message_kind: BoundedText::try_new("create")?,
        payload: b"exact approved secret action".to_vec(),
        deadline: t(900_000)?,
        compensation: None,
    })
}
fn pending(o: &mut RuntimeOwner, n: u64) -> Result<OutboxMessageId> {
    o.prepare_action(action(n)?)?;
    o.approve_action(id(n)?, 0, t(800_000)?, t(100)?)?;
    Ok(o.enqueue_action(id(n)?, 1, t(100)?)?)
}
fn started(o: &mut RuntimeOwner, n: u64) -> Result<DurableSendAttempt> {
    let outbox = pending(o, n)?;
    let lease = o.claim_dispatch(outbox, id(20)?, t(700_000)?, t(100)?)?;
    Ok(o.begin_authorized_dispatch(lease, t(100)?)?)
}
fn attestation(
    op: &DurableSendAttempt,
    verdict: ReconciliationVerdict,
) -> Result<ReadOnlyAttestation> {
    Ok(ReadOnlyAttestation {
        schema_version: 1,
        evidence_id: Uuid::new_v4(),
        binding: AttemptBinding {
            operation_id: op.operation_id(),
            account_id: id(12)?,
            capability_id: id(13)?,
            arguments_hash: digest(op.payload()),
            attempt_id: op.attempt_id(),
        },
        verdict,
        observed_at: t(100)?,
        valid_until: t(600_000)?,
    })
}
fn signed(value: &ReadOnlyAttestation) -> Result<VerifiedReadOnlyEvidence> {
    let bytes = serde_json::to_vec(value)?;
    let mut mac = Hmac::<Sha256>::new_from_slice(&KEY)?;
    mac.update(b"intent.read-only-reconciliation.v1\0");
    mac.update(&bytes);
    let tag: [u8; 32] = mac.finalize().into_bytes().into();
    Ok(EvidenceVerifier::new(KEY)?.verify(&bytes, &tag, t(100)?)?)
}
fn rev(o: &RuntimeOwner, n: u64) -> Result<u64> {
    Ok(o.state()
        .load_operation(id(n)?)?
        .ok_or("operation absent")?
        .revision())
}

#[test]
fn profile_lock_is_exclusive_and_survives_path_aliases() -> Result {
    let p = Profile::new()?;
    let first = RuntimeOwner::open_profile(&p.0, t(100)?)?;
    assert!(RuntimeOwner::open_profile(p.0.join("."), t(100)?).is_err());
    assert!(!first.state().dispatch_status()?.enabled);
    drop(first);
    let second = RuntimeOwner::open_profile(&p.0, t(100)?)?;
    assert!(!second.state().dispatch_status()?.enabled);
    Ok(())
}
#[test]
fn profile_rejects_public_permissions_symlinks_and_hardlinked_databases() -> Result {
    use std::os::unix::fs::symlink;
    let p = Profile::new()?;
    fs::set_permissions(&p.0, fs::Permissions::from_mode(0o755))?;
    assert!(RuntimeOwner::open_profile(&p.0, t(100)?).is_err());
    fs::set_permissions(&p.0, fs::Permissions::from_mode(0o700))?;
    let file = p.0.join("other");
    fs::write(&file, b"")?;
    fs::set_permissions(&file, fs::Permissions::from_mode(0o600))?;
    symlink(&file, p.0.join("state.sqlite3"))?;
    assert!(RuntimeOwner::open_profile(&p.0, t(100)?).is_err());
    fs::remove_file(p.0.join("state.sqlite3"))?;
    fs::hard_link(file, p.0.join("state.sqlite3"))?;
    assert!(RuntimeOwner::open_profile(&p.0, t(100)?).is_err());
    Ok(())
}
#[test]
fn exact_approved_dispatch_persists_identity_before_exposing_bytes() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let sent = started(&mut o, 30)?;
    let op = o.state().load_operation(id(30)?)?.ok_or("operation")?;
    assert_eq!(op.state(), DurableOperationState::Attempting);
    assert_eq!(op.attempt_identity(), Some(sent.attempt_id()));
    assert_eq!(op.revision(), 3);
    assert_eq!(sent.payload(), b"exact approved secret action");
    assert_eq!(o.state().operation_journal(id(30)?)?.len(), 4);
    assert!(!format!("{sent:?}").contains("secret"));
    assert!(!format!("{:?}", action(31)?).contains("secret"));
    let probe = StateStore::open(p.0.join("state.sqlite3"))?;
    assert!(probe.load_outbox(sent.outbox_id).is_err());
    o.record_transport_observation(sent, TransportObservation::AcceptedUnverified, t(100)?)?;
    assert_eq!(
        o.state()
            .load_operation(id(30)?)?
            .ok_or("operation")?
            .state(),
        DurableOperationState::Accepted
    );
    Ok(())
}
#[test]
fn managed_actions_reject_legacy_transition_stage_claim_and_result_bypasses() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let outbox = pending(&mut o, 30)?;
    assert!(
        o.state_mut()
            .transition_operation(
                id(30)?,
                OperationTransition {
                    expected_revision: 2,
                    next_state: DurableOperationState::Verified,
                    state_detail: None,
                    attempt_identity: None,
                    occurred_at: t(100)?
                }
            )
            .is_err()
    );
    assert!(
        o.state_mut()
            .claim_outbox(BoundedText::try_new("bypass")?, t(100)?, t(200)?, 10)?
            .is_empty()
    );
    assert!(
        o.state_mut()
            .begin_dispatch(outbox, &BoundedText::try_new("bypass")?, t(100)?)
            .is_err()
    );
    let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
    let _attempt = o.begin_authorized_dispatch(lease, t(100)?)?;
    assert!(
        o.state_mut()
            .record_dispatch_result(outbox, crate::DispatchResult::Accepted, t(100)?)
            .is_err()
    );
    assert_eq!(rev(&o, 30)?, 3);
    Ok(())
}
#[test]
fn revoked_worker_denies_claimed_dispatch_without_starting_attempt() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let outbox = pending(&mut o, 30)?;
    let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
    o.revoke_worker(id(20)?, t(100)?)?;
    assert!(o.begin_authorized_dispatch(lease, t(100)?).is_err());
    assert_eq!(rev(&o, 30)?, 2);
    assert!(o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?).is_err());
    Ok(())
}
#[test]
fn changed_authority_or_source_invalidates_an_already_issued_claim() -> Result {
    for source in [false, true] {
        let p = Profile::new()?;
        let mut o = owner(&p)?;
        let outbox = pending(&mut o, 30)?;
        let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
        let mut a = authority(1)?;
        if source {
            a.source_revision = digest(b"changed-source");
        } else {
            a.enabled = false;
        }
        o.update_authority(a, t(100)?)?;
        assert!(o.begin_authorized_dispatch(lease, t(100)?).is_err());
        assert_eq!(rev(&o, 30)?, 2);
    }
    Ok(())
}
#[test]
fn exact_payload_destination_and_schema_are_bound_before_approval() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let mut a = action(30)?;
    a.payload = b"substitute".to_vec();
    assert!(o.prepare_action(a).is_err());
    let mut a = action(30)?;
    a.scope = ArtifactScope::try_new("other")?;
    assert!(o.prepare_action(a).is_err());
    let mut a = action(30)?;
    a.operation.task_id = id(333)?;
    assert!(o.prepare_action(a).is_err());
    let outbox = pending(&mut o, 30)?;
    let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
    o.store
        .connection
        .execute_batch("DROP TRIGGER outbox_immutable_identity;")?;
    o.store.connection.execute(
        "UPDATE outbox_messages SET destination='fixture://unapproved' WHERE outbox_id=?1",
        [outbox.to_string()],
    )?;
    assert!(o.begin_authorized_dispatch(lease, t(100)?).is_err());
    assert_eq!(rev(&o, 30)?, 2);
    Ok(())
}
#[test]
fn claims_are_exclusive_and_expired_claim_tokens_cannot_steal_new_lease() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let outbox = pending(&mut o, 30)?;
    let old = o.claim_dispatch(outbox, id(20)?, t(110)?, t(100)?)?;
    assert!(o.claim_dispatch(outbox, id(20)?, t(120)?, t(100)?).is_err());
    let new = o.claim_dispatch(outbox, id(20)?, t(120)?, t(110)?)?;
    assert!(o.begin_authorized_dispatch(old, t(110)?).is_err());
    o.begin_authorized_dispatch(new, t(110)?)?;
    Ok(())
}
#[test]
fn worker_scope_capability_and_expiry_are_checked_at_dispatch() -> Result {
    for case in 0..3 {
        let p = Profile::new()?;
        let mut o = owner(&p)?;
        let outbox = pending(&mut o, 30)?;
        let mut w = worker(21)?;
        match case {
            0 => w.account_id = id(444)?,
            1 => w.task_id = id(555)?,
            _ => w.capabilities = vec![id(666)?],
        };
        o.register_worker(w, t(100)?)?;
        assert!(o.claim_dispatch(outbox, id(21)?, t(200)?, t(100)?).is_err());
    }
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let outbox = pending(&mut o, 30)?;
    let mut w = worker(21)?;
    w.expires_at = t(101)?;
    o.register_worker(w, t(100)?)?;
    let lease = o.claim_dispatch(outbox, id(21)?, t(110)?, t(100)?)?;
    assert!(o.begin_authorized_dispatch(lease, t(101)?).is_err());
    Ok(())
}
#[test]
fn stale_approvals_expiry_and_wall_clock_rollback_fail_closed() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    o.prepare_action(action(30)?)?;
    o.approve_action(id(30)?, 0, t(101)?, t(100)?)?;
    assert!(o.enqueue_action(id(30)?, 1, t(101)?).is_err());
    assert!(o.approve_action(id(30)?, 0, t(1000)?, t(100)?).is_err());
    o.update_authority(authority(1)?, t(200)?)?;
    assert!(o.approve_action(id(30)?, 1, t(1000)?, t(199)?).is_err());
    Ok(())
}
#[test]
fn unaccepted_external_effect_is_fenced_and_reconciled_after_owner_restart() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let attempt = started(&mut o, 30)?;
    let evidence = attestation(
        &attempt,
        ReconciliationVerdict::Committed {
            receipt: digest(b"independently observed receipt"),
        },
    )?;
    let old_epoch = o.epoch();
    drop(o);
    let mut next = RuntimeOwner::open_profile(&p.0, t(100)?)?;
    assert_ne!(old_epoch, next.epoch());
    assert!(!next.state().dispatch_status()?.enabled);
    assert!(next.activate_after_planning(t(100)?).is_err());
    let batch = next.plan_startup(t(100)?, 1)?;
    assert!(!batch.remaining);
    assert_eq!(
        batch.plans[0].decision.disposition,
        RecoveryDisposition::ReadOnlyReconciliation
    );
    assert_eq!(
        batch.plans[0].state,
        DurableOperationState::NeedsReconciliation
    );
    next.activate_after_planning(t(100)?)?;
    assert!(
        next.record_transport_observation(
            attempt,
            TransportObservation::AcceptedUnverified,
            t(100)?
        )
        .is_err()
    );
    assert!(next.approve_action(id(30)?, 4, t(1000)?, t(100)?).is_err());
    next.reconcile(signed(&evidence)?, 4, t(100)?)?;
    assert_eq!(
        next.state()
            .load_operation(id(30)?)?
            .ok_or("operation")?
            .state(),
        DurableOperationState::Verified
    );
    Ok(())
}
#[test]
fn final_receipt_survives_grant_revocation_but_cannot_authorize_retry() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let attempt = started(&mut o, 30)?;
    let evidence = attestation(
        &attempt,
        ReconciliationVerdict::Committed {
            receipt: digest(b"receipt"),
        },
    )?;
    let mut a = authority(1)?;
    a.enabled = false;
    o.update_authority(a, t(100)?)?;
    o.reconcile(signed(&evidence)?, 3, t(100)?)?;
    assert_eq!(rev(&o, 30)?, 4);
    assert!(o.approve_action(id(30)?, 4, t(1000)?, t(100)?).is_err());
    assert_eq!(
        o.recovery_view(id(30)?, t(100)?)?.plan.decision.disposition,
        RecoveryDisposition::NoReplayKnownCommit
    );
    Ok(())
}
#[test]
fn evidence_mutations_wrong_signature_and_reused_id_never_change_state() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let attempt = started(&mut o, 30)?;
    let correct = attestation(
        &attempt,
        ReconciliationVerdict::Committed {
            receipt: digest(b"receipt"),
        },
    )?;
    for mutation in 0..5 {
        let mut value = correct.clone();
        match mutation {
            0 => value.binding.operation_id = id(31)?,
            1 => value.binding.account_id = id::<AccountId>(99)?,
            2 => value.binding.capability_id = id::<CapabilityId>(99)?,
            3 => value.binding.attempt_id = id::<OperationAttemptId>(99)?,
            _ => value.binding.arguments_hash = digest(b"other"),
        };
        assert!(o.reconcile(signed(&value)?, 3, t(100)?).is_err());
        assert_eq!(rev(&o, 30)?, 3);
    }
    let bytes = serde_json::to_vec(&correct)?;
    assert!(
        EvidenceVerifier::new(KEY)?
            .verify(&bytes, &[0; 32], t(100)?)
            .is_err()
    );
    o.reconcile(signed(&correct)?, 3, t(100)?)?;
    let journal = o.state().operation_journal(id(30)?)?.len();
    o.reconcile(signed(&correct)?, 3, t(100)?)?;
    assert_eq!(o.state().operation_journal(id(30)?)?.len(), journal);
    let mut different = correct;
    different.verdict = ReconciliationVerdict::AuthoritativeNonCommit {
        observation: digest(b"false"),
    };
    assert!(o.reconcile(signed(&different)?, 4, t(100)?).is_err());
    Ok(())
}
#[test]
fn inconclusive_evidence_never_turns_into_noncommit_and_conflicts_fail_closed() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let attempt = started(&mut o, 30)?;
    o.reconcile(
        signed(&attestation(&attempt, ReconciliationVerdict::Inconclusive)?)?,
        3,
        t(100)?,
    )?;
    assert!(o.approve_action(id(30)?, 4, t(1000)?, t(100)?).is_err());
    o.reconcile(
        signed(&attestation(
            &attempt,
            ReconciliationVerdict::Committed {
                receipt: digest(b"receipt"),
            },
        )?)?,
        4,
        t(100)?,
    )?;
    assert!(
        o.reconcile(
            signed(&attestation(
                &attempt,
                ReconciliationVerdict::AuthoritativeNonCommit {
                    observation: digest(b"absent")
                }
            )?)?,
            5,
            t(100)?
        )
        .is_err()
    );
    o.reconcile(
        signed(&attestation(&attempt, ReconciliationVerdict::Inconclusive)?)?,
        5,
        t(100)?,
    )?;
    assert_eq!(rev(&o, 30)?, 5);
    assert_eq!(
        o.state()
            .load_operation(id(30)?)?
            .ok_or("operation")?
            .state(),
        DurableOperationState::Verified
    );
    Ok(())
}
#[test]
fn proven_noncommit_requires_fresh_approval_and_distinct_attempt() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let old = started(&mut o, 30)?;
    o.reconcile(
        signed(&attestation(
            &old,
            ReconciliationVerdict::AuthoritativeNonCommit {
                observation: digest(b"definitive rejection"),
            },
        )?)?,
        3,
        t(100)?,
    )?;
    assert!(o.enqueue_action(id(30)?, 4, t(100)?).is_err());
    o.approve_action(id(30)?, 4, t(1000)?, t(100)?)?;
    let outbox = o.enqueue_action(id(30)?, 5, t(100)?)?;
    let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
    let new = o.begin_authorized_dispatch(lease, t(100)?)?;
    assert_ne!(new.attempt_id(), old.attempt_id());
    assert_eq!(
        o.store.connection.query_row(
            "SELECT count(*) FROM recovery_attempts WHERE operation_id=?1",
            [id::<OperationId>(30)?.to_string()],
            |r| r.get::<_, i64>(0)
        )?,
        2
    );
    Ok(())
}
#[test]
fn compensation_has_its_own_approval_bytes_attempt_and_original_receipt() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let original = started(&mut o, 30)?;
    let receipt = digest(b"original-receipt");
    o.reconcile(
        signed(&attestation(
            &original,
            ReconciliationVerdict::Committed { receipt },
        )?)?,
        3,
        t(100)?,
    )?;
    let mut compensation = action(31)?;
    compensation.payload = b"separate compensation bytes".to_vec();
    compensation.operation.arguments_hash = digest(&compensation.payload);
    compensation.compensation = Some(CompensationOrigin {
        operation_id: id(30)?,
        attempt_id: original.attempt_id(),
        receipt,
    });
    o.prepare_action(compensation)?;
    assert!(o.enqueue_action(id(31)?, 0, t(100)?).is_err());
    o.approve_action(id(31)?, 0, t(1000)?, t(100)?)?;
    let outbox = o.enqueue_action(id(31)?, 1, t(100)?)?;
    let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
    let attempt = o.begin_authorized_dispatch(lease, t(100)?)?;
    assert_ne!(original.attempt_id(), attempt.attempt_id());
    o.reconcile(
        signed(&attestation(
            &attempt,
            ReconciliationVerdict::Committed {
                receipt: digest(b"compensation-receipt"),
            },
        )?)?,
        3,
        t(100)?,
    )?;
    let op = o.state().load_operation(id(30)?)?.ok_or("operation")?;
    assert_eq!(op.state(), DurableOperationState::Compensated);
    let view = o.recovery_view(id(30)?, t(100)?)?;
    assert_eq!(view.plan.compensation_operation, Some(id(31)?));
    assert_eq!(
        view.plan.decision.disposition,
        RecoveryDisposition::NoReplayKnownCommit
    );
    assert_eq!(
        o.recovery_view(id(31)?, t(100)?)?.plan.decision.disposition,
        RecoveryDisposition::NoReplayCompensationConfirmed
    );

    assert_eq!(op.arguments_hash(), digest(original.payload()));
    assert_eq!(op.attempt_identity(), Some(original.attempt_id()));
    assert_eq!(
        o.store.connection.query_row(
            "SELECT count(*) FROM recovery_evidence WHERE verdict='committed'",
            [],
            |r| r.get::<_, i64>(0)
        )?,
        2
    );
    Ok(())
}
#[test]
fn unknown_original_effect_cannot_be_marked_compensated() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let original = started(&mut o, 30)?;
    let mut compensation = action(31)?;
    compensation.compensation = Some(CompensationOrigin {
        operation_id: id(30)?,
        attempt_id: original.attempt_id(),
        receipt: digest(b"invented"),
    });
    assert!(o.prepare_action(compensation).is_err());
    assert_eq!(rev(&o, 30)?, 3);
    Ok(())
}
#[test]
fn revoked_evidence_key_and_evidence_expired_after_verification_are_denied() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let attempt = started(&mut o, 30)?;
    let e = attestation(
        &attempt,
        ReconciliationVerdict::Committed {
            receipt: digest(b"receipt"),
        },
    )?;
    let verified = signed(&e)?;
    assert!(o.reconcile(verified, 3, t(600_000)?).is_err());
    o.revoke_evidence_key(
        id(12)?,
        id(13)?,
        EvidenceVerifier::new(KEY)?.key_id(),
        t(100)?,
    )?;
    assert!(o.reconcile(signed(&e)?, 3, t(100)?).is_err());
    assert_eq!(rev(&o, 30)?, 3);
    Ok(())
}
#[test]
fn recovery_controls_revalidate_view_and_never_offer_blind_retry() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    pending(&mut o, 30)?;
    let view = o.recovery_view(id(30)?, t(100)?)?;
    assert!(view.actions.contains(&RecoveryUiAction::CancelUnsent));
    o.apply_recovery_control(&view, RecoveryUiAction::CancelUnsent, t(100)?)?;
    assert!(
        o.apply_recovery_control(&view, RecoveryUiAction::CancelUnsent, t(100)?)
            .is_err()
    );
    let attempt = started(&mut o, 31)?;
    let view = o.recovery_view(id(31)?, t(100)?)?;
    assert!(!view.actions.contains(&RecoveryUiAction::CancelUnsent));
    assert!(
        !view
            .actions
            .contains(&RecoveryUiAction::ReviewFreshApproval)
    );
    assert!(
        o.apply_recovery_control(&view, RecoveryUiAction::CancelUnsent, t(100)?)
            .is_err()
    );
    drop(attempt);
    Ok(())
}
#[test]
fn startup_batches_are_bounded_idempotent_and_activation_rejects_missing_work() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    for n in 30..33 {
        o.prepare_action(action(n)?)?;
    }
    drop(o);
    let mut o = RuntimeOwner::open_profile(&p.0, t(100)?)?;
    assert!(o.plan_startup(t(100)?, 129).is_err());
    assert!(o.activate_after_planning(t(100)?).is_err());
    for count in 0..3 {
        let page = o.plan_startup(t(100)?, 1)?;
        assert_eq!(page.plans.len(), 1);
        assert_eq!(page.remaining, count != 2);
    }
    assert!(o.plan_startup(t(100)?, 1)?.plans.is_empty());
    o.activate_after_planning(t(100)?)?;
    assert!(o.state().dispatch_status()?.enabled);
    Ok(())
}
#[test]
fn migration_and_plan_failures_never_enable_an_unreviewed_runtime() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let _ = started(&mut o, 30)?;
    drop(o);
    let mut o = RuntimeOwner::open_profile(&p.0, t(100)?)?;
    o.store.connection.execute_batch("CREATE TRIGGER injected_plan_failure BEFORE INSERT ON recovery_plans BEGIN SELECT RAISE(ABORT,'injected'); END;")?;
    assert!(o.plan_startup(t(100)?, 128).is_err());
    assert!(!o.state().dispatch_status()?.enabled);
    assert!(o.activate_after_planning(t(100)?).is_err());
    assert_eq!(
        o.state()
            .load_operation(id(30)?)?
            .ok_or("operation")?
            .state(),
        DurableOperationState::Attempting
    );
    Ok(())
}
#[test]
fn sql_attempt_and_evidence_history_are_immutable() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let attempt = started(&mut o, 30)?;
    o.reconcile(
        signed(&attestation(
            &attempt,
            ReconciliationVerdict::Committed {
                receipt: digest(b"receipt"),
            },
        )?)?,
        3,
        t(100)?,
    )?;
    for sql in [
        "UPDATE recovery_actions SET destination='different'",
        "UPDATE recovery_attempts SET worker_id='different'",
        "UPDATE recovery_attempts SET attempt_id='different'",
        "UPDATE recovery_evidence SET verdict='not_committed'",
        "DELETE FROM recovery_evidence",
    ] {
        assert!(o.store.connection.execute_batch(sql).is_err());
    }
    Ok(())
}

#[test]
fn restored_pre_dispatch_snapshot_cannot_blindly_repeat_a_later_external_effect() -> Result {
    let p = Profile::new()?;
    let export_parent = Profile::new()?;
    let mut o = owner(&p)?;
    let outbox = pending(&mut o, 30)?;
    let key = crate::BackupKey::from_bytes([49; 32]);
    let snapshot = export_parent.0.join("snapshot");
    let restore = export_parent.0.join("restored");
    o.state_mut().export_authenticated_plaintext_snapshot(
        &p.0,
        &snapshot,
        &key,
        t(100)?,
        crate::SnapshotLimits::default(),
        crate::PlaintextExportConsent::SensitiveDataWillBeWrittenUnencrypted,
    )?;
    let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
    let attempt = o.begin_authorized_dispatch(lease, t(100)?)?;
    // This independently observed side effect is outside the restored profile's rollback domain.
    let ledger = export_parent.0.join("external-effects");
    fs::write(&ledger, attempt.attempt_id().to_string())?;
    fs::File::open(&ledger)?.sync_all()?;
    StateStore::restore_authenticated_plaintext_snapshot(
        &snapshot,
        &restore,
        &key,
        crate::SnapshotLimits::default(),
    )?;
    let mut restored = RuntimeOwner::open_profile(&restore, t(100)?)?;
    let page = restored.plan_startup(t(100)?, 128)?;
    assert_eq!(
        page.plans[0].decision.disposition,
        RecoveryDisposition::RestoreHistoryUncertain
    );
    restored.activate_after_planning(t(100)?)?;
    restored.update_authority(authority(1)?, t(100)?)?;
    assert!(
        restored
            .approve_action(id(30)?, 2, t(300)?, t(100)?)
            .is_err()
    );
    drop(restored);
    let mut reopened = RuntimeOwner::open_profile(&restore, t(100)?)?;
    reopened.plan_startup(t(100)?, 128)?;
    reopened.activate_after_planning(t(100)?)?;
    assert!(
        reopened
            .approve_action(id(30)?, 2, t(300)?, t(100)?)
            .is_err()
    );
    assert_eq!(
        fs::read_to_string(ledger)?,
        attempt.attempt_id().to_string()
    );
    Ok(())
}
#[test]
fn startup_planner_cannot_reclassify_current_live_attempts() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let _ = started(&mut o, 30)?;
    assert!(o.plan_startup(t(100)?, 128).is_err());
    assert_eq!(rev(&o, 30)?, 3);
    Ok(())
}
#[test]
fn real_process_kill_preserves_attempts_and_independent_external_effects() -> Result {
    use std::{
        process::{Command, Stdio},
        thread,
        time::{Duration, Instant},
    };
    for phase in [
        "before_start",
        "after_start",
        "after_effect",
        "after_observation",
    ] {
        let profile = Profile::new()?;
        let outside = Profile::new()?;
        let ready = outside.0.join("ready");
        let ledger = outside.0.join("effects");
        let mut child = Command::new(std::env::current_exe()?)
            .args([
                "--exact",
                "durable_recovery::tests::crash_fixture",
                "--nocapture",
            ])
            .env_clear()
            .env("INTENT_RECOVERY_CRASH_ROOT", &profile.0)
            .env("INTENT_RECOVERY_CRASH_PHASE", phase)
            .env("INTENT_RECOVERY_READY", &ready)
            .env("INTENT_RECOVERY_LEDGER", &ledger)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()?;
        let deadline = Instant::now() + Duration::from_secs(10);
        while !ready.exists() {
            if child.try_wait()?.is_some() {
                return Err("crash fixture exited before barrier".into());
            }
            if Instant::now() >= deadline {
                child.kill()?;
                child.wait()?;
                return Err("crash fixture barrier timed out".into());
            }
            thread::sleep(Duration::from_millis(5));
        }
        child.kill()?;
        child.wait()?;
        let mut o = RuntimeOwner::open_profile(&profile.0, t(100)?)?;
        let batch = o.plan_startup(t(100)?, 128)?;
        let op = o
            .state()
            .load_operation(id(30)?)?
            .ok_or("missing durable intent after kill")?;
        if phase == "before_start" {
            assert_eq!(op.state(), DurableOperationState::DispatchPending);
            assert!(!ledger.exists());
        } else {
            assert_eq!(op.state(), DurableOperationState::NeedsReconciliation);
            assert_eq!(
                batch.plans[0].decision.disposition,
                RecoveryDisposition::ReadOnlyReconciliation
            );
        }
        if phase == "after_effect" || phase == "after_observation" {
            let observed = fs::read_to_string(&ledger)?;
            assert_eq!(
                observed,
                op.attempt_identity().ok_or("attempt identity")?.to_string()
            );
            let evidence = ReadOnlyAttestation {
                schema_version: 1,
                evidence_id: Uuid::new_v4(),
                binding: AttemptBinding {
                    operation_id: op.operation_id(),
                    account_id: op.account_id(),
                    capability_id: op.capability_id(),
                    arguments_hash: op.arguments_hash(),
                    attempt_id: op.attempt_identity().ok_or("attempt")?,
                },
                verdict: ReconciliationVerdict::Committed {
                    receipt: digest(observed.as_bytes()),
                },
                observed_at: t(100)?,
                valid_until: t(1000)?,
            };
            o.reconcile(signed(&evidence)?, op.revision(), t(100)?)?;
            assert!(
                o.approve_action(id(30)?, op.revision() + 1, t(300)?, t(100)?)
                    .is_err()
            );
            assert_eq!(fs::read_to_string(&ledger)?, observed);
        }
    }
    Ok(())
}
#[test]
fn crash_fixture() -> Result {
    use std::{io::Write, thread, time::Duration};
    let Some(root) = std::env::var_os("INTENT_RECOVERY_CRASH_ROOT") else {
        return Ok(());
    };
    let profile = Profile(PathBuf::from(root));
    let mut o = owner(&profile)?;
    let outbox = pending(&mut o, 30)?;
    let phase = std::env::var("INTENT_RECOVERY_CRASH_PHASE")?;
    if phase != "before_start" {
        let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
        let attempt = o.begin_authorized_dispatch(lease, t(100)?)?;
        if phase == "after_effect" || phase == "after_observation" {
            let mut ledger = fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(std::env::var_os("INTENT_RECOVERY_LEDGER").ok_or("ledger path")?)?;
            write!(&mut ledger, "{}", attempt.attempt_id())?;
            ledger.sync_all()?;
        }
        if phase == "after_observation" {
            o.record_transport_observation(
                attempt,
                TransportObservation::AcceptedUnverified,
                t(100)?,
            )?;
        }
    }
    fs::write(
        std::env::var_os("INTENT_RECOVERY_READY").ok_or("ready path")?,
        b"ready",
    )?;
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

#[test]
fn capability_policy_cannot_be_downgraded_by_an_action_payload() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let mut a = action(30)?;
    a.effect = RecoveryEffect::ReadOnly;
    assert!(o.prepare_action(a).is_err());
    let mut a = authority(1)?;
    a.effect = RecoveryEffect::Unknown;
    assert!(o.update_authority(a, t(100)?).is_err());
    let outbox = pending(&mut o, 30)?;
    let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
    let mut a = authority(1)?;
    a.effect = RecoveryEffect::ReadOnly;
    o.update_authority(a, t(100)?)?;
    assert!(o.begin_authorized_dispatch(lease, t(100)?).is_err());
    Ok(())
}
#[test]
fn read_capture_and_local_versions_preserve_distinct_effect_semantics() -> Result {
    for effect in [RecoveryEffect::ReadOnly, RecoveryEffect::LocalReversible] {
        let p = Profile::new()?;
        let mut o = owner(&p)?;
        let mut authority = authority(1)?;
        authority.effect = effect;
        o.update_authority(authority, t(100)?)?;
        let mut a = action(30)?;
        a.effect = effect;
        o.prepare_action(a)?;
        o.approve_action(id(30)?, 0, t(1000)?, t(100)?)?;
        let outbox = o.enqueue_action(id(30)?, 1, t(100)?)?;
        let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
        let attempt = o.begin_authorized_dispatch(lease, t(100)?)?;
        assert!(
            o.reconcile(
                signed(&attestation(
                    &attempt,
                    ReconciliationVerdict::Committed {
                        receipt: digest(b"external")
                    }
                )?)?,
                3,
                t(100)?
            )
            .is_err()
        );
        let (verdict, expected) = if effect == RecoveryEffect::ReadOnly {
            (
                ReconciliationVerdict::ReadCompleted {
                    capture: digest(b"read capture"),
                },
                RecoveryDisposition::NoReplayCapturedRead,
            )
        } else {
            (
                ReconciliationVerdict::LocalCommitted {
                    before_revision: digest(b"source-v1"),
                    after_revision: digest(b"source-v2"),
                    revision: 2,
                    receipt: digest(b"local-postcondition"),
                },
                RecoveryDisposition::NoReplayLocalCommit,
            )
        };
        o.reconcile(signed(&attestation(&attempt, verdict)?)?, 3, t(100)?)?;
        assert_eq!(
            o.recovery_view(id(30)?, t(100)?)?.plan.decision.disposition,
            expected
        );
    }
    Ok(())
}
#[test]
fn malformed_oversized_duplicate_field_and_noncanonical_evidence_is_rejected() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let attempt = started(&mut o, 30)?;
    let value = attestation(&attempt, ReconciliationVerdict::Inconclusive)?;
    let valid = serde_json::to_string(&value)?;
    for bytes in [
        format!(" {valid}").into_bytes(),
        valid
            .replace(
                "\"schema_version\":1",
                "\"schema_version\":1,\"schema_version\":1",
            )
            .into_bytes(),
        vec![b'x'; 8193],
    ] {
        let mut mac = Hmac::<Sha256>::new_from_slice(&KEY)?;
        mac.update(b"intent.read-only-reconciliation.v1\0");
        mac.update(&bytes);
        let tag: [u8; 32] = mac.finalize().into_bytes().into();
        assert!(
            EvidenceVerifier::new(KEY)?
                .verify(&bytes, &tag, t(100)?)
                .is_err()
        );
    }
    Ok(())
}
#[test]
fn authority_cas_and_failed_attempt_commit_leave_no_partial_dispatch() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    assert!(o.update_authority(authority(0)?, t(100)?).is_err());
    let outbox = pending(&mut o, 30)?;
    let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
    o.store.connection.execute_batch("CREATE TRIGGER injected_journal_failure BEFORE INSERT ON operation_journal WHEN NEW.to_state='attempting' BEGIN SELECT RAISE(ABORT,'injected'); END;")?;
    assert!(o.begin_authorized_dispatch(lease, t(100)?).is_err());
    assert_eq!(rev(&o, 30)?, 2);
    let started: i64 = o.store.connection.query_row(
        "SELECT count(*) FROM recovery_attempts WHERE started_at_micros IS NOT NULL",
        [],
        |r| r.get(0),
    )?;
    assert_eq!(started, 0);
    assert_eq!(o.state().operation_journal(id(30)?)?.len(), 3);
    Ok(())
}
