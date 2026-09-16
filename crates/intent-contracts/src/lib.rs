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
mod records;
mod values;
mod version;

pub use ids::{
    AccountId, ActionProposalId, ApprovalId, ArtifactId, CancellationId, CapabilityId, ConnectorId,
    EvidenceId, GoalContractId, MemoryRecordId, ObservationId, OperationId, ReceiptId, RequestId,
    TaskId, TraceId, ViewDefinitionId, WorkerInstanceId, WorkspaceId,
};
pub use records::{
    ActionProposal, ActionProposalDescriptor, Approval, ApprovalRequirement, ApprovalState,
    ArtifactReference, Capability, CapabilityDescriptor, CapabilityEffectClass,
    CapabilitySupportLevel, Evidence, EvidenceOrigin, EvidenceRelation, GoalConstraint,
    GoalContract, InferenceMode, MemoryKind, MemoryRecord, MemoryScope, Observation,
    ObservationKind, Operation, OperationState, Receipt, ReceiptOutcome, Task, TaskState,
    ViewBinding, ViewDefinition, Workspace, WorkspaceState,
};
pub use values::{
    AccountQualifiedResourceId, BoundedText, BoundedTextError, ByteSize, ContentHash,
    ContentHashError, CurrencyCode, CurrencyCodeError, CurrencyScale, CurrencyScaleError,
    KnownCurrencyScale, MAX_CURRENCY_SCALE, MAX_UNIX_TIMESTAMP_MICROS, MIN_UNIX_TIMESTAMP_MICROS,
    Money, ProviderAccountId, ProviderId, ProviderResourceId, TimestampError, UnixTimestampMicros,
};
pub use version::{SchemaVersion, SchemaVersionError};

/// Stable schema-family identifier for durable domain contracts.
pub const CONTRACTS_SCHEMA_FAMILY: &str = "intent.contracts";
