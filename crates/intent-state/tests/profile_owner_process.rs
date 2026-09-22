#![cfg(all(target_os = "linux", target_env = "gnu"))]

use intent_contracts::UnixTimestampMicros;
use intent_state::RuntimeOwner;
use std::{
    error::Error,
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};
use uuid::Uuid;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const ROOT_ENV: &str = "INTENT_STATE_PROFILE_OWNER_ROOT";
const READY_ENV: &str = "INTENT_STATE_PROFILE_OWNER_READY";

struct TempProfile {
    root: PathBuf,
}

impl TempProfile {
    fn new() -> TestResult<Self> {
        let root = std::env::temp_dir().join(format!(
            "intent-state-profile-owner-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        fs::create_dir(&root)?;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
        Ok(Self { root })
    }

    fn root(&self) -> &Path {
        &self.root
    }
}

impl Drop for TempProfile {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

struct ChildGuard(Child);

impl ChildGuard {
    fn terminate(&mut self) -> TestResult {
        match self.0.try_wait()? {
            Some(_) => Ok(()),
            None => {
                self.0.kill()?;
                let _ = self.0.wait()?;
                Ok(())
            }
        }
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if matches!(self.0.try_wait(), Ok(None)) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}

fn now() -> TestResult<UnixTimestampMicros> {
    Ok(UnixTimestampMicros::try_new(1)?)
}

fn wait_until_ready(child: &mut ChildGuard, ready: &Path) -> TestResult {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if ready.exists() {
            return Ok(());
        }
        if let Some(status) = child.0.try_wait()? {
            return Err(format!("profile owner child exited before readiness: {status}").into());
        }
        if Instant::now() >= deadline {
            return Err("profile owner child did not become ready".into());
        }
        thread::sleep(Duration::from_millis(10));
    }
}

#[test]
#[ignore]
fn profile_owner_child() -> TestResult {
    let root = PathBuf::from(std::env::var_os(ROOT_ENV).ok_or("missing profile root")?);
    let ready = PathBuf::from(std::env::var_os(READY_ENV).ok_or("missing ready path")?);
    let _owner = RuntimeOwner::open_profile(&root, now()?)?;
    fs::write(ready, b"ready")?;
    thread::sleep(Duration::from_secs(60));
    Ok(())
}

#[test]
fn duplicate_process_is_rejected_and_crash_releases_profile_owner() -> TestResult {
    let profile = TempProfile::new()?;
    let ready = profile.root().join("owner.ready");
    let mut child = ChildGuard(
        Command::new(std::env::current_exe()?)
            .args(["--exact", "profile_owner_child", "--ignored", "--nocapture"])
            .env(ROOT_ENV, profile.root())
            .env(READY_ENV, &ready)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?,
    );
    wait_until_ready(&mut child, &ready)?;

    assert!(
        RuntimeOwner::open_profile(profile.root(), now()?).is_err(),
        "a second process unexpectedly acquired the live profile owner guard"
    );

    child.terminate()?;

    let recovered = RuntimeOwner::open_profile(profile.root(), now()?)?;
    drop(recovered);
    let reopened_after_clean_drop = RuntimeOwner::open_profile(profile.root(), now()?)?;
    drop(reopened_after_clean_drop);
    Ok(())
}
