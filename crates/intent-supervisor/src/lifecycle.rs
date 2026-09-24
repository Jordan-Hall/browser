use intent_contracts::{CancellationId, WorkerInstanceId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum LifecycleControl {
    Cancel {
        generation: WorkerInstanceId,
        cancellation_id: CancellationId,
    },
}

impl LifecycleControl {
    #[must_use]
    pub const fn cancel(generation: WorkerInstanceId, cancellation_id: CancellationId) -> Self {
        Self::Cancel {
            generation,
            cancellation_id,
        }
    }

    #[must_use]
    pub const fn generation(self) -> WorkerInstanceId {
        match self {
            Self::Cancel { generation, .. } => generation,
        }
    }

    #[must_use]
    pub const fn cancellation_id(self) -> CancellationId {
        match self {
            Self::Cancel {
                cancellation_id, ..
            } => cancellation_id,
        }
    }
}
