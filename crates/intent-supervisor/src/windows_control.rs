use crate::{
    HealthPolicy, LifecycleControl, SupervisorError, WindowsExecutableIdentity,
    WindowsPinnedAuthenticatedWorker,
};
use intent_contracts::CancellationId;
use intent_local_transport::WorkerIdentity;
use std::{
    fs::File,
    io::Write,
    process::{Child, ExitStatus},
    thread,
    time::{Duration, Instant},
};
use uuid::Uuid;

const MAX_CONTROL_BYTES: usize = 4096;
const RETRY_DELAY: Duration = Duration::from_millis(10);
const DROP_REAP_GRACE: Duration = Duration::from_millis(100);

/// Transport-neutral lifecycle control carried over the authenticated Windows control lane.
/// This wrapper preserves the executable pin while a cooperative cancellation is in flight and
/// keeps hard termination observation bounded by the shared health policy.
#[derive(Debug)]
pub struct WindowsControlledWorker {
    child: Option<Child>,
    control: Option<File>,
    progress: Option<File>,
    control_identity: WorkerIdentity,
    progress_identity: WorkerIdentity,
    executable: WindowsExecutableIdentity,
}

impl WindowsPinnedAuthenticatedWorker {
    pub fn into_controlled(self) -> Result<WindowsControlledWorker, SupervisorError> {
        let (child, control, progress, control_identity, progress_identity, executable) =
            self.into_parts()?;
        Ok(WindowsControlledWorker {
            child: Some(child),
            control: Some(control),
            progress: Some(progress),
            control_identity,
            progress_identity,
            executable,
        })
    }
}

impl WindowsControlledWorker {
    #[must_use]
    pub const fn control_identity(&self) -> &WorkerIdentity {
        &self.control_identity
    }

    #[must_use]
    pub const fn progress_identity(&self) -> &WorkerIdentity {
        &self.progress_identity
    }

    #[must_use]
    pub const fn executable_identity(&self) -> &WindowsExecutableIdentity {
        &self.executable
    }

    pub fn child_id(&self) -> Result<u32, SupervisorError> {
        self.child
            .as_ref()
            .map(Child::id)
            .ok_or(SupervisorError::InvalidState)
    }

    pub fn stop(
        mut self,
        health: HealthPolicy,
    ) -> Result<WindowsControlledStopReport, SupervisorError> {
        health.validate()?;
        let cancellation_id = CancellationId::from_uuid(Uuid::new_v4());

        if let Some(status) = self.child_mut()?.try_wait()? {
            return Ok(WindowsControlledStopReport {
                status,
                cancellation_id,
                cancel_sent: false,
                escalated: false,
            });
        }

        let control = LifecycleControl::cancel(
            self.control_identity.instance_id(),
            cancellation_id,
        );
        if let Err(error) = self.send_control(control) {
            return match self.terminate(health.terminate_grace) {
                Ok(_) => Err(error),
                Err(cleanup_error) => Err(cleanup_error),
            };
        }

        if let Some(status) = wait_for_child_exit(
            self.child_mut()?,
            Instant::now() + health.stop_grace,
        )? {
            return Ok(WindowsControlledStopReport {
                status,
                cancellation_id,
                cancel_sent: true,
                escalated: false,
            });
        }

        let status = self.terminate(health.terminate_grace)?;
        Ok(WindowsControlledStopReport {
            status,
            cancellation_id,
            cancel_sent: true,
            escalated: true,
        })
    }

    fn child_mut(&mut self) -> Result<&mut Child, SupervisorError> {
        self.child.as_mut().ok_or(SupervisorError::InvalidState)
    }

    fn send_control(&mut self, control: LifecycleControl) -> Result<(), SupervisorError> {
        let mut bytes = serde_json::to_vec(&control).map_err(|_| SupervisorError::Protocol)?;
        if bytes.len().saturating_add(1) > MAX_CONTROL_BYTES {
            return Err(SupervisorError::InvalidConfiguration(
                "Windows lifecycle control exceeds bounded pipe budget",
            ));
        }
        bytes.push(b'\n');
        let pipe = self
            .control
            .as_mut()
            .ok_or(SupervisorError::InvalidState)?;
        pipe.write_all(&bytes)?;
        pipe.flush()?;
        Ok(())
    }

    fn terminate(&mut self, terminate_grace: Duration) -> Result<ExitStatus, SupervisorError> {
        self.control = None;
        self.progress = None;
        let child = self.child_mut()?;
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        if let Err(error) = child.kill() {
            if let Some(status) = child.try_wait()? {
                return Ok(status);
            }
            return Err(error.into());
        }
        wait_for_child_exit(child, Instant::now() + terminate_grace)?
            .ok_or(SupervisorError::DeadlineExpired)
    }
}

impl Drop for WindowsControlledWorker {
    fn drop(&mut self) {
        self.control = None;
        self.progress = None;
        let Some(child) = self.child.as_mut() else {
            return;
        };
        if matches!(child.try_wait(), Ok(Some(_))) {
            return;
        }
        let _ = child.kill();
        let _ = wait_for_child_exit(child, Instant::now() + DROP_REAP_GRACE);
    }
}

#[derive(Debug)]
pub struct WindowsControlledStopReport {
    status: ExitStatus,
    cancellation_id: CancellationId,
    cancel_sent: bool,
    escalated: bool,
}

impl WindowsControlledStopReport {
    #[must_use]
    pub fn status(&self) -> &ExitStatus {
        &self.status
    }

    #[must_use]
    pub const fn cancellation_id(&self) -> &CancellationId {
        &self.cancellation_id
    }

    #[must_use]
    pub const fn cancel_sent(&self) -> bool {
        self.cancel_sent
    }

    #[must_use]
    pub const fn escalated(&self) -> bool {
        self.escalated
    }
}

fn wait_for_child_exit(
    child: &mut Child,
    deadline: Instant,
) -> Result<Option<ExitStatus>, SupervisorError> {
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(Some(status));
        }
        let now = Instant::now();
        if now >= deadline {
            return Ok(None);
        }
        let remaining = deadline.saturating_duration_since(now);
        thread::sleep(if remaining < RETRY_DELAY {
            remaining
        } else {
            RETRY_DELAY
        });
    }
}
