use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct Fixture(PathBuf);
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn actual_checker_rejects_target_specific_dependency_renamed_to_an_allowed_name(
) -> Result<(), Box<dyn Error>> {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root =
        Fixture(std::env::temp_dir().join(format!("intent-arch-{}-{unique}", std::process::id())));
    fs::create_dir_all(root.0.join("contracts/src"))?;
    fs::create_dir_all(root.0.join("provider/src"))?;
    fs::write(
        root.0.join("Cargo.toml"),
        "[workspace]\nmembers=[\"contracts\",\"provider\"]\nresolver=\"3\"\n",
    )?;
    fs::write(
        root.0.join("contracts/Cargo.toml"),
        "[package]\nname=\"intent-contracts\"\nversion=\"0.1.0\"\nedition=\"2024\"\n[target.'cfg(any())'.dependencies]\nserde={package=\"provider-fixture\",path=\"../provider\"}\n",
    )?;
    fs::write(
        root.0.join("provider/Cargo.toml"),
        "[package]\nname=\"provider-fixture\"\nversion=\"0.1.0\"\nedition=\"2024\"\n",
    )?;
    fs::write(root.0.join("contracts/src/lib.rs"), "")?;
    fs::write(root.0.join("provider/src/lib.rs"), "")?;
    let status = Command::new(env!("CARGO"))
        .current_dir(&root.0)
        .args(["generate-lockfile", "--offline"])
        .status()?;
    if !status.success() {
        return Err(io::Error::other("failed to generate isolated fixture lockfile").into());
    }
    let output = Command::new(env!("CARGO_BIN_EXE_intent-xtask"))
        .current_dir(&root.0)
        .arg("arch-check")
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("provider-fixture"));
    fs::write(
        root.0.join("contracts/Cargo.toml"),
        "[package]\nname=\"intent-contracts\"\nversion=\"0.1.0\"\nedition=\"2024\"\n",
    )?;
    let status = Command::new(env!("CARGO"))
        .current_dir(&root.0)
        .args(["generate-lockfile", "--offline"])
        .status()?;
    assert!(status.success());
    let output = Command::new(env!("CARGO_BIN_EXE_intent-xtask"))
        .current_dir(&root.0)
        .arg("arch-check")
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}
