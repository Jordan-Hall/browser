use std::{
    fs::{self, File, OpenOptions, TryLockError},
    io,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

const LOCK_FILE_NAME: &str = ".intent-profile-owner.lock";
const LOCK_RETRIES: usize = 16;
const LOCK_RETRY_DELAY: Duration = Duration::from_millis(1);

/// Cooperative lifetime ownership for one canonical profile root.
///
/// This guard is intentionally portable and does not claim hostile-user containment or
/// filesystem durability. Runtime entry points that mutate profile state should hold one for
/// their full lifetime; process termination releases the OS file lock.
#[derive(Debug)]
pub struct ProfileOwnerGuard {
    canonical_root: PathBuf,
    _lock: File,
}

impl ProfileOwnerGuard {
    pub fn acquire(root: impl AsRef<Path>) -> io::Result<Self> {
        let canonical_root = root.as_ref().canonicalize()?;
        if !fs::metadata(&canonical_root)?.is_dir() {
            return Err(invalid("profile root is not a directory"));
        }

        let lock_path = canonical_root.join(LOCK_FILE_NAME);
        if let Ok(metadata) = fs::symlink_metadata(&lock_path)
            && (metadata.file_type().is_symlink() || !metadata.is_file())
        {
            return Err(invalid("profile owner lock is not a regular file"));
        }

        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(lock_path)?;
        if !lock.metadata()?.is_file() {
            return Err(invalid("profile owner lock is not a regular file"));
        }
        retry_lock(|| lock.try_lock())?;

        Ok(Self {
            canonical_root,
            _lock: lock,
        })
    }

    pub fn canonical_root(&self) -> &Path {
        &self.canonical_root
    }
}

fn retry_lock(mut attempt: impl FnMut() -> Result<(), TryLockError>) -> io::Result<()> {
    for retry in 0..=LOCK_RETRIES {
        match attempt() {
            Ok(()) => return Ok(()),
            Err(TryLockError::WouldBlock) if retry < LOCK_RETRIES => {
                thread::sleep(LOCK_RETRY_DELAY);
            }
            Err(error) => return Err(io::Error::other(error.to_string())),
        }
    }
    unreachable!("bounded profile owner lock retry always returns")
}

fn invalid(detail: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, detail)
}

#[cfg(test)]
mod tests {
    use super::{LOCK_RETRIES, retry_lock};
    use std::{cell::Cell, fs::TryLockError, io};

    #[test]
    fn transient_contention_is_retried() -> io::Result<()> {
        let attempts = Cell::new(0_usize);
        retry_lock(|| {
            let attempt = attempts.get();
            attempts.set(attempt + 1);
            if attempt < 2 {
                Err(TryLockError::WouldBlock)
            } else {
                Ok(())
            }
        })?;
        assert_eq!(attempts.get(), 3);
        Ok(())
    }

    #[test]
    fn persistent_contention_fails_closed() {
        let attempts = Cell::new(0_usize);
        let result = retry_lock(|| {
            attempts.set(attempts.get() + 1);
            Err(TryLockError::WouldBlock)
        });
        assert!(result.is_err());
        assert_eq!(attempts.get(), LOCK_RETRIES + 1);
    }
}
