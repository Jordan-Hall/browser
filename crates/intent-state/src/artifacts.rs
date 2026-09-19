use crate::StateStore;
use intent_contracts::{ArtifactId, BoundedText, ContentHash, UnixTimestampMicros};
use rusqlite::{OptionalExtension, TransactionBehavior, params};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub const MAX_ARTIFACT_BYTES: u64 = 256 * 1024 * 1024;
const COPY_BUFFER_BYTES: usize = 64 * 1024;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ArtifactScope(BoundedText<128>);

impl ArtifactScope {
    pub fn try_new(value: impl Into<String>) -> Result<Self, ArtifactError> {
        let value = BoundedText::try_new(value).map_err(|error| {
            ArtifactError::InvalidInput(format!("invalid artifact scope: {error}"))
        })?;
        if value.as_str().is_empty() {
            return Err(ArtifactError::InvalidInput(
                "artifact scope must not be empty".to_owned(),
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewArtifact {
    pub artifact_id: ArtifactId,
    pub privacy_scope: ArtifactScope,
    pub media_type: BoundedText<255>,
    pub created_at: UnixTimestampMicros,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactMetadata {
    artifact_id: ArtifactId,
    privacy_scope: ArtifactScope,
    content_hash: ContentHash,
    byte_size: u64,
    media_type: BoundedText<255>,
    created_at: UnixTimestampMicros,
}

impl ArtifactMetadata {
    #[must_use]
    pub const fn artifact_id(&self) -> ArtifactId {
        self.artifact_id
    }

    #[must_use]
    pub fn privacy_scope(&self) -> &ArtifactScope {
        &self.privacy_scope
    }

    #[must_use]
    pub const fn content_hash(&self) -> ContentHash {
        self.content_hash
    }

    #[must_use]
    pub const fn byte_size(&self) -> u64 {
        self.byte_size
    }

    #[must_use]
    pub fn media_type(&self) -> &BoundedText<255> {
        &self.media_type
    }

    #[must_use]
    pub const fn created_at(&self) -> UnixTimestampMicros {
        self.created_at
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactReferenceRegistration {
    pub artifact_id: ArtifactId,
    pub privacy_scope: ArtifactScope,
    pub reference_kind: BoundedText<64>,
    pub reference_id: BoundedText<256>,
    pub created_at: UnixTimestampMicros,
}

#[derive(Debug)]
pub struct VerifiedArtifact {
    metadata: ArtifactMetadata,
    file: File,
}

impl VerifiedArtifact {
    #[must_use]
    pub const fn metadata(&self) -> &ArtifactMetadata {
        &self.metadata
    }

    #[must_use]
    pub fn into_file(self) -> File {
        self.file
    }
}

impl StateStore {
    pub fn store_artifact<R: Read>(
        &mut self,
        root: &Path,
        new: NewArtifact,
        reader: &mut R,
    ) -> Result<ArtifactMetadata, ArtifactError> {
        if new.media_type.as_str().is_empty() {
            return Err(ArtifactError::InvalidInput(
                "artifact media type must not be empty".to_owned(),
            ));
        }

        let quarantine_dir = root.join("quarantine");
        create_directory_durable(&quarantine_dir)?;
        let temp_path =
            quarantine_dir.join(format!("{}-{}.partial", new.artifact_id, Uuid::new_v4()));
        let mut cleanup = QuarantineCleanup::new(temp_path.clone());
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)?;

        let mut hasher = Sha256::new();
        let mut total = 0_u64;
        let mut buffer = [0_u8; COPY_BUFFER_BYTES];
        loop {
            let read = reader.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            total = total
                .checked_add(u64::try_from(read).map_err(|_| ArtifactError::SizeOverflow)?)
                .ok_or(ArtifactError::SizeOverflow)?;
            if total > MAX_ARTIFACT_BYTES {
                return Err(ArtifactError::ArtifactTooLarge {
                    size: total,
                    maximum: MAX_ARTIFACT_BYTES,
                });
            }
            hasher.update(&buffer[..read]);
            file.write_all(&buffer[..read])?;
        }
        file.sync_all()?;
        drop(file);

        let content_hash = finalize_hash(hasher);
        if let Some(existing) = self.load_artifact_metadata_unscoped(new.artifact_id)? {
            if existing.privacy_scope != new.privacy_scope
                || existing.media_type != new.media_type
                || existing.created_at != new.created_at
                || existing.content_hash != content_hash
                || existing.byte_size != total
            {
                return Err(ArtifactError::HandleConflict(new.artifact_id));
            }
            verify_blob(root, &existing)?;
            sync_directory(&scope_blob_dir(root, &existing.privacy_scope))?;
            cleanup.disarm_after_remove()?;
            return Ok(existing);
        }

        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let blobs_dir = root.join("blobs");
        create_directory_durable(&blobs_dir)?;
        let blob_dir = scope_blob_dir(root, &new.privacy_scope);
        create_directory_durable(&blob_dir)?;
        let target_path = blob_dir.join(content_hash.to_hex());

        match fs::hard_link(&temp_path, &target_path) {
            Ok(()) => {
                sync_directory(&blob_dir)?;
                cleanup.disarm_after_remove()?;
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                let expected = ArtifactMetadata {
                    artifact_id: new.artifact_id,
                    privacy_scope: new.privacy_scope.clone(),
                    content_hash,
                    byte_size: total,
                    media_type: new.media_type.clone(),
                    created_at: new.created_at,
                };
                verify_blob_at(&target_path, &expected)?;
                sync_directory(&blob_dir)?;
                cleanup.disarm_after_remove()?;
            }
            Err(error) => return Err(error.into()),
        }

        transaction.execute(
            r#"
            INSERT INTO artifact_blobs(privacy_scope, content_hash, byte_size, created_at_micros)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(privacy_scope, content_hash) DO NOTHING
            "#,
            params![
                new.privacy_scope.as_str(),
                content_hash.to_hex(),
                to_sql_i64(total, "artifact size")?,
                new.created_at.get(),
            ],
        )?;
        let registered_size: i64 = transaction.query_row(
            "SELECT byte_size FROM artifact_blobs WHERE privacy_scope = ?1 AND content_hash = ?2",
            params![new.privacy_scope.as_str(), content_hash.to_hex()],
            |row| row.get(0),
        )?;
        if nonnegative_u64(registered_size, "artifact blob size")? != total {
            return Err(ArtifactError::BlobMetadataConflict {
                scope: new.privacy_scope.as_str().to_owned(),
                hash: content_hash,
            });
        }
        transaction.execute(
            r#"
            INSERT INTO artifact_handles(
                artifact_id, privacy_scope, content_hash, byte_size, media_type, created_at_micros
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                new.artifact_id.to_string(),
                new.privacy_scope.as_str(),
                content_hash.to_hex(),
                to_sql_i64(total, "artifact size")?,
                new.media_type.as_str(),
                new.created_at.get(),
            ],
        )?;
        transaction.commit()?;

        Ok(ArtifactMetadata {
            artifact_id: new.artifact_id,
            privacy_scope: new.privacy_scope,
            content_hash,
            byte_size: total,
            media_type: new.media_type,
            created_at: new.created_at,
        })
    }

    pub fn artifact_metadata(
        &self,
        artifact_id: ArtifactId,
        permitted_scope: &ArtifactScope,
    ) -> Result<ArtifactMetadata, ArtifactError> {
        let metadata = self
            .load_artifact_metadata_unscoped(artifact_id)?
            .ok_or(ArtifactError::ArtifactNotFound(artifact_id))?;
        if metadata.privacy_scope != *permitted_scope {
            return Err(ArtifactError::AccessDenied(artifact_id));
        }
        Ok(metadata)
    }

    pub fn open_verified_artifact(
        &self,
        root: &Path,
        artifact_id: ArtifactId,
        permitted_scope: &ArtifactScope,
    ) -> Result<VerifiedArtifact, ArtifactError> {
        let metadata = self.artifact_metadata(artifact_id, permitted_scope)?;
        let mut file = verify_blob(root, &metadata)?;
        file.seek(SeekFrom::Start(0))?;
        Ok(VerifiedArtifact { metadata, file })
    }

    pub fn register_artifact_reference(
        &mut self,
        reference: ArtifactReferenceRegistration,
    ) -> Result<(), ArtifactError> {
        if reference.reference_kind.as_str() == "runtime_checkpoint" {
            return Err(ArtifactError::InvalidInput(
                "checkpoint references are owned by the checkpoint transaction".to_owned(),
            ));
        }
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let handle: Option<(String, Option<i64>)> = transaction.query_row(
            "SELECT privacy_scope, suppressed_at_micros FROM artifact_handles_all WHERE artifact_id = ?1",
            [reference.artifact_id.to_string()],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).optional()?;
        let (scope, suppressed_at) =
            handle.ok_or(ArtifactError::ArtifactNotFound(reference.artifact_id))?;
        if scope != reference.privacy_scope.as_str() {
            return Err(ArtifactError::AccessDenied(reference.artifact_id));
        }
        if suppressed_at.is_some() {
            return Err(ArtifactError::ArtifactNotFound(reference.artifact_id));
        }
        transaction.execute(
            r#"
            INSERT INTO artifact_references(
                artifact_id, reference_kind, reference_id, created_at_micros
            ) VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(artifact_id, reference_kind, reference_id) DO NOTHING
            "#,
            params![
                reference.artifact_id.to_string(),
                reference.reference_kind.as_str(),
                reference.reference_id.as_str(),
                reference.created_at.get(),
            ],
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub fn artifact_reference_count(
        &self,
        artifact_id: ArtifactId,
        permitted_scope: &ArtifactScope,
    ) -> Result<u64, ArtifactError> {
        self.artifact_metadata(artifact_id, permitted_scope)?;
        let count: i64 = self.connection.query_row(
            "SELECT COUNT(*) FROM artifact_references WHERE artifact_id = ?1",
            [artifact_id.to_string()],
            |row| row.get(0),
        )?;
        nonnegative_u64(count, "artifact reference count")
    }

    fn load_artifact_metadata_unscoped(
        &self,
        artifact_id: ArtifactId,
    ) -> Result<Option<ArtifactMetadata>, ArtifactError> {
        load_artifact_metadata(&self.connection, artifact_id)
    }
}

pub(crate) fn load_artifact_metadata(
    connection: &rusqlite::Connection,
    artifact_id: ArtifactId,
) -> Result<Option<ArtifactMetadata>, ArtifactError> {
    let raw = connection
        .query_row(
            r#"
                SELECT privacy_scope, content_hash, byte_size, media_type, created_at_micros,
                       suppressed_at_micros
                FROM artifact_handles
                WHERE artifact_id = ?1
                "#,
            [artifact_id.to_string()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, Option<i64>>(5)?,
                ))
            },
        )
        .optional()?;
    let Some((scope, hash, byte_size, media_type, created_at, suppressed_at)) = raw else {
        return Ok(None);
    };
    if suppressed_at.is_some() {
        return Err(ArtifactError::ArtifactNotFound(artifact_id));
    }
    Ok(Some(ArtifactMetadata {
        artifact_id,
        privacy_scope: ArtifactScope::try_new(scope)?,
        content_hash: ContentHash::from_hex(&hash).map_err(|error| {
            ArtifactError::InvalidStoredRecord(format!("invalid artifact hash: {error}"))
        })?,
        byte_size: nonnegative_u64(byte_size, "artifact byte size")?,
        media_type: BoundedText::try_new(media_type).map_err(|error| {
            ArtifactError::InvalidStoredRecord(format!("invalid artifact media type: {error}"))
        })?,
        created_at: UnixTimestampMicros::try_new(created_at).map_err(|error| {
            ArtifactError::InvalidStoredRecord(format!("invalid artifact timestamp: {error}"))
        })?,
    }))
}

pub(crate) fn verify_blob(root: &Path, metadata: &ArtifactMetadata) -> Result<File, ArtifactError> {
    let path = artifact_blob_path(root, metadata);
    verify_blob_at(&path, metadata)
}

fn verify_blob_at(path: &Path, metadata: &ArtifactMetadata) -> Result<File, ArtifactError> {
    let file_type = fs::symlink_metadata(path)
        .map_err(|error| map_missing_blob(error, metadata.artifact_id))?
        .file_type();
    if !file_type.is_file() || file_type.is_symlink() {
        return Err(ArtifactError::InvalidBlobType(metadata.artifact_id));
    }
    let mut file =
        File::open(path).map_err(|error| map_missing_blob(error, metadata.artifact_id))?;
    let mut hasher = Sha256::new();
    let mut total = 0_u64;
    let mut buffer = [0_u8; COPY_BUFFER_BYTES];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        total = total
            .checked_add(u64::try_from(read).map_err(|_| ArtifactError::SizeOverflow)?)
            .ok_or(ArtifactError::SizeOverflow)?;
        if total > MAX_ARTIFACT_BYTES {
            return Err(ArtifactError::ArtifactTooLarge {
                size: total,
                maximum: MAX_ARTIFACT_BYTES,
            });
        }
        hasher.update(&buffer[..read]);
    }
    let actual_hash = finalize_hash(hasher);
    if total != metadata.byte_size || actual_hash != metadata.content_hash {
        return Err(ArtifactError::BlobIntegrityMismatch {
            artifact_id: metadata.artifact_id,
            expected_hash: metadata.content_hash,
            actual_hash,
            expected_size: metadata.byte_size,
            actual_size: total,
        });
    }
    Ok(file)
}

fn map_missing_blob(error: io::Error, artifact_id: ArtifactId) -> ArtifactError {
    if error.kind() == io::ErrorKind::NotFound {
        ArtifactError::MissingBlob(artifact_id)
    } else {
        ArtifactError::Io(error)
    }
}

pub(crate) fn artifact_blob_path(root: &Path, metadata: &ArtifactMetadata) -> PathBuf {
    scope_blob_dir(root, &metadata.privacy_scope).join(metadata.content_hash.to_hex())
}

fn scope_blob_dir(root: &Path, scope: &ArtifactScope) -> PathBuf {
    root.join("blobs").join(scope_namespace(scope))
}

fn create_directory_durable(path: &Path) -> Result<(), ArtifactError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
                return Err(ArtifactError::InvalidStorageDirectory(path.to_path_buf()));
            }
            sync_directory(path)?;
            if let Some(parent) = path
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
            {
                sync_directory(parent)?;
            }
            return Ok(());
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }

    let parent = path.parent().ok_or_else(|| {
        ArtifactError::InvalidInput(format!(
            "storage directory {} has no parent",
            path.display()
        ))
    })?;
    create_directory_durable(parent)?;
    match fs::create_dir(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            let metadata = fs::symlink_metadata(path)?;
            if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
                return Err(ArtifactError::InvalidStorageDirectory(path.to_path_buf()));
            }
        }
        Err(error) => return Err(error.into()),
    }
    sync_directory(path)?;
    sync_directory(parent)?;
    Ok(())
}

#[must_use]
fn scope_namespace(scope: &ArtifactScope) -> String {
    hash_bytes(scope.as_str().as_bytes()).to_hex()
}

#[must_use]
fn hash_bytes(bytes: &[u8]) -> ContentHash {
    let digest = Sha256::digest(bytes);
    let mut hash = [0_u8; 32];
    hash.copy_from_slice(&digest);
    ContentHash::from_bytes(hash)
}

#[must_use]
fn finalize_hash(hasher: Sha256) -> ContentHash {
    let digest = hasher.finalize();
    let mut hash = [0_u8; 32];
    hash.copy_from_slice(&digest);
    ContentHash::from_bytes(hash)
}

fn sync_directory(path: &Path) -> Result<(), ArtifactError> {
    crate::directory_sync::sync_directory(path)?;
    Ok(())
}

fn nonnegative_u64(value: i64, label: &'static str) -> Result<u64, ArtifactError> {
    u64::try_from(value)
        .map_err(|_| ArtifactError::InvalidStoredRecord(format!("negative {label}")))
}

fn to_sql_i64(value: u64, label: &'static str) -> Result<i64, ArtifactError> {
    i64::try_from(value).map_err(|_| ArtifactError::InvalidInput(format!("{label} too large")))
}

struct QuarantineCleanup {
    path: PathBuf,
    armed: bool,
}

impl QuarantineCleanup {
    fn new(path: PathBuf) -> Self {
        Self { path, armed: true }
    }

    fn disarm_after_remove(&mut self) -> Result<(), ArtifactError> {
        fs::remove_file(&self.path)?;
        if let Some(parent) = self.path.parent() {
            sync_directory(parent)?;
        }
        self.armed = false;
        Ok(())
    }
}

impl Drop for QuarantineCleanup {
    fn drop(&mut self) {
        if self.armed {
            let _ = fs::remove_file(&self.path);
        }
    }
}

#[derive(Debug)]
pub enum ArtifactError {
    Io(io::Error),
    Sqlite(rusqlite::Error),
    InvalidInput(String),
    InvalidStoredRecord(String),
    ArtifactTooLarge {
        size: u64,
        maximum: u64,
    },
    SizeOverflow,
    ArtifactNotFound(ArtifactId),
    MissingBlob(ArtifactId),
    AccessDenied(ArtifactId),
    InvalidBlobType(ArtifactId),
    InvalidStorageDirectory(PathBuf),
    HandleConflict(ArtifactId),
    BlobMetadataConflict {
        scope: String,
        hash: ContentHash,
    },
    BlobIntegrityMismatch {
        artifact_id: ArtifactId,
        expected_hash: ContentHash,
        actual_hash: ContentHash,
        expected_size: u64,
        actual_size: u64,
    },
}

impl fmt::Display for ArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "artifact filesystem error: {error}"),
            Self::Sqlite(error) => write!(formatter, "artifact metadata database error: {error}"),
            Self::InvalidInput(detail) => write!(formatter, "invalid artifact input: {detail}"),
            Self::InvalidStoredRecord(detail) => {
                write!(formatter, "invalid stored artifact record: {detail}")
            }
            Self::ArtifactTooLarge { size, maximum } => {
                write!(formatter, "artifact is {size} bytes; maximum is {maximum}")
            }
            Self::SizeOverflow => formatter.write_str("artifact size counter overflowed"),
            Self::ArtifactNotFound(id) => write!(formatter, "artifact {id} does not exist"),
            Self::MissingBlob(id) => write!(formatter, "artifact {id} blob is missing"),
            Self::AccessDenied(id) => {
                write!(formatter, "artifact {id} is outside the permitted scope")
            }
            Self::InvalidBlobType(id) => {
                write!(formatter, "artifact {id} blob is not a regular file")
            }
            Self::InvalidStorageDirectory(path) => write!(
                formatter,
                "artifact storage path {} is not a regular directory",
                path.display()
            ),
            Self::HandleConflict(id) => write!(
                formatter,
                "artifact {id} handle conflicts with existing metadata or retry bytes"
            ),
            Self::BlobMetadataConflict { scope, hash } => write!(
                formatter,
                "artifact blob metadata conflicts for scope {scope} and hash {hash}"
            ),
            Self::BlobIntegrityMismatch {
                artifact_id,
                expected_hash,
                actual_hash,
                expected_size,
                actual_size,
            } => write!(
                formatter,
                "artifact {artifact_id} integrity mismatch: expected {expected_hash}/{expected_size}, actual {actual_hash}/{actual_size}"
            ),
        }
    }
}

impl Error for ArtifactError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Sqlite(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for ArtifactError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<rusqlite::Error> for ArtifactError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ArtifactError, ArtifactReferenceRegistration, ArtifactScope, MAX_ARTIFACT_BYTES,
        NewArtifact, artifact_blob_path,
    };
    use crate::StateStore;
    use intent_contracts::{ArtifactId, BoundedText, UnixTimestampMicros};
    use std::error::Error;
    use std::fs;
    use std::io::{Cursor, Read};
    use std::path::{Path, PathBuf};
    use std::str::FromStr;
    use uuid::Uuid;

    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new() -> Result<Self, Box<dyn Error>> {
            let path = std::env::temp_dir().join(format!("intent-artifacts-{}", Uuid::new_v4()));
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

    fn artifact_id(value: u16) -> Result<ArtifactId, Box<dyn Error>> {
        let raw = format!("018f47f7-5a86-7c00-8000-00000000{value:04x}");
        Ok(ArtifactId::from_str(&raw)?)
    }

    fn new_artifact(id: ArtifactId, scope: &str) -> Result<NewArtifact, Box<dyn Error>> {
        Ok(NewArtifact {
            artifact_id: id,
            privacy_scope: ArtifactScope::try_new(scope)?,
            media_type: BoundedText::try_new("application/octet-stream")?,
            created_at: UnixTimestampMicros::try_new(100)?,
        })
    }

    #[test]
    fn stores_verifies_and_references_artifact() -> Result<(), Box<dyn Error>> {
        let root = TempRoot::new()?;
        let mut store = StateStore::open_in_memory_for_tests()?;
        let id = artifact_id(1)?;
        let scope = ArtifactScope::try_new("profile:alpha/private")?;
        let mut input = Cursor::new(b"evidence-bytes".to_vec());
        let metadata =
            store.store_artifact(root.path(), new_artifact(id, scope.as_str())?, &mut input)?;
        assert_eq!(metadata.byte_size(), 14);

        store.register_artifact_reference(ArtifactReferenceRegistration {
            artifact_id: id,
            privacy_scope: scope.clone(),
            reference_kind: BoundedText::try_new("evidence")?,
            reference_id: BoundedText::try_new("obs-1")?,
            created_at: UnixTimestampMicros::try_new(101)?,
        })?;
        assert_eq!(store.artifact_reference_count(id, &scope)?, 1);

        let verified = store.open_verified_artifact(root.path(), id, &scope)?;
        let mut bytes = Vec::new();
        verified.into_file().read_to_end(&mut bytes)?;
        assert_eq!(bytes, b"evidence-bytes");
        Ok(())
    }

    #[test]
    fn idempotent_retry_compares_incoming_bytes() -> Result<(), Box<dyn Error>> {
        let root = TempRoot::new()?;
        let mut store = StateStore::open_in_memory_for_tests()?;
        let id = artifact_id(6)?;
        let scope = ArtifactScope::try_new("profile:alpha/private")?;
        let mut first = Cursor::new(b"original".to_vec());
        store.store_artifact(root.path(), new_artifact(id, scope.as_str())?, &mut first)?;
        let mut changed = Cursor::new(b"changed".to_vec());
        let Err(error) =
            store.store_artifact(root.path(), new_artifact(id, scope.as_str())?, &mut changed)
        else {
            return Err("changed retry bytes unexpectedly reused the old handle".into());
        };
        assert!(matches!(error, ArtifactError::HandleConflict(found) if found == id));
        Ok(())
    }

    #[test]
    fn reference_count_requires_matching_scope() -> Result<(), Box<dyn Error>> {
        let root = TempRoot::new()?;
        let mut store = StateStore::open_in_memory_for_tests()?;
        let id = artifact_id(7)?;
        let scope = ArtifactScope::try_new("profile:alpha/private")?;
        let wrong_scope = ArtifactScope::try_new("profile:alpha/public")?;
        let mut input = Cursor::new(b"scoped".to_vec());
        store.store_artifact(root.path(), new_artifact(id, scope.as_str())?, &mut input)?;
        store.register_artifact_reference(ArtifactReferenceRegistration {
            artifact_id: id,
            privacy_scope: scope.clone(),
            reference_kind: BoundedText::try_new("evidence")?,
            reference_id: BoundedText::try_new("obs-7")?,
            created_at: UnixTimestampMicros::try_new(101)?,
        })?;
        let Err(error) = store.artifact_reference_count(id, &wrong_scope) else {
            return Err("cross-scope reference count unexpectedly succeeded".into());
        };
        assert!(matches!(error, ArtifactError::AccessDenied(found) if found == id));
        Ok(())
    }

    #[test]
    fn identical_plaintext_is_namespaced_by_scope() -> Result<(), Box<dyn Error>> {
        let root = TempRoot::new()?;
        let mut store = StateStore::open_in_memory_for_tests()?;
        let scope_a = ArtifactScope::try_new("profile:alpha/private")?;
        let scope_b = ArtifactScope::try_new("profile:alpha/public")?;
        let id_a = artifact_id(2)?;
        let id_b = artifact_id(3)?;
        let mut first = Cursor::new(b"same".to_vec());
        let mut second = Cursor::new(b"same".to_vec());
        let meta_a = store.store_artifact(
            root.path(),
            new_artifact(id_a, scope_a.as_str())?,
            &mut first,
        )?;
        let meta_b = store.store_artifact(
            root.path(),
            new_artifact(id_b, scope_b.as_str())?,
            &mut second,
        )?;
        assert_eq!(meta_a.content_hash(), meta_b.content_hash());
        assert_ne!(
            artifact_blob_path(root.path(), &meta_a),
            artifact_blob_path(root.path(), &meta_b)
        );
        let Err(error) = store.open_verified_artifact(root.path(), id_a, &scope_b) else {
            return Err("cross-scope artifact access unexpectedly succeeded".into());
        };
        assert!(matches!(error, ArtifactError::AccessDenied(found) if found == id_a));
        Ok(())
    }

    #[test]
    fn corrupt_blob_fails_visible_integrity_check() -> Result<(), Box<dyn Error>> {
        let root = TempRoot::new()?;
        let mut store = StateStore::open_in_memory_for_tests()?;
        let id = artifact_id(4)?;
        let scope = ArtifactScope::try_new("profile:alpha/private")?;
        let mut input = Cursor::new(b"original".to_vec());
        let metadata =
            store.store_artifact(root.path(), new_artifact(id, scope.as_str())?, &mut input)?;
        fs::write(artifact_blob_path(root.path(), &metadata), b"corrupt")?;
        let Err(error) = store.open_verified_artifact(root.path(), id, &scope) else {
            return Err("corrupt artifact unexpectedly verified".into());
        };
        assert!(matches!(error, ArtifactError::BlobIntegrityMismatch { .. }));
        Ok(())
    }

    #[test]
    fn oversized_input_is_rejected_and_quarantine_is_cleaned() -> Result<(), Box<dyn Error>> {
        let root = TempRoot::new()?;
        let mut store = StateStore::open_in_memory_for_tests()?;
        let id = artifact_id(5)?;
        let mut input = std::io::repeat(0).take(MAX_ARTIFACT_BYTES + 1);
        let Err(error) = store.store_artifact(
            root.path(),
            new_artifact(id, "profile:alpha/private")?,
            &mut input,
        ) else {
            return Err("oversized artifact unexpectedly stored".into());
        };
        assert!(matches!(error, ArtifactError::ArtifactTooLarge { .. }));
        let quarantine = root.path().join("quarantine");
        assert_eq!(fs::read_dir(quarantine)?.count(), 0);
        Ok(())
    }
    #[test]
    fn invalid_reference_never_reports_a_successful_pin() -> Result<(), Box<dyn Error>> {
        let root = TempRoot::new()?;
        let mut store = StateStore::open_in_memory_for_tests()?;
        let id = artifact_id(20)?;
        let scope = ArtifactScope::try_new("profile:test/private")?;
        store.store_artifact(
            root.path(),
            new_artifact(id, scope.as_str())?,
            &mut Cursor::new(b"keep"),
        )?;
        for (kind, reference) in [("", "ref"), ("evidence", "")] {
            assert!(
                store
                    .register_artifact_reference(ArtifactReferenceRegistration {
                        artifact_id: id,
                        privacy_scope: scope.clone(),
                        reference_kind: BoundedText::try_new(kind)?,
                        reference_id: BoundedText::try_new(reference)?,
                        created_at: UnixTimestampMicros::try_new(101)?,
                    })
                    .is_err()
            );
            assert_eq!(store.artifact_reference_count(id, &scope)?, 0);
        }
        let reference = ArtifactReferenceRegistration {
            artifact_id: id,
            privacy_scope: scope.clone(),
            reference_kind: BoundedText::try_new("evidence")?,
            reference_id: BoundedText::try_new("ref")?,
            created_at: UnixTimestampMicros::try_new(101)?,
        };
        store.register_artifact_reference(reference.clone())?;
        store.register_artifact_reference(reference)?;
        assert_eq!(store.artifact_reference_count(id, &scope)?, 1);
        Ok(())
    }
}
