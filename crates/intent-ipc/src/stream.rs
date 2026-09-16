use crate::{
    CancellationError, CancellationRegistry, DeliveryClass, EnqueueError, PriorityQueue,
    QueueLimits,
};
use intent_contracts::{ArtifactReference, CancellationId, UnixTimestampMicros};
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamEvent<T> {
    Progress(T),
    Reliable(T),
    Artifact(ArtifactReference),
    Cancel { cancellation_id: CancellationId },
    CancelAck { cancellation_id: CancellationId },
}

#[derive(Debug)]
pub struct StreamEndpoint<T> {
    queue: PriorityQueue<StreamEvent<T>>,
    cancellations: CancellationRegistry,
}

impl<T> StreamEndpoint<T> {
    #[must_use]
    pub const fn new(queue_limits: QueueLimits, max_cancellations: usize) -> Self {
        Self {
            queue: PriorityQueue::new(queue_limits),
            cancellations: CancellationRegistry::new(max_cancellations),
        }
    }

    pub fn register_cancellation(
        &mut self,
        cancellation_id: CancellationId,
    ) -> Result<(), StreamError<T>> {
        self.cancellations
            .register(cancellation_id)
            .map_err(StreamError::Cancellation)
    }

    pub fn grant_credits(&mut self, credits: usize) -> Result<(), StreamError<T>> {
        self.queue
            .grant_credits(credits)
            .map_err(StreamError::Credit)
    }

    pub fn enqueue_progress(&mut self, payload: T) -> Result<(), StreamError<T>> {
        self.queue
            .enqueue(
                DeliveryClass::BestEffortProgress,
                StreamEvent::Progress(payload),
            )
            .map_err(StreamError::Enqueue)
    }

    pub fn enqueue_reliable(&mut self, payload: T) -> Result<(), StreamError<T>> {
        self.queue
            .enqueue(
                DeliveryClass::ReliableControl,
                StreamEvent::Reliable(payload),
            )
            .map_err(StreamError::Enqueue)
    }

    pub fn enqueue_artifact(&mut self, artifact: ArtifactReference) -> Result<(), StreamError<T>> {
        self.queue
            .enqueue(
                DeliveryClass::ArtifactReference,
                StreamEvent::Artifact(artifact),
            )
            .map_err(StreamError::Enqueue)
    }

    pub fn enqueue_cancel(
        &mut self,
        cancellation_id: CancellationId,
    ) -> Result<(), StreamError<T>> {
        self.queue
            .enqueue(
                DeliveryClass::ReservedControl,
                StreamEvent::Cancel { cancellation_id },
            )
            .map_err(StreamError::Enqueue)
    }

    pub fn accept_cancel(
        &mut self,
        cancellation_id: CancellationId,
        at: UnixTimestampMicros,
    ) -> Result<(), StreamError<T>> {
        self.cancellations
            .cancel(cancellation_id, at)
            .map_err(StreamError::Cancellation)?;
        self.queue
            .enqueue(
                DeliveryClass::ReservedControl,
                StreamEvent::CancelAck { cancellation_id },
            )
            .map_err(StreamError::Enqueue)
    }

    #[must_use]
    pub fn is_cancelled(&self, cancellation_id: CancellationId) -> bool {
        self.cancellations.is_cancelled(cancellation_id)
    }

    #[must_use]
    pub fn pop_next(&mut self) -> Option<(DeliveryClass, StreamEvent<T>)> {
        self.queue.pop_next()
    }

    #[must_use]
    pub fn progress_backlog(&self) -> usize {
        self.queue.best_effort_len()
    }
}

#[derive(Debug)]
pub enum StreamError<T> {
    Cancellation(CancellationError),
    Credit(crate::CreditError),
    Enqueue(EnqueueError<StreamEvent<T>>),
}

impl<T> fmt::Display for StreamError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancellation(error) => write!(formatter, "cancellation error: {error}"),
            Self::Credit(error) => write!(formatter, "credit error: {error}"),
            Self::Enqueue(error) => write!(formatter, "enqueue error: {error}"),
        }
    }
}

impl<T: fmt::Debug> Error for StreamError<T> {}

#[cfg(test)]
mod tests {
    use super::{StreamEndpoint, StreamEvent};
    use crate::{DeliveryClass, QueueLimits};
    use intent_contracts::{CancellationId, UnixTimestampMicros};
    use std::error::Error;
    use std::str::FromStr;

    fn cancellation_id() -> Result<CancellationId, Box<dyn Error>> {
        Ok(CancellationId::from_str(
            "018f47f7-5a86-7c00-8000-000000000711",
        )?)
    }

    #[test]
    fn two_workers_ack_cancel_while_progress_queues_are_saturated() -> Result<(), Box<dyn Error>> {
        let limits = QueueLimits::try_new(4, 1, 2, 4)?;
        let cancellation_id = cancellation_id()?;
        let now = UnixTimestampMicros::try_new(123)?;
        let mut first = StreamEndpoint::new(limits, 4);
        let mut second = StreamEndpoint::new(limits, 4);

        first.register_cancellation(cancellation_id)?;
        second.register_cancellation(cancellation_id)?;
        first.grant_credits(2)?;
        second.grant_credits(2)?;
        first.enqueue_progress(1_u8)?;
        first.enqueue_progress(2_u8)?;
        second.enqueue_progress(3_u8)?;
        second.enqueue_progress(4_u8)?;

        first.enqueue_cancel(cancellation_id)?;
        second.enqueue_cancel(cancellation_id)?;

        assert_eq!(
            first.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::Cancel { cancellation_id }
            ))
        );
        assert_eq!(
            second.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::Cancel { cancellation_id }
            ))
        );

        first.accept_cancel(cancellation_id, now)?;
        second.accept_cancel(cancellation_id, now)?;
        assert!(first.is_cancelled(cancellation_id));
        assert!(second.is_cancelled(cancellation_id));

        assert_eq!(
            first.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::CancelAck { cancellation_id }
            ))
        );
        assert_eq!(
            second.pop_next(),
            Some((
                DeliveryClass::ReservedControl,
                StreamEvent::CancelAck { cancellation_id }
            ))
        );
        assert_eq!(first.progress_backlog(), 2);
        assert_eq!(second.progress_backlog(), 2);
        Ok(())
    }
}
