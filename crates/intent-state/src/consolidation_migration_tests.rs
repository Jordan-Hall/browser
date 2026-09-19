use crate::{StateError, migrations};
use rusqlite::{Connection, params};
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

fn history(connection: &Connection) -> Result<Vec<(i64, String, String)>, rusqlite::Error> {
    connection
        .prepare("SELECT version, name, checksum FROM schema_migrations ORDER BY version")?
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
        .collect()
}

#[test]
fn canonical_v8_upgrade_keeps_task_checkpoint_bytes_and_migration_checksums() -> TestResult {
    let mut connection = Connection::open_in_memory()?;
    connection.pragma_update(None, "foreign_keys", "ON")?;
    migrations::apply_migrations_through(&mut connection, 8)?;
    let before = history(&connection)?;
    let payload = b"opaque existing task-checkpoint payload";
    connection.execute(
        "INSERT INTO task_checkpoints(checkpoint_id, task_id, workspace_id, privacy_scope, graph_revision, request_hash, payload_hash, payload, journal_watermark, created_at_micros) VALUES (?1, ?2, ?3, 'legacy', 0, ?4, ?4, ?5, 0, 1)",
        params![
            "018f47f7-5a86-7c00-8000-000000000001",
            "018f47f7-5a86-7c00-8000-000000000002",
            "018f47f7-5a86-7c00-8000-000000000003",
            "0".repeat(64),
            payload.as_slice(),
        ],
    )?;
    migrations::apply_migrations(&mut connection)?;
    let after = history(&connection)?;
    assert_eq!(&after[..8], before.as_slice());
    assert_eq!(after.len(), 12);
    assert_eq!(after[8].1, "consistent_workspace_checkpoints");
    assert_eq!(after[9].1, "durable_recovery_authority");
    assert_eq!(
        connection.query_row("SELECT payload FROM task_checkpoints", [], |row| {
            row.get::<_, Vec<u8>>(0)
        })?,
        payload
    );
    assert_eq!(
        connection.query_row("SELECT count(*) FROM workspace_checkpoints", [], |row| {
            row.get::<_, i64>(0)
        })?,
        0
    );
    migrations::validate_applied_migrations(&connection)?;
    migrations::apply_migrations(&mut connection)?;
    assert_eq!(history(&connection)?, after);
    Ok(())
}

#[test]
fn divergent_branch_v8_history_is_rejected_without_appending_migrations() -> TestResult {
    let mut connection = Connection::open_in_memory()?;
    migrations::apply_migrations_through(&mut connection, 8)?;
    let other_checksum =
        migrations::migration_checksum(include_str!("workspace_checkpoints/migration.sql"))
            .to_hex();
    connection.execute(
        "UPDATE schema_migrations SET checksum = ?1 WHERE version = 8",
        [other_checksum],
    )?;
    let before = history(&connection)?;
    assert!(matches!(
        migrations::apply_migrations(&mut connection),
        Err(StateError::MigrationChecksumMismatch { version: 8, .. })
    ));
    assert_eq!(history(&connection)?, before);
    assert_eq!(
        connection.query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))?,
        8
    );
    assert_eq!(
        connection.query_row(
            "SELECT count(*) FROM sqlite_schema WHERE name = 'workspace_checkpoints'",
            [],
            |row| row.get::<_, i64>(0),
        )?,
        0
    );
    Ok(())
}
