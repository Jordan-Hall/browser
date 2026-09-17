use crate::{WireError, WireErrorCode};
use serde::Serialize;
use std::io::{self, Write};

struct BoundedWriter {
    bytes: Vec<u8>,
    limit: usize,
    exceeded: bool,
}

impl Write for BoundedWriter {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        if input.len() > self.limit.saturating_sub(self.bytes.len()) {
            self.exceeded = true;
            return Err(io::Error::other("control envelope exceeds byte limit"));
        }
        let required = self.bytes.len() + input.len();
        if required > self.bytes.capacity() {
            let capacity = self
                .bytes
                .capacity()
                .max(128)
                .saturating_mul(2)
                .min(self.limit)
                .max(required);
            self.bytes
                .try_reserve_exact(capacity - self.bytes.len())
                .map_err(|_| io::Error::other("control buffer allocation failed"))?;
        }
        self.bytes.extend_from_slice(input);
        Ok(input.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(crate) fn encode<T: Serialize>(value: &T, limit: usize) -> Result<Vec<u8>, WireError> {
    let mut writer = BoundedWriter {
        bytes: Vec::new(),
        limit,
        exceeded: false,
    };
    let result = serde_json::to_writer(&mut writer, value);
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
    Ok(writer.bytes)
}
