# Durable authorization to authenticated worker dispatch

PR #809; programme #1; CORE epic #13. This increment builds on the checkpoint lineage in #806 and durable recovery in #807, with the actual supervisor from #805. It does not introduce another checkpoint schema or incorporate the alternative checkpoint migration in #808.

## Implemented boundary

`intent-broker::RuntimeBroker` owns the `RuntimeOwner` and `Supervisor`. Neither a mutable owner nor a worker lease is exposed. Trusted application code installs the workspace graph before moving its owner into the broker, performs startup planning, activates admission, and installs current account/capability authority. Activation is not approval for any individual effect.

Workers are registered in durable state only after the real supervisor reports authenticated Ready. Registration derives the generation, task/account scope, capabilities and expiry from the broker-owned launch configuration, not from worker-provided authority claims. The implemented adapter role is ConnectorHost on Linux GNU in the approved/cooperative execution profile.

Dispatch reads non-sendable metadata, checks bounded inline size and worker scope, preflights immediate socket capacity, acquires a durable claim, refreshes wall time, and commits exact-attempt authorization through `begin_authorized_dispatch`. Only then can approved destination, message kind and payload bytes enter the authenticated Execute envelope. The effect does not enter the supervisor's asynchronous work queue. `submit_immediate` revalidates the move-only permit and starts a bounded nonblocking socket write in the same broker call.

The nested v1 invocation binds runtime epoch, worker/request identity, task/account, capability, operation/attempt identity, action deadline and exact canonical payload bytes. Workers reject mismatched outer identities, expired invocations, unknown/duplicate fields and noncanonical/corrupt payload encoding. These are transport checks, not a second policy authority.

The inline adapter supports at most 1,024 payload bytes and a 4,096-byte encoded input. Escaped routing metadata is included in a conservative preflight budget. Larger actions fail before attempt start and require a separately implemented bulk/artifact transport; there is no truncation, retry or live fallback. Fixed-field encoding is bounded again after construction.

## Ordering, cancellation and errors

The mutable broker serializes authority updates, dispatch and cancellation. Before disk work, cancellation latches local revocation; it persists worker revocation before notifying the child. A policy/source revision change invalidates affected worker generations, including enabled replacement grants. A failed policy update or cancellation persistence fences subsequent broker dispatch rather than continuing under an uncertain old grant. The broker cannot be unfenced in place: reopen the profile and run startup recovery.

The attempt commit is the admission boundary. Cancellation after that boundary cannot retract already-admitted/partially written bytes or prove provider rollback. Losing the socket, acknowledgement or process leaves the same attempt in explicit uncertainty. The broker never reconstructs sendable material or resets a started attempt to pending. Owner drop and subsequent startup preserve this distinction.

A checked worker acknowledgement is recorded only as `Accepted`, never `Verified`. Exact read-only evidence goes through the existing evidence verifier and durable reconciliation checks. Reconciliation of an inflight matching attempt requires first settling or cancelling its transport; this avoids accepting an acknowledgement over an already reconciled state. Other operations need not stop for that evidence check.

Local state inspection remains available as a read-only reference. No raw mutable state/supervisor escape is supplied. These restrictions apply to this bridge's safe API and cooperating trusted components; arbitrary code in the trusted process or an uncooperative database writer is outside this boundary.

## Executable evidence

`crates/intent-fixture-worker/tests/broker.rs` exercises actual authenticated child processes and a separate append-only external-effect fixture ledger. The worker never receives the parent's evidence-signing key. The fixture appends on every invocation, without deduplication that could hide an accidental resend. The parent independently checks the recorded operation, attempt and bytes before issuing a read-only attestation.

Tests cover exact successful dispatch, acknowledgement without an effect, lost acknowledgement, same-attempt recovery without resend, owner loss before result polling, cancellation before send, cancellation after an effect, changed/disabled authority, expired approval, authenticated wrong-scope/capability workers, failed readiness, dead sockets, oversized/escaped payload rejection, and fail-closed authority-update errors.

The process-kill scenario kills the broker test process before socket submission and after an independently recorded effect but before acknowledgement persistence. On reopen, unsent work stays unsent with stale approval, and the accepted-but-unrecorded attempt becomes reconciliation-required. Verification of the external fixture effect does not send it again. This is actual subprocess interruption, not VM or physical power-loss qualification.

`crates/intent-broker/src/invocation.rs` adds three unit tests for exact binary round-trip, debug redaction, nested identity/schema/duplicate/payload rejection and encoded budget checks. The broker integration binary contains 15 test entries, including the child-process fixture entry; normal no-environment execution of that helper performs no scenario. Together these add 18 Rust test entries to the preceding stack, not 18 separate production acceptance certificates.

Pinned local verification of the complete source passes 246 runtime tests, seven compile-fail doctests and 15 ledger tests. Formatting, warnings-denied Clippy, separately locked fuzz compilation, architecture checks and existing smoke conformance also pass. The exact published head, tested merge and remote CI outcomes are recorded on #809; local verification and initial/import CI are not substituted for final-source CI.

## Remaining acceptance

This provides the previously missing cooperative inline broker integration. It does not complete the CORE epic or automatically accept any of its 37 tracked tasks. The ledger retains unfinished acceptance rather than manufacturing review evidence.

Production provider/local-evidence adapters, durable provider resume accounting, credential-isolated replay execution and native task-centre integration remain. The fixture signer authenticates a test ledger, not an arbitrary provider's settlement system. No live provider or user account is contacted by these tests.

Whole-profile ownership and all live artifact path boundaries, hostile same-user/process-tree containment, aggregate CPU/memory/GPU enforcement, instrumented fuzz execution, system suspend, real disk-full/fault matrices, VM/power loss, cross-platform behavior and independent operational acceptance are still required at their stated strength. The alternative checkpoint PR #808 must not be merged alongside a conflicting migration 008 without an explicit reconciliation.

SQLite transaction/fsync latency is not bounded by a worker packet budget. Run the broker on a trusted runtime executor, not the UI/control event callback. These tests do not establish native control/speech latency under blocking storage or sustained model load. A progress/cancel acknowledgement is not external cancellation or rollback.
