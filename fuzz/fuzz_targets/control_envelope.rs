#![no_main]

use intent_ipc::{Frame, FrameLane, WireLimits, decode_control};
use libfuzzer_sys::fuzz_target;
use serde_json::Value;

fuzz_target!(|data: &[u8]| {
    let frame = Frame::new(FrameLane::Control, data.to_vec());
    let _ = decode_control::<Value>(&frame, WireLimits::default());
});
