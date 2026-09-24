use crate::SupervisorError;
use intent_contracts::ContentHash;
use sha2::{Digest, Sha256};
use std::{
    fs::{File, OpenOptions},
    io::Read,
    os::windows::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    sync::Arc,
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
