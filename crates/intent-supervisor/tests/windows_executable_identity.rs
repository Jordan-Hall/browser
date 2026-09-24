#![cfg(windows)]

use intent_contracts::{ContentHash, WorkerInstanceId};
use intent_local_transport::WorkerRole;
use intent_supervisor::{SupervisorError, WindowsExecutableIdentity, WindowsPendingWorker};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs::{self, OpenOptions},
    process::{Command, Stdio},
    thread,
    time::Duration,
};
use uuid::Uuid;

type TestResult = Result<(), Box<dyn Error>>;

fn instance() -> Result<WorkerInstanceId, Box<dyn Error>> {
    Ok("018f47f7-5a86-7c00-8000-000000000508".parse()?)
}

#[test]
#[ignore = "launched only by the Windows executable-identity regression"]
fn windows_executable_identity_child() {
    if std::env::var_os("INTENT_WINDOWS_PINNED_PENDING").is_some() {
        thread::sleep(Duration::from_secs(30));
    }
}

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
fn windows_pending_worker_retains_pinned_executable_until_revoke() -> TestResult {
    let directory = std::env::temp_dir().join(format!(
        "intent-supervisor-pinned-launch-{}",
        Uuid::new_v4()
    ));
    fs::create_dir(&directory)?;
    let image_path = directory.join("worker.exe");
    fs::copy(std::env::current_exe()?, &image_path)?;

    let bytes = fs::read(&image_path)?;
    let expected = ContentHash::from_bytes(Sha256::digest(&bytes).into());
    let identity = WindowsExecutableIdentity::load(&image_path, expected)?;
    let mut command = Command::new(identity.path());
    command
        .args([
            "--exact",
            "windows_executable_identity_child",
            "--ignored",
            "--nocapture",
        ])
        .env("INTENT_WINDOWS_PINNED_PENDING", "1")
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let pending = WindowsPendingWorker::spawn_pinned(
        &mut command,
        identity,
        instance()?,
        WorkerRole::BrowserWorker,
        Duration::from_secs(15),
    )?;
    assert_ne!(pending.child_id()?, std::process::id());
    assert_eq!(pending.executable_identity().hash(), expected);
    assert!(OpenOptions::new().write(true).open(&image_path).is_err());
    assert!(fs::rename(&image_path, directory.join("replaced.exe")).is_err());
    assert!(!pending.revoke_before_auth()?.success());

    OpenOptions::new().write(true).open(&image_path)?;
    fs::remove_file(&image_path)?;
    fs::remove_dir(&directory)?;
    Ok(())
}

#[test]
fn windows_pending_worker_rejects_identity_for_different_program() -> TestResult {
    let directory = std::env::temp_dir().join(format!(
        "intent-supervisor-pinned-mismatch-{}",
        Uuid::new_v4()
    ));
    fs::create_dir(&directory)?;
    let image_path = directory.join("worker.exe");
    fs::copy(std::env::current_exe()?, &image_path)?;

    let bytes = fs::read(&image_path)?;
    let expected = ContentHash::from_bytes(Sha256::digest(&bytes).into());
    let identity = WindowsExecutableIdentity::load(&image_path, expected)?;
    let mut command = Command::new(std::env::current_exe()?);
    assert!(matches!(
        WindowsPendingWorker::spawn_pinned(
            &mut command,
            identity,
            instance()?,
            WorkerRole::BrowserWorker,
            Duration::from_secs(15),
        ),
        Err(SupervisorError::ExecutableMismatch)
    ));

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
