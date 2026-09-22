use nix::{
    dir::Dir,
    fcntl::{OFlag, RenameFlags, open, openat, renameat2},
    sys::stat::{Mode, mkdirat},
    unistd::{UnlinkatFlags, geteuid, unlinkat},
};
use std::{
    fs::{File, TryLockError},
    io,
    os::{fd::AsRawFd, unix::fs::MetadataExt},
    path::{Component, Path, PathBuf},
    sync::OnceLock,
    thread,
    time::Duration,
};
use uuid::Uuid;

const DIRECTORY_FLAGS: OFlag = OFlag::O_RDONLY
    .union(OFlag::O_DIRECTORY)
    .union(OFlag::O_NOFOLLOW)
    .union(OFlag::O_CLOEXEC);
const PROFILE_OWNER_LOCK_FILE: &str = ".intent-profile-owner.lock";
const PROFILE_LOCK_RETRIES: usize = 16;
const PROFILE_LOCK_RETRY_DELAY: Duration = Duration::from_millis(1);

fn retry_profile_lock(mut attempt: impl FnMut() -> Result<(), TryLockError>) -> io::Result<()> {
    for retry in 0..=PROFILE_LOCK_RETRIES {
        match attempt() {
            Ok(()) => return Ok(()),
            Err(TryLockError::WouldBlock) if retry < PROFILE_LOCK_RETRIES => {
                thread::sleep(PROFILE_LOCK_RETRY_DELAY);
            }
            Err(error) => return Err(io::Error::other(error.to_string())),
        }
    }
    unreachable!("bounded lock retry loop always returns")
}

#[derive(Debug)]
pub(crate) struct Directory {
    file: File,
    profile_owner_lock: OnceLock<File>,
}

impl Directory {
    fn from_file(file: File) -> Self {
        Self {
            file,
            profile_owner_lock: OnceLock::new(),
        }
    }

    pub fn open_private(path: &Path) -> io::Result<Self> {
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };
        let mut current = Self::from_file(File::from(open("/", DIRECTORY_FLAGS, Mode::empty())?));
        for component in absolute.components() {
            match component {
                Component::RootDir | Component::CurDir => {}
                Component::Normal(name) => {
                    current = Self::from_file(File::from(openat(
                        &current.file,
                        name,
                        DIRECTORY_FLAGS,
                        Mode::empty(),
                    )?));
                }
                _ => return Err(invalid("parent traversal is not allowed")),
            }
        }
        let metadata = current.file.metadata()?;
        if metadata.uid() != geteuid().as_raw() || metadata.mode() & 0o077 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "snapshot roots must be owned by this user with no group/other permissions",
            ));
        }
        Ok(current)
    }

    pub fn child(&self, name: &str) -> io::Result<Self> {
        check_name(name)?;
        Ok(Self::from_file(File::from(openat(
            &self.file,
            name,
            DIRECTORY_FLAGS,
            Mode::empty(),
        )?)))
    }

    pub fn create_child(&self, name: &str) -> io::Result<Self> {
        check_name(name)?;
        mkdirat(&self.file, name, Mode::from_bits_truncate(0o700))?;
        let child = self.child(name)?;
        child.sync()?;
        self.sync()?;
        Ok(child)
    }

    pub fn open_file(&self, name: &str) -> io::Result<File> {
        check_name(name)?;
        let file = File::from(openat(
            &self.file,
            name,
            OFlag::O_RDONLY | OFlag::O_NOFOLLOW | OFlag::O_CLOEXEC | OFlag::O_NONBLOCK,
            Mode::empty(),
        )?);
        if !file.metadata()?.is_file() {
            return Err(invalid("snapshot entry is not a regular file"));
        }
        Ok(file)
    }

    pub fn create_file(&self, name: &str) -> io::Result<File> {
        check_name(name)?;
        Ok(File::from(openat(
            &self.file,
            name,
            OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_EXCL | OFlag::O_NOFOLLOW | OFlag::O_CLOEXEC,
            Mode::from_bits_truncate(0o600),
        )?))
    }

    pub fn names(&self, maximum: usize) -> io::Result<Vec<String>> {
        let mut directory = Dir::openat(&self.file, ".", DIRECTORY_FLAGS, Mode::empty())?;
        let mut names = Vec::new();
        for entry in directory.iter() {
            let entry = entry?;
            let name = entry
                .file_name()
                .to_str()
                .map_err(|_| invalid("non-UTF-8 snapshot entry"))?;
            if matches!(name, "." | "..") {
                continue;
            }
            if names.len() >= maximum {
                return Err(invalid("snapshot directory entry budget exceeded"));
            }
            names.push(name.to_owned());
        }
        names.sort();
        Ok(names)
    }

    pub fn lock_profile(&self) -> io::Result<File> {
        let owner_lock = File::from(openat(
            &self.file,
            PROFILE_OWNER_LOCK_FILE,
            OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_NOFOLLOW | OFlag::O_CLOEXEC,
            Mode::from_bits_truncate(0o600),
        )?);
        let owner_metadata = owner_lock.metadata()?;
        if !owner_metadata.is_file()
            || owner_metadata.nlink() != 1
            || owner_metadata.uid() != geteuid().as_raw()
            || owner_metadata.mode() & 0o077 != 0
        {
            return Err(invalid(
                "profile owner lock must be a private, singly linked regular file",
            ));
        }
        retry_profile_lock(|| owner_lock.try_lock())?;
        self.profile_owner_lock.set(owner_lock).map_err(|_| {
            io::Error::new(
                io::ErrorKind::AlreadyExists,
                "profile owner lock already acquired by this directory handle",
            )
        })?;

        retry_profile_lock(|| self.file.try_lock())?;
        let file = File::from(openat(
            &self.file,
            "state.sqlite3",
            OFlag::O_RDWR
                | OFlag::O_CREAT
                | OFlag::O_NOFOLLOW
                | OFlag::O_CLOEXEC
                | OFlag::O_NONBLOCK,
            Mode::from_bits_truncate(0o600),
        )?);
        let metadata = file.metadata()?;
        if !metadata.is_file()
            || metadata.nlink() != 1
            || metadata.uid() != geteuid().as_raw()
            || metadata.mode() & 0o077 != 0
        {
            return Err(invalid(
                "profile database must be a private, singly linked regular file",
            ));
        }
        retry_profile_lock(|| file.try_lock())?;
        self.sync()?;
        Ok(file)
    }

    pub fn sqlite_path(&self) -> PathBuf {
        PathBuf::from(format!(
            "/proc/self/fd/{}/state.sqlite3",
            self.file.as_raw_fd()
        ))
    }

    pub fn sync(&self) -> io::Result<()> {
        self.file.sync_all()
    }

    fn remove_tree(&self, name: &str, depth: usize) -> io::Result<()> {
        if depth > 4 {
            return Err(invalid("unexpected snapshot staging depth"));
        }
        match self.child(name) {
            Ok(child) => {
                for entry in child.names(8192)? {
                    child.remove_tree(&entry, depth + 1)?;
                }
                unlinkat(&self.file, name, UnlinkatFlags::RemoveDir)?;
            }
            Err(_) => unlinkat(&self.file, name, UnlinkatFlags::NoRemoveDir)?,
        }
        Ok(())
    }
}

pub(crate) struct StagingDirectory {
    parent: Directory,
    name: String,
    pub directory: Directory,
    published: bool,
}

impl StagingDirectory {
    pub fn new(parent: Directory) -> io::Result<Self> {
        let name = format!(".snapshot-pending-{}", Uuid::new_v4());
        let directory = parent.create_child(&name)?;
        Ok(Self {
            parent,
            name,
            directory,
            published: false,
        })
    }

    pub fn publish(&mut self, name: &str) -> io::Result<()> {
        check_name(name)?;
        self.directory.sync()?;
        renameat2(
            &self.parent.file,
            self.name.as_str(),
            &self.parent.file,
            name,
            RenameFlags::RENAME_NOREPLACE,
        )?;
        self.published = true;
        Ok(())
    }

    pub fn sync_parent(&self) -> io::Result<()> {
        self.parent.sync()
    }
}

impl Drop for StagingDirectory {
    fn drop(&mut self) {
        if !self.published {
            let _ = self.parent.remove_tree(&self.name, 0);
            let _ = self.parent.sync();
        }
    }
}

pub(crate) fn check_name(name: &str) -> io::Result<()> {
    if name.is_empty()
        || name.len() > 255
        || name.contains('/')
        || name.contains('\0')
        || matches!(name, "." | "..")
    {
        Err(invalid("invalid snapshot component"))
    } else {
        Ok(())
    }
}

fn invalid(detail: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, detail)
}

#[cfg(test)]
mod tests {
    use super::{PROFILE_LOCK_RETRIES, retry_profile_lock};
    use std::{cell::Cell, fs::TryLockError, io};

    #[test]
    fn transient_profile_lock_contention_is_retried() -> io::Result<()> {
        let attempts = Cell::new(0_usize);
        retry_profile_lock(|| {
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
    fn persistent_profile_lock_contention_still_fails_closed() {
        let attempts = Cell::new(0_usize);
        let result = retry_profile_lock(|| {
            attempts.set(attempts.get() + 1);
            Err(TryLockError::WouldBlock)
        });
        assert!(
            result.is_err(),
            "persistent lock contention unexpectedly succeeded"
        );
        assert_eq!(
            result.err().map(|error| error.kind()),
            Some(io::ErrorKind::Other)
        );
        assert_eq!(attempts.get(), PROFILE_LOCK_RETRIES + 1);
    }
}
