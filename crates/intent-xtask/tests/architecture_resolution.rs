use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn rejects_registry_replacement(
    table: &str,
    optional: bool,
    replacement: &str,
) -> Result<(), Box<dyn Error>> {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "intent-arch-resolution-{}-{unique}-{sequence}",
        std::process::id()
    ));
    fs::create_dir(&path)?;
    let fixture = Fixture(path);
    for package in ["contracts", "provider"] {
        fs::create_dir_all(fixture.0.join(package).join("src"))?;
        fs::write(fixture.0.join(package).join("src/lib.rs"), "")?;
    }
    fs::write(
        fixture.0.join("Cargo.toml"),
        format!("[workspace]\nmembers=[\"contracts\"]\nresolver=\"3\"\n{replacement}\n"),
    )?;
    fs::write(
        fixture.0.join("contracts/Cargo.toml"),
        format!(
            "[package]\nname=\"intent-contracts\"\nversion=\"0.1.0\"\nedition=\"2024\"\n\
             [{table}]\nwire_serde={{package=\"serde\",version=\"=1.0.229\",optional={optional}}}\n"
        ),
    )?;
    fs::write(
        fixture.0.join("provider/Cargo.toml"),
        "[package]\nname=\"serde\"\nversion=\"1.0.229\"\nedition=\"2024\"\n",
    )?;
    let lock = Command::new(env!("CARGO"))
        .current_dir(&fixture.0)
        .args(["generate-lockfile", "--offline"])
        .output()?;
    if !lock.status.success() {
        return Err(io::Error::other(format!(
            "fixture lockfile failed: {}",
            String::from_utf8_lossy(&lock.stderr)
        ))
        .into());
    }
    let before = fs::read(fixture.0.join("Cargo.lock"))?;
    let output = Command::new(env!("CARGO_BIN_EXE_intent-xtask"))
        .current_dir(&fixture.0)
        .arg("arch-check")
        .output()?;
    assert!(
        !output.status.success(),
        "checker accepted a registry declaration resolved to local code: {table}"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("resolved dependency `serde`"), "{stderr}");
    assert_eq!(fs::read(fixture.0.join("Cargo.lock"))?, before);
    Ok(())
}

const PATCH: &str = "[patch.crates-io]\nserde={path=\"provider\"}";

#[test]
fn rejects_patched_normal_dependency() -> Result<(), Box<dyn Error>> {
    rejects_registry_replacement("dependencies", false, PATCH)
}

#[test]
fn rejects_patched_development_dependency() -> Result<(), Box<dyn Error>> {
    rejects_registry_replacement("dev-dependencies", false, PATCH)
}

#[test]
fn rejects_patched_build_dependency() -> Result<(), Box<dyn Error>> {
    rejects_registry_replacement("build-dependencies", false, PATCH)
}

#[test]
fn rejects_patched_inactive_target_dependency() -> Result<(), Box<dyn Error>> {
    rejects_registry_replacement(
        "target.'cfg(target_os = \"none\")'.dependencies",
        false,
        PATCH,
    )
}

#[test]
fn rejects_patched_optional_dependency() -> Result<(), Box<dyn Error>> {
    rejects_registry_replacement("dependencies", true, PATCH)
}

#[test]
fn rejects_legacy_replace_override() -> Result<(), Box<dyn Error>> {
    rejects_registry_replacement(
        "dependencies",
        false,
        "[replace]\n\"serde:1.0.229\"={path=\"provider\"}",
    )
}
