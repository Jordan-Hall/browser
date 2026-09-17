use super::*;
use crate::{ArtifactReferenceRegistration, test_support::*};
use rusqlite::ErrorCode;
use std::{
    cell::Cell,
    io::{Cursor, Read},
    time::Duration,
};

fn hold(number: u64, scope: &ArtifactScope) -> TestResult<ArtifactRetentionHold> {
    Ok(ArtifactRetentionHold {
        hold_id: BoundedText::try_new(format!("hold-{number}"))?,
        artifact_id: id(number)?,
        privacy_scope: scope.clone(),
        reason: BoundedText::try_new("evidence")?,
        expires_at: None,
        created_at: time(11)?,
    })
}

fn reference(number: u64, scope: &ArtifactScope) -> TestResult<ArtifactReferenceRegistration> {
    Ok(ArtifactReferenceRegistration {
        artifact_id: id(number)?,
        privacy_scope: scope.clone(),
        reference_kind: BoundedText::try_new("receipt")?,
        reference_id: BoundedText::try_new("one")?,
        created_at: time(11)?,
    })
}

fn is_busy(error: &rusqlite::Error) -> bool {
    matches!(error, rusqlite::Error::SqliteFailure(code, _) if code.code == ErrorCode::DatabaseBusy)
}

#[test]
fn gc_writer_lock_excludes_publication_reference_and_hold_through_unlink() -> TestResult {
    let profile = Profile::new()?;
    let root = profile.artifacts();
    let mut collector = StateStore::open(profile.database())?;
    let mut writer = StateStore::open(profile.database())?;
    writer.connection.busy_timeout(Duration::ZERO)?;
    let scope = ArtifactScope::try_new("private")?;
    let original = artifact(&mut collector, &root, 1, &scope)?;
    collector.suppress_artifact(original.artifact_id(), &scope, time(20)?)?;
    let new_hold = hold(1, &scope)?;
    let new_reference = reference(1, &scope)?;
    let new_handle = new_artifact(2, &scope)?;
    let outcome = collector.collect_blob_with(&root, scope.as_str(), &original.content_hash().to_hex(), time(30)?, |path| {
        if !matches!(writer.add_artifact_retention_hold(new_hold), Err(RetentionError::Sqlite(error)) if is_busy(&error)) {
            return Err(io::Error::other("hold writer was not excluded by collector"));
        }
        if !matches!(writer.register_artifact_reference(new_reference), Err(ArtifactError::Sqlite(error)) if is_busy(&error)) {
            return Err(io::Error::other("reference writer was not excluded by collector"));
        }
        if !matches!(writer.store_artifact(&root, new_handle, &mut Cursor::new(b"retained bytes")), Err(ArtifactError::Sqlite(error)) if is_busy(&error)) {
            return Err(io::Error::other("publisher was not excluded before final link"));
        }
        remove_blob_and_sync(path)
    })?;
    assert_eq!(outcome, GcOutcome::Deleted);
    let published = artifact(&mut writer, &root, 2, &scope)?;
    let mut file = writer
        .open_verified_artifact(&root, published.artifact_id(), &scope)?
        .into_file();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    assert_eq!(bytes, b"retained bytes");
    assert_eq!(
        writer
            .connection
            .query_row("SELECT count(*) FROM artifact_handles_all", [], |row| row
                .get::<_, i64>(
                0
            ))?,
        1
    );
    Ok(())
}

#[test]
fn committed_live_handle_or_hold_prevents_unlink() -> TestResult {
    for protect_with_hold in [false, true] {
        let profile = Profile::new()?;
        let root = profile.artifacts();
        let mut collector = StateStore::open(profile.database())?;
        let mut writer = StateStore::open(profile.database())?;
        let scope = ArtifactScope::try_new("private")?;
        let original = artifact(&mut collector, &root, 1, &scope)?;
        collector.suppress_artifact(original.artifact_id(), &scope, time(20)?)?;
        if protect_with_hold {
            writer.add_artifact_retention_hold(hold(1, &scope)?)?;
        } else {
            artifact(&mut writer, &root, 2, &scope)?;
        }
        let called = Cell::new(false);
        let outcome = collector.collect_blob_with(
            &root,
            scope.as_str(),
            &original.content_hash().to_hex(),
            time(30)?,
            |_| {
                called.set(true);
                Err(io::Error::other("must not unlink protected content"))
            },
        )?;
        assert_eq!(outcome, GcOutcome::Deferred);
        assert!(!called.get());
        assert!(blob_path(&root, scope.as_str(), &original.content_hash().to_hex())?.is_file());
    }
    Ok(())
}

#[test]
fn publication_must_acquire_the_writer_lock_before_exposing_a_final_path() -> TestResult {
    let profile = Profile::new()?;
    let root = profile.artifacts();
    let mut owner = StateStore::open(profile.database())?;
    let mut publisher = StateStore::open(profile.database())?;
    publisher.connection.busy_timeout(Duration::ZERO)?;
    let transaction = owner
        .connection
        .transaction_with_behavior(TransactionBehavior::Immediate)?;
    let scope = ArtifactScope::try_new("private")?;
    let result = publisher.store_artifact(
        &root,
        new_artifact(1, &scope)?,
        &mut Cursor::new(b"retained bytes"),
    );
    assert!(matches!(result, Err(ArtifactError::Sqlite(error)) if is_busy(&error)));
    assert!(!root.join("blobs").exists());
    transaction.commit()?;
    artifact(&mut publisher, &root, 1, &scope)?;
    Ok(())
}

#[test]
fn missing_file_retry_repeats_the_failed_directory_barrier() -> TestResult {
    let profile = Profile::new()?;
    let path = profile.artifacts();
    fs::write(&path, b"delete")?;
    let sync_calls = Cell::new(0);
    let first = remove_blob_and_sync_with(
        &path,
        |path| fs::remove_file(path),
        |_| {
            sync_calls.set(sync_calls.get() + 1);
            Err(io::Error::other("injected directory sync failure"))
        },
    );
    assert!(first.is_err());
    assert!(!path.exists());
    remove_blob_and_sync_with(
        &path,
        |path| fs::remove_file(path),
        |parent| {
            sync_calls.set(sync_calls.get() + 1);
            sync_directory(parent)
        },
    )?;
    assert_eq!(sync_calls.get(), 2);
    Ok(())
}

#[test]
fn unicode_gc_diagnostic_is_bounded_and_failure_remains_retryable() -> TestResult {
    let profile = Profile::new()?;
    let root = profile.artifacts();
    let mut store = StateStore::open(profile.database())?;
    let scope = ArtifactScope::try_new("private")?;
    let item = artifact(&mut store, &root, 1, &scope)?;
    store.suppress_artifact(item.artifact_id(), &scope, time(20)?)?;
    let message = format!("{}é tail", "a".repeat(2047));
    let result = store.collect_blob_with(
        &root,
        scope.as_str(),
        &item.content_hash().to_hex(),
        time(30)?,
        |_| Err(io::Error::other(message)),
    )?;
    assert_eq!(result, GcOutcome::Failed);
    let (state, diagnostic): (String, String) = store.connection.query_row(
        "SELECT state, last_error FROM artifact_gc_queue",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(state, "failed");
    assert!(diagnostic.len() <= 2048);
    assert!(diagnostic.is_char_boundary(diagnostic.len()));
    assert_eq!(store.run_artifact_gc(&root, time(31)?, 1)?.deleted_blobs, 1);
    Ok(())
}

#[test]
fn candidate_limit_excludes_held_referenced_live_shared_and_queued_prefixes() -> TestResult {
    let profile = Profile::new()?;
    let root = profile.artifacts();
    let mut store = StateStore::open(profile.database())?;
    let scopes = [
        "a-held",
        "b-referenced",
        "c-shared",
        "d-queued",
        "e-eligible",
        "f-eligible",
    ];
    for (index, name) in scopes.iter().enumerate() {
        let number = u64::try_from(index)? + 1;
        let scope = ArtifactScope::try_new(*name)?;
        artifact(&mut store, &root, number, &scope)?;
        if index == 0 {
            store.add_artifact_retention_hold(hold(number, &scope)?)?;
        }
        if index == 1 {
            store.register_artifact_reference(reference(number, &scope)?)?;
        }
        if index == 2 {
            artifact(&mut store, &root, 100, &scope)?;
        }
        store.suppress_artifact(id(number)?, &scope, time(20)?)?;
    }
    store.connection.execute(
        "DELETE FROM artifact_gc_queue WHERE privacy_scope != 'd-queued'",
        [],
    )?;
    assert_eq!(store.enqueue_eligible_artifact_blobs(time(30)?, 1)?, 1);
    let first: String = store.connection.query_row(
        "SELECT privacy_scope FROM artifact_gc_queue WHERE privacy_scope != 'd-queued'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(first, "e-eligible");
    assert_eq!(store.enqueue_eligible_artifact_blobs(time(30)?, 1)?, 1);
    assert_eq!(store.enqueue_eligible_artifact_blobs(time(30)?, 1)?, 0);
    assert_eq!(
        store
            .connection
            .query_row("SELECT count(*) FROM artifact_gc_queue", [], |row| row
                .get::<_, i64>(0))?,
        3
    );
    Ok(())
}

#[test]
fn repeated_suppression_is_idempotent_and_zero_batch_does_not_prune() -> TestResult {
    let profile = Profile::new()?;
    let root = profile.artifacts();
    let mut store = StateStore::open(profile.database())?;
    let scope = ArtifactScope::try_new("private")?;
    artifact(&mut store, &root, 1, &scope)?;
    let mut expires = hold(1, &scope)?;
    expires.expires_at = Some(time(25)?);
    store.add_artifact_retention_hold(expires)?;
    assert_eq!(
        store.suppress_artifact(id(1)?, &scope, time(20)?)?,
        SuppressionResult::Suppressed
    );
    assert_eq!(
        store.suppress_artifact(id(1)?, &scope, time(21)?)?,
        SuppressionResult::AlreadySuppressed
    );
    assert_eq!(store.enqueue_eligible_artifact_blobs(time(30)?, 0)?, 0);
    assert_eq!(
        store.run_artifact_gc(&root, time(30)?, 0)?,
        GcReport::default()
    );
    assert_eq!(
        store
            .connection
            .query_row("SELECT count(*) FROM artifact_retention_holds", [], |row| {
                row.get::<_, i64>(0)
            })?,
        1
    );
    assert_eq!(store.enqueue_eligible_artifact_blobs(time(30)?, 1)?, 1);
    assert_eq!(store.enqueue_eligible_artifact_blobs(time(31)?, 1)?, 0);
    Ok(())
}
