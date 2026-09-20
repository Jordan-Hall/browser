use crate::{
    AccountId, AccountQualifiedResourceId, ApprovalRequirement, ArtifactReference,
    CapabilityEffectClass, CapabilityId, ContentHash, TaskId, UnixTimestampMicros,
};
use serde::{Deserialize, Serialize};

/// Versioned rules for the exact approved artifact bytes. No byte transformation is implied.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalizationVersion {
    ExactBytesV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionContext {
    pub source_revision: ContentHash,
    pub canonicalization: CanonicalizationVersion,
}

/// Authority-neutral material that must match across proposal, approval and durable operation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionBinding {
    pub task_id: TaskId,
    pub account_id: AccountId,
    pub capability_id: CapabilityId,
    pub target_resource: Option<AccountQualifiedResourceId>,
    pub canonical_arguments: ArtifactReference,
    pub context: ActionContext,
    pub effect_class: CapabilityEffectClass,
    pub approval_requirement: ApprovalRequirement,
    pub expires_at: Option<UnixTimestampMicros>,
}
