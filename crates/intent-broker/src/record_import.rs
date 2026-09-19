use intent_contracts::{ArtifactId, BoundedText, SchemaVersion, UnixTimestampMicros};
use intent_ipc::{
    CoreDocumentImport, CoreRecord, CoreRecordKind, ReadOnlyCoreDocument, WireError, WireLimits,
    import_core_document, migrate_legacy_numeric_money_goal_v1,
};
use intent_state::{ArtifactError, ArtifactMetadata, ArtifactScope, NewArtifact, StateStore};
use std::error::Error;
use std::fmt;
use std::io::Cursor;
use std::path::Path;

const CURRENT_MEDIA_TYPE: &str = "application/vnd.intent.core-record+json";
const READ_ONLY_MEDIA_TYPE: &str = "application/vnd.intent.core-record.readonly+json";

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
}

impl fmt::Display for CoreDocumentPersistenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wire(error) => write!(formatter, "core document validation failed: {error}"),
            Self::Artifact(error) => write!(formatter, "core document persistence failed: {error}"),
            Self::LegacyModeRequiresGoalContract => formatter
                .write_str("legacy numeric-money import is valid only for GoalContract documents"),
            Self::StaticMediaType(error) => write!(formatter, "invalid import media type: {error}"),
        }
    }
}

impl Error for CoreDocumentPersistenceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Wire(error) => Some(error),
            Self::Artifact(error) => Some(error),
            Self::LegacyModeRequiresGoalContract | Self::StaticMediaType(_) => None,
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
