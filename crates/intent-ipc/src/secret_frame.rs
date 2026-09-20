use crate::erasing_bytes::ErasingBytes;
use crate::{Envelope, FRAME_HEADER_BYTES, FrameLane, WireError, WireErrorCode, WireLimits};
use serde::Serialize;
use std::io::{self, Write};

/// Owns a bounded control frame whose current allocation is erased on disposal.
/// Borrowed bytes can be copied by callers; those copies are outside this owner.
pub struct SecretFrame {
    bytes: ErasingBytes,
}

impl SecretFrame {
    pub fn encode_control<T: Serialize>(
        envelope: &Envelope<T>,
        limits: WireLimits,
    ) -> Result<Self, WireError> {
        let capacity = limits
            .max_control_frame_bytes
            .checked_add(FRAME_HEADER_BYTES)
            .ok_or_else(|| {
                WireError::new(
                    WireErrorCode::FrameTooLarge,
                    "control frame capacity overflow",
                )
            })?;
        let mut frame = Self {
            bytes: ErasingBytes::new(Vec::new()),
        };
        frame.bytes.try_reserve_exact(capacity).map_err(|_| {
            WireError::new(
                WireErrorCode::AllocationFailed,
                "failed to reserve secret control frame",
            )
        })?;
        frame.bytes.resize(FRAME_HEADER_BYTES, 0);
        let mut writer = FixedWriter {
            bytes: &mut frame.bytes,
            capacity,
            exceeded: false,
        };
        let result = serde_json::to_writer(&mut writer, envelope);
        if writer.exceeded {
            return Err(WireError::new(
                WireErrorCode::FrameTooLarge,
                "control envelope exceeds configured byte limit",
            ));
        }
        result.map_err(|error| {
            WireError::new(
                WireErrorCode::InvalidEnvelope,
                format!("failed to serialize control envelope: {error}"),
            )
        })?;
        crate::bounded_json::validate(&frame.bytes[FRAME_HEADER_BYTES..], limits)?;
        let header = crate::frame::encode_header(
            FrameLane::Control,
            frame.bytes.len() - FRAME_HEADER_BYTES,
            limits,
        )?;
        frame.bytes[..FRAME_HEADER_BYTES].copy_from_slice(&header);
        Ok(frame)
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Sends the frame and disposes of its storage on either success or error.
    pub fn write_all(self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&self.bytes)
    }
}

struct FixedWriter<'a> {
    bytes: &'a mut Vec<u8>,
    capacity: usize,
    exceeded: bool,
}

impl Write for FixedWriter<'_> {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        if input.len() > self.capacity.saturating_sub(self.bytes.len()) {
            self.exceeded = true;
            return Err(io::Error::other("control envelope exceeds byte limit"));
        }
        self.bytes.extend_from_slice(input);
        Ok(input.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests;
