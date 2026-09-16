use crate::{BoundedText, BoundedTextError, SchemaVersion};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

pub const MAX_MIGRATION_STEPS: usize = 256;
pub type MigrationFn = fn(&[u8]) -> Result<Vec<u8>, MigrationFailure>;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct RecordFamily(BoundedText<128>);

impl RecordFamily {
    pub fn try_new(value: impl Into<String>) -> Result<Self, BoundedTextError> {
        BoundedText::try_new(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationFailure {
    detail: Box<str>,
}

impl MigrationFailure {
    #[must_use]
    pub fn new(detail: impl Into<Box<str>>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    #[must_use]
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for MigrationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl Error for MigrationFailure {}

#[derive(Clone, Debug)]
pub struct MigrationStep {
    family: RecordFamily,
    from: SchemaVersion,
    to: SchemaVersion,
    migrate: MigrationFn,
}

impl MigrationStep {
    pub fn try_new(
        family: RecordFamily,
        from: SchemaVersion,
        to: SchemaVersion,
        migrate: MigrationFn,
    ) -> Result<Self, MigrationRegistryError> {
        if to <= from {
            return Err(MigrationRegistryError::NonForwardStep { from, to });
        }
        Ok(Self {
            family,
            from,
            to,
            migrate,
        })
    }
}

#[derive(Debug, Default)]
pub struct MigrationRegistry {
    steps: Vec<MigrationStep>,
}

impl MigrationRegistry {
    #[must_use]
    pub const fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn register(&mut self, step: MigrationStep) -> Result<(), MigrationRegistryError> {
        if self.steps.len() >= MAX_MIGRATION_STEPS {
            return Err(MigrationRegistryError::RegistryFull);
        }
        if self
            .steps
            .iter()
            .any(|existing| existing.family == step.family && existing.from == step.from)
        {
            return Err(MigrationRegistryError::AmbiguousSource {
                family: step.family,
                from: step.from,
            });
        }
        self.steps.push(step);
        Ok(())
    }

    pub fn migrate(
        &self,
        family: &RecordFamily,
        source_version: SchemaVersion,
        target_version: SchemaVersion,
        source_bytes: &[u8],
    ) -> Result<MigrationOutcome, MigrationRegistryError> {
        if target_version < source_version {
            return Err(MigrationRegistryError::DowngradeForbidden {
                source: source_version,
                target: target_version,
            });
        }
        if source_version == target_version {
            return Ok(MigrationOutcome {
                source_version,
                target_version,
                bytes: source_bytes.to_vec(),
            });
        }

        let mut current = source_version;
        let mut bytes = source_bytes.to_vec();
        let mut applied_steps = 0_usize;
        while current != target_version {
            if applied_steps >= MAX_MIGRATION_STEPS {
                return Err(MigrationRegistryError::MigrationLoop);
            }
            let step = self
                .steps
                .iter()
                .find(|step| step.family == *family && step.from == current)
                .ok_or_else(|| MigrationRegistryError::MissingStep {
                    family: family.clone(),
                    from: current,
                    target: target_version,
                })?;
            if step.to > target_version {
                return Err(MigrationRegistryError::StepOvershootsTarget {
                    from: step.from,
                    to: step.to,
                    target: target_version,
                });
            }
            bytes =
                (step.migrate)(&bytes).map_err(|failure| MigrationRegistryError::StepFailed {
                    family: family.clone(),
                    from: step.from,
                    to: step.to,
                    detail: failure.detail,
                })?;
            current = step.to;
            applied_steps += 1;
        }

        Ok(MigrationOutcome {
            source_version,
            target_version,
            bytes,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationOutcome {
    source_version: SchemaVersion,
    target_version: SchemaVersion,
    bytes: Vec<u8>,
}

impl MigrationOutcome {
    #[must_use]
    pub const fn source_version(&self) -> SchemaVersion {
        self.source_version
    }

    #[must_use]
    pub const fn target_version(&self) -> SchemaVersion {
        self.target_version
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DocumentAccess {
    ReadWrite,
    MigrationRequired,
    ReadOnlyNewerMinor,
    UnsupportedMajor,
}

#[must_use]
pub fn assess_document_access(
    writer_version: SchemaVersion,
    document_version: SchemaVersion,
) -> DocumentAccess {
    if writer_version.major() == document_version.major() {
        return match writer_version.minor().cmp(&document_version.minor()) {
            std::cmp::Ordering::Equal => DocumentAccess::ReadWrite,
            std::cmp::Ordering::Greater => DocumentAccess::MigrationRequired,
            std::cmp::Ordering::Less => DocumentAccess::ReadOnlyNewerMinor,
        };
    }
    if writer_version.major() > document_version.major() {
        DocumentAccess::MigrationRequired
    } else {
        DocumentAccess::UnsupportedMajor
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MigrationRegistryError {
    RegistryFull,
    NonForwardStep {
        from: SchemaVersion,
        to: SchemaVersion,
    },
    AmbiguousSource {
        family: RecordFamily,
        from: SchemaVersion,
    },
    DowngradeForbidden {
        source: SchemaVersion,
        target: SchemaVersion,
    },
    MissingStep {
        family: RecordFamily,
        from: SchemaVersion,
        target: SchemaVersion,
    },
    StepOvershootsTarget {
        from: SchemaVersion,
        to: SchemaVersion,
        target: SchemaVersion,
    },
    StepFailed {
        family: RecordFamily,
        from: SchemaVersion,
        to: SchemaVersion,
        detail: Box<str>,
    },
    MigrationLoop,
}

impl fmt::Display for MigrationRegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegistryFull => formatter.write_str("migration registry is full"),
            Self::NonForwardStep { from, to } => write!(
                formatter,
                "migration step must move forward, got {}.{} -> {}.{}",
                from.major(),
                from.minor(),
                to.major(),
                to.minor()
            ),
            Self::AmbiguousSource { family, from } => write!(
                formatter,
                "migration family {} already has an outgoing step from {}.{}",
                family.as_str(),
                from.major(),
                from.minor()
            ),
            Self::DowngradeForbidden { source, target } => write!(
                formatter,
                "schema downgrade is forbidden: {}.{} -> {}.{}",
                source.major(),
                source.minor(),
                target.major(),
                target.minor()
            ),
            Self::MissingStep {
                family,
                from,
                target,
            } => write!(
                formatter,
                "no migration step for {} from {}.{} toward {}.{}",
                family.as_str(),
                from.major(),
                from.minor(),
                target.major(),
                target.minor()
            ),
            Self::StepOvershootsTarget { from, to, target } => write!(
                formatter,
                "migration {}.{} -> {}.{} overshoots target {}.{}",
                from.major(),
                from.minor(),
                to.major(),
                to.minor(),
                target.major(),
                target.minor()
            ),
            Self::StepFailed {
                family,
                from,
                to,
                detail,
            } => write!(
                formatter,
                "migration {} {}.{} -> {}.{} failed: {detail}",
                family.as_str(),
                from.major(),
                from.minor(),
                to.major(),
                to.minor()
            ),
            Self::MigrationLoop => formatter.write_str("migration exceeded bounded step count"),
        }
    }
}

impl Error for MigrationRegistryError {}

#[cfg(test)]
mod tests {
    use super::{
        DocumentAccess, MigrationFailure, MigrationRegistry, MigrationStep, RecordFamily,
        assess_document_access,
    };
    use crate::SchemaVersion;
    use serde_json::{Value, json};
    use std::error::Error;

    fn add_display_name(input: &[u8]) -> Result<Vec<u8>, MigrationFailure> {
        let mut value: Value = serde_json::from_slice(input)
            .map_err(|error| MigrationFailure::new(error.to_string()))?;
        value["display_name"] = json!("Imported workspace");
        serde_json::to_vec(&value).map_err(|error| MigrationFailure::new(error.to_string()))
    }

    fn add_revision(input: &[u8]) -> Result<Vec<u8>, MigrationFailure> {
        let mut value: Value = serde_json::from_slice(input)
            .map_err(|error| MigrationFailure::new(error.to_string()))?;
        value["revision"] = json!(1);
        serde_json::to_vec(&value).map_err(|error| MigrationFailure::new(error.to_string()))
    }

    #[test]
    fn migration_preserves_source_version_and_applies_explicit_chain() -> Result<(), Box<dyn Error>>
    {
        let family = RecordFamily::try_new("workspace")?;
        let v1 = SchemaVersion::try_new(1, 0)?;
        let v1_1 = SchemaVersion::try_new(1, 1)?;
        let v1_2 = SchemaVersion::try_new(1, 2)?;
        let mut registry = MigrationRegistry::new();
        registry.register(MigrationStep::try_new(
            family.clone(),
            v1,
            v1_1,
            add_display_name,
        )?)?;
        registry.register(MigrationStep::try_new(
            family.clone(),
            v1_1,
            v1_2,
            add_revision,
        )?)?;

        let outcome = registry.migrate(&family, v1, v1_2, br#"{"id":"workspace-1"}"#)?;
        let value: Value = serde_json::from_slice(outcome.bytes())?;

        assert_eq!(outcome.source_version(), v1);
        assert_eq!(outcome.target_version(), v1_2);
        assert_eq!(value["display_name"], "Imported workspace");
        assert_eq!(value["revision"], 1);
        Ok(())
    }

    #[test]
    fn duplicate_outgoing_steps_are_rejected() -> Result<(), Box<dyn Error>> {
        let family = RecordFamily::try_new("task")?;
        let v1 = SchemaVersion::try_new(1, 0)?;
        let v1_1 = SchemaVersion::try_new(1, 1)?;
        let mut registry = MigrationRegistry::new();
        registry.register(MigrationStep::try_new(
            family.clone(),
            v1,
            v1_1,
            add_revision,
        )?)?;
        assert!(
            registry
                .register(MigrationStep::try_new(family, v1, v1_1, add_revision)?)
                .is_err()
        );
        Ok(())
    }

    #[test]
    fn older_writer_cannot_rewrite_newer_minor_document() -> Result<(), Box<dyn Error>> {
        let old_writer = SchemaVersion::try_new(1, 1)?;
        let newer_document = SchemaVersion::try_new(1, 2)?;
        assert_eq!(
            assess_document_access(old_writer, newer_document),
            DocumentAccess::ReadOnlyNewerMinor
        );
        Ok(())
    }

    #[test]
    fn newer_major_document_is_not_treated_as_writable() -> Result<(), Box<dyn Error>> {
        let old_writer = SchemaVersion::try_new(1, 9)?;
        let newer_document = SchemaVersion::try_new(2, 0)?;
        assert_eq!(
            assess_document_access(old_writer, newer_document),
            DocumentAccess::UnsupportedMajor
        );
        Ok(())
    }
}
