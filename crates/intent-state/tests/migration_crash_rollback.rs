use intent_state::StateStore;
use rusqlite::{Connection, TransactionBehavior, params};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};
use uuid::Uuid;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const DATABASE_ENV: &str = "INTENT_STATE_MIGRATION_CRASH_DATABASE";
const READY_ENV: &str = "INTENT_STATE_MIGRATION_CRASH_READY";
const INTERRUPTED_TABLE: &str = "interrupted_migration_marker";

struct TempDirectory {
    root: PathBuf,
}

impl TempDirectory {
    fn new() -> TestResult<Self> {
        let root = std::env::temp_dir().join(format!(
            "intent-state-migration-crash-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        fs::create_dir(&root)?;
        Ok(Self { root })
    }

    fn root(&self) -> &Path {
        &self.root
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

struct ChildGuard(Child);

impl ChildGuard {
    fn terminate(&mut self) -> TestResult {
        match self.0.try_wait()? {
            Some(status) => Err(format!(
                "migration child exited before forced termination: {status}"
            )
            .into()),
            None => {
                self.0.kill()?;
                let _ = self.0.wait()?;
                Ok(())
            }
        }
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if matches!(self.0.try_wait(), Ok(None)) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}

fn wait_until_ready(child: &mut ChildGuard, ready: &Path) -> TestResult {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if ready.exists() {
            return Ok(());
        }
        if let Some(status) = child.0.try_wait()? {
            return Err(format!("migration child exited before readiness: {status}").into());
        }
        if Instant::now() >= deadline {
            return Err("migration child did not become ready".into());
        }
        thread::sleep(Duration::from_millis(10));
    }
}

#[test]
#[ignore]
fn migration_crash_child() -> TestResult {
    let database = PathBuf::from(std::env::var_os(DATABASE_ENV).ok_or("missing database path")?);
    let ready = PathBuf::from(std::env::var_os(READY_ENV).ok_or("missing ready path")?);
    let mut connection = Connection::open(database)?;
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let baseline: i64 = transaction.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    let interrupted_version = baseline.checked_add(1).ok_or("migration version overflow")?;

    transaction.execute_batch(&format!(
        "CREATE TABLE {INTERRUPTED_TABLE} (value INTEGER NOT NULL) STRICT;"
    ))?;
    transaction.execute(
        "INSERT INTO schema_migrations(version, name, checksum, applied_unix_seconds) \
         VALUES (?1, ?2, ?3, unixepoch())",
        params![
            interrupted_version,
            "interrupted_crash_qualification",
            "0".repeat(64)
        ],
    )?;
    transaction.pragma_update(None, "user_version", interrupted_version)?;
    fs::write(ready, b"ready")?;

    thread::sleep(Duration::from_secs(60));
    drop(transaction);
    Ok(())
}

#[test]
fn process_crash_rolls_back_migration_shaped_schema_ledger_and_version_changes() -> TestResult {
    let temp = TempDirectory::new()?;
    let database = temp.root().join("state.sqlite3");
    let ready = temp.root().join("migration.ready");

    let baseline = StateStore::open(&database)?;
    let baseline_version = baseline.schema_version()?;
    let baseline_store_id = baseline.store_id()?;
    assert_eq!(baseline.journal_mode()?.to_ascii_lowercase(), "wal");
    drop(baseline);

    let mut child = ChildGuard(
        Command::new(std::env::current_exe()?)
            .args(["--exact", "migration_crash_child", "--ignored", "--nocapture"])
            .env(DATABASE_ENV, &database)
            .env(READY_ENV, &ready)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?,
    );
    wait_until_ready(&mut child, &ready)?;
    child.terminate()?;

    let connection = Connection::open(&database)?;
    let user_version: i64 = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    let ledger_version: i64 = connection.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
        [],
        |row| row.get(0),
    )?;
    let interrupted_tables: i64 = connection.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
        [INTERRUPTED_TABLE],
        |row| row.get(0),
    )?;
    assert_eq!(user_version, baseline_version);
    assert_eq!(ledger_version, baseline_version);
    assert_eq!(interrupted_tables, 0);
    drop(connection);

    let reopened = StateStore::open(&database)?;
    assert_eq!(reopened.schema_version()?, baseline_version);
    assert_eq!(reopened.store_id()?, baseline_store_id);
    reopened.integrity_check()?;
    Ok(())
}
