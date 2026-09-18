//! Worker-side endpoint. Only the supervisor's authenticated readiness creates a live session.
use crate::{
    BootstrapPacket, ChannelKind, ControlMessage, ProgressMessage, SupervisorError,
    wire::{FramedSocket, ReadOutcome, decode, encode_envelope, encode_event, read_blocking},
};
use intent_contracts::{SchemaVersion, WorkerInstanceId};
use intent_ipc::{ControlCodec, Envelope};
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct WorkerClient {
    generation: WorkerInstanceId,
    codec: ControlCodec,
    control: FramedSocket,
    progress: FramedSocket,
    heartbeat: u64,
    progress_sequence: u64,
}
impl WorkerClient {
    pub fn read_bootstrap(reader: &mut impl Read) -> Result<BootstrapPacket, SupervisorError> {
        let envelope: Envelope<BootstrapPacket> = read_blocking(reader, None)?;
        Ok(envelope.into_payload())
    }
    pub fn connect(packet: BootstrapPacket) -> Result<Self, SupervisorError> {
        if packet.control.channel != ChannelKind::Control
            || packet.progress.channel != ChannelKind::Progress
            || packet.control.identity.instance_id() != packet.progress.identity.instance_id()
            || packet.control.identity.role() != packet.progress.identity.role()
            || !(1..=60000).contains(&packet.handshake_timeout_millis)
        {
            return Err(SupervisorError::Protocol);
        }
        let generation = packet.control.identity.instance_id();
        let codec = ControlCodec::negotiate(&packet.control.offer)
            .map_err(|_| SupervisorError::Protocol)?;
        let deadline =
            Instant::now() + Duration::from_millis(u64::from(packet.handshake_timeout_millis));
        let mut control = UnixStream::connect(packet.control_endpoint.as_str())?;
        let mut progress = UnixStream::connect(packet.progress_endpoint.as_str())?;
        for (stream, hello) in [
            (&mut control, packet.control),
            (&mut progress, packet.progress),
        ] {
            let remaining = deadline
                .checked_duration_since(Instant::now())
                .ok_or(SupervisorError::DeadlineExpired)?;
            stream.set_read_timeout(Some(remaining))?;
            stream.set_write_timeout(Some(remaining))?;
            stream.write_all(&encode_event(ControlMessage::Hello(hello), None)?)?;
        }
        for stream in [&mut control, &mut progress] {
            let remaining = deadline
                .checked_duration_since(Instant::now())
                .ok_or(SupervisorError::DeadlineExpired)?;
            stream.set_read_timeout(Some(remaining))?;
            let welcome: Envelope<ControlMessage> = read_blocking(stream, Some(&codec))?;
            if !matches!(welcome.payload(), ControlMessage::Welcome { generation: id, selected } if *id == generation && *selected == SchemaVersion::V1)
            {
                return Err(SupervisorError::Protocol);
            }
        }
        control.write_all(&encode_event(
            ControlMessage::Ready { generation },
            Some(&codec),
        )?)?;
        Ok(Self {
            generation,
            codec,
            control: FramedSocket::new(control)?,
            progress: FramedSocket::new(progress)?,
            heartbeat: 0,
            progress_sequence: 0,
        })
    }
    pub const fn generation(&self) -> WorkerInstanceId {
        self.generation
    }
    pub fn poll_control(&mut self) -> Result<Option<Envelope<ControlMessage>>, SupervisorError> {
        self.control.flush(4096)?;
        self.progress.flush(4096)?;
        let mut budget = 4096;
        match self.control.read_one(&mut budget)? {
            ReadOutcome::Pending => Ok(None),
            ReadOutcome::Closed => Err(SupervisorError::Io(
                std::io::ErrorKind::UnexpectedEof.into(),
            )),
            ReadOutcome::Frame(frame) => Ok(Some(decode(&frame, Some(&self.codec))?)),
        }
    }
    pub fn send(&mut self, envelope: &Envelope<ControlMessage>) -> Result<(), SupervisorError> {
        self.control
            .queue(encode_envelope(envelope, Some(&self.codec))?)?;
        self.control.flush(4096)?;
        Ok(())
    }
    pub fn heartbeat(&mut self) -> Result<(), SupervisorError> {
        if !self.control.idle() {
            return Ok(());
        }
        self.heartbeat = self
            .heartbeat
            .checked_add(1)
            .ok_or(SupervisorError::CounterExhausted)?;
        self.control.queue(encode_event(
            ControlMessage::Heartbeat {
                generation: self.generation,
                sequence: self.heartbeat,
            },
            Some(&self.codec),
        )?)?;
        self.control.flush(4096)?;
        Ok(())
    }
    pub fn progress(&mut self, work_sequence: u64) -> Result<bool, SupervisorError> {
        self.progress.flush(4096)?;
        if !self.progress.idle() {
            return Ok(false);
        }
        self.progress_sequence = self
            .progress_sequence
            .checked_add(1)
            .ok_or(SupervisorError::CounterExhausted)?;
        self.progress.queue(encode_event(
            ProgressMessage {
                generation: self.generation,
                sequence: self.progress_sequence,
                work_sequence,
            },
            Some(&self.codec),
        )?)?;
        self.progress.flush(4096)?;
        Ok(true)
    }
}
