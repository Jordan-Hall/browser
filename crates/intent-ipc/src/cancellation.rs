use intent_contracts::{CancellationId, UnixTimestampMicros, WorkerInstanceId};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

pub const MAX_CANCELLATION_RECORDS: usize = 16_384;

/// Binds a cancellation identity to the worker generation that owns it.
///
/// A registration is not authority by itself: it must be present and active in
/// the receiving endpoint's bounded registry. Reusing the same cancellation ID
/// for a replacement worker therefore cannot make a late old-generation cancel
/// apply to the replacement.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CancellationRegistration {
    cancellation_id: CancellationId,
    generation: WorkerInstanceId,
}

impl CancellationRegistration {
    #[must_use]
    pub const fn new(cancellation_id: CancellationId, generation: WorkerInstanceId) -> Self {
        Self {
            cancellation_id,
            generation,
        }
    }

    #[must_use]
    pub const fn cancellation_id(self) -> CancellationId {
        self.cancellation_id
    }

    #[must_use]
    pub const fn generation(self) -> WorkerInstanceId {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationState {
    Active,
    Cancelled { at: UnixTimestampMicros },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CancellationRecord {
    generation: WorkerInstanceId,
    state: CancellationState,
}

#[derive(Debug)]
pub struct CancellationRegistry {
    records: BTreeMap<CancellationId, CancellationRecord>,
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

    pub fn register(
        &mut self,
        registration: CancellationRegistration,
    ) -> Result<(), CancellationError> {
        if let Some(existing) = self.records.get(&registration.cancellation_id) {
            if existing.generation == registration.generation {
                return Ok(());
            }
            return Err(CancellationError::GenerationConflict(
                registration.cancellation_id,
            ));
        }
        if self.records.len() >= self.max_records.min(MAX_CANCELLATION_RECORDS) {
            return Err(CancellationError::RegistryFull);
        }
        self.records.insert(
            registration.cancellation_id,
            CancellationRecord {
                generation: registration.generation,
                state: CancellationState::Active,
            },
        );
        Ok(())
    }

    pub fn cancel(
        &mut self,
        registration: CancellationRegistration,
        at: UnixTimestampMicros,
    ) -> Result<bool, CancellationError> {
        let record = self.record_mut(registration)?;
        if matches!(record.state, CancellationState::Cancelled { .. }) {
            return Ok(false);
        }
        record.state = CancellationState::Cancelled { at };
        Ok(true)
    }

    pub fn retire(
        &mut self,
        registration: CancellationRegistration,
    ) -> Result<CancellationState, CancellationError> {
        let state = self.state(registration)?;
        if state == CancellationState::Active {
            return Err(CancellationError::RetireActive(
                registration.cancellation_id,
            ));
        }
        let removed = self
            .records
            .remove(&registration.cancellation_id)
            .ok_or(CancellationError::UnknownId(registration.cancellation_id))?;
        Ok(removed.state)
    }

    pub fn state(
        &self,
        registration: CancellationRegistration,
    ) -> Result<CancellationState, CancellationError> {
        let record = self.record(registration)?;
        Ok(record.state)
    }

    #[must_use]
    pub fn is_active(&self, registration: CancellationRegistration) -> bool {
        matches!(self.state(registration), Ok(CancellationState::Active))
    }

    #[must_use]
    pub fn is_cancelled(&self, registration: CancellationRegistration) -> bool {
        matches!(
            self.state(registration),
            Ok(CancellationState::Cancelled { .. })
        )
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    fn record(
        &self,
        registration: CancellationRegistration,
    ) -> Result<&CancellationRecord, CancellationError> {
        let Some(record) = self.records.get(&registration.cancellation_id) else {
            return Err(CancellationError::UnknownId(registration.cancellation_id));
        };
        if record.generation != registration.generation {
            return Err(CancellationError::StaleGeneration(
                registration.cancellation_id,
            ));
        }
        Ok(record)
    }

    fn record_mut(
        &mut self,
        registration: CancellationRegistration,
    ) -> Result<&mut CancellationRecord, CancellationError> {
        let Some(record) = self.records.get_mut(&registration.cancellation_id) else {
            return Err(CancellationError::UnknownId(registration.cancellation_id));
        };
        if record.generation != registration.generation {
            return Err(CancellationError::StaleGeneration(
                registration.cancellation_id,
            ));
        }
        Ok(record)
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
    GenerationConflict(CancellationId),
    StaleGeneration(CancellationId),
    RetireActive(CancellationId),
}

impl fmt::Display for CancellationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegistryFull => formatter.write_str("cancellation registry is full"),
            Self::UnknownId(id) => write!(formatter, "unknown cancellation id {id}"),
            Self::GenerationConflict(id) => {
                write!(
                    formatter,
                    "cancellation id {id} is owned by another generation"
                )
            }
            Self::StaleGeneration(id) => {
                write!(formatter, "stale generation for cancellation id {id}")
            }
            Self::RetireActive(id) => {
                write!(formatter, "active cancellation id {id} cannot be retired")
            }
        }
    }
}

impl Error for CancellationError {}

#[cfg(test)]
mod tests {
    use super::{
        CancellationError, CancellationRegistration, CancellationRegistry, CancellationState,
        DeadlineStatus, deadline_status,
    };
    use intent_contracts::{CancellationId, UnixTimestampMicros, WorkerInstanceId};
    use std::error::Error;
    use std::str::FromStr;

    fn cancellation_id() -> Result<CancellationId, Box<dyn Error>> {
        Ok(CancellationId::from_str(
            "018f47f7-5a86-7c00-8000-000000000701",
        )?)
    }

    fn generation(value: &str) -> Result<WorkerInstanceId, Box<dyn Error>> {
        Ok(WorkerInstanceId::from_str(value)?)
    }

    fn registration() -> Result<CancellationRegistration, Box<dyn Error>> {
        Ok(CancellationRegistration::new(
            cancellation_id()?,
            generation("018f47f7-5a86-7c00-8000-000000000801")?,
        ))
    }

    #[test]
    fn cancellation_is_idempotent_after_first_transition() -> Result<(), Box<dyn Error>> {
        let registration = registration()?;
        let now = UnixTimestampMicros::try_new(10)?;
        let mut registry = CancellationRegistry::new(4);
        registry.register(registration)?;

        assert!(registry.cancel(registration, now)?);
        assert!(!registry.cancel(registration, now)?);
        assert_eq!(
            registry.state(registration)?,
            CancellationState::Cancelled { at: now }
        );
        Ok(())
    }

    #[test]
    fn bounded_registry_reuses_capacity_only_after_cancelled_retirement()
    -> Result<(), Box<dyn Error>> {
        let first = registration()?;
        let second = CancellationRegistration::new(
            CancellationId::from_str("018f47f7-5a86-7c00-8000-000000000702")?,
            first.generation(),
        );
        let mut registry = CancellationRegistry::new(1);
        registry.register(first)?;
        assert_eq!(
            registry.register(second),
            Err(CancellationError::RegistryFull)
        );
        assert_eq!(
            registry.retire(first),
            Err(CancellationError::RetireActive(first.cancellation_id()))
        );
        let now = UnixTimestampMicros::try_new(10)?;
        registry.cancel(first, now)?;
        assert_eq!(
            registry.retire(first)?,
            CancellationState::Cancelled { at: now }
        );
        registry.register(second)?;
        assert!(registry.is_active(second));
        Ok(())
    }

    #[test]
    fn old_generation_cannot_cancel_reused_identity() -> Result<(), Box<dyn Error>> {
        let first = registration()?;
        let replacement = CancellationRegistration::new(
            first.cancellation_id(),
            generation("018f47f7-5a86-7c00-8000-000000000802")?,
        );
        let now = UnixTimestampMicros::try_new(10)?;
        let mut registry = CancellationRegistry::new(1);
        registry.register(first)?;
        registry.cancel(first, now)?;
        registry.retire(first)?;
        registry.register(replacement)?;
        assert_eq!(
            registry.cancel(first, now),
            Err(CancellationError::StaleGeneration(first.cancellation_id()))
        );
        assert!(registry.is_active(replacement));
        assert!(!registry.is_cancelled(replacement));
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
