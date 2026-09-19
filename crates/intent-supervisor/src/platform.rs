use crate::{ProcessLimits, SupervisorError};
use intent_contracts::ContentHash;
use nix::{
    fcntl::{FcntlArg, OFlag, SealFlag, fcntl},
    sys::{
        memfd::{MFdFlags, memfd_create},
        prctl,
        resource::{Resource, setrlimit},
        signal::{Signal, killpg},
        wait::{Id, WaitPidFlag, WaitStatus, waitid},
    },
    unistd::{Pid, getpid},
};
use sha2::{Digest, Sha256};
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    os::{
        fd::AsRawFd,
        unix::{fs::OpenOptionsExt, process::CommandExt},
    },
    path::Path,
    process::{Child, Command, ExitStatus},
    sync::Arc,
};

const MAX_IMAGE_BYTES: u64 = 128 * 1024 * 1024;

#[derive(Clone, Debug)]
pub struct ExecutableImage {
    file: Arc<File>,
    hash: ContentHash,
}
impl ExecutableImage {
    /// Copy and seal the exact approved ELF image. Later path replacement cannot change execution.
    /// The system loader and shared libraries remain part of the trusted host boundary.
    pub fn load(path: &Path, expected: ContentHash) -> Result<Self, SupervisorError> {
        let mut source = OpenOptions::new()
            .read(true)
            .custom_flags((OFlag::O_NOFOLLOW | OFlag::O_NONBLOCK).bits())
            .open(path)?;
        let metadata = source.metadata()?;
        if !metadata.is_file() || metadata.len() > MAX_IMAGE_BYTES {
            return Err(SupervisorError::ExecutableMismatch);
        }
        let fd = memfd_create(
            c"intent-approved-worker",
            MFdFlags::MFD_CLOEXEC | MFdFlags::MFD_ALLOW_SEALING,
        )
        .map_err(io::Error::from)?;
        let mut file = File::from(fd);
        let mut hash = Sha256::new();
        let mut buffer = [0_u8; 16384];
        let mut size = 0_u64;
        let mut magic = [0_u8; 4];
        loop {
            let length = source.read(&mut buffer)?;
            if length == 0 {
                break;
            }
            if size < 4 {
                let offset = size as usize;
                let count = length.min(4 - offset);
                magic[offset..offset + count].copy_from_slice(&buffer[..count]);
            }
            size = size
                .checked_add(length as u64)
                .ok_or(SupervisorError::ExecutableMismatch)?;
            if size > MAX_IMAGE_BYTES {
                return Err(SupervisorError::ExecutableMismatch);
            }
            hash.update(&buffer[..length]);
            file.write_all(&buffer[..length])?;
        }
        let actual = ContentHash::from_bytes(hash.finalize().into());
        if size < 4 || magic != *b"\x7fELF" || actual != expected {
            return Err(SupervisorError::ExecutableMismatch);
        }
        let seals = SealFlag::F_SEAL_WRITE
            | SealFlag::F_SEAL_GROW
            | SealFlag::F_SEAL_SHRINK
            | SealFlag::F_SEAL_SEAL;
        fcntl(&file, FcntlArg::F_ADD_SEALS(seals)).map_err(io::Error::from)?;
        if fcntl(&file, FcntlArg::F_GET_SEALS).map_err(io::Error::from)? & seals.bits()
            != seals.bits()
        {
            return Err(SupervisorError::ExecutableMismatch);
        }
        Ok(Self {
            file: Arc::new(file),
            hash: actual,
        })
    }
    pub const fn hash(&self) -> ContentHash {
        self.hash
    }
    pub(crate) fn command(&self, limits: ProcessLimits) -> Command {
        let mut command = Command::new(format!("/proc/self/fd/{}", self.file.as_raw_fd()));
        command
            .arg0("intent-approved-worker")
            .env_clear()
            .process_group(0);
        let parent = getpid();
        // SAFETY: this Linux-only hook captures integers, performs only prctl/setrlimit/getppid
        // and close_range syscalls, and constructs raw-errno errors without allocation, locks or logging.
        // The parent's sealed image FD remains live through spawn; CLOEXEC closes it only after
        // exec has resolved the ELF image. No Rust resources are accessed mutably in the hook.
        unsafe {
            command.pre_exec(move || {
                // Mark all non-stdio descriptors close-on-exec, including descriptors opened by
                // the embedding application without CLOEXEC. Keep the sealed image/error pipe
                // alive until exec resolves it; failure on older kernels is a launch failure.
                if nix::libc::syscall(
                    nix::libc::SYS_close_range,
                    3_u32,
                    u32::MAX,
                    nix::libc::CLOSE_RANGE_CLOEXEC,
                ) != 0
                {
                    return Err(io::Error::last_os_error());
                }
                prctl::set_pdeathsig(Signal::SIGKILL).map_err(io::Error::from)?;
                if nix::unistd::getppid() != parent {
                    return Err(io::Error::from_raw_os_error(nix::libc::ESRCH));
                }
                prctl::set_no_new_privs().map_err(io::Error::from)?;
                setrlimit(Resource::RLIMIT_CORE, 0, 0).map_err(io::Error::from)?;
                setrlimit(
                    Resource::RLIMIT_AS,
                    limits.address_space_bytes(),
                    limits.address_space_bytes(),
                )
                .map_err(io::Error::from)?;
                setrlimit(
                    Resource::RLIMIT_CPU,
                    limits.cpu_seconds(),
                    limits.cpu_seconds(),
                )
                .map_err(io::Error::from)?;
                setrlimit(
                    Resource::RLIMIT_NOFILE,
                    limits.file_descriptors(),
                    limits.file_descriptors(),
                )
                .map_err(io::Error::from)?;
                Ok(())
            });
        }
        command
    }
}

#[derive(Debug)]
pub(crate) struct ManagedChild {
    child: Child,
    reaped: bool,
}
impl ManagedChild {
    pub(crate) fn new(child: Child) -> Self {
        Self {
            child,
            reaped: false,
        }
    }
    pub(crate) fn id(&self) -> u32 {
        self.child.id()
    }
    pub(crate) fn signal_group(&self, signal: Signal) -> io::Result<()> {
        if self.reaped {
            return Ok(());
        }
        let pid = i32::try_from(self.child.id())
            .map_err(|_| io::Error::from_raw_os_error(nix::libc::ESRCH))?;
        match killpg(Pid::from_raw(pid), signal) {
            Ok(()) | Err(nix::errno::Errno::ESRCH) => Ok(()),
            Err(error) => Err(io::Error::from(error)),
        }
    }
    pub(crate) fn poll_exit(&mut self) -> io::Result<Option<ExitStatus>> {
        if self.reaped {
            return Ok(None);
        }
        let pid = Pid::from_raw(self.child.id() as i32);
        match waitid(
            Id::Pid(pid),
            WaitPidFlag::WEXITED | WaitPidFlag::WNOHANG | WaitPidFlag::WNOWAIT,
        )
        .map_err(io::Error::from)?
        {
            WaitStatus::StillAlive => Ok(None),
            _ => {
                // The unreaped leader pins the PID/PGID while cooperative descendants are killed.
                self.signal_group(Signal::SIGKILL)?;
                let status = self.child.wait()?;
                self.reaped = true;
                Ok(Some(status))
            }
        }
    }
}
impl Drop for ManagedChild {
    fn drop(&mut self) {
        if !self.reaped {
            let _ = self.signal_group(Signal::SIGKILL);
            // Poll-based shutdown should reap before drop. Drop is a final ownership cleanup,
            // not the latency-qualified control API; kernel-uninterruptible children can delay it.
            let _ = self.child.wait();
            self.reaped = true;
        }
    }
}
