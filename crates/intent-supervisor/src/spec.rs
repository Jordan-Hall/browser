use crate::{RestartPolicy, SupervisorError};
use intent_contracts::{AccountId, CapabilityId, TaskId};
use intent_local_transport::WorkerRole;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, time::Duration};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkerScope {
    pub task_id: TaskId,
    pub account_id: AccountId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Interactive,
    Speech,
    Background,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionBoundary {
    /// Approved, cooperative local code. rlimits do not isolate same-user hostile processes.
    CooperativeLocal,
    /// Denied until an independently qualified hostile-process sandbox is installed.
    UnattendedUntrusted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProcessLimits {
    address_space_bytes: u64,
    cpu_seconds: u64,
    file_descriptors: u64,
    cpu_admission_millis: u32,
}
impl ProcessLimits {
    pub fn new(
        address_space_bytes: u64,
        cpu_seconds: u64,
        file_descriptors: u64,
        cpu_admission_millis: u32,
    ) -> Result<Self, SupervisorError> {
        if !(16 * 1024 * 1024..=64 * 1024 * 1024 * 1024).contains(&address_space_bytes)
            || !(1..=3600).contains(&cpu_seconds)
            || !(16..=4096).contains(&file_descriptors)
            || !(1..=64000).contains(&cpu_admission_millis)
        {
            return Err(SupervisorError::InvalidConfiguration(
                "process limits are outside supported bounds",
            ));
        }
        Ok(Self {
            address_space_bytes,
            cpu_seconds,
            file_descriptors,
            cpu_admission_millis,
        })
    }
    pub const fn address_space_bytes(self) -> u64 {
        self.address_space_bytes
    }
    pub const fn cpu_seconds(self) -> u64 {
        self.cpu_seconds
    }
    pub const fn file_descriptors(self) -> u64 {
        self.file_descriptors
    }
    pub const fn cpu_admission_millis(self) -> u32 {
        self.cpu_admission_millis
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HealthPolicy {
    pub handshake_timeout: Duration,
    pub heartbeat_timeout: Duration,
    pub work_progress_timeout: Duration,
    pub stop_grace: Duration,
    pub terminate_grace: Duration,
}
impl Default for HealthPolicy {
    fn default() -> Self {
        Self {
            handshake_timeout: Duration::from_secs(3),
            heartbeat_timeout: Duration::from_secs(2),
            work_progress_timeout: Duration::from_secs(5),
            stop_grace: Duration::from_millis(100),
            terminate_grace: Duration::from_millis(100),
        }
    }
}
impl HealthPolicy {
    pub(crate) fn validate(self) -> Result<(), SupervisorError> {
        for value in [
            self.handshake_timeout,
            self.heartbeat_timeout,
            self.work_progress_timeout,
            self.stop_grace,
            self.terminate_grace,
        ] {
            if value < Duration::from_millis(1) || value > Duration::from_secs(60) {
                return Err(SupervisorError::InvalidConfiguration(
                    "health timeout must be between 1 millisecond and 60 seconds",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct WorkerConfig {
    pub(crate) scope: WorkerScope,
    pub(crate) role: WorkerRole,
    pub(crate) capabilities: BTreeSet<CapabilityId>,
    pub(crate) priority: Priority,
    pub(crate) limits: ProcessLimits,
    pub(crate) health: HealthPolicy,
    pub(crate) lifetime: Duration,
    pub(crate) restart: RestartPolicy,
}
impl WorkerConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        scope: WorkerScope,
        role: WorkerRole,
        capabilities: impl IntoIterator<Item = CapabilityId>,
        priority: Priority,
        limits: ProcessLimits,
        health: HealthPolicy,
        lifetime: Duration,
        restart: RestartPolicy,
        boundary: ExecutionBoundary,
    ) -> Result<Self, SupervisorError> {
        if boundary != ExecutionBoundary::CooperativeLocal {
            return Err(SupervisorError::UnsupportedBoundary);
        }
        health.validate()?;
        if lifetime.is_zero() || lifetime > Duration::from_secs(86400) {
            return Err(SupervisorError::InvalidConfiguration(
                "worker lifetime must be positive and at most one day",
            ));
        }
        let mut bounded = BTreeSet::new();
        for (index, capability) in capabilities.into_iter().take(65).enumerate() {
            if index == 64 {
                return Err(SupervisorError::InvalidConfiguration(
                    "at most 64 supplied capabilities",
                ));
            }
            if !bounded.insert(capability) {
                return Err(SupervisorError::InvalidConfiguration(
                    "duplicate worker capability",
                ));
            }
        }
        Ok(Self {
            scope,
            role,
            capabilities: bounded,
            priority,
            limits,
            health,
            lifetime,
            restart,
        })
    }
    pub fn capabilities(&self) -> impl Iterator<Item = CapabilityId> + '_ {
        self.capabilities.iter().copied()
    }
    pub const fn lifetime(&self) -> Duration {
        self.lifetime
    }
    pub const fn scope(&self) -> WorkerScope {
        self.scope
    }
    pub const fn role(&self) -> WorkerRole {
        self.role
    }
    pub const fn limits(&self) -> ProcessLimits {
        self.limits
    }
}
