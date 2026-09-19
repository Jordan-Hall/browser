#![forbid(unsafe_code)]
//! Runtime-owned bridge between durable authorization and authenticated cooperative workers.
//! Worker acknowledgement is transport evidence, never a verified external receipt.
mod error;
mod invocation;
mod record_import;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
mod runtime;
pub use error::*;
pub use invocation::*;
pub use record_import::*;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub use runtime::*;
