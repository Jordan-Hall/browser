//! Worker-side endpoint. Only the supervisor's authenticated readiness creates a live session.
use crate::{
    BootstrapPacket, ChannelKind, ControlMessage, ProgressMessage, SupervisorError,
    wire::{
        FramedSocket, ReadOutcome, decode, encode_bootstrap_event, encode_envelope, encode_event,
        read_blocking,
    },
};
use intent_contracts::{SchemaVersion, WorkerInstanceId};
use intent_ipc::{ControlCodec, Envelope, EnvelopeKind};
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
            encode_bootstrap_event(ControlMessage::Hello(hello))?.write_all(stream)?;
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
            ReadOutcome::Frame(frame) => {
                let envelope = decode(frame, Some(&self.codec))?;
                self.validate_supervisor_control(&envelope)?;
                Ok(Some(envelope))
            }
        }
    }
    fn validate_supervisor_control(
        &self,
        envelope: &Envelope<ControlMessage>,
    ) -> Result<(), SupervisorError> {
        let valid = match envelope.payload() {
            ControlMessage::Execute {
                generation,
                request_id,
                deadline,
                ..
            } => {
                *generation == self.generation
                    && envelope.message()
                        == (EnvelopeKind::Request {
                            request_id: *request_id,
                        })
                    && envelope.deadline() == Some(*deadline)
            }
            ControlMessage::Cancel {
                generation,
                cancellation_id,
            } => {
                *generation == self.generation
                    && envelope.message() == EnvelopeKind::Event
                    && envelope.cancellation_id() == Some(*cancellation_id)
            }
            ControlMessage::Yield { generation } => {
                *generation == self.generation && envelope.message() == EnvelopeKind::Event
            }
            _ => false,
        };
        if valid {
            Ok(())
        } else {
            Err(SupervisorError::Protocol)
        }
    }
    pub fn send(&mut self, envelope: &Envelope<ControlMessage>) -> Result<(), SupervisorError> {
        let bytes = encode_envelope(envelope, Some(&self.codec))?;
        if matches!(envelope.payload(), ControlMessage::Cancelled { .. }) {
            self.control.queue_reserved(bytes)?;
        } else {
            self.control.queue(bytes)?;
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::{decode, offer, wire_limits};
    use intent_contracts::{CancellationId, TraceId};
    use intent_ipc::{Envelope, Frame, FrameLane};
    use std::io::Write as _;
    use uuid::Uuid;

    fn test_client() -> Result<(WorkerClient, UnixStream), Box<dyn std::error::Error>> {
        let (worker_control, peer_control) = UnixStream::pair()?;
        let (worker_progress, _peer_progress) = UnixStream::pair()?;
        let codec = ControlCodec::negotiate(&offer()?)?;
        let generation = WorkerInstanceId::from_uuid(Uuid::new_v4());
        Ok((
            WorkerClient {
                generation,
                codec,
                control: FramedSocket::new(worker_control)?,
                progress: FramedSocket::new(worker_progress)?,
                heartbeat: 0,
                progress_sequence: 0,
            },
            peer_control,
        ))
    }

    #[test]
    fn poll_control_rejects_worker_direction_and_foreign_generation()
    -> Result<(), Box<dyn std::error::Error>> {
        let (mut client, mut peer) = test_client()?;
        let generation = client.generation();
        let allowed = Envelope::event(
            TraceId::from_uuid(Uuid::new_v4()),
            ControlMessage::Yield { generation },
        );
        peer.write_all(&encode_envelope(&allowed, Some(&client.codec))?)?;
        assert!(matches!(
            client.poll_control()?.map(Envelope::into_payload),
            Some(ControlMessage::Yield { generation: actual }) if actual == generation
        ));

        let worker_direction = Envelope::event(
            TraceId::from_uuid(Uuid::new_v4()),
            ControlMessage::Heartbeat {
                generation,
                sequence: 1,
            },
        );
        peer.write_all(&encode_envelope(
            &worker_direction,
            Some(&client.codec),
        )?)?;
        assert!(matches!(client.poll_control(), Err(SupervisorError::Protocol)));

        let foreign_generation = WorkerInstanceId::from_uuid(Uuid::new_v4());
        let wrong_instance = Envelope::event(
            TraceId::from_uuid(Uuid::new_v4()),
            ControlMessage::Yield {
                generation: foreign_generation,
            },
        );
        peer.write_all(&encode_envelope(
            &wrong_instance,
            Some(&client.codec),
        )?)?;
        assert!(matches!(client.poll_control(), Err(SupervisorError::Protocol)));
        Ok(())
    }

    #[test]
    fn cancellation_ack_queues_behind_a_partially_written_control_frame()
    -> Result<(), Box<dyn std::error::Error>> {
        let (worker_control, peer_control) = UnixStream::pair()?;
        let (worker_progress, _peer_progress) = UnixStream::pair()?;
        let codec = ControlCodec::negotiate(&offer()?)?;
        let generation = WorkerInstanceId::from_uuid(Uuid::new_v4());
        let mut client = WorkerClient {
            generation,
            codec,
            control: FramedSocket::new(worker_control)?,
            progress: FramedSocket::new(worker_progress)?,
            heartbeat: 0,
            progress_sequence: 0,
        };
        let filler = Frame::new(FrameLane::Control, vec![7; 5000]).encode(wire_limits())?;
        client.control.queue(filler)?;
        client.control.flush(4096)?;
        assert!(!client.control.idle());

        let trace = TraceId::from_uuid(Uuid::new_v4());
        let cancellation_id = CancellationId::from_uuid(Uuid::new_v4());
        client.send(
            &Envelope::event(
                trace,
                ControlMessage::Cancelled {
                    generation,
                    cancellation_id,
                },
            )
            .with_cancellation_id(cancellation_id),
        )?;

        let mut peer = FramedSocket::new(peer_control)?;
        let mut budget = 16 * 1024;
        let first = peer.read_one(&mut budget)?;
        assert!(matches!(first, ReadOutcome::Frame(frame) if frame.payload() == vec![7; 5000]));
        let second = peer.read_one(&mut budget)?;
        let ReadOutcome::Frame(frame) = second else {
            return Err("reserved cancellation acknowledgement was not delivered".into());
        };
        let envelope: Envelope<ControlMessage> = decode(frame, Some(&client.codec))?;
        assert_eq!(envelope.trace_id(), trace);
        assert_eq!(envelope.cancellation_id(), Some(cancellation_id));
        assert!(matches!(
            envelope.into_payload(),
            ControlMessage::Cancelled {
                generation: actual_generation,
                cancellation_id: actual_cancellation,
            } if actual_generation == generation && actual_cancellation == cancellation_id
        ));
        Ok(())
    }
}
