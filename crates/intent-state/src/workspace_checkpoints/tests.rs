use super::*;
use crate::{
    ConsumerEffect, DurableOperationState, InboxEvent, NewArtifact, NewDurableOperation,
    OperationTransition,
};
use intent_contracts::{BoundedText, ByteSize, Task, Workspace};
use std::{
    error::Error,
    fs,
    io::Cursor,
    os::unix::fs::DirBuilderExt,
    path::PathBuf,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;
struct Profile(PathBuf);
impl Profile {
    fn new() -> Result<Self> {
        let root = std::env::temp_dir().join(format!("intent-checkpoint-{}", Uuid::new_v4()));
        fs::DirBuilder::new().mode(0o700).create(&root)?;
        fs::DirBuilder::new()
            .mode(0o700)
            .create(root.join("artifacts"))?;
        Ok(Self(root))
    }
    fn db(&self) -> PathBuf {
        self.0.join("state.sqlite3")
    }
    fn blobs(&self) -> PathBuf {
        self.0.join("artifacts")
    }
}
impl Drop for Profile {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn id<T: std::str::FromStr>(n: u64) -> Result<T>
where
    T::Err: Error + 'static,
{
    Ok(format!("018f47f7-5a86-7c00-8000-{n:012x}").parse()?)
}
fn now(n: i64) -> Result<UnixTimestampMicros> {
    Ok(UnixTimestampMicros::try_new(n)?)
}
fn scope() -> Result<ArtifactScope> {
    Ok(ArtifactScope::try_new("workspace-scope")?)
}
fn graph() -> Result<WorkspaceGraph> {
    Ok(WorkspaceGraph {
        schema_version: SchemaVersion::V1,
        workspace: Workspace::new(
            id(1)?,
            BoundedText::try_new("runtime owned workspace")?,
            now(1)?,
        ),
        goals: vec![],
        tasks: vec![Task::new(
            id(2)?,
            id(1)?,
            BoundedText::try_new("verified result")?,
            now(1)?,
        )],
        dependencies: vec![],
        retained_artifacts: vec![],
        cursors: vec![],
        provider_references: vec![],
        worker_instances: vec![],
    })
}
fn request() -> Result<CheckpointRequest> {
    Ok(CheckpointRequest {
        checkpoint_id: CheckpointId::new(),
        workspace_id: id(1)?,
        expected_graph_revision: 1,
        created_at: now(5)?,
    })
}
fn add_artifact(
    store: &mut StateStore,
    profile: &Profile,
    graph: &mut WorkspaceGraph,
) -> Result<ArtifactId> {
    let id = id(70)?;
    let m = store.store_artifact(
        &profile.blobs(),
        NewArtifact {
            artifact_id: id,
            privacy_scope: scope()?,
            media_type: BoundedText::try_new("application/octet-stream")?,
            created_at: now(1)?,
        },
        &mut Cursor::new(b"durable source evidence"),
    )?;
    graph.retained_artifacts.push(ArtifactReference::new(
        id,
        m.content_hash(),
        ByteSize::from_bytes(m.byte_size()),
        BoundedText::try_new("application/octet-stream")?,
    ));
    Ok(id)
}
fn prepared(store: &mut StateStore) -> Result<crate::DurableOperation> {
    Ok(store.create_operation(NewDurableOperation {
        operation_id: id(10)?,
        task_id: id(2)?,
        action_proposal_id: id(11)?,
        account_id: id(12)?,
        capability_id: id(13)?,
        arguments_hash: hash(b"action"),
        source_schema: SchemaVersion::V1,
        created_at: now(1)?,
    })?)
}

#[test]
fn checkpoint_captures_durable_graph_operations_cursors_and_secondary_references() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let mut g = graph()?;
    let artifact = add_artifact(&mut store, &profile, &mut g)?;
    prepared(&mut store)?;
    let key = CheckpointCursorKey {
        consumer: BoundedText::try_new("consumer")?,
        source: BoundedText::try_new("source")?,
        stream: BoundedText::try_new("stream")?,
    };
    store.apply_inbox_event(
        &key.consumer,
        InboxEvent {
            source: key.source.clone(),
            event_id: BoundedText::try_new("event")?,
            stream: key.stream.clone(),
            sequence: 0,
            payload: b"observed".to_vec(),
            received_at: now(2)?,
        },
        vec![ConsumerEffect {
            key: BoundedText::try_new("projection")?,
            payload: b"result".to_vec(),
        }],
        now(3)?,
    )?;
    g.cursors.push(key);
    g.worker_instances.push(id(20)?);
    g.provider_references.push(CheckpointProviderReference {
        provider: BoundedText::try_new("fixture")?,
        account_id: id(12)?,
        session_reference: hash(b"secondary-reference"),
    });
    store.save_workspace_graph(&scope()?, 0, &g, now(4)?)?;
    let req = request()?;
    let receipt = store.checkpoint_workspace(&profile.blobs(), &scope()?, &req)?;
    assert_eq!(receipt.artifact_count, 1);
    let restored =
        store.load_verified_checkpoint(&profile.blobs(), &scope()?, req.checkpoint_id)?;
    assert_eq!(restored.graph(), &g);
    assert_eq!(restored.operations().len(), 1);
    assert_eq!(
        restored.operations()[0].state,
        DurableOperationState::Prepared
    );
    assert_eq!(restored.cursors()[0].last_sequence, 0);
    assert!(!restored.grants_execution_authority());
    assert_eq!(store.artifact_reference_count(artifact, &scope()?)?, 1);
    assert!(!store.dispatch_status()?.enabled);
    drop(store);
    let mut store = StateStore::open(profile.db())?;
    assert_eq!(
        store.load_verified_checkpoint(&profile.blobs(), &scope()?, req.checkpoint_id)?,
        restored
    );
    assert!(!store.dispatch_status()?.enabled);
    Ok(())
}

#[test]
fn graph_cas_scope_and_node_ownership_are_atomic() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let g = graph()?;
    assert_eq!(
        store
            .save_workspace_graph(&scope()?, 0, &g, now(1)?)?
            .revision,
        1
    );
    assert!(matches!(
        store.save_workspace_graph(&scope()?, 0, &g, now(2)?),
        Err(CheckpointError::Conflict)
    ));
    assert!(matches!(
        store.save_workspace_graph(&ArtifactScope::try_new("other")?, 1, &g, now(2)?),
        Err(CheckpointError::AccessDenied)
    ));
    let mut other = g.clone();
    other.workspace = Workspace::new(id(3)?, BoundedText::try_new("other")?, now(1)?);
    other.tasks = vec![Task::new(
        id(2)?,
        id(3)?,
        BoundedText::try_new("reuse")?,
        now(1)?,
    )];
    assert!(
        store
            .save_workspace_graph(&scope()?, 0, &other, now(2)?)
            .is_err()
    );
    assert_eq!(
        store
            .connection
            .query_row("SELECT COUNT(*) FROM workspace_graphs", [], |r| r
                .get::<_, i64>(0))?,
        1
    );
    assert_eq!(
        store
            .save_workspace_graph(&scope()?, 1, &g, now(3)?)?
            .revision,
        2
    );
    Ok(())
}

#[test]
fn cyclic_duplicate_foreign_and_excessive_graphs_are_rejected() -> Result {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let mut g = graph()?;
    g.tasks.push(Task::new(
        id(3)?,
        id(1)?,
        BoundedText::try_new("next")?,
        now(1)?,
    ));
    g.dependencies = vec![
        TaskDependency {
            prerequisite: id(2)?,
            dependent: id(3)?,
        },
        TaskDependency {
            prerequisite: id(3)?,
            dependent: id(2)?,
        },
    ];
    assert!(
        store
            .save_workspace_graph(&scope()?, 0, &g, now(1)?)
            .is_err()
    );
    g.dependencies.pop();
    g.dependencies.push(g.dependencies[0]);
    assert!(
        store
            .save_workspace_graph(&scope()?, 0, &g, now(1)?)
            .is_err()
    );
    g.dependencies.clear();
    g.tasks.push(g.tasks[0].clone());
    assert!(
        store
            .save_workspace_graph(&scope()?, 0, &g, now(1)?)
            .is_err()
    );
    g = graph()?;
    g.tasks[0] = Task::new(id(2)?, id(9)?, BoundedText::try_new("foreign")?, now(1)?);
    assert!(
        store
            .save_workspace_graph(&scope()?, 0, &g, now(1)?)
            .is_err()
    );
    g = graph()?;
    g.tasks = vec![g.tasks[0].clone(); MAX_GRAPH_TASKS + 1];
    assert!(
        store
            .save_workspace_graph(&scope()?, 0, &g, now(1)?)
            .is_err()
    );
    assert_eq!(
        store
            .connection
            .query_row("SELECT COUNT(*) FROM workspace_graphs", [], |r| r
                .get::<_, i64>(0))?,
        0
    );
    Ok(())
}

#[test]
fn removing_a_task_cannot_hide_its_operation_history() -> Result {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let mut g = graph()?;
    g.tasks.push(Task::new(
        id(3)?,
        id(1)?,
        BoundedText::try_new("remaining")?,
        now(1)?,
    ));
    store.save_workspace_graph(&scope()?, 0, &g, now(1)?)?;
    prepared(&mut store)?;
    g.tasks.remove(0);
    assert!(
        store
            .save_workspace_graph(&scope()?, 1, &g, now(2)?)
            .is_err()
    );
    assert_eq!(load_graph(&store.connection, &scope()?, id(1)?)?.0, 1);
    Ok(())
}

#[test]
fn checkpoint_retry_is_idempotent_and_changed_requests_conflict() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let g = graph()?;
    store.save_workspace_graph(&scope()?, 0, &g, now(1)?)?;
    let req = request()?;
    let saved = store.checkpoint_workspace(&profile.blobs(), &scope()?, &req)?;
    store.save_workspace_graph(&scope()?, 1, &g, now(6)?)?;
    assert_eq!(
        store.checkpoint_workspace(&profile.blobs(), &scope()?, &req)?,
        saved
    );
    let changed = CheckpointRequest {
        expected_graph_revision: 2,
        ..req.clone()
    };
    assert!(matches!(
        store.checkpoint_workspace(&profile.blobs(), &scope()?, &changed),
        Err(CheckpointError::Conflict)
    ));
    assert!(matches!(
        store.checkpoint_workspace(&profile.blobs(), &ArtifactScope::try_new("foreign")?, &req),
        Err(CheckpointError::AccessDenied)
    ));
    Ok(())
}

#[test]
fn checkpoint_pin_survives_suppression_and_requires_explicit_release() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let mut g = graph()?;
    let artifact = add_artifact(&mut store, &profile, &mut g)?;
    store.save_workspace_graph(&scope()?, 0, &g, now(2)?)?;
    let req = request()?;
    let receipt = store.checkpoint_workspace(&profile.blobs(), &scope()?, &req)?;
    store.suppress_artifact(artifact, &scope()?, now(6)?)?;
    assert!(matches!(
        store.load_verified_checkpoint(&profile.blobs(), &scope()?, req.checkpoint_id),
        Err(CheckpointError::SuppressedArtifact(_))
    ));
    assert!(
        store
            .remove_artifact_reference(
                artifact,
                &scope()?,
                &BoundedText::try_new("core_checkpoint")?,
                &BoundedText::try_new(req.checkpoint_id.to_string())?,
                now(7)?
            )
            .is_err()
    );
    assert!(
        store
            .connection
            .execute(
                "DELETE FROM workspace_checkpoint_pins WHERE checkpoint_id=?1",
                [req.checkpoint_id.to_string()]
            )
            .is_err()
    );
    assert_eq!(store.enqueue_eligible_artifact_blobs(now(7)?, 64)?, 0);
    assert!(matches!(
        store.release_checkpoint(&scope()?, req.checkpoint_id, hash(b"wrong")),
        Err(CheckpointError::Conflict)
    ));
    assert!(store.release_checkpoint(&scope()?, req.checkpoint_id, receipt.digest)?);
    assert!(!store.release_checkpoint(&scope()?, req.checkpoint_id, receipt.digest)?);
    assert_eq!(store.enqueue_eligible_artifact_blobs(now(8)?, 64)?, 1);
    assert_eq!(
        store
            .run_artifact_gc(&profile.blobs(), now(9)?, 64)?
            .deleted_blobs,
        1
    );
    Ok(())
}

#[test]
fn missing_suppressed_cross_scope_and_wrong_hash_dependencies_fail_before_commit() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let mut g = graph()?;
    let artifact = add_artifact(&mut store, &profile, &mut g)?;
    g.retained_artifacts[0] = ArtifactReference::new(
        artifact,
        hash(b"not-the-bytes"),
        g.retained_artifacts[0].byte_size(),
        BoundedText::try_new("application/octet-stream")?,
    );
    store.save_workspace_graph(&scope()?, 0, &g, now(1)?)?;
    assert!(matches!(
        store.checkpoint_workspace(&profile.blobs(), &scope()?, &request()?),
        Err(CheckpointError::CorruptArtifact(_))
    ));
    store.suppress_artifact(artifact, &scope()?, now(2)?)?;
    assert!(matches!(
        store.checkpoint_workspace(&profile.blobs(), &scope()?, &request()?),
        Err(CheckpointError::SuppressedArtifact(_))
    ));
    assert_eq!(
        store
            .connection
            .query_row("SELECT COUNT(*) FROM workspace_checkpoints", [], |r| r
                .get::<_, i64>(0))?,
        0
    );
    Ok(())
}

#[test]
fn corrupt_or_missing_blob_cannot_restore_a_checkpoint() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let mut g = graph()?;
    add_artifact(&mut store, &profile, &mut g)?;
    store.save_workspace_graph(&scope()?, 0, &g, now(1)?)?;
    let req = request()?;
    store.checkpoint_workspace(&profile.blobs(), &scope()?, &req)?;
    let path = profile
        .blobs()
        .join("blobs")
        .join(hash(scope()?.as_str().as_bytes()).to_hex())
        .join(g.retained_artifacts[0].content_hash().to_hex());
    fs::write(&path, b"corrupt")?;
    assert!(matches!(
        store.load_verified_checkpoint(&profile.blobs(), &scope()?, req.checkpoint_id),
        Err(CheckpointError::CorruptArtifact(_))
    ));
    fs::remove_file(&path)?;
    assert!(matches!(
        store.load_verified_checkpoint(&profile.blobs(), &scope()?, req.checkpoint_id),
        Err(CheckpointError::MissingArtifact(_))
    ));
    Ok(())
}

#[test]
fn injected_failure_rolls_back_the_snapshot_and_all_pins() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let mut g = graph()?;
    let artifact = add_artifact(&mut store, &profile, &mut g)?;
    store.save_workspace_graph(&scope()?, 0, &g, now(1)?)?;
    let req = request()?;
    assert!(
        store
            .checkpoint_workspace_with_hook(&profile.blobs(), &scope()?, &req, &mut || Err(
                CheckpointError::Invalid("injected failure")
            ))
            .is_err()
    );
    assert_eq!(store.artifact_reference_count(artifact, &scope()?)?, 0);
    assert!(matches!(
        store.load_verified_checkpoint(&profile.blobs(), &scope()?, req.checkpoint_id),
        Err(CheckpointError::NotFound)
    ));
    store.checkpoint_workspace(&profile.blobs(), &scope()?, &req)?;
    Ok(())
}

#[test]
fn another_writer_cannot_cross_checkpoint_publication() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let mut g = graph()?;
    let artifact = add_artifact(&mut store, &profile, &mut g)?;
    store.save_workspace_graph(&scope()?, 0, &g, now(1)?)?;
    let mut second = StateStore::open(profile.db())?;
    second.connection.busy_timeout(Duration::ZERO)?;
    let req = request()?;
    store.checkpoint_workspace_with_hook(&profile.blobs(), &scope()?, &req, &mut || {
        assert!(
            second
                .suppress_artifact(
                    artifact,
                    &scope().map_err(|_| CheckpointError::Invalid("scope"))?,
                    now(7).map_err(|_| CheckpointError::Invalid("time"))?
                )
                .is_err()
        );
        Ok(())
    })?;
    assert_eq!(store.artifact_reference_count(artifact, &scope()?)?, 1);
    Ok(())
}

#[test]
fn projection_journal_mismatch_is_not_frozen_as_valid_history() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    prepared(&mut store)?;
    store.save_workspace_graph(&scope()?, 0, &graph()?, now(1)?)?;
    store
        .connection
        .execute("UPDATE durable_operations SET revision=42", [])?;
    assert!(matches!(
        store.checkpoint_workspace(&profile.blobs(), &scope()?, &request()?),
        Err(CheckpointError::Invalid(_))
    ));
    Ok(())
}

#[test]
fn graph_and_checkpoint_payload_mutation_is_rejected() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    store.save_workspace_graph(&scope()?, 0, &graph()?, now(1)?)?;
    let req = request()?;
    store.checkpoint_workspace(&profile.blobs(), &scope()?, &req)?;
    assert!(
        store
            .connection
            .execute("UPDATE workspace_checkpoints SET payload=X'7B7D'", [])
            .is_err()
    );
    store
        .connection
        .execute("UPDATE workspace_graphs SET graph=X'7B7D'", [])?;
    assert!(
        store
            .checkpoint_workspace(&profile.blobs(), &scope()?, &request()?)
            .is_err()
    );
    Ok(())
}

#[test]
fn snapshot_restore_preserves_checkpoint_pins_without_restoring_authority() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let mut g = graph()?;
    add_artifact(&mut store, &profile, &mut g)?;
    store.save_workspace_graph(&scope()?, 0, &g, now(1)?)?;
    let req = request()?;
    store.checkpoint_workspace(&profile.blobs(), &scope()?, &req)?;
    let backup = profile.0.join("backup");
    let restored = profile.0.join("restored");
    let key = crate::BackupKey::from_bytes([7; 32]);
    store.export_authenticated_plaintext_snapshot(
        &profile.blobs(),
        &backup,
        &key,
        now(10)?,
        crate::SnapshotLimits::default(),
        crate::PlaintextExportConsent::SensitiveDataWillBeWrittenUnencrypted,
    )?;
    StateStore::restore_authenticated_plaintext_snapshot(
        &backup,
        &restored,
        &key,
        crate::SnapshotLimits::default(),
    )?;
    let mut restored_store = StateStore::open(restored.join("state.sqlite3"))?;
    assert!(!restored_store.dispatch_status()?.enabled);
    assert_eq!(
        restored_store
            .load_verified_checkpoint(&restored, &scope()?, req.checkpoint_id)?
            .graph(),
        &g
    );
    Ok(())
}

#[test]
fn missing_cursor_and_bounded_encoder_fail_without_partial_rows() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let mut g = graph()?;
    g.cursors.push(CheckpointCursorKey {
        consumer: BoundedText::try_new("missing")?,
        source: BoundedText::try_new("source")?,
        stream: BoundedText::try_new("stream")?,
    });
    store.save_workspace_graph(&scope()?, 0, &g, now(1)?)?;
    assert!(matches!(
        store.checkpoint_workspace(&profile.blobs(), &scope()?, &request()?),
        Err(CheckpointError::MissingCursor(_))
    ));
    assert!(encode_bounded(&g, 1).is_err());
    assert_eq!(
        store
            .connection
            .query_row("SELECT COUNT(*) FROM workspace_checkpoints", [], |r| r
                .get::<_, i64>(0))?,
        0
    );
    Ok(())
}

#[test]
fn a_later_operation_transition_does_not_rewrite_a_checkpoint() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    prepared(&mut store)?;
    store.save_workspace_graph(&scope()?, 0, &graph()?, now(1)?)?;
    let req = request()?;
    store.checkpoint_workspace(&profile.blobs(), &scope()?, &req)?;
    store.transition_operation(
        id(10)?,
        OperationTransition {
            expected_revision: 0,
            next_state: DurableOperationState::Approved,
            state_detail: None,
            attempt_identity: None,
            occurred_at: now(8)?,
        },
    )?;
    let old = store.load_verified_checkpoint(&profile.blobs(), &scope()?, req.checkpoint_id)?;
    assert_eq!(old.operations()[0].state, DurableOperationState::Prepared);
    assert_eq!(
        store.load_operation(id(10)?)?.ok_or("missing")?.state(),
        DurableOperationState::Approved
    );
    Ok(())
}

#[test]
fn checkpoint_crash_child() -> Result {
    let Some(root) = std::env::var_os("INTENT_CHECKPOINT_KILL_FIXTURE") else {
        return Ok(());
    };
    let root = PathBuf::from(root);
    let mut store = StateStore::open(root.join("state.sqlite3"))?;
    let req: CheckpointRequest = CheckpointRequest {
        checkpoint_id: serde_json::from_str(&fs::read_to_string(root.join("request.json"))?)?,
        workspace_id: id(1)?,
        expected_graph_revision: 1,
        created_at: now(5)?,
    };
    store.checkpoint_workspace_with_hook(&root.join("artifacts"), &scope()?, &req, &mut || {
        fs::write(root.join("inside-transaction"), b"ready")?;
        loop {
            thread::sleep(Duration::from_millis(100));
        }
    })?;
    Ok(())
}

#[test]
fn real_process_kill_before_checkpoint_commit_leaves_no_orphan_pin() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let mut g = graph()?;
    let artifact = add_artifact(&mut store, &profile, &mut g)?;
    store.save_workspace_graph(&scope()?, 0, &g, now(1)?)?;
    let req = request()?;
    fs::write(
        profile.0.join("request.json"),
        serde_json::to_vec(&req.checkpoint_id)?,
    )?;
    drop(store);
    let mut child = Command::new(std::env::current_exe()?)
        .args([
            "--exact",
            "workspace_checkpoints::tests::checkpoint_crash_child",
            "--nocapture",
        ])
        .env_clear()
        .env("INTENT_CHECKPOINT_KILL_FIXTURE", &profile.0)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let started = Instant::now();
    while !profile.0.join("inside-transaction").exists() {
        if child.try_wait()?.is_some() || started.elapsed() > Duration::from_secs(10) {
            let _ = child.kill();
            let _ = child.wait();
            return Err("checkpoint child failed to reach publication barrier".into());
        }
        thread::sleep(Duration::from_millis(5));
    }
    child.kill()?;
    child.wait()?;
    let mut store = StateStore::open(profile.db())?;
    assert_eq!(store.artifact_reference_count(artifact, &scope()?)?, 0);
    assert_eq!(
        store
            .connection
            .query_row("SELECT COUNT(*) FROM workspace_checkpoints", [], |r| r
                .get::<_, i64>(0))?,
        0
    );
    store.checkpoint_workspace(&profile.blobs(), &scope()?, &req)?;
    assert_eq!(store.artifact_reference_count(artifact, &scope()?)?, 1);
    Ok(())
}

#[test]
fn altered_task_registry_or_journal_gap_cannot_omit_work_from_checkpoint() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    prepared(&mut store)?;
    store.save_workspace_graph(&scope()?, 0, &graph()?, now(1)?)?;
    store
        .connection
        .execute("UPDATE workspace_graph_tasks SET active=0", [])?;
    assert!(
        store
            .checkpoint_workspace(&profile.blobs(), &scope()?, &request()?)
            .is_err()
    );
    store
        .connection
        .execute("UPDATE workspace_graph_tasks SET active=1", [])?;
    store.transition_operation(
        id(10)?,
        OperationTransition {
            expected_revision: 0,
            next_state: DurableOperationState::Approved,
            state_detail: None,
            attempt_identity: None,
            occurred_at: now(3)?,
        },
    )?;
    store
        .connection
        .execute("DELETE FROM operation_journal WHERE revision=0", [])?;
    assert!(
        store
            .checkpoint_workspace(&profile.blobs(), &scope()?, &request()?)
            .is_err()
    );
    Ok(())
}

#[test]
fn reserved_checkpoint_reference_namespace_cannot_create_immortal_or_extra_pins() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let mut g = graph()?;
    let artifact = add_artifact(&mut store, &profile, &mut g)?;
    assert!(
        store
            .register_artifact_reference(crate::ArtifactReferenceRegistration {
                artifact_id: artifact,
                privacy_scope: scope()?,
                reference_kind: BoundedText::try_new("core_checkpoint")?,
                reference_id: BoundedText::try_new(CheckpointId::new().to_string())?,
                created_at: now(1)?
            })
            .is_err()
    );
    assert_eq!(store.artifact_reference_count(artifact, &scope()?)?, 0);
    Ok(())
}

#[test]
fn checkpoint_debug_does_not_disclose_domain_content() -> Result {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.db())?;
    let mut g = graph()?;
    g.workspace = Workspace::new(
        id(1)?,
        BoundedText::try_new("PRIVATE_TITLE_SENTINEL")?,
        now(1)?,
    );
    assert!(!format!("{g:?}").contains("PRIVATE_TITLE_SENTINEL"));
    store.save_workspace_graph(&scope()?, 0, &g, now(1)?)?;
    let req = request()?;
    store.checkpoint_workspace(&profile.blobs(), &scope()?, &req)?;
    let loaded = store.load_verified_checkpoint(&profile.blobs(), &scope()?, req.checkpoint_id)?;
    assert!(!format!("{loaded:?}").contains("PRIVATE_TITLE_SENTINEL"));
    Ok(())
}

#[test]
fn concurrent_graph_cas_produces_one_success_and_one_conflict() -> Result {
    let profile = Profile::new()?;
    let mut first = StateStore::open(profile.db())?;
    first.save_workspace_graph(&scope()?, 0, &graph()?, now(1)?)?;
    let mut second = StateStore::open(profile.db())?;
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let peer = barrier.clone();
    let worker = thread::spawn(move || {
        peer.wait();
        second.save_workspace_graph(
            &scope().map_err(|_| CheckpointError::Invalid("scope"))?,
            1,
            &graph().map_err(|_| CheckpointError::Invalid("graph"))?,
            now(2).map_err(|_| CheckpointError::Invalid("time"))?,
        )
    });
    barrier.wait();
    let a = first.save_workspace_graph(&scope()?, 1, &graph()?, now(2)?);
    let b = worker.join().map_err(|_| "graph writer panicked")?;
    assert_eq!(usize::from(a.is_ok()) + usize::from(b.is_ok()), 1);
    let failed = if a.is_err() { a } else { b };
    assert!(matches!(failed, Err(CheckpointError::Conflict)));
    Ok(())
}
