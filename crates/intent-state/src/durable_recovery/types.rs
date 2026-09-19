use crate::{ArtifactScope, DurableOperationState, NewDurableOperation};
use intent_contracts::{
    AccountId, BoundedText, CapabilityId, ContentHash, OperationAttemptId, OperationId,
    OutboxMessageId, TaskId, UnixTimestampMicros, WorkerInstanceId,
};
use intent_recovery::{RecoveryDecision, RecoveryEffect};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Canonicalization contract for immutable action bytes admitted by the durable broker.
pub const ACTION_CANONICALIZATION_VERSION: u16 = 1;

/// Supplied only by the trusted account/policy and read-only source adapters.
#[derive(Clone, Debug)]
pub struct AuthorityUpdate {
    pub account_id: AccountId,
    pub capability_id: CapabilityId,
    pub expected_revision: u64,
    pub effect: RecoveryEffect,
    pub enabled: bool,
    pub source_revision: ContentHash,
    pub valid_until: UnixTimestampMicros,
    pub evidence_key_id: ContentHash,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompensationOrigin {
    pub operation_id: OperationId,
    pub attempt_id: OperationAttemptId,
    pub receipt: ContentHash,
}
/// Exact bytes and destination to be approved. Debug deliberately omits content.
pub struct RecoverableAction {
    pub operation: NewDurableOperation,
    pub scope: ArtifactScope,
    pub effect: RecoveryEffect,
    pub source_revision: ContentHash,
    pub canonicalization_version: u16,
    pub destination: BoundedText<512>,
    pub message_kind: BoundedText<128>,
    pub payload: Vec<u8>,
    pub deadline: UnixTimestampMicros,
    pub compensation: Option<CompensationOrigin>,
}
impl fmt::Debug for RecoverableAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecoverableAction")
            .field("operation_id", &self.operation.operation_id)
            .field("effect", &self.effect)
            .field("payload_bytes", &self.payload.len())
            .finish_non_exhaustive()
    }
}
/// A supervisor-owned registration, not a deserializable worker claim.
#[derive(Clone, Debug)]
pub struct WorkerRegistration {
    pub worker_id: WorkerInstanceId,
    pub task_id: TaskId,
    pub account_id: AccountId,
    pub capabilities: Vec<CapabilityId>,
    pub expires_at: UnixTimestampMicros,
}
/// Redacted, one-use claim. Only the profile owner can construct one.
///
/// ```compile_fail
/// fn duplicate(value: intent_state::DurableDispatchLease) {
///     let _copy = value.clone();
/// }
/// ```
pub struct DurableDispatchLease {
    pub(super) outbox_id: OutboxMessageId,
    pub(super) worker_id: WorkerInstanceId,
    pub(super) epoch: Uuid,
    pub(super) token: [u8; 32],
}
impl fmt::Debug for DurableDispatchLease {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DurableDispatchLease")
            .field("outbox_id", &self.outbox_id)
            .finish_non_exhaustive()
    }
}
/// Sendable bytes are exposed only after the durable attempt transaction commits.
/// Loss of this value must be reconciled; it must never be reconstructed for retry.
///
/// ```compile_fail
/// fn duplicate(value: intent_state::DurableSendAttempt) {
///     let _copy = value.clone();
/// }
/// ```
pub struct DurableSendAttempt {
    pub(super) outbox_id: OutboxMessageId,
    pub(super) operation_id: OperationId,
    pub(super) attempt_id: OperationAttemptId,
    pub(super) epoch: Uuid,
    pub(super) destination: BoundedText<512>,
    pub(super) message_kind: BoundedText<128>,
    pub(super) payload: Vec<u8>,
}
impl DurableSendAttempt {
    pub fn operation_id(&self) -> OperationId {
        self.operation_id
    }
    pub fn attempt_id(&self) -> OperationAttemptId {
        self.attempt_id
    }
    pub fn destination(&self) -> &str {
        self.destination.as_str()
    }
    pub fn message_kind(&self) -> &str {
        self.message_kind.as_str()
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}
impl fmt::Debug for DurableSendAttempt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DurableSendAttempt")
            .field("operation_id", &self.operation_id)
            .field("attempt_id", &self.attempt_id)
            .finish_non_exhaustive()
    }
}
#[derive(Clone, Copy, Debug)]
pub enum TransportObservation {
    AcceptedUnverified,
    OutcomeUnknown,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DurableRecoveryPlan {
    pub schema_version: u16,
    pub runtime_epoch: Uuid,
    pub operation_id: OperationId,
    pub operation_revision: u64,
    pub state: DurableOperationState,
    pub compensation_operation: Option<OperationId>,
    pub decision: RecoveryDecision,
    pub created_at: UnixTimestampMicros,
}
#[derive(Clone, Debug)]
pub struct StartupBatch {
    pub plans: Vec<DurableRecoveryPlan>,
    pub remaining: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryUiAction {
    Inspect,
    CancelUnsent,
    RequestReadOnlyReconciliation,
    ReviewFreshApproval,
}
#[derive(Clone, Debug, Serialize)]
pub struct RecoveryTaskView {
    pub plan: DurableRecoveryPlan,
    pub actions: Vec<RecoveryUiAction>,
}

/// Non-sendable dispatch metadata. No payload or destination is exposed before attempt commit.
#[derive(Clone, Copy, Debug)]
pub struct DispatchMetadata {
    pub operation_id: OperationId,
    pub attempt_id: OperationAttemptId,
    pub task_id: TaskId,
    pub account_id: AccountId,
    pub capability_id: CapabilityId,
    pub deadline: UnixTimestampMicros,
    pub payload_bytes: u64,
    pub routing_json_bytes: u64,
}
