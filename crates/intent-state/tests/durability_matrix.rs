use intent_state::StateStore;
use rusqlite::Connection;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

const APPLICATION_ID: i64 = 0x494E_544E;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

struct TempDatabase {
    path: PathBuf,
}

impl TempDatabase {
    fn new() -> Self {
        Self {
            path: std::env::temp_dir().join(format!(
                "intent-state-durability-matrix-{}-{}.sqlite3",
                std::process::id(),
                Uuid::new_v4()
            )),
        }
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
fn native_file_store_retains_wal_identity_and_integrity_across_reopen() -> TestResult {
    let temp = TempDatabase::new();
    let store = StateStore::open(temp.path())?;
    let store_id = store.store_id()?;
    let schema_version = store.schema_version()?;
    assert!(schema_version > 0);
    assert_eq!(store.journal_mode()?.to_ascii_lowercase(), "wal");
    assert!(store.foreign_keys_enabled()?);
    store.integrity_check()?;
    drop(store);

    let independent = Connection::open(temp.path())?;
    let application_id: i64 =
        independent.query_row("PRAGMA application_id", [], |row| row.get(0))?;
    let journal_mode: String =
        independent.query_row("PRAGMA journal_mode", [], |row| row.get(0))?;
    let quick_check: String =
        independent.query_row("PRAGMA quick_check", [], |row| row.get(0))?;
    assert_eq!(application_id, APPLICATION_ID);
    assert_eq!(journal_mode.to_ascii_lowercase(), "wal");
    assert_eq!(quick_check, "ok");
    drop(independent);

    let reopened = StateStore::open(temp.path())?;
    assert_eq!(reopened.store_id()?, store_id);
    assert_eq!(reopened.schema_version()?, schema_version);
    assert_eq!(reopened.journal_mode()?.to_ascii_lowercase(), "wal");
    assert!(reopened.foreign_keys_enabled()?);
    reopened.integrity_check()?;
    Ok(())
}
