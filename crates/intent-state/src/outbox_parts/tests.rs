#[cfg(test)]
mod tests {
    use super::{DispatchResult, NewOutboxMessage, OutboxError, OutboxState};
    use crate::{DurableOperationState, NewDurableOperation, OperationTransition, StateStore};
    use intent_contracts::{
        AccountId, ActionProposalId, BoundedText, CapabilityId, ContentHash, OperationAttemptId,
        OperationId, OutboxMessageId, SchemaVersion, TaskId, UnixTimestampMicros,
    };
    use rusqlite::params;
    use std::error::Error;
    use std::str::FromStr;

    fn operation_id() -> Result<OperationId, Box<dyn Error>> {
        Ok(OperationId::from_str(
            "018f47f7-5a86-7c00-8000-000000000831",
        )?)
    }

    fn attempt_id() -> Result<OperationAttemptId, Box<dyn Error>> {
        Ok(OperationAttemptId::from_str(
            "018f47f7-5a86-7c00-8000-000000000832",
        )?)
    }

    fn outbox_id() -> Result<OutboxMessageId, Box<dyn Error>> {
        Ok(OutboxMessageId::from_str(
            "018f47f7-5a86-7c00-8000-000000000833",
        )?)
    }

    fn approved_operation(store: &mut StateStore) -> Result<(), Box<dyn Error>> {
        let operation = store.create_operation(NewDurableOperation {
            binding: None,
            operation_id: operation_id()?,
            task_id: TaskId::from_str("018f47f7-5a86-7c00-8000-000000000834")?,
            action_proposal_id: ActionProposalId::from_str("018f47f7-5a86-7c00-8000-000000000835")?,
            account_id: AccountId::from_str("018f47f7-5a86-7c00-8000-000000000836")?,
            capability_id: CapabilityId::from_str("018f47f7-5a86-7c00-8000-000000000837")?,
            arguments_hash: ContentHash::from_bytes([0x83; 32]),
            source_schema: SchemaVersion::V1,
            created_at: UnixTimestampMicros::try_new(100)?,
        })?;
        store.transition_operation(
            operation.operation_id(),
            OperationTransition {
                expected_revision: operation.revision(),
                next_state: DurableOperationState::Approved,
                state_detail: None,
                attempt_identity: None,
                occurred_at: UnixTimestampMicros::try_new(110)?,
            },
        )?;
        Ok(())
    }

    fn message() -> Result<NewOutboxMessage, Box<dyn Error>> {
        Ok(NewOutboxMessage {
            outbox_id: outbox_id()?,
            operation_id: operation_id()?,
            attempt_identity: attempt_id()?,
            destination: BoundedText::try_new("connector://checkout")?,
            message_kind: BoundedText::try_new("commit")?,
            payload: br#"{"cart":"stable"}"#.to_vec(),
            created_at: UnixTimestampMicros::try_new(120)?,
        })
    }

    #[test]
    fn staging_atomically_marks_operation_pending_and_writes_outbox() -> Result<(), Box<dyn Error>>
    {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        let staged = store.stage_outbox(message()?, 1)?;
        assert_eq!(staged.state(), OutboxState::Pending);
        assert_eq!(
            store
                .load_operation(operation_id()?)?
                .ok_or("operation missing")?
                .state(),
            DurableOperationState::DispatchPending
        );
        assert_eq!(store.operation_journal(operation_id()?)?.len(), 3);
        Ok(())
    }

    #[test]
    fn only_lease_holder_can_begin_external_attempt() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        store.stage_outbox(message()?, 1)?;
        let owner = BoundedText::try_new("dispatcher-a")?;
        let claimed = store.claim_outbox(
            owner.clone(),
            UnixTimestampMicros::try_new(130)?,
            UnixTimestampMicros::try_new(230)?,
            1,
        )?;
        assert_eq!(claimed.len(), 1);
        assert_eq!(claimed[0].outbox_id(), outbox_id()?);
        assert_eq!(claimed[0].operation_id(), operation_id()?);
        assert_eq!(claimed[0].attempt_identity(), attempt_id()?);
        assert_eq!(
            claimed[0].lease_expires_at(),
            UnixTimestampMicros::try_new(230)?
        );
        let wrong_owner = BoundedText::try_new("dispatcher-b")?;
        let Err(error) = store.begin_dispatch(
            outbox_id()?,
            &wrong_owner,
            UnixTimestampMicros::try_new(140)?,
        ) else {
            return Err("wrong lease owner unexpectedly began dispatch".into());
        };
        assert!(matches!(error, OutboxError::LeaseNotHeld(_)));
        let attempt =
            store.begin_dispatch(outbox_id()?, &owner, UnixTimestampMicros::try_new(140)?)?;
        assert_eq!(attempt.attempt_identity(), attempt_id()?);
        assert_eq!(
            store
                .load_operation(operation_id()?)?
                .ok_or("operation missing")?
                .state(),
            DurableOperationState::Attempting
        );
        Ok(())
    }

    #[test]
    fn corrupted_payload_claim_is_opaque_and_dispatch_fails_closed() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        let original = message()?;
        let expected_hash = super::hash_payload(&original.payload).to_hex();
        store.stage_outbox(original, 1)?;
        store
            .connection
            .execute_batch("DROP TRIGGER outbox_immutable_identity;")?;
        let corrupted = br#"{"cart":"changed"}"#;
        let actual_hash = super::hash_payload(corrupted).to_hex();
        store.connection.execute(
            "UPDATE outbox_messages SET payload=?1 WHERE outbox_id=?2",
            params![corrupted, outbox_id()?.to_string()],
        )?;

        let owner = BoundedText::try_new("dispatcher")?;
        let claimed = store.claim_outbox(
            owner.clone(),
            UnixTimestampMicros::try_new(130)?,
            UnixTimestampMicros::try_new(230)?,
            1,
        )?;
        assert_eq!(claimed.len(), 1);
        assert_eq!(claimed[0].outbox_id(), outbox_id()?);
        assert_eq!(claimed[0].operation_id(), operation_id()?);
        assert_eq!(claimed[0].attempt_identity(), attempt_id()?);

        let error = store
            .begin_dispatch(outbox_id()?, &owner, UnixTimestampMicros::try_new(140)?)
            .err()
            .ok_or("corrupted payload unexpectedly began dispatch")?;
        assert!(matches!(
            &error,
            OutboxError::PayloadHashMismatch { outbox_id: id } if *id == outbox_id()?
        ));
        let display = error.to_string();
        let debug = format!("{error:?}");
        for hash in [expected_hash, actual_hash] {
            assert!(!display.contains(&hash));
            assert!(!debug.contains(&hash));
        }
        assert_eq!(
            store
                .load_operation(operation_id()?)?
                .ok_or("operation missing")?
                .state(),
            DurableOperationState::DispatchPending
        );
        let state: String = store.connection.query_row(
            "SELECT state FROM outbox_messages WHERE outbox_id=?1",
            [outbox_id()?.to_string()],
            |row| row.get(0),
        )?;
        assert_eq!(state, "leased");
        assert_eq!(store.operation_journal(operation_id()?)?.len(), 3);
        Ok(())
    }

    #[test]
    fn ambiguous_dispatch_becomes_reconciliation_not_success() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        store.stage_outbox(message()?, 1)?;
        let owner = BoundedText::try_new("dispatcher")?;
        store.claim_outbox(
            owner.clone(),
            UnixTimestampMicros::try_new(130)?,
            UnixTimestampMicros::try_new(230)?,
            1,
        )?;
        store.begin_dispatch(outbox_id()?, &owner, UnixTimestampMicros::try_new(140)?)?;
        store.record_dispatch_result(
            outbox_id()?,
            DispatchResult::Ambiguous(BoundedText::try_new("connection reset after write")?),
            UnixTimestampMicros::try_new(150)?,
        )?;
        assert_eq!(
            store
                .load_operation(operation_id()?)?
                .ok_or("operation missing")?
                .state(),
            DurableOperationState::NeedsReconciliation
        );
        Ok(())
    }
}
