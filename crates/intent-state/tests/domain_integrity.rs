use intent_state::StateStore;
use rusqlite::{Connection, params};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

struct TempDatabase {
    path: PathBuf,
}

impl TempDatabase {
    fn new() -> Self {
        Self {
            path: std::env::temp_dir().join(format!(
                "intent-state-domain-integrity-{}-{}.sqlite3",
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
            let _ = fs::remove_file(PathBuf::from(format!(
                "{}{}",
                self.path.display(),
                suffix
            )));
        }
    }
}

#[test]
fn persisted_schema_enforces_foreign_keys_and_domain_constraints() -> TestResult {
    let temp = TempDatabase::new();
    let store = StateStore::open(temp.path())?;
    drop(store);

    let connection = Connection::open(temp.path())?;
    connection.pragma_update(None, "foreign_keys", "ON")?;
    let foreign_keys: i64 = connection.query_row("PRAGMA foreign_keys", [], |row| row.get(0))?;
    assert_eq!(foreign_keys, 1);

    let orphan_outbox = connection.execute(
        "INSERT INTO outbox_messages(
            outbox_id, operation_id, attempt_identity, destination, message_kind,
            payload, payload_hash, state, lease_owner, lease_expires_at_micros,
            dispatch_started_at_micros, completed_at_micros, failure_detail,
            created_at_micros, updated_at_micros
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'pending', NULL, NULL, NULL, NULL, NULL, 1, 1)",
        params![
            "orphan-outbox",
            "missing-operation",
            "attempt-1",
            "fixture",
            "fixture",
            b"{}".as_slice(),
            "0".repeat(64),
        ],
    );
    assert!(
        orphan_outbox.is_err(),
        "actual outbox foreign key accepted a missing durable operation"
    );

    let invalid_state = connection.execute(
        "INSERT INTO durable_operations(
            operation_id, task_id, action_proposal_id, account_id, capability_id,
            arguments_hash, source_schema_major, source_schema_minor, state,
            state_detail, attempt_identity, revision, created_at_micros, updated_at_micros
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1, 0, 'not-a-state', NULL, NULL, 0, 1, 1)",
        params![
            "invalid-operation",
            "task-1",
            "proposal-1",
            "account-1",
            "capability-1",
            "1".repeat(64),
        ],
    );
    assert!(
        invalid_state.is_err(),
        "durable operation state domain accepted an unknown value"
    );

    connection.execute(
        "INSERT INTO durable_operations(
            operation_id, task_id, action_proposal_id, account_id, capability_id,
            arguments_hash, source_schema_major, source_schema_minor, state,
            state_detail, attempt_identity, revision, created_at_micros, updated_at_micros
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1, 0, 'prepared', NULL, NULL, 0, 1, 1)",
        params![
            "immutable-operation",
            "task-1",
            "proposal-1",
            "account-1",
            "capability-1",
            "2".repeat(64),
        ],
    )?;
    let identity_update = connection.execute(
        "UPDATE durable_operations SET task_id = 'task-2' WHERE operation_id = 'immutable-operation'",
        [],
    );
    assert!(
        identity_update.is_err(),
        "durable operation identity trigger allowed task reassignment"
    );
    let task_id: String = connection.query_row(
        "SELECT task_id FROM durable_operations WHERE operation_id = 'immutable-operation'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(task_id, "task-1");
    Ok(())
}
