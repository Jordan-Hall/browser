use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "platform", rename_all = "snake_case")]
pub enum PeerCredentialEvidence {
    Unix {
        pid: Option<u32>,
        uid: u32,
        gid: u32,
    },
    Windows {
        process_id: u32,
    },
    Synthetic {
        process_id: u32,
        principal_id: u64,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PeerExpectation {
    AnyLocal,
    Exact(PeerCredentialEvidence),
}

impl PeerExpectation {
    #[must_use]
    pub const fn exact(evidence: PeerCredentialEvidence) -> Self {
        Self::Exact(evidence)
    }

    #[must_use]
    pub const fn matches(self, evidence: PeerCredentialEvidence) -> bool {
        match self {
            Self::AnyLocal => true,
            Self::Exact(expected) => expected == evidence,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PeerCredentialError {
    UnsupportedPlatform,
    Os(i32),
    InvalidProcessId,
}

impl fmt::Display for PeerCredentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlatform => {
                formatter.write_str("peer credential extraction is unsupported on this platform")
            }
            Self::Os(code) => write!(formatter, "peer credential OS call failed with code {code}"),
            Self::InvalidProcessId => formatter.write_str("peer process id was invalid"),
        }
    }
}

impl Error for PeerCredentialError {}

#[cfg(unix)]
pub fn anonymous_unix_channel_pair() -> std::io::Result<(
    std::os::unix::net::UnixStream,
    std::os::unix::net::UnixStream,
)> {
    std::os::unix::net::UnixStream::pair()
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn unix_peer_credentials(
    stream: &std::os::unix::net::UnixStream,
) -> Result<PeerCredentialEvidence, PeerCredentialError> {
    use nix::sys::socket::{getsockopt, sockopt::PeerCredentials};

    let credentials = getsockopt(stream, PeerCredentials)
        .map_err(|error| PeerCredentialError::Os(error as i32))?;
    let pid =
        u32::try_from(credentials.pid()).map_err(|_| PeerCredentialError::InvalidProcessId)?;

    Ok(PeerCredentialEvidence::Unix {
        pid: Some(pid),
        uid: credentials.uid(),
        gid: credentials.gid(),
    })
}

#[cfg(target_vendor = "apple")]
pub fn unix_peer_credentials(
    stream: &std::os::unix::net::UnixStream,
) -> Result<PeerCredentialEvidence, PeerCredentialError> {
    use nix::sys::socket::{getsockopt, sockopt::LocalPeerPid};
    use nix::unistd::getpeereid;

    let (uid, gid) = getpeereid(stream).map_err(|error| PeerCredentialError::Os(error as i32))?;
    let raw_pid =
        getsockopt(stream, LocalPeerPid).map_err(|error| PeerCredentialError::Os(error as i32))?;
    let pid = u32::try_from(raw_pid).map_err(|_| PeerCredentialError::InvalidProcessId)?;

    Ok(PeerCredentialEvidence::Unix {
        pid: Some(pid),
        uid: uid.as_raw(),
        gid: gid.as_raw(),
    })
}

#[cfg(all(
    unix,
    not(any(target_os = "linux", target_os = "android", target_vendor = "apple"))
))]
pub fn unix_peer_credentials(
    _stream: &std::os::unix::net::UnixStream,
) -> Result<PeerCredentialEvidence, PeerCredentialError> {
    Err(PeerCredentialError::UnsupportedPlatform)
}

#[cfg(windows)]
#[allow(unsafe_code)]
pub fn named_pipe_client_credentials<H: std::os::windows::io::AsRawHandle>(
    pipe: &H,
) -> Result<PeerCredentialEvidence, PeerCredentialError> {
    use windows_sys::Win32::System::Pipes::GetNamedPipeClientProcessId;

    let mut process_id = 0_u32;
    let raw_handle = pipe.as_raw_handle();
    // SAFETY: the caller supplies an owned/borrowed object implementing AsRawHandle. We pass the
    // handle without retaining it, and `process_id` points to valid writable storage for the call.
    let result = unsafe { GetNamedPipeClientProcessId(raw_handle, &mut process_id) };
    if result == 0 {
        let error_code = std::io::Error::last_os_error().raw_os_error().map_or(-1, |code| code);
        return Err(PeerCredentialError::Os(error_code));
    }
    if process_id == 0 {
        return Err(PeerCredentialError::InvalidProcessId);
    }

    Ok(PeerCredentialEvidence::Windows { process_id })
}

#[cfg(test)]
mod tests {
    use super::{PeerCredentialEvidence, PeerExpectation};

    #[test]
    fn exact_peer_expectation_rejects_changed_process() {
        let expected = PeerCredentialEvidence::Unix {
            pid: Some(10),
            uid: 1000,
            gid: 1000,
        };
        let changed = PeerCredentialEvidence::Unix {
            pid: Some(11),
            uid: 1000,
            gid: 1000,
        };

        assert!(PeerExpectation::exact(expected).matches(expected));
        assert!(!PeerExpectation::exact(expected).matches(changed));
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn anonymous_unix_pair_exposes_current_peer_credentials()
    -> Result<(), Box<dyn std::error::Error>> {
        use super::{anonymous_unix_channel_pair, unix_peer_credentials};
        use nix::unistd::{getegid, geteuid};

        let (supervisor, _worker) = anonymous_unix_channel_pair()?;
        let evidence = unix_peer_credentials(&supervisor)?;
        let PeerCredentialEvidence::Unix { pid, uid, gid } = evidence else {
            return Err("unix socket returned non-unix peer credentials".into());
        };

        assert_eq!(pid, Some(std::process::id()));
        assert_eq!(uid, geteuid().as_raw());
        assert_eq!(gid, getegid().as_raw());
        Ok(())
    }
}
