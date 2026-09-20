# Oversized encoder allocation evidence

The `intent-ipc/encoder_allocations` Cargo test target is a standalone executable
with `harness = false`. It measures Rust allocation requests around synchronous
`encode_control` calls. Run it with:

```sh
cargo test --locked -p intent-ipc --test encoder_allocations
```

The input emits 1,048,576 zero elements lazily. It does not allocate a collection
of its own. Each oversized call must return `FrameTooLarge` and stop consumption
early. At byte limits 1,024, 16,384 and 65,536, the fixed bounds are:

- Peak requested live bytes: limit + 4,096.
- Conservative requested bytes including realloc overlap: 2 * limit + 4,096.

The allowance covers the error objects and their text. It is fixed before
measurement and is not calibrated upward from a sample. A materializing
`serde_json::to_vec` positive control must consume the complete input, produce
more than two MiB and exceed the largest bounded envelope.

A private test allocator forwards all requests unchanged to `System`. Its
callbacks use nonpanicking atomic accounting. The executable checks allocation,
zeroed allocation, reallocation, release and arithmetic anomaly detection before
the encoder cases. Each encoder and positive-control sample must observe
allocation, finish with zero live requested bytes and have no accounting anomaly. Inputs are borrowed from
outside the window; results are reduced to copyable facts and dropped inside it.
Assertions and JSON output run after accounting is disabled. Production code
retains its unsafe-code prohibition.

The process is dedicated to these synchronous cases and starts no other work.
The measurement is not a general-purpose allocator profiler. It does not bound
RSS, allocator metadata, private allocations by arbitrary serializers, or the
allocation cost of successful large encodes and subsequent JSON validation.
Realloc overlap is a conservative estimate, not a measurement of physical pages.

## CI execution and retained samples

The existing all-target Cargo commands execute this target. A separate required
`portable.encoder_allocation` step also runs it on Ubuntu, Windows and macOS to
retain attributable samples without parsing the full test-suite output.

Each `encoder-allocation-<os>-<run>-<attempt>` artifact contains raw stdout and
stderr, the exit code, compiler and Cargo versions, the actual checked-out commit
and relevant source blobs. Output contains one JSON row per completed case,
including bounds, consumption, error code and measured requests. A failed or
partial run is retained but does not qualify the missing cases. The ordinary test
profile applies; no release-mode or physical-memory claim is implied.

The selected libtest inventory in `core-01-conformance.json` remains unchanged:
this executable has no `#[test]` functions and is not represented as a libtest
entry. The native required steps and their artifacts are separate evidence.
This unit contributes to CORE-01.T04; it does not establish whole-task acceptance.
