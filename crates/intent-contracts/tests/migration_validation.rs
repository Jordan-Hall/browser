use intent_contracts::{
    MAX_MIGRATION_DOCUMENT_BYTES, MAX_MIGRATION_ERROR_BYTES, MAX_MIGRATION_SCHEMAS,
    MigrationFailure, MigrationRegistry, MigrationRegistryError, MigrationStep, RecordFamily,
    SchemaVersion,
};
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

fn v1(input: &[u8]) -> Result<(), MigrationFailure> {
    if input != b"v1" {
        return Err(MigrationFailure::new("expected v1"));
    }
    Ok(())
}
fn v2(input: &[u8]) -> Result<(), MigrationFailure> {
    if input != b"v2" {
        return Err(MigrationFailure::new("expected v2"));
    }
    Ok(())
}
fn nonempty(input: &[u8]) -> Result<(), MigrationFailure> {
    if input.is_empty() {
        return Err(MigrationFailure::new("empty input"));
    }
    Ok(())
}
fn to_v2(_: &[u8]) -> Result<Vec<u8>, MigrationFailure> {
    Ok(b"v2".to_vec())
}
fn invalid_output(_: &[u8]) -> Result<Vec<u8>, MigrationFailure> {
    Ok(b"invalid".to_vec())
}
fn must_not_run(_: &[u8]) -> Result<Vec<u8>, MigrationFailure> {
    Err(MigrationFailure::new("transformation must not run"))
}
fn too_large(_: &[u8]) -> Result<Vec<u8>, MigrationFailure> {
    Ok(vec![0; MAX_MIGRATION_DOCUMENT_BYTES + 1])
}

#[test]
fn same_version_import_requires_a_registered_validator() -> TestResult {
    let family = RecordFamily::try_new("test.record")?;
    let mut registry = MigrationRegistry::new();
    assert!(matches!(
        registry.migrate(&family, SchemaVersion::V1, SchemaVersion::V1, b"v1"),
        Err(MigrationRegistryError::UnregisteredSchema { .. })
    ));
    registry.register_schema(family.clone(), SchemaVersion::V1, v1)?;
    let outcome = registry.migrate(&family, SchemaVersion::V1, SchemaVersion::V1, b"v1")?;
    assert_eq!(outcome.bytes(), b"v1");
    assert!(matches!(
        registry.migrate(&family, SchemaVersion::V1, SchemaVersion::V1, b"garbage"),
        Err(MigrationRegistryError::SchemaValidationFailed {
            version: SchemaVersion::V1,
            ..
        })
    ));
    Ok(())
}

#[test]
fn invalid_source_is_rejected_before_transforming() -> TestResult {
    let family = RecordFamily::try_new("test.record")?;
    let target = SchemaVersion::try_new(1, 1)?;
    let mut registry = MigrationRegistry::new();
    registry.register_schema(family.clone(), SchemaVersion::V1, v1)?;
    registry.register_schema(family.clone(), target, v2)?;
    registry.register(MigrationStep::try_new(
        family.clone(),
        SchemaVersion::V1,
        target,
        must_not_run,
    )?)?;
    assert!(matches!(
        registry.migrate(&family, SchemaVersion::V1, target, b"invalid"),
        Err(MigrationRegistryError::SchemaValidationFailed {
            version: SchemaVersion::V1,
            ..
        })
    ));
    Ok(())
}

#[test]
fn transformed_output_must_satisfy_its_declared_target() -> TestResult {
    let family = RecordFamily::try_new("test.record")?;
    let target = SchemaVersion::try_new(1, 1)?;
    for (transform, succeeds) in [
        (to_v2 as intent_contracts::MigrationFn, true),
        (invalid_output, false),
    ] {
        let mut registry = MigrationRegistry::new();
        registry.register_schema(family.clone(), SchemaVersion::V1, v1)?;
        registry.register_schema(family.clone(), target, v2)?;
        registry.register(MigrationStep::try_new(
            family.clone(),
            SchemaVersion::V1,
            target,
            transform,
        )?)?;
        let source = b"v1".to_vec();
        let result = registry.migrate(&family, SchemaVersion::V1, target, &source);
        assert_eq!(source, b"v1");
        if succeeds {
            let outcome = result?;
            assert_eq!(outcome.bytes(), b"v2");
            assert_eq!(outcome.source_version(), SchemaVersion::V1);
            assert_eq!(outcome.target_version(), target);
        } else {
            assert!(
                matches!(result, Err(MigrationRegistryError::SchemaValidationFailed { version, .. }) if version == target)
            );
        }
    }
    Ok(())
}

#[test]
fn intermediate_output_is_validated_before_the_next_step() -> TestResult {
    let family = RecordFamily::try_new("test.record")?;
    let middle = SchemaVersion::try_new(1, 1)?;
    let target = SchemaVersion::try_new(1, 2)?;
    let mut registry = MigrationRegistry::new();
    registry.register_schema(family.clone(), SchemaVersion::V1, v1)?;
    registry.register_schema(family.clone(), middle, v2)?;
    registry.register_schema(family.clone(), target, nonempty)?;
    registry.register(MigrationStep::try_new(
        family.clone(),
        SchemaVersion::V1,
        middle,
        invalid_output,
    )?)?;
    registry.register(MigrationStep::try_new(
        family.clone(),
        middle,
        target,
        must_not_run,
    )?)?;
    assert!(
        matches!(registry.migrate(&family, SchemaVersion::V1, target, b"v1"),
        Err(MigrationRegistryError::SchemaValidationFailed { version, .. }) if version == middle)
    );
    Ok(())
}

#[test]
fn complete_path_is_checked_before_any_transformation() -> TestResult {
    let family = RecordFamily::try_new("test.record")?;
    let middle = SchemaVersion::try_new(1, 1)?;
    let target = SchemaVersion::try_new(1, 2)?;
    let mut registry = MigrationRegistry::new();
    registry.register_schema(family.clone(), SchemaVersion::V1, v1)?;
    registry.register_schema(family.clone(), target, nonempty)?;
    registry.register(MigrationStep::try_new(
        family.clone(),
        SchemaVersion::V1,
        middle,
        must_not_run,
    )?)?;
    registry.register(MigrationStep::try_new(
        family.clone(),
        middle,
        target,
        must_not_run,
    )?)?;
    assert!(
        matches!(registry.migrate(&family, SchemaVersion::V1, target, b"v1"),
        Err(MigrationRegistryError::UnregisteredSchema { version, .. }) if version == middle)
    );
    let missing_target = SchemaVersion::try_new(1, 3)?;
    assert!(
        matches!(registry.migrate(&family, SchemaVersion::V1, missing_target, b"v1"),
        Err(MigrationRegistryError::UnregisteredSchema { version, .. }) if version == missing_target)
    );
    Ok(())
}

#[test]
fn missing_and_overshooting_paths_never_run_a_transform() -> TestResult {
    let family = RecordFamily::try_new("test.record")?;
    let middle = SchemaVersion::try_new(1, 1)?;
    let target = SchemaVersion::try_new(1, 2)?;
    let mut registry = MigrationRegistry::new();
    for version in [SchemaVersion::V1, middle, target] {
        registry.register_schema(family.clone(), version, nonempty)?;
    }
    assert!(matches!(
        registry.migrate(&family, SchemaVersion::V1, target, b"v1"),
        Err(MigrationRegistryError::MissingStep { .. })
    ));
    registry.register(MigrationStep::try_new(
        family.clone(),
        SchemaVersion::V1,
        target,
        must_not_run,
    )?)?;
    assert!(matches!(
        registry.migrate(&family, SchemaVersion::V1, middle, b"v1"),
        Err(MigrationRegistryError::StepOvershootsTarget { .. })
    ));
    assert!(matches!(
        registry.migrate(&family, target, SchemaVersion::V1, b"v2"),
        Err(MigrationRegistryError::DowngradeForbidden { .. })
    ));
    Ok(())
}

#[test]
fn validators_are_family_specific_and_cannot_be_replaced() -> TestResult {
    let family = RecordFamily::try_new("test.first")?;
    let other = RecordFamily::try_new("test.second")?;
    let mut registry = MigrationRegistry::new();
    registry.register_schema(family.clone(), SchemaVersion::V1, v1)?;
    assert!(matches!(
        registry.register_schema(family.clone(), SchemaVersion::V1, v2),
        Err(MigrationRegistryError::DuplicateSchema { .. })
    ));
    assert!(
        registry
            .migrate(&family, SchemaVersion::V1, SchemaVersion::V1, b"v1")
            .is_ok()
    );
    assert!(matches!(
        registry.migrate(&other, SchemaVersion::V1, SchemaVersion::V1, b"v1"),
        Err(MigrationRegistryError::UnregisteredSchema { .. })
    ));
    Ok(())
}

#[test]
fn input_output_and_same_version_buffers_obey_the_budget() -> TestResult {
    let family = RecordFamily::try_new("test.bytes")?;
    let target = SchemaVersion::try_new(1, 1)?;
    let mut registry = MigrationRegistry::new();
    registry.register_schema(family.clone(), SchemaVersion::V1, nonempty)?;
    registry.register_schema(family.clone(), target, nonempty)?;
    registry.register(MigrationStep::try_new(
        family.clone(),
        SchemaVersion::V1,
        target,
        too_large,
    )?)?;
    let mut bytes = vec![0; MAX_MIGRATION_DOCUMENT_BYTES];
    assert_eq!(
        registry
            .migrate(&family, SchemaVersion::V1, SchemaVersion::V1, &bytes)?
            .bytes(),
        bytes
    );
    bytes.push(0);
    assert!(matches!(
        registry.migrate(&family, SchemaVersion::V1, SchemaVersion::V1, &bytes),
        Err(MigrationRegistryError::DocumentTooLarge { .. })
    ));
    assert!(matches!(
        registry.migrate(&family, SchemaVersion::V1, target, b"v1"),
        Err(MigrationRegistryError::DocumentTooLarge { .. })
    ));
    Ok(())
}

#[test]
fn validator_population_and_diagnostics_are_bounded() -> TestResult {
    let family = RecordFamily::try_new("test.record")?;
    let mut registry = MigrationRegistry::new();
    for minor in 0..MAX_MIGRATION_SCHEMAS {
        registry.register_schema(
            family.clone(),
            SchemaVersion::try_new(1, u16::try_from(minor)?)?,
            nonempty,
        )?;
    }
    assert!(matches!(
        registry.register_schema(family, SchemaVersion::try_new(2, 0)?, nonempty),
        Err(MigrationRegistryError::SchemaRegistryFull)
    ));
    let detail = "a".repeat(MAX_MIGRATION_ERROR_BYTES - 1) + "é remainder";
    let error = MigrationFailure::new(detail);
    assert_eq!(error.detail().len(), MAX_MIGRATION_ERROR_BYTES - 1);
    Ok(())
}
