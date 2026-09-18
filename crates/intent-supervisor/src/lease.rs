use crate::{SupervisorError, WorkerScope};
use intent_contracts::{CapabilityId, RequestId, WorkerInstanceId};
use intent_local_transport::{MessageFamily, WorkerRole};
use std::{
    collections::BTreeSet,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Instant,
};

const REVOKED: u64 = 1 << 63;

#[derive(Debug)]
struct Gate {
    state: AtomicU64,
}

/// Clones share one revocation cell; they cannot revive a generation.
/// This is worker-scoped admission, not approval to perform an external effect.
#[derive(Clone, Debug)]
pub struct WorkerLease {
    gate: Arc<Gate>,
    generation: WorkerInstanceId,
    scope: WorkerScope,
    role: WorkerRole,
    capabilities: Arc<BTreeSet<CapabilityId>>,
    expires: Instant,
}

/// Move-only request admission. It cannot be deserialized, cloned or constructed externally.
///
/// ```compile_fail
/// use intent_supervisor::RequestPermit;
/// fn duplicate(permit: RequestPermit) { let _ = permit.clone(); }
/// ```
#[derive(Debug)]
pub struct RequestPermit {
    pub(crate) lease: WorkerLease,
    pub(crate) request_id: RequestId,
    pub(crate) capability: CapabilityId,
    pub(crate) family: MessageFamily,
    pub(crate) expires: Instant,
    sequence: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RevocationReceipt {
    pub generation: WorkerInstanceId,
    pub newly_revoked: bool,
    /// Requests admitted before revocation are not evidence of rollback or provider cancellation.
    pub previously_admitted_requests: u64,
}

impl WorkerLease {
    pub(crate) fn new(
        generation: WorkerInstanceId,
        scope: WorkerScope,
        role: WorkerRole,
        capabilities: BTreeSet<CapabilityId>,
        expires: Instant,
    ) -> Self {
        Self {
            gate: Arc::new(Gate {
                state: AtomicU64::new(0),
            }),
            generation,
            scope,
            role,
            capabilities: Arc::new(capabilities),
            expires,
        }
    }
    pub const fn generation(&self) -> WorkerInstanceId {
        self.generation
    }
    pub const fn scope(&self) -> WorkerScope {
        self.scope
    }
    pub fn is_revoked(&self) -> bool {
        self.gate.state.load(Ordering::Acquire) & REVOKED != 0
    }
    pub fn revoke(&self) -> RevocationReceipt {
        let previous = self.gate.state.fetch_or(REVOKED, Ordering::AcqRel);
        RevocationReceipt {
            generation: self.generation,
            newly_revoked: previous & REVOKED == 0,
            previously_admitted_requests: previous & !REVOKED,
        }
    }
    pub fn admit(
        &self,
        request_id: RequestId,
        scope: WorkerScope,
        capability: CapabilityId,
        family: MessageFamily,
        expires: Instant,
    ) -> Result<RequestPermit, SupervisorError> {
        let now = Instant::now();
        if now >= self.expires || now >= expires {
            return Err(SupervisorError::DeadlineExpired);
        }
        if scope != self.scope {
            return Err(SupervisorError::ScopeMismatch);
        }
        if !self.capabilities.contains(&capability) || !self.role.allows(family) {
            return Err(SupervisorError::CapabilityDenied);
        }
        let previous = self
            .gate
            .state
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| {
                if value & REVOKED != 0 || value == REVOKED - 1 {
                    None
                } else {
                    Some(value + 1)
                }
            })
            .map_err(|value| {
                if value & REVOKED != 0 {
                    SupervisorError::Revoked
                } else {
                    SupervisorError::CounterExhausted
                }
            })?;
        Ok(RequestPermit {
            lease: self.clone(),
            request_id,
            capability,
            family,
            expires: expires.min(self.expires),
            sequence: previous + 1,
        })
    }
    pub(crate) fn same_generation(&self, other: &Self) -> bool {
        self.generation == other.generation && Arc::ptr_eq(&self.gate, &other.gate)
    }
    pub(crate) fn expired(&self, now: Instant) -> bool {
        now >= self.expires
    }
}
impl RequestPermit {
    pub const fn request_id(&self) -> RequestId {
        self.request_id
    }
    pub const fn generation(&self) -> WorkerInstanceId {
        self.lease.generation
    }
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }
    pub(crate) fn validate(
        &self,
        active: &WorkerLease,
        now: Instant,
    ) -> Result<(), SupervisorError> {
        if !self.lease.same_generation(active) || self.lease.is_revoked() {
            return Err(SupervisorError::Revoked);
        }
        if now >= self.expires || active.expired(now) {
            return Err(SupervisorError::DeadlineExpired);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use uuid::Uuid;
    fn lease() -> (WorkerLease, CapabilityId) {
        let capability = CapabilityId::from_uuid(Uuid::new_v4());
        (
            WorkerLease::new(
                WorkerInstanceId::from_uuid(Uuid::new_v4()),
                WorkerScope {
                    task_id: intent_contracts::TaskId::from_uuid(Uuid::new_v4()),
                    account_id: intent_contracts::AccountId::from_uuid(Uuid::new_v4()),
                },
                WorkerRole::ConnectorHost,
                [capability].into(),
                Instant::now() + Duration::from_secs(30),
            ),
            capability,
        )
    }
    #[test]
    fn revocation_is_shared_and_permanently_denies_late_admission() -> Result<(), SupervisorError> {
        let (lease, cap) = lease();
        let copy = lease.clone();
        let permit = lease.admit(
            RequestId::from_uuid(Uuid::new_v4()),
            lease.scope(),
            cap,
            MessageFamily::ConnectorCall,
            Instant::now() + Duration::from_secs(1),
        )?;
        let receipt = copy.revoke();
        assert!(receipt.newly_revoked);
        assert_eq!(receipt.previously_admitted_requests, 1);
        assert!(!lease.revoke().newly_revoked);
        assert!(matches!(
            permit.validate(&lease, Instant::now()),
            Err(SupervisorError::Revoked)
        ));
        assert!(matches!(
            copy.admit(
                RequestId::from_uuid(Uuid::new_v4()),
                lease.scope(),
                cap,
                MessageFamily::ConnectorCall,
                Instant::now() + Duration::from_secs(1)
            ),
            Err(SupervisorError::Revoked)
        ));
        Ok(())
    }
    #[test]
    fn scope_role_capability_deadline_and_generation_are_all_checked() -> Result<(), SupervisorError>
    {
        let (lease, cap) = lease();
        let request = RequestId::from_uuid(Uuid::new_v4());
        let future = Instant::now() + Duration::from_secs(1);
        let wrong = WorkerScope {
            account_id: intent_contracts::AccountId::from_uuid(Uuid::new_v4()),
            ..lease.scope()
        };
        assert!(matches!(
            lease.admit(request, wrong, cap, MessageFamily::ConnectorCall, future),
            Err(SupervisorError::ScopeMismatch)
        ));
        assert!(matches!(
            lease.admit(
                request,
                lease.scope(),
                cap,
                MessageFamily::PolicyDecision,
                future
            ),
            Err(SupervisorError::CapabilityDenied)
        ));
        assert!(matches!(
            lease.admit(
                request,
                lease.scope(),
                CapabilityId::from_uuid(Uuid::new_v4()),
                MessageFamily::ConnectorCall,
                future
            ),
            Err(SupervisorError::CapabilityDenied)
        ));
        assert!(matches!(
            lease.admit(
                request,
                lease.scope(),
                cap,
                MessageFamily::ConnectorCall,
                Instant::now()
            ),
            Err(SupervisorError::DeadlineExpired)
        ));
        let permit = lease.admit(
            request,
            lease.scope(),
            cap,
            MessageFamily::ConnectorCall,
            future,
        )?;
        let other = WorkerLease::new(
            lease.generation(),
            lease.scope(),
            lease.role,
            [cap].into(),
            future,
        );
        assert!(matches!(
            permit.validate(&other, Instant::now()),
            Err(SupervisorError::Revoked)
        ));
        Ok(())
    }
    #[test]
    fn concurrent_revocation_has_a_single_admission_order() -> Result<(), Box<dyn std::error::Error>>
    {
        let (lease, cap) = lease();
        let shared = lease.clone();
        let worker = std::thread::spawn(move || {
            let mut accepted = 0;
            for _ in 0..1024 {
                if shared
                    .admit(
                        RequestId::from_uuid(Uuid::new_v4()),
                        shared.scope(),
                        cap,
                        MessageFamily::ConnectorCall,
                        Instant::now() + Duration::from_secs(1),
                    )
                    .is_ok()
                {
                    accepted += 1;
                }
            }
            accepted
        });
        let receipt = lease.revoke();
        let accepted = worker.join().map_err(|_| "admission worker panicked")?;
        assert_eq!(receipt.previously_admitted_requests, accepted);
        assert!(lease.is_revoked());
        Ok(())
    }
}
