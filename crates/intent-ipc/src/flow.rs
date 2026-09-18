use intent_contracts::ArtifactReference;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

pub const MAX_QUEUE_CAPACITY: usize = 65_536;
pub const MAX_FLOW_CREDITS: usize = 65_536;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeliveryClass {
    ReservedControl,
    ReliableControl,
    BestEffortProgress,
    ArtifactReference,
}

/// Validated queue limits cannot be constructed or mutated externally.
///
/// ```compile_fail
/// use intent_ipc::QueueLimits;
/// let _ = QueueLimits { reserved_control: 0, reliable_control: 0, best_effort: 0, max_credits: 0 };
/// ```
///
/// ```compile_fail
/// use intent_ipc::QueueLimits;
/// fn corrupt(mut limits: QueueLimits) { limits.reserved_control = 0; }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueLimits {
    reserved_control: usize,
    reliable_control: usize,
    best_effort: usize,
    max_credits: usize,
}

impl QueueLimits {
    pub const fn try_new(
        reserved_control: usize,
        reliable_control: usize,
        best_effort: usize,
        max_credits: usize,
    ) -> Result<Self, QueueConfigError> {
        if reserved_control == 0 {
            return Err(QueueConfigError::ReservedCapacityRequired);
        }
        if reserved_control > MAX_QUEUE_CAPACITY
            || reliable_control > MAX_QUEUE_CAPACITY
            || best_effort > MAX_QUEUE_CAPACITY
        {
            return Err(QueueConfigError::CapacityTooLarge);
        }
        if max_credits > MAX_FLOW_CREDITS {
            return Err(QueueConfigError::CreditLimitTooLarge);
        }
        Ok(Self {
            reserved_control,
            reliable_control,
            best_effort,
            max_credits,
        })
    }

    #[must_use]
    pub const fn conservative_default() -> Self {
        Self {
            reserved_control: 32,
            reliable_control: 256,
            best_effort: 1_024,
            max_credits: 1_024,
        }
    }
}

#[derive(Debug)]
pub struct PriorityQueue<T> {
    limits: QueueLimits,
    reserved: VecDeque<T>,
    reliable: VecDeque<T>,
    best_effort: VecDeque<(DeliveryClass, T)>,
    credits: usize,
}

impl<T> PriorityQueue<T> {
    #[must_use]
    pub const fn new(limits: QueueLimits) -> Self {
        Self {
            limits,
            reserved: VecDeque::new(),
            reliable: VecDeque::new(),
            best_effort: VecDeque::new(),
            credits: 0,
        }
    }

    pub fn grant_credits(&mut self, credits: usize) -> Result<(), CreditError> {
        let new_total = self
            .credits
            .checked_add(credits)
            .ok_or(CreditError::Overflow)?;
        if new_total > self.limits.max_credits {
            return Err(CreditError::ExceedsLimit {
                requested_total: new_total,
                limit: self.limits.max_credits,
            });
        }
        self.credits = new_total;
        Ok(())
    }

    pub fn enqueue(&mut self, class: DeliveryClass, item: T) -> Result<(), EnqueueError<T>> {
        match class {
            DeliveryClass::ReservedControl => {
                if self.reserved.len() >= self.limits.reserved_control {
                    return Err(EnqueueError::new(EnqueueErrorKind::ReservedQueueFull, item));
                }
                self.reserved.push_back(item);
            }
            DeliveryClass::ReliableControl => {
                if self.reliable.len() >= self.limits.reliable_control {
                    return Err(EnqueueError::new(EnqueueErrorKind::ReliableQueueFull, item));
                }
                self.reliable.push_back(item);
            }
            DeliveryClass::BestEffortProgress | DeliveryClass::ArtifactReference => {
                if self.credits == 0 {
                    return Err(EnqueueError::new(EnqueueErrorKind::NoCredit, item));
                }
                if self.best_effort.len() >= self.limits.best_effort {
                    return Err(EnqueueError::new(
                        EnqueueErrorKind::BestEffortQueueFull,
                        item,
                    ));
                }
                self.credits -= 1;
                self.best_effort.push_back((class, item));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn pop_next(&mut self) -> Option<(DeliveryClass, T)> {
        if let Some(item) = self.reserved.pop_front() {
            return Some((DeliveryClass::ReservedControl, item));
        }
        if let Some(item) = self.reliable.pop_front() {
            return Some((DeliveryClass::ReliableControl, item));
        }
        self.best_effort.pop_front()
    }

    #[must_use]
    pub const fn credits(&self) -> usize {
        self.credits
    }

    #[must_use]
    pub fn reserved_len(&self) -> usize {
        self.reserved.len()
    }

    #[must_use]
    pub fn reliable_len(&self) -> usize {
        self.reliable.len()
    }

    #[must_use]
    pub fn best_effort_len(&self) -> usize {
        self.best_effort.len()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactDispatch {
    reference: ArtifactReference,
}

impl ArtifactDispatch {
    #[must_use]
    pub const fn new(reference: ArtifactReference) -> Self {
        Self { reference }
    }

    #[must_use]
    pub const fn reference(&self) -> &ArtifactReference {
        &self.reference
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueConfigError {
    ReservedCapacityRequired,
    CapacityTooLarge,
    CreditLimitTooLarge,
}

impl fmt::Display for QueueConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReservedCapacityRequired => {
                formatter.write_str("reserved control capacity must be non-zero")
            }
            Self::CapacityTooLarge => formatter.write_str("queue capacity exceeds hard limit"),
            Self::CreditLimitTooLarge => {
                formatter.write_str("flow credit limit exceeds hard limit")
            }
        }
    }
}

impl Error for QueueConfigError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CreditError {
    Overflow,
    ExceedsLimit {
        requested_total: usize,
        limit: usize,
    },
}

impl fmt::Display for CreditError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow => formatter.write_str("flow credit counter overflowed"),
            Self::ExceedsLimit {
                requested_total,
                limit,
            } => write!(
                formatter,
                "flow credit total {requested_total} exceeds configured limit {limit}"
            ),
        }
    }
}

impl Error for CreditError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnqueueErrorKind {
    ReservedQueueFull,
    ReliableQueueFull,
    BestEffortQueueFull,
    NoCredit,
}

#[derive(Debug)]
pub struct EnqueueError<T> {
    kind: EnqueueErrorKind,
    item: T,
}

impl<T> EnqueueError<T> {
    #[must_use]
    pub const fn new(kind: EnqueueErrorKind, item: T) -> Self {
        Self { kind, item }
    }

    #[must_use]
    pub const fn kind(&self) -> EnqueueErrorKind {
        self.kind
    }

    #[must_use]
    pub fn into_item(self) -> T {
        self.item
    }
}

impl<T> fmt::Display for EnqueueError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            EnqueueErrorKind::ReservedQueueFull => {
                formatter.write_str("reserved control queue is full")
            }
            EnqueueErrorKind::ReliableQueueFull => {
                formatter.write_str("reliable control queue is full")
            }
            EnqueueErrorKind::BestEffortQueueFull => {
                formatter.write_str("best-effort queue is full")
            }
            EnqueueErrorKind::NoCredit => {
                formatter.write_str("no flow-control credit is available")
            }
        }
    }
}

impl<T: fmt::Debug> Error for EnqueueError<T> {}

#[cfg(test)]
mod tests {
    use super::{DeliveryClass, EnqueueErrorKind, PriorityQueue, QueueLimits};
    use std::error::Error;

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum TestMessage {
        Progress(u8),
        State(u8),
        Cancel,
        Artifact,
    }

    fn limits() -> Result<QueueLimits, Box<dyn Error>> {
        Ok(QueueLimits::try_new(2, 2, 2, 2)?)
    }

    #[test]
    fn best_effort_requires_explicit_credit() -> Result<(), Box<dyn Error>> {
        let mut queue = PriorityQueue::new(limits()?);
        let Err(error) = queue.enqueue(DeliveryClass::BestEffortProgress, TestMessage::Progress(1))
        else {
            return Err("best-effort message unexpectedly bypassed credit".into());
        };
        assert_eq!(error.kind(), EnqueueErrorKind::NoCredit);
        assert_eq!(error.into_item(), TestMessage::Progress(1));
        Ok(())
    }

    #[test]
    fn reliable_control_is_not_displaced_by_progress() -> Result<(), Box<dyn Error>> {
        let mut queue = PriorityQueue::new(limits()?);
        queue.grant_credits(2)?;
        queue.enqueue(DeliveryClass::BestEffortProgress, TestMessage::Progress(1))?;
        queue.enqueue(DeliveryClass::BestEffortProgress, TestMessage::Progress(2))?;
        queue.enqueue(DeliveryClass::ReliableControl, TestMessage::State(9))?;

        assert_eq!(
            queue.pop_next(),
            Some((DeliveryClass::ReliableControl, TestMessage::State(9)))
        );
        Ok(())
    }

    #[test]
    fn reserved_control_preempts_every_other_class() -> Result<(), Box<dyn Error>> {
        let mut queue = PriorityQueue::new(limits()?);
        queue.grant_credits(1)?;
        queue.enqueue(DeliveryClass::ReliableControl, TestMessage::State(1))?;
        queue.enqueue(DeliveryClass::BestEffortProgress, TestMessage::Progress(1))?;
        queue.enqueue(DeliveryClass::ReservedControl, TestMessage::Cancel)?;

        assert_eq!(
            queue.pop_next(),
            Some((DeliveryClass::ReservedControl, TestMessage::Cancel))
        );
        Ok(())
    }

    #[test]
    fn full_reliable_queue_returns_item_instead_of_dropping_it() -> Result<(), Box<dyn Error>> {
        let mut queue = PriorityQueue::new(QueueLimits::try_new(1, 1, 1, 1)?);
        queue.enqueue(DeliveryClass::ReliableControl, TestMessage::State(1))?;
        let Err(error) = queue.enqueue(DeliveryClass::ReliableControl, TestMessage::State(2))
        else {
            return Err("reliable queue unexpectedly accepted beyond capacity".into());
        };
        assert_eq!(error.kind(), EnqueueErrorKind::ReliableQueueFull);
        assert_eq!(error.into_item(), TestMessage::State(2));
        Ok(())
    }

    #[test]
    fn artifact_reference_keeps_its_delivery_class() -> Result<(), Box<dyn Error>> {
        let mut queue = PriorityQueue::new(limits()?);
        queue.grant_credits(1)?;
        queue.enqueue(DeliveryClass::ArtifactReference, TestMessage::Artifact)?;
        assert_eq!(
            queue.pop_next(),
            Some((DeliveryClass::ArtifactReference, TestMessage::Artifact))
        );
        Ok(())
    }
}
