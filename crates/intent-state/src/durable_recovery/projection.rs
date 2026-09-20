use super::dispatch::parse;
use super::*;
use intent_contracts::{
    ContentHash, EvidenceId, ExecutionAttempt, ExecutionEvidence, ExecutionObservation,
    ExecutionObservationData, ExecutionOutcome, ExecutionPhase, ExecutionStage, Operation,
    VerifiedCompensation,
};

impl RuntimeOwner {
    /// Returns recorded execution facts without creating consent or dispatch authority.
    pub fn project_operation(&self, id: OperationId) -> Result<Operation, RecoveryError> {
        let tx = self.store.connection.unchecked_transaction()?;
        current_epoch(&tx, self.epoch, false)?;
        let op = load(&tx, id)?;
        let mut facts = load_facts(&tx, &op)?;
        if op.state() == DurableOperationState::Compensated {
            facts.compensation = Some(load_compensation(&tx, &op, &facts)?);
        }
        let observation =
            ExecutionObservation::try_from(facts).map_err(RecoveryError::Integrity)?;
        let record = Operation::from_execution(
            op.operation_id(),
            op.action_proposal_id(),
            op.account_id(),
            observation,
        )
        .map_err(RecoveryError::Integrity)?;
        tx.commit()?;
        Ok(record)
    }
}

fn stage(state: DurableOperationState) -> Result<ExecutionStage, RecoveryError> {
    use DurableOperationState as D;
    Ok(match state {
        D::Prepared => ExecutionStage::Prepared,
        D::Approved => ExecutionStage::Approved,
        D::DispatchPending => ExecutionStage::DispatchPending,
        D::Attempting => ExecutionStage::Attempting,
        D::Accepted => ExecutionStage::Accepted,
        D::Verified => ExecutionStage::Verified,
        D::Failed => ExecutionStage::Failed,
        D::NeedsReconciliation => ExecutionStage::NeedsReconciliation,
        D::Cancelled => ExecutionStage::Cancelled,
        D::Compensated => ExecutionStage::Compensated,
        D::Compensating => {
            return Err(RecoveryError::Denied(
                "legacy combined compensation has no separate managed lineage",
            ));
        }
    })
}

fn hash(value: &str) -> Result<ContentHash, RecoveryError> {
    ContentHash::from_hex(value).map_err(|_| RecoveryError::Integrity("stored outcome hash"))
}

fn timestamp(value: i64) -> Result<UnixTimestampMicros, RecoveryError> {
    UnixTimestampMicros::try_new(value).map_err(|_| RecoveryError::Integrity("stored outcome time"))
}

type EvidenceRow = (
    String,
    String,
    Option<String>,
    Option<Vec<u8>>,
    String,
    i64,
    String,
    String,
);

fn validate_action_binding(
    op: &DurableOperation,
    effect: &str,
    source: &str,
) -> Result<(), RecoveryError> {
    let binding = op.binding().ok_or(RecoveryError::Denied(
        "legacy unbound execution observation",
    ))?;
    let expected_effect = match binding.effect_class {
        intent_contracts::CapabilityEffectClass::ReadOnly => "read_only",
        intent_contracts::CapabilityEffectClass::LocalReversible => "local_reversible",
        intent_contracts::CapabilityEffectClass::ExternalCompensatable
        | intent_contracts::CapabilityEffectClass::IrreversibleOrUncertain => "external_write",
    };
    if effect != expected_effect {
        return Err(RecoveryError::Integrity(
            "execution effect differs from action binding",
        ));
    }
    if binding.context.source_revision != hash(source)? {
        return Err(RecoveryError::Integrity(
            "execution source differs from action binding",
        ));
    }
    Ok(())
}

fn load_facts(
    db: &Connection,
    op: &DurableOperation,
) -> Result<ExecutionObservationData, RecoveryError> {
    type Policy = (
        String,
        String,
        Option<String>,
        Option<String>,
        Option<String>,
    );
    let policy: Policy = db.query_row(
        "SELECT effect,source_revision,original_operation_id,original_attempt_id,original_receipt FROM recovery_actions WHERE operation_id=?1",
        [op.operation_id().to_string()], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?)),
    ).optional()?.ok_or(RecoveryError::Denied("unmanaged operation has no execution observation"))?;
    let (effect, source, origin, origin_attempt, origin_receipt) = policy;
    validate_action_binding(op, &effect, &source)?;
    let phase = match (origin, origin_attempt, origin_receipt) {
        (None, None, None) => ExecutionPhase::Original {},
        (Some(operation), Some(attempt), Some(receipt)) => ExecutionPhase::Compensation {
            original_operation_id: parse(&operation)?,
            original_attempt_id: parse(&attempt)?,
            original_receipt: hash(&receipt)?,
        },
        _ => return Err(RecoveryError::Integrity("incomplete compensation origin")),
    };
    validate_origin(db, op, phase)?;
    let attempt = if let Some(id) = op.attempt_identity() {
        let started: Option<i64> = db.query_row(
            "SELECT started_at_micros FROM recovery_attempts WHERE attempt_id=?1 AND operation_id=?2",
            params![id.to_string(),op.operation_id().to_string()], |r| r.get(0),
        ).optional()?.ok_or(RecoveryError::Integrity("missing exact execution attempt"))?;
        Some(ExecutionAttempt {
            id,
            started_at: started.map(timestamp).transpose()?,
        })
    } else {
        None
    };
    let evidence = attempt
        .map(|a| load_evidence(db, op, a, &effect, &source))
        .transpose()?
        .flatten();
    Ok(ExecutionObservationData {
        task_id: op.task_id(),
        capability_id: op.capability_id(),
        arguments_hash: op.arguments_hash(),
        revision: op.revision(),
        stage: stage(op.state())?,
        attempt,
        phase,
        evidence,
        compensation: None,
        detail: op.state_detail().cloned(),
    })
}

fn validate_origin(
    db: &Connection,
    op: &DurableOperation,
    phase: ExecutionPhase,
) -> Result<(), RecoveryError> {
    let ExecutionPhase::Compensation {
        original_operation_id,
        original_attempt_id,
        original_receipt,
    } = phase
    else {
        return Ok(());
    };
    let original = load(db, original_operation_id)?;
    if original.operation_id() == op.operation_id()
        || original.account_id() != op.account_id()
        || original.attempt_identity() != Some(original_attempt_id)
        || !matches!(
            original.state(),
            DurableOperationState::Verified | DurableOperationState::Compensated
        )
    {
        return Err(RecoveryError::Integrity(
            "compensation original identity or state",
        ));
    }
    let (effect, source, started, same_scope): (String, String, Option<i64>, bool) = db.query_row(
        "SELECT a.effect,a.source_revision,t.started_at_micros,a.privacy_scope=c.privacy_scope FROM recovery_actions a JOIN recovery_attempts t ON t.operation_id=a.operation_id JOIN recovery_actions c ON c.operation_id=?3 WHERE a.operation_id=?1 AND t.attempt_id=?2",
        params![original_operation_id.to_string(),original_attempt_id.to_string(),op.operation_id().to_string()],
        |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?)),
    ).optional()?.ok_or(RecoveryError::Integrity("missing compensation original action"))?;
    if !same_scope {
        return Err(RecoveryError::Integrity(
            "compensation original privacy scope",
        ));
    }
    let attempt = ExecutionAttempt {
        id: original_attempt_id,
        started_at: started.map(timestamp).transpose()?,
    };
    validate_action_binding(&original, &effect, &source)?;
    let evidence = load_evidence(db, &original, attempt, &effect, &source)?.ok_or(
        RecoveryError::Integrity("compensation original evidence is missing"),
    )?;
    if evidence.outcome.committed_reference() != Some(original_receipt) {
        return Err(RecoveryError::Integrity(
            "compensation original receipt differs from evidence",
        ));
    }
    Ok(())
}

fn decode_evidence_row(
    op: &DurableOperation,
    attempt: ExecutionAttempt,
    effect: &str,
    source: &str,
    row: EvidenceRow,
) -> Result<ExecutionEvidence, RecoveryError> {
    let (id, verdict, receipt, payload, payload_hash, recorded, key, attempt_key) = row;
    let payload = payload.ok_or(RecoveryError::Limit("stored outcome envelope"))?;
    if digest(&payload) != hash(&payload_hash)? || key != attempt_key {
        return Err(RecoveryError::Integrity(
            "stored evidence hash or trust-key binding",
        ));
    }
    let value: ReadOnlyAttestation = serde_json::from_slice(&payload)?;
    let recorded_at = timestamp(recorded)?;
    if value.schema_version != 1
        || value.evidence_id.is_nil()
        || value.evidence_id.to_string() != id
        || value.binding.operation_id != op.operation_id()
        || value.binding.account_id != op.account_id()
        || value.binding.capability_id != op.capability_id()
        || value.binding.arguments_hash != op.arguments_hash()
        || value.binding.attempt_id != attempt.id
        || !attempt.started_at.is_some_and(|at| at <= value.observed_at)
        || value.observed_at > recorded_at
        || value.valid_until <= recorded_at
        || serde_json::to_vec(&value)? != payload
    {
        return Err(RecoveryError::Integrity(
            "stored evidence does not bind the recorded attempt",
        ));
    }
    let (expected_verdict, expected_receipt, outcome) = match value.verdict {
        ReconciliationVerdict::ReadCompleted { capture } if effect == "read_only" => (
            "committed",
            Some(capture),
            ExecutionOutcome::ReadCompleted { capture },
        ),
        ReconciliationVerdict::LocalCommitted {
            before_revision,
            after_revision,
            revision,
            receipt,
        } if effect == "local_reversible" && before_revision == hash(source)? => (
            "committed",
            Some(receipt),
            ExecutionOutcome::LocalCommitted {
                before_revision,
                after_revision,
                revision,
                receipt,
            },
        ),
        ReconciliationVerdict::Committed { receipt } if effect == "external_write" => (
            "committed",
            Some(receipt),
            ExecutionOutcome::ExternalCommitted { receipt },
        ),
        ReconciliationVerdict::AuthoritativeNonCommit { observation } => (
            "not_committed",
            Some(observation),
            ExecutionOutcome::ProvenNotCommitted { observation },
        ),
        ReconciliationVerdict::Inconclusive => {
            ("inconclusive", None, ExecutionOutcome::Inconclusive {})
        }
        _ => return Err(RecoveryError::Integrity("evidence effect or source domain")),
    };
    if verdict != expected_verdict || receipt != expected_receipt.map(|h| h.to_hex()) {
        return Err(RecoveryError::Integrity(
            "stored evidence outcome columns disagree",
        ));
    }
    Ok(ExecutionEvidence {
        evidence_id: EvidenceId::from_uuid(value.evidence_id),
        attempt_id: attempt.id,
        payload_hash: hash(&payload_hash)?,
        observed_at: value.observed_at,
        recorded_at,
        outcome,
    })
}

fn load_evidence(
    db: &Connection,
    op: &DurableOperation,
    attempt: ExecutionAttempt,
    effect: &str,
    source: &str,
) -> Result<Option<ExecutionEvidence>, RecoveryError> {
    let row: Option<EvidenceRow> = db.query_row(
        "SELECT e.evidence_id,e.verdict,e.receipt,CASE WHEN length(e.payload)<=8192 THEN e.payload END,e.payload_hash,e.recorded_at_micros,e.key_id,a.evidence_key_id FROM recovery_evidence e JOIN recovery_attempts a ON a.attempt_id=e.attempt_id AND a.operation_id=e.operation_id WHERE e.operation_id=?1 AND e.attempt_id=?2 ORDER BY CASE e.verdict WHEN 'inconclusive' THEN 1 ELSE 0 END,e.recorded_at_micros DESC,e.evidence_id LIMIT 1",
        params![op.operation_id().to_string(),attempt.id.to_string()],
        |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?,r.get(5)?,r.get(6)?,r.get(7)?)),
    ).optional()?;
    let Some(row) = row else {
        return Ok(None);
    };
    let selected = decode_evidence_row(op, attempt, effect, source, row)?;
    let mut stmt = db.prepare(
        "SELECT e.evidence_id,e.verdict,e.receipt,CASE WHEN length(e.payload)<=8192 THEN e.payload END,e.payload_hash,e.recorded_at_micros,e.key_id,a.evidence_key_id FROM recovery_evidence e JOIN recovery_attempts a ON a.attempt_id=e.attempt_id AND a.operation_id=e.operation_id WHERE e.operation_id=?1 AND e.attempt_id=?2 AND e.verdict!='inconclusive' ORDER BY e.recorded_at_micros,e.evidence_id",
    )?;
    let rows = stmt.query_map(
        params![op.operation_id().to_string(), attempt.id.to_string()],
        |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?,r.get(5)?,r.get(6)?,r.get(7)?)),
    )?;
    for row in rows {
        let candidate = decode_evidence_row(op, attempt, effect, source, row?)?;
        if candidate.outcome != selected.outcome {
            return Err(RecoveryError::Integrity(
                "contradictory stored final evidence",
            ));
        }
    }
    Ok(Some(selected))
}

fn load_compensation(
    db: &Connection,
    original: &DurableOperation,
    facts: &ExecutionObservationData,
) -> Result<VerifiedCompensation, RecoveryError> {
    let attempt = facts
        .attempt
        .ok_or(RecoveryError::Integrity("compensated original attempt"))?;
    let receipt = facts
        .evidence
        .as_ref()
        .and_then(|e| e.outcome.committed_reference())
        .ok_or(RecoveryError::Integrity("compensated original evidence"))?;
    let mut stmt = db.prepare("SELECT c.operation_id FROM recovery_actions c JOIN durable_operations o ON o.operation_id=c.operation_id WHERE c.original_operation_id=?1 AND o.state='verified' LIMIT 2")?;
    let ids = stmt
        .query_map([original.operation_id().to_string()], |r| {
            r.get::<_, String>(0)
        })?
        .collect::<Result<Vec<_>, _>>()?;
    if ids.len() != 1 {
        return Err(RecoveryError::Integrity(
            "missing or ambiguous verified compensation",
        ));
    }
    let compensation = load(db, parse(&ids[0])?)?;
    if compensation.account_id() != original.account_id() {
        return Err(RecoveryError::Integrity(
            "compensation account differs from original",
        ));
    }
    let observed = ExecutionObservation::try_from(load_facts(db, &compensation)?)
        .map_err(RecoveryError::Integrity)?;
    let data = observed.data();
    if data.phase
        != (ExecutionPhase::Compensation {
            original_operation_id: original.operation_id(),
            original_attempt_id: attempt.id,
            original_receipt: receipt,
        })
    {
        return Err(RecoveryError::Integrity(
            "compensation origin no longer matches original evidence",
        ));
    }
    Ok(VerifiedCompensation {
        operation_id: compensation.operation_id(),
        attempt: data
            .attempt
            .ok_or(RecoveryError::Integrity("verified compensation attempt"))?,
        evidence: data
            .evidence
            .clone()
            .ok_or(RecoveryError::Integrity("verified compensation evidence"))?,
    })
}
