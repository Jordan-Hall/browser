use crate::{SupervisorError, WorkerScope};
use intent_contracts::{
    BoundedText, CancellationId, CapabilityId, RequestId, SchemaVersion, TraceId,
    UnixTimestampMicros, WorkerInstanceId,
};
use intent_ipc::{
    ControlCodec, Envelope, Frame, FrameDecoder, ProtocolOffer, ProtocolRange, WireLimits,
};
use intent_local_transport::{MessageFamily, WorkerHello};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{
    io::{self, Read, Write},
    os::unix::net::UnixStream,
};
use uuid::Uuid;

pub const MAX_PACKET_BYTES: usize = 16384;
pub fn wire_limits() -> WireLimits {
    WireLimits {
        max_control_frame_bytes: MAX_PACKET_BYTES,
        max_artifact_frame_bytes: 0,
        max_frames_per_feed: 1,
        max_json_depth: 16,
        max_collection_entries: 128,
        max_json_nodes: 512,
    }
}
pub(crate) fn offer() -> Result<ProtocolOffer, SupervisorError> {
    ProtocolOffer::try_new(
        vec![ProtocolRange::try_new(1, 0, 0).map_err(|_| SupervisorError::Protocol)?],
        [],
    )
    .map_err(|_| SupervisorError::Protocol)
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelKind {
    Control,
    Progress,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelHello {
    pub channel: ChannelKind,
    pub identity: WorkerHello,
    pub offer: ProtocolOffer,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BootstrapPacket {
    pub control_endpoint: BoundedText<108>,
    pub progress_endpoint: BoundedText<108>,
    pub control: ChannelHello,
    pub progress: ChannelHello,
    pub scope: WorkerScope,
    pub handshake_timeout_millis: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ControlMessage {
    Hello(ChannelHello),
    Welcome {
        generation: WorkerInstanceId,
        selected: SchemaVersion,
    },
    Ready {
        generation: WorkerInstanceId,
    },
    Heartbeat {
        generation: WorkerInstanceId,
        sequence: u64,
    },
    Execute {
        generation: WorkerInstanceId,
        request_id: RequestId,
        sequence: u64,
        scope: WorkerScope,
        capability: CapabilityId,
        family: MessageFamily,
        deadline: UnixTimestampMicros,
        input: BoundedText<4096>,
    },
    Observed {
        generation: WorkerInstanceId,
        request_id: RequestId,
        sequence: u64,
    },
    Cancel {
        generation: WorkerInstanceId,
        cancellation_id: CancellationId,
    },
    Cancelled {
        generation: WorkerInstanceId,
        cancellation_id: CancellationId,
    },
    Yield {
        generation: WorkerInstanceId,
    },
    Yielded {
        generation: WorkerInstanceId,
    },
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProgressMessage {
    pub generation: WorkerInstanceId,
    pub sequence: u64,
    pub work_sequence: u64,
}

pub(crate) fn encode_event<T: Serialize>(
    payload: T,
    codec: Option<&ControlCodec>,
) -> Result<Vec<u8>, SupervisorError> {
    let envelope = Envelope::event(TraceId::from_uuid(Uuid::new_v4()), payload);
    encode_envelope(&envelope, codec)
}
pub(crate) fn encode_envelope<T: Serialize>(
    envelope: &Envelope<T>,
    codec: Option<&ControlCodec>,
) -> Result<Vec<u8>, SupervisorError> {
    let frame = match codec {
        Some(codec) => codec.encode(envelope, wire_limits())?,
        None => intent_ipc::encode_control(envelope, wire_limits())?,
    };
    Ok(frame.encode(wire_limits())?)
}
pub(crate) fn decode<T: DeserializeOwned>(
    frame: &Frame,
    codec: Option<&ControlCodec>,
) -> Result<Envelope<T>, SupervisorError> {
    Ok(match codec {
        Some(codec) => codec.decode(frame, wire_limits())?,
        None => intent_ipc::decode_control(frame, wire_limits())?,
    })
}
pub(crate) fn read_blocking<T: DeserializeOwned>(
    reader: &mut impl Read,
    codec: Option<&ControlCodec>,
) -> Result<Envelope<T>, SupervisorError> {
    let mut header = [0_u8; 5];
    reader.read_exact(&mut header)?;
    let length = u32::from_be_bytes([header[1], header[2], header[3], header[4]]) as usize;
    if header[0] != 1 || length > MAX_PACKET_BYTES {
        return Err(SupervisorError::Protocol);
    }
    let mut payload = vec![0; length];
    reader.read_exact(&mut payload)?;
    decode(&Frame::new(intent_ipc::FrameLane::Control, payload), codec)
}

#[derive(Debug)]
pub(crate) struct ReadBudget {
    limit: usize,
    remaining: usize,
    blocked_reads: usize,
}
impl ReadBudget {
    pub(crate) fn new(limit: usize) -> Self {
        Self {
            limit,
            remaining: limit,
            blocked_reads: 0,
        }
    }
    pub(crate) fn consumed(&self) -> usize {
        self.limit.saturating_sub(self.remaining)
    }
    pub(crate) fn blocked_reads(&self) -> usize {
        self.blocked_reads
    }
    pub(crate) fn read_one(
        &mut self,
        socket: &mut FramedSocket,
        socket_budget: usize,
    ) -> Result<ReadOutcome, SupervisorError> {
        let allowance = self.remaining.min(socket_budget);
        let mut local = allowance;
        let result = socket.read_one(&mut local);
        self.remaining = self
            .remaining
            .saturating_sub(allowance.saturating_sub(local));
        // A reduced allowance is not a scheduling block until it is exhausted
        // without producing a frame. Buffered frames and WouldBlock with unused
        // bytes must not extend a worker's health deadline.
        if allowance < socket_budget && local == 0 && matches!(&result, Ok(ReadOutcome::Pending)) {
            self.blocked_reads = self.blocked_reads.saturating_add(1);
        }
        result
    }
}

pub(crate) enum ReadOutcome {
    Pending,
    Closed,
    Frame(Frame),
}
#[derive(Debug)]
struct Outgoing {
    bytes: Vec<u8>,
    offset: usize,
}
pub(crate) struct FramedSocket {
    pub(crate) stream: UnixStream,
    decoder: FrameDecoder,
    buffer: [u8; 4096],
    offset: usize,
    length: usize,
    outgoing: Option<Outgoing>,
}
impl FramedSocket {
    pub(crate) fn new(stream: UnixStream) -> io::Result<Self> {
        stream.set_nonblocking(true)?;
        Ok(Self {
            stream,
            decoder: FrameDecoder::new(wire_limits()),
            buffer: [0; 4096],
            offset: 0,
            length: 0,
            outgoing: None,
        })
    }
    pub(crate) fn read_one(
        &mut self,
        byte_budget: &mut usize,
    ) -> Result<ReadOutcome, SupervisorError> {
        loop {
            if self.offset == self.length {
                if *byte_budget == 0 {
                    return Ok(ReadOutcome::Pending);
                }
                let limit = self.buffer.len().min(*byte_budget);
                match self.stream.read(&mut self.buffer[..limit]) {
                    Ok(0) => {
                        self.decoder.finish()?;
                        return Ok(ReadOutcome::Closed);
                    }
                    Ok(length) => {
                        self.offset = 0;
                        self.length = length;
                        *byte_budget -= length;
                    }
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                        return Ok(ReadOutcome::Pending);
                    }
                    Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                        return Ok(ReadOutcome::Pending);
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            let batch = self.decoder.push(&self.buffer[self.offset..self.length])?;
            self.offset += batch.consumed();
            if let Some(frame) = batch.into_frames().into_iter().next() {
                return Ok(ReadOutcome::Frame(frame));
            }
        }
    }
    pub(crate) fn queue(&mut self, bytes: Vec<u8>) -> Result<(), SupervisorError> {
        if self.outgoing.is_some() {
            return Err(SupervisorError::QueueFull);
        }
        if bytes.len() > MAX_PACKET_BYTES + 5 {
            return Err(SupervisorError::Protocol);
        }
        self.outgoing = Some(Outgoing { bytes, offset: 0 });
        Ok(())
    }
    pub(crate) fn idle(&self) -> bool {
        self.outgoing.is_none()
    }
    pub(crate) fn flush(&mut self, budget: usize) -> io::Result<()> {
        let Some(outgoing) = self.outgoing.as_mut() else {
            return Ok(());
        };
        let end = outgoing
            .offset
            .saturating_add(budget)
            .min(outgoing.bytes.len());
        match self.stream.write(&outgoing.bytes[outgoing.offset..end]) {
            Ok(0) => Err(io::Error::from(io::ErrorKind::WriteZero)),
            Ok(sent) => {
                outgoing.offset += sent;
                if outgoing.offset == outgoing.bytes.len() {
                    self.outgoing = None;
                }
                Ok(())
            }
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                ) =>
            {
                Ok(())
            }
            Err(error) => Err(error),
        }
    }
    /// Never splice Stop into a partially written frame. Escalation still bypasses this stream.
    pub(crate) fn discard_unstarted(&mut self) -> bool {
        if self
            .outgoing
            .as_ref()
            .is_some_and(|message| message.offset != 0)
        {
            return false;
        }
        self.outgoing = None;
        true
    }
}

impl std::fmt::Debug for ControlMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Hello(_) => "Hello",
            Self::Welcome { .. } => "Welcome",
            Self::Ready { .. } => "Ready",
            Self::Heartbeat { .. } => "Heartbeat",
            Self::Execute { .. } => "Execute",
            Self::Observed { .. } => "Observed",
            Self::Cancel { .. } => "Cancel",
            Self::Cancelled { .. } => "Cancelled",
            Self::Yield { .. } => "Yield",
            Self::Yielded { .. } => "Yielded",
        };
        f.debug_struct(name).finish_non_exhaustive()
    }
}
impl std::fmt::Debug for FramedSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FramedSocket")
            .field("unconsumed_bytes", &self.length.saturating_sub(self.offset))
            .field(
                "partial_payload_bytes",
                &self.decoder.buffered_payload_len(),
            )
            .field(
                "outgoing_bytes",
                &self
                    .outgoing
                    .as_ref()
                    .map(|m| m.bytes.len().saturating_sub(m.offset)),
            )
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_read_budget_caps_many_ready_sockets() -> Result<(), Box<dyn std::error::Error>> {
        let bytes =
            Frame::new(intent_ipc::FrameLane::Control, vec![0_u8; 3000]).encode(wire_limits())?;
        let mut senders = Vec::new();
        let mut receivers = Vec::new();
        for _ in 0..16 {
            let (receiver, mut sender) = UnixStream::pair()?;
            sender.write_all(&bytes)?;
            senders.push(sender);
            receivers.push(FramedSocket::new(receiver)?);
        }
        let mut budget = ReadBudget::new(8192);
        let mut complete = 0;
        for receiver in &mut receivers {
            if matches!(budget.read_one(receiver, 4096)?, ReadOutcome::Frame(_)) {
                complete += 1;
            }
        }
        assert_eq!(budget.consumed(), 8192);
        assert_eq!(complete, 2);
        drop(senders);
        Ok(())
    }

    #[test]
    fn transport_waits_for_complete_frames_and_never_debugs_buffer_contents()
    -> Result<(), Box<dyn std::error::Error>> {
        let (receiver, mut sender) = UnixStream::pair()?;
        let mut receiver = FramedSocket::new(receiver)?;
        let message = ControlMessage::Heartbeat {
            generation: WorkerInstanceId::from_uuid(Uuid::new_v4()),
            sequence: 1,
        };
        let bytes = encode_event(message, None)?;
        sender.write_all(&bytes[..3])?;
        assert!(matches!(
            receiver.read_one(&mut 4096)?,
            ReadOutcome::Pending
        ));
        sender.write_all(&bytes[3..])?;
        let ReadOutcome::Frame(frame) = receiver.read_one(&mut 4096)? else {
            return Err("complete frame missing".into());
        };
        let decoded: Envelope<ControlMessage> = decode(&frame, None)?;
        assert!(matches!(
            decoded.payload(),
            ControlMessage::Heartbeat { sequence: 1, .. }
        ));
        let debug = format!("{receiver:?}");
        assert!(!debug.contains("sequence"));
        assert!(!debug.contains("buffer: ["));
        assert_eq!(
            format!(
                "{:?}",
                ControlMessage::Hello(ChannelHello {
                    channel: ChannelKind::Control,
                    identity: WorkerHello::new(
                        WorkerInstanceId::from_uuid(Uuid::new_v4()),
                        intent_local_transport::WorkerRole::FixtureWorker,
                        intent_local_transport::BootstrapToken::from_bytes([171; 32])
                    ),
                    offer: offer()?,
                })
            ),
            "Hello { .. }"
        );
        Ok(())
    }
    #[test]
    fn oversized_header_does_not_allocate_a_payload_or_allow_a_partial_frame_to_be_spliced()
    -> Result<(), Box<dyn std::error::Error>> {
        let (receiver, mut sender) = UnixStream::pair()?;
        let mut receiver = FramedSocket::new(receiver)?;
        sender.write_all(&[1, 0, 0, 64, 1])?;
        assert!(receiver.read_one(&mut 4096).is_err());
        assert_eq!(receiver.decoder.buffered_payload_len(), 0);
        receiver.outgoing = Some(Outgoing {
            bytes: vec![1, 2, 3],
            offset: 1,
        });
        assert!(!receiver.discard_unstarted());
        assert!(!receiver.idle());
        receiver.outgoing = Some(Outgoing {
            bytes: vec![1, 2, 3],
            offset: 0,
        });
        assert!(receiver.discard_unstarted());
        assert!(receiver.idle());
        Ok(())
    }

    #[test]
    fn eof_with_partial_frame_is_a_protocol_error() -> Result<(), Box<dyn std::error::Error>> {
        let (receiver, mut sender) = UnixStream::pair()?;
        let mut receiver = FramedSocket::new(receiver)?;
        let bytes = encode_event(
            ControlMessage::Heartbeat {
                generation: WorkerInstanceId::from_uuid(Uuid::new_v4()),
                sequence: 9,
            },
            None,
        )?;
        sender.write_all(&bytes[..bytes.len() - 1])?;
        drop(sender);

        let mut budget = MAX_PACKET_BYTES;
        assert!(receiver.read_one(&mut budget).is_err());
        assert!(receiver.decoder.is_poisoned());
        Ok(())
    }

    #[test]
    fn clean_eof_remains_a_closed_transport() -> Result<(), Box<dyn std::error::Error>> {
        let (receiver, sender) = UnixStream::pair()?;
        let mut receiver = FramedSocket::new(receiver)?;
        drop(sender);
        let mut budget = MAX_PACKET_BYTES;
        assert!(matches!(
            receiver.read_one(&mut budget)?,
            ReadOutcome::Closed
        ));
        assert!(!receiver.decoder.is_poisoned());
        Ok(())
    }
}

#[cfg(test)]
mod budget_tests;
