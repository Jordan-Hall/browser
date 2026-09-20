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

Profile, workspace and account scope cannot be substituted implicitly:

```compile_fail
use intent_contracts::{ProfileId, WorkspaceId};
fn requires_profile(_: ProfileId) {}
fn invalid(workspace: WorkspaceId) { requires_profile(workspace); }
```

```compile_fail
use intent_contracts::{AccountId, TaskId};
fn requires_account(_: AccountId) {}
fn invalid(task: TaskId) { requires_account(task); }
```
"#]

mod action_binding;
mod byte_size;
mod decimal_i128;
mod execution;
mod ids;
mod migration;
mod records;
mod values;
mod version;

pub use action_binding::{ActionBinding, ActionContext, CanonicalizationVersion};
pub use execution::{
    ExecutionAttempt, ExecutionEvidence, ExecutionObservation, ExecutionObservationData,
    ExecutionOutcome, ExecutionPhase, ExecutionStage, VerifiedCompensation,
};
pub use ids::{
    AccountId, ActionProposalId, ApprovalId, ArtifactId, CancellationId, CapabilityId,
    CheckpointId, ConnectorId, EvidenceId, GoalContractId, MemoryRecordId, ObservationId,
    OperationAttemptId, OperationId, OutboxMessageId, ProfileId, ReceiptId, RequestId, TaskId,
    TraceId, ViewDefinitionId, WorkerInstanceId, WorkspaceId,
};
pub use migration::{
    DocumentAccess, MAX_MIGRATION_DOCUMENT_BYTES, MAX_MIGRATION_ERROR_BYTES, MAX_MIGRATION_SCHEMAS,
    MAX_MIGRATION_STEPS, MigrationFailure, MigrationFn, MigrationOutcome, MigrationRegistry,
    MigrationRegistryError, MigrationStep, RecordFamily, SchemaValidatorFn, assess_document_access,
};
pub use records::{
    ActionProposal, ActionProposalDescriptor, Approval, ApprovalRequirement, ApprovalState,
    ArtifactReference, Capability, CapabilityDescriptor, CapabilityEffectClass,
    CapabilitySupportLevel, Evidence, EvidenceOrigin, EvidenceRelation, GoalConstraint,
    GoalContract, InferenceMode, MAX_RECORD_COLLECTION_ENTRIES, MemoryKind, MemoryRecord,
    MemoryScope, Observation, ObservationKind, Operation, OperationState, Receipt, ReceiptOutcome,
    RecordValidationError, Task, TaskState, ViewBinding, ViewDefinition, Workspace, WorkspaceState,
};
pub use values::{
    AccountQualifiedResourceId, BoundedText, BoundedTextError, ByteSize, ContentHash,
    ContentHashError, CurrencyCode, CurrencyCodeError, CurrencyScale, CurrencyScaleError,
    KnownCurrencyScale, MAX_CURRENCY_SCALE, MAX_UNIX_TIMESTAMP_MICROS, MIN_UNIX_TIMESTAMP_MICROS,
    Money, ProviderAccountId, ProviderId, ProviderIdentifierError, ProviderResourceId, SpendAmount,
    SpendAmountError, TimestampError, UnixTimestampMicros,
};
pub use version::{SchemaVersion, SchemaVersionError};

/// Stable schema-family identifier for durable domain contracts.
pub const CONTRACTS_SCHEMA_FAMILY: &str = "intent.contracts";
