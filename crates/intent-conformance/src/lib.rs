#![forbid(unsafe_code)]
#![doc = "Deterministic conformance checks for Intent Browser core contracts and local IPC."]

use intent_contracts::{
    Approval, MigrationFailure, MigrationRegistry, MigrationStep, RecordFamily, SchemaVersion,
};
use intent_ipc::{
    NegotiationError, ProtocolCapability, ProtocolOffer, ProtocolRange, negotiate_protocol,
};
use intent_local_transport::{MessageFamily, WorkerRole};
use serde::Serialize;
use std::error::Error;

mod record_fixtures;

pub const CONFORMANCE_FORMAT_VERSION: u16 = 1;
pub type ConformanceResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConformanceCheck {
    name: &'static str,
    passed: bool,
}

impl ConformanceCheck {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConformanceReport {
    format_version: u16,
    rustc_version: &'static str,
    contracts_schema_family: &'static str,
    ipc_schema_family: &'static str,
    current_schema_version: String,
    canonical_protocol_fixture: String,
    checks: Vec<ConformanceCheck>,
}

impl ConformanceReport {
    #[must_use]
    pub fn checks(&self) -> &[ConformanceCheck] {
        &self.checks
    }

    pub fn to_pretty_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderedConformance {
    json: String,
    passed: bool,
}

impl RenderedConformance {
    #[must_use]
    pub fn json(&self) -> &str {
        &self.json
    }

    #[must_use]
    pub const fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ConformanceFailureReport {
    format_version: u16,
    result: &'static str,
    rustc_version: &'static str,
    error: String,
}

pub fn render_conformance_result(
    result: ConformanceResult<ConformanceReport>,
) -> Result<RenderedConformance, serde_json::Error> {
    match result {
        Ok(report) => Ok(RenderedConformance {
            json: report.to_pretty_json()?,
            passed: true,
        }),
        Err(error) => {
            let report = ConformanceFailureReport {
                format_version: CONFORMANCE_FORMAT_VERSION,
                result: "failure",
                rustc_version: env!("INTENT_RUSTC_VERSION"),
                error: error.to_string(),
            };
            Ok(RenderedConformance {
                json: serde_json::to_string_pretty(&report)?,
                passed: false,
            })
        }
    }
}

pub fn run_conformance() -> ConformanceResult<ConformanceReport> {
    let canonical_protocol_fixture = canonical_protocol_fixture()?;
    check_migration_chain()?;
    check_outdated_worker_rejected()?;
    check_role_capability_binding()?;
    check_malformed_approval_rejected()?;
    let record_checks = record_fixtures::check()?;
    let current = SchemaVersion::V1;

    Ok(ConformanceReport {
        format_version: CONFORMANCE_FORMAT_VERSION,
        rustc_version: env!("INTENT_RUSTC_VERSION"),
        contracts_schema_family: intent_contracts::CONTRACTS_SCHEMA_FAMILY,
        ipc_schema_family: intent_ipc::IPC_SCHEMA_FAMILY,
        current_schema_version: format!("{}.{}", current.major(), current.minor()),
        canonical_protocol_fixture,
        checks: {
            let mut checks = vec![
                ConformanceCheck {
                    name: "canonical_protocol_fixture",
                    passed: true,
                },
                ConformanceCheck {
                    name: "migration_chain",
                    passed: true,
                },
                ConformanceCheck {
                    name: "outdated_worker_rejected",
                    passed: true,
                },
                ConformanceCheck {
                    name: "role_capability_binding_negative",
                    passed: true,
                },
                ConformanceCheck {
                    name: "malformed_approval_rejected",
                    passed: true,
                },
            ];
            checks.extend(record_checks);
            checks
        },
    })
}

fn canonical_protocol_fixture() -> ConformanceResult<String> {
    let offer = ProtocolOffer::try_new(
        vec![ProtocolRange::try_new(1, 0, 2)?],
        [
            ProtocolCapability::try_new("zeta")?,
            ProtocolCapability::try_new("alpha")?,
        ],
    )?;
    let encoded = serde_json::to_string(&offer)?;
    const EXPECTED: &str =
        r#"{"ranges":[{"major":1,"min_minor":0,"max_minor":2}],"capabilities":["alpha","zeta"]}"#;
    if encoded != EXPECTED {
        return Err(
            "canonical protocol fixture changed without an explicit conformance update".into(),
        );
    }
    Ok(encoded)
}

fn check_migration_chain() -> ConformanceResult<()> {
    fn migrate_fixture(input: &[u8]) -> Result<Vec<u8>, MigrationFailure> {
        let mut migrated = input.to_vec();
        migrated.extend_from_slice(b"|v1.1");
        Ok(migrated)
    }

    fn validate_source(input: &[u8]) -> Result<(), MigrationFailure> {
        if input != b"v1" {
            return Err(MigrationFailure::new("invalid synthetic v1 fixture"));
        }
        Ok(())
    }
    fn validate_target(input: &[u8]) -> Result<(), MigrationFailure> {
        if input != b"v1|v1.1" {
            return Err(MigrationFailure::new("invalid synthetic v1.1 fixture"));
        }
        Ok(())
    }

    let family = RecordFamily::try_new("intent.conformance.fixture")?;
    let source = SchemaVersion::V1;
    let target = SchemaVersion::try_new(1, 1)?;
    let mut registry = MigrationRegistry::new();
    registry.register_schema(family.clone(), source, validate_source)?;
    registry.register_schema(family.clone(), target, validate_target)?;
    registry.register(MigrationStep::try_new(
        family.clone(),
        source,
        target,
        migrate_fixture,
    )?)?;
    let outcome = registry.migrate(&family, source, target, b"v1")?;
    if outcome.source_version() != source
        || outcome.target_version() != target
        || outcome.bytes() != b"v1|v1.1"
    {
        return Err("migration outcome did not preserve version lineage and bytes".into());
    }
    Ok(())
}

fn check_outdated_worker_rejected() -> ConformanceResult<()> {
    let local = ProtocolOffer::try_new(vec![ProtocolRange::try_new(2, 0, 0)?], [])?;
    let outdated = ProtocolOffer::try_new(vec![ProtocolRange::try_new(1, 0, 9)?], [])?;
    if !matches!(
        negotiate_protocol(&local, &outdated),
        Err(NegotiationError::NoCompatibleVersion)
    ) {
        return Err("incompatible worker protocol was not rejected".into());
    }
    Ok(())
}

fn check_role_capability_binding() -> ConformanceResult<()> {
    if WorkerRole::BrowserWorker.allows(MessageFamily::PolicyDecision) {
        return Err("browser worker was allowed to issue a policy decision".into());
    }
    if !WorkerRole::PolicyBroker.allows(MessageFamily::PolicyDecision) {
        return Err("policy broker lost its declared policy message family".into());
    }
    Ok(())
}

fn check_malformed_approval_rejected() -> ConformanceResult<()> {
    let malformed = r#"{"schema_version":{"major":1,"minor":0}}"#;
    if serde_json::from_str::<Approval>(malformed).is_ok() {
        return Err("malformed approval unexpectedly deserialized".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        CONFORMANCE_FORMAT_VERSION, ConformanceReport, ConformanceResult, render_conformance_result,
        run_conformance,
    };
    use std::error::Error;

    #[test]
    fn report_is_deterministic_and_all_checks_pass() -> Result<(), Box<dyn Error>> {
        let first = run_conformance()?;
        let second = run_conformance()?;
        assert_eq!(first, second);
        assert!(first.checks().iter().all(|check| check.passed()));
        assert_eq!(first.to_pretty_json()?, second.to_pretty_json()?);
        Ok(())
    }

    #[test]
    fn failure_report_is_structured_and_marks_execution_failed() -> Result<(), Box<dyn Error>> {
        let failure: ConformanceResult<ConformanceReport> = Err("synthetic conformance failure".into());
        let rendered = render_conformance_result(failure)?;
        assert!(!rendered.passed());

        let report: serde_json::Value = serde_json::from_str(rendered.json())?;
        assert_eq!(report["format_version"], CONFORMANCE_FORMAT_VERSION);
        assert_eq!(report["result"], "failure");
        assert_eq!(report["rustc_version"], env!("INTENT_RUSTC_VERSION"));
        assert_eq!(report["error"], "synthetic conformance failure");
        Ok(())
    }
}
