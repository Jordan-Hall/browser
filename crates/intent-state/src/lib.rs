#![forbid(unsafe_code)]
#![doc = "Single-owner durable local state for the Intent Browser trusted runtime."]

mod migrations;

use migrations::apply_migrations;
use rusqlite::{Connection, TransactionBehavior};
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::time::Duration;
use uuid::Uuid;

const APPLICATION_ID: i64 = 0x494E_544E;
const BUSY_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub struct StateStore {
    connection: Connection,
}

impl StateStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StateError> {
        let mut connection = Connection::open(path)?;
        configure_connection(&mut connection, true)?;
        apply_migrations(&mut connection)?;
        initialize_store_metadata(&connection)?;
        Ok(Self { connection })
    }

    #[doc(hidden)]
    pub fn open_in_memory_for_tests() -> Result<Self, StateError> {
        let mut connection = Connection::open_in_memory()?;
        configure_connection(&mut connection, false)?;
        apply_migrations(&mut connection)?;
        initialize_store_metadata(&connection)?;
        Ok(Self { connection })
    }

    pub fn schema_version(&self) -> Result<i64, StateError> {
        Ok(self
            .connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))?)
    }

    pub fn store_id(&self) -> Result<Uuid, StateError> {
        let value: String = self.connection.query_row(
            "SELECT store_uuid FROM store_metadata WHERE singleton = 1",
            [],
            |row| row.get(0),
        )?;
        Uuid::parse_str(&value).map_err(|_| StateError::InvalidStoreId(value))
    }

    pub fn journal_mode(&self) -> Result<String, StateError> {
        Ok(self
            .connection
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))?)
    }

    pub fn foreign_keys_enabled(&self) -> Result<bool, StateError> {
        let enabled: i64 = self
            .connection
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))?;
        Ok(enabled == 1)
    }

    pub fn integrity_check(&self) -> Result<(), StateError> {
        let result: String = self
            .connection
            .query_row("PRAGMA quick_check", [], |row| row.get(0))?;
        if result == "ok" {
            return Ok(());
        }
        Err(StateError::IntegrityCheckFailed(result))
    }
}

fn configure_connection(connection: &mut Connection, require_wal: bool) -> Result<(), StateError> {
    connection.busy_timeout(BUSY_TIMEOUT)?;
    {
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let application_id: i64 =
            transaction.query_row("PRAGMA application_id", [], |row| row.get(0))?;
        match application_id {
            0 => {
                let populated: bool = transaction.query_row(
                    "SELECT EXISTS(SELECT 1 FROM sqlite_schema)",
                    [],
                    |row| row.get(0),
                )?;
                let user_version: i64 =
                    transaction.query_row("PRAGMA user_version", [], |row| row.get(0))?;
                if populated || user_version != 0 {
                    return Err(StateError::WrongApplicationId { found: 0 });
                }
                transaction.pragma_update(None, "application_id", APPLICATION_ID)?;
            }
            APPLICATION_ID => {}
            found => return Err(StateError::WrongApplicationId { found }),
        }
        transaction.commit()?;
    }
    connection.pragma_update(None, "foreign_keys", "ON")?;
    connection.pragma_update(None, "trusted_schema", "OFF")?;
    connection.pragma_update(None, "synchronous", "FULL")?;
    if require_wal {
        let mode: String =
            connection.query_row("PRAGMA journal_mode = WAL", [], |row| row.get(0))?;
        if !mode.eq_ignore_ascii_case("wal") {
            return Err(StateError::WalUnavailable(mode));
        }
        connection.pragma_update(None, "wal_autocheckpoint", 1_000_i64)?;
    }
    Ok(())
}

fn initialize_store_metadata(connection: &Connection) -> Result<(), StateError> {
    let store_id = Uuid::new_v4().to_string();
    connection.execute(
        "INSERT OR IGNORE INTO store_metadata(singleton, store_uuid, created_unix_seconds) VALUES (1, ?1, unixepoch())",
        [&store_id],
    )?;
    Ok(())
}

#[derive(Debug)]
pub enum StateError {
    Sqlite(rusqlite::Error),
    WrongApplicationId {
        found: i64,
    },
    WalUnavailable(String),
    InvalidStoreId(String),
    IntegrityCheckFailed(String),
    MigrationVersionMismatch {
        user_version: i64,
        ledger_version: i64,
    },
    MigrationGap {
        expected: i64,
        found: i64,
    },
    UnknownAppliedMigration {
        version: i64,
    },
    MigrationChecksumMismatch {
        version: i64,
        expected_name: &'static str,
        stored_name: String,
        expected_checksum: String,
        stored_checksum: String,
    },
}

impl fmt::Display for StateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => write!(formatter, "SQLite state error: {error}"),
            Self::WrongApplicationId { found } => write!(
                formatter,
                "database application_id {found} does not belong to Intent Browser"
            ),
            Self::WalUnavailable(mode) => {
                write!(
                    formatter,
                    "SQLite WAL mode is unavailable; active mode is {mode}"
                )
            }
            Self::InvalidStoreId(value) => {
                write!(formatter, "stored state UUID is invalid: {value}")
            }
            Self::IntegrityCheckFailed(detail) => {
                write!(formatter, "SQLite quick_check failed: {detail}")
            }
            Self::MigrationVersionMismatch {
                user_version,
                ledger_version,
            } => write!(
                formatter,
                "schema user_version {user_version} disagrees with migration ledger {ledger_version}"
            ),
            Self::MigrationGap { expected, found } => write!(
                formatter,
                "migration ledger has a gap: expected version {expected}, found {found}"
            ),
            Self::UnknownAppliedMigration { version } => write!(
                formatter,
                "database contains unknown applied migration version {version}"
            ),
            Self::MigrationChecksumMismatch {
                version,
                expected_name,
                stored_name,
                expected_checksum,
                stored_checksum,
            } => write!(
                formatter,
                "migration {version} does not match compiled history: expected {expected_name}/{expected_checksum}, stored {stored_name}/{stored_checksum}"
            ),
        }
    }
}

impl Error for StateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Sqlite(error) => Some(error),
            _ => None,
        }
    }
}

impl From<rusqlite::Error> for StateError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{APPLICATION_ID, StateError, StateStore};
    use rusqlite::Connection;
    use std::error::Error;
    use std::fs;
    use std::path::{Path, PathBuf};
    use uuid::Uuid;

    struct TempDatabase {
        path: PathBuf,
    }

    impl TempDatabase {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "intent-state-{}-{}.sqlite3",
                std::process::id(),
                Uuid::new_v4()
            ));
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempDatabase {
        fn drop(&mut self) {
            for suffix in ["", "-wal", "-shm"] {
                let candidate = PathBuf::from(format!("{}{}", self.path.display(), suffix));
                let _ = fs::remove_file(candidate);
            }
        }
    }

    #[test]
    fn file_store_bootstraps_wal_migrations_and_identity() -> Result<(), Box<dyn Error>> {
        let temp = TempDatabase::new();
        let store = StateStore::open(temp.path())?;
        assert_eq!(store.schema_version()?, 1);
        assert_eq!(store.journal_mode()?.to_ascii_lowercase(), "wal");
        assert!(store.foreign_keys_enabled()?);
        store.integrity_check()?;
        let first_id = store.store_id()?;
        drop(store);

        let reopened = StateStore::open(temp.path())?;
        assert_eq!(reopened.store_id()?, first_id);
        Ok(())
    }

    #[test]
    fn altered_migration_checksum_fails_closed() -> Result<(), Box<dyn Error>> {
        let temp = TempDatabase::new();
        let store = StateStore::open(temp.path())?;
        drop(store);

        let connection = Connection::open(temp.path())?;
        connection.execute(
            "UPDATE schema_migrations SET checksum = ?1 WHERE version = 1",
            ["0000000000000000000000000000000000000000000000000000000000000000"],
        )?;
        drop(connection);

        let Err(error) = StateStore::open(temp.path()) else {
            return Err("tampered migration unexpectedly opened".into());
        };
        assert!(matches!(
            error,
            StateError::MigrationChecksumMismatch { version: 1, .. }
        ));
        Ok(())
    }

    #[test]
    fn wrong_application_id_is_rejected() -> Result<(), Box<dyn Error>> {
        let temp = TempDatabase::new();
        let connection = Connection::open(temp.path())?;
        connection.pragma_update(None, "application_id", APPLICATION_ID + 1)?;
        drop(connection);

        let Err(error) = StateStore::open(temp.path()) else {
            return Err("foreign database unexpectedly opened".into());
        };
        assert!(matches!(error, StateError::WrongApplicationId { .. }));
        Ok(())
    }

    #[test]
    fn foreign_key_enforcement_is_effective() -> Result<(), Box<dyn Error>> {
        let store = StateStore::open_in_memory_for_tests()?;
        store.connection.execute_batch(
            "CREATE TABLE parent(id INTEGER PRIMARY KEY); CREATE TABLE child(parent_id INTEGER NOT NULL REFERENCES parent(id));",
        )?;
        assert!(
            store
                .connection
                .execute("INSERT INTO child(parent_id) VALUES (99)", [])
                .is_err()
        );
        Ok(())
    }
}

#[cfg(test)]
mod hardening_tests;
#[cfg(test)]
mod test_support;
