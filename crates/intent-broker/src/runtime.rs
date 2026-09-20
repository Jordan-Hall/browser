use crate::{
    BrokerError, CancellationFailure, CancellationFailureCause, CancellationPersistenceStatus,
    CancellationTerminationStatus, invocation,
};
use intent_contracts::{
    CancellationId, CapabilityId, OperationAttemptId, OperationId, OutboxMessageId, RequestId,
    UnixTimestampMicros, WorkerInstanceId,
};
use intent_local_transport::{MessageFamily, WorkerRole};
use intent_state::{
    AuthorityUpdate, DurableOperationState, DurableSendAttempt, RecoverableAction,
    RecoveryTaskView, RuntimeOwner, StartupBatch, StateStore, TransportObservation,
    WorkerRegistration,
};
use intent_supervisor::{
    AdmissionLimits, ExecutableImage, PollReport, SchedulerLimits, Supervisor, TerminalReport,
    WorkerConfig, WorkerLease, WorkerScope, WorkerSnapshot, WorkerState,
};
use std::{
    collections::BTreeMap,
    path::Path,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

const MAX_INFLIGHT: usize = 128;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DispatchTicket {
    pub operation_id: OperationId,
    pub attempt_id: OperationAttemptId,
    pub request_id: RequestId,
    pub worker_id: WorkerInstanceId,
}
#[derive(Clone, Copy, Debug)]
pub struct DispatchObservation {
    pub ticket: DispatchTicket,
    pub state: DurableOperationState,
}
#[derive(Debug)]
pub struct BrokerPoll {
    pub workers: PollReport,
    pub outcomes: Vec<DispatchObservation>,
}
struct WorkerRecord {
    scope: WorkerScope,
    capabilities: Vec<CapabilityId>,
    expires_at: UnixTimestampMicros,
    lease: Option<WorkerLease>,
    revoked: bool,
}
struct Inflight {
    ticket: DispatchTicket,
    attempt: DurableSendAttempt,
    deadline: Instant,
}

/// One mutable owner serializes authority updates, durable attempt admission, socket submission
/// and revocation. Neither the raw supervisor nor a mutable RuntimeOwner can escape this bridge.
/// Run this on a trusted runtime executor, not the UI event loop: SQLite fsync can block.
/// Previously admitted writes can finish after cancellation and remain tracked as uncertain.
pub struct RuntimeBroker {
    // Drop the supervisor before releasing profile ownership.
    supervisor: Supervisor,
    owner: RuntimeOwner,
    workers: BTreeMap<WorkerInstanceId, WorkerRecord>,
    inflight: BTreeMap<RequestId, Inflight>,
    last_wall: UnixTimestampMicros,
    faulted: bool,
}
impl RuntimeBroker {
    pub fn new(
        owner: RuntimeOwner,
        runtime_parent: &Path,
        admission: AdmissionLimits,
        scheduler: SchedulerLimits,
    ) -> Result<Self, BrokerError> {
        Ok(Self {
            supervisor: Supervisor::new(runtime_parent, admission, scheduler)?,
            owner,
            workers: BTreeMap::new(),
            inflight: BTreeMap::new(),
            last_wall: wall_now()?,
            faulted: false,
        })
    }
    pub fn state(&self) -> &StateStore {
        self.owner.state()
    }
    pub fn epoch(&self) -> Uuid {
        self.owner.epoch()
    }
    pub fn is_fenced(&self) -> bool {
        self.faulted
    }
    pub fn inflight_count(&self) -> usize {
        self.inflight.len()
    }
    pub fn plan_startup(&mut self, limit: usize) -> Result<StartupBatch, BrokerError> {
        let now = self.now()?;
        Ok(self.owner.plan_startup(now, limit)?)
    }
    pub fn activate_after_planning(&mut self) -> Result<(), BrokerError> {
        let now = self.now()?;
        Ok(self.owner.activate_after_planning(now)?)
    }
    pub fn prepare_action(
        &mut self,
        mut action: RecoverableAction,
    ) -> Result<OperationId, BrokerError> {
        let now = self.now()?;
        let routing = serde_json::to_string(action.destination.as_str())
            .map_err(|_| BrokerError::Invalid("destination encoding"))?
            .len()
            + serde_json::to_string(action.message_kind.as_str())
                .map_err(|_| BrokerError::Invalid("message kind encoding"))?
                .len();
        invocation::check_shape(action.payload.len() as u64, routing as u64)?;
        action.operation.created_at = now;
        Ok(self.owner.prepare_action(action)?)
    }
    pub fn approve_action(
        &mut self,
        id: OperationId,
        revision: u64,
        expires: UnixTimestampMicros,
    ) -> Result<u64, BrokerError> {
        let now = self.now()?;
        Ok(self.owner.approve_action(id, revision, expires, now)?)
    }
    pub fn enqueue_action(
        &mut self,
        id: OperationId,
        revision: u64,
    ) -> Result<OutboxMessageId, BrokerError> {
        let now = self.now()?;
        Ok(self.owner.enqueue_action(id, revision, now)?)
    }
    /// Apply verified read-only evidence only after the matching transport attempt has settled.
    pub fn reconcile(
        &mut self,
        evidence: intent_state::VerifiedReadOnlyEvidence,
        revision: u64,
    ) -> Result<u64, BrokerError> {
        let now = self.now()?;
        let binding = evidence.binding();
        if self.inflight.values().any(|p| {
            p.ticket.operation_id == binding.operation_id
                && p.ticket.attempt_id == binding.attempt_id
        }) {
            return Err(BrokerError::Invalid(
                "settle or cancel the matching transport before reconciliation",
            ));
        }
        Ok(self.owner.reconcile(evidence, revision, now)?)
    }
    pub fn recovery_view(&mut self, id: OperationId) -> Result<RecoveryTaskView, BrokerError> {
        let now = self.now()?;
        Ok(self.owner.recovery_view(id, now)?)
    }
    /// Every policy/source revision invalidates dependent worker generations, even a new enabled grant.
    pub fn update_authority(&mut self, input: AuthorityUpdate) -> Result<u64, BrokerError> {
        let now = self.now()?;
        let ids: Vec<_> = self
            .workers
            .iter()
            .filter(|(_, worker)| {
                worker.scope.account_id == input.account_id
                    && worker.capabilities.contains(&input.capability_id)
                    && !worker.revoked
            })
            .map(|(id, _)| *id)
            .collect();
        let revision = match self.owner.update_authority(input, now) {
            Ok(revision) => revision,
            Err(error) => {
                self.fence();
                return Err(error.into());
            }
        };
        for id in ids {
            self.cancel_worker(id)?;
        }
        Ok(revision)
    }
    /// Only a real authenticated Ready generation is registered with the durable owner (in poll).
    pub fn launch(
        &mut self,
        image: ExecutableImage,
        config: WorkerConfig,
        args: &[String],
    ) -> Result<WorkerInstanceId, BrokerError> {
        let now = self.now()?;
        if config.role() != WorkerRole::ConnectorHost {
            return Err(BrokerError::Invalid(
                "broker requires a connector-host role",
            ));
        }
        let record = WorkerRecord {
            scope: config.scope(),
            capabilities: config.capabilities().collect(),
            expires_at: timestamp_add(now, config.lifetime())?,
            lease: None,
            revoked: false,
        };
        if record.capabilities.is_empty() {
            return Err(BrokerError::Invalid("worker has no capabilities"));
        }
        let id = self.supervisor.launch(image, config, args)?;
        self.workers.insert(id, record);
        Ok(id)
    }
    pub fn worker_snapshot(&self, worker: WorkerInstanceId) -> Result<WorkerSnapshot, BrokerError> {
        Ok(self.supervisor.snapshot(worker)?)
    }
    /// Commit the exact attempt before the first socket write. Never use the asynchronous work queue
    /// for effects: a later policy update must not leave an unstarted effect sitting in that queue.
    pub fn dispatch(
        &mut self,
        outbox: OutboxMessageId,
        worker: WorkerInstanceId,
        timeout: Duration,
    ) -> Result<DispatchTicket, BrokerError> {
        let now = self.now()?;
        if timeout.is_zero() || timeout > Duration::from_secs(60) {
            return Err(BrokerError::Invalid("dispatch timeout must be in (0,60s]"));
        }
        if self.inflight.len() >= MAX_INFLIGHT {
            return Err(BrokerError::Invalid("inflight budget"));
        }
        let metadata = self.owner.dispatch_metadata(outbox)?;
        invocation::check_shape(metadata.payload_bytes, metadata.routing_json_bytes)?;
        let record = self
            .workers
            .get(&worker)
            .ok_or(BrokerError::Invalid("worker is not owned by this broker"))?;
        if record.revoked {
            return Err(BrokerError::Invalid("worker is revoked"));
        }
        let lease = record
            .lease
            .as_ref()
            .ok_or(BrokerError::Invalid("worker readiness is not registered"))?;
        let scope = WorkerScope {
            task_id: metadata.task_id,
            account_id: metadata.account_id,
        };
        let expires = timestamp_add(now, timeout)?
            .min(metadata.deadline)
            .min(record.expires_at);
        let remaining = expires
            .get()
            .checked_sub(now.get())
            .filter(|n| *n > 0)
            .ok_or(BrokerError::Clock)?;
        let deadline = Instant::now()
            .checked_add(Duration::from_micros(remaining as u64))
            .ok_or(BrokerError::Clock)?;
        let request_id = RequestId::from_uuid(Uuid::new_v4());
        let permit = lease.admit(
            request_id,
            scope,
            metadata.capability_id,
            MessageFamily::ConnectorCall,
            deadline,
        )?;
        self.supervisor.can_submit_immediate(&permit)?;
        let claim = self.owner.claim_dispatch(outbox, worker, expires, now)?;
        // Refresh time after potentially slow claim/lock acquisition. A stale deadline is never extended.
        let now = self.now()?;
        let attempt = self.owner.begin_authorized_dispatch(claim, now)?;
        let ticket = DispatchTicket {
            operation_id: attempt.operation_id(),
            attempt_id: attempt.attempt_id(),
            request_id,
            worker_id: worker,
        };
        let encoded =
            invocation::encode(&attempt, metadata, self.owner.epoch(), worker, request_id);
        let submitted = encoded.and_then(|input| {
            self.supervisor
                .submit_immediate(permit, input)
                .map_err(BrokerError::from)
        });
        if submitted.is_err() {
            if self
                .owner
                .record_transport_observation(
                    attempt,
                    TransportObservation::OutcomeUnknown,
                    self.last_wall,
                )
                .is_err()
            {
                self.fence();
            }
            return Err(BrokerError::Uncertain {
                operation_id: ticket.operation_id,
                attempt_id: ticket.attempt_id,
            });
        }
        self.inflight.insert(
            request_id,
            Inflight {
                ticket,
                attempt,
                deadline,
            },
        );
        Ok(ticket)
    }
    /// Latch local revocation before any fallible disk work; persist it before sending Cancel.
    /// A persistence or termination-request failure fences all future dispatches and returns an
    /// explicit bounded status rather than implying that cancellation completed durably.
    pub fn cancel_worker(
        &mut self,
        worker: WorkerInstanceId,
    ) -> Result<Vec<DispatchObservation>, BrokerError> {
        let record = self
            .workers
            .get_mut(&worker)
            .ok_or(BrokerError::Invalid("unknown broker worker"))?;
        if let Some(lease) = &record.lease {
            lease.revoke();
        }
        record.revoked = true;
        let registered = record.lease.is_some();
        let now = self.now()?;
        let persistence = if registered {
            match self.owner.revoke_worker(worker, now) {
                Ok(()) => CancellationPersistenceStatus::Persisted,
                Err(error) => {
                    let termination = self.fence_with_cancellation_status(worker);
                    return Err(BrokerError::Cancellation {
                        status: CancellationFailure {
                            worker_id: worker,
                            persistence: CancellationPersistenceStatus::Failed,
                            termination,
                        },
                        source: CancellationFailureCause::Persistence(error),
                    });
                }
            }
        } else {
            CancellationPersistenceStatus::NotRequired
        };
        if let Err(error) = self
            .supervisor
            .cancel(worker, CancellationId::from_uuid(worker.as_uuid()))
        {
            self.fence();
            return Err(BrokerError::Cancellation {
                status: CancellationFailure {
                    worker_id: worker,
                    persistence,
                    termination: CancellationTerminationStatus::Failed,
                },
                source: CancellationFailureCause::Termination(error),
            });
        }
        self.settle_generation(worker, now)
    }
    pub fn poll(&mut self) -> Result<BrokerPoll, BrokerError> {
        if self.faulted {
            self.supervisor.poll();
            return Err(BrokerError::Blocked);
        }
        self.now()?;
        let workers = self.supervisor.poll();
        let mut outcomes = Vec::new();
        let ids: Vec<_> = self.workers.keys().copied().collect();
        for id in ids {
            let now = self.now()?;
            let snapshot = self.supervisor.snapshot(id)?;
            let record = self
                .workers
                .get_mut(&id)
                .ok_or(BrokerError::Invalid("worker registry"))?;
            if record.lease.is_none() && !record.revoked && snapshot.state == WorkerState::Ready {
                let registration = WorkerRegistration {
                    worker_id: id,
                    task_id: record.scope.task_id,
                    account_id: record.scope.account_id,
                    capabilities: record.capabilities.clone(),
                    expires_at: record.expires_at,
                };
                if let Err(error) = self.owner.register_worker(registration, now) {
                    self.fence();
                    return Err(error.into());
                }
                record.lease = Some(self.supervisor.lease(id)?);
            }
            if matches!(
                snapshot.state,
                WorkerState::Draining | WorkerState::Stopped | WorkerState::Failed
            ) && !record.revoked
            {
                outcomes.extend(self.cancel_worker(id)?);
            }
        }
        let requests: Vec<_> = self.inflight.keys().copied().collect();
        for request in requests {
            let pending = self
                .inflight
                .get(&request)
                .ok_or(BrokerError::Invalid("inflight registry"))?;
            let result = self
                .supervisor
                .take_result(pending.ticket.worker_id, request)?;
            let observed = if result.is_some() {
                Some(TransportObservation::AcceptedUnverified)
            } else if Instant::now() >= pending.deadline {
                Some(TransportObservation::OutcomeUnknown)
            } else {
                None
            };
            if let Some(observed) = observed {
                let now = self.now()?;
                outcomes.push(self.settle(request, observed, now)?);
            }
        }
        Ok(BrokerPoll { workers, outcomes })
    }
    /// Retirement requires a reaped child and records durable revocation; it never resends work.
    pub fn retire(&mut self, worker: WorkerInstanceId) -> Result<TerminalReport, BrokerError> {
        self.now()?;
        if self.workers.get(&worker).is_none_or(|w| !w.revoked) {
            return Err(BrokerError::Invalid(
                "worker must be revoked before retirement",
            ));
        }
        let report = self.supervisor.retire(worker)?;
        self.workers.remove(&worker);
        Ok(report)
    }
    fn settle_generation(
        &mut self,
        worker: WorkerInstanceId,
        now: UnixTimestampMicros,
    ) -> Result<Vec<DispatchObservation>, BrokerError> {
        let ids: Vec<_> = self
            .inflight
            .iter()
            .filter(|(_, p)| p.ticket.worker_id == worker)
            .map(|(id, _)| *id)
            .collect();
        ids.into_iter()
            .map(|id| self.settle(id, TransportObservation::OutcomeUnknown, now))
            .collect()
    }
    fn settle(
        &mut self,
        request: RequestId,
        observation: TransportObservation,
        now: UnixTimestampMicros,
    ) -> Result<DispatchObservation, BrokerError> {
        let pending = self
            .inflight
            .remove(&request)
            .ok_or(BrokerError::Invalid("unknown inflight request"))?;
        let state = match observation {
            TransportObservation::AcceptedUnverified => DurableOperationState::Accepted,
            TransportObservation::OutcomeUnknown => DurableOperationState::NeedsReconciliation,
        };
        if let Err(error) =
            self.owner
                .record_transport_observation(pending.attempt, observation, now)
        {
            self.fence();
            return Err(error.into());
        }
        Ok(DispatchObservation {
            ticket: pending.ticket,
            state,
        })
    }
    fn now(&mut self) -> Result<UnixTimestampMicros, BrokerError> {
        if self.faulted {
            return Err(BrokerError::Blocked);
        }
        match wall_now() {
            Ok(now) if now >= self.last_wall => {
                self.last_wall = now;
                Ok(now)
            }
            _ => {
                self.fence();
                Err(BrokerError::Clock)
            }
        }
    }
    fn fence_with_cancellation_status(
        &mut self,
        target: WorkerInstanceId,
    ) -> CancellationTerminationStatus {
        self.faulted = true;
        let mut target_status = CancellationTerminationStatus::Failed;
        for (id, record) in &mut self.workers {
            if let Some(lease) = &record.lease {
                lease.revoke();
            }
            record.revoked = true;
            let result = self
                .supervisor
                .cancel(*id, CancellationId::from_uuid(id.as_uuid()));
            if *id == target {
                target_status = if result.is_ok() {
                    CancellationTerminationStatus::Requested
                } else {
                    CancellationTerminationStatus::Failed
                };
            }
        }
        target_status
    }
    fn fence(&mut self) {
        self.faulted = true;
        for (id, record) in &mut self.workers {
            if let Some(lease) = &record.lease {
                lease.revoke();
            }
            record.revoked = true;
            let _ = self
                .supervisor
                .cancel(*id, CancellationId::from_uuid(id.as_uuid()));
        }
    }
}
impl Drop for RuntimeBroker {
    fn drop(&mut self) {
        self.fence();
    }
}
fn wall_now() -> Result<UnixTimestampMicros, BrokerError> {
    let micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| BrokerError::Clock)?
        .as_micros();
    UnixTimestampMicros::try_new(i64::try_from(micros).map_err(|_| BrokerError::Clock)?)
        .map_err(|_| BrokerError::Clock)
}
fn timestamp_add(
    now: UnixTimestampMicros,
    duration: Duration,
) -> Result<UnixTimestampMicros, BrokerError> {
    let delta = i64::try_from(duration.as_micros()).map_err(|_| BrokerError::Clock)?;
    UnixTimestampMicros::try_new(now.get().checked_add(delta).ok_or(BrokerError::Clock)?)
        .map_err(|_| BrokerError::Clock)
}
