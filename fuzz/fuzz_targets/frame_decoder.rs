#![no_main]

use intent_ipc::{FrameDecoder, WireLimits};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let limits = WireLimits::conservative_default();
    let mut decoder = FrameDecoder::new(limits);
    let mut cursor = 0_usize;

    while cursor < data.len() {
        let step = usize::from(data[cursor] % 31).saturating_add(1);
        let end = cursor.saturating_add(step).min(data.len());
        match decoder.push(&data[cursor..end]) {
            Ok(batch) => {
                if batch.consumed() == 0 && end > cursor {
                    break;
                }
            }
            Err(_) => break,
        }
        cursor = end;
    }
});
