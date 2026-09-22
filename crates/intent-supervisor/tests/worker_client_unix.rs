#![cfg(any(target_os = "linux", target_vendor = "apple"))]

use intent_contracts::{AccountId, BoundedText, SchemaVersion, TaskId, TraceId, WorkerInstanceId};
use intent_ipc::{
    ControlCodec, Envelope, Frame, FrameLane, ProtocolOffer, ProtocolRange, decode_control_owned,
};
use intent_local_transport::{
    AuthenticationError, ExpectedPeer, WorkerChannel, WorkerHello, WorkerRole,
    issue_worker_channel_authentication,
};
use intent_supervisor::{
    BootstrapPacket, ChannelHello, ChannelKind, ControlMessage, WorkerScope, wire_limits,
    worker::WorkerClient,
};
use nix::unistd::{getegid, geteuid};
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use uuid::Uuid;

const IO_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_BOOTSTRAP_BYTES: u64 = 16 * 1024;

type TestResult = Result<(), Box<dyn Error>>;

struct EndpointPair {
    directory: PathBuf,
    control: PathBuf,
    progress: PathBuf,
}

impl EndpointPair {
    fn new() -> Result<Self, Box<dyn Error>> {
        let directory = PathBuf::from("/tmp").join(format!(
            "intent-worker-client-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        fs::create_dir(&directory)?;
        fs::set_permissions(&directory, fs::Permissions::from_mode(0o700))?;
        Ok(Self {
            control: directory.join("control"),
            progress: directory.join("progress"),
            directory,
        })
    }
}

impl Drop for EndpointPair {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.control);
        let _ = fs::remove_file(&self.progress);
        let _ = fs::remove_dir(&self.directory);
    }
}

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn offer() -> Result<ProtocolOffer, Box<dyn Error>> {
    Ok(ProtocolOffer::try_new(
        vec![ProtocolRange::try_new(1, 0, 0)?],
        [],
    )?)
}

fn endpoint_text(path: &std::path::Path) -> Result<BoundedText<108>, Box<dyn Error>> {
    Ok(BoundedText::try_new(
        path.to_str().ok_or("fixture endpoint is not UTF-8")?,
    )?)
}

fn read_frame(stream: &mut UnixStream) -> Result<Frame, Box<dyn Error>> {
    let mut header = [0_u8; 5];
    stream.read_exact(&mut header)?;
    if header[0] != 1 {
        return Err("fixture received a non-control frame".into());
    }
    let length = u32::from_be_bytes([header[1], header[2], header[3], header[4]]) as usize;
    if length > wire_limits().max_control_frame_bytes {
        return Err("fixture received an oversized frame".into());
    }
    let mut payload = vec![0; length];
    stream.read_exact(&mut payload)?;
    Ok(Frame::new(FrameLane::Control, payload))
}

fn read_bootstrap_hello(stream: &mut UnixStream) -> Result<ChannelHello, Box<dyn Error>> {
    let envelope: Envelope<ControlMessage> =
        decode_control_owned(read_frame(stream)?, wire_limits())?;
    match envelope.into_payload() {
        ControlMessage::Hello(hello) => Ok(hello),
        _ => Err("fixture expected bootstrap Hello".into()),
    }
}

fn write_welcome(
    stream: &mut UnixStream,
    codec: &ControlCodec,
    generation: WorkerInstanceId,
) -> Result<(), Box<dyn Error>> {
    let envelope = Envelope::event(
        TraceId::from_uuid(Uuid::new_v4()),
        ControlMessage::Welcome {
            generation,
            selected: SchemaVersion::V1,
        },
    );
    let bytes = codec
        .encode(&envelope, wire_limits())?
        .encode(wire_limits())?;
    stream.write_all(&bytes)?;
    Ok(())
}

#[test]
#[ignore = "launched by worker_client_authenticates_real_child_on_both_unix_lanes"]
fn worker_client_child() -> TestResult {
    let mut bootstrap = Vec::new();
    std::io::stdin()
        .take(MAX_BOOTSTRAP_BYTES + 1)
        .read_to_end(&mut bootstrap)?;
    if bootstrap.len() as u64 > MAX_BOOTSTRAP_BYTES {
        return Err("oversized worker bootstrap".into());
    }
    let packet: BootstrapPacket = serde_json::from_slice(&bootstrap)?;
    let generation = packet.control.identity.instance_id();
    let client = WorkerClient::connect(packet)?;
    assert_eq!(client.generation(), generation);
    Ok(())
}

#[test]
fn worker_client_authenticates_real_child_on_both_unix_lanes() -> TestResult {
    let endpoints = EndpointPair::new()?;
    let control_listener = UnixListener::bind(&endpoints.control)?;
    let progress_listener = UnixListener::bind(&endpoints.progress)?;
    fs::set_permissions(&endpoints.control, fs::Permissions::from_mode(0o600))?;
    fs::set_permissions(&endpoints.progress, fs::Permissions::from_mode(0o600))?;

    let generation = WorkerInstanceId::from_uuid(Uuid::new_v4());
    let (control_token, control_pending) = issue_worker_channel_authentication(
        generation,
        WorkerRole::FixtureWorker,
        WorkerChannel::Control,
    )
    .map_err(|error| format!("control bootstrap entropy: {error}"))?;
    let (progress_token, progress_pending) = issue_worker_channel_authentication(
        generation,
        WorkerRole::FixtureWorker,
        WorkerChannel::Progress,
    )
    .map_err(|error| format!("progress bootstrap entropy: {error}"))?;
    let scope = WorkerScope {
        task_id: TaskId::from_uuid(Uuid::new_v4()),
        account_id: AccountId::from_uuid(Uuid::new_v4()),
    };
    let packet = BootstrapPacket {
        control_endpoint: endpoint_text(&endpoints.control)?,
        progress_endpoint: endpoint_text(&endpoints.progress)?,
        control: ChannelHello {
            channel: ChannelKind::Control,
            identity: WorkerHello::new(generation, WorkerRole::FixtureWorker, control_token),
            offer: offer()?,
        },
        progress: ChannelHello {
            channel: ChannelKind::Progress,
            identity: WorkerHello::new(generation, WorkerRole::FixtureWorker, progress_token),
            offer: offer()?,
        },
        scope,
        handshake_timeout_millis: u32::try_from(IO_TIMEOUT.as_millis())?,
    };
    let bootstrap = serde_json::to_vec(&packet)?;
    if bootstrap.len() as u64 > MAX_BOOTSTRAP_BYTES {
        return Err("fixture bootstrap exceeds its private stdin bound".into());
    }

    let mut child = ChildGuard(
        Command::new(std::env::current_exe()?)
            .args(["--exact", "worker_client_child", "--ignored", "--nocapture"])
            .stdin(Stdio::piped())
            .spawn()?,
    );
    assert_ne!(child.0.id(), std::process::id());
    child
        .0
        .stdin
        .take()
        .ok_or("missing child bootstrap stdin")?
        .write_all(&bootstrap)?;

    let expected =
        ExpectedPeer::unix_process(child.0.id(), geteuid().as_raw(), getegid().as_raw())?;
    let mut control_verifier = control_pending.bind(expected);
    let mut progress_verifier = progress_pending.bind(expected);

    let (mut control, _) = control_listener.accept()?;
    let (mut progress, _) = progress_listener.accept()?;
    for stream in [&control, &progress] {
        stream.set_read_timeout(Some(IO_TIMEOUT))?;
        stream.set_write_timeout(Some(IO_TIMEOUT))?;
    }
    control_verifier.check_unix_peer(&control)?;
    progress_verifier.check_unix_peer(&progress)?;

    let control_hello = read_bootstrap_hello(&mut control)?;
    let progress_hello = read_bootstrap_hello(&mut progress)?;
    assert_eq!(control_hello.channel, ChannelKind::Control);
    assert_eq!(progress_hello.channel, ChannelKind::Progress);
    let control_identity = control_verifier.authenticate_unix_channel(
        WorkerChannel::Control,
        &control_hello.identity,
        &control,
    )?;
    let progress_identity = progress_verifier.authenticate_unix_channel(
        WorkerChannel::Progress,
        &progress_hello.identity,
        &progress,
    )?;
    assert_eq!(control_identity.instance_id(), generation);
    assert_eq!(progress_identity.instance_id(), generation);
    assert_eq!(control_identity.role(), WorkerRole::FixtureWorker);
    assert_eq!(progress_identity.role(), WorkerRole::FixtureWorker);
    assert_eq!(control_identity.channel(), Some(WorkerChannel::Control));
    assert_eq!(progress_identity.channel(), Some(WorkerChannel::Progress));

    let control_codec = ControlCodec::negotiate(&control_hello.offer)?;
    let progress_codec = ControlCodec::negotiate(&progress_hello.offer)?;
    write_welcome(&mut control, &control_codec, generation)?;
    write_welcome(&mut progress, &progress_codec, generation)?;

    let ready: Envelope<ControlMessage> =
        control_codec.decode_owned(read_frame(&mut control)?, wire_limits())?;
    assert!(matches!(
        ready.into_payload(),
        ControlMessage::Ready { generation: actual } if actual == generation
    ));

    let status = child.0.wait()?;
    assert!(status.success(), "worker client child failed: {status}");
    Ok(())
}

#[test]
fn real_child_rejects_a_control_launch_record_bound_to_progress_lane() -> TestResult {
    let endpoints = EndpointPair::new()?;
    let control_listener = UnixListener::bind(&endpoints.control)?;
    let progress_listener = UnixListener::bind(&endpoints.progress)?;
    fs::set_permissions(&endpoints.control, fs::Permissions::from_mode(0o600))?;
    fs::set_permissions(&endpoints.progress, fs::Permissions::from_mode(0o600))?;

    let generation = WorkerInstanceId::from_uuid(Uuid::new_v4());
    let (control_token, control_pending) = issue_worker_channel_authentication(
        generation,
        WorkerRole::FixtureWorker,
        WorkerChannel::Progress,
    )
    .map_err(|error| format!("wrong-lane control bootstrap entropy: {error}"))?;
    let (progress_token, _progress_pending) = issue_worker_channel_authentication(
        generation,
        WorkerRole::FixtureWorker,
        WorkerChannel::Progress,
    )
    .map_err(|error| format!("progress bootstrap entropy: {error}"))?;
    let packet = BootstrapPacket {
        control_endpoint: endpoint_text(&endpoints.control)?,
        progress_endpoint: endpoint_text(&endpoints.progress)?,
        control: ChannelHello {
            channel: ChannelKind::Control,
            identity: WorkerHello::new(generation, WorkerRole::FixtureWorker, control_token),
            offer: offer()?,
        },
        progress: ChannelHello {
            channel: ChannelKind::Progress,
            identity: WorkerHello::new(generation, WorkerRole::FixtureWorker, progress_token),
            offer: offer()?,
        },
        scope: WorkerScope {
            task_id: TaskId::from_uuid(Uuid::new_v4()),
            account_id: AccountId::from_uuid(Uuid::new_v4()),
        },
        handshake_timeout_millis: u32::try_from(IO_TIMEOUT.as_millis())?,
    };
    let bootstrap = serde_json::to_vec(&packet)?;
    if bootstrap.len() as u64 > MAX_BOOTSTRAP_BYTES {
        return Err("fixture bootstrap exceeds its private stdin bound".into());
    }

    let mut child = ChildGuard(
        Command::new(std::env::current_exe()?)
            .args(["--exact", "worker_client_child", "--ignored", "--nocapture"])
            .stdin(Stdio::piped())
            .spawn()?,
    );
    child
        .0
        .stdin
        .take()
        .ok_or("missing child bootstrap stdin")?
        .write_all(&bootstrap)?;

    let expected =
        ExpectedPeer::unix_process(child.0.id(), geteuid().as_raw(), getegid().as_raw())?;
    let mut control_verifier = control_pending.bind(expected);
    let (mut control, _) = control_listener.accept()?;
    let (_progress, _) = progress_listener.accept()?;
    control.set_read_timeout(Some(IO_TIMEOUT))?;
    control.set_write_timeout(Some(IO_TIMEOUT))?;
    control_verifier.check_unix_peer(&control)?;

    let control_hello = read_bootstrap_hello(&mut control)?;
    assert_eq!(control_hello.channel, ChannelKind::Control);
    assert_eq!(control_hello.identity.instance_id(), generation);
    assert_eq!(control_hello.identity.role(), WorkerRole::FixtureWorker);
    assert_eq!(
        control_verifier.authenticate_unix_channel(
            WorkerChannel::Control,
            &control_hello.identity,
            &control,
        ),
        Err(AuthenticationError::LaunchIdentityMismatch)
    );
    assert_eq!(
        control_verifier.authenticate_unix_channel(
            WorkerChannel::Progress,
            &control_hello.identity,
            &control,
        ),
        Err(AuthenticationError::AlreadyConsumed)
    );
    Ok(())
}
