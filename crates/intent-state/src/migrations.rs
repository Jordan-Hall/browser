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

pub(crate) const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    name: "bootstrap_store_metadata",
    sql: MIGRATION_001,
}];

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

    for migration in MIGRATIONS.iter().filter(|migration| migration.version > highest) {
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
    let mut statement = connection.prepare(
        "SELECT version, name, checksum FROM schema_migrations ORDER BY version ASC",
    )?;
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

        let Some(known) = MIGRATIONS.iter().find(|migration| migration.version == version) else {
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

    #[test]
    fn migration_versions_are_contiguous_and_checksums_are_stable_length() {
        for (index, migration) in MIGRATIONS.iter().enumerate() {
            assert_eq!(migration.version, i64::try_from(index).unwrap_or_default() + 1);
            assert_eq!(migration_checksum(migration.sql).to_hex().len(), 64);
        }
    }
}
