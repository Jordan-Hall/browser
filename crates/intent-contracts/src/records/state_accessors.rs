use super::{GoalConstraint, GoalContract, Task, TaskState, Workspace};
use crate::{AccountId, SpendAmount, SpendAmountError};

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
