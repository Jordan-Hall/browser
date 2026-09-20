# Core contract conformance

The `intent-conformance` crate is the executable compatibility gate for the trusted contract/IPC layer. Its JSON report is generated in CI with the exact compiler version, contract families, current schema version, canonical protocol fixture, migration result checks, worker authentication negatives and role/capability negatives.

A change to a canonical fixture is intentional only when the corresponding schema/protocol change is reviewed. The conformance executable must not normalize an incompatible wire change into success.

## Acceptance evidence map

`docs/core-01-conformance.json` maps every acceptance criterion on CORE-01 / issue #2 to stable evidence IDs, exact test functions or fixtures, evidence class, CI step and supported targets. Unit and subprocess entries also name their exact Cargo package and target. `scripts/core_conformance_gate.py` validates that map against repository source and combines it with the actual CI step outcomes for one exact commit. The emitted `target/conformance/core-01-evidence.json` therefore records the tested revision and a result for every required evidence item instead of treating the existence of a test or an empty list as a pass.

The gate is fail-closed. CI enumerates each declared Cargo library or integration-test target separately with `cargo test -- --list`, retaining target identity in separate inventory files. Unit and subprocess evidence must exist as a non-ignored `#[test]` in source and appear under its exact qualified inventory name in the declared Cargo target before a successful aggregate test step can satisfy the mapping. A same-named test from a different package or target cannot satisfy the evidence. Any Rust attribute block containing `ignore`, including reason-bearing and conditional forms, is rejected conservatively rather than credited as executed evidence. Missing invariants, malformed manifest shape, missing required evidence IDs, absent test functions/fixtures, unsupported-only evidence, missing target inventories, and missing/skipped/failed CI steps make the report fail and return a non-zero status. The report is written before the failure status is returned, so the normal `always()` artifact upload retains diagnostic evidence from failing runs. Gate regressions deliberately remove the action-hash and queue-limit evidence as well as a complete acceptance criterion, exercise wrong-target collisions and ignored-test forms, and execute malformed-manifest plus missing/skipped/failed outcome paths.

Evidence classes remain distinct. Unit checks demonstrate narrow contract behavior, subprocess checks demonstrate launched process boundaries, and the conformance executable validates canonical fixture/codec behavior. This mapping is not independent task acceptance and does not turn a supported target into executed native-platform qualification; those gates remain separate.

## Fuzz targets

The `fuzz/` package is deliberately excluded from the default workspace so libFuzzer is opt-in and does not become a production dependency. Its pinned CI smoke workflow builds and executes three AddressSanitizer targets:

- incremental frame decoding under arbitrary fragmentation, malformed headers and EOF finalization;
- typed control-envelope parsing under arbitrary bytes, including depth/collection/error paths;
- complete record-codec parsing and round trips across the registered record families.

The workflow records toolchain, cargo-fuzz version, lockfile/fixture identities, corpus identity, actual executed-input counts and failure artifacts. It preserves a non-zero exit status while still writing a structured report and retaining the corpus/logs. These finite seeded runs are parser smoke evidence, not exhaustive fuzzing or production qualification.

Fuzz findings that can affect authority, memory bounds or parser ambiguity are release-blocking until minimized into deterministic regression tests.
