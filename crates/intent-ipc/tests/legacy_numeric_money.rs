use intent_contracts::SchemaVersion;
use intent_ipc::{
    CoreRecord, CoreRecordKind, WireErrorCode, WireLimits, decode_core_record,
    migrate_legacy_numeric_money_goal_v1,
};
use serde_json::{Value, json};
use std::error::Error;

const GOAL_ID: &str = "018f47f7-5a86-7c00-8000-000000000001";

fn legacy_goal(minor_units: &str) -> Vec<u8> {
    format!(
        r#"{{"schema_version":{{"major":1,"minor":0}},"id":"{GOAL_ID}","original_request":"legacy","inference_mode":"offline","budget":{{"currency":"GBP","minor_units":{minor_units},"scale":{{"known":2}}}},"success_predicate":"migrate exactly","approval_requirement":"always"}}"#
    )
    .into_bytes()
}

#[test]
fn numeric_money_is_rejected_by_current_codec_but_explicitly_migrates() -> Result<(), Box<dyn Error>>
{
    let source = legacy_goal("12345");
    assert!(matches!(
        decode_core_record(CoreRecordKind::GoalContract, &source, WireLimits::default()),
        Err(error) if error.code() == WireErrorCode::InvalidRecord
    ));

    let imported = migrate_legacy_numeric_money_goal_v1(&source, WireLimits::default())?;
    assert_eq!(imported.source_version(), SchemaVersion::V1);
    assert_eq!(imported.target_version(), SchemaVersion::V1);
    assert!(matches!(imported.record(), CoreRecord::GoalContract(_)));
    let canonical: Value = serde_json::from_slice(imported.canonical_bytes())?;
    assert_eq!(canonical["budget"]["minor_units"], json!("12345"));
    assert_eq!(source, legacy_goal("12345"), "source bytes were mutated");
    Ok(())
}

#[test]
fn full_i128_legacy_integer_domain_is_converted_without_json_number_rounding()
-> Result<(), Box<dyn Error>> {
    for amount in [i128::MIN, -1, 0, 1, i128::MAX] {
        let source = legacy_goal(&amount.to_string());
        let imported = migrate_legacy_numeric_money_goal_v1(&source, WireLimits::default())?;
        assert!(matches!(imported.record(), CoreRecord::GoalContract(_)));
        let canonical: Value = serde_json::from_slice(imported.canonical_bytes())?;
        assert_eq!(
            canonical["budget"]["minor_units"],
            Value::String(amount.to_string())
        );
    }
    Ok(())
}

#[test]
fn explicit_legacy_path_rejects_current_and_noncanonical_number_spellings() {
    for amount in ["\"12\"", "1.0", "1e2", "-0", "01", "+1"] {
        let result =
            migrate_legacy_numeric_money_goal_v1(&legacy_goal(amount), WireLimits::default());
        assert!(result.is_err(), "accepted legacy minor_units {amount}");
    }
}

#[test]
fn legacy_adapter_does_not_turn_other_record_shapes_into_goal_contracts() {
    let workspace = br#"{"schema_version":{"major":1,"minor":0},"id":"018f47f7-5a86-7c00-8000-000000000002","title":"workspace","state":"active","budget":{"currency":"GBP","minor_units":1,"scale":{"known":2}}}"#;
    assert!(matches!(
        migrate_legacy_numeric_money_goal_v1(workspace, WireLimits::default()),
        Err(error) if error.code() == WireErrorCode::InvalidRecord
    ));
}

#[test]
fn legacy_adapter_requires_the_historical_v1_header_and_an_actual_budget()
-> Result<(), Box<dyn Error>> {
    let no_budget = format!(
        r#"{{"schema_version":{{"major":1,"minor":0}},"id":"{GOAL_ID}","original_request":"legacy","inference_mode":"offline","success_predicate":"migrate exactly","approval_requirement":"always"}}"#
    );
    assert!(matches!(
        migrate_legacy_numeric_money_goal_v1(no_budget.as_bytes(), WireLimits::default()),
        Err(error) if error.code() == WireErrorCode::InvalidRecord
    ));

    let future = String::from_utf8(legacy_goal("1"))?.replace(
        r#""schema_version":{"major":1,"minor":0}"#,
        r#""schema_version":{"major":1,"minor":1}"#,
    );
    assert!(matches!(
        migrate_legacy_numeric_money_goal_v1(future.as_bytes(), WireLimits::default()),
        Err(error) if error.code() == WireErrorCode::UnsupportedSchema
    ));
    Ok(())
}

#[test]
fn conversion_preserves_duplicate_unknown_and_output_byte_limits() -> Result<(), Box<dyn Error>> {
    let duplicate = String::from_utf8(legacy_goal("1"))?.replace(
        r#""original_request":"legacy""#,
        r#""original_request":"legacy","original_request":"other""#,
    );
    assert!(matches!(
        migrate_legacy_numeric_money_goal_v1(duplicate.as_bytes(), WireLimits::default()),
        Err(error) if error.code() == WireErrorCode::DuplicateJsonKey
    ));

    let unknown = String::from_utf8(legacy_goal("1"))?.replace(
        r#""success_predicate":"migrate exactly""#,
        r#""unexpected":true,"success_predicate":"migrate exactly""#,
    );
    assert!(matches!(
        migrate_legacy_numeric_money_goal_v1(unknown.as_bytes(), WireLimits::default()),
        Err(error) if error.code() == WireErrorCode::InvalidRecord
    ));

    let source = legacy_goal("1");
    let limits = WireLimits {
        max_control_frame_bytes: source.len() + 1,
        ..WireLimits::default()
    };
    assert!(matches!(
        migrate_legacy_numeric_money_goal_v1(&source, limits),
        Err(error) if error.code() == WireErrorCode::FrameTooLarge
    ));
    Ok(())
}
