use intent_contracts::*;
use uuid::Uuid;

fn failed() -> ExecutionObservationData {
    ExecutionObservationData {
        task_id: TaskId::from_uuid(Uuid::from_u128(1)),
        capability_id: CapabilityId::from_uuid(Uuid::from_u128(2)),
        arguments_hash: ContentHash::from_bytes([3; 32]),
        revision: 1,
        stage: ExecutionStage::Failed,
        attempt: None,
        phase: ExecutionPhase::Original {},
        evidence: None,
        compensation: None,
        detail: None,
    }
}

#[test]
fn failed_execution_requires_a_started_attempt() {
    let mut data = failed();
    assert!(ExecutionObservation::try_from(data.clone()).is_err());

    data.attempt = Some(ExecutionAttempt {
        id: OperationAttemptId::from_uuid(Uuid::from_u128(4)),
        started_at: None,
    });
    assert!(ExecutionObservation::try_from(data.clone()).is_err());

    data.attempt.as_mut().expect("attempt").started_at =
        Some(UnixTimestampMicros::try_new(10).expect("timestamp"));
    assert!(ExecutionObservation::try_from(data).is_ok());
}
