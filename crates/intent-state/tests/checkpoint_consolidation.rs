#![cfg(all(target_os = "linux", target_env = "gnu"))]

use intent_contracts::{
    BoundedText, CheckpointId, SchemaVersion, Task, TaskId, UnixTimestampMicros, Workspace,
    WorkspaceId,
};
use intent_state::{
    ArtifactScope, CheckpointGraph, CheckpointRequest, StateStore, WorkspaceCheckpointError,
    WorkspaceCheckpointId, WorkspaceCheckpointRequest, workspace_checkpoints::WorkspaceGraph,
};
use std::{error::Error, fs, os::unix::fs::DirBuilderExt, path::PathBuf};
use uuid::Uuid;

type TestResult = Result<(), Box<dyn Error>>;

struct Profile(PathBuf);

impl Profile {
    fn new() -> Result<Self, Box<dyn Error>> {
        let path = std::env::temp_dir().join(format!("intent-consolidation-{}", Uuid::new_v4()));
        fs::DirBuilder::new().mode(0o700).create(&path)?;
        Ok(Self(path))
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn task_and_workspace_checkpoints_with_the_same_uuid_remain_independent() -> TestResult {
    let profile = Profile::new()?;
    let mut store = StateStore::open_in_memory_for_tests()?;
    let scope = ArtifactScope::try_new("consolidation")?;
    let now = UnixTimestampMicros::try_new(1)?;
    let workspace_id = WorkspaceId::from_uuid(Uuid::new_v4());
    let workspace = Workspace::new(workspace_id, BoundedText::try_new("workspace")?, now);
    let task = Task::new(
        TaskId::from_uuid(Uuid::new_v4()),
        workspace_id,
        BoundedText::try_new("task")?,
        now,
    );
    let workspace_checkpoint_id = WorkspaceCheckpointId::new();
    let task_checkpoint_id = CheckpointId::from_uuid(workspace_checkpoint_id.as_uuid());
    let saved_task = store.save_task_checkpoint(
        &profile.0,
        CheckpointRequest {
            checkpoint_id: task_checkpoint_id,
            privacy_scope: BoundedText::try_new(scope.as_str())?,
            expected_graph_revision: None,
            graph: CheckpointGraph {
                workspace: workspace.clone(),
                task: task.clone(),
                goal: None,
                nodes: vec![],
                artifact_ids: vec![],
                cursor_keys: vec![],
                provider_references: vec![],
                worker_epoch: None,
            },
            created_at: now,
        },
    )?;
    let revision = store.save_workspace_graph(
        &scope,
        0,
        &WorkspaceGraph {
            schema_version: SchemaVersion::V1,
            workspace,
            goals: vec![],
            tasks: vec![task],
            dependencies: vec![],
            retained_artifacts: vec![],
            cursors: vec![],
            provider_references: vec![],
            worker_instances: vec![],
        },
        now,
    )?;
    let receipt = store.checkpoint_workspace(
        &profile.0,
        &scope,
        &WorkspaceCheckpointRequest {
            checkpoint_id: workspace_checkpoint_id,
            workspace_id,
            expected_graph_revision: revision.revision,
            created_at: now,
        },
    )?;
    let saved_workspace =
        store.load_verified_checkpoint(&profile.0, &scope, workspace_checkpoint_id)?;
    assert_eq!(saved_workspace.workspace_id(), workspace_id);
    assert_eq!(saved_workspace.graph_revision(), 1);
    assert_eq!(saved_task.graph_revision(), 0);
    assert!(!saved_task.grants_execution_authority());
    assert!(!saved_workspace.grants_execution_authority());
    assert!(store.release_checkpoint(&scope, workspace_checkpoint_id, receipt.digest)?);
    assert!(matches!(
        store.load_verified_checkpoint(&profile.0, &scope, workspace_checkpoint_id),
        Err(WorkspaceCheckpointError::NotFound)
    ));
    assert_eq!(
        store.load_task_checkpoint(&profile.0, task_checkpoint_id, &scope)?,
        saved_task
    );
    Ok(())
}
