//! Authenticated, explicitly plaintext snapshots for private Linux/GNU directories.
//! The SQLite writer boundary pins the entire copied inventory against cooperating GC.

use crate::{
    MAX_ARTIFACT_BYTES, StateError, StateStore,
    migrations::{MIGRATIONS, apply_migrations, validate_applied_migrations},
    snapshot_fs::{Directory, StagingDirectory, check_name},
};
use hmac::{Hmac, Mac};
use intent_contracts::{BoundedText, ContentHash, UnixTimestampMicros};
use rusqlite::{
    Connection, OpenFlags, TransactionBehavior,
    backup::{Backup, StepResult},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt,
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use uuid::Uuid;

const FORMAT_VERSION: u32 = 1;
const MAX_MANIFEST_BYTES: usize = 1024 * 1024;
const MAX_BLOBS: usize = 4096;
const MAX_DATABASE_BYTES: u64 = 64 * 1024 * 1024;
const MAX_TOTAL_BLOB_BYTES: u64 = 512 * 1024 * 1024;
const AUTHENTICATION_DOMAIN: &[u8] = b"IntentBrowser/authenticated-plaintext-snapshot/v1\0";

type HmacSha256 = Hmac<Sha256>;

/// A dedicated 256-bit key obtained independently of the bundle. No serialization or cloning.
pub struct BackupKey([u8; 32]);

impl BackupKey {
    pub fn generate() -> Result<Self, SnapshotError> {
        let mut bytes = [0_u8; 32];
        getrandom::fill(&mut bytes)
            .map_err(|_| SnapshotError::Invalid("OS randomness unavailable"))?;
        Ok(Self(bytes))
    }

    /// Import a dedicated, uniformly random key from the trusted application's key store.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    fn authenticator(&self) -> Result<HmacSha256, SnapshotError> {
        HmacSha256::new_from_slice(&self.0).map_err(|_| SnapshotError::Authentication)
    }
}

impl fmt::Debug for BackupKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BackupKey([REDACTED])")
    }
}

/// This API is not an encrypted backup. The trusted UI must obtain explicit export consent.
#[derive(Clone, Copy, Debug)]
pub enum PlaintextExportConsent {
    SensitiveDataWillBeWrittenUnencrypted,
}

#[derive(Clone, Copy, Debug)]
pub struct SnapshotLimits {
    database_bytes: u64,
    blob_bytes: u64,
    blobs: usize,
    duration: Duration,
}

impl Default for SnapshotLimits {
    fn default() -> Self {
        Self {
            database_bytes: MAX_DATABASE_BYTES,
            blob_bytes: MAX_TOTAL_BLOB_BYTES,
            blobs: MAX_BLOBS,
            duration: Duration::from_secs(30),
        }
    }
}

impl SnapshotLimits {
    pub fn try_new(
        database_bytes: u64,
        blob_bytes: u64,
        blobs: usize,
        duration: Duration,
    ) -> Result<Self, SnapshotError> {
        if database_bytes == 0
            || database_bytes > MAX_DATABASE_BYTES
            || blob_bytes > MAX_TOTAL_BLOB_BYTES
            || blobs > MAX_BLOBS
            || duration.is_zero()
            || duration > Duration::from_secs(30)
        {
            return Err(SnapshotError::Invalid(
                "snapshot limits exceed the supported hard budget",
            ));
        }
        Ok(Self {
            database_bytes,
            blob_bytes,
            blobs,
            duration,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotReceipt {
    pub path: PathBuf,
    pub snapshot_id: Uuid,
    pub store_id: Uuid,
    pub runtime_epoch: Uuid,
    pub schema_version: i64,
    pub journal_sequence: u64,
    pub blob_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BlobEntry {
    privacy_scope: BoundedText<128>,
    content_hash: ContentHash,
    byte_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    format_version: u32,
    snapshot_id: Uuid,
    store_id: Uuid,
    runtime_epoch: Uuid,
    schema_version: i64,
    journal_sequence: u64,
    created_at: UnixTimestampMicros,
    database_hash: ContentHash,
    database_bytes: u64,
    #[serde(deserialize_with = "deserialize_inventory")]
    blobs: Vec<BlobEntry>,
}

fn deserialize_inventory<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<BlobEntry>, D::Error> {
    struct InventoryVisitor;
    impl<'de> serde::de::Visitor<'de> for InventoryVisitor {
        type Value = Vec<BlobEntry>;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a bounded snapshot blob inventory")
        }
        fn visit_seq<A: serde::de::SeqAccess<'de>>(
            self,
            mut seq: A,
        ) -> Result<Self::Value, A::Error> {
            let mut result = Vec::new();
            while let Some(entry) = seq.next_element()? {
                if result.len() >= MAX_BLOBS {
                    return Err(serde::de::Error::custom(
                        "snapshot inventory exceeds entry budget",
                    ));
                }
                result.push(entry);
            }
            Ok(result)
        }
    }
    deserializer.deserialize_seq(InventoryVisitor)
}

struct Budget {
    started: Instant,
    limits: SnapshotLimits,
}

impl Budget {
    fn new(limits: SnapshotLimits) -> Self {
        Self {
            started: Instant::now(),
            limits,
        }
    }
    fn check(&self) -> Result<(), SnapshotError> {
        if self.started.elapsed() >= self.limits.duration {
            Err(SnapshotError::Budget("snapshot deadline exceeded"))
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SnapshotPhase {
    WriterLocked,
    DatabaseCopied,
    BlobCopied,
    BeforePublish,
    Published,
}

impl StateStore {
    /// Export an authenticated but UNENCRYPTED snapshot. Both artifact root and destination
    /// parent must already be private directories. Destination must not exist.
    ///
    /// Cooperating SQLite/artifact writers are blocked for the bounded copy phase. A successful
    /// result means all barriers completed; PublicationUncertain exposes a renamed destination
    /// whose parent-directory barrier failed, never permission to overwrite it.
    pub fn export_authenticated_plaintext_snapshot(
        &mut self,
        artifact_root: &Path,
        destination: &Path,
        key: &BackupKey,
        created_at: UnixTimestampMicros,
        limits: SnapshotLimits,
        _consent: PlaintextExportConsent,
    ) -> Result<SnapshotReceipt, SnapshotError> {
        self.export_snapshot_inner(
            artifact_root,
            destination,
            key,
            created_at,
            limits,
            &mut |_| Ok(()),
        )
    }

    pub(crate) fn export_snapshot_inner(
        &mut self,
        artifact_root: &Path,
        destination: &Path,
        key: &BackupKey,
        created_at: UnixTimestampMicros,
        limits: SnapshotLimits,
        progress: &mut impl FnMut(SnapshotPhase) -> io::Result<()>,
    ) -> Result<SnapshotReceipt, SnapshotError> {
        let budget = Budget::new(limits);
        let source_path = PathBuf::from(self.connection.path().filter(|p| !p.is_empty()).ok_or(
            SnapshotError::Invalid("snapshots require a file-backed private database"),
        )?);
        let source_parent = source_path
            .parent()
            .ok_or(SnapshotError::Invalid("database has no parent"))?;
        let source_directory = Directory::open_private(source_parent)?;
        let source_name = source_path
            .file_name()
            .and_then(|p| p.to_str())
            .ok_or(SnapshotError::Invalid("invalid database name"))?;
        let source_file = source_directory.open_file(source_name)?;
        if source_file.metadata()?.len() > limits.database_bytes {
            return Err(SnapshotError::Budget("source database size exceeds budget"));
        }
        let source_artifacts = Directory::open_private(artifact_root)?;
        let (parent, name) = destination_parts(destination)?;
        let mut staging = StagingDirectory::new(parent)?;
        let store_id = self.store_id()?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        progress(SnapshotPhase::WriterLocked)?;
        budget.check()?;
        let source = Connection::open_with_flags(
            &source_path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NOFOLLOW,
        )?;
        source.busy_timeout(Duration::from_millis(50))?;
        source.execute_batch("PRAGMA trusted_schema=OFF; PRAGMA query_only=ON;")?;
        if database_store_id(&source)? != store_id {
            return Err(SnapshotError::Invalid("source database identity changed"));
        }
        let pages = nonnegative(source.query_row("PRAGMA page_count", [], |r| r.get(0))?)?;
        let page_size = nonnegative(source.query_row("PRAGMA page_size", [], |r| r.get(0))?)?;
        if pages
            .checked_mul(page_size)
            .is_none_or(|size| size > limits.database_bytes)
        {
            return Err(SnapshotError::Budget(
                "source database page budget exceeded",
            ));
        }
        drop(staging.directory.create_file("state.sqlite3")?);
        let mut snapshot = Connection::open(staging.directory.sqlite_path())?;
        snapshot.pragma_update(None, "synchronous", "FULL")?;
        {
            let backup = Backup::new(&source, &mut snapshot)?;
            loop {
                budget.check()?;
                match backup.step(128)? {
                    StepResult::Done => break,
                    StepResult::More => {}
                    _ => {
                        return Err(SnapshotError::Invalid(
                            "snapshot copy could not obtain a required lock",
                        ));
                    }
                }
            }
        }
        validate_database(&snapshot)?;
        let runtime_epoch = disable_snapshot_dispatch(&snapshot)?;
        let schema_version = snapshot.query_row("PRAGMA user_version", [], |r| r.get(0))?;
        let journal_sequence = journal_sequence(&snapshot)?;
        let blobs = database_inventory(&snapshot, limits)?;
        snapshot.pragma_update(None, "journal_mode", "DELETE")?;
        snapshot.close().map_err(|(_, error)| error)?;
        let (database_hash, database_bytes) = hash_file(
            staging.directory.open_file("state.sqlite3")?,
            limits.database_bytes,
            &budget,
        )?;
        progress(SnapshotPhase::DatabaseCopied)?;
        copy_blobs(
            &source_artifacts,
            &staging.directory,
            &blobs,
            &budget,
            progress,
        )?;
        drop(source);
        transaction.rollback()?;
        let manifest = Manifest {
            format_version: FORMAT_VERSION,
            snapshot_id: Uuid::new_v4(),
            store_id,
            runtime_epoch,
            schema_version,
            journal_sequence,
            created_at,
            database_hash,
            database_bytes,
            blobs,
        };
        write_manifest(&staging.directory, key, &manifest)?;
        publish(&mut staging, name, destination, &budget, progress)?;
        Ok(receipt(destination, &manifest, runtime_epoch))
    }

    /// Verify before SQLite parsing, copy into private staging, and publish without replacing
    /// any profile. The restored database has a fresh epoch and durable dispatch prohibition.
    /// The original snapshot and every existing active profile remain untouched.
    pub fn restore_authenticated_plaintext_snapshot(
        source: &Path,
        destination: &Path,
        key: &BackupKey,
        limits: SnapshotLimits,
    ) -> Result<SnapshotReceipt, SnapshotError> {
        Self::restore_snapshot_inner(source, destination, key, limits, &mut |_| Ok(()))
    }

    pub(crate) fn restore_snapshot_inner(
        source: &Path,
        destination: &Path,
        key: &BackupKey,
        limits: SnapshotLimits,
        progress: &mut impl FnMut(SnapshotPhase) -> io::Result<()>,
    ) -> Result<SnapshotReceipt, SnapshotError> {
        let budget = Budget::new(limits);
        let source = Directory::open_private(source)?;
        let manifest = read_manifest(&source, key, limits)?;
        verify_directory_inventory(&source, &manifest.blobs)?;
        let (parent, name) = destination_parts(destination)?;
        let mut staging = StagingDirectory::new(parent)?;
        copy_verified_file(
            source.open_file("state.sqlite3")?,
            staging.directory.create_file("state.sqlite3")?,
            manifest.database_bytes,
            manifest.database_hash,
            limits.database_bytes,
            &budget,
        )?;
        progress(SnapshotPhase::DatabaseCopied)?;
        let snapshot = Connection::open(staging.directory.sqlite_path())?;
        snapshot.execute_batch(
            "PRAGMA trusted_schema=OFF; PRAGMA foreign_keys=ON; PRAGMA synchronous=FULL;",
        )?;
        validate_database(&snapshot)?;
        let schema_version: i64 = snapshot.query_row("PRAGMA user_version", [], |r| r.get(0))?;
        if schema_version != manifest.schema_version
            || database_store_id(&snapshot)? != manifest.store_id
            || journal_sequence(&snapshot)? != manifest.journal_sequence
            || database_inventory(&snapshot, limits)? != manifest.blobs
        {
            return Err(SnapshotError::Invalid(
                "manifest does not match complete database inventory",
            ));
        }
        let (epoch, enabled): (String, bool) = snapshot.query_row(
            "SELECT epoch, dispatch_enabled FROM runtime_control WHERE singleton=1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )?;
        if enabled || epoch != manifest.runtime_epoch.to_string() {
            return Err(SnapshotError::Invalid(
                "snapshot does not contain the declared dispatch barrier",
            ));
        }
        copy_blobs(
            &source,
            &staging.directory,
            &manifest.blobs,
            &budget,
            progress,
        )?;
        let runtime_epoch = disable_snapshot_dispatch(&snapshot)?;
        snapshot.pragma_update(None, "journal_mode", "DELETE")?;
        snapshot.close().map_err(|(_, error)| error)?;
        staging.directory.open_file("state.sqlite3")?.sync_all()?;
        publish(&mut staging, name, destination, &budget, progress)?;
        Ok(receipt(destination, &manifest, runtime_epoch))
    }
}

fn destination_parts(destination: &Path) -> Result<(Directory, &str), SnapshotError> {
    let parent = destination
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let name = destination
        .file_name()
        .and_then(|p| p.to_str())
        .ok_or(SnapshotError::Invalid("invalid destination name"))?;
    check_name(name)?;
    Ok((Directory::open_private(parent)?, name))
}

fn publish(
    staging: &mut StagingDirectory,
    name: &str,
    destination: &Path,
    budget: &Budget,
    progress: &mut impl FnMut(SnapshotPhase) -> io::Result<()>,
) -> Result<(), SnapshotError> {
    budget.check()?;
    progress(SnapshotPhase::BeforePublish)?;
    budget.check()?;
    staging.publish(name)?;
    progress(SnapshotPhase::Published)
        .and_then(|()| staging.sync_parent())
        .map_err(|source| SnapshotError::PublicationUncertain {
            path: destination.to_path_buf(),
            source,
        })
}

fn receipt(path: &Path, manifest: &Manifest, runtime_epoch: Uuid) -> SnapshotReceipt {
    SnapshotReceipt {
        path: path.to_path_buf(),
        snapshot_id: manifest.snapshot_id,
        store_id: manifest.store_id,
        runtime_epoch,
        schema_version: manifest.schema_version,
        journal_sequence: manifest.journal_sequence,
        blob_count: manifest.blobs.len(),
    }
}

fn disable_snapshot_dispatch(connection: &Connection) -> Result<Uuid, SnapshotError> {
    let epoch = Uuid::new_v4();
    let changed = connection.execute(
        "UPDATE runtime_control SET epoch=?1, dispatch_enabled=0, reason='restored snapshot: authority and startup recovery required' WHERE singleton=1",
        [epoch.to_string()],
    )?;
    if changed != 1 {
        return Err(SnapshotError::Invalid("missing runtime dispatch barrier"));
    }
    Ok(epoch)
}

fn nonnegative(value: i64) -> Result<u64, SnapshotError> {
    u64::try_from(value).map_err(|_| SnapshotError::Invalid("negative snapshot counter"))
}

fn database_store_id(connection: &Connection) -> Result<Uuid, SnapshotError> {
    let raw: String = connection.query_row(
        "SELECT store_uuid FROM store_metadata WHERE singleton=1",
        [],
        |r| r.get(0),
    )?;
    Uuid::parse_str(&raw).map_err(|_| SnapshotError::Invalid("invalid database store identity"))
}

fn journal_sequence(connection: &Connection) -> Result<u64, SnapshotError> {
    nonnegative(connection.query_row(
        "SELECT COALESCE(MAX(sequence), 0) FROM operation_journal",
        [],
        |r| r.get(0),
    )?)
}

fn schema_inventory(
    connection: &Connection,
) -> Result<Vec<(String, String, String, String)>, SnapshotError> {
    let mut statement = connection.prepare(
        "SELECT type,name,tbl_name,COALESCE(sql,'') FROM sqlite_schema WHERE name NOT GLOB 'sqlite_*' ORDER BY type,name",
    )?;
    let rows = statement.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?;
    Ok(rows.collect::<Result<_, _>>()?)
}

fn validate_database(connection: &Connection) -> Result<(), SnapshotError> {
    let application: i64 = connection.query_row("PRAGMA application_id", [], |r| r.get(0))?;
    let version: i64 = connection.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    if application != crate::APPLICATION_ID || Some(version) != MIGRATIONS.last().map(|m| m.version)
    {
        return Err(SnapshotError::Invalid(
            "unsupported snapshot database identity or schema",
        ));
    }
    validate_applied_migrations(connection)?;
    let mut expected = Connection::open_in_memory()?;
    apply_migrations(&mut expected)?;
    if schema_inventory(connection)? != schema_inventory(&expected)? {
        return Err(SnapshotError::Invalid(
            "snapshot schema differs from compiled schema",
        ));
    }
    let mut statement = connection.prepare("PRAGMA quick_check")?;
    let diagnostics = statement
        .query_map([], |r| r.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    if diagnostics != ["ok"] {
        return Err(SnapshotError::Invalid(
            "snapshot SQLite integrity check failed",
        ));
    }
    if connection
        .prepare("PRAGMA foreign_key_check")?
        .query([])?
        .next()?
        .is_some()
    {
        return Err(SnapshotError::Invalid(
            "snapshot contains broken foreign keys",
        ));
    }
    let bad_handles: bool = connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM artifact_handles_all h JOIN artifact_blobs b USING(privacy_scope,content_hash) WHERE h.byte_size != b.byte_size)",
        [], |r| r.get(0),
    )?;
    if bad_handles {
        return Err(SnapshotError::Invalid(
            "snapshot handle sizes disagree with blob inventory",
        ));
    }
    Ok(())
}

fn database_inventory(
    connection: &Connection,
    limits: SnapshotLimits,
) -> Result<Vec<BlobEntry>, SnapshotError> {
    let mut statement = connection.prepare(
        "SELECT privacy_scope, content_hash, byte_size FROM artifact_blobs ORDER BY privacy_scope, content_hash LIMIT ?1",
    )?;
    let mut rows = statement
        .query([i64::try_from(limits.blobs + 1)
            .map_err(|_| SnapshotError::Budget("inventory overflow"))?])?;
    let mut result = Vec::new();
    while let Some(row) = rows.next()? {
        if result.len() >= limits.blobs {
            return Err(SnapshotError::Budget("blob inventory exceeds entry budget"));
        }
        let scope: String = row.get(0)?;
        let hash: String = row.get(1)?;
        result.push(BlobEntry {
            privacy_scope: BoundedText::try_new(scope)
                .map_err(|_| SnapshotError::Invalid("invalid privacy scope"))?,
            content_hash: ContentHash::from_hex(&hash)
                .map_err(|_| SnapshotError::Invalid("invalid blob hash"))?,
            byte_size: nonnegative(row.get(2)?)?,
        });
    }
    validate_inventory(&result, limits)?;
    Ok(result)
}

fn validate_inventory(blobs: &[BlobEntry], limits: SnapshotLimits) -> Result<(), SnapshotError> {
    if blobs.len() > limits.blobs {
        return Err(SnapshotError::Budget("blob inventory exceeds entry budget"));
    }
    let mut previous = None;
    let mut total = 0_u64;
    for entry in blobs {
        if entry.privacy_scope.as_str().is_empty() || entry.byte_size > MAX_ARTIFACT_BYTES {
            return Err(SnapshotError::Invalid("invalid snapshot blob entry"));
        }
        let current = (entry.privacy_scope.as_str(), entry.content_hash.to_hex());
        if previous.as_ref().is_some_and(|last| last >= &current) {
            return Err(SnapshotError::Invalid(
                "duplicate or unordered snapshot inventory",
            ));
        }
        previous = Some(current);
        total = total
            .checked_add(entry.byte_size)
            .ok_or(SnapshotError::Budget("snapshot size overflow"))?;
        if total > limits.blob_bytes {
            return Err(SnapshotError::Budget(
                "snapshot aggregate blob budget exceeded",
            ));
        }
    }
    Ok(())
}

fn write_manifest(
    directory: &Directory,
    key: &BackupKey,
    manifest: &Manifest,
) -> Result<(), SnapshotError> {
    let bytes = serde_json::to_vec(manifest)?;
    if bytes.len() > MAX_MANIFEST_BYTES {
        return Err(SnapshotError::Budget("manifest byte budget exceeded"));
    }
    let mut mac = key.authenticator()?;
    mac.update(AUTHENTICATION_DOMAIN);
    mac.update(&bytes);
    let mut output = directory.create_file("manifest.json")?;
    output.write_all(&bytes)?;
    output.sync_all()?;
    let mut tag = directory.create_file("manifest.hmac")?;
    tag.write_all(&mac.finalize().into_bytes())?;
    tag.sync_all()?;
    directory.sync()?;
    Ok(())
}

fn read_manifest(
    directory: &Directory,
    key: &BackupKey,
    limits: SnapshotLimits,
) -> Result<Manifest, SnapshotError> {
    let bytes = read_bounded(directory.open_file("manifest.json")?, MAX_MANIFEST_BYTES)?;
    let tag = read_bounded(directory.open_file("manifest.hmac")?, 32)?;
    let mut mac = key.authenticator()?;
    mac.update(AUTHENTICATION_DOMAIN);
    mac.update(&bytes);
    mac.verify_slice(&tag)
        .map_err(|_| SnapshotError::Authentication)?;
    let manifest: Manifest = serde_json::from_slice(&bytes)?;
    if manifest.format_version != FORMAT_VERSION {
        return Err(SnapshotError::Invalid(
            "unsupported snapshot manifest version",
        ));
    }
    if manifest.database_bytes == 0 || manifest.database_bytes > limits.database_bytes {
        return Err(SnapshotError::Budget(
            "snapshot database byte budget exceeded",
        ));
    }
    validate_inventory(&manifest.blobs, limits)?;
    Ok(manifest)
}

fn read_bounded(file: File, maximum: usize) -> Result<Vec<u8>, SnapshotError> {
    if file.metadata()?.len() > maximum as u64 {
        return Err(SnapshotError::Budget(
            "snapshot metadata byte budget exceeded",
        ));
    }
    let mut bytes = Vec::new();
    file.take(maximum as u64 + 1).read_to_end(&mut bytes)?;
    if bytes.len() > maximum {
        return Err(SnapshotError::Budget(
            "snapshot metadata grew past byte budget",
        ));
    }
    Ok(bytes)
}

fn namespace(entry: &BlobEntry) -> String {
    ContentHash::from_bytes(Sha256::digest(entry.privacy_scope.as_str().as_bytes()).into()).to_hex()
}

fn verify_directory_inventory(
    directory: &Directory,
    entries: &[BlobEntry],
) -> Result<(), SnapshotError> {
    if directory.names(4)? != ["blobs", "manifest.hmac", "manifest.json", "state.sqlite3"] {
        return Err(SnapshotError::Invalid("unexpected snapshot root inventory"));
    }
    let mut expected: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for entry in entries {
        expected
            .entry(namespace(entry))
            .or_default()
            .insert(entry.content_hash.to_hex());
    }
    let blobs = directory.child("blobs")?;
    if blobs.names(MAX_BLOBS)? != expected.keys().cloned().collect::<Vec<_>>() {
        return Err(SnapshotError::Invalid(
            "unexpected privacy namespace inventory",
        ));
    }
    for (scope, files) in expected {
        if blobs.child(&scope)?.names(MAX_BLOBS)? != files.into_iter().collect::<Vec<_>>() {
            return Err(SnapshotError::Invalid("missing or extra snapshot blob"));
        }
    }
    Ok(())
}

fn copy_blobs(
    source: &Directory,
    destination: &Directory,
    entries: &[BlobEntry],
    budget: &Budget,
    progress: &mut impl FnMut(SnapshotPhase) -> io::Result<()>,
) -> Result<(), SnapshotError> {
    let output_blobs = destination.create_child("blobs")?;
    if entries.is_empty() {
        return Ok(());
    }
    let source_blobs = source.child("blobs")?;
    let mut created = BTreeSet::new();
    for entry in entries {
        budget.check()?;
        let scope = namespace(entry);
        let target_scope = if created.insert(scope.clone()) {
            output_blobs.create_child(&scope)?
        } else {
            output_blobs.child(&scope)?
        };
        let file_name = entry.content_hash.to_hex();
        copy_verified_file(
            source_blobs.child(&scope)?.open_file(&file_name)?,
            target_scope.create_file(&file_name)?,
            entry.byte_size,
            entry.content_hash,
            MAX_ARTIFACT_BYTES,
            budget,
        )?;
        target_scope.sync()?;
        progress(SnapshotPhase::BlobCopied)?;
    }
    output_blobs.sync()?;
    Ok(())
}

fn copy_verified_file(
    mut input: File,
    mut output: File,
    expected_size: u64,
    expected_hash: ContentHash,
    maximum: u64,
    budget: &Budget,
) -> Result<(), SnapshotError> {
    let (actual_hash, size) = stream_hash(&mut input, &mut output, maximum, budget)?;
    if size != expected_size || actual_hash != expected_hash {
        return Err(SnapshotError::Invalid(
            "snapshot file size or hash mismatch",
        ));
    }
    output.sync_all()?;
    Ok(())
}

fn hash_file(
    mut input: File,
    maximum: u64,
    budget: &Budget,
) -> Result<(ContentHash, u64), SnapshotError> {
    stream_hash(&mut input, &mut io::sink(), maximum, budget)
}

fn stream_hash(
    input: &mut File,
    output: &mut impl Write,
    maximum: u64,
    budget: &Budget,
) -> Result<(ContentHash, u64), SnapshotError> {
    if input.metadata()?.len() > maximum {
        return Err(SnapshotError::Budget(
            "snapshot file size exceeds byte budget",
        ));
    }
    let mut hasher = Sha256::new();
    let mut count = 0_u64;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        budget.check()?;
        let read = input.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        count = count
            .checked_add(read as u64)
            .ok_or(SnapshotError::Budget("snapshot byte counter overflow"))?;
        if count > maximum {
            return Err(SnapshotError::Budget("snapshot stream exceeds byte budget"));
        }
        hasher.update(&buffer[..read]);
        output.write_all(&buffer[..read])?;
    }
    Ok((ContentHash::from_bytes(hasher.finalize().into()), count))
}

#[derive(Debug)]
pub enum SnapshotError {
    Io(io::Error),
    Sqlite(rusqlite::Error),
    State(StateError),
    Json(serde_json::Error),
    Invalid(&'static str),
    Budget(&'static str),
    Authentication,
    PublicationUncertain { path: PathBuf, source: io::Error },
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "snapshot filesystem error: {error}"),
            Self::Sqlite(error) => write!(f, "snapshot database error: {error}"),
            Self::State(error) => write!(f, "snapshot state error: {error}"),
            Self::Json(error) => write!(f, "snapshot manifest error: {error}"),
            Self::Invalid(detail) | Self::Budget(detail) => f.write_str(detail),
            Self::Authentication => f.write_str("snapshot authentication failed"),
            Self::PublicationUncertain { path, source } => write!(
                f,
                "snapshot exists at {} but its publication durability is uncertain: {source}",
                path.display()
            ),
        }
    }
}

impl Error for SnapshotError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) | Self::PublicationUncertain { source: error, .. } => Some(error),
            Self::Sqlite(error) => Some(error),
            Self::State(error) => Some(error),
            Self::Json(error) => Some(error),
            _ => None,
        }
    }
}
impl From<io::Error> for SnapshotError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
impl From<rusqlite::Error> for SnapshotError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sqlite(error)
    }
}
impl From<StateError> for SnapshotError {
    fn from(error: StateError) -> Self {
        Self::State(error)
    }
}
impl From<serde_json::Error> for SnapshotError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

#[cfg(test)]
mod tests;
