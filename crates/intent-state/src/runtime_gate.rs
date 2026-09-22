use crate::{OutboxError, StateError, StateStore};
use rusqlite::Connection;
use std::{
    fs::{self, File, OpenOptions, TryLockError},
    io,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    thread,
    time::Duration,
};
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use uuid::Uuid;

const PROFILE_OWNER_LOCK_FILE: &str = ".intent-profile-owner.lock";
const PROFILE_OWNER_LOCK_RETRIES: usize = 16;
const PROFILE_OWNER_LOCK_RETRY_DELAY: Duration = Duration::from_millis(1);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatchStatus {
    pub epoch: Uuid,
    pub enabled: bool,
    pub reason: String,
}

#[derive(Debug)]
struct OwnedProfileStateStore {
    store: StateStore,
    _owner_lock: File,
    _canonical_root: PathBuf,
}

impl OwnedProfileStateStore {
    fn open(root: impl AsRef<Path>) -> io::Result<Self> {
        let canonical_root = root.as_ref().canonicalize()?;
        if !fs::metadata(&canonical_root)?.is_dir() {
            return Err(invalid_owner("profile root is not a directory"));
        }
        let owner_lock = acquire_owner_lock(&canonical_root)?;
        let store = StateStore::open(canonical_root.join("state.sqlite3"))
            .map_err(|error| io::Error::other(error.to_string()))?;
        Ok(Self {
            store,
            _owner_lock: owner_lock,
            _canonical_root: canonical_root,
        })
    }
}

impl Deref for OwnedProfileStateStore {
    type Target = StateStore;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl DerefMut for OwnedProfileStateStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}

impl StateStore {
    /// Opens the canonical profile database while holding one cooperative process owner lock.
    ///
    /// The sidecar lock is portable across supported native platforms and is released by process
    /// termination. It coordinates trusted runtime owners; it is not a hostile-user sandbox or a
    /// filesystem power-loss guarantee.
    pub fn open_owned_profile(
        root: impl AsRef<Path>,
    ) -> io::Result<impl DerefMut<Target = StateStore>> {
        OwnedProfileStateStore::open(root)
    }

    pub fn dispatch_status(&self) -> Result<DispatchStatus, StateError> {
        let (epoch, enabled, reason): (String, bool, String) = self.connection.query_row(
            "SELECT epoch, dispatch_enabled, reason FROM runtime_control WHERE singleton = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;
        Ok(DispatchStatus {
            epoch: Uuid::parse_str(&epoch).map_err(|_| StateError::InvalidStoreId(epoch))?,
            enabled,
            reason,
        })
    }
}

fn acquire_owner_lock(canonical_root: &Path) -> io::Result<File> {
    let lock_path = canonical_root.join(PROFILE_OWNER_LOCK_FILE);
    if let Ok(metadata) = fs::symlink_metadata(&lock_path)
        && (metadata.file_type().is_symlink() || !metadata.is_file())
    {
        return Err(invalid_owner("profile owner lock is not a regular file"));
    }

    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true);
    #[cfg(unix)]
    options.mode(0o600);
    let lock = options.open(lock_path)?;
    if !lock.metadata()?.is_file() {
        return Err(invalid_owner("profile owner lock is not a regular file"));
    }
    retry_owner_lock(|| lock.try_lock())?;
    Ok(lock)
}

fn retry_owner_lock(mut attempt: impl FnMut() -> Result<(), TryLockError>) -> io::Result<()> {
    for retry in 0..=PROFILE_OWNER_LOCK_RETRIES {
        match attempt() {
            Ok(()) => return Ok(()),
            Err(TryLockError::WouldBlock) if retry < PROFILE_OWNER_LOCK_RETRIES => {
                thread::sleep(PROFILE_OWNER_LOCK_RETRY_DELAY);
            }
            Err(error) => return Err(io::Error::other(error.to_string())),
        }
    }
    unreachable!("bounded profile owner lock retry always returns")
}

fn invalid_owner(detail: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, detail)
}

pub(crate) fn require_dispatch(
    connection: &Connection,
    runtime_epoch: Option<Uuid>,
) -> Result<(), OutboxError> {
    let (epoch, enabled): (String, bool) = connection.query_row(
        "SELECT epoch, dispatch_enabled FROM runtime_control WHERE singleton = 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    if enabled && runtime_epoch.is_some_and(|expected| expected.to_string() == epoch) {
        Ok(())
    } else {
        Err(OutboxError::RecoveryRequired)
    }
}
