use crate::{ArtifactScope, DurableOperationState};
use intent_contracts::{
    AccountId, ActionProposalId, ArtifactId, ArtifactReference, BoundedText, CapabilityId,
    ContentHash, GoalContract, OperationAttemptId, OperationId, SchemaVersion, Task, TaskId,
    UnixTimestampMicros, WorkerInstanceId, Workspace, WorkspaceId,
};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};
use uuid::Uuid;

pub const MAX_GRAPH_BYTES: usize = 1024 * 1024;
pub const MAX_CHECKPOINT_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_GRAPH_TASKS: usize = 256;
pub const MAX_GRAPH_DEPENDENCIES: usize = 2048;
pub const MAX_CHECKPOINT_ARTIFACTS: usize = 512;
pub const MAX_CHECKPOINT_OPERATIONS: usize = 512;
pub const MAX_CHECKPOINT_BLOB_BYTES: u64 = 256 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CheckpointId(Uuid);
impl CheckpointId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }
}
impl Default for CheckpointId {
    fn default() -> Self {
        Self::new()
    }
}
impl fmt::Display for CheckpointId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskDependency {
    pub prerequisite: TaskId,
    pub dependent: TaskId,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointCursorKey {
    pub consumer: BoundedText<128>,
    pub source: BoundedText<128>,
    pub stream: BoundedText<128>,
}

/// A reference into the trusted provider-session store, not a bearer credential.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointProviderReference {
    pub provider: BoundedText<128>,
    pub account_id: AccountId,
    pub session_reference: ContentHash,
}

/// The domain records remain the source of truth; this is their graph container.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceGraph {
    pub schema_version: SchemaVersion,
    pub workspace: Workspace,
    pub goals: Vec<GoalContract>,
    pub tasks: Vec<Task>,
    pub dependencies: Vec<TaskDependency>,
    pub retained_artifacts: Vec<ArtifactReference>,
    pub cursors: Vec<CheckpointCursorKey>,
    pub provider_references: Vec<CheckpointProviderReference>,
    pub worker_instances: Vec<WorkerInstanceId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphRevision {
    pub workspace_id: WorkspaceId,
    pub revision: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckpointRequest {
    pub checkpoint_id: CheckpointId,
    pub workspace_id: WorkspaceId,
    pub expected_graph_revision: u64,
    pub created_at: UnixTimestampMicros,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapturedOperation {
    pub action_proposal_id: ActionProposalId,
    pub source_schema: SchemaVersion,
    pub state_detail: Option<BoundedText<2048>>,
    pub created_at: UnixTimestampMicros,
    pub updated_at: UnixTimestampMicros,
    pub operation_id: OperationId,
    pub task_id: TaskId,
    pub account_id: AccountId,
    pub capability_id: CapabilityId,
    pub arguments_hash: ContentHash,
    pub state: DurableOperationState,
    pub attempt_identity: Option<OperationAttemptId>,
    pub revision: u64,
    pub journal_sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapturedCursor {
    pub key: CheckpointCursorKey,
    pub last_sequence: u64,
}

/// Historical facts only. A checkpoint cannot enable dispatch or revive a worker lease.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceCheckpoint {
    pub(super) schema_version: SchemaVersion,
    pub(super) checkpoint_id: CheckpointId,
    pub(super) store_id: Uuid,
    pub(super) workspace_id: WorkspaceId,
    pub(super) privacy_scope: BoundedText<128>,
    pub(super) graph_revision: u64,
    pub(super) runtime_epoch: Uuid,
    pub(super) created_at: UnixTimestampMicros,
    pub(super) graph: WorkspaceGraph,
    pub(super) operations: Vec<CapturedOperation>,
    pub(super) cursors: Vec<CapturedCursor>,
}
impl WorkspaceCheckpoint {
    pub const fn checkpoint_id(&self) -> CheckpointId {
        self.checkpoint_id
    }
    pub const fn workspace_id(&self) -> WorkspaceId {
        self.workspace_id
    }
    pub const fn graph_revision(&self) -> u64 {
        self.graph_revision
    }
    pub const fn historical_runtime_epoch(&self) -> Uuid {
        self.runtime_epoch
    }
    pub fn graph(&self) -> &WorkspaceGraph {
        &self.graph
    }
    pub fn operations(&self) -> &[CapturedOperation] {
        &self.operations
    }
    pub fn cursors(&self) -> &[CapturedCursor] {
        &self.cursors
    }
    pub const fn grants_execution_authority(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckpointReceipt {
    pub checkpoint_id: CheckpointId,
    pub workspace_id: WorkspaceId,
    pub graph_revision: u64,
    pub digest: ContentHash,
    pub artifact_count: usize,
}

#[derive(Debug)]
pub enum CheckpointError {
    Sqlite(rusqlite::Error),
    Io(std::io::Error),
    Json(serde_json::Error),
    State(crate::StateError),
    Invalid(&'static str),
    Conflict,
    NotFound,
    AccessDenied,
    MissingArtifact(ArtifactId),
    SuppressedArtifact(ArtifactId),
    CorruptArtifact(ArtifactId),
    MissingCursor(CheckpointCursorKey),
}
impl fmt::Display for CheckpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(e) => write!(f, "checkpoint database error: {e}"),
            Self::Io(e) => write!(f, "checkpoint filesystem error: {e}"),
            Self::Json(e) => write!(f, "checkpoint encoding error: {e}"),
            Self::State(e) => write!(f, "checkpoint state error: {e}"),
            Self::Invalid(s) => write!(f, "invalid checkpoint: {s}"),
            Self::Conflict => f.write_str("checkpoint request or graph revision conflicts"),
            Self::NotFound => f.write_str("checkpoint or workspace graph not found"),
            Self::AccessDenied => f.write_str("checkpoint privacy scope mismatch"),
            Self::MissingArtifact(id) => write!(f, "checkpoint artifact {id} is missing"),
            Self::SuppressedArtifact(id) => write!(f, "checkpoint artifact {id} is suppressed"),
            Self::CorruptArtifact(id) => {
                write!(f, "checkpoint artifact {id} failed integrity validation")
            }
            Self::MissingCursor(_) => f.write_str("checkpoint cursor does not exist"),
        }
    }
}
impl Error for CheckpointError {}
impl From<rusqlite::Error> for CheckpointError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Sqlite(e)
    }
}
impl From<std::io::Error> for CheckpointError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<serde_json::Error> for CheckpointError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}
impl From<crate::StateError> for CheckpointError {
    fn from(e: crate::StateError) -> Self {
        Self::State(e)
    }
}

pub(super) fn scope_text(scope: &ArtifactScope) -> Result<BoundedText<128>, CheckpointError> {
    BoundedText::try_new(scope.as_str()).map_err(|_| CheckpointError::Invalid("privacy scope"))
}

impl fmt::Debug for WorkspaceGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorkspaceGraph")
            .field("workspace_id", &self.workspace.workspace_id())
            .field("task_count", &self.tasks.len())
            .field("content", &"[REDACTED]")
            .finish()
    }
}
impl fmt::Debug for WorkspaceCheckpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorkspaceCheckpoint")
            .field("checkpoint_id", &self.checkpoint_id)
            .field("graph_revision", &self.graph_revision)
            .field("content", &"[REDACTED]")
            .finish()
    }
}
