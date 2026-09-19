use crate::{CoreRecord, CoreRecordKind, WireError, WireErrorCode, WireLimits};
use intent_contracts::{DocumentAccess, SchemaVersion, assess_document_access};
use std::fmt;

/// A validated current record or an explicitly opaque, read-only newer document.
/// Neither variant grants authority to execute any recorded action.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CoreDocumentImport {
    Current(CoreRecord),
    ReadOnlyNewerMinor(ReadOnlyCoreDocument),
}

impl CoreDocumentImport {
    #[must_use]
    pub const fn kind(&self) -> CoreRecordKind {
        match self {
            Self::Current(record) => record.kind(),
            Self::ReadOnlyNewerMinor(document) => document.kind(),
        }
    }

    #[must_use]
    pub const fn source_version(&self) -> SchemaVersion {
        match self {
            Self::Current(_) => SchemaVersion::V1,
            Self::ReadOnlyNewerMinor(document) => document.source_version(),
        }
    }

    /// Encode an understood record. Refuse to silently remove newer fields or
    /// relabel their semantics as v1. Use `original_bytes` only for verbatim
    /// archival of an opaque document, never as a validated dispatch payload.
    pub fn encode_for_write(&self, limits: WireLimits) -> Result<Vec<u8>, WireError> {
        match self {
            Self::Current(record) => record.encode(limits),
            Self::ReadOnlyNewerMinor(_) => Err(WireError::new(
                WireErrorCode::UnsupportedSchema,
                "newer-minor document is read-only; schema 1.0 cannot rewrite it",
            )),
        }
    }
}

/// Bounded, syntactically valid JSON retained without interpreting its fields.
/// The selected family is caller-supplied routing metadata, not proof that the
/// unknown payload satisfies that family's newer schema. `Debug` omits content.
///
/// This type has no public constructor, field mutation, Serde implementation,
/// or conversion into a validated record. Keep it outside live write APIs.
///
/// ```compile_fail
/// use intent_ipc::ReadOnlyCoreDocument;
/// fn serialize_unknown(document: &ReadOnlyCoreDocument) {
///     let _ = serde_json::to_vec(document);
/// }
/// ```
///
/// ```compile_fail
/// use intent_ipc::{CoreRecord, ReadOnlyCoreDocument};
/// fn reinterpret_unknown(document: ReadOnlyCoreDocument) -> CoreRecord {
///     document.into()
/// }
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct ReadOnlyCoreDocument {
    kind: CoreRecordKind,
    source_version: SchemaVersion,
    original: Box<[u8]>,
}

impl ReadOnlyCoreDocument {
    #[must_use]
    pub const fn kind(&self) -> CoreRecordKind {
        self.kind
    }

    #[must_use]
    pub const fn source_version(&self) -> SchemaVersion {
        self.source_version
    }

    /// Exact original bytes, including whitespace, number spellings and unknown
    /// fields. These bytes remain untrusted and are not a validated record.
    #[must_use]
    pub fn original_bytes(&self) -> &[u8] {
        &self.original
    }
}

impl fmt::Debug for ReadOnlyCoreDocument {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReadOnlyCoreDocument")
            .field("kind", &self.kind)
            .field("source_version", &self.source_version)
            .field("bytes", &self.original.len())
            .finish_non_exhaustive()
    }
}

/// Import through the current strict codec or preserve a same-major newer-minor
/// document for read-only archival. Future schemas are not advertised as live
/// codecs. New major versions and documents requiring a migration are rejected.
///
/// All paths enforce byte, depth, collection and node limits, unique JSON keys,
/// valid UTF-8, a strict schema header and a single complete JSON value before
/// allocating retained bytes. No schema migration or provider action is run.
pub fn import_core_document(
    kind: CoreRecordKind,
    input: &[u8],
    limits: WireLimits,
) -> Result<CoreDocumentImport, WireError> {
    let version = crate::document_syntax::schema_version(input, limits)?;
    match assess_document_access(SchemaVersion::V1, version) {
        DocumentAccess::ReadWrite => crate::record_codec::decode_core_record_value(
            kind,
            crate::strict_json::decode(input, limits)?,
        )
        .map(CoreDocumentImport::Current),
        DocumentAccess::ReadOnlyNewerMinor => {
            let mut original = Vec::new();
            original.try_reserve_exact(input.len()).map_err(|_| {
                WireError::new(
                    WireErrorCode::AllocationFailed,
                    "document retention allocation failed",
                )
            })?;
            original.extend_from_slice(input);
            Ok(CoreDocumentImport::ReadOnlyNewerMinor(
                ReadOnlyCoreDocument {
                    kind,
                    source_version: version,
                    original: original.into_boxed_slice(),
                },
            ))
        }
        DocumentAccess::MigrationRequired | DocumentAccess::UnsupportedMajor => {
            Err(WireError::new(
                WireErrorCode::UnsupportedSchema,
                "document requires an explicitly supported schema or migration",
            ))
        }
    }
}
