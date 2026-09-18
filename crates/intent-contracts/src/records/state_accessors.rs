use super::{GoalConstraint, GoalContract, Task, TaskState, Workspace};
use crate::AccountId;

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
