# CORE storage durability matrix

This document records the storage behavior that has actually been implemented and the qualification boundary for `CORE-02.T01`. It is deliberately narrower than a production power-loss claim.

## Supported file-store contract

`StateStore::open` treats a file-backed SQLite database as supported only when SQLite can enter WAL mode. The production connection also enables foreign keys, disables trusted schema execution, requests `synchronous=FULL`, uses a bounded five-second busy timeout, and configures WAL auto-checkpointing at 1,000 pages. Existing Intent Browser stores must retain their application identity and store UUID; an unrelated or malformed store fails closed.

The supported storage class is therefore a local filesystem on which the platform SQLite implementation provides the documented WAL and file synchronization semantics. The project does not currently claim network filesystems, FUSE/virtual filesystems, removable media, or a named filesystem type merely because a hosted CI runner happened to use one.

## Native qualification matrix

| Platform | File-backed StateStore | WAL / application identity / quick-check across reopen | Process-crash transaction rollback | Profile-owner crash release | Power-loss / VM-reset qualification |
| --- | --- | --- | --- | --- | --- |
| Ubuntu 24.04 CI | required | covered by `crates/intent-state/tests/durability_matrix.rs` when the native job passes | covered by PR #865 real-process rollback regression | covered narrowly on Linux GNU by PR #864 | **not qualified** |
| Windows 2025 CI | required | covered by `crates/intent-state/tests/durability_matrix.rs` when the native job passes | covered by PR #865 real-process rollback regression | **not implemented by RuntimeOwner** | **not qualified** |
| macOS 15 CI | required | covered by `crates/intent-state/tests/durability_matrix.rs` when the native job passes | covered by PR #865 real-process rollback regression | **not implemented by RuntimeOwner** | **not qualified** |

The native durability regression creates a real database in the runner's temporary directory through production `StateStore::open`, verifies WAL and foreign-key policy while the production store is open, closes it, checks persisted application identity/WAL/integrity from an independent SQLite connection, then reopens the production store and requires the same store UUID, schema version, WAL mode and integrity result.

Passing this regression on a hosted runner qualifies that runner/platform combination only at the SQLite/process-crash boundary above. It does not identify or certify the host filesystem type and does not model sudden power removal, write-cache loss, controller failure, torn sectors, filesystem journal replay, or VM-host failure.

## Existing crash evidence

PR #865 qualifies rollback after forcibly terminating a process that owns an uncommitted `IMMEDIATE` SQLite transaction containing migration-shaped DDL, the next migration-ledger row and `PRAGMA user_version` change. Reopen requires all of those mutations to have rolled back, the original store identity to remain intact and SQLite integrity checks to pass. This is process-crash evidence, not power-loss evidence and not termination injected inside private `apply_migrations_through`.

PR #864 separately proves duplicate-owner rejection and crash-release for the existing Linux GNU `RuntimeOwner::open_profile` lock. The owner implementation remains Linux-only, so Windows/macOS rows must not be inferred from portable `StateStore` tests.

## Unqualified durability boundaries

Production readiness still requires explicit qualification for the filesystem and failure modes that the product will support. In particular, no current test establishes durability after host power loss or VM reset, storage-device write-cache loss, disk-full at every transaction boundary, corrupt/torn WAL or database pages, or profile-owner coordination on Windows/macOS. Snapshot publication/activation also remains Linux-specific where it relies on the descriptor-relative `snapshot_fs` backend.

Until those boundaries are exercised on controlled storage targets, `production_ready=false` and the acceptance ledger must remain fail-closed. Green native CI proves only the matrix cells explicitly marked as covered above.
