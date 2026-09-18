use intent_contracts::{OperationAttemptId, OperationId};
use intent_state::RecoveryError;
use intent_supervisor::SupervisorError;
use std::fmt;

#[derive(Debug)]
pub enum BrokerError {
    Recovery(RecoveryError),
    Supervisor(SupervisorError),
    Invalid(&'static str),
    Blocked,
    Clock,
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
