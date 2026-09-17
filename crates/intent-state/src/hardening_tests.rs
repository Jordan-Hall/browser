use super::*;
use crate::test_support::*;
use intent_contracts::*;

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
            ledger_version: 2
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
        2
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
        2
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
