#![no_main]

use intent_ipc::{FrameDecoder, WireLimits};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let limits = WireLimits {
        max_frames_per_feed: 1,
        ..WireLimits::default()
    };
    let mut decoder = FrameDecoder::new(limits);
    let mut cursor = 0_usize;
    while cursor < data.len() {
        let step = usize::from(data[cursor] % 31) + 1;
        let end = cursor.saturating_add(step).min(data.len());
        match decoder.push(&data[cursor..end]) {
            Ok(batch) => {
                assert!(batch.consumed() > 0 && batch.consumed() <= end - cursor);
                assert!(batch.frames().len() <= limits.max_frames_per_feed);
                cursor += batch.consumed();
            }
            Err(_) => break,
        }
    }
});
