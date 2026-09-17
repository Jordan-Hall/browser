use std::{error::Error, fs, path::PathBuf};
use uuid::Uuid;

pub type TestResult<T = ()> = Result<T, Box<dyn Error>>;

pub struct Profile(PathBuf);
impl Profile {
    pub fn new() -> TestResult<Self> {
        let root = std::env::temp_dir().join(format!("intent-hardening-{}", Uuid::new_v4()));
        fs::create_dir_all(&root)?;
        Ok(Self(root))
    }
    pub fn database(&self) -> PathBuf {
        self.0.join("state.sqlite3")
    }
}
impl Drop for Profile {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
