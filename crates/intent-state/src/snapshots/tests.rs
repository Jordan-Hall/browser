use super::*;
use crate::{test_support::*, *};
use intent_contracts::BoundedText;
use std::{
    fs,
    os::unix::{
        fs::{PermissionsExt, symlink},
        process::ExitStatusExt,
    },
    process::{Child, Command, Stdio},
    thread,
};

type TestResult<T = ()> = crate::test_support::TestResult<T>;

struct Fixture {
    root: PathBuf,
    artifacts: PathBuf,
    database: PathBuf,
}

impl Fixture {
    fn new() -> TestResult<Self> {
        let root = std::env::temp_dir().join(format!("intent-snapshot-{}", Uuid::new_v4()));
        fs::create_dir(&root)?;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
        let artifacts = root.join("artifacts");
        fs::create_dir(&artifacts)?;
        fs::set_permissions(&artifacts, fs::Permissions::from_mode(0o700))?;
        let database = root.join("state.sqlite3");
        Ok(Self {
            root,
            artifacts,
            database,
        })
    }
    fn snapshot(&self) -> PathBuf {
        self.root.join("backup")
    }
    fn restored(&self) -> PathBuf {
        self.root.join("restored")
    }
    fn key(&self) -> BackupKey {
        BackupKey::from_bytes([42; 32])
    }
    fn populated(&self) -> TestResult<(StateStore, ArtifactMetadata)> {
        let mut store = StateStore::open(&self.database)?;
        let scope = ArtifactScope::try_new("private-account")?;
        let stored = artifact(&mut store, &self.artifacts, 900, &scope)?;
        Ok((store, stored))
    }
    fn export(&self, store: &mut StateStore) -> Result<SnapshotReceipt, SnapshotError> {
        store.export_authenticated_plaintext_snapshot(
            &self.artifacts,
            &self.snapshot(),
            &self.key(),
            UnixTimestampMicros::try_new(1000)
                .map_err(|_| SnapshotError::Invalid("invalid fixture time"))?,
            SnapshotLimits::default(),
            PlaintextExportConsent::SensitiveDataWillBeWrittenUnencrypted,
        )
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn snapshot_roundtrip_preserves_suppression_holds_and_blocks_restored_dispatch() -> TestResult {
    let f = Fixture::new()?;
    let (mut store, retained) = f.populated()?;
    store.register_artifact_reference(ArtifactReferenceRegistration {
        artifact_id: retained.artifact_id(),
        privacy_scope: retained.privacy_scope().clone(),
        reference_kind: BoundedText::try_new("receipt")?,
        reference_id: BoundedText::try_new("receipt-1")?,
        created_at: time(20)?,
    })?;
    store.add_artifact_retention_hold(ArtifactRetentionHold {
        hold_id: BoundedText::try_new("hold-1")?,
        artifact_id: retained.artifact_id(),
        privacy_scope: retained.privacy_scope().clone(),
        reason: BoundedText::try_new("retained evidence")?,
        expires_at: None,
        created_at: time(25)?,
    })?;
    store.suppress_artifact(retained.artifact_id(), retained.privacy_scope(), time(30)?)?;
    let visible = artifact(&mut store, &f.artifacts, 901, retained.privacy_scope())?;
    let operation = approved(&mut store, 905)?;
    let outbox = store.stage_outbox(
        NewOutboxMessage {
            outbox_id: id(910)?,
            operation_id: operation.operation_id(),
            attempt_identity: id(911)?,
            destination: BoundedText::try_new("fixture://target")?,
            message_kind: BoundedText::try_new("fixture")?,
            payload: b"never resend".to_vec(),
            created_at: time(200)?,
        },
        operation.revision(),
    )?;
    let owner = BoundedText::try_new("old-worker")?;
    store.claim_outbox(owner.clone(), time(201)?, time(10000)?, 1)?;
    let exported = f.export(&mut store)?;
    assert!(!store.dispatch_status()?.enabled);
    let restored = StateStore::restore_authenticated_plaintext_snapshot(
        &f.snapshot(),
        &f.restored(),
        &f.key(),
        SnapshotLimits::default(),
    )?;
    assert_eq!(restored.store_id, exported.store_id);
    assert_ne!(restored.runtime_epoch, exported.runtime_epoch);
    let mut copy = StateStore::open(f.restored().join("state.sqlite3"))?;
    assert!(!copy.dispatch_status()?.enabled);
    assert!(
        copy.open_verified_artifact(
            &f.restored(),
            retained.artifact_id(),
            retained.privacy_scope()
        )
        .is_err()
    );
    let mut bytes = Vec::new();
    copy.open_verified_artifact(
        &f.restored(),
        visible.artifact_id(),
        visible.privacy_scope(),
    )?
    .into_file()
    .read_to_end(&mut bytes)?;
    assert_eq!(bytes, b"retained bytes");
    assert_eq!(
        copy.connection
            .query_row("SELECT count(*) FROM artifact_references", [], |r| r
                .get::<_, i64>(0))?,
        1
    );
    assert_eq!(
        copy.connection.query_row(
            "SELECT count(*) FROM artifact_retention_holds WHERE expires_at_micros IS NULL",
            [],
            |r| r.get::<_, i64>(0)
        )?,
        1
    );
    assert!(matches!(
        copy.begin_dispatch(outbox.outbox_id(), &owner, time(300)?),
        Err(OutboxError::RecoveryRequired)
    ));
    assert_eq!(
        copy.load_outbox(outbox.outbox_id())?
            .ok_or("missing outbox")?
            .state(),
        OutboxState::Leased
    );
    Ok(())
}

fn resign(path: &Path, key: &BackupKey, manifest: &Manifest) -> TestResult {
    let data = serde_json::to_vec(manifest)?;
    let mut mac = key.authenticator()?;
    mac.update(AUTHENTICATION_DOMAIN);
    mac.update(&data);
    fs::write(path.join("manifest.json"), data)?;
    fs::write(path.join("manifest.hmac"), mac.finalize().into_bytes())?;
    Ok(())
}

#[test]
fn wrong_key_and_unauthenticated_manifest_fail_before_database_parsing() -> TestResult {
    let f = Fixture::new()?;
    let (mut store, _) = f.populated()?;
    f.export(&mut store)?;
    fs::write(f.snapshot().join("state.sqlite3"), b"not even sqlite")?;
    assert!(matches!(
        StateStore::restore_authenticated_plaintext_snapshot(
            &f.snapshot(),
            &f.restored(),
            &BackupKey::from_bytes([3; 32]),
            SnapshotLimits::default()
        ),
        Err(SnapshotError::Authentication)
    ));
    let mut manifest = fs::read(f.snapshot().join("manifest.json"))?;
    manifest.push(b' ');
    fs::write(f.snapshot().join("manifest.json"), manifest)?;
    assert!(matches!(
        StateStore::restore_authenticated_plaintext_snapshot(
            &f.snapshot(),
            &f.restored(),
            &f.key(),
            SnapshotLimits::default()
        ),
        Err(SnapshotError::Authentication)
    ));
    assert!(!f.restored().exists());
    Ok(())
}

#[test]
fn missing_corrupt_extra_and_symlinked_blobs_are_rejected_without_activation() -> TestResult {
    for variant in ["missing", "corrupt", "extra", "symlink", "database"] {
        let f = Fixture::new()?;
        let (mut store, stored) = f.populated()?;
        f.export(&mut store)?;
        let blob = f
            .snapshot()
            .join("blobs")
            .join(
                ContentHash::from_bytes(
                    Sha256::digest(stored.privacy_scope().as_str().as_bytes()).into(),
                )
                .to_hex(),
            )
            .join(stored.content_hash().to_hex());
        match variant {
            "missing" => fs::remove_file(&blob)?,
            "corrupt" => fs::write(&blob, b"broken bytes")?,
            "extra" => fs::write(f.snapshot().join("extra"), b"not inventoried")?,
            "symlink" => {
                fs::remove_file(&blob)?;
                symlink(&f.database, &blob)?;
            }
            "database" => fs::write(f.snapshot().join("state.sqlite3"), b"broken database")?,
            _ => return Err("unknown variant".into()),
        }
        assert!(
            StateStore::restore_authenticated_plaintext_snapshot(
                &f.snapshot(),
                &f.restored(),
                &f.key(),
                SnapshotLimits::default()
            )
            .is_err(),
            "{variant}"
        );
        assert!(!f.restored().exists());
        assert_eq!(
            store.store_id()?,
            StateStore::open(&f.database)?.store_id()?
        );
    }
    Ok(())
}

#[test]
fn authenticated_but_inconsistent_or_unsupported_inventory_fails_closed() -> TestResult {
    for variant in [
        "duplicate",
        "version",
        "scope",
        "count",
        "identity",
        "epoch",
    ] {
        let f = Fixture::new()?;
        let (mut store, _) = f.populated()?;
        f.export(&mut store)?;
        let mut manifest: Manifest =
            serde_json::from_slice(&fs::read(f.snapshot().join("manifest.json"))?)?;
        match variant {
            "duplicate" => manifest.blobs.push(manifest.blobs[0].clone()),
            "version" => manifest.format_version += 1,
            "scope" => manifest.blobs[0].privacy_scope = BoundedText::try_new("../outside")?,
            "count" => manifest.blobs.clear(),
            "identity" => manifest.store_id = Uuid::new_v4(),
            "epoch" => manifest.runtime_epoch = Uuid::new_v4(),
            _ => return Err("unknown variant".into()),
        }
        resign(&f.snapshot(), &f.key(), &manifest)?;
        assert!(
            StateStore::restore_authenticated_plaintext_snapshot(
                &f.snapshot(),
                &f.restored(),
                &f.key(),
                SnapshotLimits::default()
            )
            .is_err(),
            "{variant}"
        );
        assert!(!f.restored().exists());
    }
    Ok(())
}

#[test]
fn authenticated_altered_schema_is_not_executed_during_restore() -> TestResult {
    let f = Fixture::new()?;
    let (mut store, _) = f.populated()?;
    f.export(&mut store)?;
    let path = f.snapshot().join("state.sqlite3");
    let conn = Connection::open(&path)?;
    conn.execute_batch("CREATE TRIGGER tamper BEFORE UPDATE ON runtime_control BEGIN DELETE FROM artifact_handles_all; END;")?;
    drop(conn);
    let mut manifest: Manifest =
        serde_json::from_slice(&fs::read(f.snapshot().join("manifest.json"))?)?;
    (manifest.database_hash, manifest.database_bytes) = hash_file(
        File::open(path)?,
        MAX_DATABASE_BYTES,
        &Budget::new(SnapshotLimits::default()),
    )?;
    resign(&f.snapshot(), &f.key(), &manifest)?;
    assert!(matches!(
        StateStore::restore_authenticated_plaintext_snapshot(
            &f.snapshot(),
            &f.restored(),
            &f.key(),
            SnapshotLimits::default()
        ),
        Err(SnapshotError::Invalid(
            "snapshot schema differs from compiled schema"
        ))
    ));
    assert!(!f.restored().exists());
    Ok(())
}

#[test]
fn export_and_restore_budgets_are_enforced_and_keys_are_redacted() -> TestResult {
    let f = Fixture::new()?;
    let (mut store, _) = f.populated()?;
    let limits = SnapshotLimits::try_new(MAX_DATABASE_BYTES, 1, 10, Duration::from_secs(1))?;
    assert!(
        store
            .export_snapshot_inner(
                &f.artifacts,
                &f.snapshot(),
                &f.key(),
                time(1000)?,
                limits,
                &mut |_| Ok(())
            )
            .is_err()
    );
    assert!(!f.snapshot().exists());
    f.export(&mut store)?;
    assert!(
        StateStore::restore_authenticated_plaintext_snapshot(
            &f.snapshot(),
            &f.restored(),
            &f.key(),
            limits
        )
        .is_err()
    );
    assert!(
        StateStore::restore_authenticated_plaintext_snapshot(
            &f.snapshot(),
            &f.restored(),
            &f.key(),
            SnapshotLimits::try_new(1, MAX_TOTAL_BLOB_BYTES, MAX_BLOBS, Duration::from_secs(1))?
        )
        .is_err()
    );
    assert_eq!(format!("{:?}", f.key()), "BackupKey([REDACTED])");
    assert!(SnapshotLimits::try_new(MAX_DATABASE_BYTES + 1, 0, 0, Duration::from_secs(1)).is_err());
    assert!(SnapshotLimits::try_new(1, 0, 0, Duration::ZERO).is_err());
    Ok(())
}

#[test]
fn failures_before_publish_clean_staging_but_after_rename_report_uncertainty() -> TestResult {
    for phase in [
        SnapshotPhase::DatabaseCopied,
        SnapshotPhase::BlobCopied,
        SnapshotPhase::BeforePublish,
        SnapshotPhase::Published,
    ] {
        let f = Fixture::new()?;
        let (mut store, _) = f.populated()?;
        let result = store.export_snapshot_inner(
            &f.artifacts,
            &f.snapshot(),
            &f.key(),
            time(1000)?,
            SnapshotLimits::default(),
            &mut |stage| {
                if phase == stage {
                    Err(io::Error::other("injected storage failure"))
                } else {
                    Ok(())
                }
            },
        );
        assert!(result.is_err());
        if phase == SnapshotPhase::Published {
            assert!(matches!(
                result,
                Err(SnapshotError::PublicationUncertain { .. })
            ));
            assert!(f.snapshot().exists());
            StateStore::restore_authenticated_plaintext_snapshot(
                &f.snapshot(),
                &f.restored(),
                &f.key(),
                SnapshotLimits::default(),
            )?;
        } else {
            assert!(!f.snapshot().exists());
            assert!(!fs::read_dir(&f.root)?.any(|e| e.is_ok_and(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with(".snapshot-pending-")
            })));
        }
    }
    Ok(())
}

#[test]
fn destination_is_never_overwritten_and_private_paths_are_required() -> TestResult {
    let f = Fixture::new()?;
    let (mut store, _) = f.populated()?;
    f.export(&mut store)?;
    let before = fs::read(f.snapshot().join("manifest.json"))?;
    assert!(f.export(&mut store).is_err());
    assert_eq!(fs::read(f.snapshot().join("manifest.json"))?, before);
    fs::create_dir(f.restored())?;
    fs::write(f.restored().join("keep"), b"active profile")?;
    assert!(
        StateStore::restore_authenticated_plaintext_snapshot(
            &f.snapshot(),
            &f.restored(),
            &f.key(),
            SnapshotLimits::default()
        )
        .is_err()
    );
    assert_eq!(fs::read(f.restored().join("keep"))?, b"active profile");
    let link = f.root.join("parent-link");
    symlink(&f.root, &link)?;
    assert!(Directory::open_private(&link).is_err());
    fs::set_permissions(&f.artifacts, fs::Permissions::from_mode(0o755))?;
    assert!(
        store
            .export_snapshot_inner(
                &f.artifacts,
                &f.root.join("other"),
                &f.key(),
                time(1000)?,
                SnapshotLimits::default(),
                &mut |_| Ok(())
            )
            .is_err()
    );
    Ok(())
}

#[test]
fn writer_boundary_excludes_gc_until_all_snapshot_blobs_are_copied() -> TestResult {
    let f = Fixture::new()?;
    let (mut store, stored) = f.populated()?;
    store.suppress_artifact(stored.artifact_id(), stored.privacy_scope(), time(20)?)?;
    store.enqueue_eligible_artifact_blobs(time(30)?, 1)?;
    let mut collector = StateStore::open(&f.database)?;
    collector
        .connection
        .busy_timeout(Duration::from_millis(10))?;
    let mut attempted = false;
    store.export_snapshot_inner(
        &f.artifacts,
        &f.snapshot(),
        &f.key(),
        time(1000)?,
        SnapshotLimits::default(),
        &mut |phase| {
            if phase == SnapshotPhase::WriterLocked {
                attempted = true;
                assert!(
                    collector
                        .run_artifact_gc(
                            &f.artifacts,
                            UnixTimestampMicros::try_new(1000).map_err(io::Error::other)?,
                            1
                        )
                        .is_err()
                );
            }
            Ok(())
        },
    )?;
    assert!(attempted);
    collector.run_artifact_gc(&f.artifacts, time(1001)?, 1)?;
    StateStore::restore_authenticated_plaintext_snapshot(
        &f.snapshot(),
        &f.restored(),
        &f.key(),
        SnapshotLimits::default(),
    )?;
    let restored = StateStore::open(f.restored().join("state.sqlite3"))?;
    assert_eq!(
        restored
            .connection
            .query_row("SELECT count(*) FROM artifact_blobs", [], |r| r
                .get::<_, i64>(0))?,
        1
    );
    assert!(
        restored
            .artifact_metadata(stored.artifact_id(), stored.privacy_scope())
            .is_err()
    );
    Ok(())
}

struct ChildGuard(Child);
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn process_fixture() -> TestResult {
    let Some(root) = std::env::var_os("INTENT_SNAPSHOT_TEST_ROOT") else {
        return Ok(());
    };
    let root = PathBuf::from(root);
    let phase_name = std::env::var("INTENT_SNAPSHOT_TEST_PHASE")?;
    let target = if phase_name == "published" {
        SnapshotPhase::Published
    } else {
        SnapshotPhase::BlobCopied
    };
    let mut store = StateStore::open(root.join("state.sqlite3"))?;
    store.export_snapshot_inner(
        &root.join("artifacts"),
        &root.join("backup"),
        &BackupKey::from_bytes([42; 32]),
        time(1000)?,
        SnapshotLimits::default(),
        &mut |phase| {
            if phase == target {
                fs::write(root.join("ready-to-kill"), b"phase reached")?;
                loop {
                    thread::sleep(Duration::from_millis(50));
                }
            }
            Ok(())
        },
    )?;
    Err("child unexpectedly completed without reaching failpoint".into())
}

#[test]
fn actual_process_death_does_not_activate_a_partial_export_or_hold_gc_locks() -> TestResult {
    for phase in ["copied", "published"] {
        let f = Fixture::new()?;
        let (store, _) = f.populated()?;
        let original_id = store.store_id()?;
        drop(store);
        let mut child = ChildGuard(
            Command::new(std::env::current_exe()?)
                .args([
                    "--exact",
                    "snapshots::tests::process_fixture",
                    "--nocapture",
                ])
                .env("INTENT_SNAPSHOT_TEST_ROOT", &f.root)
                .env("INTENT_SNAPSHOT_TEST_PHASE", phase)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?,
        );
        let started = Instant::now();
        while !f.root.join("ready-to-kill").exists() {
            if started.elapsed() > Duration::from_secs(10) || child.0.try_wait()?.is_some() {
                return Err("child failed to reach snapshot crash boundary".into());
            }
            thread::sleep(Duration::from_millis(10));
        }
        child.0.kill()?;
        assert!(child.0.wait()?.signal().is_some());
        let mut reopened = StateStore::open(&f.database)?;
        assert_eq!(reopened.store_id()?, original_id);
        assert!(!reopened.dispatch_status()?.enabled);
        if phase == "published" {
            StateStore::restore_authenticated_plaintext_snapshot(
                &f.snapshot(),
                &f.restored(),
                &f.key(),
                SnapshotLimits::default(),
            )?;
        } else {
            assert!(!f.snapshot().exists());
            f.export(&mut reopened)?;
        }
    }
    Ok(())
}

#[test]
fn authenticated_version_seven_snapshot_upgrades_only_the_private_restore_candidate() -> TestResult
{
    let f = Fixture::new()?;
    let parent = Directory::open_private(&f.root)?;
    let source = parent.create_child("backup")?;
    drop(source.create_file("state.sqlite3")?);
    let mut legacy = Connection::open(source.sqlite_path())?;
    legacy.execute_batch("PRAGMA foreign_keys=ON; PRAGMA synchronous=FULL;")?;
    legacy.pragma_update(None, "application_id", crate::APPLICATION_ID)?;
    crate::migrations::apply_migrations_through(&mut legacy, 7)?;
    let store_id = Uuid::new_v4();
    legacy.execute(
        "INSERT INTO store_metadata(singleton,store_uuid,created_unix_seconds) VALUES (1,?1,0)",
        [store_id.to_string()],
    )?;
    // Construct the original v7 export barrier without invoking a v10 helper.
    // The recovery restore fence does not exist in this historical schema.
    let epoch = Uuid::new_v4();
    assert_eq!(
        legacy.execute(
            "UPDATE runtime_control SET epoch=?1, dispatch_enabled=0, reason='legacy snapshot: recovery required' WHERE singleton=1",
            [epoch.to_string()],
        )?,
        1
    );
    validate_database_version(&legacy, false)?;
    assert!(validate_database(&legacy).is_err());
    legacy.close().map_err(|(_, e)| e)?;
    let (database_hash, database_bytes) = hash_file(
        source.open_file("state.sqlite3")?,
        MAX_DATABASE_BYTES,
        &Budget::new(SnapshotLimits::default()),
    )?;
    let manifest = Manifest {
        format_version: FORMAT_VERSION,
        snapshot_id: Uuid::new_v4(),
        store_id,
        runtime_epoch: epoch,
        schema_version: 7,
        journal_sequence: 0,
        created_at: time(100)?,
        database_hash,
        database_bytes,
        blobs: vec![],
    };
    source.create_child("blobs")?;
    write_manifest(&source, &f.key(), &manifest)?;
    let restored = StateStore::restore_authenticated_plaintext_snapshot(
        &f.snapshot(),
        &f.restored(),
        &f.key(),
        SnapshotLimits::default(),
    )?;
    assert_eq!(restored.source_schema_version, 7);
    assert_eq!(restored.schema_version, 11);
    assert_ne!(restored.runtime_epoch, epoch);
    let copy = StateStore::open(f.restored().join("state.sqlite3"))?;
    assert!(!copy.dispatch_status()?.enabled);
    assert_eq!(copy.store_id()?, store_id);
    assert_eq!(
        copy.connection.query_row(
            "SELECT required FROM recovery_restore_fence WHERE singleton=1",
            [],
            |row| row.get::<_, i64>(0),
        )?,
        1
    );
    assert_eq!(
        copy.connection
            .query_row("SELECT COUNT(*) FROM task_checkpoints", [], |r| r
                .get::<_, i64>(0))?,
        0
    );
    let original = Connection::open(source.sqlite_path())?;
    assert_eq!(
        original.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))?,
        7
    );
    let (actual_hash, actual_size) = hash_file(
        source.open_file("state.sqlite3")?,
        MAX_DATABASE_BYTES,
        &Budget::new(SnapshotLimits::default()),
    )?;
    assert_eq!((actual_hash, actual_size), (database_hash, database_bytes));
    Ok(())
}

#[test]
fn unknown_or_tampered_legacy_snapshot_schema_is_not_migrated() -> TestResult {
    let f = Fixture::new()?;
    let (mut store, _) = f.populated()?;
    f.export(&mut store)?;
    let backup = Directory::open_private(&f.snapshot())?;
    let mut manifest = read_manifest(&backup, &f.key(), SnapshotLimits::default())?;
    let database = Connection::open(backup.sqlite_path())?;
    database.execute_batch("CREATE TABLE unexpected_authority(enabled INTEGER)")?;
    database.close().map_err(|(_, e)| e)?;
    let (hash, size) = hash_file(
        backup.open_file("state.sqlite3")?,
        MAX_DATABASE_BYTES,
        &Budget::new(SnapshotLimits::default()),
    )?;
    manifest.database_hash = hash;
    manifest.database_bytes = size;
    // A valid MAC from a trusted exporter is not permission to interpret an
    // unexpected database schema, even when its declared version is supported.
    fs::remove_file(f.snapshot().join("manifest.json"))?;
    fs::remove_file(f.snapshot().join("manifest.hmac"))?;
    write_manifest(&backup, &f.key(), &manifest)?;
    assert!(
        StateStore::restore_authenticated_plaintext_snapshot(
            &f.snapshot(),
            &f.restored(),
            &f.key(),
            SnapshotLimits::default()
        )
        .is_err()
    );
    assert!(!f.restored().exists());
    Ok(())
}
