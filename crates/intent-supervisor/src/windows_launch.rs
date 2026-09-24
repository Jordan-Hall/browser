use crate::{
    HealthPolicy, RestartBudget, RestartDecision, RestartPolicy, SupervisorError,
    WindowsControlledStopReport, WindowsControlledWorker,
};
use intent_contracts::{BoundedText, WorkerInstanceId};
use intent_local_transport::{
    ExpectedPeer, WorkerChannel, WorkerHello, WorkerIdentity, WorkerRole, WorkerVerifier,
    create_current_user_named_pipe, issue_worker_channel_authentication,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read, Write},
    os::windows::io::AsRawHandle,
    process::{Child, Command, ExitStatus, Stdio},
    ptr::null_mut,
    thread,
    time::{Duration, Instant},
};
use uuid::Uuid;
use windows_sys::Win32::{
    Foundation::{ERROR_NO_DATA, ERROR_PIPE_CONNECTED, ERROR_PIPE_LISTENING},
    System::Pipes::ConnectNamedPipe,
};

const MAX_BOOTSTRAP_BYTES: usize = 4096;
const PIPE_BUFFER_BYTES: u32 = 4096;
const MAX_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(60);
const MAX_WORKER_LIFETIME: Duration = Duration::from_secs(86400);
const RETRY_DELAY: Duration = Duration::from_millis(10);

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WindowsBootstrapPacket {
    pub supervisor_process_id: u32,
    pub control_endpoint: BoundedText<256>,
    pub progress_endpoint: BoundedText<256>,
    pub control: WorkerHello,
    pub progress: WorkerHello,
    pub handshake_timeout_millis: u32,
}

#[derive(Debug)]
struct PendingLane {
    pipe: Option<File>,
    verifier: Option<WorkerVerifier>,
    channel: WorkerChannel,
}

impl PendingLane {
    fn authenticate(
        &mut self,
        deadline: Instant,
    ) -> Result<(File, WorkerIdentity), SupervisorError> {
        let pipe = self.pipe.as_mut().ok_or(SupervisorError::InvalidState)?;
        wait_for_connection(pipe, deadline)?;
        let verifier = self
            .verifier
            .as_mut()
            .ok_or(SupervisorError::InvalidState)?;
        verifier
            .check_named_pipe_client(pipe)
            .map_err(|_| SupervisorError::Protocol)?;
        let hello: WorkerHello = serde_json::from_slice(&read_line(pipe, deadline)?)
            .map_err(|_| SupervisorError::Protocol)?;
        let identity = verifier
            .authenticate_named_pipe_client_channel(self.channel, &hello, pipe)
            .map_err(|_| SupervisorError::Protocol)?;
        self.verifier = None;
        Ok((
            self.pipe.take().ok_or(SupervisorError::InvalidState)?,
            identity,
        ))
    }
}

#[derive(Debug)]
pub struct WindowsPendingWorker {
    child: Option<Child>,
    control: PendingLane,
    progress: PendingLane,
    generation: WorkerInstanceId,
    role: WorkerRole,
    deadline: Instant,
}

impl WindowsPendingWorker {
    pub(crate) fn spawn(
        command: &mut Command,
        generation: WorkerInstanceId,
        role: WorkerRole,
        handshake_timeout: Duration,
    ) -> Result<Self, SupervisorError> {
        if handshake_timeout.is_zero() || handshake_timeout > MAX_HANDSHAKE_TIMEOUT {
            return Err(SupervisorError::InvalidConfiguration(
                "Windows worker handshake timeout must be 1ms..=60s",
            ));
        }
        let timeout_millis = u32::try_from(handshake_timeout.as_millis()).map_err(|_| {
            SupervisorError::InvalidConfiguration("Windows worker handshake timeout overflow")
        })?;
        if timeout_millis == 0 {
            return Err(SupervisorError::InvalidConfiguration(
                "Windows worker handshake timeout must be at least 1ms",
            ));
        }

        let nonce = Uuid::new_v4();
        let control_name = format!(
            r"\\.\pipe\intent-supervisor-{}-{nonce}-control",
            std::process::id()
        );
        let progress_name = format!(
            r"\\.\pipe\intent-supervisor-{}-{nonce}-progress",
            std::process::id()
        );
        let control_pipe = create_current_user_named_pipe(control_name.as_ref(), PIPE_BUFFER_BYTES)
            .map_err(|_| SupervisorError::Protocol)?;
        let progress_pipe =
            create_current_user_named_pipe(progress_name.as_ref(), PIPE_BUFFER_BYTES)
                .map_err(|_| SupervisorError::Protocol)?;
        let (control_token, control_pending) =
            issue_worker_channel_authentication(generation, role, WorkerChannel::Control)
                .map_err(|_| SupervisorError::InvalidConfiguration("OS randomness unavailable"))?;
        let (progress_token, progress_pending) =
            issue_worker_channel_authentication(generation, role, WorkerChannel::Progress)
                .map_err(|_| SupervisorError::InvalidConfiguration("OS randomness unavailable"))?;
        let packet = WindowsBootstrapPacket {
            supervisor_process_id: std::process::id(),
            control_endpoint: BoundedText::try_new(control_name).map_err(|_| {
                SupervisorError::InvalidConfiguration("Windows control endpoint too long")
            })?,
            progress_endpoint: BoundedText::try_new(progress_name).map_err(|_| {
                SupervisorError::InvalidConfiguration("Windows progress endpoint too long")
            })?,
            control: WorkerHello::new(generation, role, control_token),
            progress: WorkerHello::new(generation, role, progress_token),
            handshake_timeout_millis: timeout_millis,
        };
        let bootstrap = serde_json::to_vec(&packet).map_err(|_| SupervisorError::Protocol)?;
        if bootstrap.len() > MAX_BOOTSTRAP_BYTES {
            return Err(SupervisorError::InvalidConfiguration(
                "Windows worker bootstrap exceeds bounded stdin budget",
            ));
        }

        let mut child = command.stdin(Stdio::piped()).spawn()?;
        let expected =
            ExpectedPeer::windows_process(child.id()).map_err(|_| SupervisorError::Protocol)?;
        let mut pending = Self {
            child: Some(child),
            control: PendingLane {
                pipe: Some(control_pipe),
                verifier: Some(control_pending.bind(expected)),
                channel: WorkerChannel::Control,
            },
            progress: PendingLane {
                pipe: Some(progress_pipe),
                verifier: Some(progress_pending.bind(expected)),
                channel: WorkerChannel::Progress,
            },
            generation,
            role,
            deadline: Instant::now() + handshake_timeout,
        };
        pending
            .child
            .as_mut()
            .and_then(|child| child.stdin.take())
            .ok_or(SupervisorError::InvalidState)?
            .write_all(&bootstrap)?;
        Ok(pending)
    }

    pub fn child_id(&self) -> Result<u32, SupervisorError> {
        self.child
            .as_ref()
            .map(Child::id)
            .ok_or(SupervisorError::InvalidState)
    }

    pub fn revoke_before_auth(mut self) -> Result<ExitStatus, SupervisorError> {
        self.control.verifier = None;
        self.progress.verifier = None;
        self.control.pipe = None;
        self.progress.pipe = None;
        let mut child = self.child.take().ok_or(SupervisorError::InvalidState)?;
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        child.kill()?;
        Ok(child.wait()?)
    }

    pub fn authenticate(mut self) -> Result<WindowsAuthenticatedWorker, SupervisorError> {
        if Instant::now() >= self.deadline {
            return Err(SupervisorError::DeadlineExpired);
        }
        let (control, control_identity) = self.control.authenticate(self.deadline)?;
        let (progress, progress_identity) = self.progress.authenticate(self.deadline)?;
        if control_identity.instance_id() != self.generation
            || progress_identity.instance_id() != self.generation
            || control_identity.role() != self.role
            || progress_identity.role() != self.role
            || control_identity.channel() != Some(WorkerChannel::Control)
            || progress_identity.channel() != Some(WorkerChannel::Progress)
        {
            return Err(SupervisorError::Protocol);
        }
        Ok(WindowsAuthenticatedWorker {
            child: self.child.take(),
            control: Some(control),
            progress: Some(progress),
            control_identity,
            progress_identity,
        })
    }
}

impl Drop for WindowsPendingWorker {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[derive(Debug)]
pub struct WindowsAuthenticatedWorker {
    child: Option<Child>,
    control: Option<File>,
    progress: Option<File>,
    control_identity: WorkerIdentity,
    progress_identity: WorkerIdentity,
}

impl WindowsAuthenticatedWorker {
    pub fn child_id(&self) -> Result<u32, SupervisorError> {
        self.child
            .as_ref()
            .map(Child::id)
            .ok_or(SupervisorError::InvalidState)
    }

    #[must_use]
    pub const fn control_identity(&self) -> &WorkerIdentity {
        &self.control_identity
    }

    #[must_use]
    pub const fn progress_identity(&self) -> &WorkerIdentity {
        &self.progress_identity
    }

    pub fn revoke_after_auth(mut self) -> Result<ExitStatus, SupervisorError> {
        self.control = None;
        self.progress = None;
        let mut child = self.child.take().ok_or(SupervisorError::InvalidState)?;
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        child.kill()?;
        Ok(child.wait()?)
    }

    fn stop_with_grace(
        mut self,
        stop_grace: Duration,
        terminate_grace: Duration,
    ) -> Result<WindowsStopReport, SupervisorError> {
        for (grace, reason) in [
            (stop_grace, "Windows worker stop grace must be 1ms..=60s"),
            (
                terminate_grace,
                "Windows worker terminate grace must be 1ms..=60s",
            ),
        ] {
            if grace.is_zero() || grace > MAX_HANDSHAKE_TIMEOUT {
                return Err(SupervisorError::InvalidConfiguration(reason));
            }
        }
        self.control = None;
        self.progress = None;
        let mut child = self.child.take().ok_or(SupervisorError::InvalidState)?;
        if let Some(status) = wait_for_child_exit(&mut child, Instant::now() + stop_grace)? {
            return Ok(WindowsStopReport {
                status,
                escalated: false,
            });
        }
        child.kill()?;
        if let Some(status) = wait_for_child_exit(&mut child, Instant::now() + terminate_grace)? {
            return Ok(WindowsStopReport {
                status,
                escalated: true,
            });
        }
        Err(SupervisorError::DeadlineExpired)
    }

    pub fn into_parts(
        mut self,
    ) -> Result<(Child, File, File, WorkerIdentity, WorkerIdentity), SupervisorError> {
        Ok((
            self.child.take().ok_or(SupervisorError::InvalidState)?,
            self.control.take().ok_or(SupervisorError::InvalidState)?,
            self.progress.take().ok_or(SupervisorError::InvalidState)?,
            self.control_identity.clone(),
            self.progress_identity.clone(),
        ))
    }
}

impl Drop for WindowsAuthenticatedWorker {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[derive(Debug)]
pub struct WindowsStopReport {
    status: ExitStatus,
    escalated: bool,
}

impl WindowsStopReport {
    #[must_use]
    pub const fn escalated(&self) -> bool {
        self.escalated
    }

    #[must_use]
    pub fn status(&self) -> &ExitStatus {
        &self.status
    }
}

#[derive(Debug)]
pub struct WindowsRestartLifecycle {
    budget: RestartBudget,
}

impl WindowsRestartLifecycle {
    pub fn new(policy: RestartPolicy, lifetime: Duration) -> Result<Self, SupervisorError> {
        if lifetime.is_zero() || lifetime > MAX_WORKER_LIFETIME {
            return Err(SupervisorError::InvalidConfiguration(
                "Windows worker lifetime must be positive and at most one day",
            ));
        }
        let now = Instant::now();
        Ok(Self {
            budget: RestartBudget::new(policy, now + lifetime),
        })
    }

    #[must_use]
    pub fn decision(&self) -> RestartDecision {
        self.budget.decision(Instant::now())
    }

    pub fn revoke_before_auth(
        &mut self,
        worker: WindowsPendingWorker,
    ) -> Result<ExitStatus, SupervisorError> {
        let status = worker.revoke_before_auth()?;
        self.budget.failed(Instant::now());
        Ok(status)
    }

    pub fn revoke_after_auth(
        &mut self,
        worker: WindowsAuthenticatedWorker,
    ) -> Result<ExitStatus, SupervisorError> {
        let status = worker.revoke_after_auth()?;
        self.budget.failed(Instant::now());
        Ok(status)
    }

    pub fn stop_after_auth(
        &mut self,
        worker: WindowsAuthenticatedWorker,
        health: HealthPolicy,
    ) -> Result<WindowsStopReport, SupervisorError> {
        health.validate()?;
        match worker.stop_with_grace(health.stop_grace, health.terminate_grace) {
            Ok(report) => {
                if report.escalated() || !report.status().success() {
                    self.budget.failed(Instant::now());
                }
                Ok(report)
            }
            Err(error) => {
                self.budget.failed(Instant::now());
                Err(error)
            }
        }
    }

    pub fn stop_controlled_after_auth(
        &mut self,
        worker: WindowsControlledWorker,
        health: HealthPolicy,
    ) -> Result<WindowsControlledStopReport, SupervisorError> {
        health.validate()?;
        match worker.stop(health) {
            Ok(report) => {
                if report.escalated() || !report.status().success() {
                    self.budget.failed(Instant::now());
                }
                Ok(report)
            }
            Err(error) => {
                self.budget.failed(Instant::now());
                Err(error)
            }
        }
    }

    pub(crate) fn restart(
        &mut self,
        command: &mut Command,
        generation: WorkerInstanceId,
        role: WorkerRole,
        handshake_timeout: Duration,
    ) -> Result<WindowsPendingWorker, SupervisorError> {
        self.budget.consume(Instant::now())?;
        match WindowsPendingWorker::spawn(command, generation, role, handshake_timeout) {
            Ok(worker) => Ok(worker),
            Err(error) => {
                self.budget.failed(Instant::now());
                Err(error)
            }
        }
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

fn wait_for_connection(pipe: &File, deadline: Instant) -> Result<(), SupervisorError> {
    loop {
        if Instant::now() >= deadline {
            return Err(SupervisorError::DeadlineExpired);
        }
        // SAFETY: pipe owns a live synchronous nonblocking named-pipe server handle. This call
        // does not retain the handle or the null OVERLAPPED pointer.
        if unsafe { ConnectNamedPipe(pipe.as_raw_handle(), null_mut()) } != 0 {
            return Ok(());
        }
        let error = io::Error::last_os_error();
        match error.raw_os_error() {
            Some(code) if code == ERROR_PIPE_CONNECTED as i32 => return Ok(()),
            Some(code) if code == ERROR_PIPE_LISTENING as i32 => thread::sleep(RETRY_DELAY),
            _ => return Err(error.into()),
        }
    }
}

fn read_line(pipe: &mut File, deadline: Instant) -> Result<Vec<u8>, SupervisorError> {
    let mut bytes = Vec::new();
    loop {
        if Instant::now() >= deadline {
            return Err(SupervisorError::DeadlineExpired);
        }
        let mut byte = [0_u8];
        match pipe.read(&mut byte) {
            Ok(1) if byte[0] == b'\n' => return Ok(bytes),
            Ok(1) => bytes.push(byte[0]),
            Ok(_) => thread::sleep(RETRY_DELAY),
            Err(error) if error.raw_os_error() == Some(ERROR_NO_DATA as i32) => {
                thread::sleep(RETRY_DELAY);
            }
            Err(error) => return Err(error.into()),
        }
        if bytes.len() > MAX_BOOTSTRAP_BYTES {
            return Err(SupervisorError::Protocol);
        }
    }
}
