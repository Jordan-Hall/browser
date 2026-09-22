use intent_state::StateStore;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
use intent_contracts::UnixTimestampMicros;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
use intent_state::RuntimeOwner;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use uuid::Uuid;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const ROOT_ENV: &str = "INTENT_STATE_PORTABLE_OWNER_ROOT";
const READY_ENV: &str = "INTENT_STATE_PORTABLE_OWNER_READY";

struct TempProfile {
    root: PathBuf,
}

impl TempProfile {
    fn new() -> TestResult<Self> {
        let root = std::env::temp_dir().join(format!(
            "intent-state-portable-owner-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        fs::create_dir(&root)?;
        #[cfg(unix)]
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

fn wait_until_ready(child: &mut ChildGuard, ready: &Path) -> TestResult {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if ready.exists() {
            return Ok(());
        }
        if let Some(status) = child.0.try_wait()? {
            return Err(format!("portable profile owner child exited before readiness: {status}").into());
        }
        if Instant::now() >= deadline {
            return Err("portable profile owner child did not become ready".into());
        }
        thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
fn now() -> TestResult<UnixTimestampMicros> {
    Ok(UnixTimestampMicros::try_new(1)?)
}

#[test]
#[ignore]
fn portable_profile_owner_child() -> TestResult {
    let root = PathBuf::from(std::env::var_os(ROOT_ENV).ok_or("missing profile root")?);
    let ready = PathBuf::from(std::env::var_os(READY_ENV).ok_or("missing ready path")?);
    let _owner = StateStore::open_owned_profile(&root)?;
    fs::write(ready, b"ready")?;
    thread::sleep(Duration::from_secs(60));
    Ok(())
}

#[test]
fn duplicate_process_and_crash_release_are_portable() -> TestResult {
    let profile = TempProfile::new()?;
    let ready = profile.root().join("owner.ready");
    let mut child = ChildGuard(
        Command::new(std::env::current_exe()?)
            .args([
                "--exact",
                "portable_profile_owner_child",
                "--ignored",
                "--nocapture",
            ])
            .env(ROOT_ENV, profile.root())
            .env(READY_ENV, &ready)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?,
    );
    wait_until_ready(&mut child, &ready)?;

    assert!(
        StateStore::open_owned_profile(profile.root()).is_err(),
        "a second process unexpectedly acquired the portable profile owner"
    );
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    assert!(
        RuntimeOwner::open_profile(profile.root(), now()?).is_err(),
        "Linux RuntimeOwner did not participate in the canonical profile owner lock"
    );

    child.terminate()?;

    let recovered = StateStore::open_owned_profile(profile.root())?;
    let store_id = recovered.store_id()?;
    drop(recovered);

    let reopened = StateStore::open_owned_profile(profile.root())?;
    assert_eq!(reopened.store_id()?, store_id);
    drop(reopened);

    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    {
        let runtime_owner = RuntimeOwner::open_profile(profile.root(), now()?)?;
        assert!(
            StateStore::open_owned_profile(profile.root()).is_err(),
            "portable profile owner did not share the Linux RuntimeOwner lock"
        );
        drop(runtime_owner);
        let portable_after_runtime = StateStore::open_owned_profile(profile.root())?;
        drop(portable_after_runtime);
    }

    Ok(())
}
