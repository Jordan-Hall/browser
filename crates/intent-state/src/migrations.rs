use crate::StateError;
use intent_contracts::ContentHash;
use rusqlite::{Connection, TransactionBehavior, params};
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Migration {
    pub version: i64,
    pub name: &'static str,
    pub sql: &'static str,
}

const MIGRATION_001: &str = r#"
CREATE TABLE store_metadata (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    store_uuid TEXT NOT NULL UNIQUE,
    created_unix_seconds INTEGER NOT NULL
) STRICT;
"#;

const MIGRATION_002: &str = r#"
CREATE TABLE durable_operations (
    operation_id TEXT PRIMARY KEY NOT NULL,
    task_id TEXT NOT NULL,
    action_proposal_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    capability_id TEXT NOT NULL,
    arguments_hash TEXT NOT NULL CHECK (length(arguments_hash) = 64),
    source_schema_major INTEGER NOT NULL CHECK (source_schema_major > 0),
    source_schema_minor INTEGER NOT NULL CHECK (source_schema_minor >= 0),
    state TEXT NOT NULL CHECK (state IN (
        'prepared', 'approved', 'dispatch_pending', 'attempting', 'accepted',
        'verified', 'failed', 'needs_reconciliation', 'cancelled',
        'compensating', 'compensated'
    )),
    state_detail TEXT,
    attempt_identity TEXT,
    revision INTEGER NOT NULL CHECK (revision >= 0),
    created_at_micros INTEGER NOT NULL,
    updated_at_micros INTEGER NOT NULL
) STRICT;

CREATE INDEX durable_operations_task_idx ON durable_operations(task_id);
CREATE INDEX durable_operations_state_idx ON durable_operations(state);

CREATE TABLE operation_journal (
    sequence INTEGER PRIMARY KEY,
    operation_id TEXT NOT NULL REFERENCES durable_operations(operation_id) ON DELETE RESTRICT,
    revision INTEGER NOT NULL CHECK (revision >= 0),
    from_state TEXT,
    to_state TEXT NOT NULL,
    state_detail TEXT,
    attempt_identity TEXT,
    occurred_at_micros INTEGER NOT NULL,
    UNIQUE(operation_id, revision)
) STRICT;

CREATE INDEX operation_journal_operation_idx
    ON operation_journal(operation_id, revision);

CREATE TRIGGER durable_operations_immutable_identity
BEFORE UPDATE OF
    task_id, action_proposal_id, account_id, capability_id, arguments_hash,
    source_schema_major, source_schema_minor, created_at_micros
ON durable_operations
BEGIN
    SELECT RAISE(ABORT, 'durable operation identity is immutable');
END;
"#;

const MIGRATION_003: &str = r#"
CREATE TABLE outbox_messages (
    outbox_id TEXT PRIMARY KEY NOT NULL,
    operation_id TEXT NOT NULL REFERENCES durable_operations(operation_id) ON DELETE RESTRICT,
    attempt_identity TEXT NOT NULL,
    destination TEXT NOT NULL CHECK (length(destination) BETWEEN 1 AND 512),
    message_kind TEXT NOT NULL CHECK (length(message_kind) BETWEEN 1 AND 128),
    payload BLOB NOT NULL CHECK (length(payload) <= 1048576),
    payload_hash TEXT NOT NULL CHECK (length(payload_hash) = 64),
    state TEXT NOT NULL CHECK (state IN ('pending', 'leased', 'attempting', 'completed', 'failed')),
    lease_owner TEXT,
    lease_expires_at_micros INTEGER,
    dispatch_started_at_micros INTEGER,
    completed_at_micros INTEGER,
    failure_detail TEXT,
    created_at_micros INTEGER NOT NULL,
    updated_at_micros INTEGER NOT NULL,
    UNIQUE(operation_id, attempt_identity),
    CHECK ((lease_owner IS NULL) = (lease_expires_at_micros IS NULL)),
    CHECK (state != 'leased' OR lease_owner IS NOT NULL),
    CHECK (state != 'attempting' OR dispatch_started_at_micros IS NOT NULL)
) STRICT;

CREATE INDEX outbox_ready_idx
    ON outbox_messages(state, lease_expires_at_micros, created_at_micros);
CREATE INDEX outbox_operation_idx
    ON outbox_messages(operation_id, created_at_micros);

CREATE TRIGGER outbox_immutable_identity
BEFORE UPDATE OF
    operation_id, attempt_identity, destination, message_kind, payload, payload_hash, created_at_micros
ON outbox_messages
BEGIN
    SELECT RAISE(ABORT, 'outbox message identity and payload are immutable');
END;
"#;

const MIGRATION_004: &str = r#"
CREATE TABLE inbox_events (
    source TEXT NOT NULL CHECK (length(source) BETWEEN 1 AND 128),
    event_id TEXT NOT NULL CHECK (length(event_id) BETWEEN 1 AND 256),
    stream TEXT NOT NULL CHECK (length(stream) BETWEEN 1 AND 128),
    sequence INTEGER NOT NULL CHECK (sequence >= 0),
    payload BLOB NOT NULL CHECK (length(payload) <= 1048576),
    payload_hash TEXT NOT NULL CHECK (length(payload_hash) = 64),
    received_at_micros INTEGER NOT NULL,
    PRIMARY KEY(source, event_id),
    UNIQUE(source, stream, sequence)
) STRICT, WITHOUT ROWID;

CREATE TABLE consumer_events (
    consumer TEXT NOT NULL CHECK (length(consumer) BETWEEN 1 AND 128),
    source TEXT NOT NULL,
    event_id TEXT NOT NULL,
    stream TEXT NOT NULL,
    sequence INTEGER NOT NULL CHECK (sequence >= 0),
    effects_hash TEXT NOT NULL CHECK (length(effects_hash) = 64),
    processed_at_micros INTEGER NOT NULL,
    PRIMARY KEY(consumer, source, event_id),
    FOREIGN KEY(source, event_id) REFERENCES inbox_events(source, event_id) ON DELETE RESTRICT
) STRICT, WITHOUT ROWID;

CREATE TABLE consumer_cursors (
    consumer TEXT NOT NULL CHECK (length(consumer) BETWEEN 1 AND 128),
    source TEXT NOT NULL CHECK (length(source) BETWEEN 1 AND 128),
    stream TEXT NOT NULL CHECK (length(stream) BETWEEN 1 AND 128),
    last_sequence INTEGER NOT NULL CHECK (last_sequence >= 0),
    updated_at_micros INTEGER NOT NULL,
    PRIMARY KEY(consumer, source, stream)
) STRICT, WITHOUT ROWID;

CREATE TABLE consumer_effects (
    consumer TEXT NOT NULL,
    source TEXT NOT NULL,
    event_id TEXT NOT NULL,
    effect_key TEXT NOT NULL CHECK (length(effect_key) BETWEEN 1 AND 256),
    payload BLOB NOT NULL CHECK (length(payload) <= 1048576),
    payload_hash TEXT NOT NULL CHECK (length(payload_hash) = 64),
    PRIMARY KEY(consumer, source, event_id, effect_key),
    FOREIGN KEY(consumer, source, event_id)
        REFERENCES consumer_events(consumer, source, event_id) ON DELETE RESTRICT
) STRICT, WITHOUT ROWID;

CREATE INDEX consumer_events_stream_idx
    ON consumer_events(consumer, source, stream, sequence);
"#;

const MIGRATION_005: &str = r#"
CREATE TABLE artifact_blobs (
    privacy_scope TEXT NOT NULL CHECK (length(privacy_scope) BETWEEN 1 AND 128),
    content_hash TEXT NOT NULL CHECK (length(content_hash) = 64),
    byte_size INTEGER NOT NULL CHECK (byte_size >= 0 AND byte_size <= 268435456),
    created_at_micros INTEGER NOT NULL,
    PRIMARY KEY(privacy_scope, content_hash)
) STRICT, WITHOUT ROWID;

CREATE TABLE artifact_handles (
    artifact_id TEXT PRIMARY KEY NOT NULL,
    privacy_scope TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    byte_size INTEGER NOT NULL CHECK (byte_size >= 0 AND byte_size <= 268435456),
    media_type TEXT NOT NULL CHECK (length(media_type) BETWEEN 1 AND 255),
    created_at_micros INTEGER NOT NULL,
    FOREIGN KEY(privacy_scope, content_hash)
        REFERENCES artifact_blobs(privacy_scope, content_hash) ON DELETE RESTRICT
) STRICT;

CREATE INDEX artifact_handles_scope_hash_idx
    ON artifact_handles(privacy_scope, content_hash);

CREATE TABLE artifact_references (
    artifact_id TEXT NOT NULL REFERENCES artifact_handles(artifact_id) ON DELETE RESTRICT,
    reference_kind TEXT NOT NULL CHECK (length(reference_kind) BETWEEN 1 AND 64),
    reference_id TEXT NOT NULL CHECK (length(reference_id) BETWEEN 1 AND 256),
    created_at_micros INTEGER NOT NULL,
    PRIMARY KEY(artifact_id, reference_kind, reference_id)
) STRICT, WITHOUT ROWID;

CREATE TRIGGER artifact_blobs_immutable
BEFORE UPDATE OF privacy_scope, content_hash, byte_size, created_at_micros
ON artifact_blobs
BEGIN
    SELECT RAISE(ABORT, 'artifact blob identity is immutable');
END;

CREATE TRIGGER artifact_handles_immutable
BEFORE UPDATE OF artifact_id, privacy_scope, content_hash, byte_size, media_type, created_at_micros
ON artifact_handles
BEGIN
    SELECT RAISE(ABORT, 'artifact handle metadata is immutable');
END;
"#;

pub(crate) const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "bootstrap_store_metadata",
        sql: MIGRATION_001,
    },
    Migration {
        version: 2,
        name: "durable_operation_journal",
        sql: MIGRATION_002,
    },
    Migration {
        version: 3,
        name: "transactional_outbox",
        sql: MIGRATION_003,
    },
    Migration {
        version: 4,
        name: "inbox_deduplication",
        sql: MIGRATION_004,
    },
    Migration {
        version: 5,
        name: "scoped_immutable_artifacts",
        sql: MIGRATION_005,
    },
];

pub(crate) fn apply_migrations(connection: &mut Connection) -> Result<(), StateError> {
    connection.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY CHECK (version > 0),
            name TEXT NOT NULL UNIQUE,
            checksum TEXT NOT NULL CHECK (length(checksum) = 64),
            applied_unix_seconds INTEGER NOT NULL
        ) STRICT;
        "#,
    )?;

    validate_applied_migrations(connection)?;

    let highest: i64 = connection.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
        [],
        |row| row.get(0),
    )?;

    for migration in MIGRATIONS
        .iter()
        .filter(|migration| migration.version > highest)
    {
        let checksum = migration_checksum(migration.sql).to_hex();
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        transaction.execute_batch(migration.sql)?;
        transaction.execute(
            "INSERT INTO schema_migrations(version, name, checksum, applied_unix_seconds) VALUES (?1, ?2, ?3, unixepoch())",
            params![migration.version, migration.name, checksum],
        )?;
        transaction.pragma_update(None, "user_version", migration.version)?;
        transaction.commit()?;
    }

    validate_applied_migrations(connection)
}

pub(crate) fn validate_applied_migrations(connection: &Connection) -> Result<(), StateError> {
    let mut statement = connection
        .prepare("SELECT version, name, checksum FROM schema_migrations ORDER BY version ASC")?;
    let mut rows = statement.query([])?;
    let mut expected_version = 1_i64;

    while let Some(row) = rows.next()? {
        let version: i64 = row.get(0)?;
        let name: String = row.get(1)?;
        let checksum: String = row.get(2)?;

        if version != expected_version {
            return Err(StateError::MigrationGap {
                expected: expected_version,
                found: version,
            });
        }

        let Some(known) = MIGRATIONS
            .iter()
            .find(|migration| migration.version == version)
        else {
            return Err(StateError::UnknownAppliedMigration { version });
        };
        let expected_checksum = migration_checksum(known.sql).to_hex();
        if known.name != name || expected_checksum != checksum {
            return Err(StateError::MigrationChecksumMismatch {
                version,
                expected_name: known.name,
                stored_name: name,
                expected_checksum,
                stored_checksum: checksum,
            });
        }

        expected_version += 1;
    }

    Ok(())
}

#[must_use]
pub(crate) fn migration_checksum(sql: &str) -> ContentHash {
    let digest = Sha256::digest(sql.as_bytes());
    let mut bytes = [0_u8; 32];
    bytes.copy_from_slice(&digest);
    ContentHash::from_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::{MIGRATIONS, migration_checksum};
    use std::error::Error;

    #[test]
    fn migration_versions_are_contiguous_and_checksums_are_stable_length()
    -> Result<(), Box<dyn Error>> {
        for (index, migration) in MIGRATIONS.iter().enumerate() {
            let expected = i64::try_from(index)? + 1;
            assert_eq!(migration.version, expected);
            assert_eq!(migration_checksum(migration.sql).to_hex().len(), 64);
        }
        Ok(())
    }
}
