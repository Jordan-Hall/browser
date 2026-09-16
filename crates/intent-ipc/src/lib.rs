#![forbid(unsafe_code)]
#![doc = "Bounded local IPC primitives for Intent Browser trusted processes."]

/// Stable schema-family identifier for IPC envelopes.
pub const IPC_SCHEMA_FAMILY: &str = "intent.ipc";

/// Return the domain contract schema family consumed by this IPC layer.
#[must_use]
pub const fn contracts_schema_family() -> &'static str {
    intent_contracts::CONTRACTS_SCHEMA_FAMILY
}
