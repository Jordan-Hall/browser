use super::{
    ActionProposal, Approval, ApprovalState, ArtifactReference, GoalConstraint, GoalContract, Task,
    TaskState, Workspace,
};
use crate::{
    AccountId, AccountQualifiedResourceId, ActionProposalId, CapabilityId, ContentHash, SpendAmount,
    SpendAmountError, TaskId, UnixTimestampMicros,
};

impl GoalContract {
    /// Validate a declared budget before using it as a spending limit.
    /// An absent limit remains absent and does not grant unlimited spending.
    pub fn checked_budget(&self) -> Result<Option<SpendAmount>, SpendAmountError> {
        self.budget.map(SpendAmount::try_new).transpose()
    }

    #[must_use]
    pub fn constraints(&self) -> &[GoalConstraint] {
        &self.clarified_constraints
    }

    #[must_use]
    pub fn authorized_accounts(&self) -> &[AccountId] {
        &self.authorized_accounts
    }
}

impl Workspace {
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }
}

impl Task {
    #[must_use]
    pub const fn state(&self) -> TaskState {
        self.state
    }
}

impl ActionProposal {
    #[must_use]
    pub const fn action_proposal_id(&self) -> ActionProposalId {
        self.id
    }

    #[must_use]
    pub const fn task_id(&self) -> TaskId {
        self.task_id
    }

    #[must_use]
    pub const fn capability_id(&self) -> CapabilityId {
        self.capability_id
    }

    #[must_use]
    pub const fn account_id(&self) -> AccountId {
        self.account_id
    }

    #[must_use]
    pub fn target_resource(&self) -> Option<&AccountQualifiedResourceId> {
        self.target_resource.as_ref()
    }

    #[must_use]
    pub const fn canonical_arguments(&self) -> &ArtifactReference {
        &self.canonical_arguments
    }

    #[must_use]
    pub const fn arguments_hash(&self) -> ContentHash {
        self.arguments_hash
    }

    #[must_use]
    pub const fn expires_at(&self) -> Option<UnixTimestampMicros> {
        self.expires_at
    }
}

impl Approval {
    #[must_use]
    pub const fn action_proposal_id(&self) -> ActionProposalId {
        self.action_proposal_id
    }

    #[must_use]
    pub const fn exact_arguments_hash(&self) -> ContentHash {
        self.exact_arguments_hash
    }

    #[must_use]
    pub const fn state(&self) -> &ApprovalState {
        &self.state
    }
}
