#![no_main]

use intent_contracts::{MigrationRegistry, SchemaVersion};
use intent_ipc::{
    CoreDocumentImport, CoreRecordKind, WireErrorCode, WireLimits, decode_core_record,
    import_core_document, migrate_legacy_numeric_money_goal_v1,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Some((&selector, input)) = data.split_first() else {
        return;
    };
    let kind = CoreRecordKind::ALL[usize::from(selector) % CoreRecordKind::ALL.len()];
    let limits = WireLimits::default();
    if let Ok(imported) = import_core_document(kind, input, limits) {
        assert_eq!(imported.kind(), kind);
        match &imported {
            CoreDocumentImport::Current(record) => {
                assert_eq!(decode_core_record(kind, input, limits).as_ref(), Ok(record));
            }
            CoreDocumentImport::ReadOnlyNewerMinor(document) => {
                assert_eq!(document.original_bytes(), input);
                assert_eq!(document.source_version().major(), SchemaVersion::V1.major());
                assert!(document.source_version().minor() > SchemaVersion::V1.minor());
                assert_eq!(
                    imported.encode_for_write(limits).unwrap_err().code(),
                    WireErrorCode::UnsupportedSchema
                );
                assert_eq!(
                    decode_core_record(kind, input, limits).unwrap_err().code(),
                    WireErrorCode::UnsupportedSchema
                );
            }
        }
    }
    if kind == CoreRecordKind::GoalContract
        && let Ok(imported) = migrate_legacy_numeric_money_goal_v1(input, limits)
    {
        assert_eq!(imported.record().kind(), CoreRecordKind::GoalContract);
        assert_eq!(
            decode_core_record(
                CoreRecordKind::GoalContract,
                imported.canonical_bytes(),
                limits
            )
            .as_ref(),
            Ok(imported.record())
        );
    }

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
