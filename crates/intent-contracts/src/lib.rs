#![forbid(unsafe_code)]
#![doc = r#"Versioned, authority-neutral domain contracts shared by Intent Browser processes.

Typed identifiers are deliberately non-interchangeable:

```compile_fail
use intent_contracts::{TaskId, WorkspaceId};

fn requires_workspace(_: WorkspaceId) {}
fn invalid(task: TaskId) {
    requires_workspace(task);
}
```
"#]

mod ids;
mod values;

pub use ids::{
    AccountId, ActionProposalId, ApprovalId, ArtifactId, CancellationId, CapabilityId, ConnectorId,
    EvidenceId, GoalContractId, MemoryRecordId, ObservationId, OperationId, ReceiptId, RequestId,
    TaskId, TraceId, ViewDefinitionId, WorkspaceId,
};
pub use values::{
    AccountQualifiedResourceId, BoundedText, BoundedTextError, ByteSize, ContentHash,
    ContentHashError, CurrencyCode, CurrencyCodeError, CurrencyScale, CurrencyScaleError,
    KnownCurrencyScale, MAX_CURRENCY_SCALE, MAX_UNIX_TIMESTAMP_MICROS, MIN_UNIX_TIMESTAMP_MICROS,
    Money, ProviderAccountId, ProviderId, ProviderResourceId, TimestampError, UnixTimestampMicros,
};

/// Stable schema-family identifier for durable domain contracts.
pub const CONTRACTS_SCHEMA_FAMILY: &str = "intent.contracts";
