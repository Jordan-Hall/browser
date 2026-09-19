use super::*;
use crate::test_support::*;
use intent_contracts::*;
use rusqlite::params;
use std::{
    fs,
    sync::{Arc, Barrier},
    thread,
};

#[test]
fn populated_zero_id_database_is_rejected_without_mutation() -> TestResult {
    let profile = Profile::new()?;
    let connection = Connection::open(profile.database())?;
    connection.execute_batch(
        "CREATE TABLE unrelated(value TEXT); INSERT INTO unrelated VALUES ('keep');",
    )?;
    drop(connection);
    let before = fs::read(profile.database())?;
    assert!(matches!(
        StateStore::open(profile.database()),
        Err(StateError::WrongApplicationId { found: 0 })
    ));
    assert_eq!(fs::read(profile.database())?, before);
    let connection = Connection::open(profile.database())?;
    assert_eq!(
        connection.query_row("SELECT value FROM unrelated", [], |row| row
            .get::<_, String>(0))?,
        "keep"
    );
    assert_eq!(
        connection.query_row("PRAGMA application_id", [], |row| row.get::<_, i64>(0))?,
        0
    );
    assert_eq!(
        connection.query_row("PRAGMA journal_mode", [], |row| row.get::<_, String>(0))?,
        "delete"
    );
    Ok(())
}

#[test]
fn user_version_disagreement_fails_without_rewriting_the_ledger() -> TestResult {
    let profile = Profile::new()?;
    drop(StateStore::open(profile.database())?);
    let connection = Connection::open(profile.database())?;
    connection.pragma_update(None, "user_version", 99)?;
    drop(connection);
    assert!(matches!(
        StateStore::open(profile.database()),
        Err(StateError::MigrationVersionMismatch {
            user_version: 99,
            ledger_version: 11
        })
    ));
    let connection = Connection::open(profile.database())?;
    assert_eq!(
        connection.query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))?,
        99
    );
    assert_eq!(
        connection.query_row("SELECT count(*) FROM schema_migrations", [], |row| row
            .get::<_, i64>(0))?,
        11
    );
    Ok(())
}

#[test]
fn migration_batch_failure_rolls_back_earlier_ddl_and_new_ledger() -> TestResult {
    let mut connection = Connection::open_in_memory()?;
    connection.execute_batch("CREATE TABLE durable_operations(unrelated TEXT);")?;
    assert!(migrations::apply_migrations(&mut connection).is_err());
    let introduced: i64 = connection.query_row(
        "SELECT count(*) FROM sqlite_schema WHERE name IN ('schema_migrations', 'store_metadata')",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(introduced, 0);
    assert_eq!(
        connection.query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))?,
        0
    );
    Ok(())
}

#[test]
fn concurrent_initializers_select_migrations_after_acquiring_writer_lock() -> TestResult {
    let profile = Profile::new()?;
    let connection = Connection::open(profile.database())?;
    connection.pragma_update(None, "journal_mode", "WAL")?;
    drop(connection);
    let barrier = Arc::new(Barrier::new(2));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let path = profile.database();
        let barrier = Arc::clone(&barrier);
        workers.push(thread::spawn(move || -> Result<(), String> {
            let mut connection = Connection::open(path).map_err(|e| e.to_string())?;
            connection
                .busy_timeout(Duration::from_secs(5))
                .map_err(|e| e.to_string())?;
            barrier.wait();
            migrations::apply_migrations(&mut connection).map_err(|e| e.to_string())
        }));
    }
    for worker in workers {
        worker.join().map_err(|_| "initializer thread unwound")??;
    }
    let connection = Connection::open(profile.database())?;
    assert_eq!(
        connection.query_row("SELECT count(*) FROM schema_migrations", [], |row| row
            .get::<_, i64>(0))?,
        11
    );
    migrations::validate_applied_migrations(&connection)?;
    Ok(())
}

#[test]
fn outcome_cannot_substitute_attempt_or_append_a_journal_entry() -> TestResult {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let operation = approved(&mut store, 1)?.operation_id();
    advance(
        &mut store,
        operation,
        DurableOperationState::DispatchPending,
        None,
    )?;
    let attempt: OperationAttemptId = id(900)?;
    let other = id(901)?;
    let attempting = advance(
        &mut store,
        operation,
        DurableOperationState::Attempting,
        Some(attempt),
    )?;
    let journal = store.operation_journal(operation)?;
    assert!(
        advance(
            &mut store,
            operation,
            DurableOperationState::Accepted,
            Some(other)
        )
        .is_err()
    );
    assert_eq!(store.load_operation(operation)?, Some(attempting));
    assert_eq!(store.operation_journal(operation)?, journal);
    advance(
        &mut store,
        operation,
        DurableOperationState::Accepted,
        Some(attempt),
    )?;
    advance(
        &mut store,
        operation,
        DurableOperationState::Verified,
        Some(attempt),
    )?;
    assert!(
        advance(
            &mut store,
            operation,
            DurableOperationState::Compensating,
            Some(attempt)
        )
        .is_err()
    );
    advance(
        &mut store,
        operation,
        DurableOperationState::Compensating,
        Some(other),
    )?;
    Ok(())
}

fn staged(store: &mut StateStore, number: u64) -> TestResult<OutboxMessage> {
    let operation = approved(store, number)?;
    Ok(store.stage_outbox(
        NewOutboxMessage {
            outbox_id: id(number + 10000)?,
            operation_id: operation.operation_id(),
            attempt_identity: id(number + 20000)?,
            destination: BoundedText::try_new("test://destination")?,
            message_kind: BoundedText::try_new("test")?,
            payload: b"payload".to_vec(),
            created_at: time(200 + i64::try_from(number)?)?,
        },
        operation.revision(),
    )?)
}

#[test]
fn staged_dispatch_projection_journal_and_attempt_share_identity() -> TestResult {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let message = staged(&mut store, 1)?;
    let operation = store
        .load_operation(message.operation_id())?
        .ok_or("missing operation")?;
    assert_eq!(
        operation.attempt_identity(),
        Some(message.attempt_identity())
    );
    let journal = store.operation_journal(message.operation_id())?;
    assert_eq!(
        journal.last().ok_or("missing journal")?.attempt_identity(),
        operation.attempt_identity()
    );
    assert!(
        advance(
            &mut store,
            message.operation_id(),
            DurableOperationState::Attempting,
            Some(id(999)?)
        )
        .is_err()
    );
    let owner = BoundedText::try_new("worker")?;
    store.claim_outbox(owner.clone(), time(300)?, time(400)?, 1)?;
    let dispatch = store.begin_dispatch(message.outbox_id(), &owner, time(301)?)?;
    assert_eq!(dispatch.attempt_identity(), message.attempt_identity());
    store.record_dispatch_result(message.outbox_id(), DispatchResult::Accepted, time(302)?)?;
    Ok(())
}

#[test]
fn cancelled_pending_and_expired_claims_cannot_starve_live_work() -> TestResult {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let expired = staged(&mut store, 1)?;
    let owner = BoundedText::try_new("worker")?;
    store.claim_outbox(owner.clone(), time(300)?, time(400)?, 1)?;
    advance(
        &mut store,
        expired.operation_id(),
        DurableOperationState::Cancelled,
        Some(expired.attempt_identity()),
    )?;
    let pending = staged(&mut store, 2)?;
    advance(
        &mut store,
        pending.operation_id(),
        DurableOperationState::Cancelled,
        Some(pending.attempt_identity()),
    )?;
    let live = staged(&mut store, 3)?;
    let claimed = store.claim_outbox(owner.clone(), time(500)?, time(600)?, 1)?;
    assert_eq!(claimed.len(), 1);
    assert_eq!(claimed[0].outbox_id(), live.outbox_id());
    advance(
        &mut store,
        live.operation_id(),
        DurableOperationState::Cancelled,
        Some(live.attempt_identity()),
    )?;
    assert!(
        store
            .begin_dispatch(live.outbox_id(), &owner, time(501)?)
            .is_err()
    );
    Ok(())
}

#[test]
fn dispatch_result_rejects_an_active_attempt_mismatch() -> TestResult {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let message = staged(&mut store, 1)?;
    let owner = BoundedText::try_new("worker")?;
    store.claim_outbox(owner.clone(), time(300)?, time(400)?, 1)?;
    store.begin_dispatch(message.outbox_id(), &owner, time(301)?)?;
    let other: OperationAttemptId = id(999)?;
    store.connection.execute(
        "UPDATE durable_operations SET attempt_identity = ?1 WHERE operation_id = ?2",
        params![other.to_string(), message.operation_id().to_string()],
    )?;
    let journal = store.operation_journal(message.operation_id())?;
    assert!(
        store
            .record_dispatch_result(message.outbox_id(), DispatchResult::Accepted, time(302)?)
            .is_err()
    );
    assert_eq!(store.operation_journal(message.operation_id())?, journal);
    assert_eq!(
        store
            .load_outbox(message.outbox_id())?
            .ok_or("missing outbox")?
            .state(),
        OutboxState::Attempting
    );
    Ok(())
}

fn inbox_fixture() -> TestResult<(BoundedText<128>, InboxEvent, Vec<ConsumerEffect>)> {
    Ok((
        BoundedText::try_new("consumer")?,
        InboxEvent {
            source: BoundedText::try_new("source")?,
            event_id: BoundedText::try_new("event")?,
            stream: BoundedText::try_new("stream")?,
            sequence: 1,
            payload: vec![7],
            received_at: time(1)?,
        },
        vec![ConsumerEffect {
            key: BoundedText::try_new("effect")?,
            payload: vec![8],
        }],
    ))
}

#[test]
fn aggregate_inbox_limit_is_atomic_and_inclusive() -> TestResult {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let (consumer, event, mut effects) = inbox_fixture()?;
    effects[0].payload = vec![8; MAX_INBOX_PAYLOAD_BYTES];
    assert!(matches!(
        store.apply_inbox_event(&consumer, event.clone(), effects.clone(), time(2)?),
        Err(InboxError::PayloadTooLarge { .. })
    ));
    assert_eq!(
        store.consumer_cursor(&consumer, &event.source, &event.stream)?,
        None
    );
    assert_eq!(
        store
            .connection
            .query_row("SELECT count(*) FROM inbox_events", [], |row| row
                .get::<_, i64>(0))?,
        0
    );
    effects[0].payload.pop();
    assert!(matches!(
        store.apply_inbox_event(&consumer, event.clone(), effects, time(2)?)?,
        InboxApplyResult::Applied { .. }
    ));
    assert_eq!(
        store.consumer_effects(&consumer, &event.source, &event.event_id)?[0]
            .payload()
            .len(),
        MAX_INBOX_PAYLOAD_BYTES - 1
    );
    Ok(())
}

#[test]
fn duplicate_acknowledgement_rejects_corrupt_or_missing_stored_data() -> TestResult {
    for corruption in [
        "DELETE FROM consumer_effects",
        "UPDATE consumer_effects SET payload = X'99'",
        "UPDATE inbox_events SET payload = X'99'",
    ] {
        let mut store = StateStore::open_in_memory_for_tests()?;
        let (consumer, event, effects) = inbox_fixture()?;
        store.apply_inbox_event(&consumer, event.clone(), effects.clone(), time(2)?)?;
        store.connection.execute_batch(corruption)?;
        assert!(
            store
                .apply_inbox_event(&consumer, event.clone(), effects, time(3)?)
                .is_err(),
            "corruption was acknowledged: {corruption}"
        );
        assert_eq!(
            store.consumer_cursor(&consumer, &event.source, &event.stream)?,
            Some(1)
        );
    }
    Ok(())
}

#[test]
fn restored_effects_cannot_bypass_the_aggregate_read_budget() -> TestResult {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let (consumer, event, effects) = inbox_fixture()?;
    store.apply_inbox_event(&consumer, event.clone(), effects, time(2)?)?;
    store.connection.execute(
        "UPDATE consumer_effects SET payload = ?1",
        [vec![8; MAX_INBOX_PAYLOAD_BYTES]],
    )?;
    assert!(matches!(
        store.consumer_effects(&consumer, &event.source, &event.event_id),
        Err(InboxError::PayloadTooLarge { .. })
    ));
    Ok(())
}
