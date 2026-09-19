use intent_contracts::{ArtifactId, BoundedText, SchemaVersion, UnixTimestampMicros};
use intent_ipc::{
    CoreDocumentImport, CoreRecord, CoreRecordKind, ReadOnlyCoreDocument, WireError, WireLimits,
    import_core_document, migrate_legacy_numeric_money_goal_v1,
};
use intent_state::{
    ArtifactCatalogEntry, ArtifactError, ArtifactMetadata, ArtifactScope, NewArtifact, StateStore,
};
use std::error::Error;
use std::fmt;
use std::io::Cursor;
use std::path::Path;

const CURRENT_MEDIA_TYPE: &str = "application/vnd.intent.core-record+json";
const READ_ONLY_MEDIA_TYPE: &str = "application/vnd.intent.core-record.readonly+json";
pub const MAX_CORE_DOCUMENT_ARCHIVE_PAGE: usize = 128;

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
        }
    }
}

impl Error for CoreDocumentPersistenceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Wire(error) => Some(error),
            Self::Artifact(error) => Some(error),
            Self::LegacyModeRequiresGoalContract
            | Self::StaticMediaType(_)
            | Self::InvalidArchivePage
            | Self::NotCoreDocumentArtifact(_)
            | Self::InvalidArchiveMetadata(_) => None,
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
