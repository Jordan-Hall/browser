use intent_contracts::{AccountId, ContentHash, SchemaVersion, TaskId};
use sha2::{Digest, Sha256};
use std::{
    collections::VecDeque,
    error::Error,
    fmt,
    time::{Duration, Instant},
};

pub const MAX_REPLAY_ENTRIES: usize = 4096;
pub const MAX_REPLAY_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_REPLAY_ENTRY_BYTES: usize = 1024 * 1024;
pub const MAX_REPLAY_DURATION: Duration = Duration::from_secs(60);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayLimits {
    entries: usize,
    bytes: usize,
    entry_bytes: usize,
    elapsed: Duration,
}

impl ReplayLimits {
    pub fn try_new(
        entries: usize,
        bytes: usize,
        entry_bytes: usize,
        elapsed: Duration,
    ) -> Result<Self, ReplayError> {
        if entries == 0
            || entries > MAX_REPLAY_ENTRIES
            || bytes == 0
            || bytes > MAX_REPLAY_BYTES
            || entry_bytes == 0
            || entry_bytes > MAX_REPLAY_ENTRY_BYTES
            || entry_bytes > bytes
            || elapsed.is_zero()
            || elapsed > MAX_REPLAY_DURATION
        {
            return Err(ReplayError::InvalidLimits);
        }
        Ok(Self {
            entries,
            bytes,
            entry_bytes,
            elapsed,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapturedKind {
    Observation,
    Evidence,
}

#[derive(Clone, Eq, PartialEq)]
pub struct CapturedRecord {
    pub schema: SchemaVersion,
    pub task_id: TaskId,
    pub account_id: AccountId,
    pub sequence: u64,
    pub kind: CapturedKind,
    pub hash: ContentHash,
    pub bytes: Vec<u8>,
}

/// Output is explicitly captured data, not a live observation or a dispatch
/// request. This type has no conversion to a sendable operation.
#[derive(Clone, Eq, PartialEq)]
pub struct ReplayedRecord {
    sequence: u64,
    kind: CapturedKind,
    hash: ContentHash,
    bytes: Vec<u8>,
}

impl fmt::Debug for CapturedRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CapturedRecord")
            .field("sequence", &self.sequence)
            .field("kind", &self.kind)
            .field("byte_count", &self.bytes.len())
            .finish_non_exhaustive()
    }
}
impl fmt::Debug for ReplayedRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReplayedRecord")
            .field("sequence", &self.sequence)
            .field("kind", &self.kind)
            .field("byte_count", &self.bytes.len())
            .finish_non_exhaustive()
    }
}

impl ReplayedRecord {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }
    #[must_use]
    pub const fn kind(&self) -> CapturedKind {
        self.kind
    }
    #[must_use]
    pub const fn hash(&self) -> ContentHash {
        self.hash
    }
    #[must_use]
    pub fn captured_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// A replay owns only verified captured bytes and a monotonic budget. There is
/// deliberately no provider, callback, credential, filesystem or process handle.
#[derive(Debug)]
pub struct NoProductionReplay {
    entries: VecDeque<ReplayedRecord>,
    started: Instant,
    limits: ReplayLimits,
    remaining_bytes: usize,
    expired: bool,
}

impl NoProductionReplay {
    pub fn from_captures(
        task: TaskId,
        account: AccountId,
        captures: impl IntoIterator<Item = CapturedRecord>,
        limits: ReplayLimits,
    ) -> Result<Self, ReplayError> {
        let started = Instant::now();
        let mut entries = VecDeque::new();
        let mut previous = None;
        let mut remaining_bytes = 0usize;
        for (index, entry) in captures.into_iter().take(limits.entries + 1).enumerate() {
            if started.elapsed() >= limits.elapsed {
                return Err(ReplayError::Expired);
            }
            if index == limits.entries {
                return Err(ReplayError::EntryLimit);
            }
            if entry.schema != SchemaVersion::V1 {
                return Err(ReplayError::UnsupportedSchema);
            }
            if entry.task_id != task || entry.account_id != account {
                return Err(ReplayError::ScopeMismatch);
            }
            if previous.is_some_and(|previous| entry.sequence <= previous) {
                return Err(ReplayError::UnorderedSequence);
            }
            if entry.bytes.len() > limits.entry_bytes {
                return Err(ReplayError::ByteLimit);
            }
            remaining_bytes = remaining_bytes
                .checked_add(entry.bytes.len())
                .ok_or(ReplayError::ByteLimit)?;
            if remaining_bytes > limits.bytes {
                return Err(ReplayError::ByteLimit);
            }
            let digest: [u8; 32] = Sha256::digest(&entry.bytes).into();
            if entry.hash != ContentHash::from_bytes(digest) {
                return Err(ReplayError::CorruptCapture);
            }
            previous = Some(entry.sequence);
            entries.push_back(ReplayedRecord {
                sequence: entry.sequence,
                kind: entry.kind,
                hash: entry.hash,
                bytes: entry.bytes,
            });
        }
        Ok(Self {
            entries,
            started,
            limits,
            remaining_bytes,
            expired: false,
        })
    }

    pub fn step(&mut self) -> Result<Option<ReplayedRecord>, ReplayError> {
        self.step_at(Instant::now())
    }

    fn step_at(&mut self, now: Instant) -> Result<Option<ReplayedRecord>, ReplayError> {
        if self.expired || now.saturating_duration_since(self.started) >= self.limits.elapsed {
            self.expired = true;
            self.entries.clear();
            self.remaining_bytes = 0;
            return Err(ReplayError::Expired);
        }
        let record = self.entries.pop_front();
        if let Some(record) = &record {
            self.remaining_bytes -= record.bytes.len();
        }
        Ok(record)
    }

    #[must_use]
    pub fn remaining_entries(&self) -> usize {
        self.entries.len()
    }
    #[must_use]
    pub const fn remaining_bytes(&self) -> usize {
        self.remaining_bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayError {
    InvalidLimits,
    EntryLimit,
    ByteLimit,
    UnsupportedSchema,
    ScopeMismatch,
    UnorderedSequence,
    CorruptCapture,
    Expired,
}
impl fmt::Display for ReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "captured replay rejected: {self:?}")
    }
}
impl Error for ReplayError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expiration_releases_captured_buffers_and_is_sticky() -> Result<(), Box<dyn Error>> {
        let task = "018f47f7-5a86-7c00-8000-000000000001".parse()?;
        let account = "018f47f7-5a86-7c00-8000-000000000002".parse()?;
        let bytes = b"captured".to_vec();
        let record = CapturedRecord {
            schema: SchemaVersion::V1,
            task_id: task,
            account_id: account,
            sequence: 1,
            kind: CapturedKind::Observation,
            hash: ContentHash::from_bytes(Sha256::digest(&bytes).into()),
            bytes,
        };
        let mut replay = NoProductionReplay::from_captures(
            task,
            account,
            [record],
            ReplayLimits::try_new(1, 8, 8, Duration::from_secs(1))?,
        )?;
        assert_eq!(
            replay.step_at(replay.started + Duration::from_secs(1)),
            Err(ReplayError::Expired)
        );
        assert_eq!(replay.remaining_bytes(), 0);
        assert_eq!(replay.remaining_entries(), 0);
        assert_eq!(replay.step_at(replay.started), Err(ReplayError::Expired));
        Ok(())
    }
}
