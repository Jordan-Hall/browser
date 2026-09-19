#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![doc = "Bounded Linux worker supervision. Worker results are observations, never authorization or proof of external completion."]

mod admission;
mod error;
mod lease;
mod observation;
mod restart;
mod spec;

pub use admission::*;
pub use error::*;
pub use lease::*;
pub use observation::ProcessObservation;
pub use restart::*;
pub use spec::*;

#[cfg(target_os = "linux")]
#[allow(unsafe_code)]
mod platform;
#[cfg(target_os = "linux")]
mod runtime;
#[cfg(target_os = "linux")]
mod wire;
#[cfg(target_os = "linux")]
pub mod worker;

#[cfg(target_os = "linux")]
pub use platform::ExecutableImage;
#[cfg(target_os = "linux")]
pub use runtime::*;
#[cfg(target_os = "linux")]
pub use wire::{
    BootstrapPacket, ChannelHello, ChannelKind, ControlMessage, ProgressMessage, wire_limits,
};
