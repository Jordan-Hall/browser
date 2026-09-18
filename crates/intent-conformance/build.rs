use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let output = Command::new(rustc).arg("--version").output()?;
    if !output.status.success() {
        return Err("rustc --version failed while building conformance report".into());
    }
    let version = String::from_utf8(output.stdout)?;
    println!("cargo:rustc-env=INTENT_RUSTC_VERSION={}", version.trim());
    Ok(())
}
