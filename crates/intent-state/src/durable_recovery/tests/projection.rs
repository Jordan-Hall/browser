use super::*;
use intent_contracts::{
    ExecutionObservationData, ExecutionOutcome, ExecutionPhase, ExecutionStage,
};

fn projected(o: &RuntimeOwner, n: u64) -> Result<ExecutionObservationData> {
    let record = o.project_operation(id(n)?)?;
    assert!(record.idempotency_key().is_none());
    let decoded: intent_contracts::Operation =
        serde_json::from_slice(&serde_json::to_vec(&record)?)?;
    assert_eq!(decoded, record);
    Ok(record.execution().ok_or("execution")?.data().clone())
}

#[test]
fn projection_preserves_approval_dispatch_and_evidence_boundaries() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    o.prepare_action(action(30)?)?;
    assert_eq!(projected(&o, 30)?.stage, ExecutionStage::Prepared);
    o.approve_action(id(30)?, 0, t(800_000)?, t(100)?)?;
    assert_eq!(projected(&o, 30)?.stage, ExecutionStage::Approved);
    let outbox = o.enqueue_action(id(30)?, 1, t(100)?)?;
    let pending = projected(&o, 30)?;
    assert_eq!(pending.stage, ExecutionStage::DispatchPending);
    assert!(pending.attempt.ok_or("attempt")?.started_at.is_none());
    let lease = o.claim_dispatch(outbox, id(20)?, t(700_000)?, t(100)?)?;
    let sent = o.begin_authorized_dispatch(lease, t(100)?)?;
    assert_eq!(projected(&o, 30)?.stage, ExecutionStage::Attempting);
    let inconclusive = attestation(&sent, ReconciliationVerdict::Inconclusive)?;
    let committed = attestation(
        &sent,
        ReconciliationVerdict::Committed {
            receipt: digest(b"receipt"),
        },
    )?;
    o.record_transport_observation(sent, TransportObservation::AcceptedUnverified, t(100)?)?;
    let accepted = projected(&o, 30)?;
    assert_eq!(accepted.stage, ExecutionStage::Accepted);
    assert!(accepted.evidence.is_none());
    assert_eq!(
        accepted.attempt.map(|a| a.id),
        pending.attempt.map(|a| a.id)
    );
    o.reconcile(signed(&inconclusive)?, rev(&o, 30)?, t(100)?)?;
    let unresolved = projected(&o, 30)?;
    assert_eq!(unresolved.stage, ExecutionStage::NeedsReconciliation);
    assert_eq!(
        unresolved.evidence.ok_or("evidence")?.outcome,
        ExecutionOutcome::Inconclusive {}
    );
    o.reconcile(signed(&committed)?, rev(&o, 30)?, t(100)?)?;
    let verified = projected(&o, 30)?;
    assert_eq!(verified.stage, ExecutionStage::Verified);
    assert_eq!(
        verified.evidence.as_ref().ok_or("evidence")?.outcome,
        ExecutionOutcome::ExternalCommitted {
            receipt: digest(b"receipt")
        }
    );
    let journal = o.state().operation_journal(id(30)?)?;
    assert_eq!(projected(&o, 30)?, verified);
    assert_eq!(o.state().operation_journal(id(30)?)?, journal);
    drop(o);
    let reopened = RuntimeOwner::open_profile(&p.0, t(100)?)?;
    assert_eq!(projected(&reopened, 30)?, verified);
    Ok(())
}

#[test]
fn projection_preserves_unknown_noncommit_and_cancelled_unstarted_attempts() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let sent = started(&mut o, 30)?;
    let noncommit = attestation(
        &sent,
        ReconciliationVerdict::AuthoritativeNonCommit {
            observation: digest(b"no effect"),
        },
    )?;
    o.record_transport_observation(sent, TransportObservation::OutcomeUnknown, t(100)?)?;
    assert!(projected(&o, 30)?.evidence.is_none());
    assert_eq!(
        projected(&o, 30)?.stage,
        ExecutionStage::NeedsReconciliation
    );
    o.reconcile(signed(&noncommit)?, rev(&o, 30)?, t(100)?)?;
    let failed = projected(&o, 30)?;
    assert_eq!(failed.stage, ExecutionStage::Failed);
    assert!(matches!(
        failed.evidence.ok_or("noncommit")?.outcome,
        ExecutionOutcome::ProvenNotCommitted { .. }
    ));
    o.approve_action(id(30)?, rev(&o, 30)?, t(500_000)?, t(100)?)?;
    assert!(projected(&o, 30)?.attempt.is_none());
    pending(&mut o, 31)?;
    let before = projected(&o, 31)?;
    let view = o.recovery_view(id(31)?, t(100)?)?;
    o.apply_recovery_control(&view, RecoveryUiAction::CancelUnsent, t(100)?)?;
    let cancelled = projected(&o, 31)?;
    assert_eq!(cancelled.stage, ExecutionStage::Cancelled);
    assert_eq!(cancelled.attempt, before.attempt);
    assert!(cancelled.attempt.ok_or("unstarted")?.started_at.is_none());
    Ok(())
}

#[test]
fn projection_keeps_both_sides_of_verified_compensation() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let original = started(&mut o, 30)?;
    let receipt = digest(b"original receipt");
    o.reconcile(
        signed(&attestation(
            &original,
            ReconciliationVerdict::Committed { receipt },
        )?)?,
        3,
        t(100)?,
    )?;
    let original_evidence = projected(&o, 30)?.evidence;
    let mut compensation = action(31)?;
    compensation.compensation = Some(CompensationOrigin {
        operation_id: id(30)?,
        attempt_id: original.attempt_id(),
        receipt,
    });
    o.prepare_action(compensation)?;
    assert_eq!(
        projected(&o, 31)?.phase,
        ExecutionPhase::Compensation {
            original_operation_id: id(30)?,
            original_attempt_id: original.attempt_id(),
            original_receipt: receipt
        }
    );
    o.approve_action(id(31)?, 0, t(1000)?, t(100)?)?;
    let outbox = o.enqueue_action(id(31)?, 1, t(100)?)?;
    let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
    let sent = o.begin_authorized_dispatch(lease, t(100)?)?;
    let evidence = attestation(
        &sent,
        ReconciliationVerdict::Committed {
            receipt: digest(b"compensation receipt"),
        },
    )?;
    o.record_transport_observation(sent, TransportObservation::AcceptedUnverified, t(100)?)?;
    assert_eq!(projected(&o, 30)?.stage, ExecutionStage::Verified);
    assert!(projected(&o, 30)?.compensation.is_none());
    o.reconcile(signed(&evidence)?, rev(&o, 31)?, t(150)?)?;
    let wire = serde_json::to_value(o.project_operation(id(30)?)?)?;
    assert_eq!(wire["last_reconciled_at"], 150);
    let mut stale_time = wire;
    stale_time["last_reconciled_at"] = serde_json::json!(100);
    assert!(serde_json::from_value::<intent_contracts::Operation>(stale_time).is_err());
    let original = projected(&o, 30)?;
    let compensation = projected(&o, 31)?;
    assert_eq!(original.stage, ExecutionStage::Compensated);
    assert_eq!(original.evidence, original_evidence);
    let link = original.compensation.as_ref().ok_or("link")?;
    assert_eq!(link.operation_id, id(31)?);
    assert_eq!(Some(link.attempt), compensation.attempt);
    assert_eq!(Some(&link.evidence), compensation.evidence.as_ref());
    drop(o);
    let reopened = RuntimeOwner::open_profile(&p.0, t(150)?)?;
    assert_eq!(projected(&reopened, 30)?, original);
    reopened
        .store
        .connection
        .execute_batch("DROP TRIGGER recovery_action_immutable")?;
    reopened.store.connection.execute(
        "UPDATE recovery_actions SET source_revision=?1 WHERE operation_id=?2",
        params![
            digest(b"wrong source").to_hex(),
            id::<OperationId>(30)?.to_string()
        ],
    )?;
    assert!(reopened.project_operation(id(30)?).is_err());
    assert!(reopened.project_operation(id(31)?).is_err());
    reopened.store.connection.execute(
        "UPDATE recovery_actions SET source_revision=?1 WHERE operation_id=?2",
        params![
            digest(b"source-v1").to_hex(),
            id::<OperationId>(30)?.to_string()
        ],
    )?;
    assert_eq!(projected(&reopened, 30)?, original);
    reopened.store.connection.execute(
        "UPDATE recovery_actions SET original_receipt=?1 WHERE operation_id=?2",
        params![
            digest(b"wrong original receipt").to_hex(),
            id::<OperationId>(31)?.to_string()
        ],
    )?;
    assert!(reopened.project_operation(id(30)?).is_err());
    assert!(reopened.project_operation(id(31)?).is_err());
    Ok(())
}

#[test]
fn projection_refuses_corrupt_missing_and_contradictory_evidence() -> Result {
    for corruption in [
        "hash",
        "binding",
        "columns",
        "missing",
        "contradiction",
        "combined_state",
    ] {
        let p = Profile::new()?;
        let mut o = owner(&p)?;
        let sent = started(&mut o, 30)?;
        let value = attestation(
            &sent,
            ReconciliationVerdict::Committed {
                receipt: digest(b"receipt"),
            },
        )?;
        o.reconcile(signed(&value)?, 3, t(100)?)?;
        let journal = o.state().operation_journal(id(30)?)?;
        o.store.connection.execute_batch(
            "DROP TRIGGER recovery_evidence_immutable; DROP TRIGGER recovery_evidence_no_delete;",
        )?;
        match corruption {
            "hash" => {
                o.store.connection.execute(
                    "UPDATE recovery_evidence SET payload_hash=?1",
                    [digest(b"wrong").to_hex()],
                )?;
            }
            "binding" => {
                let mut bad = value.clone();
                bad.binding.account_id = id(999)?;
                let bytes = serde_json::to_vec(&bad)?;
                o.store.connection.execute(
                    "UPDATE recovery_evidence SET payload=?1,payload_hash=?2",
                    params![&bytes, digest(&bytes).to_hex()],
                )?;
            }
            "columns" => {
                o.store
                    .connection
                    .execute("UPDATE recovery_evidence SET verdict='not_committed'", [])?;
            }
            "missing" => {
                o.store
                    .connection
                    .execute("DELETE FROM recovery_evidence", [])?;
            }
            "contradiction" => {
                o.store.connection.execute("INSERT INTO recovery_evidence SELECT ?1,operation_id,attempt_id,'not_committed',receipt,payload,payload_hash,key_id,authentication_tag,recorded_at_micros FROM recovery_evidence",[Uuid::new_v4().to_string()])?;
            }
            "combined_state" => {
                o.store
                    .connection
                    .execute("UPDATE durable_operations SET state='compensating'", [])?;
            }
            _ => unreachable!(),
        }
        assert!(o.project_operation(id(30)?).is_err(), "{corruption}");
        assert_eq!(o.state().operation_journal(id(30)?)?, journal);
    }
    Ok(())
}

#[test]
fn projection_rejects_same_columns_with_different_local_commit_outcomes() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let mut local_authority = authority(1)?;
    local_authority.effect = RecoveryEffect::LocalReversible;
    o.update_authority(local_authority, t(100)?)?;
    o.register_worker(worker(21)?, t(100)?)?;

    let mut local = action(30)?;
    local.effect = RecoveryEffect::LocalReversible;
    local
        .operation
        .binding
        .as_mut()
        .ok_or("binding")?
        .effect_class = intent_contracts::CapabilityEffectClass::LocalReversible;
    o.prepare_action(local)?;
    o.approve_action(id(30)?, 0, t(800_000)?, t(100)?)?;
    let outbox = o.enqueue_action(id(30)?, 1, t(100)?)?;
    let lease = o.claim_dispatch(outbox, id(21)?, t(700_000)?, t(100)?)?;
    let sent = o.begin_authorized_dispatch(lease, t(100)?)?;
    let receipt = digest(b"local receipt");
    let first = attestation(
        &sent,
        ReconciliationVerdict::LocalCommitted {
            before_revision: digest(b"source-v1"),
            after_revision: digest(b"after-one"),
            revision: 1,
            receipt,
        },
    )?;
    o.reconcile(signed(&first)?, 3, t(100)?)?;
    assert!(matches!(
        projected(&o, 30)?.evidence.ok_or("evidence")?.outcome,
        ExecutionOutcome::LocalCommitted { revision: 1, .. }
    ));

    o.store.connection.execute_batch(
        "DROP TRIGGER recovery_evidence_immutable; DROP TRIGGER recovery_evidence_no_delete;",
    )?;
    let second = attestation(
        &sent,
        ReconciliationVerdict::LocalCommitted {
            before_revision: digest(b"source-v1"),
            after_revision: digest(b"after-two"),
            revision: 2,
            receipt,
        },
    )?;
    let bytes = serde_json::to_vec(&second)?;
    let mut mac = Hmac::<Sha256>::new_from_slice(&KEY)?;
    mac.update(b"intent.read-only-reconciliation.v1\0");
    mac.update(&bytes);
    let tag: [u8; 32] = mac.finalize().into_bytes().into();
    o.store.connection.execute(
        "INSERT INTO recovery_evidence SELECT ?1,operation_id,attempt_id,verdict,receipt,?2,?3,key_id,?4,?5 FROM recovery_evidence LIMIT 1",
        params![
            second.evidence_id.to_string(),
            &bytes,
            digest(&bytes).to_hex(),
            &tag[..],
            101_i64
        ],
    )?;
    assert!(o.project_operation(id(30)?).is_err());
    Ok(())
}

#[test]
fn reconciliation_rejects_same_receipt_with_different_local_commit_metadata() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let mut local_authority = authority(1)?;
    local_authority.effect = RecoveryEffect::LocalReversible;
    o.update_authority(local_authority, t(100)?)?;
    o.register_worker(worker(21)?, t(100)?)?;

    let mut local = action(30)?;
    local.effect = RecoveryEffect::LocalReversible;
    local
        .operation
        .binding
        .as_mut()
        .ok_or("binding")?
        .effect_class = intent_contracts::CapabilityEffectClass::LocalReversible;
    o.prepare_action(local)?;
    o.approve_action(id(30)?, 0, t(800_000)?, t(100)?)?;
    let outbox = o.enqueue_action(id(30)?, 1, t(100)?)?;
    let lease = o.claim_dispatch(outbox, id(21)?, t(700_000)?, t(100)?)?;
    let sent = o.begin_authorized_dispatch(lease, t(100)?)?;
    let receipt = digest(b"local receipt");
    let first = attestation(
        &sent,
        ReconciliationVerdict::LocalCommitted {
            before_revision: digest(b"source-v1"),
            after_revision: digest(b"after-one"),
            revision: 1,
            receipt,
        },
    )?;
    o.reconcile(signed(&first)?, 3, t(100)?)?;
    let before = projected(&o, 30)?;
    let count_before: i64 = o.store.connection.query_row(
        "SELECT COUNT(*) FROM recovery_evidence WHERE operation_id=?1",
        [id::<OperationId>(30)?.to_string()],
        |r| r.get(0),
    )?;

    let second = attestation(
        &sent,
        ReconciliationVerdict::LocalCommitted {
            before_revision: digest(b"source-v1"),
            after_revision: digest(b"after-two"),
            revision: 2,
            receipt,
        },
    )?;
    assert!(
        o.reconcile(signed(&second)?, rev(&o, 30)?, t(101)?)
            .is_err()
    );
    let count_after: i64 = o.store.connection.query_row(
        "SELECT COUNT(*) FROM recovery_evidence WHERE operation_id=?1",
        [id::<OperationId>(30)?.to_string()],
        |r| r.get(0),
    )?;
    assert_eq!(count_after, count_before);
    assert_eq!(projected(&o, 30)?, before);
    Ok(())
}

#[test]
fn compensated_original_rejects_later_decisive_duplicate() -> Result {
    let p = Profile::new()?;
    let mut o = owner(&p)?;
    let original = started(&mut o, 30)?;
    let receipt = digest(b"original receipt");
    let first = attestation(&original, ReconciliationVerdict::Committed { receipt })?;
    o.reconcile(signed(&first)?, 3, t(100)?)?;

    let mut compensation = action(31)?;
    compensation.compensation = Some(CompensationOrigin {
        operation_id: id(30)?,
        attempt_id: original.attempt_id(),
        receipt,
    });
    o.prepare_action(compensation)?;
    o.approve_action(id(31)?, 0, t(1000)?, t(100)?)?;
    let outbox = o.enqueue_action(id(31)?, 1, t(100)?)?;
    let lease = o.claim_dispatch(outbox, id(20)?, t(200)?, t(100)?)?;
    let compensation_attempt = o.begin_authorized_dispatch(lease, t(100)?)?;
    let compensation_evidence = attestation(
        &compensation_attempt,
        ReconciliationVerdict::Committed {
            receipt: digest(b"compensation receipt"),
        },
    )?;
    o.reconcile(signed(&compensation_evidence)?, rev(&o, 31)?, t(150)?)?;
    let before = projected(&o, 30)?;
    assert_eq!(before.stage, ExecutionStage::Compensated);
    let count_before: i64 = o.store.connection.query_row(
        "SELECT COUNT(*) FROM recovery_evidence WHERE operation_id=?1",
        [id::<OperationId>(30)?.to_string()],
        |r| r.get(0),
    )?;

    let late = attestation(&original, ReconciliationVerdict::Committed { receipt })?;
    assert!(o.reconcile(signed(&late)?, rev(&o, 30)?, t(151)?).is_err());
    let count_after: i64 = o.store.connection.query_row(
        "SELECT COUNT(*) FROM recovery_evidence WHERE operation_id=?1",
        [id::<OperationId>(30)?.to_string()],
        |r| r.get(0),
    )?;
    assert_eq!(count_after, count_before);
    assert_eq!(projected(&o, 30)?, before);
    Ok(())
}
