use super::dispatch::parse;
use super::*;
use crate::recovery_error::bounded_json;
use intent_recovery::{
    AttemptPhase, BoundOutcome, CurrentAuthority, RecordedOutcome, RecoveryEffect, RecoveryFacts,
    RecoveryStage, SourcePrecondition, classify_recovery,
};

impl RuntimeOwner {
    /// Plans a bounded batch after the epoch fence was committed by open_profile.
    /// An unfinished or stale plan keeps activation blocked; planning never executes an effect.
    pub fn plan_startup(
        &mut self,
        now: UnixTimestampMicros,
        limit: usize,
    ) -> Result<StartupBatch, RecoveryError> {
        if !(1..=128).contains(&limit) {
            return Err(RecoveryError::Limit(
                "startup batch must contain 1..=128 operations",
            ));
        }
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, false)?;
        observe_clock(&tx, now)?;
        let enabled: bool = tx.query_row(
            "SELECT dispatch_enabled FROM runtime_control WHERE singleton=1",
            [],
            |r| r.get(0),
        )?;
        if enabled {
            return Err(RecoveryError::Denied(
                "startup planning cannot reclassify a running incarnation",
            ));
        }
        let ids = {
            let mut statement=tx.prepare("SELECT o.operation_id FROM durable_operations o WHERE o.state NOT IN ('verified','compensated','cancelled') AND NOT EXISTS(SELECT 1 FROM recovery_plans p WHERE p.runtime_epoch=?1 AND p.operation_id=o.operation_id AND p.operation_revision=o.revision) ORDER BY o.operation_id LIMIT ?2")?;
            statement
                .query_map(params![self.epoch.to_string(), limit as i64], |r| {
                    r.get::<_, String>(0)
                })?
                .collect::<Result<Vec<_>, _>>()?
        };
        let mut plans = Vec::with_capacity(ids.len());
        for id in ids {
            let id = parse(&id)?;
            let mut op = load(&tx, id)?;
            if matches!(
                op.state(),
                DurableOperationState::Attempting
                    | DurableOperationState::Accepted
                    | DurableOperationState::Compensating
            ) {
                // Never rewind the attempt to pending. A remote system may have accepted it.
                transition(
                    &tx,
                    &op,
                    DurableOperationState::NeedsReconciliation,
                    op.attempt_identity(),
                    now,
                    "startup found an unverified prior-incarnation effect",
                )?;
                tx.execute("UPDATE outbox_messages SET state='failed',lease_owner=NULL,lease_expires_at_micros=NULL,failure_detail='startup reconciliation required',updated_at_micros=?2 WHERE operation_id=?1 AND state IN ('attempting','leased')",params![id.to_string(),now.get()])?;
                op = load(&tx, id)?;
            }
            let plan = build_plan(&tx, &op, self.epoch, now)?;
            let bytes = bounded_json(&plan, 8192)?;
            tx.execute(
                "INSERT INTO recovery_plans VALUES(?1,?2,?3,?4,?5,?6)",
                params![
                    self.epoch.to_string(),
                    id.to_string(),
                    sql_u64(op.revision())?,
                    now.get(),
                    &bytes,
                    digest(&bytes).to_hex()
                ],
            )?;
            plans.push(plan);
        }
        let remaining = unplanned(&tx, self.epoch)?;
        tx.commit()?;
        Ok(StartupBatch { plans, remaining })
    }
    /// Opens the runtime admission gate, not any action's authority. Every dispatch
    /// separately requires a fresh explicit approval, source revision and worker lease.
    pub fn activate_after_planning(
        &mut self,
        now: UnixTimestampMicros,
    ) -> Result<(), RecoveryError> {
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, false)?;
        observe_clock(&tx, now)?;
        if unplanned(&tx, self.epoch)? {
            return Err(RecoveryError::Denied(
                "unfinished or stale startup recovery plans",
            ));
        }
        tx.execute("UPDATE runtime_control SET dispatch_enabled=1,reason='startup planned; per-action authority still required' WHERE singleton=1 AND epoch=?1",[self.epoch.to_string()])?;
        tx.commit()?;
        Ok(())
    }
    pub fn recovery_view(
        &self,
        id: OperationId,
        now: UnixTimestampMicros,
    ) -> Result<RecoveryTaskView, RecoveryError> {
        let tx = self.store.connection.unchecked_transaction()?;
        current_epoch(&tx, self.epoch, false)?;
        let op = load(&tx, id)?;
        let plan = build_plan(&tx, &op, self.epoch, now)?;
        let mut actions = vec![RecoveryUiAction::Inspect];
        match op.state() {
            DurableOperationState::Prepared
            | DurableOperationState::Approved
            | DurableOperationState::DispatchPending => {
                actions.push(RecoveryUiAction::CancelUnsent);
                if managed(&tx, id)? {
                    actions.push(RecoveryUiAction::ReviewFreshApproval);
                }
            }
            DurableOperationState::Attempting
            | DurableOperationState::Accepted
            | DurableOperationState::NeedsReconciliation
            | DurableOperationState::Compensating => {
                actions.push(RecoveryUiAction::RequestReadOnlyReconciliation)
            }
            DurableOperationState::Failed
                if plan.decision.disposition
                    == intent_recovery::RecoveryDisposition::FreshApprovalAfterProvenNonCommit =>
            {
                actions.push(RecoveryUiAction::ReviewFreshApproval)
            }
            _ => {}
        }
        if plan.decision.disposition
            == intent_recovery::RecoveryDisposition::RestoreHistoryUncertain
        {
            actions.retain(|a| *a != RecoveryUiAction::ReviewFreshApproval);
        }
        tx.commit()?;
        Ok(RecoveryTaskView { plan, actions })
    }
    /// Executes only local control intent; no Retry action exists for uncertain effects.
    pub fn apply_recovery_control(
        &mut self,
        view: &RecoveryTaskView,
        action: RecoveryUiAction,
        now: UnixTimestampMicros,
    ) -> Result<u64, RecoveryError> {
        if view.plan.runtime_epoch != self.epoch {
            return Err(RecoveryError::Denied("view belongs to an old runtime"));
        }
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, false)?;
        observe_clock(&tx, now)?;
        let op = load(&tx, view.plan.operation_id)?;
        if op.revision() != view.plan.operation_revision {
            return Err(RecoveryError::Conflict("recovery view is stale"));
        }
        let next = match action {
            RecoveryUiAction::Inspect => {
                tx.commit()?;
                return Ok(op.revision());
            }
            RecoveryUiAction::ReviewFreshApproval => {
                return Err(RecoveryError::Denied(
                    "approval requires the exact-action approval API",
                ));
            }
            RecoveryUiAction::CancelUnsent => {
                if !matches!(
                    op.state(),
                    DurableOperationState::Prepared
                        | DurableOperationState::Approved
                        | DurableOperationState::DispatchPending
                ) {
                    return Err(RecoveryError::Denied(
                        "local stop cannot erase an uncertain external effect",
                    ));
                }
                let started:bool=tx.query_row("SELECT EXISTS(SELECT 1 FROM recovery_attempts WHERE operation_id=?1 AND started_at_micros IS NOT NULL AND attempt_id=?2)",params![op.operation_id().to_string(),op.attempt_identity().map(|v|v.to_string())],|r|r.get(0))?;
                if started {
                    return Err(RecoveryError::Denied("attempt already started"));
                }
                tx.execute(
                    "DELETE FROM recovery_approval_heads WHERE operation_id=?1",
                    [op.operation_id().to_string()],
                )?;
                tx.execute("DELETE FROM recovery_claims WHERE outbox_id IN (SELECT outbox_id FROM outbox_messages WHERE operation_id=?1)",[op.operation_id().to_string()])?;
                tx.execute("UPDATE outbox_messages SET state='failed',lease_owner=NULL,lease_expires_at_micros=NULL,failure_detail='cancelled before dispatch',updated_at_micros=?2 WHERE operation_id=?1 AND state IN ('pending','leased')",params![op.operation_id().to_string(),now.get()])?;
                DurableOperationState::Cancelled
            }
            RecoveryUiAction::RequestReadOnlyReconciliation => {
                if !matches!(
                    op.state(),
                    DurableOperationState::Attempting
                        | DurableOperationState::Accepted
                        | DurableOperationState::NeedsReconciliation
                        | DurableOperationState::Compensating
                ) {
                    return Err(RecoveryError::Denied("operation has no uncertain attempt"));
                }
                DurableOperationState::NeedsReconciliation
            }
        };
        let rev = transition(
            &tx,
            &op,
            next,
            op.attempt_identity(),
            now,
            "current-state-checked local recovery control",
        )?;
        tx.commit()?;
        Ok(rev)
    }
}
fn unplanned(db: &Connection, epoch: Uuid) -> Result<bool, RecoveryError> {
    Ok(db.query_row("SELECT EXISTS(SELECT 1 FROM durable_operations o WHERE o.state NOT IN ('verified','compensated','cancelled') AND NOT EXISTS(SELECT 1 FROM recovery_plans p WHERE p.runtime_epoch=?1 AND p.operation_id=o.operation_id AND p.operation_revision=o.revision))",[epoch.to_string()],|r|r.get(0))?)
}
fn build_plan(
    db: &Connection,
    op: &DurableOperation,
    epoch: Uuid,
    now: UnixTimestampMicros,
) -> Result<DurableRecoveryPlan, RecoveryError> {
    type Policy = (String, String, i64, Option<String>, Option<String>);
    let policy:Option<Policy>=db.query_row("SELECT effect,source_revision,deadline_micros,original_attempt_id,original_receipt FROM recovery_actions WHERE operation_id=?1",[op.operation_id().to_string()],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?))).optional()?;
    let mut effect = RecoveryEffect::Unknown;
    let mut phase = AttemptPhase::Original;
    let mut deadline = None;
    let mut authority = CurrentAuthority::Unknown;
    let mut source = SourcePrecondition::Unknown;
    if let Some((raw, expected, expires, origin, receipt)) = policy {
        effect = match raw.as_str() {
            "read_only" => RecoveryEffect::ReadOnly,
            "local_reversible" => RecoveryEffect::LocalReversible,
            "external_write" => RecoveryEffect::ExternalWrite,
            "unknown" => RecoveryEffect::Unknown,
            _ => return Err(RecoveryError::Integrity("recovery effect")),
        };
        deadline = Some(
            UnixTimestampMicros::try_new(expires)
                .map_err(|_| RecoveryError::Integrity("action deadline"))?,
        );
        if let (Some(original), Some(receipt), Some(_)) = (origin, receipt, op.attempt_identity())
            && matches!(
                op.state(),
                DurableOperationState::Attempting
                    | DurableOperationState::Accepted
                    | DurableOperationState::NeedsReconciliation
                    | DurableOperationState::Verified
                    | DurableOperationState::Failed
                    | DurableOperationState::Compensated
            )
        {
            phase = AttemptPhase::Compensation {
                original_attempt: parse(&original)?,
                original_receipt: intent_contracts::ContentHash::from_hex(&receipt)
                    .map_err(|_| RecoveryError::Integrity("compensation receipt"))?,
            };
        }
        let current:Option<(bool,String,i64)>=db.query_row("SELECT enabled,source_revision,valid_until_micros FROM recovery_authorities WHERE account_id=?1 AND capability_id=?2",params![op.account_id().to_string(),op.capability_id().to_string()],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?))).optional()?;
        if let Some((enabled, revision, valid_until)) = current {
            authority = if !enabled {
                CurrentAuthority::Revoked
            } else if valid_until <= now.get() {
                CurrentAuthority::Expired
            } else {
                CurrentAuthority::Current
            };
            source = if revision != expected {
                SourcePrecondition::Conflict
            } else if valid_until <= now.get() {
                SourcePrecondition::Stale
            } else {
                SourcePrecondition::Fresh
            };
        }
    }
    let evidence:Option<(Vec<u8>,String)>=db.query_row("SELECT payload,payload_hash FROM recovery_evidence WHERE operation_id=?1 AND attempt_id=?2 AND length(payload)<=8192 ORDER BY CASE verdict WHEN 'inconclusive' THEN 1 ELSE 0 END,recorded_at_micros DESC,evidence_id LIMIT 1",params![op.operation_id().to_string(),op.attempt_identity().map(|a|a.to_string())],|r|Ok((r.get(0)?,r.get(1)?))).optional()?;
    let outcome = if let Some((bytes, hash)) = evidence {
        if digest(&bytes).to_hex() != hash {
            return Err(RecoveryError::Integrity("stored reconciliation evidence"));
        }
        let attestation: ReadOnlyAttestation = serde_json::from_slice(&bytes)?;
        let verdict = match attestation.verdict {
            ReconciliationVerdict::ReadCompleted { capture } => {
                RecordedOutcome::ReadCompleted { capture }
            }
            ReconciliationVerdict::LocalCommitted {
                revision, receipt, ..
            } => RecordedOutcome::LocalCommitted { revision, receipt },
            ReconciliationVerdict::Committed { receipt } => {
                RecordedOutcome::ExternalCommitted { receipt }
            }
            ReconciliationVerdict::AuthoritativeNonCommit { .. } => {
                RecordedOutcome::ProvenNotCommitted
            }
            ReconciliationVerdict::Inconclusive => RecordedOutcome::Inconclusive,
        };
        Some(BoundOutcome {
            binding: attestation.binding,
            outcome: verdict,
        })
    } else {
        None
    };
    let compensation_operation: Option<String> = db.query_row("SELECT c.operation_id FROM recovery_actions c JOIN durable_operations o ON o.operation_id=c.operation_id WHERE c.original_operation_id=?1 AND o.state='verified'",[op.operation_id().to_string()],|r|r.get(0)).optional()?;
    if op.state() == DurableOperationState::Compensated && compensation_operation.is_none() {
        return Err(RecoveryError::Integrity(
            "compensated operation lacks a verified compensation lineage",
        ));
    }
    // The original receipt remains attached to its original attempt. The separate
    // compensation operation owns its own phase, attempt and receipt.
    let stage: RecoveryStage = if op.state() == DurableOperationState::Compensated {
        RecoveryStage::Verified
    } else {
        serde_json::from_str(&format!("\"{}\"", op.state().as_str()))?
    };
    let mut decision = classify_recovery(
        &RecoveryFacts {
            policy_version: 1,
            operation_id: op.operation_id(),
            account_id: op.account_id(),
            capability_id: op.capability_id(),
            arguments_hash: op.arguments_hash(),
            effect,
            stage,
            attempt_id: op.attempt_identity(),
            phase,
            authority,
            source,
            deadline,
            outcome,
            idempotency: None,
        },
        now,
    );
    let restored: bool = db.query_row(
        "SELECT required FROM recovery_restore_fence WHERE singleton=1",
        [],
        |r| r.get(0),
    )?;
    if restored
        && effect != RecoveryEffect::ReadOnly
        && !matches!(
            decision.disposition,
            intent_recovery::RecoveryDisposition::NoReplayKnownCommit
                | intent_recovery::RecoveryDisposition::NoReplayCapturedRead
                | intent_recovery::RecoveryDisposition::NoReplayCompensationConfirmed
                | intent_recovery::RecoveryDisposition::NoReplayLocalCommit
                | intent_recovery::RecoveryDisposition::TerminalWithoutReplay
        )
    {
        decision.disposition = intent_recovery::RecoveryDisposition::RestoreHistoryUncertain;
        decision.required_evidence =
            intent_recovery::RequiredEvidence::IndependentRollbackDomainHistory;
    }
    Ok(DurableRecoveryPlan {
        schema_version: 1,
        runtime_epoch: epoch,
        operation_id: op.operation_id(),
        operation_revision: op.revision(),
        state: op.state(),
        compensation_operation: compensation_operation.map(|v| parse(&v)).transpose()?,
        decision,
        created_at: now,
    })
}
