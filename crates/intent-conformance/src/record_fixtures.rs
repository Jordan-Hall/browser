use crate::{ConformanceCheck, ConformanceResult};
use intent_contracts::{MigrationRegistry, SchemaVersion};
use intent_ipc::{CoreRecordKind, WireLimits, decode_core_record};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Fixture {
    family: CoreRecordKind,
    full: Value,
    minimal: Value,
}

pub(crate) fn check() -> ConformanceResult<Vec<ConformanceCheck>> {
    let fixtures: Vec<Fixture> =
        serde_json::from_str(include_str!("../../../fixtures/core/records-v1.json"))?;
    let families: HashSet<_> = fixtures.iter().map(|fixture| fixture.family).collect();
    if families.len() != fixtures.len() || families != CoreRecordKind::ALL.iter().copied().collect()
    {
        return Err("record fixtures must cover every implemented family exactly once".into());
    }
    let mut registry = MigrationRegistry::new();
    for kind in CoreRecordKind::ALL {
        registry.register_schema(kind.record_family()?, SchemaVersion::V1, kind.validator())?;
    }
    let mut checks = Vec::new();
    for fixture in fixtures {
        let kind = fixture.family;
        let family = kind.record_family()?;
        for (index, document) in [&fixture.full, &fixture.minimal].into_iter().enumerate() {
            let source = serde_json::to_vec(document)?;
            let imported =
                registry.migrate(&family, SchemaVersion::V1, SchemaVersion::V1, &source)?;
            if imported.bytes() != source {
                return Err("same-version record import unexpectedly changed source bytes".into());
            }
            let record = decode_core_record(kind, imported.bytes(), WireLimits::default())?;
            let encoded = record.encode(WireLimits::default())?;
            let roundtrip = decode_core_record(kind, &encoded, WireLimits::default())?;
            if roundtrip != record {
                return Err("record changed meaning on roundtrip".into());
            }
            if index == 0 && serde_json::from_slice::<Value>(&encoded)? != fixture.full {
                return Err("full record fixture lost or changed a field".into());
            }
        }
        let mut unsupported = fixture.full;
        unsupported["authority_override"] = Value::Bool(true);
        if registry
            .migrate(
                &family,
                SchemaVersion::V1,
                SchemaVersion::V1,
                &serde_json::to_vec(&unsupported)?,
            )
            .is_ok()
        {
            return Err("unsupported record restriction was silently discarded".into());
        }
        checks.push(ConformanceCheck {
            name: kind.family_name(),
            passed: true,
        });
    }
    Ok(checks)
}
