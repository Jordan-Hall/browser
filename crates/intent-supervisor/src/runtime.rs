use crate::{
    AdmissionBook, AdmissionLimits, BootstrapPacket, ChannelHello, ChannelKind, ControlMessage,
    ExecutableImage, ProcessObservation, ProgressMessage, RequestPermit, RestartBudget,
    RestartDecision, RevocationReceipt, ScheduledMessage, SchedulerLimits, SupervisorError,
    WorkQueues, WorkerConfig, WorkerLease, WorkerScope,
    observation::observe_unreaped,
    platform::ManagedChild,
    wire::{FramedSocket, ReadBudget, ReadOutcome, decode, encode_envelope, encode_event, offer},
};
use intent_contracts::{
    BoundedText, CancellationId, ContentHash, RequestId, SchemaVersion, TaskId, TraceId,
    UnixTimestampMicros, WorkerInstanceId,
};
use intent_ipc::{ControlCodec, Envelope, EnvelopeKind};
use intent_local_transport::{
    BootstrapToken, OneShotAuthenticator, PeerCredentialEvidence, PeerExpectation, WorkerHello,
    WorkerIdentity, WorkerLaunchRecord, unix_peer_credentials,
};
use nix::{
    fcntl::{OFlag, open, openat},
    sys::{
        signal::Signal,
        stat::{Mode, mkdirat},
    },
    unistd::{UnlinkatFlags, getegid, geteuid, unlinkat},
};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, Write},
    os::{
        fd::AsRawFd,
        unix::{net::UnixListener, process::ExitStatusExt},
    },
    path::{Component, Path},
    process::Stdio,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

const MAX_PENDING_REQUESTS: usize = 32;
const MAX_READ_BYTES_PER_POLL: usize = 32 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkerState {
    Starting,
    Ready,
    Draining,
    Stopped,
    Failed,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkerFailure {
    HandshakeTimeout,
    HeartbeatTimeout,
    ProgressTimeout,
    DeadlineExpired,
    ProtocolViolation,
    ControlClosed,
    OsExit,
    OsFailure,
}
#[derive(Clone, Debug)]
pub struct WorkerSnapshot {
    pub generation: WorkerInstanceId,
    pub process_id: u32,
    pub scope: WorkerScope,
    pub state: WorkerState,
    pub failure: Option<WorkerFailure>,
    pub executable_hash: ContentHash,
    pub selected_schema: Option<SchemaVersion>,
    pub cancellation_acknowledged: bool,
    pub stop_escalated: bool,
    pub exit_code: Option<i32>,
    pub exit_signal: Option<i32>,
    pub pending_requests: usize,
    pub retained_observations: usize,
    pub rejected_peers: u64,
    pub late_messages: u64,
    pub yield_acknowledged: bool,
}
#[derive(Clone, Debug)]
pub struct WorkObservation {
    pub generation: WorkerInstanceId,
    pub request_id: RequestId,
    pub sequence: u64,
}
#[derive(Debug)]
pub struct TerminalReport {
    pub snapshot: WorkerSnapshot,
    pub unresolved_requests: Vec<RequestId>,
    pub observations: Vec<WorkObservation>,
}
#[derive(Clone, Debug)]
pub struct PollReport {
    pub active_workers: usize,
    pub queued_requests: usize,
    pub discarded_queued_requests: u64,
    pub read_bytes: usize,
    pub read_byte_limit: usize,
    pub elapsed: Duration,
}
#[derive(Debug)]
struct PendingRequest {
    sequence: u64,
    expires: Instant,
    sent: bool,
}

#[derive(Debug)]
struct Namespace {
    parent: File,
    root: File,
    name: String,
}
impl Namespace {
    fn new(parent_path: &Path) -> Result<Self, SupervisorError> {
        if !parent_path.is_absolute() {
            return Err(SupervisorError::InvalidConfiguration(
                "runtime parent must be absolute",
            ));
        }
        let mut parent = File::from(
            open(
                Path::new("/"),
                OFlag::O_DIRECTORY | OFlag::O_NOFOLLOW | OFlag::O_CLOEXEC,
                Mode::empty(),
            )
            .map_err(io::Error::from)?,
        );
        let mut components = 0;
        for component in parent_path.components() {
            match component {
                Component::RootDir => {}
                Component::Normal(name) => {
                    components += 1;
                    if components > 64 {
                        return Err(SupervisorError::InvalidConfiguration(
                            "runtime path is too deep",
                        ));
                    }
                    parent = File::from(
                        openat(
                            &parent,
                            name,
                            OFlag::O_DIRECTORY | OFlag::O_NOFOLLOW | OFlag::O_CLOEXEC,
                            Mode::empty(),
                        )
                        .map_err(io::Error::from)?,
                    );
                }
                _ => {
                    return Err(SupervisorError::InvalidConfiguration(
                        "runtime parent contains traversal",
                    ));
                }
            }
        }
        let name = format!("intent-workers-{}", Uuid::new_v4());
        mkdirat(&parent, name.as_str(), Mode::S_IRWXU).map_err(io::Error::from)?;
        let result = openat(
            &parent,
            name.as_str(),
            OFlag::O_DIRECTORY | OFlag::O_NOFOLLOW | OFlag::O_CLOEXEC,
            Mode::empty(),
        )
        .map(File::from)
        .map_err(io::Error::from);
        match result {
            Ok(root) => Ok(Self { parent, root, name }),
            Err(error) => {
                let _ = unlinkat(&parent, name.as_str(), UnlinkatFlags::RemoveDir);
                Err(error.into())
            }
        }
    }
    fn endpoint(&self, name: &str, for_worker: bool) -> String {
        let process = if for_worker {
            std::process::id().to_string()
        } else {
            "self".to_owned()
        };
        format!("/proc/{process}/fd/{}/{name}", self.root.as_raw_fd())
    }
    fn listener(&self, name: &str) -> io::Result<UnixListener> {
        let listener = UnixListener::bind(self.endpoint(name, false))?;
        listener.set_nonblocking(true)?;
        Ok(listener)
    }
    fn remove(&self, name: &str) {
        let _ = unlinkat(&self.root, name, UnlinkatFlags::NoRemoveDir);
    }
}
impl Drop for Namespace {
    fn drop(&mut self) {
        let _ = unlinkat(&self.parent, self.name.as_str(), UnlinkatFlags::RemoveDir);
    }
}

#[derive(Debug)]
struct Lane {
    name: String,
    listener: UnixListener,
    socket: Option<FramedSocket>,
    authenticator: Option<OneShotAuthenticator>,
    identity: Option<WorkerIdentity>,
    codec: Option<ControlCodec>,
}
impl Lane {
    fn poll_authentication(
        &mut self,
        pid: u32,
        channel: ChannelKind,
        generation: WorkerInstanceId,
        rejected: &mut u64,
        read_budget: &mut ReadBudget,
    ) -> Result<(), SupervisorError> {
        if self.socket.is_none() {
            for _ in 0..4 {
                let stream = match self.listener.accept() {
                    Ok((stream, _)) => stream,
                    Err(error)
                        if matches!(
                            error.kind(),
                            io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                        ) =>
                    {
                        return Ok(());
                    }
                    Err(error) => return Err(error.into()),
                };
                let peer = unix_peer_credentials(&stream).map_err(|_| SupervisorError::Protocol)?;
                if peer != expected_peer(pid) {
                    *rejected = rejected.saturating_add(1);
                    continue;
                }
                self.socket = Some(FramedSocket::new(stream)?);
                break;
            }
        }
        if self.identity.is_some() {
            return Ok(());
        }
        let Some(socket) = self.socket.as_mut() else {
            return Ok(());
        };
        match read_budget.read_one(socket, 4096)? {
            ReadOutcome::Pending => Ok(()),
            ReadOutcome::Closed => Err(SupervisorError::Protocol),
            ReadOutcome::Frame(frame) => {
                let envelope: Envelope<ControlMessage> = decode(&frame, None)?;
                if envelope.message() != EnvelopeKind::Event {
                    return Err(SupervisorError::Protocol);
                }
                let ControlMessage::Hello(hello) = envelope.into_payload() else {
                    return Err(SupervisorError::Protocol);
                };
                if hello.channel != channel {
                    return Err(SupervisorError::Protocol);
                }
                let codec =
                    ControlCodec::negotiate(&hello.offer).map_err(|_| SupervisorError::Protocol)?;
                let auth = self.authenticator.take().ok_or(SupervisorError::Protocol)?;
                let peer =
                    unix_peer_credentials(&socket.stream).map_err(|_| SupervisorError::Protocol)?;
                let identity = auth
                    .authenticate(&hello.identity, peer)
                    .map_err(|_| SupervisorError::Protocol)?;
                socket.queue(encode_event(
                    ControlMessage::Welcome {
                        generation,
                        selected: SchemaVersion::V1,
                    },
                    Some(&codec),
                )?)?;
                socket.flush(4096)?;
                self.identity = Some(identity);
                self.codec = Some(codec);
                Ok(())
            }
        }
    }
}
fn expected_peer(pid: u32) -> PeerCredentialEvidence {
    PeerCredentialEvidence::Unix {
        pid: Some(pid),
        uid: geteuid().as_raw(),
        gid: getegid().as_raw(),
    }
}

#[derive(Debug)]
struct Entry {
    generation: WorkerInstanceId,
    child: ManagedChild,
    image: ExecutableImage,
    config: WorkerConfig,
    args: Vec<String>,
    restart: RestartBudget,
    lease: WorkerLease,
    control: Lane,
    progress: Lane,
    state: WorkerState,
    failure: Option<WorkerFailure>,
    started: Instant,
    heartbeat: Instant,
    progress_at: Instant,
    last_heartbeat: u64,
    last_progress: u64,
    stop_at: Option<Instant>,
    cancel_id: Option<CancellationId>,
    cancel_sent: bool,
    cancel_ack: bool,
    term_sent: bool,
    kill_sent: bool,
    reaped: bool,
    exit_code: Option<i32>,
    exit_signal: Option<i32>,
    pending: BTreeMap<RequestId, PendingRequest>,
    observations: BTreeMap<RequestId, WorkObservation>,
    rejected_peers: u64,
    late_messages: u64,
    yield_requested: Option<Instant>,
    yield_sent: bool,
    yield_ack: bool,
    observation: Option<ProcessObservation>,
    sampled_at: Instant,
}
impl Entry {
    fn snapshot(&self) -> WorkerSnapshot {
        WorkerSnapshot {
            generation: self.generation,
            process_id: self.child.id(),
            scope: self.config.scope,
            state: self.state,
            failure: self.failure,
            executable_hash: self.image.hash(),
            selected_schema: self.control.codec.as_ref().map(|_| SchemaVersion::V1),
            cancellation_acknowledged: self.cancel_ack,
            stop_escalated: self.term_sent || self.kill_sent,
            exit_code: self.exit_code,
            exit_signal: self.exit_signal,
            pending_requests: self.pending.len(),
            retained_observations: self.observations.len(),
            rejected_peers: self.rejected_peers,
            late_messages: self.late_messages,
            yield_acknowledged: self.yield_ack,
        }
    }
    fn stop(&mut self, now: Instant, cancellation: CancellationId, failure: Option<WorkerFailure>) {
        self.lease.revoke();
        if self.reaped || self.stop_at.is_some() {
            return;
        }
        self.failure = failure;
        self.state = WorkerState::Draining;
        self.stop_at = Some(now);
        self.cancel_id = Some(cancellation);
        if let Some(socket) = self.control.socket.as_mut() {
            socket.discard_unstarted();
        }
    }
    fn fail(&mut self, now: Instant, failure: WorkerFailure) {
        self.stop(
            now,
            CancellationId::from_uuid(Uuid::new_v4()),
            Some(failure),
        );
    }
    fn poll(
        &mut self,
        now: Instant,
        read_budget: &mut ReadBudget,
    ) -> Result<bool, SupervisorError> {
        if self.reaped {
            return Ok(false);
        }
        if let Some(exit) = self.child.poll_exit()? {
            self.exit_code = exit.code();
            self.exit_signal = exit.signal();
            self.lease.revoke();
            self.reaped = true;
            if self.stop_at.is_none() {
                self.failure = Some(WorkerFailure::OsExit);
            }
            self.state = if self.failure.is_some() {
                WorkerState::Failed
            } else {
                WorkerState::Stopped
            };
            if self.state == WorkerState::Failed {
                self.restart.failed(now);
            }
            return Ok(true);
        }
        if self.lease.expired(now) || self.pending.values().any(|request| now >= request.expires) {
            self.fail(now, WorkerFailure::DeadlineExpired);
        }
        if self.lease.is_revoked() && self.stop_at.is_none() {
            self.stop(now, CancellationId::from_uuid(Uuid::new_v4()), None);
        }
        match self.state {
            WorkerState::Starting
                if now.duration_since(self.started) >= self.config.health.handshake_timeout =>
            {
                self.fail(now, WorkerFailure::HandshakeTimeout)
            }
            WorkerState::Ready
                if now.duration_since(self.heartbeat) >= self.config.health.heartbeat_timeout =>
            {
                self.fail(now, WorkerFailure::HeartbeatTimeout)
            }
            WorkerState::Ready
                if self.pending.values().any(|r| r.sent)
                    && now.duration_since(self.progress_at)
                        >= self.config.health.work_progress_timeout =>
            {
                self.fail(now, WorkerFailure::ProgressTimeout)
            }
            _ => {}
        }
        if self.state == WorkerState::Starting {
            self.control.poll_authentication(
                self.child.id(),
                ChannelKind::Control,
                self.generation,
                &mut self.rejected_peers,
                read_budget,
            )?;
            self.progress.poll_authentication(
                self.child.id(),
                ChannelKind::Progress,
                self.generation,
                &mut self.rejected_peers,
                read_budget,
            )?;
        }
        if self.control.identity.is_some() {
            self.read_control(now, read_budget)?;
        }
        if self.progress.identity.is_some() {
            self.read_progress(now, read_budget)?;
        }
        if let Some(stop) = self.stop_at {
            if let Some(socket) = self.control.socket.as_mut()
                && !self.cancel_sent
                && socket.idle()
                && self.control.codec.is_some()
            {
                let cancellation = self.cancel_id.ok_or(SupervisorError::InvalidState)?;
                let envelope = Envelope::event(
                    TraceId::from_uuid(Uuid::new_v4()),
                    ControlMessage::Cancel {
                        generation: self.generation,
                        cancellation_id: cancellation,
                    },
                )
                .with_cancellation_id(cancellation);
                socket.queue(encode_envelope(&envelope, self.control.codec.as_ref())?)?;
                self.cancel_sent = true;
            }
            if now.duration_since(stop) >= self.config.health.stop_grace && !self.term_sent {
                self.child.signal_group(Signal::SIGTERM)?;
                self.term_sent = true;
            }
            if now.duration_since(stop)
                >= self.config.health.stop_grace + self.config.health.terminate_grace
                && !self.kill_sent
            {
                self.child.signal_group(Signal::SIGKILL)?;
                self.kill_sent = true;
            }
        } else if self.state == WorkerState::Ready
            && self.yield_requested.is_some()
            && !self.yield_sent
            && let Some(socket) = self.control.socket.as_mut()
            && socket.idle()
        {
            socket.queue(encode_event(
                ControlMessage::Yield {
                    generation: self.generation,
                },
                self.control.codec.as_ref(),
            )?)?;
            self.yield_sent = true;
        }
        if let Some(socket) = self.control.socket.as_mut()
            && let Err(error) = socket.flush(4096)
            && self.stop_at.is_none()
        {
            return Err(error.into());
        }
        if let Some(socket) = self.progress.socket.as_mut()
            && let Err(error) = socket.flush(4096)
            && self.stop_at.is_none()
        {
            return Err(error.into());
        }
        Ok(false)
    }
    fn read_control(
        &mut self,
        now: Instant,
        read_budget: &mut ReadBudget,
    ) -> Result<(), SupervisorError> {
        let Some(socket) = self.control.socket.as_mut() else {
            return Ok(());
        };
        let mut lane_budget = 8192;
        for _ in 0..8 {
            let before = read_budget.consumed();
            let outcome = read_budget.read_one(socket, lane_budget)?;
            lane_budget = lane_budget
                .saturating_sub(read_budget.consumed().saturating_sub(before));
            let frame = match outcome {
                ReadOutcome::Pending => break,
                ReadOutcome::Closed => {
                    if self.stop_at.is_none() {
                        self.fail(now, WorkerFailure::ControlClosed);
                    }
                    break;
                }
                ReadOutcome::Frame(frame) => frame,
            };
            let envelope: Envelope<ControlMessage> = decode(&frame, self.control.codec.as_ref())?;
            let kind = envelope.message();
            let cancellation = envelope.cancellation_id();
            match envelope.into_payload() {
                ControlMessage::Ready { generation }
                    if generation == self.generation
                        && self.state == WorkerState::Starting
                        && self.progress.identity.is_some()
                        && kind == EnvelopeKind::Event =>
                {
                    self.state = WorkerState::Ready;
                    self.heartbeat = now;
                    self.progress_at = now;
                }
                ControlMessage::Heartbeat {
                    generation,
                    sequence,
                } if generation == self.generation && kind == EnvelopeKind::Event => {
                    if sequence <= self.last_heartbeat || self.state == WorkerState::Starting {
                        return Err(SupervisorError::Protocol);
                    }
                    self.last_heartbeat = sequence;
                    self.heartbeat = now;
                }
                ControlMessage::Observed {
                    generation,
                    request_id,
                    sequence,
                } if generation == self.generation
                    && kind == (EnvelopeKind::Response { request_id }) =>
                {
                    if self.lease.is_revoked() {
                        self.late_messages = self.late_messages.saturating_add(1);
                        continue;
                    }
                    let pending = self
                        .pending
                        .get(&request_id)
                        .ok_or(SupervisorError::Protocol)?;
                    if !pending.sent || pending.sequence != sequence || now >= pending.expires {
                        return Err(SupervisorError::Protocol);
                    }
                    self.pending.remove(&request_id);
                    self.observations.insert(
                        request_id,
                        WorkObservation {
                            generation,
                            request_id,
                            sequence,
                        },
                    );
                    self.progress_at = now;
                }
                ControlMessage::Cancelled {
                    generation,
                    cancellation_id,
                } if generation == self.generation
                    && Some(cancellation_id) == self.cancel_id
                    && cancellation == Some(cancellation_id)
                    && kind == EnvelopeKind::Event =>
                {
                    self.cancel_ack = true
                }
                ControlMessage::Yielded { generation }
                    if generation == self.generation
                        && self.yield_sent
                        && kind == EnvelopeKind::Event =>
                {
                    self.yield_ack = true
                }
                _ => return Err(SupervisorError::Protocol),
            }
        }
        Ok(())
    }
    fn read_progress(
        &mut self,
        now: Instant,
        read_budget: &mut ReadBudget,
    ) -> Result<(), SupervisorError> {
        let Some(socket) = self.progress.socket.as_mut() else {
            return Ok(());
        };
        let mut lane_budget = 4096;
        for _ in 0..4 {
            let before = read_budget.consumed();
            let outcome = read_budget.read_one(socket, lane_budget)?;
            lane_budget = lane_budget
                .saturating_sub(read_budget.consumed().saturating_sub(before));
            let frame = match outcome {
                ReadOutcome::Pending | ReadOutcome::Closed => break,
                ReadOutcome::Frame(frame) => frame,
            };
            let envelope: Envelope<ProgressMessage> = decode(&frame, self.progress.codec.as_ref())?;
            if envelope.message() != EnvelopeKind::Event {
                return Err(SupervisorError::Protocol);
            }
            let progress = envelope.into_payload();
            if progress.generation != self.generation || progress.sequence <= self.last_progress {
                return Err(SupervisorError::Protocol);
            }
            self.last_progress = progress.sequence;
            if self.lease.is_revoked() {
                self.late_messages = self.late_messages.saturating_add(1);
            } else if self
                .pending
                .values()
                .any(|request| request.sent && request.sequence == progress.work_sequence)
            {
                self.progress_at = now;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Supervisor {
    entries: BTreeMap<WorkerInstanceId, Entry>,
    admission: AdmissionBook,
    queues: WorkQueues,
    namespace: Namespace,
    max_entries: usize,
    read_cursor: usize,
    metrics_cursor: usize,
}
impl Supervisor {
    pub fn new(
        runtime_parent: &Path,
        limits: AdmissionLimits,
        scheduler: SchedulerLimits,
    ) -> Result<Self, SupervisorError> {
        Ok(Self {
            entries: BTreeMap::new(),
            admission: AdmissionBook::new(limits),
            queues: WorkQueues::new(scheduler),
            namespace: Namespace::new(runtime_parent)?,
            max_entries: limits.workers() * 2,
            read_cursor: 0,
            metrics_cursor: 0,
        })
    }
    /// Executable verification and spawn belong on the launch executor, not a UI event callback.
    pub fn launch(
        &mut self,
        image: ExecutableImage,
        config: WorkerConfig,
        args: &[String],
    ) -> Result<WorkerInstanceId, SupervisorError> {
        let now = Instant::now();
        let restart = RestartBudget::new(config.restart, now + config.lifetime);
        self.launch_inner(image, config, args, restart)
    }
    fn launch_inner(
        &mut self,
        image: ExecutableImage,
        config: WorkerConfig,
        args: &[String],
        restart: RestartBudget,
    ) -> Result<WorkerInstanceId, SupervisorError> {
        if self.entries.len() >= self.max_entries {
            return Err(SupervisorError::AdmissionExhausted);
        }
        if args.len() > 16
            || args
                .iter()
                .any(|arg| arg.len() > 256 || arg.as_bytes().contains(&0))
        {
            return Err(SupervisorError::InvalidConfiguration(
                "unbounded launch arguments",
            ));
        }
        let generation = WorkerInstanceId::from_uuid(Uuid::new_v4());
        self.admission
            .acquire(generation, config.priority, config.limits)?;
        let control_name = format!("c-{generation}");
        let progress_name = format!("p-{generation}");
        let created = (|| -> Result<Entry, SupervisorError> {
            let control_listener = self.namespace.listener(&control_name)?;
            let progress_listener = self.namespace.listener(&progress_name)?;
            let control_token = BootstrapToken::generate()
                .map_err(|_| SupervisorError::InvalidConfiguration("OS randomness unavailable"))?;
            let progress_token = BootstrapToken::generate()
                .map_err(|_| SupervisorError::InvalidConfiguration("OS randomness unavailable"))?;
            let packet = BootstrapPacket {
                control_endpoint: BoundedText::try_new(
                    self.namespace.endpoint(&control_name, true),
                )
                .map_err(|_| SupervisorError::InvalidConfiguration("socket endpoint too long"))?,
                progress_endpoint: BoundedText::try_new(
                    self.namespace.endpoint(&progress_name, true),
                )
                .map_err(|_| SupervisorError::InvalidConfiguration("socket endpoint too long"))?,
                control: ChannelHello {
                    channel: ChannelKind::Control,
                    identity: WorkerHello::new(generation, config.role, control_token.clone()),
                    offer: offer()?,
                },
                progress: ChannelHello {
                    channel: ChannelKind::Progress,
                    identity: WorkerHello::new(generation, config.role, progress_token.clone()),
                    offer: offer()?,
                },
                scope: config.scope,
                handshake_timeout_millis: u32::try_from(
                    config.health.handshake_timeout.as_millis(),
                )
                .map_err(|_| SupervisorError::InvalidConfiguration("handshake timeout overflow"))?,
            };
            let bootstrap = encode_event(packet, None)?;
            if bootstrap.len() > 4096 {
                return Err(SupervisorError::InvalidConfiguration(
                    "bootstrap exceeds one pipe budget",
                ));
            }
            let mut raw = image
                .command(config.limits)
                .args(args)
                .current_dir("/")
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;
            let stdin = raw.stdin.take();
            let child = ManagedChild::new(raw);
            stdin
                .ok_or(SupervisorError::InvalidState)?
                .write_all(&bootstrap)?;
            let expectation = PeerExpectation::exact(expected_peer(child.id()));
            let lane = |name, listener, token| Lane {
                name,
                listener,
                socket: None,
                authenticator: Some(OneShotAuthenticator::new(WorkerLaunchRecord::new(
                    generation,
                    config.role,
                    token,
                    expectation,
                ))),
                identity: None,
                codec: None,
            };
            let control = lane(control_name.clone(), control_listener, control_token);
            let progress = lane(progress_name.clone(), progress_listener, progress_token);
            let lease = WorkerLease::new(
                generation,
                config.scope,
                config.role,
                config.capabilities.clone(),
                restart.lifetime_end(),
            );
            let now = Instant::now();
            Ok(Entry {
                generation,
                child,
                image,
                config,
                args: args.to_vec(),
                restart,
                lease,
                control,
                progress,
                state: WorkerState::Starting,
                failure: None,
                started: now,
                heartbeat: now,
                progress_at: now,
                last_heartbeat: 0,
                last_progress: 0,
                stop_at: None,
                cancel_id: None,
                cancel_sent: false,
                cancel_ack: false,
                term_sent: false,
                kill_sent: false,
                reaped: false,
                exit_code: None,
                exit_signal: None,
                pending: BTreeMap::new(),
                observations: BTreeMap::new(),
                rejected_peers: 0,
                late_messages: 0,
                yield_requested: None,
                yield_sent: false,
                yield_ack: false,
                observation: None,
                sampled_at: now,
            })
        })();
        match created {
            Ok(entry) => {
                self.entries.insert(generation, entry);
                Ok(generation)
            }
            Err(error) => {
                self.admission.release(generation);
                self.namespace.remove(&control_name);
                self.namespace.remove(&progress_name);
                Err(error)
            }
        }
    }
    pub fn lease(&self, generation: WorkerInstanceId) -> Result<WorkerLease, SupervisorError> {
        let entry = self
            .entries
            .get(&generation)
            .ok_or(SupervisorError::UnknownWorker)?;
        if entry.state != WorkerState::Ready || entry.lease.is_revoked() {
            return Err(SupervisorError::InvalidState);
        }
        Ok(entry.lease.clone())
    }
    pub fn snapshot(
        &self,
        generation: WorkerInstanceId,
    ) -> Result<WorkerSnapshot, SupervisorError> {
        Ok(self
            .entries
            .get(&generation)
            .ok_or(SupervisorError::UnknownWorker)?
            .snapshot())
    }
    pub fn submit(
        &mut self,
        permit: RequestPermit,
        input: BoundedText<4096>,
    ) -> Result<(), SupervisorError> {
        self.submit_inner(permit, input, false)
    }
    /// Checks capacity without consuming the permit or starting a durable attempt.
    pub fn can_submit_immediate(&self, permit: &RequestPermit) -> Result<(), SupervisorError> {
        let entry = self
            .entries
            .get(&permit.generation())
            .ok_or(SupervisorError::UnknownWorker)?;
        permit.validate(&entry.lease, Instant::now())?;
        if entry.state != WorkerState::Ready {
            return Err(SupervisorError::InvalidState);
        }
        if entry.pending.len() + entry.observations.len() >= MAX_PENDING_REQUESTS
            || entry.pending.contains_key(&permit.request_id)
            || entry.observations.contains_key(&permit.request_id)
            || !entry
                .control
                .socket
                .as_ref()
                .is_some_and(FramedSocket::idle)
        {
            return Err(SupervisorError::QueueFull);
        }
        Ok(())
    }
    /// Starts a bounded socket write now, without entering the best-effort work queue.
    /// An error can follow a partial write; callers must retain the durable attempt as uncertain.
    pub fn submit_immediate(
        &mut self,
        permit: RequestPermit,
        input: BoundedText<4096>,
    ) -> Result<(), SupervisorError> {
        self.can_submit_immediate(&permit)?;
        self.submit_inner(permit, input, true)
    }
    fn submit_inner(
        &mut self,
        permit: RequestPermit,
        input: BoundedText<4096>,
        immediate: bool,
    ) -> Result<(), SupervisorError> {
        let now = Instant::now();
        let entry = self
            .entries
            .get_mut(&permit.generation())
            .ok_or(SupervisorError::UnknownWorker)?;
        if entry.state != WorkerState::Ready {
            return Err(SupervisorError::InvalidState);
        }
        permit.validate(&entry.lease, now)?;
        if entry.pending.len() + entry.observations.len() >= MAX_PENDING_REQUESTS
            || entry.pending.contains_key(&permit.request_id)
            || entry.observations.contains_key(&permit.request_id)
        {
            return Err(SupervisorError::QueueFull);
        }
        let ttl = permit.expires.saturating_duration_since(now);
        let wall = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| SupervisorError::DeadlineExpired)?
            .checked_add(ttl)
            .ok_or(SupervisorError::DeadlineExpired)?;
        let deadline = UnixTimestampMicros::try_new(
            i64::try_from(wall.as_micros()).map_err(|_| SupervisorError::DeadlineExpired)?,
        )
        .map_err(|_| SupervisorError::DeadlineExpired)?;
        let request_id = permit.request_id;
        let pending = PendingRequest {
            sequence: permit.sequence(),
            expires: permit.expires,
            sent: immediate,
        };
        let envelope = Envelope::request(
            TraceId::from_uuid(Uuid::new_v4()),
            request_id,
            ControlMessage::Execute {
                generation: entry.generation,
                request_id,
                sequence: permit.sequence(),
                scope: entry.config.scope,
                capability: permit.capability,
                family: permit.family,
                deadline,
                input,
            },
        )
        .with_deadline(deadline)
        .with_cancellation_id(CancellationId::from_uuid(entry.generation.as_uuid()));
        let bytes = encode_envelope(&envelope, entry.control.codec.as_ref())?;
        if immediate {
            let socket = entry
                .control
                .socket
                .as_mut()
                .ok_or(SupervisorError::InvalidState)?;
            socket.queue(bytes)?;
            entry.pending.insert(request_id, pending);
            entry.progress_at = now;
            if let Err(error) = socket.flush(4096) {
                entry.fail(now, WorkerFailure::OsFailure);
                return Err(error.into());
            }
            return Ok(());
        }
        self.queues.enqueue(
            entry.config.priority,
            ScheduledMessage {
                permit,
                bytes,
                enqueued: now,
            },
        )?;
        entry.pending.insert(request_id, pending);
        Ok(())
    }
    pub fn cancel(
        &mut self,
        generation: WorkerInstanceId,
        cancellation: CancellationId,
    ) -> Result<RevocationReceipt, SupervisorError> {
        let entry = self
            .entries
            .get_mut(&generation)
            .ok_or(SupervisorError::UnknownWorker)?;
        let receipt = entry.lease.revoke();
        entry.stop(Instant::now(), cancellation, None);
        self.queues.remove_generation(generation);
        Ok(receipt)
    }
    pub fn cancel_task(
        &mut self,
        task: TaskId,
        cancellation: CancellationId,
    ) -> Vec<RevocationReceipt> {
        let ids: Vec<_> = self
            .entries
            .iter()
            .filter(|(_, e)| e.config.scope.task_id == task)
            .map(|(id, _)| *id)
            .collect();
        ids.into_iter()
            .filter_map(|id| self.cancel(id, cancellation).ok())
            .collect()
    }
    pub fn request_yield(&mut self, generation: WorkerInstanceId) -> Result<bool, SupervisorError> {
        let entry = self
            .entries
            .get_mut(&generation)
            .ok_or(SupervisorError::UnknownWorker)?;
        let now = Instant::now();
        if entry.state != WorkerState::Ready || entry.lease.is_revoked() {
            return Err(SupervisorError::InvalidState);
        }
        if entry
            .yield_requested
            .is_some_and(|last| now.duration_since(last) < Duration::from_millis(500))
        {
            return Ok(false);
        }
        entry.yield_requested = Some(now);
        entry.yield_sent = false;
        entry.yield_ack = false;
        Ok(true)
    }
    pub fn observation(&self, generation: WorkerInstanceId) -> Option<&ProcessObservation> {
        self.entries
            .get(&generation)?
            .observation
            .as_ref()
            .filter(|sample| sample.sampled_at.elapsed() <= Duration::from_secs(1))
    }
    pub fn take_result(
        &mut self,
        generation: WorkerInstanceId,
        request: RequestId,
    ) -> Result<Option<WorkObservation>, SupervisorError> {
        Ok(self
            .entries
            .get_mut(&generation)
            .ok_or(SupervisorError::UnknownWorker)?
            .observations
            .remove(&request))
    }
    pub fn poll(&mut self) -> PollReport {
        let started = Instant::now();
        let now = started;
        let mut read_budget = ReadBudget::new(MAX_READ_BYTES_PER_POLL);
        let generations: Vec<_> = self.entries.keys().copied().collect();
        let read_start = if generations.is_empty() {
            0
        } else {
            self.read_cursor % generations.len()
        };
        self.read_cursor = self.read_cursor.wrapping_add(1);
        for offset in 0..generations.len() {
            let id = generations[(read_start + offset) % generations.len()];
            let Some(entry) = self.entries.get_mut(&id) else {
                continue;
            };
            match entry.poll(now, &mut read_budget) {
                Ok(true) => {
                    self.admission.release(id);
                    self.queues.remove_generation(id);
                }
                Ok(false) => {}
                Err(error) => {
                    entry.fail(
                        now,
                        if matches!(error, SupervisorError::Protocol) {
                            WorkerFailure::ProtocolViolation
                        } else {
                            WorkerFailure::OsFailure
                        },
                    );
                    // I/O errors must not prevent timed escalation on later polls.
                    if let Some(stop) = entry.stop_at
                        && now.duration_since(stop)
                            >= entry.config.health.stop_grace + entry.config.health.terminate_grace
                    {
                        let _ = entry.child.signal_group(Signal::SIGKILL);
                        entry.kill_sent = true;
                    }
                    self.queues.remove_generation(id);
                }
            }
        }
        for _ in 0..8 {
            let entries = &self.entries;
            let message = self.queues.pop_ready(now, |id| {
                entries.get(&id).is_some_and(|e| {
                    e.state == WorkerState::Ready
                        && !e.lease.is_revoked()
                        && e.control.socket.as_ref().is_some_and(FramedSocket::idle)
                })
            });
            let Some(message) = message else {
                break;
            };
            if let Some(entry) = self.entries.get_mut(&message.permit.generation()) {
                if message
                    .permit
                    .validate(&entry.lease, Instant::now())
                    .is_err()
                {
                    continue;
                }
                if let Some(socket) = entry.control.socket.as_mut()
                    && socket.queue(message.bytes).is_ok()
                {
                    if let Some(pending) = entry.pending.get_mut(&message.permit.request_id) {
                        pending.sent = true;
                    }
                    entry.progress_at = now;
                    if socket.flush(4096).is_err() {
                        entry.fail(now, WorkerFailure::OsFailure);
                    }
                }
            }
        }
        // At most one bounded /proc read per poll; unavailable observations stay explicit.
        if !self.entries.is_empty() {
            let index = self.metrics_cursor % self.entries.len();
            self.metrics_cursor = self.metrics_cursor.wrapping_add(1);
            if let Some((_, entry)) = self.entries.iter_mut().nth(index)
                && !entry.reaped
                && now.duration_since(entry.sampled_at) >= Duration::from_millis(250)
            {
                entry.observation = observe_unreaped(entry.child.id()).ok();
                entry.sampled_at = now;
            }
        }
        PollReport {
            active_workers: self.admission.active_count(),
            queued_requests: self.queues.len(),
            discarded_queued_requests: self.queues.dropped,
            read_bytes: read_budget.consumed(),
            read_byte_limit: MAX_READ_BYTES_PER_POLL,
            elapsed: started.elapsed(),
        }
    }
    pub fn restart_decision(
        &self,
        generation: WorkerInstanceId,
    ) -> Result<RestartDecision, SupervisorError> {
        Ok(self
            .entries
            .get(&generation)
            .ok_or(SupervisorError::UnknownWorker)?
            .restart
            .decision(Instant::now()))
    }
    /// No outstanding work is replayed. Callers must persist/consume the terminal report first.
    pub fn restart(
        &mut self,
        generation: WorkerInstanceId,
    ) -> Result<WorkerInstanceId, SupervisorError> {
        let entry = self
            .entries
            .get_mut(&generation)
            .ok_or(SupervisorError::UnknownWorker)?;
        if !entry.reaped
            || entry.state != WorkerState::Failed
            || !entry.pending.is_empty()
            || !entry.observations.is_empty()
        {
            return Err(SupervisorError::RestartDenied);
        }
        entry.restart.consume(Instant::now())?;
        let image = entry.image.clone();
        let config = entry.config.clone();
        let args = entry.args.clone();
        let restart = entry.restart.clone();
        match self.launch_inner(image, config, &args, restart) {
            Ok(id) => {
                self.retire(generation)?;
                Ok(id)
            }
            Err(error) => {
                if let Some(entry) = self.entries.get_mut(&generation) {
                    entry.restart.failed(Instant::now());
                }
                Err(error)
            }
        }
    }
    pub fn retire(
        &mut self,
        generation: WorkerInstanceId,
    ) -> Result<TerminalReport, SupervisorError> {
        let entry = self
            .entries
            .get(&generation)
            .ok_or(SupervisorError::UnknownWorker)?;
        if !entry.reaped {
            return Err(SupervisorError::InvalidState);
        }
        let entry = self
            .entries
            .remove(&generation)
            .ok_or(SupervisorError::UnknownWorker)?;
        entry.lease.revoke();
        self.queues.remove_generation(generation);
        self.namespace.remove(&entry.control.name);
        self.namespace.remove(&entry.progress.name);
        Ok(TerminalReport {
            snapshot: entry.snapshot(),
            unresolved_requests: entry.pending.keys().copied().collect(),
            observations: entry.observations.into_values().collect(),
        })
    }
}
impl Drop for Supervisor {
    fn drop(&mut self) {
        for entry in self.entries.values() {
            entry.lease.revoke();
            let _ = entry.child.signal_group(Signal::SIGKILL);
            self.namespace.remove(&entry.control.name);
            self.namespace.remove(&entry.progress.name);
        }
        self.entries.clear();
    }
}
