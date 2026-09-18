//! Immutable task checkpoints. A checkpoint is historical data, never a grant.

use crate::{ArtifactError, ArtifactScope, StateError, StateStore, artifacts};
use intent_contracts::{
    AccountId, ActionProposalId, ArtifactId, BoundedText, CapabilityId, CheckpointId, ContentHash,
    GoalContract, OperationAttemptId, OperationId, ProviderId, SchemaVersion, Task, TaskId,
    TaskState, UnixTimestampMicros, WorkerInstanceId, Workspace, WorkspaceId,
};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt, io,
    path::Path,
};

const MAX_CHECKPOINT_BYTES: usize = 1024 * 1024;
const MAX_GRAPH_NODES: usize = 256;
const MAX_ARTIFACTS: usize = 256;
const MAX_OPERATIONS: usize = 256;
const MAX_CURSORS: usize = 64;
const MAX_PROVIDER_REFERENCES: usize = 32;
const MAX_DEPENDENCIES: usize = 64;
const MAX_TOTAL_ARTIFACT_BYTES: u64 = 256 * 1024 * 1024;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointNode {
    pub id: u32,
    pub state: TaskState,
    pub intent: BoundedText<4096>,
    pub depends_on: Vec<u32>,
    pub outputs: Vec<ArtifactId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CursorKey {
    pub consumer: BoundedText<128>,
    pub source: BoundedText<128>,
    pub stream: BoundedText<128>,
}

/// The reference identifies runtime-owned session metadata, not a bearer credential.
/// Actual credentials and private provider state are deliberately not representable.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderCheckpointReference {
    pub provider: ProviderId,
    pub account: AccountId,
    pub metadata_reference: ContentHash,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointGraph {
    pub workspace: Workspace,
    pub task: Task,
    pub goal: Option<GoalContract>,
    pub nodes: Vec<CheckpointNode>,
    pub artifact_ids: Vec<ArtifactId>,
    pub cursor_keys: Vec<CursorKey>,
    pub provider_references: Vec<ProviderCheckpointReference>,
    pub worker_epoch: Option<WorkerInstanceId>,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointRequest {
    pub checkpoint_id: CheckpointId,
    pub privacy_scope: BoundedText<128>,
    pub expected_graph_revision: Option<u64>,
    pub graph: CheckpointGraph,
    pub created_at: UnixTimestampMicros,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointCursor {
    pub key: CursorKey,
    pub sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointOperation {
    pub operation_id: OperationId,
    pub action_proposal_id: ActionProposalId,
    pub account_id: AccountId,
    pub capability_id: CapabilityId,
    pub arguments_hash: ContentHash,
    pub source_schema: SchemaVersion,
    pub revision: u64,
    pub state: BoundedText<32>,
    pub attempt_identity: Option<OperationAttemptId>,
    pub updated_at: UnixTimestampMicros,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckpointArtifact {
    artifact_id: ArtifactId,
    content_hash: ContentHash,
    byte_size: u64,
    media_type: BoundedText<255>,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckpointDocument {
    schema_version: u16,
    checkpoint_id: CheckpointId,
    task_id: TaskId,
    workspace_id: WorkspaceId,
    privacy_scope: BoundedText<128>,
    graph_revision: u64,
    predecessor: Option<CheckpointId>,
    request_hash: ContentHash,
    graph: CheckpointGraph,
    artifacts: Vec<CheckpointArtifact>,
    operations: Vec<CheckpointOperation>,
    cursors: Vec<CheckpointCursor>,
    journal_watermark: u64,
    created_at: UnixTimestampMicros,
}

/// Verified historical data returned only by the state store, not deserializable
/// as a trusted checkpoint by an IPC peer.
///
/// ```compile_fail
/// let _: intent_state::StoredCheckpoint = serde_json::from_str("{}").unwrap();
/// ```
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct StoredCheckpoint {
    document: CheckpointDocument,
}

impl fmt::Debug for StoredCheckpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StoredCheckpoint")
            .field("checkpoint_id", &self.document.checkpoint_id)
            .field("task_id", &self.document.task_id)
            .field("graph_revision", &self.document.graph_revision)
            .field("artifact_count", &self.document.artifacts.len())
            .field("operation_count", &self.document.operations.len())
            .field("content", &"[REDACTED]")
            .finish()
    }
}

impl StoredCheckpoint {
    #[must_use]
    pub const fn id(&self) -> CheckpointId {
        self.document.checkpoint_id
    }
    #[must_use]
    pub const fn task_id(&self) -> TaskId {
        self.document.task_id
    }
    #[must_use]
    pub const fn graph_revision(&self) -> u64 {
        self.document.graph_revision
    }
    #[must_use]
    pub const fn predecessor(&self) -> Option<CheckpointId> {
        self.document.predecessor
    }
    #[must_use]
    pub const fn graph(&self) -> &CheckpointGraph {
        &self.document.graph
    }
    #[must_use]
    pub fn operations(&self) -> &[CheckpointOperation] {
        &self.document.operations
    }
    #[must_use]
    pub fn cursors(&self) -> &[CheckpointCursor] {
        &self.document.cursors
    }
    #[must_use]
    pub const fn journal_watermark(&self) -> u64 {
        self.document.journal_watermark
    }
    #[must_use]
    pub const fn grants_execution_authority(&self) -> bool {
        false
    }
}

impl StateStore {
    /// Commits graph, database observations and retention pins under one writer
    /// boundary. The caller supplies already-authorized task data, not authority.
    /// A repeated checkpoint ID must repeat the exact original request.
    pub fn save_task_checkpoint(
        &mut self,
        artifact_root: &Path,
        request: CheckpointRequest,
    ) -> Result<StoredCheckpoint, CheckpointError> {
        self.save_checkpoint_inner(artifact_root, request, || Ok(()))
    }

    fn save_checkpoint_inner(
        &mut self,
        artifact_root: &Path,
        request: CheckpointRequest,
        before_commit: impl FnOnce() -> Result<(), CheckpointError>,
    ) -> Result<StoredCheckpoint, CheckpointError> {
        validate_request(&request)?;
        let request_hash = hash(&bounded_json(&request)?);
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let existing: Option<String> = transaction
            .query_row(
                "SELECT request_hash FROM task_checkpoints WHERE checkpoint_id = ?1",
                [request.checkpoint_id.to_string()],
                |r| r.get(0),
            )
            .optional()?;
        if let Some(existing) = existing {
            if existing != request_hash.to_hex() {
                return Err(CheckpointError::IdentityConflict);
            }
            return load_checkpoint(
                &transaction,
                artifact_root,
                request.checkpoint_id,
                request.privacy_scope.as_str(),
            );
        }
        let head: Option<(String, String, String, i64)> = transaction.query_row(
            "SELECT workspace_id, privacy_scope, checkpoint_id, graph_revision FROM task_checkpoint_heads WHERE task_id = ?1",
            [request.graph.task.id().to_string()], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        ).optional()?;
        let (revision, predecessor) = match head {
            None if request.expected_graph_revision.is_none() => (0, None),
            Some((workspace, scope, previous, revision)) => {
                if workspace != request.graph.workspace.id().to_string()
                    || scope != request.privacy_scope.as_str()
                {
                    return Err(CheckpointError::ScopeMismatch);
                }
                let valid_head: bool = transaction.query_row(
                    "SELECT EXISTS(SELECT 1 FROM task_checkpoints WHERE checkpoint_id=?1 AND task_id=?2 AND workspace_id=?3 AND privacy_scope=?4 AND graph_revision=?5 AND retired=0)",
                    params![previous, request.graph.task.id().to_string(), workspace, scope, revision], |r| r.get(0),
                )?;
                if !valid_head {
                    return Err(CheckpointError::Corrupt);
                }
                let revision = nonnegative(revision)?;
                if Some(revision) != request.expected_graph_revision {
                    return Err(CheckpointError::StaleRevision);
                }
                (
                    revision.checked_add(1).ok_or(CheckpointError::Capacity)?,
                    Some(parse_id(&previous)?),
                )
            }
            _ => return Err(CheckpointError::StaleRevision),
        };
        let artifacts = capture_artifacts(
            &transaction,
            artifact_root,
            &request.graph,
            request.privacy_scope.as_str(),
        )?;
        let operations = capture_operations(&transaction, request.graph.task.id())?;
        let cursors = capture_cursors(&transaction, &request.graph.cursor_keys)?;
        let watermark: i64 = transaction.query_row(
            "SELECT COALESCE(MAX(sequence), 0) FROM operation_journal",
            [],
            |r| r.get(0),
        )?;
        let checkpoint = CheckpointDocument {
            schema_version: 1,
            checkpoint_id: request.checkpoint_id,
            task_id: request.graph.task.id(),
            workspace_id: request.graph.workspace.id(),
            privacy_scope: request.privacy_scope,
            graph_revision: revision,
            predecessor,
            request_hash,
            graph: request.graph,
            artifacts,
            operations,
            cursors,
            journal_watermark: nonnegative(watermark)?,
            created_at: request.created_at,
        };
        let payload = bounded_json(&checkpoint)?;
        transaction.execute(
            "INSERT INTO task_checkpoints(checkpoint_id,task_id,workspace_id,privacy_scope,graph_revision,predecessor,request_hash,payload_hash,payload,journal_watermark,created_at_micros) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
            params![checkpoint.checkpoint_id.to_string(), checkpoint.task_id.to_string(), checkpoint.workspace_id.to_string(),
                checkpoint.privacy_scope.as_str(), sql_integer(revision)?, predecessor.map(|id| id.to_string()),
                request_hash.to_hex(), hash(&payload).to_hex(), payload, watermark, checkpoint.created_at.get()],
        )?;
        for artifact in &checkpoint.artifacts {
            transaction.execute(
                "INSERT INTO checkpoint_artifacts(checkpoint_id,artifact_id) VALUES (?1,?2)",
                params![
                    checkpoint.checkpoint_id.to_string(),
                    artifact.artifact_id.to_string()
                ],
            )?;
        }
        transaction.execute(
            "INSERT INTO task_checkpoint_heads(task_id,workspace_id,privacy_scope,checkpoint_id,graph_revision) VALUES (?1,?2,?3,?4,?5) ON CONFLICT(task_id) DO UPDATE SET checkpoint_id=excluded.checkpoint_id,graph_revision=excluded.graph_revision",
            params![checkpoint.task_id.to_string(), checkpoint.workspace_id.to_string(), checkpoint.privacy_scope.as_str(),
                checkpoint.checkpoint_id.to_string(), sql_integer(revision)?],
        )?;
        before_commit()?;
        transaction.commit()?;
        Ok(StoredCheckpoint {
            document: checkpoint,
        })
    }

    /// Resolves every declared dependency from a single read snapshot, verifies
    /// bytes, and returns historical data without changing any live state.
    pub fn load_task_checkpoint(
        &self,
        artifact_root: &Path,
        checkpoint_id: CheckpointId,
        permitted_scope: &ArtifactScope,
    ) -> Result<StoredCheckpoint, CheckpointError> {
        // Writer exclusion also protects against a concurrent checkpoint retirement
        // followed by GC while file verification is in progress.
        let transaction =
            rusqlite::Transaction::new_unchecked(&self.connection, TransactionBehavior::Immediate)?;
        let checkpoint = load_checkpoint(
            &transaction,
            artifact_root,
            checkpoint_id,
            permitted_scope.as_str(),
        )?;
        transaction.commit()?;
        Ok(checkpoint)
    }

    pub fn retire_task_checkpoint(
        &mut self,
        checkpoint_id: CheckpointId,
        permitted_scope: &ArtifactScope,
        expected_head_revision: u64,
    ) -> Result<(), CheckpointError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let row: Option<(String, String, bool)> = transaction.query_row(
            "SELECT task_id,privacy_scope,retired FROM task_checkpoints WHERE checkpoint_id = ?1",
            [checkpoint_id.to_string()], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?)),
        ).optional()?;
        let (task_id, scope, retired) = row.ok_or(CheckpointError::NotFound)?;
        if scope != permitted_scope.as_str() {
            return Err(CheckpointError::ScopeMismatch);
        }
        let (head, revision): (String, i64) = transaction.query_row(
            "SELECT checkpoint_id,graph_revision FROM task_checkpoint_heads WHERE task_id=?1",
            [task_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )?;
        if nonnegative(revision)? != expected_head_revision {
            return Err(CheckpointError::StaleRevision);
        }
        if head == checkpoint_id.to_string() {
            return Err(CheckpointError::CurrentCheckpoint);
        }
        if !retired {
            transaction.execute(
                "UPDATE task_checkpoints SET retired=1 WHERE checkpoint_id=?1",
                [checkpoint_id.to_string()],
            )?;
        }
        transaction.commit()?;
        Ok(())
    }
}

fn validate_request(request: &CheckpointRequest) -> Result<(), CheckpointError> {
    if request.checkpoint_id.as_uuid().is_nil() || request.privacy_scope.as_str().is_empty() {
        return Err(CheckpointError::Invalid(
            "checkpoint identity/scope is empty",
        ));
    }
    if let Some(revision) = request.expected_graph_revision {
        sql_integer(revision)?;
    }
    validate_graph(&request.graph)
}

fn validate_graph(graph: &CheckpointGraph) -> Result<(), CheckpointError> {
    if graph.task.id().as_uuid().is_nil()
        || graph.workspace.id().as_uuid().is_nil()
        || graph.task.workspace_id() != graph.workspace.id()
    {
        return Err(CheckpointError::ScopeMismatch);
    }
    if graph.nodes.len() > MAX_GRAPH_NODES
        || graph.artifact_ids.len() > MAX_ARTIFACTS
        || graph.task.result_artifacts().len() > MAX_ARTIFACTS
        || graph.task.required_capabilities().len() > MAX_GRAPH_NODES
        || graph.cursor_keys.len() > MAX_CURSORS
        || graph.provider_references.len() > MAX_PROVIDER_REFERENCES
    {
        return Err(CheckpointError::Capacity);
    }
    let goal_id = graph.goal.as_ref().map(GoalContract::id);
    if [
        graph.task.goal_contract_id(),
        graph.workspace.goal_contract_id(),
    ]
    .into_iter()
    .flatten()
    .any(|id| Some(id) != goal_id)
    {
        return Err(CheckpointError::Invalid(
            "declared goal is missing or inconsistent",
        ));
    }
    if let Some(goal) = &graph.goal
        && (goal.constraints().len() > 64 || goal.authorized_accounts().len() > 64)
    {
        return Err(CheckpointError::Capacity);
    }
    if graph.worker_epoch.is_some_and(|id| id.as_uuid().is_nil()) {
        return Err(CheckpointError::Invalid("nil worker epoch"));
    }
    let artifacts: BTreeSet<_> = graph.artifact_ids.iter().copied().collect();
    if artifacts.len() != graph.artifact_ids.len()
        || artifacts.iter().any(|id| id.as_uuid().is_nil())
    {
        return Err(CheckpointError::Invalid(
            "duplicate or nil artifact identity",
        ));
    }
    for result in graph.task.result_artifacts() {
        if !artifacts.contains(&result.artifact_id()) {
            return Err(CheckpointError::Invalid(
                "task result artifact is not declared",
            ));
        }
    }
    let mut dependency_counts = BTreeMap::new();
    let mut dependents: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for node in &graph.nodes {
        if node.id == 0
            || dependency_counts
                .insert(node.id, node.depends_on.len())
                .is_some()
        {
            return Err(CheckpointError::Invalid("duplicate or zero graph node"));
        }
        if node.depends_on.len() > MAX_DEPENDENCIES || node.outputs.len() > MAX_ARTIFACTS {
            return Err(CheckpointError::Capacity);
        }
        if node.depends_on.iter().collect::<BTreeSet<_>>().len() != node.depends_on.len()
            || node.outputs.iter().collect::<BTreeSet<_>>().len() != node.outputs.len()
            || node.outputs.iter().any(|id| !artifacts.contains(id))
        {
            return Err(CheckpointError::Invalid(
                "duplicate or undeclared node dependency",
            ));
        }
        for dependency in &node.depends_on {
            dependents.entry(*dependency).or_default().push(node.id);
        }
    }
    if dependents
        .keys()
        .any(|id| !dependency_counts.contains_key(id))
    {
        return Err(CheckpointError::Invalid("graph dependency does not exist"));
    }
    let mut ready: Vec<_> = dependency_counts
        .iter()
        .filter_map(|(id, count)| (*count == 0).then_some(*id))
        .collect();
    let mut visited = 0;
    while let Some(id) = ready.pop() {
        visited += 1;
        for dependent in dependents.get(&id).into_iter().flatten() {
            let count = dependency_counts
                .get_mut(dependent)
                .ok_or(CheckpointError::Corrupt)?;
            *count -= 1;
            if *count == 0 {
                ready.push(*dependent);
            }
        }
    }
    if visited != graph.nodes.len() {
        return Err(CheckpointError::Invalid("cyclic task graph"));
    }
    if graph.cursor_keys.iter().collect::<BTreeSet<_>>().len() != graph.cursor_keys.len()
        || graph.cursor_keys.iter().any(|key| {
            [
                key.consumer.as_str(),
                key.source.as_str(),
                key.stream.as_str(),
            ]
            .contains(&"")
        })
    {
        return Err(CheckpointError::Invalid("empty or duplicate cursor key"));
    }
    let mut sessions = BTreeSet::new();
    for session in &graph.provider_references {
        if session.provider.as_str().is_empty()
            || session.account.as_uuid().is_nil()
            || !sessions.insert((session.provider.as_str(), session.account))
        {
            return Err(CheckpointError::Invalid(
                "nil account or duplicate provider session",
            ));
        }
    }
    Ok(())
}

fn capture_artifacts(
    connection: &Connection,
    root: &Path,
    graph: &CheckpointGraph,
    scope: &str,
) -> Result<Vec<CheckpointArtifact>, CheckpointError> {
    let mut captured = Vec::with_capacity(graph.artifact_ids.len());
    let mut total = 0_u64;
    let ids: BTreeSet<_> = graph.artifact_ids.iter().copied().collect();
    for id in ids {
        let live: bool = connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM artifact_handles_all WHERE artifact_id=?1 AND privacy_scope=?2 AND suppressed_at_micros IS NULL)",
            params![id.to_string(), scope], |r| r.get(0),
        )?;
        if !live {
            return Err(CheckpointError::ArtifactUnavailable(id));
        }
        let metadata = artifacts::load_artifact_metadata(connection, id)?
            .ok_or(CheckpointError::ArtifactUnavailable(id))?;
        total = total
            .checked_add(metadata.byte_size())
            .ok_or(CheckpointError::Capacity)?;
        if total > MAX_TOTAL_ARTIFACT_BYTES {
            return Err(CheckpointError::Capacity);
        }
        artifacts::verify_blob(root, &metadata)?;
        for result in graph
            .task
            .result_artifacts()
            .iter()
            .filter(|result| result.artifact_id() == id)
        {
            if result.content_hash() != metadata.content_hash()
                || result.byte_size().as_bytes() != metadata.byte_size()
                || result.media_type() != metadata.media_type().as_str()
            {
                return Err(CheckpointError::Invalid(
                    "task result descriptor differs from immutable artifact",
                ));
            }
        }
        captured.push(CheckpointArtifact {
            artifact_id: id,
            content_hash: metadata.content_hash(),
            byte_size: metadata.byte_size(),
            media_type: metadata.media_type().clone(),
        });
    }
    Ok(captured)
}

fn capture_operations(
    connection: &Connection,
    task: TaskId,
) -> Result<Vec<CheckpointOperation>, CheckpointError> {
    let mut statement = connection.prepare("SELECT operation_id FROM durable_operations WHERE task_id=?1 ORDER BY operation_id LIMIT ?2")?;
    let ids = statement
        .query_map(params![task.to_string(), MAX_OPERATIONS as i64 + 1], |r| {
            r.get::<_, String>(0)
        })?
        .collect::<Result<Vec<_>, _>>()?;
    if ids.len() > MAX_OPERATIONS {
        return Err(CheckpointError::Capacity);
    }
    ids.into_iter()
        .map(|id| {
            let operation =
                crate::operations::load_operation_from_connection(connection, parse_id(&id)?)?
                    .ok_or(CheckpointError::Corrupt)?;
            Ok(CheckpointOperation {
                operation_id: operation.operation_id(),
                action_proposal_id: operation.action_proposal_id(),
                account_id: operation.account_id(),
                capability_id: operation.capability_id(),
                arguments_hash: operation.arguments_hash(),
                source_schema: operation.source_schema(),
                revision: operation.revision(),
                state: BoundedText::try_new(operation.state().as_str())
                    .map_err(|_| CheckpointError::Corrupt)?,
                attempt_identity: operation.attempt_identity(),
                updated_at: operation.updated_at(),
            })
        })
        .collect()
}

fn capture_cursors(
    connection: &Connection,
    keys: &[CursorKey],
) -> Result<Vec<CheckpointCursor>, CheckpointError> {
    let mut keys = keys.to_vec();
    keys.sort();
    keys.into_iter().map(|key| {
        let value: Option<i64> = connection.query_row(
            "SELECT last_sequence FROM consumer_cursors WHERE consumer=?1 AND source=?2 AND stream=?3",
            params![key.consumer.as_str(),key.source.as_str(),key.stream.as_str()], |r| r.get(0),
        ).optional()?;
        Ok(CheckpointCursor { key, sequence: nonnegative(value.ok_or(CheckpointError::CursorUnavailable)?)? })
    }).collect()
}

fn load_checkpoint(
    connection: &Connection,
    root: &Path,
    id: CheckpointId,
    scope: &str,
) -> Result<StoredCheckpoint, CheckpointError> {
    type Header = (
        String,
        String,
        String,
        i64,
        Option<String>,
        String,
        String,
        Option<Vec<u8>>,
        i64,
        i64,
        bool,
    );
    let row: Option<Header> = connection.query_row(
        "SELECT task_id,workspace_id,privacy_scope,graph_revision,predecessor,request_hash,payload_hash,CASE WHEN length(payload)<=1048576 THEN payload ELSE NULL END,journal_watermark,created_at_micros,retired FROM task_checkpoints WHERE checkpoint_id=?1",
        [id.to_string()], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?,r.get(5)?,r.get(6)?,r.get(7)?,r.get(8)?,r.get(9)?,r.get(10)?)),
    ).optional()?;
    let (
        task,
        workspace,
        stored_scope,
        revision,
        predecessor,
        request_hash,
        payload_hash,
        payload,
        watermark,
        created_at,
        retired,
    ) = row.ok_or(CheckpointError::NotFound)?;
    if stored_scope != scope {
        return Err(CheckpointError::ScopeMismatch);
    }
    if retired {
        return Err(CheckpointError::Retired);
    }
    let payload = payload.ok_or(CheckpointError::Capacity)?;
    if hash(&payload).to_hex() != payload_hash {
        return Err(CheckpointError::Corrupt);
    }
    let checkpoint: CheckpointDocument = serde_json::from_slice(&payload)?;
    if checkpoint.schema_version != 1
        || checkpoint.checkpoint_id != id
        || checkpoint.task_id.to_string() != task
        || checkpoint.workspace_id.to_string() != workspace
        || checkpoint.privacy_scope.as_str() != scope
        || checkpoint.graph_revision != nonnegative(revision)?
        || checkpoint.predecessor.map(|id| id.to_string()) != predecessor
        || checkpoint.request_hash.to_hex() != request_hash
        || checkpoint.journal_watermark != nonnegative(watermark)?
        || checkpoint.created_at.get() != created_at
        || checkpoint.graph.task.id() != checkpoint.task_id
        || checkpoint.graph.workspace.id() != checkpoint.workspace_id
    {
        return Err(CheckpointError::Corrupt);
    }
    validate_graph(&checkpoint.graph)?;
    let original_request = CheckpointRequest {
        checkpoint_id: checkpoint.checkpoint_id,
        privacy_scope: checkpoint.privacy_scope.clone(),
        expected_graph_revision: checkpoint.graph_revision.checked_sub(1),
        graph: checkpoint.graph.clone(),
        created_at: checkpoint.created_at,
    };
    if hash(&bounded_json(&original_request)?) != checkpoint.request_hash {
        return Err(CheckpointError::Corrupt);
    }
    if checkpoint.artifacts.len() > MAX_ARTIFACTS
        || checkpoint.operations.len() > MAX_OPERATIONS
        || checkpoint.cursors.len() > MAX_CURSORS
    {
        return Err(CheckpointError::Capacity);
    }
    let verified = capture_artifacts(connection, root, &checkpoint.graph, scope)?;
    if checkpoint.artifacts != verified {
        return Err(CheckpointError::Corrupt);
    }
    let mut pins = connection.prepare(
        "SELECT p.artifact_id FROM checkpoint_artifacts p JOIN artifact_references r ON r.artifact_id=p.artifact_id AND r.reference_kind='runtime_checkpoint' AND r.reference_id=p.checkpoint_id WHERE p.checkpoint_id=?1 ORDER BY p.artifact_id LIMIT 257",
    )?;
    let pinned = pins
        .query_map([id.to_string()], |r| r.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    if pinned
        != checkpoint
            .artifacts
            .iter()
            .map(|a| a.artifact_id.to_string())
            .collect::<Vec<_>>()
    {
        return Err(CheckpointError::Corrupt);
    }
    let current_cursors = capture_cursors(connection, &checkpoint.graph.cursor_keys)?;
    if checkpoint.cursors.len() != current_cursors.len()
        || checkpoint
            .cursors
            .iter()
            .zip(current_cursors)
            .any(|(old, now)| old.key != now.key || old.sequence > now.sequence)
    {
        return Err(CheckpointError::Corrupt);
    }
    let mut operation_ids = connection.prepare(
        "SELECT DISTINCT o.operation_id FROM durable_operations o JOIN operation_journal j USING(operation_id) WHERE o.task_id=?1 AND j.sequence<=?2 ORDER BY o.operation_id LIMIT 257",
    )?;
    let expected_ids = operation_ids
        .query_map(params![task, watermark], |r| r.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    if expected_ids
        != checkpoint
            .operations
            .iter()
            .map(|o| o.operation_id.to_string())
            .collect::<Vec<_>>()
    {
        return Err(CheckpointError::Corrupt);
    }
    let mut seen = BTreeSet::new();
    for operation in &checkpoint.operations {
        if !seen.insert(operation.operation_id) {
            return Err(CheckpointError::Corrupt);
        }
        let current =
            crate::operations::load_operation_from_connection(connection, operation.operation_id)?
                .ok_or(CheckpointError::Corrupt)?;
        if current.task_id() != checkpoint.task_id
            || current.action_proposal_id() != operation.action_proposal_id
            || current.account_id() != operation.account_id
            || current.capability_id() != operation.capability_id
            || current.arguments_hash() != operation.arguments_hash
            || current.source_schema() != operation.source_schema
            || current.revision() < operation.revision
        {
            return Err(CheckpointError::Corrupt);
        }
        let captured_revision: i64 = connection.query_row(
            "SELECT MAX(revision) FROM operation_journal WHERE operation_id=?1 AND sequence<=?2",
            params![operation.operation_id.to_string(), watermark],
            |r| r.get(0),
        )?;
        if nonnegative(captured_revision)? != operation.revision {
            return Err(CheckpointError::Corrupt);
        }
        let journal: Option<(String, Option<String>, i64, i64)> = connection.query_row(
            "SELECT to_state,attempt_identity,occurred_at_micros,sequence FROM operation_journal WHERE operation_id=?1 AND revision=?2",
            params![operation.operation_id.to_string(),sql_integer(operation.revision)?], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?)),
        ).optional()?;
        let Some((state, attempt, time, sequence)) = journal else {
            return Err(CheckpointError::Corrupt);
        };
        if state != operation.state.as_str()
            || attempt != operation.attempt_identity.map(|a| a.to_string())
            || time != operation.updated_at.get()
            || nonnegative(sequence)? > checkpoint.journal_watermark
        {
            return Err(CheckpointError::Corrupt);
        }
    }
    Ok(StoredCheckpoint {
        document: checkpoint,
    })
}

fn bounded_json(value: &impl Serialize) -> Result<Vec<u8>, CheckpointError> {
    struct Writer {
        bytes: Vec<u8>,
        exceeded: bool,
    }
    impl io::Write for Writer {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if bytes.len() > MAX_CHECKPOINT_BYTES.saturating_sub(self.bytes.len()) {
                self.exceeded = true;
                return Err(io::Error::other("checkpoint byte budget exceeded"));
            }
            self.bytes
                .try_reserve(bytes.len())
                .map_err(|_| io::Error::other("checkpoint allocation failed"))?;
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let mut writer = Writer {
        bytes: Vec::new(),
        exceeded: false,
    };
    let result = serde_json::to_writer(&mut writer, value);
    if writer.exceeded {
        return Err(CheckpointError::Capacity);
    }
    result?;
    Ok(writer.bytes)
}

fn hash(bytes: &[u8]) -> ContentHash {
    ContentHash::from_bytes(Sha256::digest(bytes).into())
}
fn nonnegative(value: i64) -> Result<u64, CheckpointError> {
    value.try_into().map_err(|_| CheckpointError::Corrupt)
}
fn sql_integer(value: u64) -> Result<i64, CheckpointError> {
    value.try_into().map_err(|_| CheckpointError::Capacity)
}
fn parse_id<T: std::str::FromStr>(value: &str) -> Result<T, CheckpointError> {
    value.parse().map_err(|_| CheckpointError::Corrupt)
}

#[derive(Debug)]
pub enum CheckpointError {
    Sqlite(rusqlite::Error),
    State(StateError),
    Artifact(ArtifactError),
    Json(serde_json::Error),
    Invalid(&'static str),
    Capacity,
    ScopeMismatch,
    StaleRevision,
    IdentityConflict,
    NotFound,
    Retired,
    CurrentCheckpoint,
    CursorUnavailable,
    ArtifactUnavailable(ArtifactId),
    Corrupt,
}
impl fmt::Display for CheckpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "checkpoint rejected: {self:?}")
    }
}
impl Error for CheckpointError {}
impl From<rusqlite::Error> for CheckpointError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Sqlite(e)
    }
}
impl From<StateError> for CheckpointError {
    fn from(e: StateError) -> Self {
        Self::State(e)
    }
}
impl From<ArtifactError> for CheckpointError {
    fn from(e: ArtifactError) -> Self {
        Self::Artifact(e)
    }
}
impl From<serde_json::Error> for CheckpointError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

#[cfg(test)]
mod tests;
