#[cfg(all(target_os = "linux", target_env = "gnu"))]
use intent_contracts::WorkspaceState;
use intent_contracts::{ArtifactId, BoundedText, SchemaVersion, UnixTimestampMicros};
use intent_ipc::{
    CoreDocumentImport, CoreRecord, CoreRecordKind, ReadOnlyCoreDocument, WireError, WireLimits,
    import_core_document, migrate_legacy_numeric_money_goal_v1,
};
use intent_state::{
    ArtifactCatalogEntry, ArtifactError, ArtifactMetadata, ArtifactScope, NewArtifact, StateStore,
};
#[cfg(all(target_os = "linux", target_env = "gnu"))]
use intent_state::{
    GraphRevision, MAX_GRAPH_DEPENDENCIES, MAX_GRAPH_TASKS, RecoveryError, RuntimeOwner,
    TaskDependency, WorkspaceCheckpointError, WorkspaceGraph,
};
#[cfg(all(target_os = "linux", target_env = "gnu"))]
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::io::Cursor;
use std::path::Path;

const CURRENT_MEDIA_TYPE: &str = "application/vnd.intent.core-record+json";
const READ_ONLY_MEDIA_TYPE: &str = "application/vnd.intent.core-record.readonly+json";
pub const MAX_CORE_DOCUMENT_ARCHIVE_PAGE: usize = 128;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub const MAX_WORKSPACE_ACTIVATION_GOALS: usize = 64;

/// Selects an explicit import path. Historical compatibility is never auto-detected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoreDocumentImportMode {
    Strict,
    LegacyNumericMoneyGoalV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurableCoreDocumentRequest {
    pub artifact_id: ArtifactId,
    pub privacy_scope: ArtifactScope,
    pub created_at: UnixTimestampMicros,
    pub kind: CoreRecordKind,
    pub mode: CoreDocumentImportMode,
}

/// Explicit durable archive inputs for installing a new workspace graph while the runtime remains
/// startup-fenced. Runtime/provider/worker state is intentionally absent from this request.
#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreWorkspaceActivationRequest {
    pub privacy_scope: ArtifactScope,
    pub workspace_artifact: ArtifactId,
    pub goal_artifacts: Vec<ArtifactId>,
    pub task_artifacts: Vec<ArtifactId>,
    pub dependencies: Vec<TaskDependency>,
    pub activated_at: UnixTimestampMicros,
}

/// A validated current record or an opaque newer-minor document whose admitted bytes have
/// been durably stored as an artifact. Neither outcome grants approval or dispatch authority.
#[derive(Clone, Debug)]
pub enum PersistedCoreDocumentImport {
    Current {
        record: CoreRecord,
        artifact: ArtifactMetadata,
        legacy_numeric_money: bool,
    },
    ReadOnlyNewerMinor {
        document: ReadOnlyCoreDocument,
        artifact: ArtifactMetadata,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreDocumentArchiveEntry {
    artifact: ArtifactCatalogEntry,
    kind: CoreRecordKind,
    read_only: bool,
}

impl CoreDocumentArchiveEntry {
    #[must_use]
    pub const fn artifact(&self) -> &ArtifactCatalogEntry {
        &self.artifact
    }

    #[must_use]
    pub const fn kind(&self) -> CoreRecordKind {
        self.kind
    }

    #[must_use]
    pub const fn is_read_only(&self) -> bool {
        self.read_only
    }
}

/// A durable archive selection revalidated from the stored bytes at selection time.
/// It is still data only and never carries approval or dispatch authority.
#[derive(Clone, Debug)]
pub enum CoreDocumentArchiveSelection {
    Current {
        record: CoreRecord,
        artifact: ArtifactMetadata,
    },
    ReadOnlyNewerMinor {
        document: ReadOnlyCoreDocument,
        artifact: ArtifactMetadata,
    },
}

impl CoreDocumentArchiveSelection {
    #[must_use]
    pub const fn artifact(&self) -> &ArtifactMetadata {
        match self {
            Self::Current { artifact, .. } | Self::ReadOnlyNewerMinor { artifact, .. } => artifact,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> CoreRecordKind {
        match self {
            Self::Current { record, .. } => record.kind(),
            Self::ReadOnlyNewerMinor { document, .. } => document.kind(),
        }
    }

    #[must_use]
    pub const fn source_version(&self) -> SchemaVersion {
        match self {
            Self::Current { .. } => SchemaVersion::V1,
            Self::ReadOnlyNewerMinor { document, .. } => document.source_version(),
        }
    }

    #[must_use]
    pub const fn is_read_only(&self) -> bool {
        matches!(self, Self::ReadOnlyNewerMinor { .. })
    }

    #[must_use]
    pub fn record(&self) -> Option<&CoreRecord> {
        match self {
            Self::Current { record, .. } => Some(record),
            Self::ReadOnlyNewerMinor { .. } => None,
        }
    }
}

impl PersistedCoreDocumentImport {
    #[must_use]
    pub const fn source_version(&self) -> SchemaVersion {
        match self {
            Self::Current { .. } => SchemaVersion::V1,
            Self::ReadOnlyNewerMinor { document, .. } => document.source_version(),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> CoreRecordKind {
        match self {
            Self::Current { record, .. } => record.kind(),
            Self::ReadOnlyNewerMinor { document, .. } => document.kind(),
        }
    }

    #[must_use]
    pub const fn artifact(&self) -> &ArtifactMetadata {
        match self {
            Self::Current { artifact, .. } | Self::ReadOnlyNewerMinor { artifact, .. } => artifact,
        }
    }

    #[must_use]
    pub fn record(&self) -> Option<&CoreRecord> {
        match self {
            Self::Current { record, .. } => Some(record),
            Self::ReadOnlyNewerMinor { .. } => None,
        }
    }

    #[must_use]
    pub const fn is_read_only(&self) -> bool {
        matches!(self, Self::ReadOnlyNewerMinor { .. })
    }

    #[must_use]
    pub const fn used_legacy_numeric_money_adapter(&self) -> bool {
        matches!(
            self,
            Self::Current {
                legacy_numeric_money: true,
                ..
            }
        )
    }
}

#[derive(Debug)]
pub enum CoreDocumentPersistenceError {
    Wire(WireError),
    Artifact(ArtifactError),
    LegacyModeRequiresGoalContract,
    StaticMediaType(String),
    InvalidArchivePage,
    NotCoreDocumentArtifact(ArtifactId),
    InvalidArchiveMetadata(ArtifactId),
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    Recovery(RecoveryError),
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    WorkspaceGraph(WorkspaceCheckpointError),
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    InvalidWorkspaceActivation(&'static str),
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    DuplicateActivationArtifact(ArtifactId),
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    ReadOnlyActivationArtifact(ArtifactId),
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    UnexpectedActivationRecord {
        artifact_id: ArtifactId,
        expected: CoreRecordKind,
        actual: CoreRecordKind,
    },
}

impl fmt::Display for CoreDocumentPersistenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wire(error) => write!(formatter, "core document validation failed: {error}"),
            Self::Artifact(error) => write!(formatter, "core document persistence failed: {error}"),
            Self::LegacyModeRequiresGoalContract => formatter
                .write_str("legacy numeric-money import is valid only for GoalContract documents"),
            Self::StaticMediaType(error) => write!(formatter, "invalid import media type: {error}"),
            Self::InvalidArchivePage => write!(
                formatter,
                "core document archive page must contain 1..={MAX_CORE_DOCUMENT_ARCHIVE_PAGE} entries"
            ),
            Self::NotCoreDocumentArtifact(artifact_id) => write!(
                formatter,
                "artifact {artifact_id} is not a persisted core document import"
            ),
            Self::InvalidArchiveMetadata(artifact_id) => write!(
                formatter,
                "artifact {artifact_id} has core-document metadata inconsistent with its bytes"
            ),
            #[cfg(all(target_os = "linux", target_env = "gnu"))]
            Self::Recovery(error) => {
                write!(formatter, "workspace activation fence failed: {error}")
            }
            #[cfg(all(target_os = "linux", target_env = "gnu"))]
            Self::WorkspaceGraph(error) => {
                write!(formatter, "workspace activation failed: {error}")
            }
            #[cfg(all(target_os = "linux", target_env = "gnu"))]
            Self::InvalidWorkspaceActivation(reason) => {
                write!(formatter, "invalid workspace activation: {reason}")
            }
            #[cfg(all(target_os = "linux", target_env = "gnu"))]
            Self::DuplicateActivationArtifact(artifact_id) => write!(
                formatter,
                "workspace activation artifact {artifact_id} was supplied more than once"
            ),
            #[cfg(all(target_os = "linux", target_env = "gnu"))]
            Self::ReadOnlyActivationArtifact(artifact_id) => write!(
                formatter,
                "read-only newer-minor artifact {artifact_id} cannot activate writable workspace state"
            ),
            #[cfg(all(target_os = "linux", target_env = "gnu"))]
            Self::UnexpectedActivationRecord {
                artifact_id,
                expected,
                actual,
            } => write!(
                formatter,
                "workspace activation artifact {artifact_id} is {}, expected {}",
                actual.family_name(),
                expected.family_name()
            ),
        }
    }
}

impl Error for CoreDocumentPersistenceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Wire(error) => Some(error),
            Self::Artifact(error) => Some(error),
            #[cfg(all(target_os = "linux", target_env = "gnu"))]
            Self::Recovery(error) => Some(error),
            #[cfg(all(target_os = "linux", target_env = "gnu"))]
            Self::WorkspaceGraph(error) => Some(error),
            Self::LegacyModeRequiresGoalContract
            | Self::StaticMediaType(_)
            | Self::InvalidArchivePage
            | Self::NotCoreDocumentArtifact(_)
            | Self::InvalidArchiveMetadata(_) => None,
            #[cfg(all(target_os = "linux", target_env = "gnu"))]
            Self::InvalidWorkspaceActivation(_)
            | Self::DuplicateActivationArtifact(_)
            | Self::ReadOnlyActivationArtifact(_)
            | Self::UnexpectedActivationRecord { .. } => None,
        }
    }
}

impl From<WireError> for CoreDocumentPersistenceError {
    fn from(error: WireError) -> Self {
        Self::Wire(error)
    }
}

impl From<ArtifactError> for CoreDocumentPersistenceError {
    fn from(error: ArtifactError) -> Self {
        Self::Artifact(error)
    }
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
impl From<RecoveryError> for CoreDocumentPersistenceError {
    fn from(error: RecoveryError) -> Self {
        Self::Recovery(error)
    }
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
impl From<WorkspaceCheckpointError> for CoreDocumentPersistenceError {
    fn from(error: WorkspaceCheckpointError) -> Self {
        Self::WorkspaceGraph(error)
    }
}

/// List persisted CORE document imports for one privacy scope without opening or interpreting
/// their payloads. Entries are ordered newest-first and bounded to a single product page.
/// Selection must still call `select_persisted_core_document_import`, which verifies and
/// revalidates the stored bytes before returning typed data.
pub fn list_persisted_core_document_imports(
    store: &StateStore,
    privacy_scope: &ArtifactScope,
    limit: usize,
) -> Result<Vec<CoreDocumentArchiveEntry>, CoreDocumentPersistenceError> {
    if limit == 0 || limit > MAX_CORE_DOCUMENT_ARCHIVE_PAGE {
        return Err(CoreDocumentPersistenceError::InvalidArchivePage);
    }
    let mut entries = Vec::new();
    for kind in CoreRecordKind::ALL.iter().copied() {
        for (base, read_only) in [(CURRENT_MEDIA_TYPE, false), (READ_ONLY_MEDIA_TYPE, true)] {
            let media_type = bounded_media_type(base, kind)?;
            entries.extend(
                store
                    .list_artifact_metadata_by_media_type(
                        privacy_scope,
                        media_type.as_str(),
                        limit,
                    )?
                    .into_iter()
                    .map(|artifact| CoreDocumentArchiveEntry {
                        artifact,
                        kind,
                        read_only,
                    }),
            );
        }
    }
    entries.sort_by(|left, right| {
        right
            .artifact
            .created_at()
            .cmp(&left.artifact.created_at())
            .then_with(|| {
                left.artifact
                    .artifact_id()
                    .to_string()
                    .cmp(&right.artifact.artifact_id().to_string())
            })
    });
    entries.truncate(limit);
    Ok(entries)
}

/// Select and revalidate one durable CORE document import. The artifact handle must belong to
/// the supplied privacy scope, its blob is integrity-checked, its media metadata must exactly bind
/// a known record family/read-only mode, and the stored bytes must still satisfy that mode.
/// Verification and capture occur in one pass bounded by the durable byte size and selected wire
/// limit, so corrupted/appended blobs cannot force a larger verification read or race a second read.
/// This function does not activate a workspace or create any execution authority.
pub fn select_persisted_core_document_import(
    store: &StateStore,
    artifact_root: &Path,
    privacy_scope: &ArtifactScope,
    artifact_id: ArtifactId,
    limits: WireLimits,
) -> Result<CoreDocumentArchiveSelection, CoreDocumentPersistenceError> {
    let metadata = store.artifact_metadata(artifact_id, privacy_scope)?;
    let Some((kind, expected_read_only)) = parse_archive_media_type(metadata.media_type().as_str())
    else {
        return Err(CoreDocumentPersistenceError::NotCoreDocumentArtifact(
            artifact_id,
        ));
    };
    let maximum = u64::try_from(limits.max_control_frame_bytes).unwrap_or(u64::MAX);
    if metadata.byte_size() > maximum {
        return Err(CoreDocumentPersistenceError::Wire(WireError::new(
            intent_ipc::WireErrorCode::FrameTooLarge,
            "stored core document exceeds the selected wire byte budget",
        )));
    }
    let (verified_metadata, bytes) = store.read_verified_artifact_bytes_bounded(
        artifact_root,
        artifact_id,
        privacy_scope,
        maximum,
    )?;
    if verified_metadata != metadata {
        return Err(CoreDocumentPersistenceError::InvalidArchiveMetadata(
            artifact_id,
        ));
    }

    match (
        expected_read_only,
        import_core_document(kind, &bytes, limits)?,
    ) {
        (false, CoreDocumentImport::Current(record)) => Ok(CoreDocumentArchiveSelection::Current {
            record,
            artifact: verified_metadata,
        }),
        (true, CoreDocumentImport::ReadOnlyNewerMinor(document)) => {
            Ok(CoreDocumentArchiveSelection::ReadOnlyNewerMinor {
                document,
                artifact: verified_metadata,
            })
        }
        _ => Err(CoreDocumentPersistenceError::InvalidArchiveMetadata(
            artifact_id,
        )),
    }
}

/// Install a new writable workspace graph from already persisted CORE archive documents.
///
/// Every artifact is selected and revalidated at activation time. Same-major newer-minor documents
/// are rejected rather than reinterpreted as writable current data. Runtime-owned cursors, provider
/// references, worker instances and retained-artifact pins are never restored by this path. The
/// runtime must still be startup-fenced and remains fenced afterwards; normal startup planning and
/// explicit action authorization remain separate gates.
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub fn activate_persisted_core_workspace(
    owner: &mut RuntimeOwner,
    artifact_root: &Path,
    request: CoreWorkspaceActivationRequest,
    limits: WireLimits,
) -> Result<GraphRevision, CoreDocumentPersistenceError> {
    if request.goal_artifacts.len() > MAX_WORKSPACE_ACTIVATION_GOALS
        || request.task_artifacts.is_empty()
        || request.task_artifacts.len() > MAX_GRAPH_TASKS
        || request.dependencies.len() > MAX_GRAPH_DEPENDENCIES
    {
        return Err(CoreDocumentPersistenceError::InvalidWorkspaceActivation(
            "workspace archive collection budget exceeded",
        ));
    }

    let mut unique = BTreeSet::new();
    for artifact_id in std::iter::once(request.workspace_artifact)
        .chain(request.goal_artifacts.iter().copied())
        .chain(request.task_artifacts.iter().copied())
    {
        if !unique.insert(artifact_id.to_string()) {
            return Err(CoreDocumentPersistenceError::DuplicateActivationArtifact(
                artifact_id,
            ));
        }
    }

    let workspace = match select_current_activation_record(
        owner.state(),
        artifact_root,
        &request.privacy_scope,
        request.workspace_artifact,
        CoreRecordKind::Workspace,
        limits,
    )? {
        CoreRecord::Workspace(workspace) => *workspace,
        _ => unreachable!("record kind checked before return"),
    };
    if workspace.state() != WorkspaceState::Active {
        return Err(CoreDocumentPersistenceError::InvalidWorkspaceActivation(
            "archived workspace cannot be activated",
        ));
    }

    let mut goals = Vec::with_capacity(request.goal_artifacts.len());
    for artifact_id in request.goal_artifacts {
        match select_current_activation_record(
            owner.state(),
            artifact_root,
            &request.privacy_scope,
            artifact_id,
            CoreRecordKind::GoalContract,
            limits,
        )? {
            CoreRecord::GoalContract(goal) => goals.push(*goal),
            _ => unreachable!("record kind checked before return"),
        }
    }

    let mut tasks = Vec::with_capacity(request.task_artifacts.len());
    for artifact_id in request.task_artifacts {
        match select_current_activation_record(
            owner.state(),
            artifact_root,
            &request.privacy_scope,
            artifact_id,
            CoreRecordKind::Task,
            limits,
        )? {
            CoreRecord::Task(task) => tasks.push(*task),
            _ => unreachable!("record kind checked before return"),
        }
    }

    let graph = WorkspaceGraph {
        schema_version: SchemaVersion::V1,
        workspace,
        goals,
        tasks,
        dependencies: request.dependencies,
        retained_artifacts: Vec::new(),
        cursors: Vec::new(),
        provider_references: Vec::new(),
        worker_instances: Vec::new(),
    };

    owner.require_workspace_activation_fence()?;
    owner
        .state_mut()
        .activate_new_workspace_graph(&request.privacy_scope, &graph, request.activated_at)
        .map_err(CoreDocumentPersistenceError::from)
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
fn select_current_activation_record(
    store: &StateStore,
    artifact_root: &Path,
    privacy_scope: &ArtifactScope,
    artifact_id: ArtifactId,
    expected: CoreRecordKind,
    limits: WireLimits,
) -> Result<CoreRecord, CoreDocumentPersistenceError> {
    match select_persisted_core_document_import(
        store,
        artifact_root,
        privacy_scope,
        artifact_id,
        limits,
    )? {
        CoreDocumentArchiveSelection::ReadOnlyNewerMinor { .. } => Err(
            CoreDocumentPersistenceError::ReadOnlyActivationArtifact(artifact_id),
        ),
        CoreDocumentArchiveSelection::Current { record, .. } if record.kind() == expected => {
            Ok(record)
        }
        CoreDocumentArchiveSelection::Current { record, .. } => {
            Err(CoreDocumentPersistenceError::UnexpectedActivationRecord {
                artifact_id,
                expected,
                actual: record.kind(),
            })
        }
    }
}

/// Validate a current/future document or explicitly convert the pre-freeze numeric-money goal
/// format, then persist only the admitted representation in the durable artifact store.
///
/// Current and legacy documents are stored in canonical current-codec form. Same-major newer
/// minor documents are stored byte-for-byte and remain opaque/read-only. The artifact media type
/// binds the selected record family so an opaque artifact ID cannot be retried under another
/// family. Validation completes before the artifact writer is entered, so malformed input cannot
/// create durable import state. This function does not register an executable reference, approval,
/// operation or outbox row.
pub fn persist_core_document_import(
    store: &mut StateStore,
    artifact_root: &Path,
    request: DurableCoreDocumentRequest,
    input: &[u8],
    limits: WireLimits,
) -> Result<PersistedCoreDocumentImport, CoreDocumentPersistenceError> {
    let (record, stored_bytes, legacy_numeric_money) = match request.mode {
        CoreDocumentImportMode::Strict => {
            match import_core_document(request.kind, input, limits)? {
                CoreDocumentImport::Current(record) => {
                    let encoded = record.encode(limits)?;
                    (record, encoded, false)
                }
                CoreDocumentImport::ReadOnlyNewerMinor(document) => {
                    let exact = document.original_bytes().to_vec();
                    let artifact = store_bytes(
                        store,
                        artifact_root,
                        &request,
                        exact,
                        bounded_media_type(READ_ONLY_MEDIA_TYPE, request.kind)?,
                    )?;
                    return Ok(PersistedCoreDocumentImport::ReadOnlyNewerMinor {
                        document,
                        artifact,
                    });
                }
            }
        }
        CoreDocumentImportMode::LegacyNumericMoneyGoalV1 => {
            if request.kind != CoreRecordKind::GoalContract {
                return Err(CoreDocumentPersistenceError::LegacyModeRequiresGoalContract);
            }
            let imported = migrate_legacy_numeric_money_goal_v1(input, limits)?;
            let (record, canonical_bytes) = imported.into_parts();
            (record, canonical_bytes, true)
        }
    };

    let artifact = store_bytes(
        store,
        artifact_root,
        &request,
        stored_bytes,
        bounded_media_type(CURRENT_MEDIA_TYPE, request.kind)?,
    )?;
    Ok(PersistedCoreDocumentImport::Current {
        record,
        artifact,
        legacy_numeric_money,
    })
}

fn parse_archive_media_type(value: &str) -> Option<(CoreRecordKind, bool)> {
    let (family, read_only) = if let Some(value) = value.strip_prefix(CURRENT_MEDIA_TYPE) {
        (value.strip_prefix(";family=")?, false)
    } else {
        let value = value.strip_prefix(READ_ONLY_MEDIA_TYPE)?;
        (value.strip_prefix(";family=")?, true)
    };
    CoreRecordKind::ALL
        .iter()
        .copied()
        .find(|kind| kind.family_name() == family)
        .map(|kind| (kind, read_only))
}

fn bounded_media_type(
    base: &str,
    kind: CoreRecordKind,
) -> Result<BoundedText<255>, CoreDocumentPersistenceError> {
    BoundedText::try_new(format!("{base};family={}", kind.family_name()))
        .map_err(|error| CoreDocumentPersistenceError::StaticMediaType(error.to_string()))
}

fn store_bytes(
    store: &mut StateStore,
    artifact_root: &Path,
    request: &DurableCoreDocumentRequest,
    bytes: Vec<u8>,
    media_type: BoundedText<255>,
) -> Result<ArtifactMetadata, ArtifactError> {
    let mut reader = Cursor::new(bytes);
    store.store_artifact(
        artifact_root,
        NewArtifact {
            artifact_id: request.artifact_id,
            privacy_scope: request.privacy_scope.clone(),
            media_type,
            created_at: request.created_at,
        },
        &mut reader,
    )
}
