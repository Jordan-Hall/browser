use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ObservedPeer {
    #[cfg(unix)]
    Unix { pid: u32, uid: u32, gid: u32 },
    #[cfg(windows)]
    Windows { process_id: u32 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExpectedPeer(ObservedPeer);

impl ExpectedPeer {
    #[cfg(unix)]
    pub fn unix_process(pid: u32, uid: u32, gid: u32) -> Result<Self, PeerCredentialError> {
        if pid == 0 {
            return Err(PeerCredentialError::InvalidProcessId);
        }
        Ok(Self(ObservedPeer::Unix { pid, uid, gid }))
    }

    #[cfg(windows)]
    pub fn windows_process(process_id: u32) -> Result<Self, PeerCredentialError> {
        if process_id == 0 {
            return Err(PeerCredentialError::InvalidProcessId);
        }
        Ok(Self(ObservedPeer::Windows { process_id }))
    }

    pub(crate) fn matches(self, observed: ObservedPeer) -> bool {
        self.0 == observed
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PeerCredentialError {
    UnsupportedPlatform,
    Os(i32),
    InvalidProcessId,
    Mismatch,
}

impl fmt::Display for PeerCredentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlatform => {
                formatter.write_str("peer credential extraction is unsupported on this platform")
            }
            Self::Os(code) => write!(formatter, "peer credential OS call failed with code {code}"),
            Self::InvalidProcessId => formatter.write_str("peer process id was invalid"),
            Self::Mismatch => {
                formatter.write_str("connected peer differs from the expected process")
            }
        }
    }
}

impl Error for PeerCredentialError {}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) fn unix_peer_credentials(
    stream: &std::os::unix::net::UnixStream,
) -> Result<ObservedPeer, PeerCredentialError> {
    use nix::sys::socket::{getsockopt, sockopt::PeerCredentials};

    let credentials = getsockopt(stream, PeerCredentials)
        .map_err(|error| PeerCredentialError::Os(error as i32))?;
    let pid =
        u32::try_from(credentials.pid()).map_err(|_| PeerCredentialError::InvalidProcessId)?;

    Ok(ObservedPeer::Unix {
        pid,
        uid: credentials.uid(),
        gid: credentials.gid(),
    })
}

#[cfg(target_vendor = "apple")]
pub(crate) fn unix_peer_credentials(
    stream: &std::os::unix::net::UnixStream,
) -> Result<ObservedPeer, PeerCredentialError> {
    use nix::sys::socket::{getsockopt, sockopt::LocalPeerPid};
    use nix::unistd::getpeereid;

    let (uid, gid) = getpeereid(stream).map_err(|error| PeerCredentialError::Os(error as i32))?;
    let raw_pid =
        getsockopt(stream, LocalPeerPid).map_err(|error| PeerCredentialError::Os(error as i32))?;
    let pid = u32::try_from(raw_pid).map_err(|_| PeerCredentialError::InvalidProcessId)?;

    Ok(ObservedPeer::Unix {
        pid,
        uid: uid.as_raw(),
        gid: gid.as_raw(),
    })
}

#[cfg(all(
    unix,
    not(any(target_os = "linux", target_os = "android", target_vendor = "apple"))
))]
pub(crate) fn unix_peer_credentials(
    _stream: &std::os::unix::net::UnixStream,
) -> Result<ObservedPeer, PeerCredentialError> {
    Err(PeerCredentialError::UnsupportedPlatform)
}

#[cfg(windows)]
#[allow(unsafe_code)]
pub(crate) fn named_pipe_client_credentials<H: std::os::windows::io::AsRawHandle>(
    pipe: &H,
) -> Result<ObservedPeer, PeerCredentialError> {
    use windows_sys::Win32::System::Pipes::GetNamedPipeClientProcessId;

    let mut process_id = 0_u32;
    let raw_handle = pipe.as_raw_handle();
    // SAFETY: the caller supplies an owned/borrowed object implementing AsRawHandle. We pass the
    // handle without retaining it, and `process_id` points to valid writable storage for the call.
    let result = unsafe { GetNamedPipeClientProcessId(raw_handle, &mut process_id) };
    if result == 0 {
        let error_code = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
        return Err(PeerCredentialError::Os(error_code));
    }
    if process_id == 0 {
        return Err(PeerCredentialError::InvalidProcessId);
    }

    Ok(ObservedPeer::Windows { process_id })
}

#[cfg(windows)]
#[allow(unsafe_code)]
fn named_pipe_server_credentials<H: std::os::windows::io::AsRawHandle>(
    pipe: &H,
) -> Result<ObservedPeer, PeerCredentialError> {
    use windows_sys::Win32::System::Pipes::GetNamedPipeServerProcessId;

    let mut process_id = 0_u32;
    let raw_handle = pipe.as_raw_handle();
    // SAFETY: the borrowed AsRawHandle object remains alive for this call. The handle is not
    // retained, and process_id points to valid writable storage for the returned process ID.
    let result = unsafe { GetNamedPipeServerProcessId(raw_handle, &mut process_id) };
    if result == 0 {
        let error_code = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
        return Err(PeerCredentialError::Os(error_code));
    }
    if process_id == 0 {
        return Err(PeerCredentialError::InvalidProcessId);
    }

    Ok(ObservedPeer::Windows { process_id })
}

#[cfg(windows)]
pub fn verify_named_pipe_server<H: std::os::windows::io::AsRawHandle>(
    pipe: &H,
    expected_pid: u32,
) -> Result<(), PeerCredentialError> {
    let expected = ExpectedPeer::windows_process(expected_pid)?;
    if !expected.matches(named_pipe_server_credentials(pipe)?) {
        return Err(PeerCredentialError::Mismatch);
    }
    Ok(())
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn expected_process_must_be_nonzero() {
        assert_eq!(
            ExpectedPeer::unix_process(0, 1000, 1000),
            Err(PeerCredentialError::InvalidProcessId)
        );
    }

    #[test]
    fn expectation_checks_process_and_principal() -> Result<(), PeerCredentialError> {
        let expected = ExpectedPeer::unix_process(10, 1000, 1000)?;
        for observed in [
            ObservedPeer::Unix {
                pid: 11,
                uid: 1000,
                gid: 1000,
            },
            ObservedPeer::Unix {
                pid: 10,
                uid: 1001,
                gid: 1000,
            },
            ObservedPeer::Unix {
                pid: 10,
                uid: 1000,
                gid: 1001,
            },
        ] {
            assert!(!expected.matches(observed));
        }
        Ok(())
    }

    #[cfg(any(target_os = "linux", target_os = "android", target_vendor = "apple"))]
    #[test]
    fn anonymous_unix_pair_observes_the_creator() -> Result<(), Box<dyn std::error::Error>> {
        let (supervisor, _worker) = std::os::unix::net::UnixStream::pair()?;
        assert_eq!(
            unix_peer_credentials(&supervisor)?,
            ObservedPeer::Unix {
                pid: std::process::id(),
                uid: nix::unistd::geteuid().as_raw(),
                gid: nix::unistd::getegid().as_raw(),
            }
        );
        Ok(())
    }
}
