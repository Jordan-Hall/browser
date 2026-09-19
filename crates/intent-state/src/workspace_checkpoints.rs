//! Atomic, bounded snapshots of runtime-owned graphs and their durable dependencies.
mod types;
pub use types::*;

use crate::{ArtifactScope, StateStore, snapshot_fs::Directory};
use intent_contracts::{
    ArtifactId, ArtifactReference, ContentHash, SchemaVersion, TaskState, UnixTimestampMicros,
    WorkspaceId,
};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    io::{Read, Write},
    path::Path,
};
use uuid::Uuid;

const MAX_METADATA_ENTRIES: usize = 64;

impl StateStore {
    /// Replaces a complete graph at an expected revision. A zero expected revision
    /// means creation. Task identities can never migrate between workspaces.
    pub fn save_workspace_graph(
        &mut self,
        scope: &ArtifactScope,
        expected_revision: u64,
        graph: &WorkspaceGraph,
        now: UnixTimestampMicros,
    ) -> Result<GraphRevision, CheckpointError> {
        validate_graph(graph)?;
        let bytes = encode_bounded(graph, MAX_GRAPH_BYTES)?;
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let revision = write_workspace_graph(&tx, scope, expected_revision, graph, &bytes, now)?;
        tx.commit()?;
        Ok(revision)
    }

    /// Installs a new workspace graph from validated archive data without attaching it to any
    /// pre-existing durable operation history. This is a data-activation boundary only: it does
    /// not enable runtime dispatch, restore provider/session state, or grant execution authority.
    pub fn activate_new_workspace_graph(
        &mut self,
        scope: &ArtifactScope,
        graph: &WorkspaceGraph,
        now: UnixTimestampMicros,
    ) -> Result<GraphRevision, CheckpointError> {
        validate_graph(graph)?;
        if graph.tasks.iter().any(|task| {
            matches!(
                task.state(),
                TaskState::Completed | TaskState::Cancelled | TaskState::Failed
            )
        }) {
            return Err(CheckpointError::Invalid(
                "archive activation cannot activate a terminal task",
            ));
        }
        let bytes = encode_bounded(graph, MAX_GRAPH_BYTES)?;
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        for task in &graph.tasks {
            let used: bool = tx.query_row(
                "SELECT EXISTS(SELECT 1 FROM durable_operations WHERE task_id=?1)",
                [task.task_id().to_string()],
                |row| row.get(0),
            )?;
            if used {
                return Err(CheckpointError::Invalid(
                    "archive activation cannot attach existing operation history",
                ));
            }
        }
        let revision = write_workspace_graph(&tx, scope, 0, graph, &bytes, now)?;
        tx.commit()?;
        Ok(revision)
    }

    /// Captures graph, operation projections, cursor positions and artifact pins
    /// under the same writer boundary as publication and garbage collection.
    pub fn checkpoint_workspace(
        &mut self,
        artifact_root: &Path,
        scope: &ArtifactScope,
        request: &CheckpointRequest,
    ) -> Result<CheckpointReceipt, CheckpointError> {
        self.checkpoint_workspace_with_hook(artifact_root, scope, request, &mut || Ok(()))
    }

    fn checkpoint_workspace_with_hook(
        &mut self,
        artifact_root: &Path,
        scope: &ArtifactScope,
        request: &CheckpointRequest,
        before_commit: &mut impl FnMut() -> Result<(), CheckpointError>,
    ) -> Result<CheckpointReceipt, CheckpointError> {
        if request.checkpoint_id.as_uuid().is_nil() || request.expected_graph_revision == 0 {
            return Err(CheckpointError::Invalid(
                "nil checkpoint identity or zero graph revision",
            ));
        }
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        if let Some((record, digest)) = read_checkpoint(&tx, scope, request.checkpoint_id)? {
            if record.workspace_id != request.workspace_id
                || record.graph_revision != request.expected_graph_revision
                || record.created_at != request.created_at
            {
                return Err(CheckpointError::Conflict);
            }
            verify_dependencies(&tx, artifact_root, &record)?;
            return receipt(&record, digest);
        }
        let (revision, graph) = load_graph(&tx, scope, request.workspace_id)?;
        if revision != request.expected_graph_revision {
            return Err(CheckpointError::Conflict);
        }
        let operations = capture_operations(&tx, request.workspace_id)?;
        let mut cursors = Vec::with_capacity(graph.cursors.len());
        for key in &graph.cursors {
            let position: Option<i64> = tx.query_row(
                "SELECT last_sequence FROM consumer_cursors WHERE consumer=?1 AND source=?2 AND stream=?3",
                params![key.consumer.as_str(), key.source.as_str(), key.stream.as_str()], |r| r.get(0),
            ).optional()?;
            cursors.push(CapturedCursor {
                key: key.clone(),
                last_sequence: unsigned(
                    position.ok_or_else(|| CheckpointError::MissingCursor(key.clone()))?,
                )?,
            });
        }
        let epoch: String = tx.query_row(
            "SELECT epoch FROM runtime_control WHERE singleton=1",
            [],
            |r| r.get(0),
        )?;
        let store: String = tx.query_row(
            "SELECT store_uuid FROM store_metadata WHERE singleton=1",
            [],
            |r| r.get(0),
        )?;
        let record = WorkspaceCheckpoint {
            schema_version: SchemaVersion::V1,
            checkpoint_id: request.checkpoint_id,
            store_id: parse_uuid(&store)?,
            workspace_id: request.workspace_id,
            privacy_scope: types::scope_text(scope)?,
            graph_revision: revision,
            runtime_epoch: parse_uuid(&epoch)?,
            created_at: request.created_at,
            graph,
            operations,
            cursors,
        };
        let artifacts = graph_artifacts(&record.graph)?;
        verify_artifacts(&tx, artifact_root, scope.as_str(), &artifacts)?;
        let payload = encode_bounded(&record, MAX_CHECKPOINT_BYTES)?;
        let digest = hash(&payload);
        tx.execute("INSERT INTO workspace_checkpoints(checkpoint_id,workspace_id,privacy_scope,graph_revision,created_at_micros,payload,payload_hash) VALUES (?1,?2,?3,?4,?5,?6,?7)",
            params![request.checkpoint_id.to_string(), request.workspace_id.to_string(), scope.as_str(), sql(revision)?, request.created_at.get(), payload, digest.to_hex()])?;
        for id in artifacts.keys() {
            tx.execute(
                "INSERT INTO workspace_checkpoint_pins(checkpoint_id,artifact_id) VALUES (?1,?2)",
                params![request.checkpoint_id.to_string(), id.to_string()],
            )?;
        }
        before_commit()?;
        tx.commit()?;
        receipt(&record, digest)
    }

    /// Returns a verified historical snapshot, never an executable recovery grant.
    /// Suppressed, missing, corrupt or unpinned dependencies fail visibly.
    pub fn load_verified_checkpoint(
        &mut self,
        artifact_root: &Path,
        scope: &ArtifactScope,
        checkpoint: CheckpointId,
    ) -> Result<WorkspaceCheckpoint, CheckpointError> {
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (record, _) =
            read_checkpoint(&tx, scope, checkpoint)?.ok_or(CheckpointError::NotFound)?;
        verify_dependencies(&tx, artifact_root, &record)?;
        tx.commit()?;
        Ok(record)
    }

    /// Explicitly releases a snapshot and only that snapshot's retention pins.
    /// A wrong digest cannot remove a checkpoint created by another request.
    pub fn release_checkpoint(
        &mut self,
        scope: &ArtifactScope,
        checkpoint: CheckpointId,
        expected_digest: ContentHash,
    ) -> Result<bool, CheckpointError> {
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let Some((_, digest)) = read_checkpoint(&tx, scope, checkpoint)? else {
            return Ok(false);
        };
        if digest != expected_digest {
            return Err(CheckpointError::Conflict);
        }
        let changed = tx.execute(
            "DELETE FROM workspace_checkpoints WHERE checkpoint_id=?1 AND payload_hash=?2",
            params![checkpoint.to_string(), digest.to_hex()],
        )?;
        if changed != 1 {
            return Err(CheckpointError::Conflict);
        }
        tx.commit()?;
        Ok(true)
    }
}

fn write_workspace_graph(
    tx: &rusqlite::Transaction<'_>,
    scope: &ArtifactScope,
    expected_revision: u64,
    graph: &WorkspaceGraph,
    bytes: &[u8],
    now: UnixTimestampMicros,
) -> Result<GraphRevision, CheckpointError> {
    let next = expected_revision
        .checked_add(1)
        .ok_or(CheckpointError::Invalid("graph revision overflow"))?;
    let workspace = graph.workspace.workspace_id();
    let existing: Option<(String, i64)> = tx
        .query_row(
            "SELECT privacy_scope, revision FROM workspace_graphs WHERE workspace_id=?1",
            [workspace.to_string()],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?;
    if let Some((old_scope, revision)) = existing {
        if old_scope != scope.as_str() {
            return Err(CheckpointError::AccessDenied);
        }
        if unsigned(revision)? != expected_revision {
            return Err(CheckpointError::Conflict);
        }
    } else if expected_revision != 0 {
        return Err(CheckpointError::Conflict);
    }

    let task_ids: BTreeSet<_> = graph
        .tasks
        .iter()
        .map(|task| task.task_id().to_string())
        .collect();
    {
        let mut query = tx.prepare(
            "SELECT task_id FROM workspace_graph_tasks WHERE workspace_id=?1 AND active=1",
        )?;
        let previous =
            query.query_map([workspace.to_string()], |row| row.get::<_, String>(0))?;
        for previous in previous {
            let previous = previous?;
            if !task_ids.contains(&previous) {
                let used: bool = tx.query_row(
                    "SELECT EXISTS(SELECT 1 FROM durable_operations WHERE task_id=?1)",
                    [&previous],
                    |row| row.get(0),
                )?;
                if used {
                    return Err(CheckpointError::Invalid(
                        "cannot remove a task with durable operation history",
                    ));
                }
            }
        }
    }

    if expected_revision == 0 {
        tx.execute(
            "INSERT INTO workspace_graphs(workspace_id,privacy_scope,revision,graph,graph_hash,updated_at_micros) VALUES (?1,?2,?3,?4,?5,?6)",
            params![
                workspace.to_string(),
                scope.as_str(),
                sql(next)?,
                bytes,
                hash(bytes).to_hex(),
                now.get()
            ],
        )?;
    } else {
        let changed = tx.execute(
            "UPDATE workspace_graphs SET revision=?1,graph=?2,graph_hash=?3,updated_at_micros=?4 WHERE workspace_id=?5 AND revision=?6",
            params![
                sql(next)?,
                bytes,
                hash(bytes).to_hex(),
                now.get(),
                workspace.to_string(),
                sql(expected_revision)?
            ],
        )?;
        if changed != 1 {
            return Err(CheckpointError::Conflict);
        }
    }
    tx.execute(
        "UPDATE workspace_graph_tasks SET active=0 WHERE workspace_id=?1",
        [workspace.to_string()],
    )?;
    for task in &task_ids {
        let changed = tx.execute(
            "INSERT INTO workspace_graph_tasks(task_id,workspace_id,active) VALUES (?1,?2,1) ON CONFLICT(task_id) DO UPDATE SET active=1 WHERE workspace_graph_tasks.workspace_id=excluded.workspace_id",
            params![task, workspace.to_string()],
        )?;
        if changed != 1 {
            return Err(CheckpointError::Invalid(
                "task identity belongs to another workspace",
            ));
        }
    }
    Ok(GraphRevision {
        workspace_id: workspace,
        revision: next,
    })
}

fn validate_graph(graph: &WorkspaceGraph) -> Result<(), CheckpointError> {
    if graph.schema_version != SchemaVersion::V1 {
        return Err(CheckpointError::Invalid("unsupported graph schema"));
    }
    if graph.tasks.is_empty() || graph.tasks.len() > MAX_GRAP_TASKS
        || graph.dependencies.len() > MAX_GRAPH_DEPENDENCIES
        || graph.provider_references.len() > MAX_METADATA_ENTRIES
        || graph.worker_instances.len() > MAX_METADATA_ENTRIES
        || graph.cursors.len() > MAX_METADATA_ENTRIES
    {
        return Err(CheckpointError::Invalid(
            "graph collection budget exceeded",
        ));
    }
    let workspace = graph.workspace.workspace_id();
    let tasks: BTreeSet<_> = graph.tasks.iter().map(|t| t.task_id()).collect();
    if tasks.len() != graph.tasks.len() || graph.tasks.iter().any(t|t.workspace_id() != workspace) {
        return Err(CheckpointError::Invalid("duplicate or foreign task"));
    }
    for edge in &graph.dependencies {
        if edge.prerequisite == edge.dependent
            || !tasks.contains(&edge.prerequisite)
            || !tasks.contains(&edge.dependent)
        {
            return Err(CheckpointError::Invalid("invalid task dependency"));
        }
    }
    if has_cycle(&tasks, &graph.dependencies) {
        return Err(CheckpointError::Invalid("cyclic task graph"));
    }
    let mut goals = BTreeSet::new();
    for goal in &graph.goals {
        if !goals.insert(goal.goal_contract_id()) {
            return Err(CheckpointError::Invalid("duplicate goal"));
        }
    }
    if let Some(goal) = graph.workspace.goal_contract_id()
        && !goals.contains(&goal)
    {
        return Err(CheckpointError::Invalid(
            "workspace goal is missing from graph",
        ));
    }
    for task in &graph.tasks {
        if let Some(goal) = task.goal_contract_id()
            && !goals.contains(&goal)
        {
            return Err(CheckpointError::Invalid(
                "task goal is missing from graph",
            ));
        }
    }
    for cursor in &graph.cursors {
        if cursor.consumer.as_str().is_empty()
            || cursor.source.as_str().is_empty()
            || cursor.stream.as_str().is_empty()
        {
            return Err(CheckpointError::Invalid("empty cursor key"));
        }
    }
    let mut unique = BTreeSet::new();
    for reference in &graph.retained_artifacts {
        if !unique.insert(reference.artifact_id()) {
            return Err(CheckpointError::Invalid(
                "duplicate retained artifact",
            ));
        }
    }
    Ok(())
}
