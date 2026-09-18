# Core contract conformance

The `intent-conformance` crate is the executable compatibility gate for the trusted contract/IPC layer. Its JSON report is generated in CI with the exact compiler version, contract families, current schema version, canonical protocol fixture, migration result checks, worker authentication negatives and role/capability negatives.

A change to a canonical fixture is intentional only when the corresponding schema/protocol change is reviewed. The conformance executable must not normalize an incompatible wire change into success.

## Fuzz targets

The `fuzz/` package is deliberately excluded from the default workspace so libFuzzer is opt-in and does not become a production dependency. It contains targets for:

- incremental frame decoding under arbitrary fragmentation and malformed headers;
- typed control-envelope parsing under arbitrary bytes, including depth/collection/error paths.

Run them with `cargo fuzz run frame_decoder` and `cargo fuzz run control_envelope` from the repository root after installing `cargo-fuzz` on a supported development host.

Fuzz findings that can affect authority, memory bounds or parser ambiguity are release-blocking until minimized into deterministic regression tests.
