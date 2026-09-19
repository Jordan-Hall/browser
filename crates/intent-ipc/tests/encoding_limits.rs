use intent_contracts::{ApprovalRequirement, BoundedText, GoalContract, InferenceMode};
use intent_ipc::{
    CoreRecord, Envelope, Frame, FrameLane, WireErrorCode, WireLimits, decode_control,
    encode_control,
};
use serde::{Serialize, Serializer, ser::SerializeMap};
use serde_json::{Value, json};
use std::error::Error;

#[test]
fn encoder_and_decoder_agree_on_every_structural_budget() -> Result<(), Box<dyn Error>> {
    let envelope = Envelope::event(
        "018f47f7-5a86-7c00-8000-000000000501".parse()?,
        json!({"items": [1, 2, 3]}),
    );
    let frame = Frame::new(FrameLane::Control, serde_json::to_vec(&envelope)?);
    for limit in 0..32 {
        for budget in 0..3 {
            let mut limits = WireLimits::default();
            match budget {
                0 => limits.max_json_depth = limit,
                1 => limits.max_collection_entries = limit,
                _ => limits.max_json_nodes = limit,
            }
            let decoded = decode_control::<Value>(&frame, limits);
            let encoded = encode_control(&envelope, limits);
            assert_eq!(
                encoded.as_ref().err().map(|error| error.code()),
                decoded.as_ref().err().map(|error| error.code()),
                "budget {budget}, limit {limit}"
            );
            if let Ok(encoded) = encoded {
                assert_eq!(decode_control::<Value>(&encoded, limits)?, envelope);
            }
        }
    }
    Ok(())
}

struct DuplicateKeys;

impl Serialize for DuplicateKeys {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("account", "first")?;
        map.serialize_entry("account", "second")?;
        map.end()
    }
}

#[test]
fn serializer_cannot_emit_ambiguous_duplicate_keys() -> Result<(), Box<dyn Error>> {
    let envelope = Envelope::event(
        "018f47f7-5a86-7c00-8000-000000000501".parse()?,
        DuplicateKeys,
    );
    assert!(
        matches!(encode_control(&envelope, WireLimits::default()), Err(error) if error.code() == WireErrorCode::DuplicateJsonKey)
    );
    Ok(())
}

#[test]
fn record_encoder_enforces_structural_budgets_too() -> Result<(), Box<dyn Error>> {
    let record = CoreRecord::GoalContract(Box::new(GoalContract::new(
        "018f47f7-5a86-7c00-8000-000000000501".parse()?,
        BoundedText::try_new("test")?,
        InferenceMode::Offline,
        BoundedText::try_new("test")?,
        ApprovalRequirement::Always,
    )));
    for (limits, expected) in [
        (
            WireLimits {
                max_json_depth: 1,
                ..WireLimits::default()
            },
            WireErrorCode::JsonTooDeep,
        ),
        (
            WireLimits {
                max_collection_entries: 1,
                ..WireLimits::default()
            },
            WireErrorCode::CollectionTooLarge,
        ),
        (
            WireLimits {
                max_json_nodes: 1,
                ..WireLimits::default()
            },
            WireErrorCode::JsonNodeLimitExceeded,
        ),
    ] {
        assert!(matches!(record.encode(limits), Err(error) if error.code() == expected));
    }
    Ok(())
}
