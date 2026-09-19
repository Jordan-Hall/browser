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

## Regression and integration evidence

`crates/intent-ipc/tests/document_import.rs` exercises all 13 record families,
full and minimal current fixtures, future minors 1, 2 and 65535, original-byte
ownership and exact preservation, write refusal, strict major/header handling,
duplicate and malformed JSON, out-of-range future numbers, the hard depth bound,
inclusive byte/shape budgets and redacted opaque diagnostics. Two compile-fail
doctests prevent generic serialization and conversion of an opaque document into
a current record.

The conformance executable now checks exact future-byte preservation and both
write/typed-decoding refusals for every family. The `record_codec` fuzz target
exercises the importer and checks the same invariants. Its seeded corpus has 26
current records, 26 future-minor records, 13 unsupported-major documents, 13
duplicate-key documents and 13 out-of-range future-number documents. Python tests
verify seed selectors and ensure seed construction does not mutate the golden
fixtures. Fuzz compilation alone is not
an instrumented campaign; actual execution evidence must identify its run and
exact commit.

## Remaining scope

This is a library and conformance import boundary, not a product archive browser,
a durable restore/import transaction, a legacy numeric-money conversion, or a
new wire-schema implementation. Those integrations and independent task
acceptance remain open. All CORE acceptance statuses and the
`--require-accepted` release gate are unchanged.
