#![deny(unsafe_code)]

#[path = "support/allocation_meter.rs"]
mod allocation_meter;

use allocation_meter::{Measurement, measure};
use intent_ipc::{Envelope, WireErrorCode, WireLimits, encode_control};
use serde::{Serialize, Serializer, ser::SerializeSeq};
use std::{cell::Cell, error::Error, hint::black_box};

#[global_allocator]
static ALLOCATOR: allocation_meter::CountingSystem = allocation_meter::CountingSystem;

const ELEMENTS: usize = 1_048_576;
const LIMITS: [usize; 3] = [1_024, 16_384, 65_536];
const ALLOWANCE: usize = 4_096;
const MIN_TRAVERSAL_DIVISOR: usize = 4;

struct FiniteZeros<'a>(&'a Cell<usize>);

impl Serialize for FiniteZeros<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(None)?;
        for _ in 0..ELEMENTS {
            self.0.set(self.0.get() + 1);
            sequence.serialize_element(&0_u8)?;
        }
        sequence.end()
    }
}

#[derive(Serialize)]
struct Sample {
    case: &'static str,
    logical_elements: usize,
    attempted_elements: usize,
    frame_limit: Option<usize>,
    peak_live_ceiling: Option<usize>,
    realloc_overlap_ceiling: Option<usize>,
    error: Option<WireErrorCode>,
    output_bytes: Option<usize>,
    measurement: Measurement,
}

fn print_sample(sample: &Sample) -> Result<(), serde_json::Error> {
    println!("{}", serde_json::to_string(sample)?);
    Ok(())
}

fn valid_measurement(measured: Measurement) {
    assert!(!measured.accounting_invalid, "allocation accounting failed");
    assert_eq!(measured.live_bytes, 0, "allocation escaped measurement");
    assert!(measured.successful_requests > 0, "no allocations observed");
}

fn main() -> Result<(), Box<dyn Error>> {
    allocation_meter::verify_accounting()?;
    let attempted = Cell::new(0);
    let envelope = Envelope::event(
        "018f47f7-5a86-7c00-8000-000000000501".parse()?,
        FiniteZeros(&attempted),
    );
    for limit in LIMITS {
        attempted.set(0);
        let limits = WireLimits {
            max_control_frame_bytes: limit,
            ..WireLimits::default()
        };
        let (error, measurement) = measure(|| {
            let result = encode_control(black_box(&envelope), limits);
            let code = result.as_ref().err().map(|error| error.code());
            drop(result);
            code
        });
        print_sample(&Sample {
            case: "oversized_bounded_encoder",
            logical_elements: ELEMENTS,
            attempted_elements: attempted.get(),
            frame_limit: Some(limit),
            peak_live_ceiling: Some(limit + ALLOWANCE),
            realloc_overlap_ceiling: Some(2 * limit + ALLOWANCE),
            error,
            output_bytes: None,
            measurement,
        })?;
        valid_measurement(measurement);
        assert_eq!(error, Some(WireErrorCode::FrameTooLarge));
        assert!(measurement.peak_live_bytes <= limit + ALLOWANCE);
        assert!(measurement.peak_realloc_overlap_bytes <= 2 * limit + ALLOWANCE);
        assert!(attempted.get() < ELEMENTS);
        assert!(
            attempted.get() >= limit / MIN_TRAVERSAL_DIVISOR,
            "bounded encoder stopped before traversing a limit-scaled prefix: limit={limit}, attempted={}",
            attempted.get()
        );
        assert!(attempted.get() <= limit + 1);
    }

    attempted.set(0);
    let (output_bytes, measurement) = measure(|| {
        let result = serde_json::to_vec(black_box(&envelope));
        let length = result.as_ref().ok().map(|bytes| black_box(bytes).len());
        drop(result);
        length
    });
    print_sample(&Sample {
        case: "materializing_positive_control",
        logical_elements: ELEMENTS,
        attempted_elements: attempted.get(),
        frame_limit: None,
        peak_live_ceiling: None,
        realloc_overlap_ceiling: None,
        error: None,
        output_bytes,
        measurement,
    })?;
    valid_measurement(measurement);
    assert_eq!(attempted.get(), ELEMENTS);
    assert!(output_bytes.is_some_and(|bytes| bytes > 2 * ELEMENTS));
    assert!(measurement.peak_live_bytes > 2 * LIMITS[2] + ALLOWANCE);
    Ok(())
}
