#![cfg(unix)]

use intent_contracts::{AccountId, BoundedText, SchemaVersion, TaskId, TraceId, WorkerInstanceId};
use intent_ipc::{
    ControlCodec, Envelope, Frame, FrameLane, ProtocolOffer, ProtocolRange, decode_control_owned,
};
use intent_local_transport::{WorkerHello, WorkerRole, issue_worker_authentication};
use intent_supervisor::{
    BootstrapPacket, ChannelHello, ChannelKind, ControlMessage, WorkerScope, wire_limits,
    worker::WorkerClient,
};
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

const IO_TIMEOUT: Duration = Duration::from_secs(5);

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
    let envelope: Envelope<ControlMessage> = decode_control_owned(read_frame(stream)?, wire_limits())?;
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
    let bytes = codec.encode(&envelope, wire_limits())?.encode(wire_limits())?;
    stream.write_all(&bytes)?;
    Ok(())
}

#[test]
fn worker_client_completes_two_lane_bootstrap_on_unix() -> TestResult {
    let endpoints = EndpointPair::new()?;
    let control_listener = UnixListener::bind(&endpoints.control)?;
    let progress_listener = UnixListener::bind(&endpoints.progress)?;
    fs::set_permissions(&endpoints.control, fs::Permissions::from_mode(0o600))?;
    fs::set_permissions(&endpoints.progress, fs::Permissions::from_mode(0o600))?;

    let generation = WorkerInstanceId::from_uuid(Uuid::new_v4());
    let (control_token, _control_verifier) =
        issue_worker_authentication(generation, WorkerRole::FixtureWorker)
            .map_err(|error| format!("control bootstrap entropy: {error}"))?;
    let (progress_token, _progress_verifier) =
        issue_worker_authentication(generation, WorkerRole::FixtureWorker)
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

    let worker = thread::spawn(move || {
        WorkerClient::connect(packet)
            .map(|client| client.generation())
            .map_err(|error| error.to_string())
    });

    let (mut control, _) = control_listener.accept()?;
    let (mut progress, _) = progress_listener.accept()?;
    for stream in [&control, &progress] {
        stream.set_read_timeout(Some(IO_TIMEOUT))?;
        stream.set_write_timeout(Some(IO_TIMEOUT))?;
    }

    let control_hello = read_bootstrap_hello(&mut control)?;
    let progress_hello = read_bootstrap_hello(&mut progress)?;
    assert_eq!(control_hello.channel, ChannelKind::Control);
    assert_eq!(progress_hello.channel, ChannelKind::Progress);
    assert_eq!(control_hello.identity.instance_id(), generation);
    assert_eq!(progress_hello.identity.instance_id(), generation);
    assert_eq!(control_hello.identity.role(), WorkerRole::FixtureWorker);
    assert_eq!(progress_hello.identity.role(), WorkerRole::FixtureWorker);

    let control_codec = ControlCodec::negotiate(&control_hello.offer)?;
    let progress_codec = ControlCodec::negotiate(&progress_hello.offer)?;
    write_welcome(&mut control, &control_codec, generation)?;
    write_welcome(&mut progress, &progress_codec, generation)?;

    let ready: Envelope<ControlMessage> = control_codec.decode_owned(read_frame(&mut control)?, wire_limits())?;
    assert!(matches!(
        ready.into_payload(),
        ControlMessage::Ready { generation: actual } if actual == generation
    ));
    assert_eq!(
        worker.join().map_err(|_| "worker client thread panicked")??,
        generation
    );
    Ok(())
}
