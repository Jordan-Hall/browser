use intent_contracts::*;
use intent_ipc::*;
use serde::{Serialize, Serializer, ser::SerializeSeq};
use serde_json::{Value, json};
use std::cell::Cell;
use std::error::Error;

struct InfiniteSequence<'a>(&'a Cell<usize>);

impl Serialize for InfiniteSequence<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(None)?;
        loop {
            self.0.set(self.0.get() + 1);
            sequence.serialize_element(&0_u8)?;
        }
    }
}

#[test]
fn lazy_unbounded_serializer_is_stopped_by_the_output_budget() -> Result<(), Box<dyn Error>> {
    let count = Cell::new(0);
    let value = Envelope::event(
        "018f47f7-5a86-7c00-8000-000000000501".parse()?,
        InfiniteSequence(&count),
    );
    let result = encode_control(&value, WireLimits::for_tests());
    assert!(matches!(result, Err(error) if error.code() == WireErrorCode::FrameTooLarge));
    assert!(count.get() > 0 && count.get() < WireLimits::for_tests().max_control_frame_bytes);
    Ok(())
}

#[test]
fn encoder_accepts_exact_budget_and_rejects_one_byte_less() -> Result<(), Box<dyn Error>> {
    let value = Envelope::event("018f47f7-5a86-7c00-8000-000000000501".parse()?, "small");
    let expected = serde_json::to_vec(&value)?;
    let mut limits = WireLimits::for_tests();
    limits.max_control_frame_bytes = expected.len();
    assert_eq!(encode_control(&value, limits)?.payload(), expected);
    for limit in [0, expected.len() - 1] {
        limits.max_control_frame_bytes = limit;
        assert!(
            matches!(encode_control(&value, limits), Err(error) if error.code() == WireErrorCode::FrameTooLarge)
        );
    }
    Ok(())
}

struct SwallowedWriterError;
impl Serialize for SwallowedWriterError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(None)?;
        let _ = sequence.serialize_element(&"x".repeat(2048));
        sequence.end()
    }
}

#[test]
fn swallowed_writer_error_cannot_return_a_truncated_success() -> Result<(), Box<dyn Error>> {
    let value = Envelope::event(
        "018f47f7-5a86-7c00-8000-000000000501".parse()?,
        SwallowedWriterError,
    );
    assert!(
        matches!(encode_control(&value, WireLimits::for_tests()), Err(error) if error.code() == WireErrorCode::FrameTooLarge)
    );
    Ok(())
}

struct InvalidSerializer;
impl Serialize for InvalidSerializer {
    fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
        Err(serde::ser::Error::custom("invalid source value"))
    }
}

#[test]
fn unrelated_serialization_failure_keeps_its_error_class() -> Result<(), Box<dyn Error>> {
    let value = Envelope::event(
        "018f47f7-5a86-7c00-8000-000000000501".parse()?,
        InvalidSerializer,
    );
    assert!(
        matches!(encode_control(&value, WireLimits::for_tests()), Err(error) if error.code() == WireErrorCode::InvalidEnvelope)
    );
    Ok(())
}

#[test]
fn unsupported_schema_is_classified_before_the_typed_payload() -> Result<(), Box<dyn Error>> {
    let value = Envelope::event(
        "018f47f7-5a86-7c00-8000-000000000501".parse()?,
        json!({"not": "a u64"}),
    );
    let mut wire = serde_json::to_value(value)?;
    wire["schema_version"] = json!({"major": 2, "minor": 0});
    let frame = Frame::new(FrameLane::Control, serde_json::to_vec(&wire)?);
    assert!(
        matches!(decode_control::<u64>(&frame, WireLimits::for_tests()), Err(error) if error.code() == WireErrorCode::UnsupportedSchema)
    );
    wire["schema_version"] = json!(null);
    let frame = Frame::new(FrameLane::Control, serde_json::to_vec(&wire)?);
    assert!(
        matches!(decode_control::<u64>(&frame, WireLimits::for_tests()), Err(error) if error.code() == WireErrorCode::InvalidEnvelope)
    );
    Ok(())
}

#[test]
fn unknown_envelope_authority_fields_are_not_silently_ignored() -> Result<(), Box<dyn Error>> {
    let value = Envelope::event("018f47f7-5a86-7c00-8000-000000000501".parse()?, 7);
    let mut wire = serde_json::to_value(value)?;
    wire["override_deadline"] = json!(true);
    let frame = Frame::new(FrameLane::Control, serde_json::to_vec(&wire)?);
    assert!(
        matches!(decode_control::<u64>(&frame, WireLimits::for_tests()), Err(error) if error.code() == WireErrorCode::InvalidEnvelope)
    );
    Ok(())
}

#[test]
fn event_message_fields_cannot_silently_discard_correlation_or_authority()
-> Result<(), Box<dyn Error>> {
    let value = Envelope::event("018f47f7-5a86-7c00-8000-000000000501".parse()?, 7);
    for (field, extra) in [
        ("request_id", json!("018f47f7-5a86-7c00-8000-000000000502")),
        ("override_deadline", json!(true)),
    ] {
        let mut wire = serde_json::to_value(&value)?;
        wire["message"][field] = extra;
        let frame = Frame::new(FrameLane::Control, serde_json::to_vec(&wire)?);
        assert!(
            matches!(decode_control::<u64>(&frame, WireLimits::for_tests()), Err(error) if error.code() == WireErrorCode::InvalidEnvelope),
            "event accepted unexpected message field {field}"
        );
        assert!(serde_json::from_value::<EnvelopeKind>(wire["message"].clone()).is_err());
    }
    Ok(())
}

#[test]
fn message_kind_json_stays_compatible_and_strict_for_every_variant() -> Result<(), Box<dyn Error>> {
    let request_id: RequestId = "018f47f7-5a86-7c00-8000-000000000502".parse()?;
    for (kind, wire) in [
        (
            EnvelopeKind::Request { request_id },
            json!({"kind":"request","request_id":request_id}),
        ),
        (
            EnvelopeKind::Response { request_id },
            json!({"kind":"response","request_id":request_id}),
        ),
        (EnvelopeKind::Event, json!({"kind":"event"})),
    ] {
        assert_eq!(serde_json::to_value(kind)?, wire);
        assert_eq!(serde_json::from_value::<EnvelopeKind>(wire.clone())?, kind);
        let mut extended = wire;
        extended["override_deadline"] = json!(true);
        assert!(serde_json::from_value::<EnvelopeKind>(extended.clone()).is_err());
        let envelope = json!({
            "schema_version":{"major":1,"minor":0},
            "trace_id":"018f47f7-5a86-7c00-8000-000000000501",
            "message":serde_json::to_value(kind)?,
            "payload":7
        });
        let frame = Frame::new(FrameLane::Control, serde_json::to_vec(&envelope)?);
        let decoded = decode_control::<u64>(&frame, WireLimits::for_tests())?;
        assert_eq!(decoded.message(), kind);
        assert_eq!(decoded.deadline(), None);
        assert_eq!(decoded.cancellation_id(), None);
        let mut invalid = envelope;
        invalid["message"] = extended;
        let frame = Frame::new(FrameLane::Control, serde_json::to_vec(&invalid)?);
        assert!(
            matches!(decode_control::<u64>(&frame, WireLimits::for_tests()), Err(error) if error.code() == WireErrorCode::InvalidEnvelope)
        );
    }
    Ok(())
}

#[test]
fn full_money_domain_roundtrips_through_the_real_envelope_codec() -> Result<(), Box<dyn Error>> {
    for amount in [i128::MIN, -1, 0, 1, (1_i128 << 53) + 1, i128::MAX] {
        let money = Money::new(CurrencyCode::parse("USD")?, amount, CurrencyScale::Unknown);
        let value = Envelope::event("018f47f7-5a86-7c00-8000-000000000501".parse()?, money);
        let frame = encode_control(&value, WireLimits::default())?;
        assert_eq!(
            decode_control::<Money>(&frame, WireLimits::default())?,
            value
        );
    }
    Ok(())
}

#[test]
fn infinite_duplicate_capabilities_are_rejected_after_bounded_consumption()
-> Result<(), Box<dyn Error>> {
    let count = Cell::new(0);
    let capability = ProtocolCapability::try_new("cancel")?;
    let repeated = std::iter::repeat_with(|| {
        count.set(count.get() + 1);
        capability.clone()
    });
    let result = ProtocolOffer::try_new(vec![ProtocolRange::try_new(1, 0, 0)?], repeated);
    assert!(matches!(
        result,
        Err(ProtocolOfferError::TooManyCapabilities(65))
    ));
    assert_eq!(count.get(), MAX_PROTOCOL_CAPABILITIES + 1);
    assert!(
        ProtocolOffer::try_new(
            vec![ProtocolRange::try_new(1, 0, 0)?],
            std::iter::repeat_n(capability, MAX_PROTOCOL_CAPABILITIES)
        )
        .is_ok()
    );
    Ok(())
}

#[test]
fn wire_offer_sequence_limits_include_duplicate_entries() -> Result<(), Box<dyn Error>> {
    let wire = json!({"ranges": [{"major":1,"min_minor":0,"max_minor":0}],
        "capabilities": vec!["cancel"; MAX_PROTOCOL_CAPABILITIES + 1]});
    assert!(serde_json::from_value::<ProtocolOffer>(wire).is_err());
    let wire = json!({"ranges": vec![json!({"major":1,"min_minor":0,"max_minor":0}); MAX_PROTOCOL_RANGES + 1]});
    assert!(serde_json::from_value::<ProtocolOffer>(wire).is_err());
    Ok(())
}

#[test]
fn negotiated_control_codec_never_advertises_unimplemented_versions() -> Result<(), Box<dyn Error>>
{
    for range in [
        ProtocolRange::try_new(1, 1, 1)?,
        ProtocolRange::try_new(2, 0, 0)?,
    ] {
        let remote = ProtocolOffer::try_new(vec![range], [])?;
        assert!(matches!(
            ControlCodec::negotiate(&remote),
            Err(NegotiationError::NoCompatibleVersion)
        ));
    }
    let remote = ProtocolOffer::try_new(vec![ProtocolRange::try_new(1, 0, 5)?], [])?;
    let codec = ControlCodec::negotiate(&remote)?;
    assert_eq!(codec.protocol().version(), SchemaVersion::V1);
    let value = Envelope::event(
        "018f47f7-5a86-7c00-8000-000000000501".parse()?,
        json!({"read":true}),
    );
    let frame = codec.encode(&value, WireLimits::for_tests())?;
    assert_eq!(
        codec.decode::<Value>(&frame, WireLimits::for_tests())?,
        value
    );
    Ok(())
}

#[test]
fn duplicate_keys_are_rejected_before_typed_payload_parsing() -> Result<(), Box<dyn Error>> {
    for payload in [
        r#"{"schema_version":{"major":1,"minor":0},"schema_version":{"major":2,"minor":0}}"#,
        r#"{"payload":{"amount":1,"amount":2}}"#,
        r#"{"payload":{"account":1,"\u0061ccount":2}}"#,
        r#"{"payload":[{"allow":false,"allow":true}]}"#,
    ] {
        let frame = Frame::new(FrameLane::Control, payload.as_bytes().to_vec());
        assert!(
            matches!(decode_control::<Value>(&frame, WireLimits::for_tests()), Err(error) if error.code() == WireErrorCode::DuplicateJsonKey)
        );
    }
    let value = Envelope::event(
        "018f47f7-5a86-7c00-8000-000000000501".parse()?,
        json!({"first":{"id":1},"second":{"id":2}}),
    );
    let frame = encode_control(&value, WireLimits::for_tests())?;
    assert_eq!(
        decode_control::<Value>(&frame, WireLimits::for_tests())?,
        value
    );
    Ok(())
}
