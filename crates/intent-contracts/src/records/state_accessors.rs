use super::{GoalConstraint, GoalContract, Task, TaskState, Workspace};
use crate::{AccountId, ArtifactReference, CapabilityId, GoalContractId, WorkspaceId};

impl GoalContract {
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
    pub const fn goal_contract_id(&self) -> Option<GoalContractId> {
        self.goal_contract_id
    }
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }
}

impl Task {
    #[must_use]
    pub const fn workspace_id(&self) -> WorkspaceId {
        self.workspace_id
    }
    #[must_use]
    pub const fn goal_contract_id(&self) -> Option<GoalContractId> {
        self.goal_contract_id
    }
    #[must_use]
    pub const fn state(&self) -> TaskState {
        self.state
    }
    #[must_use]
    pub fn required_capabilities(&self) -> &[CapabilityId] {
        &self.required_capabilities
    }
    #[must_use]
    pub fn result_artifacts(&self) -> &[ArtifactReference] {
        &self.result_artifacts
    }
}
