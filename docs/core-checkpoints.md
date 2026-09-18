# Consistent task checkpoints and authenticated restore upgrades

Programme #1; CORE epic #13; CORE-04 parent #5; implementation for #146.
This also contributes storage/restore regressions to #135 and #136. It does not
activate recovered execution or complete startup recovery #147.

## Transaction and identity

`StateStore::save_task_checkpoint` validates a bounded task graph, then holds
one SQLite immediate transaction while it verifies declared artifact bytes,
captures current task operations and declared consumer cursors, creates the
immutable checkpoint, registers protected retention pins, and advances the
compare-and-swap task head. A caller cannot supply invented operation outcomes
or cursor positions: these are read from the same database transaction.

The first checkpoint has graph revision zero and no predecessor. Later saves
must name the exact current revision, workspace and privacy scope. A raced or
stale writer receives `StaleRevision` without any graph/pin mutation. Each
checkpoint ID binds the complete original request hash. Repeating it returns
the same original historical record; different input with that ID conflicts.
Later operations/cursor movement do not silently change the retry result.

The record contains the typed workspace/task/goal, acyclic graph, immutable
artifact descriptions, exact operation identity/revision/attempt/outcome at its
journal watermark, declared cursor positions, and optional worker epoch.
Provider references contain only provider/account IDs and a content hash to
runtime-owned metadata. There is no bearer-token, credential or OS-handle field.
Free-text intent and artifact content may still be sensitive application data;
this is not a secret scanner or an encrypted-at-rest format.

`StoredCheckpoint` cannot be deserialized by a caller. Loading it through the
state owner verifies payload/request hashes and header bindings, the complete
operation set at the captured watermark, journal revision/outcome consistency,
non-regressed cursors, exact scoped artifacts, file bytes, and the complete
active pin set. It returns historical data, never a grant or a live-state rewind.
Both creation and loading preserve the closed file-backed dispatch gate.

## Retention ownership

Migration 008 creates immutable checkpoint records, revisioned heads and exact
artifact membership. Membership triggers install the reserved
`runtime_checkpoint` reference. Generic reference registration cannot fabricate
this kind, and generic removal cannot remove a live checkpoint's pin. A current
head cannot be retired. Retiring an older checkpoint requires the current head
revision, preserves an immutable tombstone and releases its memberships/pins
in one transaction. Existing independent references and holds still protect
content. Suppression remains immediate: a suppressed dependency makes loading
fail visibly even though retention preserves its bytes.

Publication, checkpoint pins, suppression, holds and GC all use the existing
shared SQLite writer boundary. This applies to cooperating writers sharing one
database and a trusted root. It does not solve hostile ancestor replacement,
root ownership, another database sharing the directory, or a raw SQL attacker.

## Bounds

A checkpoint has a 1 MiB serialized cap enforced during writing, at most 256
nodes, 64 dependencies per node, 256 artifacts, 256 task operations, 64 cursor
keys and 32 provider references. Total verified artifact bytes are capped at
256 MiB. Duplicate/cyclic/dangling dependencies and invalid scope/identity fail
before commit. Reads check SQL blob length before loading checkpoint bytes.
The bounded writer's overflow state is sticky; a swallowed serializer error
cannot turn a truncated output into success.

Filesystem verification holds the writer lock and may increase write latency.
The byte budget does not preempt a blocked disk operation. Large histories need
explicit retention/compaction rather than silently dropping operations to fit.

## Snapshot evolution

The snapshot format starts at storage schema 7. Restore now supports an exact,
authenticated schema-7 snapshot into a fresh schema-8 directory. Authentication,
source database hash, supported source ledger and exact source schema/inventory
are checked before executing any upgrade. Only the private destination database
is upgraded; the authenticated source is unchanged. The destination is validated
again, assigned a new disabled runtime epoch, flushed and published without
replacing an existing directory. Unknown/tampered source schemas fail closed.
The receipt distinguishes `source_schema_version` from restored `schema_version`.
Existing migration SQL 001–007 is unchanged. Migration 008 is additive.

An authenticated current snapshot preserves checkpoint graph/history, pins,
consumer cursors, uncertainty and retention. A snapshot/VM rollback cannot undo
an external effect; the restored checkpoint never permits blind resend.

## Executable evidence and limits

`checkpoints/tests.rs` exercises exact round-trips/reopen, stable retries,
concurrent CAS, injected rollback after pins, invalid graphs/cursors/scopes,
aggregate bounds, pin retirement/GC, missing/corrupt/misdescribed artifacts,
corrupted payload/pins/heads, omission of an operation at the journal watermark,
uncertain attempts, backup/restore, writer exclusion, and actual child-process
kills immediately before/after commit. A compile-fail doctest prevents importing
an unverified `StoredCheckpoint` from JSON. Snapshot tests cover authenticated
schema-7 upgrade and authenticated-but-altered legacy schema rejection.

Process kills do not qualify abrupt VM/power loss. Native task-centre integration,
authoritative graph mutation, profile ownership, fresh authority/source checks,
startup activation, reconciliation and whole-task acceptance remain separate
requirements. No acceptance checkbox is changed merely by adding these tests.

Publication: PR #808; initial source commit `de2832606f7fcc31513f1f81ac3d497845a92e10`. Remote source CI is recorded in the PR after it actually executes.
