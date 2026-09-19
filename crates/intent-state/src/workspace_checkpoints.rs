//! Atomic, bounded snapshots of runtime-owned graphs and their durable dependencies.
mod types;
pub use types::*;

use crate::{ArtifactScope, StateStore, snapshot_fs::Directory};
use intent_contracts::{
    ArtifactId, ArtifactReference, ContentHash, SchemaVersion, UnixTimestampMicros, WorkspaceId,
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
        let previous = query.query_map([workspace.to_string()], |row| row.get::<_, String>(0))?;
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
    if graph.tasks.is_empty()
        || graph.tasks.len() > MAX_GRAPH_TASKS
        || graph.dependencies.len() > MAX_GRAPH_DEPENDENCIES
        || graph.goals.len() > MAX_METADATA_ENTRIES
        || graph.cursors.len() > MAX_METADATA_ENTRIES
        || graph.provider_references.len() > MAX_METADATA_ENTRIES
        || graph.worker_instances.len() > MAX_GRAPH_TASKS
    {
        return Err(CheckpointError::Invalid("graph collection budget exceeded"));
    }
    let goals: BTreeSet<_> = graph.goals.iter().map(|g| g.goal_contract_id()).collect();
    if goals.len() != graph.goals.len()
        || graph
            .workspace
            .goal_contract_id()
            .is_some_and(|id| !goals.contains(&id))
    {
        return Err(CheckpointError::Invalid(
            "duplicate or missing goal contract",
        ));
    }
    let workspace = graph.workspace.workspace_id();
    let tasks: BTreeSet<_> = graph.tasks.iter().map(|t| t.task_id()).collect();
    if tasks.len() != graph.tasks.len()
        || graph.tasks.iter().any(|t| {
            t.workspace_id() != workspace
                || t.goal_contract_id().is_some_and(|id| !goals.contains(&id))
                || t.required_capabilities().len() > MAX_METADATA_ENTRIES
        })
    {
        return Err(CheckpointError::Invalid(
            "duplicate, foreign or invalid task node",
        ));
    }
    let mut indegree: BTreeMap<_, usize> = tasks.iter().map(|id| (*id, 0)).collect();
    let mut edges: BTreeMap<_, Vec<_>> = BTreeMap::new();
    let mut unique = BTreeSet::new();
    for edge in &graph.dependencies {
        if !tasks.contains(&edge.prerequisite)
            || !tasks.contains(&edge.dependent)
            || edge.prerequisite == edge.dependent
            || !unique.insert((edge.prerequisite, edge.dependent))
        {
            return Err(CheckpointError::Invalid("invalid or duplicate graph edge"));
        }
        *indegree
            .get_mut(&edge.dependent)
            .ok_or(CheckpointError::Invalid("missing graph node"))? += 1;
        edges
            .entry(edge.prerequisite)
            .or_default()
            .push(edge.dependent);
    }
    let mut ready: VecDeque<_> = indegree
        .iter()
        .filter(|(_, n)| **n == 0)
        .map(|(id, _)| *id)
        .collect();
    let mut visited = 0;
    while let Some(node) = ready.pop_front() {
        visited += 1;
        for dependent in edges.get(&node).into_iter().flatten() {
            let n = indegree
                .get_mut(dependent)
                .ok_or(CheckpointError::Invalid("missing dependent"))?;
            *n -= 1;
            if *n == 0 {
                ready.push_back(*dependent);
            }
        }
    }
    if visited != tasks.len() {
        return Err(CheckpointError::Invalid("cyclic task graph"));
    }
    let mut cursors = BTreeSet::new();
    for cursor in &graph.cursors {
        if [
            cursor.consumer.as_str(),
            cursor.source.as_str(),
            cursor.stream.as_str(),
        ]
        .iter()
        .any(|v| v.is_empty())
            || !cursors.insert(cursor.clone())
        {
            return Err(CheckpointError::Invalid(
                "empty or duplicate cursor binding",
            ));
        }
    }
    let mut providers = BTreeSet::new();
    for provider in &graph.provider_references {
        if provider.provider.as_str().is_empty()
            || !providers.insert((
                provider.provider.as_str(),
                provider.account_id,
                provider.session_reference.to_hex(),
            ))
        {
            return Err(CheckpointError::Invalid(
                "empty or duplicate provider reference",
            ));
        }
    }
    let workers: BTreeSet<_> = graph.worker_instances.iter().collect();
    if workers.len() != graph.worker_instances.len() {
        return Err(CheckpointError::Invalid("duplicate worker instance"));
    }
    graph_artifacts(graph)?;
    Ok(())
}

fn graph_artifacts(
    graph: &WorkspaceGraph,
) -> Result<BTreeMap<ArtifactId, ArtifactReference>, CheckpointError> {
    let count = graph
        .tasks
        .iter()
        .try_fold(graph.retained_artifacts.len(), |n, t| {
            n.checked_add(t.result_artifacts().len())
        })
        .ok_or(CheckpointError::Invalid("artifact counter overflow"))?;
    if count > MAX_CHECKPOINT_ARTIFACTS {
        return Err(CheckpointError::Invalid(
            "artifact collection budget exceeded",
        ));
    }
    let mut artifacts = BTreeMap::new();
    for item in graph
        .retained_artifacts
        .iter()
        .chain(graph.tasks.iter().flat_map(|t| t.result_artifacts()))
    {
        if let Some(existing) = artifacts.insert(item.artifact_id(), item.clone())
            && existing != *item
        {
            return Err(CheckpointError::Invalid("conflicting artifact references"));
        }
    }
    Ok(artifacts)
}

fn load_graph(
    connection: &Connection,
    scope: &ArtifactScope,
    workspace: WorkspaceId,
) -> Result<(u64, WorkspaceGraph), CheckpointError> {
    let (stored_scope, revision, bytes, digest): (String,i64,Vec<u8>,String) = connection.query_row(
        "SELECT privacy_scope,revision,graph,graph_hash FROM workspace_graphs WHERE workspace_id=?1", [workspace.to_string()],
        |r| {
            let bytes = r.get_ref(2)?.as_blob()?;
            if bytes.len() > MAX_GRAPH_BYTES { return Err(rusqlite::Error::InvalidQuery); }
            Ok((r.get(0)?,r.get(1)?,bytes.to_vec(),r.get(3)?))
        },
    ).optional()?.ok_or(CheckpointError::NotFound)?;
    if stored_scope != scope.as_str() {
        return Err(CheckpointError::AccessDenied);
    }
    if hash(&bytes).to_hex() != digest {
        return Err(CheckpointError::Invalid("workspace graph hash mismatch"));
    }
    let graph: WorkspaceGraph = serde_json::from_slice(&bytes)?;
    validate_graph(&graph)?;
    if encode_bounded(&graph, MAX_GRAPH_BYTES)? != bytes {
        return Err(CheckpointError::Invalid(
            "noncanonical or unsupported graph data",
        ));
    }
    if graph.workspace.workspace_id() != workspace {
        return Err(CheckpointError::Invalid("workspace identity mismatch"));
    }
    let mut query = connection.prepare("SELECT task_id FROM workspace_graph_tasks WHERE workspace_id=?1 AND active=1 ORDER BY task_id LIMIT ?2")?;
    let registered = query
        .query_map(
            params![
                workspace.to_string(),
                i64::try_from(MAX_GRAPH_TASKS + 1)
                    .map_err(|_| CheckpointError::Invalid("task bound"))?
            ],
            |r| r.get::<_, String>(0),
        )?
        .collect::<Result<Vec<_>, _>>()?;
    let expected: Vec<_> = graph
        .tasks
        .iter()
        .map(|t| t.task_id().to_string())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if registered != expected {
        return Err(CheckpointError::Invalid(
            "graph task registry is inconsistent",
        ));
    }
    Ok((unsigned(revision)?, graph))
}

fn capture_operations(
    connection: &Connection,
    workspace: WorkspaceId,
) -> Result<Vec<CapturedOperation>, CheckpointError> {
    let mut query = connection.prepare("SELECT o.operation_id FROM durable_operations o JOIN workspace_graph_tasks t ON t.task_id=o.task_id WHERE t.workspace_id=?1 AND t.active=1 ORDER BY o.operation_id LIMIT ?2")?;
    let ids = query.query_map(
        params![
            workspace.to_string(),
            i64::try_from(MAX_CHECKPOINT_OPERATIONS + 1)
                .map_err(|_| CheckpointError::Invalid("operation limit"))?
        ],
        |r| r.get::<_, String>(0),
    )?;
    let mut result = Vec::new();
    for id in ids {
        if result.len() >= MAX_CHECKPOINT_OPERATIONS {
            return Err(CheckpointError::Invalid(
                "checkpoint operation budget exceeded",
            ));
        }
        let id = id?
            .parse()
            .map_err(|_| CheckpointError::Invalid("invalid operation identity"))?;
        let operation = crate::operations::load_operation_from_connection(connection, id)?
            .ok_or(CheckpointError::Invalid("operation disappeared"))?;
        let (revision, state, attempt, sequence): (i64,String,Option<String>,i64) = connection.query_row(
            "SELECT revision,to_state,attempt_identity,sequence FROM operation_journal WHERE operation_id=?1 ORDER BY revision DESC LIMIT 1", [operation.operation_id().to_string()],
            |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?)),
        )?;
        if unsigned(revision)? != operation.revision()
            || state != operation.state().as_str()
            || attempt != operation.attempt_identity().map(|v| v.to_string())
        {
            return Err(CheckpointError::Invalid(
                "operation projection and journal disagree",
            ));
        }
        if operation.revision() >= 4096 {
            return Err(CheckpointError::Invalid(
                "journal validation budget exceeded",
            ));
        }
        let (count, minimum, maximum): (i64, i64, i64) = connection.query_row(
            "SELECT COUNT(*),COALESCE(MIN(revision),-1),COALESCE(MAX(revision),-1) FROM (SELECT revision FROM operation_journal WHERE operation_id=?1 ORDER BY revision LIMIT 4097)",
            [operation.operation_id().to_string()], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?)),
        )?;
        if count != sql(operation.revision() + 1)?
            || minimum != 0
            || maximum != sql(operation.revision())?
        {
            return Err(CheckpointError::Invalid(
                "operation journal has missing revisions",
            ));
        }
        result.push(CapturedOperation {
            action_proposal_id: operation.action_proposal_id(),
            source_schema: operation.source_schema(),
            state_detail: operation.state_detail().cloned(),
            created_at: operation.created_at(),
            updated_at: operation.updated_at(),
            operation_id: operation.operation_id(),
            task_id: operation.task_id(),
            account_id: operation.account_id(),
            capability_id: operation.capability_id(),
            arguments_hash: operation.arguments_hash(),
            state: operation.state(),
            attempt_identity: operation.attempt_identity(),
            revision: operation.revision(),
            journal_sequence: unsigned(sequence)?,
        });
    }
    Ok(result)
}

fn read_checkpoint(
    connection: &Connection,
    scope: &ArtifactScope,
    id: CheckpointId,
) -> Result<Option<(WorkspaceCheckpoint, ContentHash)>, CheckpointError> {
    let raw = connection.query_row("SELECT privacy_scope,workspace_id,graph_revision,created_at_micros,payload,payload_hash FROM workspace_checkpoints WHERE checkpoint_id=?1", [id.to_string()], |r| {
        let payload = r.get_ref(4)?.as_blob()?;
        if payload.len() > MAX_CHECKPOINT_BYTES { return Err(rusqlite::Error::InvalidQuery); }
        Ok((r.get::<_,String>(0)?,r.get::<_,String>(1)?,r.get::<_,i64>(2)?,r.get::<_,i64>(3)?,payload.to_vec(),r.get::<_,String>(5)?))
    }).optional()?;
    let Some((stored_scope, workspace, revision, created, payload, digest)) = raw else {
        return Ok(None);
    };
    if stored_scope != scope.as_str() {
        return Err(CheckpointError::AccessDenied);
    }
    let actual = hash(&payload);
    if actual.to_hex() != digest {
        return Err(CheckpointError::Invalid("checkpoint payload hash mismatch"));
    }
    let record: WorkspaceCheckpoint = serde_json::from_slice(&payload)?;
    if record.checkpoint_id.as_uuid().is_nil()
        || record.schema_version != SchemaVersion::V1
        || record.checkpoint_id != id
        || record.workspace_id.to_string() != workspace
        || record.privacy_scope.as_str() != stored_scope
        || record.graph_revision != unsigned(revision)?
        || record.created_at.get() != created
        || record.graph.workspace.workspace_id() != record.workspace_id
        || record.operations.len() > MAX_CHECKPOINT_OPERATIONS
        || record.cursors.len() > MAX_METADATA_ENTRIES
    {
        return Err(CheckpointError::Invalid(
            "checkpoint header or collection mismatch",
        ));
    }
    validate_graph(&record.graph)?;
    if encode_bounded(&record, MAX_CHECKPOINT_BYTES)? != payload {
        return Err(CheckpointError::Invalid(
            "noncanonical or unsupported checkpoint data",
        ));
    }
    let store: String = connection.query_row(
        "SELECT store_uuid FROM store_metadata WHERE singleton=1",
        [],
        |r| r.get(0),
    )?;
    if record.store_id.to_string() != store {
        return Err(CheckpointError::Invalid(
            "checkpoint belongs to another store",
        ));
    }
    let graph_tasks: BTreeSet<_> = record.graph.tasks.iter().map(|t| t.task_id()).collect();
    let mut operations = BTreeSet::new();
    for op in &record.operations {
        if !graph_tasks.contains(&op.task_id) || !operations.insert(op.operation_id) {
            return Err(CheckpointError::Invalid(
                "foreign or duplicate captured operation",
            ));
        }
    }
    let cursor_keys: BTreeSet<_> = record.cursors.iter().map(|c| &c.key).collect();
    if cursor_keys.len() != record.cursors.len()
        || cursor_keys != record.graph.cursors.iter().collect()
    {
        return Err(CheckpointError::Invalid(
            "captured cursor set differs from graph",
        ));
    }
    Ok(Some((record, actual)))
}

fn verify_dependencies(
    connection: &Connection,
    root: &Path,
    record: &WorkspaceCheckpoint,
) -> Result<(), CheckpointError> {
    let artifacts = graph_artifacts(&record.graph)?;
    let mut query = connection.prepare("SELECT artifact_id FROM workspace_checkpoint_pins WHERE checkpoint_id=?1 ORDER BY artifact_id LIMIT ?2")?;
    let pins = query
        .query_map(
            params![
                record.checkpoint_id.to_string(),
                i64::try_from(MAX_CHECKPOINT_ARTIFACTS + 1)
                    .map_err(|_| CheckpointError::Invalid("artifact limit"))?
            ],
            |r| r.get::<_, String>(0),
        )?
        .collect::<Result<Vec<_>, _>>()?;
    let expected: Vec<_> = artifacts.keys().map(ToString::to_string).collect();
    if pins != expected {
        return Err(CheckpointError::Invalid(
            "checkpoint pin set is missing or inconsistent",
        ));
    }
    for id in artifacts.keys() {
        let exists: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM artifact_references WHERE artifact_id=?1 AND reference_kind='core_checkpoint' AND reference_id=?2)", params![id.to_string(),record.checkpoint_id.to_string()], |r| r.get(0))?;
        if !exists {
            return Err(CheckpointError::Invalid("checkpoint reference is missing"));
        }
    }
    verify_artifacts(connection, root, record.privacy_scope.as_str(), &artifacts)
}

fn verify_artifacts(
    connection: &Connection,
    root: &Path,
    scope: &str,
    artifacts: &BTreeMap<ArtifactId, ArtifactReference>,
) -> Result<(), CheckpointError> {
    if artifacts.is_empty() {
        return Ok(());
    }
    let root = Directory::open_private(root)?
        .child("blobs")?
        .child(&hash(scope.as_bytes()).to_hex())?;
    let mut total = 0_u64;
    let mut verified = BTreeSet::new();
    for (id, reference) in artifacts {
        let metadata: Option<(String,String,i64,Option<i64>)> = connection.query_row("SELECT privacy_scope,content_hash,byte_size,suppressed_at_micros FROM artifact_handles_all WHERE artifact_id=?1", [id.to_string()], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?))).optional()?;
        let (actual_scope, digest, bytes, suppressed) =
            metadata.ok_or(CheckpointError::MissingArtifact(*id))?;
        if actual_scope != scope {
            return Err(CheckpointError::AccessDenied);
        }
        if suppressed.is_some() {
            return Err(CheckpointError::SuppressedArtifact(*id));
        }
        let bytes = unsigned(bytes)?;
        if reference.content_hash().to_hex() != digest || reference.byte_size().as_bytes() != bytes
        {
            return Err(CheckpointError::CorruptArtifact(*id));
        }
        if !verified.insert(digest.clone()) {
            continue;
        }
        total = total
            .checked_add(bytes)
            .ok_or(CheckpointError::Invalid("checkpoint blob size overflow"))?;
        if total > MAX_CHECKPOINT_BLOB_BYTES {
            return Err(CheckpointError::Invalid(
                "checkpoint blob byte budget exceeded",
            ));
        }
        let file = root.open_file(&digest).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                CheckpointError::MissingArtifact(*id)
            } else {
                e.into()
            }
        })?;
        if file.metadata()?.len() != bytes {
            return Err(CheckpointError::CorruptArtifact(*id));
        }
        let mut limited = file.take(bytes + 1);
        let mut hasher = Sha256::new();
        let mut read = 0_u64;
        let mut buffer = [0_u8; 65536];
        loop {
            let n = limited.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            read += u64::try_from(n).map_err(|_| CheckpointError::Invalid("read size overflow"))?;
            hasher.update(&buffer[..n]);
        }
        let actual = ContentHash::from_bytes(hasher.finalize().into());
        if read != bytes || actual.to_hex() != digest {
            return Err(CheckpointError::CorruptArtifact(*id));
        }
    }
    Ok(())
}

fn receipt(
    record: &WorkspaceCheckpoint,
    digest: ContentHash,
) -> Result<CheckpointReceipt, CheckpointError> {
    Ok(CheckpointReceipt {
        checkpoint_id: record.checkpoint_id,
        workspace_id: record.workspace_id,
        graph_revision: record.graph_revision,
        digest,
        artifact_count: graph_artifacts(&record.graph)?.len(),
    })
}
fn hash(bytes: &[u8]) -> ContentHash {
    ContentHash::from_bytes(Sha256::digest(bytes).into())
}
fn sql(value: u64) -> Result<i64, CheckpointError> {
    i64::try_from(value).map_err(|_| CheckpointError::Invalid("integer exceeds SQLite range"))
}
fn unsigned(value: i64) -> Result<u64, CheckpointError> {
    u64::try_from(value).map_err(|_| CheckpointError::Invalid("negative durable counter"))
}
fn parse_uuid(value: &str) -> Result<Uuid, CheckpointError> {
    Uuid::parse_str(value).map_err(|_| CheckpointError::Invalid("invalid stored UUID"))
}
fn encode_bounded<T: Serialize>(value: &T, limit: usize) -> Result<Vec<u8>, CheckpointError> {
    struct Writer {
        bytes: Vec<u8>,
        limit: usize,
        exceeded: bool,
    }
    impl Write for Writer {
        fn write(&mut self, input: &[u8]) -> std::io::Result<usize> {
            if input.len() > self.limit.saturating_sub(self.bytes.len()) {
                self.exceeded = true;
                return Err(std::io::Error::other("checkpoint encoding budget exceeded"));
            }
            self.bytes
                .try_reserve(input.len())
                .map_err(|_| std::io::Error::other("checkpoint allocation failed"))?;
            self.bytes.extend_from_slice(input);
            Ok(input.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut writer = Writer {
        bytes: Vec::new(),
        limit,
        exceeded: false,
    };
    let result = serde_json::to_writer(&mut writer, value);
    if writer.exceeded {
        return Err(CheckpointError::Invalid(
            "encoded checkpoint budget exceeded",
        ));
    }
    result?;
    Ok(writer.bytes)
}

#[cfg(test)]
mod tests;
