use intent_contracts::*;
use serde_json::{Value, json};
use uuid::Uuid;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn data() -> ExecutionObservationData {
    ExecutionObservationData {
        task_id: TaskId::from_uuid(Uuid::from_u128(1)),
        capability_id: CapabilityId::from_uuid(Uuid::from_u128(2)),
        arguments_hash: ContentHash::from_bytes([3; 32]),
        revision: 0,
        stage: ExecutionStage::Prepared,
        attempt: None,
        phase: ExecutionPhase::Original {},
        evidence: None,
        compensation: None,
        detail: None,
    }
}

fn record(data: ExecutionObservationData) -> Result<Operation> {
    Ok(Operation::from_execution(
        OperationId::from_uuid(Uuid::from_u128(4)),
        ActionProposalId::from_uuid(Uuid::from_u128(5)),
        AccountId::from_uuid(Uuid::from_u128(6)),
        ExecutionObservation::try_from(data)?,
    )?)
}

fn sent() -> Result<ExecutionObservationData> {
    let mut data = data();
    data.stage = ExecutionStage::Accepted;
    data.attempt = Some(ExecutionAttempt {
        id: OperationAttemptId::from_uuid(Uuid::from_u128(7)),
        started_at: Some(UnixTimestampMicros::try_new(100)?),
    });
    Ok(data)
}

fn evidence(outcome: ExecutionOutcome) -> Result<ExecutionEvidence> {
    Ok(ExecutionEvidence {
        evidence_id: EvidenceId::from_uuid(Uuid::from_u128(8)),
        attempt_id: OperationAttemptId::from_uuid(Uuid::from_u128(7)),
        payload_hash: ContentHash::from_bytes([9; 32]),
        observed_at: UnixTimestampMicros::try_new(110)?,
        recorded_at: UnixTimestampMicros::try_new(120)?,
        outcome,
    })
}

#[test]
fn execution_roundtrips_without_fabricating_a_key_or_provider_receipt() -> Result {
    let record = record(sent()?)?;
    let encoded = serde_json::to_value(&record)?;
    assert_eq!(encoded["state"]["state"], "execution");
    assert!(encoded.get("idempotency_key").is_none());
    assert!(!serde_json::to_string(&encoded)?.contains("provider_receipt_id"));
    assert_eq!(serde_json::from_value::<Operation>(encoded)?, record);
    assert!(record.idempotency_key().is_none());
    let mut invented = serde_json::to_value(&record)?;
    invented["idempotency_key"] = json!("invented");
    assert!(serde_json::from_value::<Operation>(invented).is_err());
    assert!(
        Operation::new(
            record.id(),
            ActionProposalId::from_uuid(Uuid::from_u128(5)),
            AccountId::from_uuid(Uuid::from_u128(6)),
            BoundedText::try_new("invented")?,
            record.state().clone(),
        )
        .is_err()
    );
    Ok(())
}

#[test]
fn accepted_verified_and_inconclusive_cannot_be_relabelled() -> Result {
    let accepted = serde_json::to_value(record(sent()?)?)?;
    let mut invalid = accepted.clone();
    invalid["state"]["details"]["observation"]["stage"] = json!("verified");
    assert!(serde_json::from_value::<Operation>(invalid).is_err());
    let mut data = sent()?;
    data.evidence = Some(evidence(ExecutionOutcome::Inconclusive {})?);
    assert!(record(data.clone()).is_err());
    data.stage = ExecutionStage::NeedsReconciliation;
    let unresolved = record(data.clone())?;
    assert_eq!(
        serde_json::from_str::<Operation>(&serde_json::to_string(&unresolved)?)?,
        unresolved
    );
    data.stage = ExecutionStage::Verified;
    assert!(record(data.clone()).is_err());
    data.evidence = Some(evidence(ExecutionOutcome::ExternalCommitted {
        receipt: ContentHash::from_bytes([10; 32]),
    })?);
    let verified = record(data)?;
    let mut encoded = serde_json::to_value(&verified)?;
    encoded["last_reconciled_at"] = Value::Null;
    assert!(serde_json::from_value::<Operation>(encoded).is_err());
    assert_eq!(
        serde_json::from_str::<Operation>(&serde_json::to_string(&verified)?)?,
        verified
    );
    Ok(())
}

#[test]
fn execution_import_rejects_unknown_fields_and_broken_attempt_bindings() -> Result {
    let encoded = serde_json::to_value(record(sent()?)?)?;
    for (key, value) in [
        ("stage", json!("settled")),
        ("provider_receipt_id", json!("invented")),
        ("attempt", Value::Null),
    ] {
        let mut mutated = encoded.clone();
        mutated["state"]["details"]["observation"][key] = value;
        assert!(serde_json::from_value::<Operation>(mutated).is_err());
    }
    let mut data = sent()?;
    data.stage = ExecutionStage::Failed;
    let mut wrong = evidence(ExecutionOutcome::ProvenNotCommitted {
        observation: ContentHash::from_bytes([11; 32]),
    })?;
    wrong.attempt_id = OperationAttemptId::from_uuid(Uuid::from_u128(12));
    data.evidence = Some(wrong);
    assert!(record(data).is_err());
    let mut self_origin = sent()?;
    self_origin.phase = ExecutionPhase::Compensation {
        original_operation_id: OperationId::from_uuid(Uuid::from_u128(4)),
        original_attempt_id: OperationAttemptId::from_uuid(Uuid::from_u128(13)),
        original_receipt: ContentHash::from_bytes([14; 32]),
    };
    assert!(record(self_origin).is_err());
    Ok(())
}

#[test]
fn compensated_observation_must_be_for_the_original_operation() -> Result {
    let mut data = sent()?;
    data.stage = ExecutionStage::Compensated;
    data.phase = ExecutionPhase::Compensation {
        original_operation_id: OperationId::from_uuid(Uuid::from_u128(20)),
        original_attempt_id: OperationAttemptId::from_uuid(Uuid::from_u128(21)),
        original_receipt: ContentHash::from_bytes([22; 32]),
    };
    data.evidence = Some(evidence(ExecutionOutcome::ExternalCommitted {
        receipt: ContentHash::from_bytes([23; 32]),
    })?);
    let compensation_attempt = ExecutionAttempt {
        id: OperationAttemptId::from_uuid(Uuid::from_u128(24)),
        started_at: Some(UnixTimestampMicros::try_new(130)?),
    };
    data.compensation = Some(VerifiedCompensation {
        operation_id: OperationId::from_uuid(Uuid::from_u128(25)),
        attempt: compensation_attempt,
        evidence: ExecutionEvidence {
            evidence_id: EvidenceId::from_uuid(Uuid::from_u128(26)),
            attempt_id: compensation_attempt.id,
            payload_hash: ContentHash::from_bytes([27; 32]),
            observed_at: UnixTimestampMicros::try_new(140)?,
            recorded_at: UnixTimestampMicros::try_new(150)?,
            outcome: ExecutionOutcome::ExternalCommitted {
                receipt: ContentHash::from_bytes([28; 32]),
            },
        },
    });
    assert!(record(data).is_err());
    Ok(())
}

#[test]
fn compensation_cannot_start_before_original_commit_evidence_is_recorded() -> Result {
    let mut data = sent()?;
    data.stage = ExecutionStage::Compensated;
    data.evidence = Some(evidence(ExecutionOutcome::ExternalCommitted {
        receipt: ContentHash::from_bytes([23; 32]),
    })?);
    let compensation_attempt = ExecutionAttempt {
        id: OperationAttemptId::from_uuid(Uuid::from_u128(24)),
        started_at: Some(UnixTimestampMicros::try_new(119)?),
    };
    data.compensation = Some(VerifiedCompensation {
        operation_id: OperationId::from_uuid(Uuid::from_u128(25)),
        attempt: compensation_attempt,
        evidence: ExecutionEvidence {
            evidence_id: EvidenceId::from_uuid(Uuid::from_u128(26)),
            attempt_id: compensation_attempt.id,
            payload_hash: ContentHash::from_bytes([27; 32]),
            observed_at: UnixTimestampMicros::try_new(121)?,
            recorded_at: UnixTimestampMicros::try_new(122)?,
            outcome: ExecutionOutcome::ExternalCommitted {
                receipt: ContentHash::from_bytes([28; 32]),
            },
        },
    });
    assert!(record(data.clone()).is_err());
    data.compensation
        .as_mut()
        .ok_or("compensation")?
        .attempt
        .started_at = Some(UnixTimestampMicros::try_new(120)?);
    assert!(record(data).is_ok());
    Ok(())
}

#[test]
fn legacy_operation_keeps_its_key_and_missing_key_still_fails() -> Result {
    let legacy = Operation::new(
        OperationId::from_uuid(Uuid::from_u128(4)),
        ActionProposalId::from_uuid(Uuid::from_u128(5)),
        AccountId::from_uuid(Uuid::from_u128(6)),
        BoundedText::try_new("provider-supplied-key")?,
        OperationState::Prepared,
    )?;
    let mut encoded = serde_json::to_value(&legacy)?;
    assert_eq!(
        serde_json::from_value::<Operation>(encoded.clone())?,
        legacy
    );
    assert!(legacy.execution().is_none());
    encoded
        .as_object_mut()
        .ok_or("object")?
        .remove("idempotency_key");
    assert!(serde_json::from_value::<Operation>(encoded).is_err());
    Ok(())
}
