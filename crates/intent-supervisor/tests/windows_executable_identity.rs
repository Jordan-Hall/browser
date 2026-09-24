#![cfg(windows)]

use intent_contracts::ContentHash;
use intent_supervisor::{SupervisorError, WindowsExecutableIdentity};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs::{self, OpenOptions},
    process::Command,
};
use uuid::Uuid;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
#[ignore = "launched only by the Windows executable-identity regression"]
fn windows_executable_identity_child() {}

#[test]
fn windows_executable_identity_pins_verified_path_until_release() -> TestResult {
    let directory = std::env::temp_dir().join(format!(
        "intent-supervisor-executable-identity-{}",
        Uuid::new_v4()
    ));
    fs::create_dir(&directory)?;
    let image_path = directory.join("worker.exe");
    fs::copy(std::env::current_exe()?, &image_path)?;

    let bytes = fs::read(&image_path)?;
    let expected = ContentHash::from_bytes(Sha256::digest(&bytes).into());
    let identity = WindowsExecutableIdentity::load(&image_path, expected)?;
    assert_eq!(identity.hash(), expected);
    assert_eq!(identity.path(), fs::canonicalize(&image_path)?);

    assert!(OpenOptions::new().write(true).open(&image_path).is_err());
    assert!(fs::rename(&image_path, directory.join("replaced.exe")).is_err());

    let status = Command::new(identity.path())
        .args([
            "--exact",
            "windows_executable_identity_child",
            "--ignored",
            "--nocapture",
        ])
        .status()?;
    assert!(status.success());

    drop(identity);
    OpenOptions::new().write(true).open(&image_path)?;
    fs::remove_file(&image_path)?;
    fs::remove_dir(&directory)?;
    Ok(())
}

#[test]
fn windows_executable_identity_rejects_unapproved_hash() -> TestResult {
    let directory = std::env::temp_dir().join(format!(
        "intent-supervisor-executable-mismatch-{}",
        Uuid::new_v4()
    ));
    fs::create_dir(&directory)?;
    let image_path = directory.join("worker.exe");
    fs::copy(std::env::current_exe()?, &image_path)?;

    assert!(matches!(
        WindowsExecutableIdentity::load(&image_path, ContentHash::from_bytes([0_u8; 32])),
        Err(SupervisorError::ExecutableMismatch)
    ));

    fs::remove_file(&image_path)?;
    fs::remove_dir(&directory)?;
    Ok(())
}
