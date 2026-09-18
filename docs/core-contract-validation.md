# CORE record validation

The v1 record catalogue lives in `intent-ipc::CoreRecordKind`. It covers goals,
workspaces, tasks, capabilities, observations, evidence, proposals, approvals,
operations, receipts, views, memory and artifact references. Only schema 1.0 is
implemented. Offering another protocol version does not create a record codec
for that version.

## Import boundary

Use `decode_core_record` for complete JSON records received from an untrusted
source. It checks the input byte budget, nesting, collection sizes, node count,
duplicate keys and trailing input before interpreting a typed record. The schema
header is checked first, so an unsupported version is not confused with a bad
payload for the current version. Errors do not include the submitted document.

Record structs and nested tagged records reject unknown fields. Missing fields
with an explicit default remain supported. A restriction that this version does
not understand is an error, not something to discard during a round trip.

`CoreRecord` contains typed data, not an execution grant. Approval freshness,
account scope, source revisions, artifact bytes and worker epochs still have to
be checked at the broker boundary. Calling Serde directly on a record bypasses
the document byte/shape budgets and duplicate-key check; it is not an equivalent
untrusted-input API. Owned values built inside the runtime remain the caller's
responsibility.

## Migration registration

`MigrationRegistry` now requires a validator for every exact family/version in a
migration path. This also applies to equal-version imports. Register the v1 CORE
validator from the catalogue:

```rust
use intent_contracts::{MigrationRegistry, SchemaVersion};
use intent_ipc::CoreRecordKind;

fn registry() -> Result<MigrationRegistry, Box<dyn std::error::Error>> {
    let mut registry = MigrationRegistry::new();
    for kind in CoreRecordKind::ALL {
        registry.register_schema(kind.record_family()?, SchemaVersion::V1, kind.validator())?;
    }
    Ok(registry)
}
```

The registry validates the source, resolves the complete path and verifies that
all destination validators exist before running a transform. It then validates
every intermediate result. Missing steps, invalid output, downgrades and paths
that overshoot the target fail without returning an imported document. Source
and target versions are retained in a successful outcome.

The input and each returned document are capped at 1 MiB. The registry allows at
most 256 transforms and 512 validators; error detail is limited to 512 UTF-8
bytes. Callbacks are trusted in-process functions. These admission limits do not
bound memory, side effects or execution time inside a user-supplied callback.
Validators must check their exact schema, not just whether the bytes are JSON.

There are no invented v1.1 CORE migrations in this change. Synthetic migration
families test the registry algorithm; they do not advertise product support for
another schema version.

## Error format and compatibility

The v1 JSON error-code format remains a string, such as `"UnsupportedSchema"`.
The spellings are explicit and pinned in `fixtures/core/wire-errors-v1.json`.
Numeric Rust discriminants are local diagnostic IDs, not a second accepted JSON
encoding. Numeric values, one-key object encodings, aliases and unknown names
are rejected. `InvalidRecord` is the additional record-validation error.

This tightens accepted input: unknown record fields and previously unchecked
migration data now fail. Successful canonical v1 record output and existing JSON
error names are unchanged. Migration users must register validators before
calling `migrate`. Other Serde formats have not been qualified by these JSON
fixtures. Existing numeric-money v1 data still requires an explicit legacy-format
migration decision; it must not be silently relabelled as the current schema.

## Checks

`fixtures/core/records-v1.json` contains full and minimal examples of all 13
families. The contract and IPC suites test field preservation, optional defaults,
wide-integer money, unknown outcomes, nested unknown-field rejection, schema
errors, corrupted proposal bindings and parser budgets. The conformance command
runs the record checks before emitting each family's result. These are schema
checks, not a claim that every broker authorization rule is satisfied.

```sh
cargo test --locked -p intent-contracts -p intent-ipc --all-targets
cargo run --locked -p intent-conformance --quiet
python3 -m unittest discover -s scripts/tests -v
```

`CORE parser fuzzing` builds three AddressSanitizer-instrumented targets with
`nightly-2026-09-17` and `cargo-fuzz` 0.13.2. It seeds them from the fixtures and
runs each for a 60-second budget with separate input, memory and wall-time limits.
The job verifies unchanged committed lockfiles, observes actual instrumentation
and executed inputs, and saves build/run logs, corpus, failures and a report even
on failure. Compilation alone cannot produce a passing fuzz result. A crash gets
one bounded minimization attempt; its original input is retained.

The frame target follows the decoder's consumed offset under a one-frame feed
budget. It no longer skips bytes that the decoder did not consume. The record
target checks typed round trips and validated equal-version imports.

This is a seeded fuzz smoke gate, not an exhaustive campaign or a production
readiness certificate. It does not establish native UI, live-provider, replay
isolation, platform resource enforcement or power-loss acceptance. The CORE task
ledger must continue to distinguish implementation from acceptance.
