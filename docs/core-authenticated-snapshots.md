# CORE-02.T07: authenticated plaintext snapshots

Task #135; parent #3; programme #1; epic #13. Related recovery barrier: #147.
Storage interruption evidence contributes to #136 but is not power-loss qualification.

## Implemented boundary

`StateStore::export_authenticated_plaintext_snapshot` uses SQLite's supported backup API
from a separate read connection while holding the same immediate writer transaction used
by cooperating blob publishers, references, holds and garbage collection. The exact snapshot
is inventoried; every stored blob, including suppressed but retained content, is copied and
hash-verified before this lock is released. No temporary pin records are created, so a crashed
backup worker cannot leave permanent pins behind. This deliberately trades writer latency for
a simple exclusion invariant. Input, manifest, entry, aggregate-byte and elapsed-time budgets
are enforced. Filesystem operations themselves are not preemptible deadline guarantees.

The versioned manifest binds the source store UUID, schema, journal position, fresh blocked
runtime epoch, database bytes/hash and complete ordered scope/hash/size inventory. The
database hash also binds every artifact handle, reference, hold, suppression flag and operation
record. RustCrypto HMAC-SHA256 authenticates the exact manifest bytes with a dedicated
32-byte key supplied independently of the snapshot; verification uses constant-time tag
comparison. The key is neither serialized nor printed. The application must manage it through
its trusted key store, not write it alongside the snapshot. Key rotation, UI/key-store integration
and in-memory secret erasure are not implemented by this API.

This is an **explicitly unencrypted export**, not an encrypted backup product. The call requires
`PlaintextExportConsent::SensitiveDataWillBeWrittenUnencrypted`. Only a trusted UI should
obtain that consent. Use protected encrypted storage when confidentiality is required; this API
makes no encryption-at-rest claim and must not be exposed as a silent automatic export.

## Restore and dispatch

`StateStore::restore_authenticated_plaintext_snapshot` authenticates the bounded manifest
before parsing any SQLite content. It copies and verifies the database into fresh private staging,
checks exact compiled schema and migration history, runs integrity and foreign-key checks,
compares the entire database blob inventory, then verifies/copies every blob. Missing, extra,
duplicate, corrupted, symlinked, unsupported and inconsistent data fail closed.

The source snapshot is never modified. Existing destinations are never replaced: publication
uses descriptor-relative `renameat2(RENAME_NOREPLACE)`, followed by directory synchronization.
Failure before publication removes only owned staging. Failure after rename reports
`PublicationUncertain` with the destination, and never deletes or overwrites the published copy.
After process death, an incomplete `.snapshot-pending-*` directory is not an activated profile.
Such crash residue is private and may require explicit operator cleanup; no recursive scavenger
is installed.

Migration 007 creates a durable dispatch barrier, disabled by default. Exported copies are already
disabled; every restore receives another fresh epoch. `begin_dispatch` verifies the durable flag
and the calling store's runtime epoch inside its writer transaction before changing state or
returning sendable bytes. Ordinary file-backed `StateStore::open` cannot resume dispatch.
Only the existing explicitly in-memory fixture constructor initializes a test epoch as ready.
A production startup/authority revalidation flow must own future activation; restoring state is
not permission to resend a purchase, deletion, message or any uncertain external effect.

Historical operation, journal, outbox and attempt records are retained, not guessed or rewritten
by snapshot import. Per-operation recovery classification and read-only reconciliation remain
separate requirements. No exactly-once external-action guarantee is made.

## Filesystem and schema support

Initial implementation: Linux/GNU with `/proc/self/fd`, descriptor-relative no-follow traversal,
no-follow/nonblocking regular-file opens, and no-replace directory rename support. Caller roots
must be owned by the current user and have no group/other permissions. Relative parent traversal
and symlinked path components are rejected. Files copied from the snapshot are never hard-linked
into a restored profile. Same-user hostile processes, a second database sharing the artifact root,
and missing exclusive profile ownership are outside this cooperating-writer invariant.

Only the current compiled database schema is imported. Existing migration SQL 001–006 is unchanged;
007 is appended. An unsupported or altered schema requires a separately reviewed migration/restore
path, not automatic downgrade or permissive import. Cross-platform restore/durability, encrypted
backup UX, protected key management and process-wide authority integration remain release gates.

## Executable evidence

The test suite exercises authenticated roundtrip, preserved suppression/references, blocked stale
leases, wrong keys, changed manifests/databases/blobs, extra/missing blobs, signed inconsistent
inventories, injected schema changes, budgets, private-path rules, collision/no-overwrite behavior,
copy/publication failures, and backup-versus-GC exclusion.

An actual child test process is killed while copying and immediately after rename. Reopening the
source and retrying/validating the snapshot verifies process-death handling and lock release.
This is not a VM power cut, storage-controller cache-loss test or Windows/macOS qualification.

Primary API references:
- SQLite online backup locking: https://www.sqlite.org/c3ref/backup_finish.html
- RustCrypto HMAC and constant-time verification: https://docs.rs/hmac/0.12.1/hmac/
- Linux no-replace rename: https://man7.org/linux/man-pages/man2/rename.2.html
