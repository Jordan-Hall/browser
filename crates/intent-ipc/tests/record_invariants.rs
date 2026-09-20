use intent_contracts::{MAX_RECORD_COLLECTION_ENTRIES, MigrationRegistry, SchemaVersion};
use intent_ipc::{
    CoreRecordKind, Envelope, WireErrorCode, WireLimits, decode_control, decode_core_record,
    encode_control, import_core_document,
};
use serde_json::{Value, json};
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

fn fixture(kind: CoreRecordKind) -> Result<Value, Box<dyn Error>> {
    let fixtures: Vec<Value> =
        serde_json::from_str(include_str!("../../../fixtures/core/records-v1.json"))?;
    let family = serde_json::to_value(kind)?;
    fixtures
        .into_iter()
        .find(|fixture| fixture["family"] == family)
        .map(|fixture| fixture["full"].clone())
        .ok_or_else(|| format!("missing fixture {kind:?}").into())
}

#[test]
fn temporal_and_provenance_failures_are_rejected_by_every_canonical_record_boundary() -> TestResult
{
    for (kind, path) in [
        (CoreRecordKind::Workspace, "/updated_at"),
        (CoreRecordKind::Task, "/updated_at"),
        (CoreRecordKind::MemoryRecord, "/expires_at"),
        (CoreRecordKind::Approval, "/state/details/expires_at"),
        (CoreRecordKind::Evidence, "/origin"),
    ] {
        let mut value = fixture(kind)?;
        let mut registry = MigrationRegistry::new();
        let family = kind.record_family()?;
        registry.register_schema(family.clone(), SchemaVersion::V1, kind.validator())?;
        let valid = serde_json::to_vec(&value)?;
        assert_eq!(
            registry
                .migrate(&family, SchemaVersion::V1, SchemaVersion::V1, &valid)?
                .bytes(),
            valid
        );
        let invalid = match kind {
            CoreRecordKind::Approval => json!(
                value["state"]["details"]["approved_at"]
                    .as_i64()
                    .ok_or("approval time")?
                    - 1
            ),
            CoreRecordKind::Evidence => {
                json!({"kind": "deterministic", "authority_override": true})
            }
            _ => json!(value["created_at"].as_i64().ok_or("creation time")? - 1),
        };
        *value.pointer_mut(path).ok_or("fixture field")? = invalid;
        let bytes = serde_json::to_vec(&value)?;
        assert_eq!(
            decode_core_record(kind, &bytes, WireLimits::default())
                .err()
                .map(|error| error.code()),
            Some(WireErrorCode::InvalidRecord),
            "{kind:?}"
        );
        assert!(import_core_document(kind, &bytes, WireLimits::default()).is_err());
        assert!(
            registry
                .migrate(&family, SchemaVersion::V1, SchemaVersion::V1, &bytes)
                .is_err()
        );
        let envelope = Envelope::event("018f47f7-5a86-7c00-8000-000000000099".parse()?, value);
        let frame = encode_control(&envelope, WireLimits::default())?;
        let decoded: Envelope<Value> = decode_control(&frame, WireLimits::default())?;
        assert_eq!(
            decode_core_record(
                kind,
                &serde_json::to_vec(decoded.payload())?,
                WireLimits::default()
            )
            .err()
            .map(|error| error.code()),
            Some(WireErrorCode::InvalidRecord)
        );
    }
    Ok(())
}

#[test]
fn every_record_list_keeps_its_schema_limit_when_transport_budgets_are_larger() -> TestResult {
    let limits = WireLimits {
        max_control_frame_bytes: 16 * 1024 * 1024,
        max_collection_entries: MAX_RECORD_COLLECTION_ENTRIES + 1,
        max_json_nodes: 1024 * 1024,
        ..WireLimits::default()
    };
    for (kind, field) in [
        (CoreRecordKind::GoalContract, "clarified_constraints"),
        (CoreRecordKind::GoalContract, "authorized_accounts"),
        (CoreRecordKind::Task, "required_capabilities"),
        (CoreRecordKind::Task, "result_artifacts"),
        (CoreRecordKind::Evidence, "source_observations"),
        (CoreRecordKind::Receipt, "evidence_ids"),
        (CoreRecordKind::ViewDefinition, "bindings"),
        (CoreRecordKind::ViewDefinition, "action_capabilities"),
    ] {
        let mut value = fixture(kind)?;
        let item = value[field][0].clone();
        assert!(!item.is_null(), "missing {kind:?}.{field} fixture element");
        value[field] = json!(vec![item.clone(); MAX_RECORD_COLLECTION_ENTRIES]);
        let bytes = serde_json::to_vec(&value)?;
        let imported = import_core_document(kind, &bytes, limits)?;
        assert_eq!(
            serde_json::from_slice::<Value>(&imported.encode_for_write(limits)?)?,
            value
        );
        value[field].as_array_mut().ok_or("collection")?.push(item);
        let bytes = serde_json::to_vec(&value)?;
        assert_eq!(
            decode_core_record(kind, &bytes, limits)
                .err()
                .map(|error| error.code()),
            Some(WireErrorCode::InvalidRecord),
            "{kind:?}.{field}"
        );
        assert!(import_core_document(kind, &bytes, limits).is_err());
    }
    Ok(())
}
