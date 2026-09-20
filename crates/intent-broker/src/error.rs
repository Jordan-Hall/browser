use intent_contracts::{OperationAttemptId, OperationId, WorkerInstanceId};
use intent_state::RecoveryError;
use intent_supervisor::SupervisorError;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationPersistenceStatus {
    NotRequired,
    Persisted,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationTerminationStatus {
    Requested,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CancellationFailure {
    pub worker_id: WorkerInstanceId,
    pub persistence: CancellationPersistenceStatus,
    pub termination: CancellationTerminationStatus,
}

#[derive(Debug)]
pub enum CancellationFailureCause {
    Persistence(RecoveryError),
    Termination(SupervisorError),
}

#[derive(Debug)]
pub enum BrokerError {
    Recovery(RecoveryError),
    Supervisor(SupervisorError),
    Invalid(&'static str),
    Blocked,
    Clock,
    /// Cancellation latched locally but could not complete every required pre-notification step.
    /// The broker is fenced; callers must inspect both statuses and recover rather than retrying blindly.
    Cancellation {
        status: CancellationFailure,
        source: CancellationFailureCause,
    },
    /// Attempt commit happened; the caller must inspect/reconcile and must not retry the effect.
    Uncertain {
        operation_id: OperationId,
        attempt_id: OperationAttemptId,
    },
}
impl fmt::Display for BrokerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Recovery(e) => write!(f, "durable broker: {e}"),
            Self::Supervisor(e) => write!(f, "worker broker: {e}"),
            Self::Invalid(s) => write!(f, "invalid broker request: {s}"),
            Self::Blocked => {
                f.write_str("broker fenced after a persistence/clock failure; reopen and recover")
            }
            Self::Clock => f.write_str("broker clock is unavailable or regressed"),
            Self::Cancellation { status, .. } => write!(
                f,
                "worker {} cancellation incomplete: persistence={:?}, termination={:?}; broker fenced",
                status.worker_id, status.persistence, status.termination
            ),
            Self::Uncertain {
                operation_id,
                attempt_id,
            } => write!(
                f,
                "operation {operation_id} attempt {attempt_id} requires reconciliation"
            ),
        }
    }
}
impl std::error::Error for BrokerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Recovery(e) => Some(e),
            Self::Supervisor(e) => Some(e),
            Self::Cancellation { source, .. } => match source {
                CancellationFailureCause::Persistence(error) => Some(error),
                CancellationFailureCause::Termination(error) => Some(error),
            },
            _ => None,
        }
    }
}
impl From<RecoveryError> for BrokerError {
    fn from(e: RecoveryError) -> Self {
        Self::Recovery(e)
    }
}
impl From<SupervisorError> for BrokerError {
    fn from(e: SupervisorError) -> Self {
        Self::Supervisor(e)
    }
}
