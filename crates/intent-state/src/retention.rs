use crate::{ArtifactError, ArtifactScope, StateStore};
use intent_contracts::{ArtifactId, BoundedText, ContentHash, UnixTimestampMicros};
use rusqlite::{OptionalExtension, TransactionBehavior, params};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

pub const MAX_GC_BATCH: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SuppressionResult {
    Suppressed,
    AlreadySuppressed,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GcReport {
    pub deleted_blobs: usize,
    pub deferred_blobs: usize,
    pub failed_blobs: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactRetentionHold {
    pub hold_id: BoundedText<128>,
    pub artifact_id: ArtifactId,
    pub privacy_scope: ArtifactScope,
    pub reason: BoundedText<512>,
    pub expires_at: Option<UnixTimestampMicros>,
    pub created_at: UnixTimestampMicros,
}

impl StateStore {
    pub fn suppress_artifact(
        &mut self,
        artifact_id: ArtifactId,
        permitted_scope: &ArtifactScope,
        suppressed_at: UnixTimestampMicros,
    ) -> Result<SuppressionResult, RetentionError> {
        let handle = load_handle(&self.connection, artifact_id)?
            .ok_or(RetentionError::ArtifactNotFound(artifact_id))?;
        ensure_scope(artifact_id, permitted_scope, &handle.scope)?;

        if handle.suppressed_at.is_some() {
            enqueue_if_eligible(
                &mut self.connection,
                &handle.scope,
                &handle.content_hash,
                suppressed_at,
            )?;
            return Ok(SuppressionResult::AlreadySuppressed);
        }

        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let changed = transaction.execute(
            r#"
            UPDATE artifact_handles
            SET suppressed_at_micros = ?1
            WHERE artifact_id = ?2 AND suppressed_at_micros IS NULL
            "#,
            params![suppressed_at.get(), artifact_id.to_string()],
        )?;
        if changed != 1 {
            return Err(RetentionError::ConcurrentSuppression(artifact_id));
        }
        transaction.commit()?;

        enqueue_if_eligible(
            &mut self.connection,
            &handle.scope,
            &handle.content_hash,
            suppressed_at,
        )?;
        Ok(SuppressionResult::Suppressed)
    }

    pub fn remove_artifact_reference(
        &mut self,
        artifact_id: ArtifactId,
        permitted_scope: &ArtifactScope,
        reference_kind: &BoundedText<64>,
        reference_id: &BoundedText<256>,
        occurred_at: UnixTimestampMicros,
    ) -> Result<bool, RetentionError> {
        let handle = load_handle(&self.connection, artifact_id)?
            .ok_or(RetentionError::ArtifactNotFound(artifact_id))?;
        ensure_scope(artifact_id, permitted_scope, &handle.scope)?;

        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let changed = transaction.execute(
            r#"
            DELETE FROM artifact_references
            WHERE artifact_id = ?1 AND reference_kind = ?2 AND reference_id = ?3
            "#,
            params![
                artifact_id.to_string(),
                reference_kind.as_str(),
                reference_id.as_str(),
            ],
        )?;
        transaction.commit()?;

        if changed == 1 {
            enqueue_if_eligible(
                &mut self.connection,
                &handle.scope,
                &handle.content_hash,
                occurred_at,
            )?;
        }
        Ok(changed == 1)
    }

    pub fn add_artifact_retention_hold(
        &mut self,
        hold: ArtifactRetentionHold,
    ) -> Result<(), RetentionError> {
        let handle = load_handle(&self.connection, hold.artifact_id)?
            .ok_or(RetentionError::ArtifactNotFound(hold.artifact_id))?;
        ensure_scope(hold.artifact_id, &hold.privacy_scope, &handle.scope)?;
        if let Some(expires_at) = hold.expires_at
            && expires_at.get() <= hold.created_at.get()
        {
            return Err(RetentionError::InvalidHoldExpiry);
        }

        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        transaction.execute(
            r#"
            INSERT INTO artifact_retention_holds(
                hold_id, privacy_scope, content_hash, reason,
                expires_at_micros, created_at_micros
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                hold.hold_id.as_str(),
                handle.scope,
                handle.content_hash,
                hold.reason.as_str(),
                hold.expires_at.map(UnixTimestampMicros::get),
                hold.created_at.get(),
            ],
        )?;
        transaction.execute(
            "DELETE FROM artifact_gc_queue WHERE privacy_scope = ?1 AND content_hash = ?2",
            params![handle.scope, handle.content_hash],
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub fn release_artifact_retention_hold(
        &mut self,
        hold_id: &BoundedText<128>,
        permitted_scope: &ArtifactScope,
        occurred_at: UnixTimestampMicros,
    ) -> Result<bool, RetentionError> {
        let raw: Option<(String, String)> = self
            .connection
            .query_row(
                r#"
                SELECT privacy_scope, content_hash
                FROM artifact_retention_holds
                WHERE hold_id = ?1
                "#,
                [hold_id.as_str()],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        let Some((scope, content_hash)) = raw else {
            return Ok(false);
        };
        if scope != permitted_scope.as_str() {
            return Err(RetentionError::HoldAccessDenied);
        }

        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let changed = transaction.execute(
            "DELETE FROM artifact_retention_holds WHERE hold_id = ?1",
            [hold_id.as_str()],
        )?;
        transaction.commit()?;
        if changed == 1 {
            enqueue_if_eligible(&mut self.connection, &scope, &content_hash, occurred_at)?;
        }
        Ok(changed == 1)
    }

    pub fn enqueue_eligible_artifact_blobs(
        &mut self,
        now: UnixTimestampMicros,
        limit: usize,
    ) -> Result<usize, RetentionError> {
        prune_expired_holds(&mut self.connection, now)?;
        let limit = limit.min(MAX_GC_BATCH);
        if limit == 0 {
            return Ok(0);
        }

        let mut statement = self.connection.prepare(
            r#"
            SELECT DISTINCT privacy_scope, content_hash
            FROM artifact_handles
            WHERE suppressed_at_micros IS NOT NULL
            ORDER BY privacy_scope, content_hash
            LIMIT ?1
            "#,
        )?;
        let rows = statement.query_map([usize_to_i64(limit)?], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        let mut keys = Vec::new();
        for row in rows {
            keys.push(row?);
        }
        drop(statement);

        let mut enqueued = 0_usize;
        for (scope, hash) in keys {
            if enqueue_if_eligible(&mut self.connection, &scope, &hash, now)? {
                enqueued += 1;
            }
        }
        Ok(enqueued)
    }

    pub fn run_artifact_gc(
        &mut self,
        root: &Path,
        now: UnixTimestampMicros,
        limit: usize,
    ) -> Result<GcReport, RetentionError> {
        prune_expired_holds(&mut self.connection, now)?;
        let limit = limit.min(MAX_GC_BATCH);
        if limit == 0 {
            return Ok(GcReport::default());
        }

        let mut statement = self.connection.prepare(
            r#"
            SELECT privacy_scope, content_hash
            FROM artifact_gc_queue
            WHERE state IN ('pending', 'deleting', 'failed')
            ORDER BY enqueued_at_micros, privacy_scope, content_hash
            LIMIT ?1
            "#,
        )?;
        let rows = statement.query_map([usize_to_i64(limit)?], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        let mut keys = Vec::new();
        for row in rows {
            keys.push(row?);
        }
        drop(statement);

        let mut report = GcReport::default();
        for (scope, hash) in keys {
            if !blob_is_eligible(&self.connection, &scope, &hash, now)? {
                self.connection.execute(
                    "DELETE FROM artifact_gc_queue WHERE privacy_scope = ?1 AND content_hash = ?2",
                    params![scope, hash],
                )?;
                report.deferred_blobs += 1;
                continue;
            }

            let transaction = self
                .connection
                .transaction_with_behavior(TransactionBehavior::Immediate)?;
            transaction.execute(
                r#"
                UPDATE artifact_gc_queue
                SET state = 'deleting', attempt_count = attempt_count + 1,
                    updated_at_micros = ?1, last_error = NULL
                WHERE privacy_scope = ?2 AND content_hash = ?3
                "#,
                params![now.get(), scope, hash],
            )?;
            transaction.commit()?;

            let path = blob_path(root, &scope, &hash)?;
            match remove_blob_and_sync(&path) {
                Ok(()) => {}
                Err(error) => {
                    record_gc_failure(&mut self.connection, &scope, &hash, now, &error)?;
                    report.failed_blobs += 1;
                    continue;
                }
            }

            let transaction = self
                .connection
                .transaction_with_behavior(TransactionBehavior::Immediate)?;
            if !blob_is_eligible(&transaction, &scope, &hash, now)? {
                transaction.execute(
                    r#"
                    UPDATE artifact_gc_queue
                    SET state = 'failed', updated_at_micros = ?1,
                        last_error = 'eligibility changed after file deletion'
                    WHERE privacy_scope = ?2 AND content_hash = ?3
                    "#,
                    params![now.get(), scope, hash],
                )?;
                transaction.commit()?;
                report.failed_blobs += 1;
                continue;
            }
            transaction.execute(
                r#"
                DELETE FROM artifact_retention_holds
                WHERE privacy_scope = ?1 AND content_hash = ?2
                  AND expires_at_micros IS NOT NULL AND expires_at_micros <= ?3
                "#,
                params![scope, hash, now.get()],
            )?;
            transaction.execute(
                r#"
                DELETE FROM artifact_handles
                WHERE privacy_scope = ?1 AND content_hash = ?2
                  AND suppressed_at_micros IS NOT NULL
                "#,
                params![scope, hash],
            )?;
            transaction.execute(
                "DELETE FROM artifact_gc_queue WHERE privacy_scope = ?1 AND content_hash = ?2",
                params![scope, hash],
            )?;
            transaction.execute(
                "DELETE FROM artifact_blobs WHERE privacy_scope = ?1 AND content_hash = ?2",
                params![scope, hash],
            )?;
            transaction.commit()?;
            report.deleted_blobs += 1;
        }
        Ok(report)
    }
}

#[derive(Debug)]
struct HandleKey {
    scope: String,
    content_hash: String,
    suppressed_at: Option<i64>,
}

fn load_handle(
    connection: &rusqlite::Connection,
    artifact_id: ArtifactId,
) -> Result<Option<HandleKey>, RetentionError> {
    connection
        .query_row(
            r#"
            SELECT privacy_scope, content_hash, suppressed_at_micros
            FROM artifact_handles
            WHERE artifact_id = ?1
            "#,
            [artifact_id.to_string()],
            |row| {
                Ok(HandleKey {
                    scope: row.get(0)?,
                    content_hash: row.get(1)?,
                    suppressed_at: row.get(2)?,
                })
            },
        )
        .optional()
        .map_err(RetentionError::from)
}

fn ensure_scope(
    artifact_id: ArtifactId,
    permitted_scope: &ArtifactScope,
    actual_scope: &str,
) -> Result<(), RetentionError> {
    if actual_scope == permitted_scope.as_str() {
        Ok(())
    } else {
        Err(RetentionError::ArtifactAccessDenied(artifact_id))
    }
}

fn prune_expired_holds(
    connection: &mut rusqlite::Connection,
    now: UnixTimestampMicros,
) -> Result<(), RetentionError> {
    connection.execute(
        "DELETE FROM artifact_retention_holds WHERE expires_at_micros IS NOT NULL AND expires_at_micros <= ?1",
        [now.get()],
    )?;
    Ok(())
}

fn enqueue_if_eligible(
    connection: &mut rusqlite::Connection,
    scope: &str,
    content_hash: &str,
    now: UnixTimestampMicros,
) -> Result<bool, RetentionError> {
    if !blob_is_eligible(connection, scope, content_hash, now)? {
        return Ok(false);
    }
    let changed = connection.execute(
        r#"
        INSERT INTO artifact_gc_queue(
            privacy_scope, content_hash, state, attempt_count,
            enqueued_at_micros, updated_at_micros, last_error
        ) VALUES (?1, ?2, 'pending', 0, ?3, ?3, NULL)
        ON CONFLICT(privacy_scope, content_hash) DO NOTHING
        "#,
        params![scope, content_hash, now.get()],
    )?;
    Ok(changed == 1)
}

fn blob_is_eligible(
    connection: &rusqlite::Connection,
    scope: &str,
    content_hash: &str,
    now: UnixTimestampMicros,
) -> Result<bool, RetentionError> {
    let eligible: i64 = connection.query_row(
        r#"
        SELECT
            NOT EXISTS(
                SELECT 1 FROM artifact_handles
                WHERE privacy_scope = ?1 AND content_hash = ?2
                  AND suppressed_at_micros IS NULL
            )
            AND NOT EXISTS(
                SELECT 1
                FROM artifact_references AS refs
                JOIN artifact_handles AS handles
                  ON handles.artifact_id = refs.artifact_id
                WHERE handles.privacy_scope = ?1 AND handles.content_hash = ?2
            )
            AND NOT EXISTS(
                SELECT 1 FROM artifact_retention_holds
                WHERE privacy_scope = ?1 AND content_hash = ?2
                  AND (expires_at_micros IS NULL OR expires_at_micros > ?3)
            )
            AND EXISTS(
                SELECT 1 FROM artifact_handles
                WHERE privacy_scope = ?1 AND content_hash = ?2
            )
        "#,
        params![scope, content_hash, now.get()],
        |row| row.get(0),
    )?;
    Ok(eligible == 1)
}

fn blob_path(root: &Path, scope: &str, content_hash: &str) -> Result<PathBuf, RetentionError> {
    let hash = ContentHash::from_hex(content_hash).map_err(|error| {
        RetentionError::InvalidStoredRecord(format!("invalid blob hash: {error}"))
    })?;
    let namespace = scope_namespace(scope);
    Ok(root.join("blobs").join(namespace).join(hash.to_hex()))
}

#[must_use]
fn scope_namespace(scope: &str) -> String {
    let digest = Sha256::digest(scope.as_bytes());
    let mut bytes = [0_u8; 32];
    bytes.copy_from_slice(&digest);
    ContentHash::from_bytes(bytes).to_hex()
}

fn remove_blob_and_sync(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => {
            if let Some(parent) = path.parent() {
                sync_directory(parent)?;
            }
            Ok(())
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(unix)]
fn sync_directory(path: &Path) -> io::Result<()> {
    File::open(path)?.sync_all()
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> io::Result<()> {
    Ok(())
}

fn record_gc_failure(
    connection: &mut rusqlite::Connection,
    scope: &str,
    content_hash: &str,
    now: UnixTimestampMicros,
    error: &io::Error,
) -> Result<(), RetentionError> {
    let mut detail = error.to_string();
    if detail.len() > 2048 {
        detail.truncate(2048);
    }
    connection.execute(
        r#"
        UPDATE artifact_gc_queue
        SET state = 'failed', updated_at_micros = ?1, last_error = ?2
        WHERE privacy_scope = ?3 AND content_hash = ?4
        "#,
        params![now.get(), detail, scope, content_hash],
    )?;
    Ok(())
}

fn usize_to_i64(value: usize) -> Result<i64, RetentionError> {
    i64::try_from(value).map_err(|_| RetentionError::InvalidLimit)
}

#[derive(Debug)]
pub enum RetentionError {
    Sqlite(rusqlite::Error),
    Io(io::Error),
    Artifact(ArtifactError),
    ArtifactNotFound(ArtifactId),
    ArtifactAccessDenied(ArtifactId),
    ConcurrentSuppression(ArtifactId),
    HoldAccessDenied,
    InvalidHoldExpiry,
    InvalidLimit,
    InvalidStoredRecord(String),
}

impl fmt::Display for RetentionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => write!(formatter, "retention database error: {error}"),
            Self::Io(error) => write!(formatter, "retention filesystem error: {error}"),
            Self::Artifact(error) => write!(formatter, "artifact retention error: {error}"),
            Self::ArtifactNotFound(id) => write!(formatter, "artifact {id} does not exist"),
            Self::ArtifactAccessDenied(id) => {
                write!(
                    formatter,
                    "artifact {id} is outside the permitted retention scope"
                )
            }
            Self::ConcurrentSuppression(id) => {
                write!(formatter, "artifact {id} suppression changed concurrently")
            }
            Self::HoldAccessDenied => {
                formatter.write_str("retention hold is outside the permitted scope")
            }
            Self::InvalidHoldExpiry => {
                formatter.write_str("retention hold expiry must be after creation")
            }
            Self::InvalidLimit => {
                formatter.write_str("retention batch limit exceeds storage range")
            }
            Self::InvalidStoredRecord(detail) => {
                write!(formatter, "invalid stored retention record: {detail}")
            }
        }
    }
}

impl Error for RetentionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Sqlite(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::Artifact(error) => Some(error),
            _ => None,
        }
    }
}

impl From<rusqlite::Error> for RetentionError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

impl From<io::Error> for RetentionError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ArtifactError> for RetentionError {
    fn from(value: ArtifactError) -> Self {
        Self::Artifact(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{ArtifactRetentionHold, GcReport, SuppressionResult};
    use crate::{
        ArtifactError, ArtifactReferenceRegistration, ArtifactScope, NewArtifact, StateStore,
    };
    use intent_contracts::{ArtifactId, BoundedText, UnixTimestampMicros};
    use std::error::Error;
    use std::fs;
    use std::io::Cursor;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;
    use uuid::Uuid;

    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new() -> Result<Self, Box<dyn Error>> {
            let path = std::env::temp_dir().join(format!("intent-retention-{}", Uuid::new_v4()));
            fs::create_dir_all(&path)?;
            Ok(Self(path))
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn artifact_id() -> Result<ArtifactId, Box<dyn Error>> {
        Ok(ArtifactId::from_str(
            "018f47f7-5a86-7c00-8000-000000001006",
        )?)
    }

    fn scope() -> Result<ArtifactScope, Box<dyn Error>> {
        Ok(ArtifactScope::try_new("profile:alpha/private")?)
    }

    fn store_fixture(
        store: &mut StateStore,
        root: &Path,
        id: ArtifactId,
        scope: &ArtifactScope,
    ) -> Result<(), Box<dyn Error>> {
        let mut input = Cursor::new(b"retained-evidence".to_vec());
        store.store_artifact(
            root,
            NewArtifact {
                artifact_id: id,
                privacy_scope: scope.clone(),
                media_type: BoundedText::try_new("application/octet-stream")?,
                created_at: UnixTimestampMicros::try_new(10)?,
            },
            &mut input,
        )?;
        Ok(())
    }

    #[test]
    fn suppression_is_immediate_for_normal_retrieval() -> Result<(), Box<dyn Error>> {
        let root = TempRoot::new()?;
        let mut store = StateStore::open_in_memory_for_tests()?;
        let id = artifact_id()?;
        let scope = scope()?;
        store_fixture(&mut store, root.path(), id, &scope)?;
        assert_eq!(
            store.suppress_artifact(id, &scope, UnixTimestampMicros::try_new(20)?)?,
            SuppressionResult::Suppressed
        );
        let Err(error) = store.open_verified_artifact(root.path(), id, &scope) else {
            return Err("suppressed artifact remained readable".into());
        };
        assert!(matches!(error, ArtifactError::InvalidStoredRecord(_)));
        Ok(())
    }

    #[test]
    fn durable_reference_blocks_gc_until_removed() -> Result<(), Box<dyn Error>> {
        let root = TempRoot::new()?;
        let mut store = StateStore::open_in_memory_for_tests()?;
        let id = artifact_id()?;
        let scope = scope()?;
        store_fixture(&mut store, root.path(), id, &scope)?;
        let kind = BoundedText::try_new("receipt")?;
        let reference_id = BoundedText::try_new("order-1")?;
        store.register_artifact_reference(ArtifactReferenceRegistration {
            artifact_id: id,
            privacy_scope: scope.clone(),
            reference_kind: kind.clone(),
            reference_id: reference_id.clone(),
            created_at: UnixTimestampMicros::try_new(15)?,
        })?;
        store.suppress_artifact(id, &scope, UnixTimestampMicros::try_new(20)?)?;
        assert_eq!(
            store.run_artifact_gc(root.path(), UnixTimestampMicros::try_new(30)?, 8)?,
            GcReport::default()
        );
        assert!(store.remove_artifact_reference(
            id,
            &scope,
            &kind,
            &reference_id,
            UnixTimestampMicros::try_new(31)?,
        )?);
        let report = store.run_artifact_gc(root.path(), UnixTimestampMicros::try_new(32)?, 8)?;
        assert_eq!(report.deleted_blobs, 1);
        Ok(())
    }

    #[test]
    fn active_hold_blocks_gc_until_released() -> Result<(), Box<dyn Error>> {
        let root = TempRoot::new()?;
        let mut store = StateStore::open_in_memory_for_tests()?;
        let id = artifact_id()?;
        let scope = scope()?;
        store_fixture(&mut store, root.path(), id, &scope)?;
        let hold_id = BoundedText::try_new("legal-hold-1")?;
        store.add_artifact_retention_hold(ArtifactRetentionHold {
            hold_id: hold_id.clone(),
            artifact_id: id,
            privacy_scope: scope.clone(),
            reason: BoundedText::try_new("receipt retention")?,
            expires_at: None,
            created_at: UnixTimestampMicros::try_new(16)?,
        })?;
        store.suppress_artifact(id, &scope, UnixTimestampMicros::try_new(20)?)?;
        assert_eq!(
            store.run_artifact_gc(root.path(), UnixTimestampMicros::try_new(30)?, 8)?,
            GcReport::default()
        );
        assert!(store.release_artifact_retention_hold(
            &hold_id,
            &scope,
            UnixTimestampMicros::try_new(31)?,
        )?);
        let report = store.run_artifact_gc(root.path(), UnixTimestampMicros::try_new(32)?, 8)?;
        assert_eq!(report.deleted_blobs, 1);
        Ok(())
    }
}
