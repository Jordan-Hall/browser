#![forbid(unsafe_code)]

use serde::Deserialize;
use std::env;
use std::error::Error;
use std::io;
use std::process::{Command, ExitCode};

const CONTRACTS_PACKAGE: &str = "intent-contracts";
const FORBIDDEN_DEPENDENCY_PREFIXES: &[&str] = &[
    "cef",
    "chromium",
    "gtk",
    "iced",
    "objc",
    "tauri",
    "webkit",
    "windows",
    "winit",
    "intent-agent",
    "intent-browser",
    "intent-gui",
    "intent-os",
    "intent-pc",
    "intent-provider",
    "intent-ui",
    "intent-web",
];

#[derive(Debug, Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    dependencies: Vec<Dependency>,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    name: String,
    path: Option<String>,
    #[serde(default)]
    source: Option<String>,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let command = args.next();

    if args.next().is_some() {
        return Err(io::Error::other("usage: cargo run -p intent-xtask -- arch-check").into());
    }

    match command.as_deref() {
        Some("arch-check") => architecture_check(),
        _ => Err(io::Error::other("usage: cargo run -p intent-xtask -- arch-check").into()),
    }
}

fn architecture_check() -> Result<(), Box<dyn Error>> {
    let output = Command::new("cargo")
        .args(["metadata", "--locked", "--format-version=1", "--no-deps"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::other(format!("cargo metadata failed: {stderr}")).into());
    }

    let metadata: Metadata = serde_json::from_slice(&output.stdout)?;
    validate_contract_dependencies(&metadata)
        .map_err(|violations| io::Error::other(violations.join("\n")))?;

    Ok(())
}

fn validate_contract_dependencies(metadata: &Metadata) -> Result<(), Vec<String>> {
    let Some(contracts) = metadata
        .packages
        .iter()
        .find(|package| package.name == CONTRACTS_PACKAGE)
    else {
        return Err(vec![format!(
            "required package `{CONTRACTS_PACKAGE}` is missing from cargo metadata"
        )]);
    };

    let violations: Vec<String> = contracts
        .dependencies
        .iter()
        .filter(|dependency| is_forbidden_contract_dependency(dependency))
        .map(|dependency| {
            format!(
                "`{CONTRACTS_PACKAGE}` must remain authority-neutral and cannot depend on `{}`",
                dependency.name
            )
        })
        .collect();

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

fn is_forbidden_contract_dependency(dependency: &Dependency) -> bool {
    if dependency.path.is_some()
        || dependency.source.as_deref()
            != Some("registry+https://github.com/rust-lang/crates.io-index")
    {
        return true;
    }

    FORBIDDEN_DEPENDENCY_PREFIXES
        .iter()
        .any(|prefix| dependency.name.starts_with(prefix))
        || !matches!(dependency.name.as_str(), "serde" | "serde_json" | "uuid")
}

#[cfg(test)]
mod tests {
    use super::{Dependency, is_forbidden_contract_dependency};

    #[test]
    fn permits_authority_neutral_serialization_dependency() {
        let dependency = Dependency {
            name: "serde".to_owned(),
            path: None,
            source: Some("registry+https://github.com/rust-lang/crates.io-index".to_owned()),
        };

        assert!(!is_forbidden_contract_dependency(&dependency));
    }

    #[test]
    fn rejects_workspace_dependency_from_contracts() {
        let dependency = Dependency {
            name: "intent-gui".to_owned(),
            path: Some("../intent-gui".to_owned()),
            source: None,
        };

        assert!(is_forbidden_contract_dependency(&dependency));
    }

    #[test]
    fn rejects_browser_engine_dependency_from_contracts() {
        let dependency = Dependency {
            name: "cef-wrapper".to_owned(),
            path: None,
            source: Some("registry+https://github.com/rust-lang/crates.io-index".to_owned()),
        };

        assert!(is_forbidden_contract_dependency(&dependency));
    }
}

#[cfg(test)]
mod hardening_tests {
    use super::{Dependency, is_forbidden_contract_dependency};

    #[test]
    fn rejects_unknown_packages_and_unreviewed_sources() {
        for name in ["egui", "slint", "provider-sdk", "an-unclassified-library"] {
            let dependency = Dependency {
                name: name.to_owned(),
                path: None,
                source: Some("registry+https://github.com/rust-lang/crates.io-index".to_owned()),
            };
            assert!(is_forbidden_contract_dependency(&dependency));
        }
        for source in [
            None,
            Some("git+https://example.invalid/serde"),
            Some("registry+https://example.invalid/index"),
        ] {
            let dependency = Dependency {
                name: "serde".to_owned(),
                path: None,
                source: source.map(str::to_owned),
            };
            assert!(is_forbidden_contract_dependency(&dependency));
        }
    }
}
