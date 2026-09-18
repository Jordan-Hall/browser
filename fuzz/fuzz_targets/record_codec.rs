#![no_main]

use intent_contracts::{MigrationRegistry, SchemaVersion};
use intent_ipc::{CoreRecordKind, WireLimits, decode_core_record};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Some((&selector, input)) = data.split_first() else {
        return;
    };
    let kind = CoreRecordKind::ALL[usize::from(selector) % CoreRecordKind::ALL.len()];
    let limits = WireLimits::default();
    if let Ok(record) = decode_core_record(kind, input, limits) {
        if let Ok(encoded) = record.encode(limits) {
            assert_eq!(
                decode_core_record(kind, &encoded, limits).as_ref(),
                Ok(&record)
            );
        }
        let family = kind.record_family().unwrap();
        let mut registry = MigrationRegistry::new();
        registry
            .register_schema(family.clone(), SchemaVersion::V1, kind.validator())
            .unwrap();
        let outcome = registry
            .migrate(&family, SchemaVersion::V1, SchemaVersion::V1, input)
            .unwrap();
        assert_eq!(outcome.bytes(), input);
    }
});
