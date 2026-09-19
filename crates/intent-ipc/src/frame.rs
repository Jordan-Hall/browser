use crate::{WireError, WireErrorCode, WireLimits};
use serde::{Deserialize, Serialize};
use std::mem;

pub const FRAME_HEADER_BYTES: usize = 5;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum FrameLane {
    Control = 1,
    Artifact = 2,
}

impl TryFrom<u8> for FrameLane {
    type Error = WireError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Control),
            2 => Ok(Self::Artifact),
            _ => Err(WireError::new(
                WireErrorCode::InvalidLane,
                format!("unknown frame lane {value}"),
            )),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Frame {
    lane: FrameLane,
    payload: Vec<u8>,
}

impl Frame {
    #[must_use]
    pub fn new(lane: FrameLane, payload: Vec<u8>) -> Self {
        Self { lane, payload }
    }

    #[must_use]
    pub const fn lane(&self) -> FrameLane {
        self.lane
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    #[must_use]
    pub fn into_payload(self) -> Vec<u8> {
        self.payload
    }

    pub fn encode(&self, limits: WireLimits) -> Result<Vec<u8>, WireError> {
        validate_length(self.lane, self.payload.len(), limits)?;
        let payload_len = u32::try_from(self.payload.len()).map_err(|_| {
            WireError::new(
                WireErrorCode::FrameTooLarge,
                "frame payload length cannot be represented by the wire format",
            )
        })?;

        let mut encoded = Vec::new();
        encoded
            .try_reserve_exact(FRAME_HEADER_BYTES + self.payload.len())
            .map_err(|_| {
                WireError::new(
                    WireErrorCode::AllocationFailed,
                    "failed to reserve encoded frame buffer",
                )
            })?;
        encoded.push(self.lane as u8);
        encoded.extend_from_slice(&payload_len.to_be_bytes());
        encoded.extend_from_slice(&self.payload);
        Ok(encoded)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodeBatch {
    frames: Vec<Frame>,
    consumed: usize,
}

impl DecodeBatch {
    #[must_use]
    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    #[must_use]
    pub fn into_frames(self) -> Vec<Frame> {
        self.frames
    }

    #[must_use]
    pub const fn consumed(&self) -> usize {
        self.consumed
    }
}

/// Incremental bounded decoder that emits only complete frames after validating the header budget.
///
/// A framing error poisons the decoder. Callers must discard the connection rather than resume at
/// an unknown offset. `finish` must be called on EOF so a truncated header or payload is rejected.
#[derive(Debug)]
pub struct FrameDecoder {
    limits: WireLimits,
    header: [u8; FRAME_HEADER_BYTES],
    header_len: usize,
    current_lane: Option<FrameLane>,
    current_payload_len: usize,
    payload: Vec<u8>,
    poisoned: bool,
}

impl FrameDecoder {
    #[must_use]
    pub const fn new(limits: WireLimits) -> Self {
        Self {
            limits,
            header: [0_u8; FRAME_HEADER_BYTES],
            header_len: 0,
            current_lane: None,
            current_payload_len: 0,
            payload: Vec::new(),
            poisoned: false,
        }
    }

    pub fn push(&mut self, input: &[u8]) -> Result<DecodeBatch, WireError> {
        if self.poisoned {
            return Err(Self::poisoned_error());
        }

        match self.push_inner(input) {
            Ok(batch) => Ok(batch),
            Err(error) => {
                self.poisoned = true;
                self.reset_frame_state();
                Err(error)
            }
        }
    }

    /// Finalize the stream at EOF. Any partial frame is a protocol error and poisons the decoder.
    pub fn finish(&mut self) -> Result<(), WireError> {
        if self.poisoned {
            return Err(Self::poisoned_error());
        }
        if self.header_len == 0 && self.current_lane.is_none() && self.payload.is_empty() {
            return Ok(());
        }

        self.poisoned = true;
        self.reset_frame_state();
        Err(WireError::new(
            WireErrorCode::InvalidEnvelope,
            "stream ended with an incomplete frame",
        ))
    }

    #[must_use]
    pub const fn is_poisoned(&self) -> bool {
        self.poisoned
    }

    #[must_use]
    pub fn buffered_payload_len(&self) -> usize {
        self.payload.len()
    }

    #[must_use]
    pub fn buffered_payload_capacity(&self) -> usize {
        self.payload.capacity()
    }

    fn push_inner(&mut self, input: &[u8]) -> Result<DecodeBatch, WireError> {
        let mut cursor = 0_usize;
        let mut frames = Vec::new();
        let frame_budget = self.limits.max_frames_per_feed.max(1);

        while cursor < input.len() && frames.len() < frame_budget {
            if self.current_lane.is_none() {
                cursor += self.consume_header(&input[cursor..])?;
                if self.current_lane.is_none() {
                    continue;
                }
                if self.current_payload_len == 0 {
                    frames.push(Frame::new(
                        self.current_lane.take().ok_or_else(|| {
                            WireError::new(
                                WireErrorCode::InvalidEnvelope,
                                "decoder lost lane while completing zero-length frame",
                            )
                        })?,
                        Vec::new(),
                    ));
                    self.reset_frame_state();
                    continue;
                }
            }

            let remaining = self.current_payload_len - self.payload.len();
            let take = remaining.min(input.len() - cursor);
            self.payload
                .extend_from_slice(&input[cursor..cursor + take]);
            cursor += take;

            if self.payload.len() == self.current_payload_len {
                let lane = self.current_lane.take().ok_or_else(|| {
                    WireError::new(
                        WireErrorCode::InvalidEnvelope,
                        "decoder lost lane while completing frame",
                    )
                })?;
                frames.push(Frame::new(lane, mem::take(&mut self.payload)));
                self.reset_frame_state();
            }
        }

        Ok(DecodeBatch {
            frames,
            consumed: cursor,
        })
    }

    fn consume_header(&mut self, input: &[u8]) -> Result<usize, WireError> {
        let missing = FRAME_HEADER_BYTES - self.header_len;
        let take = missing.min(input.len());
        self.header[self.header_len..self.header_len + take].copy_from_slice(&input[..take]);
        self.header_len += take;

        if self.header_len < FRAME_HEADER_BYTES {
            return Ok(take);
        }

        let lane = FrameLane::try_from(self.header[0])?;
        let payload_len = u32::from_be_bytes([
            self.header[1],
            self.header[2],
            self.header[3],
            self.header[4],
        ]) as usize;
        validate_length(lane, payload_len, self.limits)?;

        self.current_lane = Some(lane);
        self.current_payload_len = payload_len;
        self.payload.clear();
        if payload_len > 0 {
            self.payload.try_reserve_exact(payload_len).map_err(|_| {
                WireError::new(
                    WireErrorCode::AllocationFailed,
                    "failed to reserve bounded frame payload",
                )
            })?;
        }
        self.header_len = 0;
        Ok(take)
    }

    fn reset_frame_state(&mut self) {
        self.header_len = 0;
        self.current_lane = None;
        self.current_payload_len = 0;
        self.payload = Vec::new();
    }

    fn poisoned_error() -> WireError {
        WireError::new(
            WireErrorCode::InvalidEnvelope,
            "frame decoder is poisoned after a previous framing error",
        )
    }
}

fn validate_length(
    lane: FrameLane,
    payload_len: usize,
    limits: WireLimits,
) -> Result<(), WireError> {
    let limit = match lane {
        FrameLane::Control => limits.max_control_frame_bytes,
        FrameLane::Artifact => limits.max_artifact_frame_bytes,
    };

    if payload_len > limit {
        return Err(WireError::new(
            WireErrorCode::FrameTooLarge,
            format!("{lane:?} frame advertises {payload_len} bytes but limit is {limit}"),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Frame, FrameDecoder, FrameLane};
    use crate::{WireErrorCode, WireLimits};
    use std::error::Error;

    #[test]
    fn fragmented_frame_is_not_emitted_until_complete() -> Result<(), Box<dyn Error>> {
        let limits = WireLimits::for_tests();
        let encoded = Frame::new(FrameLane::Control, b"action".to_vec()).encode(limits)?;
        let mut decoder = FrameDecoder::new(limits);

        for byte in &encoded[..encoded.len() - 1] {
            let batch = decoder.push(std::slice::from_ref(byte))?;
            assert!(batch.frames().is_empty());
        }

        let batch = decoder.push(&encoded[encoded.len() - 1..])?;
        assert_eq!(batch.frames().len(), 1);
        assert_eq!(batch.frames()[0].payload(), b"action");
        decoder.finish()?;
        Ok(())
    }

    #[test]
    fn oversized_length_is_rejected_before_payload_allocation() -> Result<(), Box<dyn Error>> {
        let limits = WireLimits::for_tests();
        let mut decoder = FrameDecoder::new(limits);
        let advertised = (limits.max_control_frame_bytes as u32 + 1).to_be_bytes();
        let header = [
            FrameLane::Control as u8,
            advertised[0],
            advertised[1],
            advertised[2],
            advertised[3],
        ];

        let Err(error) = decoder.push(&header) else {
            return Err("oversized frame unexpectedly decoded".into());
        };
        assert_eq!(error.code(), WireErrorCode::FrameTooLarge);
        assert_eq!(decoder.buffered_payload_len(), 0);
        assert_eq!(decoder.buffered_payload_capacity(), 0);
        assert!(decoder.is_poisoned());
        Ok(())
    }

    #[test]
    fn invalid_lane_fails_closed() -> Result<(), Box<dyn Error>> {
        let limits = WireLimits::for_tests();
        let mut decoder = FrameDecoder::new(limits);
        let Err(error) = decoder.push(&[99, 0, 0, 0, 0]) else {
            return Err("invalid lane unexpectedly decoded".into());
        };
        assert_eq!(error.code(), WireErrorCode::InvalidLane);
        assert!(decoder.is_poisoned());
        Ok(())
    }

    #[test]
    fn control_and_artifact_lanes_have_independent_limits() -> Result<(), Box<dyn Error>> {
        let limits = WireLimits::for_tests();
        let payload = vec![0_u8; limits.max_control_frame_bytes + 1];
        let Err(control_error) = Frame::new(FrameLane::Control, payload.clone()).encode(limits)
        else {
            return Err("oversized control payload unexpectedly encoded".into());
        };
        assert_eq!(control_error.code(), WireErrorCode::FrameTooLarge);

        Frame::new(FrameLane::Artifact, payload).encode(limits)?;
        Ok(())
    }

    #[test]
    fn feed_frame_budget_returns_consumed_offset_without_losing_input() -> Result<(), Box<dyn Error>>
    {
        let mut limits = WireLimits::for_tests();
        limits.max_frames_per_feed = 1;
        let first = Frame::new(FrameLane::Control, b"a".to_vec()).encode(limits)?;
        let second = Frame::new(FrameLane::Control, b"b".to_vec()).encode(limits)?;
        let mut input = first;
        input.extend_from_slice(&second);
        let mut decoder = FrameDecoder::new(limits);

        let batch = decoder.push(&input)?;
        assert_eq!(batch.frames().len(), 1);
        assert!(batch.consumed() < input.len());

        let second_batch = decoder.push(&input[batch.consumed()..])?;
        assert_eq!(second_batch.frames().len(), 1);
        assert_eq!(second_batch.frames()[0].payload(), b"b");
        decoder.finish()?;
        Ok(())
    }

    #[test]
    fn malformed_batch_poisoning_prevents_resume_at_an_unknown_offset() -> Result<(), Box<dyn Error>>
    {
        let mut limits = WireLimits::for_tests();
        limits.max_frames_per_feed = 8;
        let first = Frame::new(FrameLane::Control, b"first".to_vec()).encode(limits)?;
        let third = Frame::new(FrameLane::Control, b"third".to_vec()).encode(limits)?;
        let mut input = first;
        input.extend_from_slice(&[99, 0, 0, 0, 0]);
        input.extend_from_slice(&third);
        let mut decoder = FrameDecoder::new(limits);

        let Err(error) = decoder.push(&input) else {
            return Err("valid+invalid+valid batch unexpectedly decoded".into());
        };
        assert_eq!(error.code(), WireErrorCode::InvalidLane);
        assert!(decoder.is_poisoned());

        let Err(error) = decoder.push(&third) else {
            return Err("poisoned decoder unexpectedly resumed".into());
        };
        assert_eq!(error.code(), WireErrorCode::InvalidEnvelope);
        Ok(())
    }

    #[test]
    fn eof_rejects_truncated_header_and_payload() -> Result<(), Box<dyn Error>> {
        let limits = WireLimits::for_tests();

        let mut header_decoder = FrameDecoder::new(limits);
        assert!(
            header_decoder
                .push(&[FrameLane::Control as u8, 0])?
                .frames()
                .is_empty()
        );
        let Err(header_error) = header_decoder.finish() else {
            return Err("truncated header unexpectedly accepted at EOF".into());
        };
        assert_eq!(header_error.code(), WireErrorCode::InvalidEnvelope);
        assert!(header_decoder.is_poisoned());

        let encoded = Frame::new(FrameLane::Control, b"payload".to_vec()).encode(limits)?;
        let mut payload_decoder = FrameDecoder::new(limits);
        assert!(
            payload_decoder
                .push(&encoded[..encoded.len() - 1])?
                .frames()
                .is_empty()
        );
        let Err(payload_error) = payload_decoder.finish() else {
            return Err("truncated payload unexpectedly accepted at EOF".into());
        };
        assert_eq!(payload_error.code(), WireErrorCode::InvalidEnvelope);
        assert!(payload_decoder.is_poisoned());

        let mut clean = FrameDecoder::new(limits);
        clean.finish()?;
        Ok(())
    }
}
