#![forbid(unsafe_code)]
#![doc = "Bounded local IPC primitives for Intent Browser trusted processes."]

mod bounded_json;
mod bounded_sequence;
mod cancellation;
mod codec;
mod document_import;
mod document_syntax;
mod envelope;
mod error;
mod flow;
mod frame;
mod legacy_import;
mod limits;
mod negotiation;
mod record_codec;
mod stream;
mod strict_json;

pub use cancellation::{
    CancellationError, CancellationRegistration, CancellationRegistry, CancellationState,
    DeadlineStatus, MAX_CANCELLATION_RECORDS, deadline_status,
};
pub use codec::ControlCodec;
pub use document_import::{CoreDocumentImport, ReadOnlyCoreDocument, import_core_document};
pub use envelope::{Envelope, EnvelopeKind, decode_control, encode_control};
pub use error::{WireError, WireErrorCode};
pub use flow::{
    ArtifactDispatch, CreditError, DeliveryClass, EnqueueError, EnqueueErrorKind, MAX_FLOW_CREDITS,
    MAX_QUEUE_CAPACITY, PriorityQueue, QueueConfigError, QueueLimits,
};
pub use frame::{DecodeBatch, FRAME_HEADER_BYTES, Frame, FrameDecoder, FrameLane};
pub use legacy_import::{LegacyNumericMoneyImport, migrate_legacy_numeric_money_goal_v1};
pub use limits::WireLimits;
pub use negotiation::{
    MAX_PROTOCOL_CAPABILITIES, MAX_PROTOCOL_RANGES, NegotiatedProtocol, NegotiationError,
    ProtocolCapability, ProtocolChangeClass, ProtocolOffer, ProtocolOfferError, ProtocolRange,
    VersionChangeError, negotiate_protocol, validate_version_change,
};
pub use record_codec::{CoreRecord, CoreRecordKind, decode_core_record};
pub use stream::{StreamEndpoint, StreamError, StreamEvent};

/// Stable schema-family identifier for IPC envelopes.
pub const IPC_SCHEMA_FAMILY: &str = "intent.ipc";

/// Return the domain contract schema family consumed by this IPC layer.
#[must_use]
pub const fn contracts_schema_family() -> &'static str {
    intent_contracts::CONTRACTS_SCHEMA_FAMILY
}
