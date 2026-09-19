use super::*;
use crate::{MAX_OUTBOX_PAYLOAD_BYTES, OutboxError, test_support::Profile};

#[test]
fn persisted_staging_time_cannot_disagree_with_validation_time() -> Result<(), Box<dyn Error>> {
    for created_at in [119, 121, 200] {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        let mut new = message()?;
        new.created_at = UnixTimestampMicros::try_new(created_at)?;
        assert!(
            store
                .stage_record_bound_outbox(
                    &proposal()?,
                    &approved_record()?,
                    new,
                    1,
                    UnixTimestampMicros::try_new(120)?,
                )
                .is_err(),
            "a different staging time was persisted"
        );
        assert_unstaged(&store)?;
    }
    Ok(())
}

#[test]
fn each_persisted_identity_and_digest_rejects_rebinding() -> Result<(), Box<dyn Error>> {
    for (field, expected) in [
        ("id", "proposal id"),
        ("task_id", "task id"),
        ("account_id", "account id"),
        ("capability_id", "capability id"),
        ("arguments_hash", "argument hash"),
    ] {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        let mut value = serde_json::to_value(proposal()?)?;
        if field == "arguments_hash" {
            let hash = serde_json::to_value(ContentHash::from_bytes([0xff; 32]))?;
            value[field] = hash.clone();
            value["canonical_arguments"]["content_hash"] = hash;
        } else {
            value[field] = json!("018f47f7-5a86-7c00-8000-000000000aff");
        }
        let rebound: ActionProposal = serde_json::from_value(value)?;
        let error = store
            .stage_record_bound_outbox(
                &rebound,
                &approved_record()?,
                message()?,
                1,
                UnixTimestampMicros::try_new(120)?,
            )
            .err()
            .ok_or("rebound proposal was staged")?;
        assert!(
            matches!(error, AuthorizedDispatchError::BindingMismatch(actual) if actual == expected)
        );
        assert_unstaged(&store)?;
    }
    Ok(())
}

#[test]
fn approval_identity_hash_and_state_must_match() -> Result<(), Box<dyn Error>> {
    let mut values = Vec::new();
    for field in ["action_proposal_id", "exact_arguments_hash"] {
        let mut value = serde_json::to_value(approved_record()?)?;
        value[field] = if field == "action_proposal_id" {
            json!("018f47f7-5a86-7c00-8000-000000000aff")
        } else {
            serde_json::to_value(ContentHash::from_bytes([0xff; 32]))?
        };
        values.push(serde_json::from_value::<Approval>(value)?);
    }
    for state in [
        ApprovalState::Pending,
        ApprovalState::Rejected {
            rejected_at: UnixTimestampMicros::try_new(111)?,
        },
        ApprovalState::Revoked {
            revoked_at: UnixTimestampMicros::try_new(111)?,
        },
        ApprovalState::Expired {
            expired_at: UnixTimestampMicros::try_new(111)?,
        },
        ApprovalState::Approved {
            approved_at: UnixTimestampMicros::try_new(121)?,
            expires_at: None,
        },
    ] {
        values.push(approval(state)?);
    }
    for value in values {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        assert!(
            store
                .stage_record_bound_outbox(
                    &proposal()?,
                    &value,
                    message()?,
                    1,
                    UnixTimestampMicros::try_new(120)?,
                )
                .is_err()
        );
        assert_unstaged(&store)?;
    }
    Ok(())
}

#[test]
fn stale_revisions_and_oversized_payloads_leave_no_staging_residue() -> Result<(), Box<dyn Error>> {
    let mut store = StateStore::open_in_memory_for_tests()?;
    approved_operation(&mut store)?;
    let error = store
        .stage_record_bound_outbox(
            &proposal()?,
            &approved_record()?,
            message()?,
            0,
            UnixTimestampMicros::try_new(120)?,
        )
        .err()
        .ok_or("stale revision was accepted")?;
    assert!(matches!(
        error,
        AuthorizedDispatchError::Outbox(OutboxError::StaleOperationRevision { .. })
    ));
    assert_unstaged(&store)?;
    let mut oversized = message()?;
    oversized.payload = vec![0; MAX_OUTBOX_PAYLOAD_BYTES + 1];
    let error = store
        .stage_record_bound_outbox(
            &proposal()?,
            &approved_record()?,
            oversized,
            1,
            UnixTimestampMicros::try_new(120)?,
        )
        .err()
        .ok_or("oversized payload was accepted")?;
    assert!(matches!(
        error,
        AuthorizedDispatchError::Outbox(OutboxError::PayloadTooLarge(_))
    ));
    assert_unstaged(&store)?;
    Ok(())
}

#[test]
fn failed_outbox_insert_rolls_back_the_checked_operation_and_journal() -> Result<(), Box<dyn Error>>
{
    let mut store = StateStore::open_in_memory_for_tests()?;
    approved_operation(&mut store)?;
    store.connection.execute_batch(
        "CREATE TEMP TRIGGER fail_checked_stage BEFORE INSERT ON outbox_messages
         BEGIN SELECT RAISE(ABORT, 'injected staging failure'); END;",
    )?;
    assert!(
        store
            .stage_record_bound_outbox(
                &proposal()?,
                &approved_record()?,
                message()?,
                1,
                UnixTimestampMicros::try_new(120)?,
            )
            .is_err()
    );
    assert_unstaged(&store)?;
    Ok(())
}

#[test]
fn post_insert_projection_mismatch_rolls_back_before_commit() -> Result<(), Box<dyn Error>> {
    let mut store = StateStore::open_in_memory_for_tests()?;
    approved_operation(&mut store)?;
    store.connection.execute_batch(
        "DROP TRIGGER outbox_immutable_identity;
         CREATE TEMP TRIGGER corrupt_checked_stage AFTER INSERT ON outbox_messages
         BEGIN
           UPDATE outbox_messages
           SET destination='connector://tampered'
           WHERE outbox_id=NEW.outbox_id;
         END;",
    )?;
    let error = store
        .stage_record_bound_outbox(
            &proposal()?,
            &approved_record()?,
            message()?,
            1,
            UnixTimestampMicros::try_new(120)?,
        )
        .err()
        .ok_or("corrupted staged projection unexpectedly committed")?;
    assert!(matches!(
        error,
        AuthorizedDispatchError::Outbox(OutboxError::InvalidStoredRecord(_))
    ));
    assert_unstaged(&store)?;
    Ok(())
}

#[test]
fn post_insert_terminal_metadata_rolls_back_before_commit() -> Result<(), Box<dyn Error>> {
    for mutation in [
        "completed_at_micros=121",
        "failure_detail='injected terminal metadata'",
    ] {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        store.connection.execute_batch(&format!(
            "CREATE TEMP TRIGGER corrupt_terminal_stage AFTER INSERT ON outbox_messages
             BEGIN
               UPDATE outbox_messages SET {mutation} WHERE outbox_id=NEW.outbox_id;
             END;"
        ))?;
        let error = store
            .stage_record_bound_outbox(
                &proposal()?,
                &approved_record()?,
                message()?,
                1,
                UnixTimestampMicros::try_new(120)?,
            )
            .err()
            .ok_or("staged row with terminal metadata unexpectedly committed")?;
        assert!(matches!(
            error,
            AuthorizedDispatchError::Outbox(OutboxError::InvalidStoredRecord(_))
        ));
        assert_unstaged(&store)?;
    }
    Ok(())
}

#[test]
fn imported_approved_record_cannot_authorize_file_backed_dispatch() -> Result<(), Box<dyn Error>> {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    approved_operation(&mut store)?;
    let imported: Approval = serde_json::from_slice(&serde_json::to_vec(&approved_record()?)?)?;
    store.stage_record_bound_outbox(
        &proposal()?,
        &imported,
        message()?,
        1,
        UnixTimestampMicros::try_new(120)?,
    )?;
    let worker = BoundedText::try_new("untrusted-record")?;
    let claimed = store.claim_outbox(
        worker.clone(),
        UnixTimestampMicros::try_new(121)?,
        UnixTimestampMicros::try_new(130)?,
        1,
    )?;
    assert_eq!(claimed.len(), 1);
    assert!(matches!(
        store.begin_dispatch(outbox_id()?, &worker, UnixTimestampMicros::try_new(122)?),
        Err(OutboxError::RecoveryRequired)
    ));
    assert!(!store.dispatch_status()?.enabled);
    assert_eq!(store.operation_journal(operation_id()?)?.len(), 3);
    let outbox = store
        .load_outbox(outbox_id()?)?
        .ok_or("missing staged record")?;
    assert_eq!(outbox.state(), OutboxState::Leased);
    assert_eq!(outbox.dispatch_started_at(), None);
    Ok(())
}
#[test]
fn staging_cannot_predate_the_durable_approval_transition() -> Result<(), Box<dyn Error>> {
    let mut store = StateStore::open_in_memory_for_tests()?;
    approved_operation(&mut store)?;
    let mut new = message()?;
    new.created_at = UnixTimestampMicros::try_new(109)?;
    let earlier = approval(ApprovalState::Approved {
        approved_at: UnixTimestampMicros::try_new(108)?,
        expires_at: None,
    })?;
    assert!(
        store
            .stage_record_bound_outbox(
                &proposal()?,
                &earlier,
                new,
                1,
                UnixTimestampMicros::try_new(109)?,
            )
            .is_err()
    );
    assert_unstaged(&store)?;
    Ok(())
}
