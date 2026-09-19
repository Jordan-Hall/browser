# CORE-01.T06: read-only preservation of newer schemas

This implements the bounded opaque-document path requested in issue #126,
under parent #2 and epic #13. It does not add a schema 1.1 codec, authorize a
provider action, or qualify the entire migration task for acceptance.

## Import and write boundaries

`intent_ipc::import_core_document` returns one of two explicit outcomes:

- `CoreDocumentImport::Current` contains a schema-1.0 record validated by the
  same strict, family-specific decoder as `decode_core_record`.
- `CoreDocumentImport::ReadOnlyNewerMinor` contains a same-major, newer-minor
  document as immutable original bytes. Unknown fields, JSON number spellings,
  escapes, ordering and whitespace are preserved. Its family is caller-supplied
  routing metadata; the future document's family-specific semantics have NOT
  been validated.

Both paths enforce the configured document-byte, depth, collection and node
budgets, duplicate-key rejection, valid UTF-8, the strict schema header and a
single complete JSON value. The syntax-only scanner borrows `RawValue` subtrees;
unknown numeric leaves are never coerced into `f64`. Valid future values such as
`1e400` and `1e-4000` retain their exact spellings. Invalid numeric grammar remains
an error, and the schema header still requires exact bounded integers. A hard
128-level syntax bound applies even when a caller supplies a larger depth budget.
Failure does not mutate the caller's bytes.
Unsupported major versions and versions requiring an unavailable migration fail
closed. A malformed current record never falls back to the opaque path.

`encode_for_write` accepts understood current records and returns
`UnsupportedSchema` for preserved newer documents. `ReadOnlyCoreDocument` has no
public constructor, field mutation, Serde implementation, or conversion to a
validated `CoreRecord`. Its `Debug` output contains only family, version and
byte count. `original_bytes` permits verbatim archival, not reinterpretation as
a supported record or an executable dispatch payload. Callers must continue to
treat those bytes as untrusted.

The existing live codec and `decode_core_record` still reject every schema
except 1.0. Migration validators remain exact-family/exact-version validators;
opaque retention never substitutes for schema validation or becomes a migration
success. No authority, approval, operation, receipt or source hash is recomputed
from unknown fields.

## Explicit legacy conversion

`intent_ipc::migrate_legacy_numeric_money_goal_v1` is the only compatibility
adapter for the documented pre-freeze GoalContract representation where
`budget.minor_units` was emitted as a JSON integer instead of the current decimal
string. It is opt-in rather than auto-detected. The adapter preserves the raw
numeric token long enough to parse the complete `i128` domain without floating
point conversion, rejects noncanonical number spellings and current string
values, enforces the normal syntax and structure budgets, then validates and
canonicalizes the converted bytes through the current GoalContract codec.
Other record families, unsupported schema headers and malformed records remain
rejected.

## Durable admission boundary

`intent_broker::persist_core_document_import` connects the bounded importer to
the durable artifact store without turning persisted bytes into executable
state. Current records are validated and re-encoded canonically before storage.
The explicit legacy GoalContract mode stores only the converted current-codec
bytes. Same-major newer-minor documents are stored byte-for-byte and are returned
only as `ReadOnlyCoreDocument`.

The selected record family is included in artifact media metadata, so one
artifact ID cannot be idempotently retried under another `CoreRecordKind` even
when opaque bytes, scope and timestamp are identical. Validation completes
before durable artifact publication. Failed validation therefore cannot create
an import artifact handle. Existing artifact conflict checks also reject changed
admitted bytes under an existing import ID.

Persistence is not activation or authorization: this boundary does not create
an approval, operation, outbox row, executable artifact reference or provider
request. A caller receiving a validated current `CoreRecord` must still pass the
separate authority, policy and runtime gates before any external effect.

## Regression and integration evidence

`crates/intent-ipc/tests/document_import.rs` exercises all 13 record families,
full and minimal current fixtures, future minors 1, 2 and 65535, original-byte
ownership and exact preservation, write refusal, strict major/header handling,
duplicate and malformed JSON, out-of-range future numbers, the hard depth bound,
inclusive byte/shape budgets and redacted opaque diagnostics. Two compile-fail
doctests prevent generic serialization and conversion of an opaque document into
a current record.

The conformance executable checks exact future-byte preservation and both
write/typed-decoding refusals for every family. The `record_codec` fuzz target
exercises the importer and checks the same invariants. Its seeded corpus has 26
current records, 26 future-minor records, 13 unsupported-major documents, 13
duplicate-key documents and 13 out-of-range future-number documents. Python tests
verify seed selectors and ensure seed construction does not mutate the golden
fixtures.

PR #825 supplied the explicit legacy numeric-money adapter and bounded parser
fuzz evidence. PR #826 adds seven durable broker regressions covering canonical
current persistence, exact opaque preservation, explicit legacy conversion,
validation failure without durable residue, wrong-family legacy rejection,
changed-byte artifact conflicts and opaque family rebinding. Exact source
`464c02c373a3676ce4e3d58d27f4be3327f94b10` passed CI run 35468483519: strict
workspace checks plus native Ubuntu 24.04, Windows 2025 and macOS 15 tests. The
retained CI artifact is 10591679607 with SHA-256
`d858e1d1d05c1a7d739c933ab8eda17e281fd1f98f9d95eec747609af372c168`.
The workflow compiled the fuzz targets but no new instrumented fuzz campaign was
triggered by the broker/documentation paths, so none is claimed for PR #826.

## Remaining scope

The durable admission seam is not a complete product archive browser, archive
selection UI, workspace activation transaction or restore workflow. It also does
not independently qualify compatibility policy or grant authority to imported
records. Independent task acceptance and those product-level integrations remain
open. All CORE acceptance statuses and the `--require-accepted` release gate are
unchanged.
