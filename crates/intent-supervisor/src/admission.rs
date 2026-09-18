use crate::{Priority, ProcessLimits, RequestPermit, SupervisorError};
use intent_contracts::WorkerInstanceId;
use std::{
    collections::{BTreeMap, VecDeque},
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug)]
pub struct AdmissionLimits {
    workers: usize,
    memory: u64,
    cpu_millis: u64,
    reserved_workers: [usize; 2],
    reserved_memory: [u64; 2],
    reserved_cpu: [u64; 2],
}
impl AdmissionLimits {
    pub fn new(
        workers: usize,
        memory: u64,
        cpu_millis: u64,
        reserved_workers: [usize; 2],
        reserved_memory: [u64; 2],
        reserved_cpu: [u64; 2],
    ) -> Result<Self, SupervisorError> {
        if !(1..=64).contains(&workers)
            || memory == 0
            || cpu_millis == 0
            || reserved_workers[0]
                .checked_add(reserved_workers[1])
                .is_none_or(|n| n > workers)
            || reserved_memory[0]
                .checked_add(reserved_memory[1])
                .is_none_or(|n| n > memory)
            || reserved_cpu[0]
                .checked_add(reserved_cpu[1])
                .is_none_or(|n| n > cpu_millis)
        {
            return Err(SupervisorError::InvalidConfiguration(
                "invalid admission reservations",
            ));
        }
        Ok(Self {
            workers,
            memory,
            cpu_millis,
            reserved_workers,
            reserved_memory,
            reserved_cpu,
        })
    }
    pub const fn workers(self) -> usize {
        self.workers
    }
}
#[derive(Debug)]
pub(crate) struct AdmissionBook {
    limits: AdmissionLimits,
    active: BTreeMap<WorkerInstanceId, (Priority, ProcessLimits)>,
}
impl AdmissionBook {
    pub(crate) fn new(limits: AdmissionLimits) -> Self {
        Self {
            limits,
            active: BTreeMap::new(),
        }
    }
    pub(crate) fn acquire(
        &mut self,
        id: WorkerInstanceId,
        priority: Priority,
        limits: ProcessLimits,
    ) -> Result<(), SupervisorError> {
        if self.active.contains_key(&id) {
            return Err(SupervisorError::InvalidState);
        }
        let mut count = [0_usize; 3];
        let mut memory = [0_u64; 3];
        let mut cpu = [0_u64; 3];
        for (class, resource) in self
            .active
            .values()
            .copied()
            .chain(std::iter::once((priority, limits)))
        {
            let lane = index(class);
            count[lane] += 1;
            memory[lane] = memory[lane]
                .checked_add(resource.address_space_bytes())
                .ok_or(SupervisorError::AdmissionExhausted)?;
            cpu[lane] = cpu[lane]
                .checked_add(u64::from(resource.cpu_admission_millis()))
                .ok_or(SupervisorError::AdmissionExhausted)?;
        }
        let reserved_count: usize = (0..2)
            .map(|i| self.limits.reserved_workers[i].saturating_sub(count[i]))
            .sum();
        let reserved_memory: u64 = (0..2)
            .map(|i| self.limits.reserved_memory[i].saturating_sub(memory[i]))
            .sum();
        let reserved_cpu: u64 = (0..2)
            .map(|i| self.limits.reserved_cpu[i].saturating_sub(cpu[i]))
            .sum();
        if count.iter().sum::<usize>() + reserved_count > self.limits.workers
            || memory
                .iter()
                .try_fold(reserved_memory, |sum, v| sum.checked_add(*v))
                .is_none_or(|v| v > self.limits.memory)
            || cpu
                .iter()
                .try_fold(reserved_cpu, |sum, v| sum.checked_add(*v))
                .is_none_or(|v| v > self.limits.cpu_millis)
        {
            return Err(SupervisorError::AdmissionExhausted);
        }
        self.active.insert(id, (priority, limits));
        Ok(())
    }
    pub(crate) fn release(&mut self, id: WorkerInstanceId) {
        self.active.remove(&id);
    }
    pub(crate) fn active_count(&self) -> usize {
        self.active.len()
    }
}
const fn index(priority: Priority) -> usize {
    match priority {
        Priority::Interactive => 0,
        Priority::Speech => 1,
        Priority::Background => 2,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SchedulerLimits {
    entries: [usize; 3],
    bytes: [usize; 3],
    age: Duration,
}
impl SchedulerLimits {
    pub fn new(
        entries: [usize; 3],
        bytes: [usize; 3],
        age: Duration,
    ) -> Result<Self, SupervisorError> {
        if entries.iter().any(|n| !(1..=256).contains(n))
            || bytes.iter().any(|n| !(4096..=4 * 1024 * 1024).contains(n))
            || age.is_zero()
            || age > Duration::from_secs(60)
        {
            return Err(SupervisorError::InvalidConfiguration(
                "invalid work queue budgets",
            ));
        }
        Ok(Self {
            entries,
            bytes,
            age,
        })
    }
}
impl Default for SchedulerLimits {
    fn default() -> Self {
        Self {
            entries: [32, 32, 32],
            bytes: [256 * 1024; 3],
            age: Duration::from_millis(100),
        }
    }
}
pub(crate) struct ScheduledMessage {
    pub(crate) permit: RequestPermit,
    pub(crate) bytes: Vec<u8>,
    pub(crate) enqueued: Instant,
}
#[derive(Debug)]
pub(crate) struct WorkQueues {
    limits: SchedulerLimits,
    queues: [VecDeque<ScheduledMessage>; 3],
    bytes: [usize; 3],
    turns: u64,
    pub(crate) dropped: u64,
}
impl WorkQueues {
    pub(crate) fn new(limits: SchedulerLimits) -> Self {
        Self {
            limits,
            queues: std::array::from_fn(|_| VecDeque::new()),
            bytes: [0; 3],
            turns: 0,
            dropped: 0,
        }
    }
    pub(crate) fn enqueue(
        &mut self,
        priority: Priority,
        message: ScheduledMessage,
    ) -> Result<(), SupervisorError> {
        let i = index(priority);
        if self.queues[i].len() >= self.limits.entries[i]
            || message.bytes.len() > self.limits.bytes[i].saturating_sub(self.bytes[i])
        {
            return Err(SupervisorError::QueueFull);
        }
        self.bytes[i] += message.bytes.len();
        self.queues[i].push_back(message);
        Ok(())
    }
    pub(crate) fn remove_generation(&mut self, generation: WorkerInstanceId) {
        for i in 0..3 {
            self.queues[i].retain(|item| {
                if item.permit.generation() == generation {
                    self.bytes[i] -= item.bytes.len();
                    self.dropped = self.dropped.saturating_add(1);
                    false
                } else {
                    true
                }
            });
        }
    }
    pub(crate) fn pop_ready(
        &mut self,
        now: Instant,
        eligible: impl Fn(WorkerInstanceId) -> bool,
    ) -> Option<ScheduledMessage> {
        for i in 0..3 {
            self.queues[i].retain(|item| {
                if item.permit.lease.is_revoked() || now >= item.permit.expires {
                    self.bytes[i] -= item.bytes.len();
                    self.dropped = self.dropped.saturating_add(1);
                    false
                } else {
                    true
                }
            });
        }
        let aged = self.queues[2]
            .iter()
            .any(|m| now.saturating_duration_since(m.enqueued) >= self.limits.age);
        let start = if aged && self.turns % 8 == 7 {
            2
        } else {
            (self.turns % 2) as usize
        };
        let order = match start {
            0 => [0, 1, 2],
            1 => [1, 0, 2],
            _ => [2, 0, 1],
        };
        for i in order {
            if let Some(position) = self.queues[i]
                .iter()
                .position(|item| eligible(item.permit.generation()))
            {
                let message = self.queues[i].remove(position)?;
                self.bytes[i] -= message.bytes.len();
                self.turns = self.turns.wrapping_add(1);
                return Some(message);
            }
        }
        None
    }
    pub(crate) fn len(&self) -> usize {
        self.queues.iter().map(VecDeque::len).sum()
    }
}

impl std::fmt::Debug for ScheduledMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScheduledMessage")
            .field("generation", &self.permit.generation())
            .field("request_id", &self.permit.request_id())
            .field("byte_count", &self.bytes.len())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{WorkerLease, WorkerScope};
    use intent_contracts::{AccountId, CapabilityId, RequestId, TaskId};
    use intent_local_transport::{MessageFamily, WorkerRole};
    use uuid::Uuid;
    fn id() -> WorkerInstanceId {
        WorkerInstanceId::from_uuid(Uuid::new_v4())
    }
    #[test]
    fn background_cannot_consume_interactive_or_speech_reservations() -> Result<(), SupervisorError>
    {
        let resource = ProcessLimits::new(32 * 1024 * 1024, 1, 32, 100)?;
        let mut book = AdmissionBook::new(AdmissionLimits::new(
            3,
            96 * 1024 * 1024,
            300,
            [1, 1],
            [32 * 1024 * 1024; 2],
            [100, 100],
        )?);
        let background = id();
        book.acquire(background, Priority::Background, resource)?;
        assert!(matches!(
            book.acquire(id(), Priority::Background, resource),
            Err(SupervisorError::AdmissionExhausted)
        ));
        book.acquire(id(), Priority::Interactive, resource)?;
        book.acquire(id(), Priority::Speech, resource)?;
        assert_eq!(book.active_count(), 3);
        book.release(background);
        book.acquire(id(), Priority::Background, resource)?;
        Ok(())
    }
    fn message(
        generation: WorkerInstanceId,
        bytes: usize,
        now: Instant,
    ) -> Result<ScheduledMessage, SupervisorError> {
        let capability = CapabilityId::from_uuid(Uuid::new_v4());
        let scope = WorkerScope {
            account_id: AccountId::from_uuid(Uuid::new_v4()),
            task_id: TaskId::from_uuid(Uuid::new_v4()),
        };
        let lease = WorkerLease::new(
            generation,
            scope,
            WorkerRole::FixtureWorker,
            [capability].into(),
            now + Duration::from_secs(2),
        );
        let permit = lease.admit(
            RequestId::from_uuid(Uuid::new_v4()),
            scope,
            capability,
            MessageFamily::LifecycleControl,
            now + Duration::from_secs(1),
        )?;
        Ok(ScheduledMessage {
            permit,
            bytes: vec![0; bytes],
            enqueued: now,
        })
    }
    #[test]
    fn full_progress_cannot_consume_foreground_bytes_or_block_other_workers()
    -> Result<(), SupervisorError> {
        let now = Instant::now();
        let mut queues = WorkQueues::new(SchedulerLimits::new(
            [2; 3],
            [4096; 3],
            Duration::from_millis(1),
        )?);
        let blocked = id();
        let ready = id();
        queues.enqueue(Priority::Background, message(blocked, 4096, now)?)?;
        assert!(
            queues
                .enqueue(Priority::Background, message(ready, 1, now)?)
                .is_err()
        );
        queues.enqueue(Priority::Interactive, message(blocked, 1, now)?)?;
        queues.enqueue(Priority::Interactive, message(ready, 1, now)?)?;
        let selected = queues
            .pop_ready(now, |id| id == ready)
            .ok_or(SupervisorError::QueueFull)?;
        assert_eq!(selected.permit.generation(), ready);
        queues.remove_generation(blocked);
        assert_eq!(queues.len(), 0);
        assert_eq!(queues.bytes, [0; 3]);
        Ok(())
    }
    #[test]
    fn aged_background_and_speech_both_receive_service() -> Result<(), SupervisorError> {
        let now = Instant::now();
        let mut queues = WorkQueues::new(SchedulerLimits::default());
        let bg = id();
        let speech = id();
        let fg = id();
        queues.enqueue(Priority::Background, message(bg, 5, now)?)?;
        for _ in 0..12 {
            queues.enqueue(Priority::Interactive, message(fg, 5, now)?)?;
            queues.enqueue(Priority::Speech, message(speech, 5, now)?)?;
        }
        let mut served = Vec::new();
        for _ in 0..8 {
            served.push(
                queues
                    .pop_ready(now + Duration::from_millis(200), |_| true)
                    .ok_or(SupervisorError::QueueFull)?
                    .permit
                    .generation(),
            );
        }
        assert!(served.contains(&bg));
        assert!(served.contains(&speech));
        assert!(served.contains(&fg));
        Ok(())
    }

    #[test]
    fn empty_speech_queue_does_not_promote_unaged_background_over_interactive()
    -> Result<(), SupervisorError> {
        let now = Instant::now();
        let mut queues = WorkQueues::new(SchedulerLimits::default());
        let fg = id();
        let bg = id();
        queues.enqueue(Priority::Background, message(bg, 5, now)?)?;
        for _ in 0..3 {
            queues.enqueue(Priority::Interactive, message(fg, 5, now)?)?;
        }
        for _ in 0..3 {
            assert_eq!(
                queues
                    .pop_ready(now, |_| true)
                    .ok_or(SupervisorError::QueueFull)?
                    .permit
                    .generation(),
                fg
            );
        }
        assert_eq!(
            queues
                .pop_ready(now, |_| true)
                .ok_or(SupervisorError::QueueFull)?
                .permit
                .generation(),
            bg
        );
        Ok(())
    }
}
