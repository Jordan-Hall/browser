use intent_contracts::{CancellationId, UnixTimestampMicros};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

pub const MAX_CANCELLATION_RECORDS: usize = 16_384;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationState {
    Active,
    Cancelled { at: UnixTimestampMicros },
}

#[derive(Debug)]
pub struct CancellationRegistry {
    records: BTreeMap<CancellationId, CancellationState>,
    max_records: usize,
}

impl CancellationRegistry {
    #[must_use]
    pub const fn new(max_records: usize) -> Self {
        Self {
            records: BTreeMap::new(),
            max_records,
        }
    }

    pub fn register(&mut self, id: CancellationId) -> Result<(), CancellationError> {
        if self.records.contains_key(&id) {
            return Ok(());
        }
        if self.records.len() >= self.max_records.min(MAX_CANCELLATION_RECORDS) {
            return Err(CancellationError::RegistryFull);
        }
        self.records.insert(id, CancellationState::Active);
        Ok(())
    }

    pub fn cancel(
        &mut self,
        id: CancellationId,
        at: UnixTimestampMicros,
    ) -> Result<bool, CancellationError> {
        let Some(state) = self.records.get_mut(&id) else {
            return Err(CancellationError::UnknownId(id));
        };
        if matches!(state, CancellationState::Cancelled { .. }) {
            return Ok(false);
        }
        *state = CancellationState::Cancelled { at };
        Ok(true)
    }

    #[must_use]
    pub fn state(&self, id: CancellationId) -> Option<CancellationState> {
        self.records.get(&id).copied()
    }

    #[must_use]
    pub fn is_cancelled(&self, id: CancellationId) -> bool {
        matches!(
            self.records.get(&id),
            Some(CancellationState::Cancelled { .. })
        )
    }

    pub fn remove(&mut self, id: CancellationId) -> Result<CancellationState, CancellationError> {
        self.records
            .remove(&id)
            .ok_or(CancellationError::UnknownId(id))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeadlineStatus {
    Open,
    Expired,
}

#[must_use]
pub fn deadline_status(
    deadline: Option<UnixTimestampMicros>,
    now: UnixTimestampMicros,
) -> DeadlineStatus {
    match deadline {
        Some(deadline) if now.get() >= deadline.get() => DeadlineStatus::Expired,
        Some(_) | None => DeadlineStatus::Open,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationError {
    RegistryFull,
    UnknownId(CancellationId),
}

impl fmt::Display for CancellationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegistryFull => formatter.write_str("cancellation registry is full"),
            Self::UnknownId(id) => write!(formatter, "unknown cancellation id {id}"),
        }
    }
}

impl Error for CancellationError {}

#[cfg(test)]
mod tests {
    use super::{CancellationRegistry, CancellationState, DeadlineStatus, deadline_status};
    use intent_contracts::{CancellationId, UnixTimestampMicros};
    use std::error::Error;
    use std::str::FromStr;

    fn cancellation_id() -> Result<CancellationId, Box<dyn Error>> {
        Ok(CancellationId::from_str(
            "018f47f7-5a86-7c00-8000-000000000701",
        )?)
    }

    #[test]
    fn cancellation_is_idempotent_after_first_transition() -> Result<(), Box<dyn Error>> {
        let id = cancellation_id()?;
        let now = UnixTimestampMicros::try_new(10)?;
        let mut registry = CancellationRegistry::new(4);
        registry.register(id)?;

        assert!(registry.cancel(id, now)?);
        assert!(!registry.cancel(id, now)?);
        assert_eq!(
            registry.state(id),
            Some(CancellationState::Cancelled { at: now })
        );
        Ok(())
    }

    #[test]
    fn bounded_registry_fails_closed() -> Result<(), Box<dyn Error>> {
        let mut registry = CancellationRegistry::new(1);
        registry.register(cancellation_id()?)?;
        let second = CancellationId::from_str("018f47f7-5a86-7c00-8000-000000000702")?;
        assert!(registry.register(second).is_err());
        Ok(())
    }

    #[test]
    fn deadlines_expire_at_or_after_the_boundary() -> Result<(), Box<dyn Error>> {
        let deadline = UnixTimestampMicros::try_new(100)?;
        assert_eq!(
            deadline_status(Some(deadline), UnixTimestampMicros::try_new(99)?),
            DeadlineStatus::Open
        );
        assert_eq!(
            deadline_status(Some(deadline), UnixTimestampMicros::try_new(100)?),
            DeadlineStatus::Expired
        );
        assert_eq!(
            deadline_status(None, UnixTimestampMicros::try_new(1_000)?),
            DeadlineStatus::Open
        );
        Ok(())
    }
}
