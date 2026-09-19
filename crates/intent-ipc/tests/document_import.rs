use intent_contracts::SchemaVersion;
use intent_ipc::{
    CoreDocumentImport, CoreRecordKind, WireErrorCode, WireLimits, decode_core_record,
    import_core_document,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::error::Error;

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
fn current_documents_use_the_existing_strict_typed_boundary_for_all_families() -> TestResult {
    for fixture in fixtures()? {
        for value in [fixture.full, fixture.minimal] {
            let bytes = serde_json::to_vec(&value)?;
            let imported = import_core_document(fixture.family, &bytes, WireLimits::default())?;
            assert_eq!(imported.kind(), fixture.family);
            assert_eq!(imported.source_version(), SchemaVersion::V1);
            let CoreDocumentImport::Current(record) = &imported else {
                return Err("current schema was not validated".into());
            };
            assert_eq!(
                record,
                &decode_core_record(fixture.family, &bytes, WireLimits::default())?
            );
            assert_eq!(
                decode_core_record(
                    fixture.family,
                    &imported.encode_for_write(WireLimits::default())?,
                    WireLimits::default()
                )?,
                *record
            );
            let mut unknown = value;
            unknown["unrecognized_authority"] = json!(true);
            assert_eq!(
                import_core_document(
                    fixture.family,
                    &serde_json::to_vec(&unknown)?,
                    WireLimits::default()
                )
                .err()
                .ok_or("expected schema rejection")?
                .code(),
                WireErrorCode::InvalidRecord
            );
        }
    }
    Ok(())
}

#[test]
fn newer_minor_documents_preserve_every_byte_without_validating_or_rewriting_fields() -> TestResult
{
    for minor in [1, 2, u16::MAX] {
        let bytes = format!(
            " \r\n{{ \"unknown\": {{\"new_permission\":false}},\"schema_version\":{{\"minor\":{minor},\"major\":1}},\"opaque\":\"\\u0061 λ\",\"number\":1.00e+1 }}\t\n"
        ).into_bytes();
        for kind in CoreRecordKind::ALL {
            let mut input = bytes.clone();
            let imported = import_core_document(*kind, &input, WireLimits::default())?;
            input.fill(b'!');
            assert_eq!(imported.kind(), *kind);
            assert_eq!(imported.source_version(), SchemaVersion::try_new(1, minor)?);
            let CoreDocumentImport::ReadOnlyNewerMinor(preserved) = &imported else {
                return Err("future schema became a typed current record".into());
            };
            assert_eq!(preserved.kind(), *kind);
            assert_eq!(
                preserved.source_version(),
                SchemaVersion::try_new(1, minor)?
            );
            assert_eq!(preserved.original_bytes(), bytes);
            assert_eq!(preserved.clone().original_bytes(), bytes);
            assert_eq!(
                imported
                    .encode_for_write(WireLimits::default())
                    .err()
                    .ok_or("expected schema rejection")?
                    .code(),
                WireErrorCode::UnsupportedSchema
            );
            assert_eq!(preserved.original_bytes(), bytes);
            assert_eq!(
                decode_core_record(*kind, &bytes, WireLimits::default())
                    .err()
                    .ok_or("expected schema rejection")?
                    .code(),
                WireErrorCode::UnsupportedSchema
            );
            let reparsed =
                import_core_document(*kind, preserved.original_bytes(), WireLimits::default())?;
            assert_eq!(reparsed, imported);
        }
    }
    Ok(())
}

#[test]
fn only_same_major_newer_minor_can_enter_the_opaque_read_path() -> TestResult {
    for version in [
        json!({"major":2,"minor":0}),
        json!({"major":65535,"minor":65535}),
    ] {
        let bytes = serde_json::to_vec(&json!({"schema_version":version,"task_id":false}))?;
        assert_eq!(
            import_core_document(CoreRecordKind::Task, &bytes, WireLimits::default())
                .err()
                .ok_or("expected schema rejection")?
                .code(),
            WireErrorCode::UnsupportedSchema
        );
    }
    for bytes in [
        br#"{}"#.as_slice(),
        br#"[]"#,
        br#"null"#,
        br#"{"schema_version":null}"#,
        br#"{"schema_version":{"major":0,"minor":1}}"#,
        br#"{"schema_version":{"major":1,"minor":-1}}"#,
        br#"{"schema_version":{"major":1,"minor":65536}}"#,
        br#"{"schema_version":{"major":1,"minor":1,"patch":0}}"#,
        br#"{"schema_version":{"major":1}}"#,
        br#"{"schema_version":{"major":1,"minor":0},"task_id":false}"#,
    ] {
        assert_eq!(
            import_core_document(CoreRecordKind::Task, bytes, WireLimits::default())
                .err()
                .ok_or("expected schema rejection")?
                .code(),
            WireErrorCode::InvalidRecord
        );
    }
    Ok(())
}

#[test]
fn opaque_preservation_never_bypasses_duplicate_key_or_json_validation() {
    for input in [
        br#"{"schema_version":{"major":1,"minor":1},"schema_version":{"major":1,"minor":0}}"#
            .as_slice(),
        br#"{"schema_version":{"major":1,"minor":1},"nested":{"key":1,"k\u0065y":2}}"#,
        br#"{"schema_version":{"major":1,"minor":1,"minor":2}}"#,
        br#"{"schema_version":{"major":1,"minor":1}} {}"#,
        br#"{"schema_version":{"major":1,"minor":1},"bad":"\uD800"}"#,
        br#"{"schema_version":{"major":1,"minor":1},"bad":NaN}"#,
        b"{\"schema_version\":{\"major\":1,\"minor\":1},\"bad\":\"\xff\"}",
    ] {
        assert!(import_core_document(CoreRecordKind::Task, input, WireLimits::default()).is_err());
    }
}

#[test]
fn every_document_budget_applies_before_future_bytes_are_retained() {
    let input = br#"{"schema_version":{"major":1,"minor":1},"extra":[[0],[1]]}"#;
    let limits = WireLimits {
        max_control_frame_bytes: input.len(),
        max_json_depth: 4,
        max_collection_entries: 2,
        max_json_nodes: 9,
        ..WireLimits::default()
    };
    assert!(import_core_document(CoreRecordKind::Task, input, limits).is_ok());
    for smaller in [
        WireLimits {
            max_control_frame_bytes: input.len() - 1,
            ..limits
        },
        WireLimits {
            max_json_depth: 3,
            ..limits
        },
        WireLimits {
            max_collection_entries: 1,
            ..limits
        },
        WireLimits {
            max_json_nodes: 8,
            ..limits
        },
    ] {
        assert!(import_core_document(CoreRecordKind::Task, input, smaller).is_err());
    }
    for empty in [
        WireLimits {
            max_control_frame_bytes: 0,
            ..limits
        },
        WireLimits {
            max_json_depth: 0,
            ..limits
        },
        WireLimits {
            max_collection_entries: 0,
            ..limits
        },
        WireLimits {
            max_json_nodes: 0,
            ..limits
        },
    ] {
        assert!(import_core_document(CoreRecordKind::Task, input, empty).is_err());
    }
}

#[test]
fn opaque_diagnostics_do_not_reveal_unknown_fields_or_document_contents() -> TestResult {
    let marker = "private-document-content-not-for-logs";
    let input = serde_json::to_vec(&json!({"schema_version":{"major":1,"minor":1},marker:marker}))?;
    let imported = import_core_document(CoreRecordKind::Approval, &input, WireLimits::default())?;
    let CoreDocumentImport::ReadOnlyNewerMinor(preserved) = &imported else {
        return Err("expected opaque document".into());
    };
    for diagnostic in [
        format!("{imported:?}"),
        format!("{preserved:#?}"),
        imported
            .encode_for_write(WireLimits::default())
            .err()
            .ok_or("expected schema rejection")?
            .to_string(),
    ] {
        assert!(!diagnostic.contains(marker));
        assert!(diagnostic.len() < 512);
    }
    Ok(())
}
