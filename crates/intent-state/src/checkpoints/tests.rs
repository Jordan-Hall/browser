use super::*;
use crate::{
    DurableOperationState, InboxEvent, NewDurableOperation, OperationTransition,
    test_support::{Profile, TestResult, artifact, id, time},
};
use intent_contracts::{ArtifactReference, ByteSize};
use std::{
    fs,
    sync::{Arc, Barrier},
    thread,
    time::{Duration, Instant},
};

fn request(number: u64, artifacts: Vec<ArtifactId>) -> TestResult<CheckpointRequest> {
    Ok(CheckpointRequest {
        checkpoint_id: id(number)?,
        privacy_scope: BoundedText::try_new("checkpoint-tests")?,
        expected_graph_revision: None,
        graph: CheckpointGraph {
            workspace: Workspace::new(
                id(20)?,
                BoundedText::try_new("durable workspace")?,
                time(1)?,
            ),
            task: Task::new(
                id(30)?,
                id(20)?,
                BoundedText::try_new("source-backed result")?,
                time(1)?,
            ),
            goal: None,
            nodes: vec![CheckpointNode {
                id: 1,
                state: TaskState::Planned,
                intent: BoundedText::try_new("read source")?,
                depends_on: vec![],
                outputs: artifacts.clone(),
            }],
            artifact_ids: artifacts,
            cursor_keys: vec![],
            provider_references: vec![ProviderCheckpointReference {
                provider: ProviderId::try_new("fixture")?,
                account: id(40)?,
                metadata_reference: hash(b"runtime-owned-session-metadata"),
            }],
            worker_epoch: Some(id(50)?),
        },
        created_at: time(100)?,
    })
}

fn scope() -> TestResult<ArtifactScope> {
    Ok(ArtifactScope::try_new("checkpoint-tests")?)
}
fn count(connection: &Connection, table: &str) -> TestResult<i64> {
    Ok(connection.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |r| r.get(0))?)
}

#[test]
fn checkpoint_commits_graph_cursor_operation_and_exact_artifact_then_reopens() -> TestResult {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    let meta = artifact(&mut store, &profile.artifacts(), 1, &scope()?)?;
    store.create_operation(NewDurableOperation {
        binding: None,
        operation_id: id(60)?,
        task_id: id(30)?,
        action_proposal_id: id(61)?,
        account_id: id(40)?,
        capability_id: id(62)?,
        arguments_hash: hash(b"immutable arguments"),
        source_schema: SchemaVersion::V1,
        created_at: time(50)?,
    })?;
    let key = CursorKey {
        consumer: BoundedText::try_new("task-reader")?,
        source: BoundedText::try_new("fixture")?,
        stream: BoundedText::try_new("observations")?,
    };
    store.apply_inbox_event(
        &key.consumer,
        InboxEvent {
            source: key.source.clone(),
            event_id: BoundedText::try_new("event-1")?,
            stream: key.stream.clone(),
            sequence: 0,
            payload: b"source observation".to_vec(),
            received_at: time(70)?,
        },
        vec![],
        time(75)?,
    )?;
    let mut new = request(100, vec![meta.artifact_id()])?;
    new.graph.cursor_keys.push(key);
    let saved = store.save_task_checkpoint(&profile.artifacts(), new)?;
    assert_eq!(saved.graph_revision(), 0);
    assert_eq!(saved.operations().len(), 1);
    assert_eq!(saved.cursors()[0].sequence, 0);
    assert_eq!(count(&store.connection, "checkpoint_artifacts")?, 1);
    assert_eq!(
        store.artifact_reference_count(meta.artifact_id(), &scope()?)?,
        1
    );
    assert!(!saved.grants_execution_authority());
    drop(store);
    let store = StateStore::open(profile.database())?;
    let loaded = store.load_task_checkpoint(&profile.artifacts(), saved.id(), &scope()?)?;
    assert_eq!(loaded, saved);
    assert!(!store.dispatch_status()?.enabled);
    assert!(!format!("{saved:?}").contains("source-backed result"));
    Ok(())
}

#[test]
fn identical_retry_is_stable_after_newer_operation_state_but_changed_input_conflicts() -> TestResult
{
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    let new = request(101, vec![])?;
    let saved = store.save_task_checkpoint(&profile.artifacts(), new.clone())?;
    store.create_operation(NewDurableOperation {
        binding: None,
        operation_id: id(60)?,
        task_id: id(30)?,
        action_proposal_id: id(61)?,
        account_id: id(40)?,
        capability_id: id(62)?,
        arguments_hash: hash(b"arguments"),
        source_schema: SchemaVersion::V1,
        created_at: time(150)?,
    })?;
    assert_eq!(
        store.save_task_checkpoint(&profile.artifacts(), new.clone())?,
        saved
    );
    let mut changed = new;
    changed.created_at = time(101)?;
    assert!(matches!(
        store.save_task_checkpoint(&profile.artifacts(), changed),
        Err(CheckpointError::IdentityConflict)
    ));
    assert_eq!(count(&store.connection, "task_checkpoints")?, 1);
    Ok(())
}

#[test]
fn same_revision_writers_produce_one_committed_graph_and_one_conflict() -> TestResult {
    let profile = Profile::new()?;
    let db = profile.database();
    let root = profile.artifacts();
    let mut first = StateStore::open(&db)?;
    let mut second = StateStore::open(&db)?;
    let barrier = Arc::new(Barrier::new(2));
    let other = barrier.clone();
    let root2 = root.clone();
    let req2 = request(103, vec![])?;
    let handle = thread::spawn(move || {
        other.wait();
        second.save_task_checkpoint(&root2, req2)
    });
    barrier.wait();
    let left = first.save_task_checkpoint(&root, request(102, vec![])?);
    let right = handle.join().map_err(|_| "checkpoint writer panicked")?;
    assert_ne!(left.is_ok(), right.is_ok());
    assert!(matches!(
        left.as_ref().err().or(right.as_ref().err()),
        Some(CheckpointError::StaleRevision)
    ));
    assert_eq!(count(&first.connection, "task_checkpoints")?, 1);
    assert_eq!(count(&first.connection, "task_checkpoint_heads")?, 1);
    Ok(())
}

#[test]
fn failure_after_pin_insertion_rolls_back_graph_head_and_references() -> TestResult {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    let meta = artifact(&mut store, &profile.artifacts(), 1, &scope()?)?;
    let result = store.save_checkpoint_inner(
        &profile.artifacts(),
        request(104, vec![meta.artifact_id()])?,
        || Err(CheckpointError::Invalid("injected pre-commit failure")),
    );
    assert!(result.is_err());
    for table in [
        "task_checkpoints",
        "task_checkpoint_heads",
        "checkpoint_artifacts",
        "artifact_references",
    ] {
        assert_eq!(count(&store.connection, table)?, 0);
    }
    assert!(
        store
            .open_verified_artifact(&profile.artifacts(), meta.artifact_id(), &scope()?)
            .is_ok()
    );
    Ok(())
}

#[test]
fn graph_cycles_duplicates_missing_goals_or_cursor_and_wrong_scope_are_atomic_failures()
-> TestResult {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    let meta = artifact(&mut store, &profile.artifacts(), 1, &scope()?)?;
    let base = request(105, vec![meta.artifact_id()])?;
    let mut cyclic = base.clone();
    cyclic.graph.nodes[0].depends_on.push(1);
    let mut duplicate = base.clone();
    duplicate.graph.artifact_ids.push(meta.artifact_id());
    let mut dangling = base.clone();
    dangling.graph.nodes[0].depends_on.push(99);
    let mut missing_cursor = base.clone();
    missing_cursor.graph.cursor_keys.push(CursorKey {
        consumer: BoundedText::try_new("missing")?,
        source: BoundedText::try_new("source")?,
        stream: BoundedText::try_new("stream")?,
    });
    let mut wrong_scope = base.clone();
    wrong_scope.privacy_scope = BoundedText::try_new("another-profile")?;
    let mut missing_goal = base.clone();
    let mut task = serde_json::to_value(&missing_goal.graph.task)?;
    task["goal_contract_id"] = serde_json::json!(id::<intent_contracts::GoalContractId>(99)?);
    missing_goal.graph.task = serde_json::from_value(task)?;
    for new in [
        cyclic,
        duplicate,
        dangling,
        missing_cursor,
        wrong_scope,
        missing_goal,
    ] {
        assert!(
            store
                .save_task_checkpoint(&profile.artifacts(), new)
                .is_err()
        );
        assert_eq!(count(&store.connection, "task_checkpoints")?, 0);
        assert_eq!(count(&store.connection, "artifact_references")?, 0);
    }
    store.save_task_checkpoint(&profile.artifacts(), base)?;
    assert!(matches!(
        store.load_task_checkpoint(
            &profile.artifacts(),
            id(105)?,
            &ArtifactScope::try_new("another-profile")?
        ),
        Err(CheckpointError::ScopeMismatch)
    ));
    Ok(())
}

#[test]
fn graph_and_payload_byte_limits_fail_before_committing() -> TestResult {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    let mut too_many = request(106, vec![])?;
    let small_intent = BoundedText::try_new("x")?;
    too_many.graph.nodes = (1..=257)
        .map(|id| CheckpointNode {
            id,
            state: TaskState::Planned,
            intent: small_intent.clone(),
            depends_on: vec![],
            outputs: vec![],
        })
        .collect();
    assert!(matches!(
        store.save_task_checkpoint(&profile.artifacts(), too_many),
        Err(CheckpointError::Capacity)
    ));
    let mut oversized = request(106, vec![])?;
    let intent = BoundedText::try_new("\u{0001}".repeat(4096))?;
    oversized.graph.nodes = (1..=256)
        .map(|id| CheckpointNode {
            id,
            state: TaskState::Planned,
            intent: intent.clone(),
            depends_on: vec![],
            outputs: vec![],
        })
        .collect();
    assert!(matches!(
        store.save_task_checkpoint(&profile.artifacts(), oversized),
        Err(CheckpointError::Capacity)
    ));
    assert_eq!(count(&store.connection, "task_checkpoints")?, 0);
    Ok(())
}

#[test]
fn checkpoint_owned_pin_survives_suppression_and_cannot_be_removed_outside_retirement() -> TestResult
{
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    let meta = artifact(&mut store, &profile.artifacts(), 1, &scope()?)?;
    let saved = store.save_task_checkpoint(
        &profile.artifacts(),
        request(107, vec![meta.artifact_id()])?,
    )?;
    store.suppress_artifact(meta.artifact_id(), &scope()?, time(200)?)?;
    assert!(
        store
            .remove_artifact_reference(
                meta.artifact_id(),
                &scope()?,
                &BoundedText::try_new("runtime_checkpoint")?,
                &BoundedText::try_new(saved.id().to_string())?,
                time(201)?
            )
            .is_err()
    );
    assert_eq!(
        store
            .run_artifact_gc(&profile.artifacts(), time(202)?, 64)?
            .deleted_blobs,
        0
    );
    assert!(matches!(
        store.load_task_checkpoint(&profile.artifacts(), saved.id(), &scope()?),
        Err(CheckpointError::ArtifactUnavailable(_))
    ));
    assert!(matches!(
        store.retire_task_checkpoint(saved.id(), &scope()?, 0),
        Err(CheckpointError::CurrentCheckpoint)
    ));
    let mut next = request(108, vec![])?;
    next.expected_graph_revision = Some(0);
    let next = store.save_task_checkpoint(&profile.artifacts(), next)?;
    assert_eq!(next.predecessor(), Some(saved.id()));
    assert!(matches!(
        store.retire_task_checkpoint(saved.id(), &scope()?, 0),
        Err(CheckpointError::StaleRevision)
    ));
    store.retire_task_checkpoint(saved.id(), &scope()?, 1)?;
    store.retire_task_checkpoint(saved.id(), &scope()?, 1)?;
    assert_eq!(count(&store.connection, "checkpoint_artifacts")?, 0);
    store.enqueue_eligible_artifact_blobs(time(300)?, 64)?;
    assert_eq!(
        store
            .run_artifact_gc(&profile.artifacts(), time(301)?, 64)?
            .deleted_blobs,
        1
    );
    assert!(matches!(
        store.load_task_checkpoint(&profile.artifacts(), saved.id(), &scope()?),
        Err(CheckpointError::Retired)
    ));
    assert_eq!(count(&store.connection, "task_checkpoints")?, 2);
    Ok(())
}

#[test]
fn missing_corrupt_or_misdescribed_artifact_cannot_be_checkpointed_or_restored() -> TestResult {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    let meta = artifact(&mut store, &profile.artifacts(), 1, &scope()?)?;
    let mut wrong_descriptor = request(109, vec![meta.artifact_id()])?;
    let mut task = serde_json::to_value(&wrong_descriptor.graph.task)?;
    task["result_artifacts"] = serde_json::json!([ArtifactReference::new(
        meta.artifact_id(),
        hash(b"wrong bytes"),
        ByteSize::from_bytes(meta.byte_size()),
        BoundedText::try_new("application/octet-stream")?
    )]);
    wrong_descriptor.graph.task = serde_json::from_value(task)?;
    assert!(
        store
            .save_task_checkpoint(&profile.artifacts(), wrong_descriptor)
            .is_err()
    );
    let saved = store.save_task_checkpoint(
        &profile.artifacts(),
        request(109, vec![meta.artifact_id()])?,
    )?;
    let blob = crate::artifacts::artifact_blob_path(&profile.artifacts(), &meta);
    fs::write(&blob, b"corrupt")?;
    assert!(
        store
            .load_task_checkpoint(&profile.artifacts(), saved.id(), &scope()?)
            .is_err()
    );
    fs::remove_file(&blob)?;
    assert!(
        store
            .load_task_checkpoint(&profile.artifacts(), saved.id(), &scope()?)
            .is_err()
    );
    Ok(())
}

#[test]
fn corrupted_checkpoint_and_lost_pin_fail_closed_without_enabling_dispatch() -> TestResult {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    let meta = artifact(&mut store, &profile.artifacts(), 1, &scope()?)?;
    let saved = store.save_task_checkpoint(
        &profile.artifacts(),
        request(110, vec![meta.artifact_id()])?,
    )?;
    assert!(
        store
            .connection
            .execute(
                "UPDATE task_checkpoints SET payload=x'7b7d' WHERE checkpoint_id=?1",
                [saved.id().to_string()]
            )
            .is_err()
    );
    store
        .connection
        .execute_batch("DROP TRIGGER checkpoint_pin_protect; DELETE FROM artifact_references;")?;
    assert!(matches!(
        store.load_task_checkpoint(&profile.artifacts(), saved.id(), &scope()?),
        Err(CheckpointError::Corrupt)
    ));
    store
        .connection
        .execute_batch("DROP TRIGGER task_checkpoints_immutable;")?;
    store.connection.execute(
        "UPDATE task_checkpoints SET payload=x'7b7d' WHERE checkpoint_id=?1",
        [saved.id().to_string()],
    )?;
    assert!(matches!(
        store.load_task_checkpoint(&profile.artifacts(), saved.id(), &scope()?),
        Err(CheckpointError::Corrupt)
    ));
    assert!(!store.dispatch_status()?.enabled);
    Ok(())
}

#[test]
fn historical_uncertainty_is_not_rewritten_or_reexecuted_by_loading_a_checkpoint() -> TestResult {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    let operation = store.create_operation(NewDurableOperation {
        binding: None,
        operation_id: id(60)?,
        task_id: id(30)?,
        action_proposal_id: id(61)?,
        account_id: id(40)?,
        capability_id: id(62)?,
        arguments_hash: hash(b"write"),
        source_schema: SchemaVersion::V1,
        created_at: time(10)?,
    })?;
    for (revision, state, attempt) in [
        (0, DurableOperationState::Approved, None),
        (1, DurableOperationState::DispatchPending, None),
        (2, DurableOperationState::Attempting, Some(id(63)?)),
        (3, DurableOperationState::NeedsReconciliation, Some(id(63)?)),
    ] {
        store.transition_operation(
            operation.operation_id(),
            OperationTransition {
                expected_revision: revision,
                next_state: state,
                state_detail: None,
                attempt_identity: attempt,
                occurred_at: time(20 + i64::try_from(revision)?)?,
            },
        )?;
    }
    let saved = store.save_task_checkpoint(&profile.artifacts(), request(111, vec![])?)?;
    assert_eq!(saved.operations()[0].state.as_str(), "needs_reconciliation");
    assert_eq!(saved.operations()[0].attempt_identity, Some(id(63)?));
    let revision = store
        .load_operation(operation.operation_id())?
        .ok_or("missing operation")?
        .revision();
    assert_eq!(
        store.load_task_checkpoint(&profile.artifacts(), saved.id(), &scope()?)?,
        saved
    );
    assert_eq!(
        store
            .load_operation(operation.operation_id())?
            .ok_or("missing operation")?
            .revision(),
        revision
    );
    assert!(!store.dispatch_status()?.enabled);
    Ok(())
}

#[test]
fn checkpoint_writer_excludes_collection_until_its_pin_is_committed() -> TestResult {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    let meta = artifact(&mut store, &profile.artifacts(), 1, &scope()?)?;
    let mut competitor = StateStore::open(profile.database())?;
    competitor
        .connection
        .busy_timeout(Duration::from_millis(20))?;
    let root = profile.artifacts();
    store.save_checkpoint_inner(&root, request(112, vec![meta.artifact_id()])?, || {
        assert!(
            competitor
                .suppress_artifact(
                    meta.artifact_id(),
                    &ArtifactScope::try_new("checkpoint-tests")?,
                    UnixTimestampMicros::try_new(200).map_err(|_| CheckpointError::Corrupt)?
                )
                .is_err()
        );
        Ok(())
    })?;
    competitor.suppress_artifact(meta.artifact_id(), &scope()?, time(200)?)?;
    assert_eq!(
        competitor
            .run_artifact_gc(&root, time(201)?, 64)?
            .deleted_blobs,
        0
    );
    Ok(())
}

#[test]
fn checkpoint_process_probe() -> TestResult {
    let Ok(root) = std::env::var("INTENT_CHECKPOINT_PROBE") else {
        return Ok(());
    };
    let mode = std::env::var("INTENT_CHECKPOINT_MODE")?;
    let root = std::path::PathBuf::from(root);
    let mut store = StateStore::open(root.join("state.sqlite3"))?;
    let marker = root.join("checkpoint-marker");
    let new = request(113, vec![id(1)?])?;
    let pause = || -> Result<(), CheckpointError> {
        fs::write(&marker, b"paused").map_err(|_| CheckpointError::Corrupt)?;
        loop {
            thread::sleep(Duration::from_millis(10));
        }
    };
    if mode == "before_commit" {
        store.save_checkpoint_inner(&root.join("artifacts"), new, pause)?;
    } else {
        store.save_task_checkpoint(&root.join("artifacts"), new)?;
        pause()?;
    }
    Ok(())
}

#[test]
fn actual_process_kill_before_and_after_commit_has_distinct_recoverable_results() -> TestResult {
    for before_commit in [true, false] {
        let profile = Profile::new()?;
        let mut store = StateStore::open(profile.database())?;
        artifact(&mut store, &profile.artifacts(), 1, &scope()?)?;
        drop(store);
        let root = profile
            .database()
            .parent()
            .ok_or("missing parent")?
            .to_path_buf();
        let mut child = std::process::Command::new(std::env::current_exe()?)
            .args([
                "--exact",
                "checkpoints::tests::checkpoint_process_probe",
                "--nocapture",
            ])
            .env_clear()
            .env("INTENT_CHECKPOINT_PROBE", &root)
            .env(
                "INTENT_CHECKPOINT_MODE",
                if before_commit {
                    "before_commit"
                } else {
                    "after_commit"
                },
            )
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()?;
        let deadline = Instant::now() + Duration::from_secs(10);
        while !root.join("checkpoint-marker").exists() && Instant::now() < deadline {
            if child.try_wait()?.is_some() {
                break;
            }
            thread::sleep(Duration::from_millis(5));
        }
        let reached = root.join("checkpoint-marker").exists();
        let _ = child.kill();
        child.wait()?;
        assert!(
            reached,
            "checkpoint child did not reach the selected failpoint"
        );
        let mut store = StateStore::open(profile.database())?;
        let expected = i64::from(!before_commit);
        for table in [
            "task_checkpoints",
            "checkpoint_artifacts",
            "artifact_references",
            "task_checkpoint_heads",
        ] {
            assert_eq!(count(&store.connection, table)?, expected);
        }
        let retry =
            store.save_task_checkpoint(&profile.artifacts(), request(113, vec![id(1)?])?)?;
        assert_eq!(retry.graph_revision(), 0);
        assert_eq!(count(&store.connection, "task_checkpoints")?, 1);
        assert!(!store.dispatch_status()?.enabled);
    }
    Ok(())
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[test]
fn authenticated_profile_restore_preserves_checkpoint_graph_pins_and_dispatch_barrier() -> TestResult
{
    use std::os::unix::fs::PermissionsExt;
    let profile = Profile::new()?;
    let root = profile
        .database()
        .parent()
        .ok_or("missing profile root")?
        .to_owned();
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
    fs::create_dir(profile.artifacts())?;
    fs::set_permissions(profile.artifacts(), fs::Permissions::from_mode(0o700))?;
    let mut store = StateStore::open(profile.database())?;
    let meta = artifact(&mut store, &profile.artifacts(), 1, &scope()?)?;
    let checkpoint = store.save_task_checkpoint(
        &profile.artifacts(),
        request(120, vec![meta.artifact_id()])?,
    )?;
    let key = crate::BackupKey::from_bytes([17; 32]);
    store.export_authenticated_plaintext_snapshot(
        &profile.artifacts(),
        &root.join("backup"),
        &key,
        time(200)?,
        crate::SnapshotLimits::default(),
        crate::PlaintextExportConsent::SensitiveDataWillBeWrittenUnencrypted,
    )?;
    let receipt = StateStore::restore_authenticated_plaintext_snapshot(
        &root.join("backup"),
        &root.join("restored"),
        &key,
        crate::SnapshotLimits::default(),
    )?;
    let restored = StateStore::open(root.join("restored/state.sqlite3"))?;
    assert_eq!(
        restored.load_task_checkpoint(&root.join("restored"), checkpoint.id(), &scope()?)?,
        checkpoint
    );
    assert_eq!(
        restored.artifact_reference_count(meta.artifact_id(), &scope()?)?,
        1
    );
    assert_eq!(receipt.schema_version, 12);
    assert_eq!(receipt.source_schema_version, 12);
    assert!(!restored.dispatch_status()?.enabled);
    Ok(())
}

#[test]
fn rehashed_payload_cannot_omit_an_operation_at_the_checkpoint_watermark() -> TestResult {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    store.create_operation(NewDurableOperation {
        binding: None,
        operation_id: id(60)?,
        task_id: id(30)?,
        action_proposal_id: id(61)?,
        account_id: id(40)?,
        capability_id: id(62)?,
        arguments_hash: hash(b"arguments"),
        source_schema: SchemaVersion::V1,
        created_at: time(50)?,
    })?;
    let saved = store.save_task_checkpoint(&profile.artifacts(), request(121, vec![])?)?;
    let mut document = saved.document.clone();
    document.operations.clear();
    let bytes = bounded_json(&document)?;
    store
        .connection
        .execute_batch("DROP TRIGGER task_checkpoints_immutable")?;
    store.connection.execute(
        "UPDATE task_checkpoints SET payload=?1,payload_hash=?2 WHERE checkpoint_id=?3",
        params![bytes, hash(&bytes).to_hex(), saved.id().to_string()],
    )?;
    assert!(matches!(
        store.load_task_checkpoint(&profile.artifacts(), saved.id(), &scope()?),
        Err(CheckpointError::Corrupt)
    ));
    Ok(())
}

#[test]
fn changed_head_binding_is_not_used_as_a_valid_compare_and_swap_predecessor() -> TestResult {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    store.save_task_checkpoint(&profile.artifacts(), request(122, vec![])?)?;
    store
        .connection
        .execute("UPDATE task_checkpoint_heads SET graph_revision=8", [])?;
    let mut next = request(123, vec![])?;
    next.expected_graph_revision = Some(8);
    assert!(matches!(
        store.save_task_checkpoint(&profile.artifacts(), next),
        Err(CheckpointError::Corrupt)
    ));
    assert_eq!(count(&store.connection, "task_checkpoints")?, 1);
    Ok(())
}
