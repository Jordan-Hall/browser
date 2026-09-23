#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![doc = "Bounded Linux worker supervision plus native launched-channel authentication primitives on Windows.
Worker results are observations, never authorization or proof of external completion."]

mod admission;
mod error;
mod lease;
mod observation;
mod restart;
mod spec;
#[cfg(windows)]
#[allow(unsafe_code)]
mod windows_launch;

pub use admission::*;
pub use error::*;
pub use lease::*;
pub use observation::ProcessObservation;
pub use restart::*;
pub use spec::*;
#[cfg(windows)]
pub use windows_launch::*;

#[cfg(target_os = "linux")]
#[allow(unsafe_code)]
mod platform;
#[cfg(target_os = "linux")]
mod runtime;
#[cfg(unix)]
mod wire;
#[cfg(unix)]
pub mod worker;

#[cfg(target_os = "linux")]
pub use platform::ExecutableImage;
#[cfg(target_os = "linux")]
pub use runtime::*;
#[cfg(unix)]
pub use wire::{
    BootstrapPacket, ChannelHello, ChannelKind, ControlMessage, ProgressMessage, wire_limits,
};
