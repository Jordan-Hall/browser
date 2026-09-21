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
    InvalidPipeName,
    InvalidPipeBufferSize,
    Mismatch,
    PrincipalMismatch,
}

impl fmt::Display for PeerCredentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlatform => {
                formatter.write_str("peer credential extraction is unsupported on this platform")
            }
            Self::Os(code) => write!(formatter, "peer credential OS call failed with code {code}"),
            Self::InvalidProcessId => formatter.write_str("peer process id was invalid"),
            Self::InvalidPipeName => formatter.write_str("named pipe endpoint name was invalid"),
            Self::InvalidPipeBufferSize => {
                formatter.write_str("named pipe buffer size was outside the supported bound")
            }
            Self::Mismatch => {
                formatter.write_str("connected peer differs from the expected process")
            }
            Self::PrincipalMismatch => {
                formatter.write_str("connected peer runs as a different Windows principal")
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
const WINDOWS_TOKEN_BUFFER_BYTES: usize =
    std::mem::size_of::<windows_sys::Win32::Security::TOKEN_USER>()
        + windows_sys::Win32::Security::SECURITY_MAX_SID_SIZE as usize;

#[cfg(windows)]
#[repr(align(16))]
struct WindowsTokenBuffer([u8; WINDOWS_TOKEN_BUFFER_BYTES]);

#[cfg(windows)]
fn windows_os_error() -> PeerCredentialError {
    PeerCredentialError::Os(
        std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap_or(-1),
    )
}

#[cfg(windows)]
#[allow(unsafe_code)]
fn windows_token_user(process_id: u32) -> Result<WindowsTokenBuffer, PeerCredentialError> {
    use std::mem::{size_of, size_of_val};
    use windows_sys::Win32::{
        Foundation::{CloseHandle, HANDLE},
        Security::{GetTokenInformation, TOKEN_QUERY, TOKEN_USER, TokenUser},
        System::Threading::{OpenProcess, OpenProcessToken, PROCESS_QUERY_LIMITED_INFORMATION},
    };

    struct OwnedHandle(HANDLE);
    impl Drop for OwnedHandle {
        fn drop(&mut self) {
            // SAFETY: each non-null handle is owned by this wrapper and closed exactly once.
            let _ = unsafe { CloseHandle(self.0) };
        }
    }

    if process_id == 0 {
        return Err(PeerCredentialError::InvalidProcessId);
    }
    // SAFETY: OpenProcess is called with a nonzero PID and query-only access; the returned
    // handle is immediately transferred into OwnedHandle when non-null.
    let process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, process_id) };
    if process.is_null() {
        return Err(windows_os_error());
    }
    let process = OwnedHandle(process);
    let mut raw_token: HANDLE = std::ptr::null_mut();
    // SAFETY: process owns a live process handle and raw_token points to writable storage.
    if unsafe { OpenProcessToken(process.0, TOKEN_QUERY, &mut raw_token) } == 0 {
        return Err(windows_os_error());
    }
    if raw_token.is_null() {
        return Err(PeerCredentialError::Os(-1));
    }
    let token = OwnedHandle(raw_token);
    let mut buffer = WindowsTokenBuffer([0; WINDOWS_TOKEN_BUFFER_BYTES]);
    let mut returned = 0_u32;
    // SAFETY: WindowsTokenBuffer is suitably aligned and remains live for the call; its complete
    // writable byte range is supplied. TokenUser is a query-only token information class.
    if unsafe {
        GetTokenInformation(
            token.0,
            TokenUser,
            buffer.0.as_mut_ptr().cast(),
            u32::try_from(size_of_val(&buffer.0)).map_err(|_| PeerCredentialError::Os(-1))?,
            &mut returned,
        )
    } == 0
    {
        return Err(windows_os_error());
    }
    if (returned as usize) < size_of::<TOKEN_USER>() {
        return Err(PeerCredentialError::Os(-1));
    }
    Ok(buffer)
}

#[cfg(windows)]
#[allow(unsafe_code)]
fn windows_sid(
    buffer: &WindowsTokenBuffer,
) -> Result<windows_sys::Win32::Security::PSID, PeerCredentialError> {
    use windows_sys::Win32::Security::{IsValidSid, PSID, TOKEN_USER};

    // SAFETY: windows_token_user accepted only buffers containing at least TOKEN_USER bytes, and
    // the OS-owned SID pointer refers into that still-live buffer for the duration of this check.
    let sid: PSID = unsafe { (*(buffer.0.as_ptr().cast::<TOKEN_USER>())).User.Sid };
    if sid.is_null() || unsafe { IsValidSid(sid) } == 0 {
        return Err(PeerCredentialError::Os(-1));
    }
    Ok(sid)
}

#[cfg(windows)]
#[allow(unsafe_code)]
fn process_uses_current_principal(process_id: u32) -> Result<bool, PeerCredentialError> {
    use windows_sys::Win32::Security::EqualSid;

    let peer = windows_token_user(process_id)?;
    let current = windows_token_user(std::process::id())?;
    let peer_sid = windows_sid(&peer)?;
    let current_sid = windows_sid(&current)?;
    // SAFETY: both validated SID pointers remain backed by live WindowsTokenBuffer values.
    Ok(unsafe { EqualSid(peer_sid, current_sid) } != 0)
}

#[cfg(windows)]
struct OwnedLocalSecurityDescriptor(windows_sys::Win32::Security::PSECURITY_DESCRIPTOR);

#[cfg(windows)]
impl Drop for OwnedLocalSecurityDescriptor {
    fn drop(&mut self) {
        // SAFETY: ConvertStringSecurityDescriptorToSecurityDescriptorW allocated this pointer with
        // LocalAlloc, ownership is unique to this wrapper, and LocalFree is called exactly once.
        let _ = unsafe { windows_sys::Win32::System::Memory::LocalFree(self.0.cast()) };
    }
}

#[cfg(windows)]
#[allow(unsafe_code)]
fn current_user_pipe_security_descriptor()
-> Result<OwnedLocalSecurityDescriptor, PeerCredentialError> {
    use windows_sys::{
        Win32::Security::Authorization::{
            ConvertSidToStringSidW, ConvertStringSecurityDescriptorToSecurityDescriptorW,
            SDDL_REVISION_1,
        },
        Win32::Security::PSECURITY_DESCRIPTOR,
        Win32::System::Memory::LocalFree,
        core::PWSTR,
    };

    let current = windows_token_user(std::process::id())?;
    let sid = windows_sid(&current)?;
    let mut raw_sid_string: PWSTR = std::ptr::null_mut();
    // SAFETY: sid is validated and backed by current for the call; the API writes one LocalAlloc
    // pointer to raw_sid_string on success.
    if unsafe { ConvertSidToStringSidW(sid, &mut raw_sid_string) } == 0 {
        return Err(windows_os_error());
    }
    if raw_sid_string.is_null() {
        return Err(PeerCredentialError::Os(-1));
    }
    struct OwnedLocalWide(PWSTR);
    impl Drop for OwnedLocalWide {
        fn drop(&mut self) {
            // SAFETY: ConvertSidToStringSidW allocated this string with LocalAlloc and ownership is
            // unique to this wrapper.
            let _ = unsafe { LocalFree(self.0.cast()) };
        }
    }
    let raw_sid_string = OwnedLocalWide(raw_sid_string);
    let mut length = 0_usize;
    loop {
        if length >= 256 {
            return Err(PeerCredentialError::Os(-1));
        }
        // SAFETY: ConvertSidToStringSidW guarantees a null-terminated string; SID text is bounded
        // far below 256 UTF-16 code units. We stop at the terminator before constructing the slice.
        if unsafe { *raw_sid_string.0.add(length) } == 0 {
            break;
        }
        length += 1;
    }
    // SAFETY: the preceding bounded scan established exactly length initialized code units before
    // the null terminator and raw_sid_string remains live for this conversion.
    let sid_text = String::from_utf16(unsafe {
        std::slice::from_raw_parts(raw_sid_string.0, length)
    })
    .map_err(|_| PeerCredentialError::Os(-1))?;
    let sddl = format!("D:P(A;;GA;;;{sid_text})");
    let wide_sddl: Vec<u16> = sddl.encode_utf16().chain(Some(0)).collect();
    let mut descriptor: PSECURITY_DESCRIPTOR = std::ptr::null_mut();
    // SAFETY: wide_sddl is null-terminated and remains live for the call. The API returns a
    // LocalAlloc-owned security descriptor through descriptor on success.
    if unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            wide_sddl.as_ptr(),
            SDDL_REVISION_1,
            &mut descriptor,
            std::ptr::null_mut(),
        )
    } == 0
    {
        return Err(windows_os_error());
    }
    if descriptor.is_null() {
        return Err(PeerCredentialError::Os(-1));
    }
    Ok(OwnedLocalSecurityDescriptor(descriptor))
}

#[cfg(windows)]
#[allow(unsafe_code)]
pub fn create_current_user_named_pipe(
    name: &std::ffi::OsStr,
    buffer_bytes: u32,
) -> Result<std::fs::File, PeerCredentialError> {
    use std::os::windows::{ffi::OsStrExt, io::FromRawHandle};
    use windows_sys::Win32::{
        Foundation::INVALID_HANDLE_VALUE,
        Security::SECURITY_ATTRIBUTES,
        Storage::FileSystem::PIPE_ACCESS_DUPLEX,
        System::Pipes::{CreateNamedPipeW, PIPE_NOWAIT, PIPE_REJECT_REMOTE_CLIENTS},
    };

    const MAX_PIPE_BUFFER_BYTES: u32 = 1024 * 1024;
    if buffer_bytes == 0 || buffer_bytes > MAX_PIPE_BUFFER_BYTES {
        return Err(PeerCredentialError::InvalidPipeBufferSize);
    }
    let mut wide_name: Vec<u16> = name.encode_wide().collect();
    let prefix: Vec<u16> = r"\\.\pipe\".encode_utf16().collect();
    if !wide_name.starts_with(&prefix) || wide_name.len() == prefix.len() || wide_name.contains(&0) {
        return Err(PeerCredentialError::InvalidPipeName);
    }
    wide_name.push(0);
    let descriptor = current_user_pipe_security_descriptor()?;
    let attributes = SECURITY_ATTRIBUTES {
        nLength: u32::try_from(std::mem::size_of::<SECURITY_ATTRIBUTES>())
            .map_err(|_| PeerCredentialError::Os(-1))?,
        lpSecurityDescriptor: descriptor.0,
        bInheritHandle: 0,
    };
    // SAFETY: wide_name and attributes are live for the call. The descriptor is a valid
    // LocalAlloc-owned self-relative descriptor with a protected DACL granting only the current
    // user. Inheritance is disabled. A successful handle is transferred exactly once into File.
    let handle = unsafe {
        CreateNamedPipeW(
            wide_name.as_ptr(),
            PIPE_ACCESS_DUPLEX,
            PIPE_NOWAIT | PIPE_REJECT_REMOTE_CLIENTS,
            1,
            buffer_bytes,
            buffer_bytes,
            0,
            &attributes,
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        return Err(windows_os_error());
    }
    // SAFETY: CreateNamedPipeW returned a fresh valid handle and ownership transfers to File.
    Ok(unsafe { std::fs::File::from_raw_handle(handle) })
}

#[cfg(windows)]
fn require_current_windows_principal(process_id: u32) -> Result<(), PeerCredentialError> {
    if !process_uses_current_principal(process_id)? {
        return Err(PeerCredentialError::PrincipalMismatch);
    }
    Ok(())
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
        let error_code = std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap_or(-1);
        return Err(PeerCredentialError::Os(error_code));
    }
    if process_id == 0 {
        return Err(PeerCredentialError::InvalidProcessId);
    }
    require_current_windows_principal(process_id)?;

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
        let error_code = std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap_or(-1);
        return Err(PeerCredentialError::Os(error_code));
    }
    if process_id == 0 {
        return Err(PeerCredentialError::InvalidProcessId);
    }
    require_current_windows_principal(process_id)?;

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

#[cfg(all(test, windows))]
mod windows_tests {
    use super::*;

    #[test]
    fn expected_windows_process_must_be_nonzero() {
        assert_eq!(
            ExpectedPeer::windows_process(0),
            Err(PeerCredentialError::InvalidProcessId)
        );
    }

    #[test]
    fn current_windows_process_principal_matches_itself() -> Result<(), PeerCredentialError> {
        assert!(process_uses_current_principal(std::process::id())?);
        Ok(())
    }
}
