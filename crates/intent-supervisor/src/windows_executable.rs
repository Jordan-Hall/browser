use crate::{
    HealthPolicy, SupervisorError, WindowsAuthenticatedWorker, WindowsPendingWorker,
    WindowsRestartLifecycle, WindowsStopReport,
};
use intent_contracts::{ContentHash, WorkerInstanceId};
use intent_local_transport::{WorkerIdentity, WorkerRole};
use sha2::{Digest, Sha256};
use std::{
    fs::{File, OpenOptions},
    io::Read,
    os::windows::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    process::{Child, Command},
    sync::Arc,
    time::Duration,
};

const MAX_IMAGE_BYTES: u64 = 128 * 1024 * 1024;
const FILE_SHARE_READ: u32 = 0x0000_0001;

/// Hash-verifies a canonical Windows executable path and keeps its file open with write/delete
/// sharing denied while the guard is alive. This pins the approved path against subsequent
/// ordinary replacement or reopening for write; it is an identity primitive, not full sandboxing.
#[derive(Clone, Debug)]
pub struct WindowsExecutableIdentity {
    path: PathBuf,
    _pin: Arc<File>,
    hash: ContentHash,
}

impl WindowsExecutableIdentity {
    pub fn load(path: &Path, expected: ContentHash) -> Result<Self, SupervisorError> {
        let path = std::fs::canonicalize(path)?;
        let mut source = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(&path)?;
        let metadata = source.metadata()?;
        if !metadata.is_file() || metadata.len() > MAX_IMAGE_BYTES {
            return Err(SupervisorError::ExecutableMismatch);
        }

        let mut hash = Sha256::new();
        let mut buffer = [0_u8; 16_384];
        let mut size = 0_u64;
        let mut magic = [0_u8; 2];
        loop {
            let length = source.read(&mut buffer)?;
            if length == 0 {
                break;
            }
            if size < 2 {
                let offset = size as usize;
                let count = length.min(2 - offset);
                magic[offset..offset + count].copy_from_slice(&buffer[..count]);
            }
            size = size
                .checked_add(length as u64)
                .ok_or(SupervisorError::ExecutableMismatch)?;
            if size > MAX_IMAGE_BYTES {
                return Err(SupervisorError::ExecutableMismatch);
            }
            hash.update(&buffer[..length]);
        }

        let actual = ContentHash::from_bytes(hash.finalize().into());
        if size < 2 || magic != *b"MZ" || actual != expected {
            return Err(SupervisorError::ExecutableMismatch);
        }

        Ok(Self {
            path,
            _pin: Arc::new(source),
            hash: actual,
        })
    }

    #[must_use]
    pub const fn hash(&self) -> ContentHash {
        self.hash
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

fn verify_command_program(
    command: &Command,
    executable: &WindowsExecutableIdentity,
) -> Result<(), SupervisorError> {
    let configured = std::fs::canonicalize(Path::new(command.get_program()))
        .map_err(|_| SupervisorError::ExecutableMismatch)?;
    if configured != executable.path() {
        return Err(SupervisorError::ExecutableMismatch);
    }
    Ok(())
}

/// A pending Windows worker whose approved executable identity remains pinned for the whole
/// supervisor-owned generation. Field order is intentional: child ownership is released before
/// the executable pin when this wrapper is dropped.
#[derive(Debug)]
pub struct WindowsPinnedPendingWorker {
    worker: WindowsPendingWorker,
    executable: WindowsExecutableIdentity,
}

impl WindowsPendingWorker {
    pub fn spawn_pinned(
        command: &mut Command,
        executable: WindowsExecutableIdentity,
        generation: WorkerInstanceId,
        role: WorkerRole,
        handshake_timeout: Duration,
    ) -> Result<WindowsPinnedPendingWorker, SupervisorError> {
        verify_command_program(command, &executable)?;
        let worker = Self::spawn(command, generation, role, handshake_timeout)?;
        Ok(WindowsPinnedPendingWorker { worker, executable })
    }
}

impl WindowsPinnedPendingWorker {
    pub fn child_id(&self) -> Result<u32, SupervisorError> {
        self.worker.child_id()
    }

    #[must_use]
    pub const fn executable_identity(&self) -> &WindowsExecutableIdentity {
        &self.executable
    }

    pub fn authenticate(self) -> Result<WindowsPinnedAuthenticatedWorker, SupervisorError> {
        let Self { worker, executable } = self;
        Ok(WindowsPinnedAuthenticatedWorker {
            worker: worker.authenticate()?,
            executable,
        })
    }

    pub fn revoke_before_auth(self) -> Result<std::process::ExitStatus, SupervisorError> {
        let Self {
            worker,
            executable: _executable,
        } = self;
        worker.revoke_before_auth()
    }
}

/// An authenticated Windows worker retaining the approved executable pin until the owned child is
/// stopped/revoked or the pin is explicitly returned with the child parts.
#[derive(Debug)]
pub struct WindowsPinnedAuthenticatedWorker {
    worker: WindowsAuthenticatedWorker,
    executable: WindowsExecutableIdentity,
}

impl WindowsPinnedAuthenticatedWorker {
    pub fn child_id(&self) -> Result<u32, SupervisorError> {
        self.worker.child_id()
    }

    #[must_use]
    pub const fn executable_identity(&self) -> &WindowsExecutableIdentity {
        &self.executable
    }

    #[must_use]
    pub const fn control_identity(&self) -> &WorkerIdentity {
        self.worker.control_identity()
    }

    #[must_use]
    pub const fn progress_identity(&self) -> &WorkerIdentity {
        self.worker.progress_identity()
    }

    pub fn revoke_after_auth(self) -> Result<std::process::ExitStatus, SupervisorError> {
        let Self {
            worker,
            executable: _executable,
        } = self;
        worker.revoke_after_auth()
    }

    pub fn into_parts(
        self,
    ) -> Result<
        (
            Child,
            File,
            File,
            WorkerIdentity,
            WorkerIdentity,
            WindowsExecutableIdentity,
        ),
        SupervisorError,
    > {
        let Self { worker, executable } = self;
        let (child, control, progress, control_identity, progress_identity) =
            worker.into_parts()?;
        Ok((
            child,
            control,
            progress,
            control_identity,
            progress_identity,
            executable,
        ))
    }
}

impl WindowsRestartLifecycle {
    pub fn restart_pinned(
        &mut self,
        command: &mut Command,
        executable: WindowsExecutableIdentity,
        generation: WorkerInstanceId,
        role: WorkerRole,
        handshake_timeout: Duration,
    ) -> Result<WindowsPinnedPendingWorker, SupervisorError> {
        verify_command_program(command, &executable)?;
        let worker = self.restart(command, generation, role, handshake_timeout)?;
        Ok(WindowsPinnedPendingWorker { worker, executable })
    }

    pub fn revoke_pinned_before_auth(
        &mut self,
        worker: WindowsPinnedPendingWorker,
    ) -> Result<std::process::ExitStatus, SupervisorError> {
        let WindowsPinnedPendingWorker {
            worker,
            executable: _executable,
        } = worker;
        self.revoke_before_auth(worker)
    }

    pub fn revoke_pinned_after_auth(
        &mut self,
        worker: WindowsPinnedAuthenticatedWorker,
    ) -> Result<std::process::ExitStatus, SupervisorError> {
        let WindowsPinnedAuthenticatedWorker {
            worker,
            executable: _executable,
        } = worker;
        self.revoke_after_auth(worker)
    }

    pub fn stop_pinned_after_auth(
        &mut self,
        worker: WindowsPinnedAuthenticatedWorker,
        health: HealthPolicy,
    ) -> Result<WindowsStopReport, SupervisorError> {
        let WindowsPinnedAuthenticatedWorker {
            worker,
            executable: _executable,
        } = worker;
        self.stop_after_auth(worker, health)
    }
}
