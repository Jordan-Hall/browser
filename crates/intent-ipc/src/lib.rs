#![forbid(unsafe_code)]
#![doc = "Bounded local IPC primitives for Intent Browser trusted processes."]

mod bounded_json;
mod bounded_sequence;
mod codec;
mod envelope;
mod error;
mod frame;
mod limits;
mod negotiation;
mod strict_json;

pub use codec::ControlCodec;
pub use envelope::{Envelope, EnvelopeKind, decode_control, encode_control};
pub use error::{WireError, WireErrorCode};
pub use frame::{DecodeBatch, FRAME_HEADER_BYTES, Frame, FrameDecoder, FrameLane};
pub use limits::WireLimits;
pub use negotiation::{
    MAX_PROTOCOL_CAPABILITIES, MAX_PROTOCOL_RANGES, NegotiatedProtocol, NegotiationError,
    ProtocolCapability, ProtocolChangeClass, ProtocolOffer, ProtocolOfferError, ProtocolRange,
    VersionChangeError, negotiate_protocol, validate_version_change,
};

/// Stable schema-family identifier for IPC envelopes.
pub const IPC_SCHEMA_FAMILY: &str = "intent.ipc";

/// Return the domain contract schema family consumed by this IPC layer.
#[must_use]
pub const fn contracts_schema_family() -> &'static str {
    intent_contracts::CONTRACTS_SCHEMA_FAMILY
}
