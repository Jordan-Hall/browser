use crate::{
    CancellationError, CancellationRegistration, CancellationRegistry, CancellationState,
    DeliveryClass, EnqueueError, PriorityQueue, QueueLimits,
};
use intent_contracts::{ArtifactReference, UnixTimestampMicros};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamEvent<T> {
    Progress {
        registration: CancellationRegistration,
        payload: T,
    },
    Reliable {
        registration: CancellationRegistration,
        payload: T,
    },
    Artifact {
        registration: CancellationRegistration,
        artifact: ArtifactReference,
    },
    Cancel {
        registration: CancellationRegistration,
    },
    CancelAck {
        registration: CancellationRegistration,
    },
}

impl<T> StreamEvent<T> {
    const fn registration(&self) -> CancellationRegistration {
        match self {
            Self::Progress { registration, .. }
            | Self::Reliable { registration, .. }
            | Self::Artifact { registration, .. }
            | Self::Cancel { registration }
            | Self::CancelAck { registration } => *registration,
        }
    }

    const fn is_cancellation_control(&self) -> bool {
        matches!(self, Self::Cancel { .. } | Self::CancelAck { .. })
    }
}

#[derive(Debug)]
pub struct StreamEndpoint<T> {
    queue: PriorityQueue<StreamEvent<T>>,
    cancellations: CancellationRegistry,
    pending_cancel_acks: BTreeSet<CancellationRegistration>,
}

impl<T> StreamEndpoint<T> {
    #[must_use]
    pub const fn new(queue_limits: QueueLimits, max_cancellations: usize) -> Self {
        Self {
            queue: PriorityQueue::new(queue_limits),
            cancellations: CancellationRegistry::new(max_cancellations),
            pending_cancel_acks: BTreeSet::new(),
        }
    }

    pub fn register_cancellation(
        &mut self,
        registration: CancellationRegistration,
    ) -> Result<(), StreamError<T>> {
        self.cancellations
            .register(registration)
            .map_err(StreamError::Cancellation)
    }

    pub fn grant_credits(&mut self, credits: usize) -> Result<(), StreamError<T>> {
        self.queue
            .grant_credits(credits)
            .map_err(StreamError::Credit)
    }

    pub fn enqueue_progress(
        &mut self,
        registration: CancellationRegistration,
        payload: T,
    ) -> Result<(), StreamError<T>> {
        self.require_active(registration)?;
        self.queue
            .enqueue(
                DeliveryClass::BestEffortProgress,
                StreamEvent::Progress {
                    registration,
                    payload,
                },
            )
            .map_err(StreamError::Enqueue)
    }

    pub fn enqueue_reliable(
        &mut self,
        registration: CancellationRegistration,
        payload: T,
    ) -> Result<(), StreamError<T>> {
        self.require_active(registration)?;
        self.queue
            .enqueue(
                DeliveryClass::ReliableControl,
                StreamEvent::Reliable {
                    registration,
                    payload,
                },
            )
            .map_err(StreamError::Enqueue)
    }

    pub fn enqueue_artifact(
        &mut self,
        registration: CancellationRegistration,
        artifact: ArtifactReference,
    ) -> Result<(), StreamError<T>> {
        self.require_active(registration)?;
        self.queue
            .enqueue(
                DeliveryClass::ArtifactReference,
                StreamEvent::Artifact {
                    registration,
                    artifact,
                },
            )
            .map_err(StreamError::Enqueue)
    }

    pub fn enqueue_cancel(
        &mut self,
        registration: CancellationRegistration,
    ) -> Result<(), StreamError<T>> {
        self.cancellations
            .state(registration)
            .map_err(StreamError::Cancellation)?;
        if self.queue.has_reserved(|event| {
            matches!(
                event,
                StreamEvent::Cancel {
                    registration: queued
                } if *queued == registration
            )
        }) {
            return Ok(());
        }
        self.queue
            .enqueue(
                DeliveryClass::ReservedControl,
                StreamEvent::Cancel { registration },
            )
            .map_err(StreamError::Enqueue)
    }

    /// Apply cancellation before attempting to queue its acknowledgement.
    /// `CancellationAppliedAckPending` leaves cancellation in force and records
    /// the missing acknowledgement durably in endpoint memory; retirement is
    /// denied until a retry queues and drains that acknowledgement.
    pub fn accept_cancel(
        &mut self,
        registration: CancellationRegistration,
        at: UnixTimestampMicros,
    ) -> Result<(), StreamError<T>> {
        self.cancellations
            .cancel(registration, at)
            .map_err(StreamError::Cancellation)?;
        if self.queue.has_reserved(|event| {
            matches!(
                event,
                StreamEvent::CancelAck {
                    registration: queued
                } if *queued == registration
            )
        }) {
            self.pending_cancel_acks.remove(&registration);
            return Ok(());
        }
        match self.queue.enqueue(
            DeliveryClass::ReservedControl,
            StreamEvent::CancelAck { registration },
        ) {
            Ok(()) => {
                self.pending_cancel_acks.remove(&registration);
                Ok(())
            }
            Err(error) => {
                self.pending_cancel_acks.insert(registration);
                Err(StreamError::CancellationAppliedAckPending(error))
            }
        }
    }

    /// Retire a completed cancellation only after its acknowledgement has been
    /// queued and drained and every request-scoped event for that exact worker
    /// generation has been removed. Capacity can then be reused without making
    /// a late old-generation registration current again.
    pub fn retire_cancellation(
        &mut self,
        registration: CancellationRegistration,
    ) -> Result<CancellationState, StreamError<T>> {
        if self.pending_cancel_acks.contains(&registration) {
            return Err(StreamError::CancellationAckPending(registration));
        }
        if self
            .queue
            .has_any(|event| event.registration() == registration)
        {
            return Err(StreamError::CancellationEventsPending(registration));
        }
        self.cancellations
            .retire(registration)
            .map_err(StreamError::Cancellation)
    }

    /// Positive activity, not absence from a cancelled set, is the dispatch-safe
    /// predicate for a registered generation.
    #[must_use]
    pub fn is_active(&self, registration: CancellationRegistration) -> bool {
        self.cancellations.is_active(registration)
    }

    #[must_use]
    pub fn is_cancelled(&self, registration: CancellationRegistration) -> bool {
        self.cancellations.is_cancelled(registration)
    }

    /// Cancelled/stale request payloads are discarded rather than emitted.
    /// Reserved cancellation control always remains observable so a peer can
    /// complete the cancellation handshake before retirement.
    #[must_use]
    pub fn pop_next(&mut self) -> Option<(DeliveryClass, StreamEvent<T>)> {
        while let Some((class, event)) = self.queue.pop_next() {
            if event.is_cancellation_control() || self.cancellations.is_active(event.registration())
            {
                return Some((class, event));
            }
        }
        None
    }

    #[must_use]
    pub fn progress_backlog(&self) -> usize {
        self.queue.progress_len()
    }

    fn require_active(&self, registration: CancellationRegistration) -> Result<(), StreamError<T>> {
        match self
            .cancellations
            .state(registration)
            .map_err(StreamError::Cancellation)?
        {
            CancellationState::Active => Ok(()),
            CancellationState::Cancelled { .. } => {
                Err(StreamError::CancellationInactive(registration))
            }
        }
    }
}

#[derive(Debug)]
pub enum StreamError<T> {
    Cancellation(CancellationError),
    Credit(crate::CreditError),
    Enqueue(EnqueueError<StreamEvent<T>>),
    CancellationAppliedAckPending(EnqueueError<StreamEvent<T>>),
    CancellationAckPending(CancellationRegistration),
    CancellationEventsPending(CancellationRegistration),
    CancellationInactive(CancellationRegistration),
}

impl<T> fmt::Display for StreamError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancellation(error) => write!(formatter, "cancellation error: {error}"),
            Self::Credit(error) => write!(formatter, "credit error: {error}"),
            Self::Enqueue(error) => write!(formatter, "enqueue error: {error}"),
            Self::CancellationAppliedAckPending(error) => {
                write!(
                    formatter,
                    "cancellation applied; acknowledgement pending: {error}"
                )
            }
            Self::CancellationAckPending(registration) => write!(
                formatter,
                "cancellation acknowledgement is pending for {}",
                registration.cancellation_id()
            ),
            Self::CancellationEventsPending(registration) => write!(
                formatter,
                "cancellation-scoped events are still queued for {}",
                registration.cancellation_id()
            ),
            Self::CancellationInactive(registration) => write!(
                formatter,
                "cancellation registration {} is not active",
                registration.cancellation_id()
            ),
        }
    }
}

impl<T: fmt::Debug> Error for StreamError<T> {}

#[cfg(test)]
mod tests {
    use super::{StreamEndpoint, StreamError, StreamEvent};
    use crate::{
        CancellationError, CancellationRegistration, DeliveryClass, EnqueueErrorKind, QueueLimits,
    };
    use intent_contracts::{
        ArtifactId, ArtifactReference, BoundedText, ByteSize, CancellationId, ContentHash,
        UnixTimestampMicros, WorkerInstanceId,
    };
    use std::error::Error;
    use std::str::FromStr;

    fn cancellation_id() -> Result<CancellationId, Box<dyn Error>> {
        Ok(CancellationId::from_str(
            "018f47f7-5a86-7c00-8000-000000000711",
        )?)
    }

    fn generation(value: u128) -> Result<WorkerInstanceId, Box<dyn Error>> {
        Ok(WorkerInstanceId::from_str(&format!(
            "00000000-0000-0000-0000-{value:012x}"
        ))?)
    }

    fn registration(
        id: CancellationId,
        generation_value: u128,
    ) -> Result<CancellationRegistration, Box<dyn Error>> {
        Ok(CancellationRegistration::new(
            id,
            generation(generation_value)?,
        ))
    }

    fn artifact() -> Result<ArtifactReference, Box<dyn Error>> {
        Ok(ArtifactReference::new(
            ArtifactId::from_str("018f47f7-5a86-7c00-8000-000000000713")?,
            ContentHash::from_bytes([7; 32]),
            ByteSize::from_bytes(4),
            BoundedText::try_new("text/plain")?,
        ))
    }

    #[test]
    fn duplicate_cancels_preserve_capacity_for_another_request() -> Result<(), Box<dyn Error>> {
        let mut endpoint = StreamEndpoint::<u8>::new(QueueLimits::try_new(2, 1, 1, 1)?, 2);
        let first = registration(cancellation_id()?, 0x101)?;
        let second = registration(
            CancellationId::from_str("018f47f7-5a86-7c00-8000-000000000712")?,
            0x101,
        )?;
        endpoint.register_cancellation(first)?;
        endpoint.register_cancellation(second)?;
        for _ in 0..100 {
            endpoint.enqueue_cancel(first)?;
        }
        endpoint.enqueue_cancel(second)?;
        for registration in [first, second] {
            assert_eq!(
                endpoint.pop_next(),
                Some((
                    DeliveryClass::ReservedControl,
                    StreamEvent::Cancel { registration }
                ))
            );
        }
        assert_eq!(endpoint.pop_next(), None);
        endpoint.enqueue_cancel(first)?;
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::Cancel {
                    registration: first
                }
            ))
        );
        Ok(())
    }

    #[test]
    fn duplicate_acknowledgements_preserve_capacity_for_another_request()
    -> Result<(), Box<dyn Error>> {
        let mut endpoint = StreamEndpoint::<u8>::new(QueueLimits::try_new(2, 1, 1, 1)?, 2);
        let first = registration(cancellation_id()?, 0x101)?;
        let second = registration(
            CancellationId::from_str("018f47f7-5a86-7c00-8000-000000000712")?,
            0x101,
        )?;
        let now = UnixTimestampMicros::try_new(123)?;
        endpoint.register_cancellation(first)?;
        endpoint.register_cancellation(second)?;
        for _ in 0..100 {
            endpoint.accept_cancel(first, now)?;
        }
        endpoint.accept_cancel(second, now)?;
        for registration in [first, second] {
            assert!(endpoint.is_cancelled(registration));
            assert_eq!(
                endpoint.pop_next(),
                Some((
                    DeliveryClass::ReservedControl,
                    StreamEvent::CancelAck { registration }
                ))
            );
        }
        assert_eq!(endpoint.pop_next(), None);
        endpoint.accept_cancel(first, now)?;
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::CancelAck {
                    registration: first
                }
            ))
        );
        Ok(())
    }

    #[test]
    fn acknowledgement_backpressure_reports_applied_cancellation() -> Result<(), Box<dyn Error>> {
        let mut endpoint = StreamEndpoint::<u8>::new(QueueLimits::try_new(1, 1, 1, 1)?, 1);
        let registration = registration(cancellation_id()?, 0x101)?;
        let now = UnixTimestampMicros::try_new(123)?;
        endpoint.register_cancellation(registration)?;
        endpoint.enqueue_cancel(registration)?;
        let Err(error) = endpoint.accept_cancel(registration, now) else {
            return Err("full reserved queue unexpectedly accepted acknowledgement".into());
        };
        assert!(endpoint.is_cancelled(registration));
        let StreamError::CancellationAppliedAckPending(pending) = error else {
            return Err("applied cancellation did not report pending acknowledgement".into());
        };
        assert_eq!(pending.kind(), EnqueueErrorKind::ReservedQueueFull);
        assert_eq!(pending.into_item(), StreamEvent::CancelAck { registration });
        assert!(matches!(
            endpoint.retire_cancellation(registration),
            Err(StreamError::CancellationAckPending(pending)) if pending == registration
        ));
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::Cancel { registration }
            ))
        );
        endpoint.accept_cancel(registration, now)?;
        endpoint.accept_cancel(registration, now)?;
        assert!(endpoint.is_cancelled(registration));
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::CancelAck { registration }
            ))
        );
        assert_eq!(endpoint.pop_next(), None);
        endpoint.retire_cancellation(registration)?;
        Ok(())
    }

    #[test]
    fn backpressured_ack_for_other_registration_blocks_retirement() -> Result<(), Box<dyn Error>> {
        let mut endpoint = StreamEndpoint::<u8>::new(QueueLimits::try_new(1, 1, 1, 1)?, 2);
        let blocker = registration(cancellation_id()?, 0x101)?;
        let target = registration(
            CancellationId::from_str("018f47f7-5a86-7c00-8000-000000000714")?,
            0x101,
        )?;
        let now = UnixTimestampMicros::try_new(123)?;
        endpoint.register_cancellation(blocker)?;
        endpoint.register_cancellation(target)?;
        endpoint.enqueue_cancel(blocker)?;
        assert!(matches!(
            endpoint.accept_cancel(target, now),
            Err(StreamError::CancellationAppliedAckPending(_))
        ));
        assert!(endpoint.is_cancelled(target));
        assert!(matches!(
            endpoint.retire_cancellation(target),
            Err(StreamError::CancellationAckPending(pending)) if pending == target
        ));
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::Cancel {
                    registration: blocker
                }
            ))
        );
        endpoint.accept_cancel(target, now)?;
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::CancelAck {
                    registration: target
                }
            ))
        );
        endpoint.retire_cancellation(target)?;
        Ok(())
    }

    #[test]
    fn unknown_cancellation_is_rejected_without_acknowledgement() -> Result<(), Box<dyn Error>> {
        let mut endpoint = StreamEndpoint::<u8>::new(QueueLimits::try_new(1, 1, 1, 1)?, 1);
        let registration = registration(cancellation_id()?, 0x101)?;
        assert!(matches!(
            endpoint.accept_cancel(registration, UnixTimestampMicros::try_new(123)?),
            Err(StreamError::Cancellation(CancellationError::UnknownId(id)))
                if id == registration.cancellation_id()
        ));
        assert!(!endpoint.is_active(registration));
        assert_eq!(endpoint.pop_next(), None);
        Ok(())
    }

    #[test]
    fn progress_backlog_excludes_artifact_references() -> Result<(), Box<dyn Error>> {
        let mut endpoint = StreamEndpoint::<u8>::new(QueueLimits::try_new(1, 1, 3, 3)?, 1);
        let registration = registration(cancellation_id()?, 0x101)?;
        endpoint.register_cancellation(registration)?;
        let artifact = artifact()?;
        endpoint.grant_credits(3)?;
        endpoint.enqueue_progress(registration, 1)?;
        endpoint.enqueue_artifact(registration, artifact.clone())?;
        endpoint.enqueue_progress(registration, 2)?;
        assert_eq!(endpoint.progress_backlog(), 2);
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::BestEffortProgress,
                StreamEvent::Progress {
                    registration,
                    payload: 1
                }
            ))
        );
        assert_eq!(endpoint.progress_backlog(), 1);
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::ArtifactReference,
                StreamEvent::Artifact {
                    registration,
                    artifact
                }
            ))
        );
        assert_eq!(endpoint.progress_backlog(), 1);
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::BestEffortProgress,
                StreamEvent::Progress {
                    registration,
                    payload: 2
                }
            ))
        );
        assert_eq!(endpoint.progress_backlog(), 0);
        Ok(())
    }

    #[test]
    fn two_workers_ack_cancel_while_progress_queues_are_saturated() -> Result<(), Box<dyn Error>> {
        let limits = QueueLimits::try_new(4, 1, 2, 4)?;
        let registration = registration(cancellation_id()?, 0x101)?;
        let now = UnixTimestampMicros::try_new(123)?;
        let mut first = StreamEndpoint::new(limits, 4);
        let mut second = StreamEndpoint::new(limits, 4);

        first.register_cancellation(registration)?;
        second.register_cancellation(registration)?;
        first.grant_credits(2)?;
        second.grant_credits(2)?;
        first.enqueue_progress(registration, 1_u8)?;
        first.enqueue_progress(registration, 2_u8)?;
        second.enqueue_progress(registration, 3_u8)?;
        second.enqueue_progress(registration, 4_u8)?;

        first.enqueue_cancel(registration)?;
        second.enqueue_cancel(registration)?;

        assert_eq!(
            first.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::Cancel { registration }
            ))
        );
        assert_eq!(
            second.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::Cancel { registration }
            ))
        );

        first.accept_cancel(registration, now)?;
        second.accept_cancel(registration, now)?;
        assert!(first.is_cancelled(registration));
        assert!(second.is_cancelled(registration));

        assert_eq!(
            first.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::CancelAck { registration }
            ))
        );
        assert_eq!(
            second.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::CancelAck { registration }
            ))
        );
        assert_eq!(first.progress_backlog(), 2);
        assert_eq!(second.progress_backlog(), 2);
        assert_eq!(first.pop_next(), None);
        assert_eq!(second.pop_next(), None);
        assert_eq!(first.progress_backlog(), 0);
        assert_eq!(second.progress_backlog(), 0);
        Ok(())
    }

    #[test]
    fn cancellation_drops_queued_request_work_before_retirement() -> Result<(), Box<dyn Error>> {
        let mut endpoint = StreamEndpoint::<u8>::new(QueueLimits::try_new(2, 2, 3, 3)?, 1);
        let registration = registration(cancellation_id()?, 0x101)?;
        let now = UnixTimestampMicros::try_new(123)?;
        endpoint.register_cancellation(registration)?;
        endpoint.grant_credits(2)?;
        endpoint.enqueue_progress(registration, 1)?;
        endpoint.enqueue_reliable(registration, 2)?;
        endpoint.enqueue_artifact(registration, artifact()?)?;
        endpoint.accept_cancel(registration, now)?;
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::CancelAck { registration }
            ))
        );
        assert!(matches!(
            endpoint.retire_cancellation(registration),
            Err(StreamError::CancellationEventsPending(pending)) if pending == registration
        ));
        assert_eq!(endpoint.pop_next(), None);
        assert_eq!(endpoint.progress_backlog(), 0);
        endpoint.retire_cancellation(registration)?;
        assert!(matches!(
            endpoint.enqueue_reliable(registration, 9),
            Err(StreamError::Cancellation(CancellationError::UnknownId(id)))
                if id == registration.cancellation_id()
        ));
        Ok(())
    }

    #[test]
    fn retirement_fences_late_generations_and_reuses_registry_capacity()
    -> Result<(), Box<dyn Error>> {
        let limits = QueueLimits::try_new(1, 1, 1, 1)?;
        let mut endpoint = StreamEndpoint::<u8>::new(limits, 1);
        let id = cancellation_id()?;
        let old = registration(id, 0x101)?;
        let now = UnixTimestampMicros::try_new(123)?;
        endpoint.register_cancellation(old)?;
        endpoint.enqueue_cancel(old)?;
        assert!(matches!(
            endpoint.accept_cancel(old, now),
            Err(StreamError::CancellationAppliedAckPending(_))
        ));
        assert!(endpoint.is_cancelled(old));
        assert!(matches!(
            endpoint.retire_cancellation(old),
            Err(StreamError::CancellationAckPending(pending)) if pending == old
        ));
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::Cancel { registration: old }
            ))
        );
        endpoint.accept_cancel(old, now)?;
        assert!(matches!(
            endpoint.retire_cancellation(old),
            Err(StreamError::CancellationEventsPending(pending)) if pending == old
        ));
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::CancelAck { registration: old }
            ))
        );
        endpoint.retire_cancellation(old)?;
        assert!(!endpoint.is_active(old));
        assert!(!endpoint.is_cancelled(old));

        let replacement = registration(id, 0x102)?;
        endpoint.register_cancellation(replacement)?;
        assert!(endpoint.is_active(replacement));
        assert!(matches!(
            endpoint.accept_cancel(old, now),
            Err(StreamError::Cancellation(CancellationError::StaleGeneration(stale))) if stale == id
        ));
        assert!(endpoint.is_active(replacement));
        endpoint.accept_cancel(replacement, now)?;
        assert_eq!(
            endpoint.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::CancelAck {
                    registration: replacement
                }
            ))
        );
        endpoint.retire_cancellation(replacement)?;

        for sequence in 0_u128..8 {
            let id = CancellationId::from_str(&format!(
                "00000000-0000-0000-0000-{:012x}",
                0x1000 + sequence
            ))?;
            let registration = registration(id, 0x102)?;
            endpoint.register_cancellation(registration)?;
            assert!(endpoint.is_active(registration));
            endpoint.accept_cancel(registration, now)?;
            assert_eq!(
                endpoint.pop_next(),
                Some((
                    DeliveryClass::ReservedControl,
                    StreamEvent::CancelAck { registration }
                ))
            );
            endpoint.retire_cancellation(registration)?;
        }
        Ok(())
    }
}
