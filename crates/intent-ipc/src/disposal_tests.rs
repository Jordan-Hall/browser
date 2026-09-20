use crate::erasing_bytes::take_disposals;
use crate::*;
use intent_contracts::TraceId;
use std::error::Error;

#[test]
fn owned_decode_erases_input_on_success_and_malformed_payload() -> Result<(), Box<dyn Error>> {
    let trace: TraceId = "018f47f7-5a86-7c00-8000-000000000501".parse()?;
    let envelope = Envelope::event(trace, vec![171_u8; 32]);
    let frame = encode_control(&envelope, WireLimits::default())?;
    let length = frame.payload().len();
    take_disposals();
    let decoded: Envelope<Vec<u8>> = decode_control_owned(frame, WireLimits::default())?;
    assert_eq!(decoded.payload(), envelope.payload());
    assert_eq!(take_disposals(), vec![(length, true)]);
    for lane in [FrameLane::Control, FrameLane::Artifact] {
        let bytes = b"{\"payload\":[171,172,".to_vec();
        let length = bytes.len();
        let result =
            decode_control_owned::<Vec<u8>>(Frame::new(lane, bytes), WireLimits::default());
        assert!(result.is_err());
        assert_eq!(take_disposals(), vec![(length, true)]);
    }
    Ok(())
}

#[test]
fn partial_decoder_erases_payload_on_truncated_eof_and_abandonment() -> Result<(), Box<dyn Error>> {
    let limits = WireLimits::default();
    let bytes = Frame::new(FrameLane::Control, vec![171; 32]).encode(limits)?;
    for eof in [true, false] {
        let mut decoder = FrameDecoder::new(limits);
        assert!(
            decoder
                .push(&bytes[..FRAME_HEADER_BYTES + 7])?
                .frames()
                .is_empty()
        );
        assert_eq!(decoder.buffered_payload_len(), 7);
        take_disposals();
        if eof {
            assert!(decoder.finish().is_err());
            assert!(decoder.is_poisoned());
            assert_eq!(decoder.buffered_payload_capacity(), 0);
            assert_eq!(take_disposals(), vec![(7, true)]);
        } else {
            drop(decoder);
            assert_eq!(take_disposals(), vec![(7, true)]);
        }
    }
    Ok(())
}
