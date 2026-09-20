#![allow(unsafe_code)]

use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst},
};

pub(super) struct CountingSystem;

static ACTIVE: AtomicBool = AtomicBool::new(false);
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static OVERLAP: AtomicUsize = AtomicUsize::new(0);
static REQUESTS: AtomicUsize = AtomicUsize::new(0);
static INVALID: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Debug, serde::Serialize)]
pub(super) struct Measurement {
    pub live_bytes: usize,
    pub peak_live_bytes: usize,
    pub peak_realloc_overlap_bytes: usize,
    pub successful_requests: usize,
    pub accounting_invalid: bool,
}

fn add(left: usize, right: usize) -> usize {
    left.checked_add(right).unwrap_or_else(|| {
        INVALID.store(true, SeqCst);
        usize::MAX
    })
}

fn subtract(left: usize, right: usize) -> usize {
    left.checked_sub(right).unwrap_or_else(|| {
        INVALID.store(true, SeqCst);
        0
    })
}

fn replace_request(old_size: usize, new_size: usize) {
    if !ACTIVE.load(SeqCst) {
        return;
    }
    let _ = REQUESTS.fetch_update(SeqCst, SeqCst, |count| Some(add(count, 1)));
    let mut current = LIVE.load(SeqCst);
    loop {
        let next = add(subtract(current, old_size), new_size);
        match LIVE.compare_exchange(current, next, SeqCst, SeqCst) {
            Ok(_) => {
                PEAK.fetch_max(next, SeqCst);
                OVERLAP.fetch_max(add(current, new_size), SeqCst);
                return;
            }
            Err(observed) => current = observed,
        }
    }
}

fn release(size: usize) {
    if ACTIVE.load(SeqCst) {
        let _ = LIVE.fetch_update(SeqCst, SeqCst, |live| Some(subtract(live, size)));
    }
}

// SAFETY: Every pointer and layout is forwarded unchanged to System. Accounting
// uses only nonpanicking atomic operations and never touches allocated memory.
unsafe impl GlobalAlloc for CountingSystem {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: The caller supplies the GlobalAlloc layout contract.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            replace_request(0, layout.size());
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: The caller supplies the GlobalAlloc layout contract.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            replace_request(0, layout.size());
        }
        pointer
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        // SAFETY: The caller supplies a live System allocation and valid new size.
        let next = unsafe { System.realloc(pointer, layout, size) };
        if !next.is_null() {
            replace_request(layout.size(), size);
        }
        next
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: The caller supplies the matching live allocation and layout.
        unsafe { System.dealloc(pointer, layout) };
        release(layout.size());
    }
}

pub(super) fn measure<R: Copy>(body: impl FnOnce() -> R) -> (R, Measurement) {
    assert!(!ACTIVE.load(SeqCst), "nested allocation measurement");
    LIVE.store(0, SeqCst);
    PEAK.store(0, SeqCst);
    OVERLAP.store(0, SeqCst);
    REQUESTS.store(0, SeqCst);
    INVALID.store(false, SeqCst);
    struct Window;
    impl Drop for Window {
        fn drop(&mut self) {
            ACTIVE.store(false, SeqCst);
        }
    }
    let window = Window;
    ACTIVE.store(true, SeqCst);
    let result = body();
    drop(window);
    (
        result,
        Measurement {
            live_bytes: LIVE.load(SeqCst),
            peak_live_bytes: PEAK.load(SeqCst),
            peak_realloc_overlap_bytes: OVERLAP.load(SeqCst),
            successful_requests: REQUESTS.load(SeqCst),
            accounting_invalid: INVALID.load(SeqCst),
        },
    )
}

pub(super) fn verify_accounting() -> Result<(), std::alloc::LayoutError> {
    let initial = Layout::from_size_align(64, 8)?;
    let grown = Layout::from_size_align(128, 8)?;
    let (allocated, measured) = measure(|| {
        // SAFETY: Layouts are nonzero and valid. Each successful allocation is
        // released exactly once using its current layout, including realloc failure.
        unsafe {
            let first = CountingSystem.alloc(initial);
            if first.is_null() {
                return false;
            }
            CountingSystem.dealloc(first, initial);
            let zeroed = CountingSystem.alloc_zeroed(initial);
            if zeroed.is_null() {
                return false;
            }
            let is_zero = std::slice::from_raw_parts(zeroed, initial.size())
                .iter()
                .all(|byte| *byte == 0);
            let next = CountingSystem.realloc(zeroed, initial, grown.size());
            if next.is_null() {
                CountingSystem.dealloc(zeroed, initial);
                return false;
            }
            CountingSystem.dealloc(next, grown);
            is_zero
        }
    });
    assert!(allocated);
    assert!(!measured.accounting_invalid);
    assert_eq!(measured.successful_requests, 3);
    assert_eq!(measured.live_bytes, 0);
    assert_eq!(measured.peak_live_bytes, 128);
    assert_eq!(measured.peak_realloc_overlap_bytes, 192);

    let (_, underflow) = measure(|| release(1));
    assert!(underflow.accounting_invalid);
    let (_, overflow) = measure(|| {
        replace_request(0, usize::MAX);
        replace_request(0, 1);
    });
    assert!(overflow.accounting_invalid);
    Ok(())
}
