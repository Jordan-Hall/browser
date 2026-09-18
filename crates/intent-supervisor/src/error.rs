use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum SupervisorError {
    Io(io::Error),
    Protocol,
    InvalidConfiguration(&'static str),
    UnsupportedBoundary,
    ExecutableMismatch,
    AdmissionExhausted,
    QueueFull,
    UnknownWorker,
    InvalidState,
    Revoked,
    DeadlineExpired,
    ScopeMismatch,
    CapabilityDenied,
    RestartDenied,
    CounterExhausted,
}

impl fmt::Display for SupervisorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "worker OS operation failed: {error}"),
            Self::Protocol => f.write_str("worker protocol violation"),
            Self::InvalidConfiguration(reason) => {
                write!(f, "invalid supervisor configuration: {reason}")
            }
            Self::UnsupportedBoundary => {
                f.write_str("requested worker containment is not implemented; launch denied")
            }
            Self::ExecutableMismatch => {
                f.write_str("worker executable does not match its approved image")
            }
            Self::AdmissionExhausted => f.write_str("worker admission budget is exhausted"),
            Self::QueueFull => f.write_str("bounded worker queue is full"),
            Self::UnknownWorker => f.write_str("unknown worker generation"),
            Self::InvalidState => f.write_str("operation is invalid in the worker's current state"),
            Self::Revoked => f.write_str("worker generation is revoked"),
            Self::DeadlineExpired => f.write_str("worker or request deadline has expired"),
            Self::ScopeMismatch => f.write_str("worker account or task does not match"),
            Self::CapabilityDenied => {
                f.write_str("worker capability or message family is not allowed")
            }
            Self::RestartDenied => {
                f.write_str("worker restart budget, backoff or lifetime forbids restart")
            }
            Self::CounterExhausted => f.write_str("worker monotonic counter is exhausted"),
        }
    }
}

impl Error for SupervisorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}
impl From<io::Error> for SupervisorError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<intent_ipc::WireError> for SupervisorError {
    fn from(_: intent_ipc::WireError) -> Self {
        Self::Protocol
    }
}
