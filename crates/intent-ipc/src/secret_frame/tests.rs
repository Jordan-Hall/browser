use super::*;
use crate::{Frame, encode_control};
use intent_contracts::TraceId;
use serde::ser::SerializeSeq;

use crate::erasing_bytes::take_disposals;

fn envelope<T>(payload: T) -> Result<Envelope<T>, Box<dyn std::error::Error>> {
    Ok(Envelope::event(
        "018f47f7-5a86-7c00-8000-000000000501".parse::<TraceId>()?,
        payload,
    ))
}

#[test]
fn secret_frame_matches_existing_wire_and_scrubs_after_write()
-> Result<(), Box<dyn std::error::Error>> {
    let value = envelope(vec![171_u8; 32])?;
    let limits = WireLimits::default();
    let expected = encode_control(&value, limits)?.encode(limits)?;
    let frame = SecretFrame::encode_control(&value, limits)?;
    assert_eq!(frame.as_bytes(), expected);
    take_disposals();
    let mut sent = Vec::new();
    frame.write_all(&mut sent)?;
    assert_eq!(sent, expected);
    assert_eq!(take_disposals(), vec![(expected.len(), true)]);
    Ok(())
}

struct PrefixFailure {
    remaining: usize,
    sent: usize,
}
impl Write for PrefixFailure {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.remaining == 0 {
            return Err(io::ErrorKind::BrokenPipe.into());
        }
        let count = bytes.len().min(self.remaining);
        self.remaining -= count;
        self.sent += count;
        Ok(count)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn partial_write_error_scrubs_the_entire_owned_frame() -> Result<(), Box<dyn std::error::Error>> {
    let frame = SecretFrame::encode_control(&envelope(vec![171_u8; 32])?, WireLimits::default())?;
    let length = frame.as_bytes().len();
    let mut writer = PrefixFailure {
        remaining: 17,
        sent: 0,
    };
    take_disposals();
    assert_eq!(
        frame.write_all(&mut writer).err().map(|error| error.kind()),
        Some(io::ErrorKind::BrokenPipe)
    );
    assert_eq!(writer.sent, 17);
    assert_eq!(take_disposals(), vec![(length, true)]);
    Ok(())
}

struct FailingPayload;
impl Serialize for FailingPayload {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(None)?;
        sequence.serialize_element(&171_u8)?;
        Err(serde::ser::Error::custom("injected serialization failure"))
    }
}

#[test]
fn serialization_failure_scrubs_the_written_prefix() -> Result<(), Box<dyn std::error::Error>> {
    take_disposals();
    assert!(
        SecretFrame::encode_control(&envelope(FailingPayload)?, WireLimits::default()).is_err()
    );
    let disposed = take_disposals();
    assert_eq!(disposed.len(), 1);
    assert!(disposed[0].0 > FRAME_HEADER_BYTES && disposed[0].1);
    Ok(())
}

struct IgnoredLimit;
impl Serialize for IgnoredLimit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(None)?;
        let _ = sequence.serialize_element(&vec![171_u8; 1024]);
        sequence.end()
    }
}

#[test]
fn serializer_cannot_ignore_the_output_limit() -> Result<(), Box<dyn std::error::Error>> {
    let limits = WireLimits {
        max_control_frame_bytes: 256,
        ..WireLimits::default()
    };
    take_disposals();
    let result = SecretFrame::encode_control(&envelope(IgnoredLimit)?, limits);
    assert!(matches!(result, Err(error) if error.code() == WireErrorCode::FrameTooLarge));
    let disposed = take_disposals();
    assert_eq!(disposed.len(), 1);
    assert!(disposed[0].0 <= 256 + FRAME_HEADER_BYTES && disposed[0].1);
    Ok(())
}

#[test]
fn fixed_writer_never_reallocates_and_exact_bound_matches_frame_encoder()
-> Result<(), Box<dyn std::error::Error>> {
    let value = envelope(vec![171_u8; 32])?;
    let ordinary: Frame = encode_control(&value, WireLimits::default())?;
    let limits = WireLimits {
        max_control_frame_bytes: ordinary.payload().len(),
        ..WireLimits::default()
    };
    assert_eq!(
        SecretFrame::encode_control(&value, limits)?.as_bytes(),
        ordinary.encode(limits)?
    );
    let smaller = WireLimits {
        max_control_frame_bytes: ordinary.payload().len() - 1,
        ..limits
    };
    assert!(
        matches!(SecretFrame::encode_control(&value, smaller), Err(error) if error.code() == WireErrorCode::FrameTooLarge)
    );
    let mut bytes = Vec::with_capacity(64);
    let pointer = bytes.as_ptr();
    let capacity = bytes.capacity();
    let mut writer = FixedWriter {
        bytes: &mut bytes,
        capacity: 64,
        exceeded: false,
    };
    for _ in 0..64 {
        writer.write_all(&[171])?;
    }
    assert!(writer.write_all(&[171]).is_err());
    assert_eq!(bytes.as_ptr(), pointer);
    assert_eq!(bytes.capacity(), capacity);
    Ok(())
}
