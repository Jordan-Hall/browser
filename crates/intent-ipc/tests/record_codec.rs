use intent_contracts::{MigrationRegistry, MigrationRegistryError, SchemaVersion};
use intent_ipc::{
    CoreRecordKind, Envelope, WireErrorCode, WireLimits, decode_control, decode_core_record,
    encode_control,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::HashSet, error::Error};

type TestResult = Result<(), Box<dyn Error>>;
#[derive(Deserialize)]
struct Fixture {
    family: CoreRecordKind,
    full: Value,
    minimal: Value,
}
fn fixtures() -> Result<Vec<Fixture>, serde_json::Error> {
    serde_json::from_str(include_str!("../../../fixtures/core/records-v1.json"))
}

#[test]
fn golden_fixtures_cover_every_implemented_record_family() -> TestResult {
    let fixtures = fixtures()?;
    let kinds: HashSet<_> = fixtures.iter().map(|fixture| fixture.family).collect();
    assert_eq!(kinds.len(), fixtures.len());
    assert_eq!(kinds, CoreRecordKind::ALL.iter().copied().collect());
    assert_eq!(
        fixtures
            .iter()
            .map(|fixture| fixture.family)
            .collect::<Vec<_>>(),
        CoreRecordKind::ALL,
        "fixture order must match the fuzz selector catalogue"
    );
    for fixture in fixtures {
        let encoded = serde_json::to_vec(&fixture.full)?;
        let record = decode_core_record(fixture.family, &encoded, WireLimits::default())?;
        assert_eq!(record.kind(), fixture.family);
        assert_eq!(
            serde_json::from_slice::<Value>(&record.encode(WireLimits::default())?)?,
            fixture.full
        );
        let minimal = serde_json::to_vec(&fixture.minimal)?;
        let record = decode_core_record(fixture.family, &minimal, WireLimits::default())?;
        assert_eq!(
            decode_core_record(
                fixture.family,
                &record.encode(WireLimits::default())?,
                WireLimits::default()
            )?,
            record
        );
    }
    Ok(())
}

#[test]
fn all_record_families_roundtrip_through_envelopes_without_losing_scope_or_unknown_outcomes()
-> TestResult {
    let trace = "018f47f7-5a86-7c00-8000-000000000099".parse()?;
    for fixture in fixtures()? {
        let envelope = Envelope::event(trace, fixture.full.clone());
        let frame = encode_control(&envelope, WireLimits::default())?;
        let decoded: Envelope<Value> = decode_control(&frame, WireLimits::default())?;
        assert_eq!(decoded.payload(), &fixture.full);
        let record = decode_core_record(
            fixture.family,
            &serde_json::to_vec(decoded.payload())?,
            WireLimits::default(),
        )?;
        assert_eq!(
            serde_json::from_slice::<Value>(&record.encode(WireLimits::default())?)?,
            fixture.full
        );
    }
    Ok(())
}

#[test]
fn all_equal_version_imports_use_registered_record_validators() -> TestResult {
    let mut registry = MigrationRegistry::new();
    for kind in CoreRecordKind::ALL {
        registry.register_schema(kind.record_family()?, SchemaVersion::V1, kind.validator())?;
    }
    for fixture in fixtures()? {
        let family = fixture.family.record_family()?;
        let bytes = serde_json::to_vec(&fixture.full)?;
        assert_eq!(
            registry
                .migrate(&family, SchemaVersion::V1, SchemaVersion::V1, &bytes)?
                .bytes(),
            bytes
        );
        let mut unknown = fixture.full;
        unknown["authority_override"] = json!(true);
        assert!(matches!(
            registry.migrate(
                &family,
                SchemaVersion::V1,
                SchemaVersion::V1,
                &serde_json::to_vec(&unknown)?
            ),
            Err(MigrationRegistryError::SchemaValidationFailed { .. })
        ));
    }
    Ok(())
}

#[test]
fn execution_outcomes_survive_canonical_import_envelopes_and_strict_validation() -> TestResult {
    let fixtures: Vec<Value> = serde_json::from_str(include_str!(
        "../../../fixtures/core/execution-records-v1.json"
    ))?;
    let kind = CoreRecordKind::Operation;
    let mut registry = MigrationRegistry::new();
    let family = kind.record_family()?;
    registry.register_schema(family.clone(), SchemaVersion::V1, kind.validator())?;
    let trace = "018f47f7-5a86-7c00-8000-000000000099".parse()?;
    let mut stages = HashSet::new();
    for fixture in fixtures {
        stages.insert(
            fixture["state"]["details"]["observation"]["stage"]
                .as_str()
                .ok_or("stage")?
                .to_owned(),
        );
        let source = serde_json::to_vec(&fixture)?;
        assert_eq!(
            registry
                .migrate(&family, SchemaVersion::V1, SchemaVersion::V1, &source)?
                .bytes(),
            source
        );
        let imported = intent_ipc::import_core_document(kind, &source, WireLimits::default())?;
        let canonical = imported.encode_for_write(WireLimits::default())?;
        assert_eq!(serde_json::from_slice::<Value>(&canonical)?, fixture);
        let frame = encode_control(
            &Envelope::event(trace, fixture.clone()),
            WireLimits::default(),
        )?;
        let envelope: Envelope<Value> = decode_control(&frame, WireLimits::default())?;
        let decoded = decode_core_record(
            kind,
            &serde_json::to_vec(envelope.payload())?,
            WireLimits::default(),
        )?;
        assert_eq!(
            serde_json::from_slice::<Value>(&decoded.encode(WireLimits::default())?)?,
            fixture
        );
        let mut paths = Vec::new();
        object_paths(&fixture, "", &mut paths);
        for path in paths {
            let mut invalid = fixture.clone();
            invalid.pointer_mut(&path).ok_or("path")?["authority_override"] = json!(true);
            assert!(
                intent_ipc::import_core_document(
                    kind,
                    &serde_json::to_vec(&invalid)?,
                    WireLimits::default()
                )
                .is_err(),
                "accepted unknown field at {path}"
            );
        }
        for (path, value) in [
            ("/idempotency_key", json!("invented")),
            ("/state/details/observation/evidence", Value::Null),
        ] {
            if path.ends_with("evidence")
                && fixture["state"]["details"]["observation"]["stage"] != "verified"
            {
                continue;
            }
            let mut invalid = fixture.clone();
            if path == "/idempotency_key" {
                invalid["idempotency_key"] = value;
            } else {
                *invalid.pointer_mut(path).ok_or("path")? = value;
            }
            assert!(
                intent_ipc::import_core_document(
                    kind,
                    &serde_json::to_vec(&invalid)?,
                    WireLimits::default()
                )
                .is_err()
            );
        }
    }
    assert_eq!(
        stages,
        [
            "accepted",
            "needs_reconciliation",
            "verified",
            "compensated"
        ]
        .map(str::to_owned)
        .into_iter()
        .collect()
    );
    Ok(())
}

fn object_paths(value: &Value, prefix: &str, output: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            output.push(prefix.to_owned());
            for (key, value) in map {
                let key = key.replace('~', "~0").replace('/', "~1");
                object_paths(value, &format!("{prefix}/{key}"), output);
            }
        }
        Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                object_paths(value, &format!("{prefix}/{index}"), output);
            }
        }
        _ => {}
    }
}

#[test]
fn unknown_fields_are_rejected_at_every_record_and_nested_object() -> TestResult {
    for fixture in fixtures()? {
        let mut paths = Vec::new();
        object_paths(&fixture.full, "", &mut paths);
        for path in paths {
            let mut changed = fixture.full.clone();
            changed.pointer_mut(&path).ok_or("missing fixture path")?["authority_override"] =
                json!(true);
            assert!(
                decode_core_record(
                    fixture.family,
                    &serde_json::to_vec(&changed)?,
                    WireLimits::default()
                )
                .is_err(),
                "accepted unsupported {} field at {path}",
                fixture.family.family_name()
            );
        }
    }
    Ok(())
}

#[test]
fn missing_malformed_and_future_schemas_are_distinct_from_typed_payload_errors() -> TestResult {
    for kind in CoreRecordKind::ALL {
        for bytes in [
            b"{}".as_slice(),
            br#"{"schema_version":null}"#,
            br#"{"schema_version":{"major":0,"minor":0}}"#,
            br#"{"schema_version":{"major":1,"minor":0,"patch":1}}"#,
        ] {
            assert_eq!(
                decode_core_record(*kind, bytes, WireLimits::default())
                    .err()
                    .ok_or("unexpected success")?
                    .code(),
                WireErrorCode::InvalidRecord
            );
        }
        for version in [SchemaVersion::try_new(1, 1)?, SchemaVersion::try_new(2, 0)?] {
            let bytes = serde_json::to_vec(&json!({"schema_version":version,"id":false}))?;
            assert_eq!(
                decode_core_record(*kind, &bytes, WireLimits::default())
                    .err()
                    .ok_or("unexpected success")?
                    .code(),
                WireErrorCode::UnsupportedSchema
            );
        }
    }
    Ok(())
}

#[test]
fn duplicate_keys_trailing_values_and_shape_budgets_apply_to_record_imports() -> TestResult {
    let fixture = fixtures()?.remove(0);
    let bytes = serde_json::to_vec(&fixture.full)?;
    let text = std::str::from_utf8(&bytes)?;
    let duplicate = format!("{{\"id\":null,{}", &text[1..]);
    let escaped = format!("{{\"\\u0069d\":null,{}", &text[1..]);
    for invalid in [duplicate, escaped] {
        assert_eq!(
            decode_core_record(fixture.family, invalid.as_bytes(), WireLimits::default())
                .err()
                .ok_or("unexpected success")?
                .code(),
            WireErrorCode::DuplicateJsonKey
        );
    }
    assert_eq!(
        decode_core_record(
            fixture.family,
            format!("{text} {{}}").as_bytes(),
            WireLimits::default()
        )
        .err()
        .ok_or("unexpected success")?
        .code(),
        WireErrorCode::MalformedJson
    );
    let mut limits = WireLimits {
        max_control_frame_bytes: bytes.len(),
        ..WireLimits::default()
    };
    assert!(decode_core_record(fixture.family, &bytes, limits).is_ok());
    limits.max_control_frame_bytes -= 1;
    assert_eq!(
        decode_core_record(fixture.family, &bytes, limits)
            .err()
            .ok_or("unexpected success")?
            .code(),
        WireErrorCode::FrameTooLarge
    );
    for (limits, expected) in [
        (
            WireLimits {
                max_json_nodes: 1,
                ..WireLimits::default()
            },
            WireErrorCode::JsonNodeLimitExceeded,
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
                max_json_depth: 1,
                ..WireLimits::default()
            },
            WireErrorCode::JsonTooDeep,
        ),
    ] {
        assert_eq!(
            decode_core_record(fixture.family, &bytes, limits)
                .err()
                .ok_or("unexpected success")?
                .code(),
            expected
        );
    }
    Ok(())
}

#[test]
fn mismatched_proposal_hash_and_unknown_state_cannot_be_imported() -> TestResult {
    for mut fixture in fixtures()? {
        fixture.full["id"] = json!("not-an-id");
        assert!(
            decode_core_record(
                fixture.family,
                &serde_json::to_vec(&fixture.full)?,
                WireLimits::default()
            )
            .is_err()
        );
    }
    for mut fixture in fixtures()? {
        if fixture.family == CoreRecordKind::ActionProposal {
            fixture.full["arguments_hash"] = json!("00".repeat(32));
            assert!(
                decode_core_record(
                    fixture.family,
                    &serde_json::to_vec(&fixture.full)?,
                    WireLimits::default()
                )
                .is_err()
            );
        }
        if fixture.family == CoreRecordKind::Operation {
            fixture.full["state"]["state"] = json!("assume_success");
            assert!(
                decode_core_record(
                    fixture.family,
                    &serde_json::to_vec(&fixture.full)?,
                    WireLimits::default()
                )
                .is_err()
            );
        }
    }
    Ok(())
}

#[test]
fn wire_error_spelling_is_pinned_and_ambiguous_encodings_fail() -> TestResult {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("../../../fixtures/core/wire-errors-v1.json"))?;
    assert_eq!(cases.len(), 12);
    for case in cases {
        let encoded = serde_json::to_vec(&case["json_name"])?;
        let code: WireErrorCode = serde_json::from_slice(&encoded)?;
        assert_eq!(
            u64::from(code as u16),
            case["diagnostic_code"].as_u64().ok_or("bad code")?
        );
        assert_eq!(serde_json::to_vec(&code)?, encoded);
        assert_eq!(
            code.wire_name(),
            case["json_name"].as_str().ok_or("bad spelling")?
        );
    }
    for invalid in [
        "3",
        "null",
        "true",
        "[]",
        r#"{"MalformedJson":null}"#,
        r#""malformed_json""#,
        r#""UnknownFutureError""#,
    ] {
        assert!(serde_json::from_str::<WireErrorCode>(invalid).is_err());
    }
    assert!(serde_json::from_value::<WireErrorCode>(json!("x".repeat(65))).is_err());
    Ok(())
}
