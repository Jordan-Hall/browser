use crate::StateError;
use std::{fmt, io};

#[derive(Debug)]
pub enum RecoveryError {
    State(StateError),
    Sqlite(rusqlite::Error),
    Io(io::Error),
    Json(serde_json::Error),
    Invalid(&'static str),
    Conflict(&'static str),
    Missing(&'static str),
    Denied(&'static str),
    Limit(&'static str),
    Integrity(&'static str),
}
impl fmt::Display for RecoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::State(e) => write!(f, "recovery state: {e}"),
            Self::Sqlite(e) => write!(f, "recovery database: {e}"),
            Self::Io(e) => write!(f, "recovery filesystem: {e}"),
            Self::Json(e) => write!(f, "recovery encoding: {e}"),
            Self::Invalid(s) => write!(f, "invalid recovery input: {s}"),
            Self::Conflict(s) => write!(f, "stale recovery state: {s}"),
            Self::Missing(s) => write!(f, "missing recovery dependency: {s}"),
            Self::Denied(s) => write!(f, "recovery access denied: {s}"),
            Self::Limit(s) => write!(f, "recovery budget exceeded: {s}"),
            Self::Integrity(s) => write!(f, "recovery integrity failure: {s}"),
        }
    }
}
impl std::error::Error for RecoveryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::State(e) => Some(e),
            Self::Sqlite(e) => Some(e),
            Self::Io(e) => Some(e),
            Self::Json(e) => Some(e),
            _ => None,
        }
    }
}
impl From<StateError> for RecoveryError {
    fn from(e: StateError) -> Self {
        Self::State(e)
    }
}
impl From<rusqlite::Error> for RecoveryError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Sqlite(e)
    }
}
impl From<io::Error> for RecoveryError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<serde_json::Error> for RecoveryError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(crate) fn sql_u64(value: u64) -> Result<i64, RecoveryError> {
    i64::try_from(value).map_err(|_| RecoveryError::Limit("signed database counter"))
}
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(crate) fn unsigned(value: i64) -> Result<u64, RecoveryError> {
    u64::try_from(value).map_err(|_| RecoveryError::Integrity("negative database counter"))
}
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(crate) fn bounded_json<T: serde::Serialize>(
    value: &T,
    limit: usize,
) -> Result<Vec<u8>, RecoveryError> {
    struct Output {
        bytes: Vec<u8>,
        limit: usize,
        overflow: bool,
    }
    impl io::Write for Output {
        fn write(&mut self, data: &[u8]) -> io::Result<usize> {
            if data.len() > self.limit.saturating_sub(self.bytes.len()) {
                self.overflow = true;
                return Err(io::Error::other("JSON output budget"));
            }
            self.bytes
                .try_reserve(data.len())
                .map_err(|_| io::Error::other("JSON allocation"))?;
            self.bytes.extend_from_slice(data);
            Ok(data.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let mut output = Output {
        bytes: Vec::new(),
        limit,
        overflow: false,
    };
    let result = serde_json::to_writer(&mut output, value);
    if output.overflow {
        return Err(RecoveryError::Limit("JSON output"));
    }
    result?;
    Ok(output.bytes)
}
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(crate) fn digest(bytes: &[u8]) -> intent_contracts::ContentHash {
    use sha2::{Digest, Sha256};
    intent_contracts::ContentHash::from_bytes(Sha256::digest(bytes).into())
}
