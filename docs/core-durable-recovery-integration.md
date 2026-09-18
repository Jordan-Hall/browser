# Durable startup recovery and exact-attempt reconciliation

Programme #1; CORE epic #13; requirements #3, #4 and #5; implementation PR #807, stacked on checkpoint PR #806. Primary tasks #147 and #148; source contributions to #129, #130, #131, #136, #141, #145, #151, #152, #211, #212 and #213. Checkpoints remain owned by #806/#146, supervision by #805 and pure classification by #804.

## Implemented state boundary

`RuntimeOwner::open_profile` opens a private Linux/GNU profile directory without following symbolic links. It holds exclusive advisory locks on the directory and the singly linked database inode for its lifetime. A second cooperating owner, symbolic-link database, hardlinked database or public profile is rejected. Before any planning, it commits a fresh runtime epoch with dispatch disabled and discards old approval heads and dispatch claims. The lock is not a hostile same-user sandbox; a raw connection deliberately bypassing the owner is outside this operating boundary.

Migration 009 adds versioned authority/source facts, immutable action and approval history, worker generations, one-use claims, immutable attempt lineage, authenticated reconciliation evidence, revisioned recovery plans and a restore-history fence. Migrations 001–008 are unchanged. Every managed action's effect must match its registered capability policy; a payload cannot relabel a write as a read. Approval binds the exact immutable account/capability/action, destination, message kind, argument bytes, source revision, authority revision, deadline and current runtime epoch.

Claim and send types are move-only and cannot be deserialized or constructed by a worker. The exact lease, live worker scope/capability/epoch, approval, source and deadline are checked again in the same writer transaction that persists attempt start. Sendable bytes are returned only after commit. Generic legacy transition, outbox read/claim/start/result paths cannot bypass managed-action checks. Historical in-memory fixture dispatch remains isolated; legacy file-backed dispatch must use the new owner path.

Transport acceptance is not a verified external effect. A lost result leaves an immutable attempt to reconcile. Reapproving an unsent action supersedes its old outbox and creates a distinct attempt; retry after dispatch requires explicit authoritative non-commit evidence and a fresh approval. Revocation does not erase previously admitted uncertainty. Current startup planning cannot reclassify a running incarnation's attempts.

## Planning, evidence and compensation

Startup planning is bounded to 128 operations per transaction. Interrupted or incomplete planning leaves activation disabled. Old `attempting`, `accepted` and `compensating` operations become explicit reconciliation states, preserving their attempt identity and journal. Activation verifies that every unfinished current revision has a plan; it does not grant any action permission. Each subsequent dispatch still needs fresh exact-action approval and current authority/source/worker facts.

A dedicated independently supplied key authenticates canonical, bounded, versioned read-only adapter attestations. The persisted evidence includes its authentication tag, key fingerprint, exact operation/account/capability/argument/attempt binding, observation validity and verdict. The key itself is never persisted or logged. The key selected for an attempt remains its verifier across ordinary grant changes; explicitly revoked evidence keys cannot be used or revived. A MAC proves an authorized adapter's attestation, not arbitrary provider settlement. The adapter must independently verify its provider-specific finality and identity contract.

Committed effects, authoritative non-commit, inconclusive observations, captured reads and local before/after results remain different facts. Expired/revoked execution grants do not undo a verified receipt. Repeated evidence is idempotent, conflicting final evidence is rejected, and inconclusive evidence cannot downgrade a known commit. Local evidence binds the approved before revision and a distinct observed after revision. A missing provider row or a timeout is not authoritative non-commit.

Compensation is a separate operation with separate approved bytes, source, capability and attempt. It references a verified original attempt and receipt in the same account/privacy scope. The original receipt and arguments remain immutable; only verified compensation evidence marks the original as compensated. Reads, unknown original outcomes and compensation-of-compensation shortcuts are rejected. The recovery view retains both operation identities rather than replacing the original attempt with the compensation attempt.

## Restore is not external rollback

An exported/restored snapshot carries a persistent rollback-domain fence. An older snapshot may predate a write that was accepted later by an external system, so even apparently unsent restored writes cannot obtain new execution approval. Reopening or completing local startup planning cannot clear this fence. Read-only work can still be explicitly authorized against current facts. There is intentionally no public Boolean or unconditional activation API that turns an old backup into permission to repeat external writes. Re-enabling such writes requires a subsequent independent-history recovery integration, not a local assertion that the old snapshot is complete.

## Recovery presentation and executable evidence

The typed task-centre view exposes inspect, cancel-unsent, request-read-only-reconciliation and review-fresh-approval intents. It has no blind Retry action. Local controls compare the runtime epoch and operation revision, and recheck the real current state rather than trusting the renderer's action list. A view model is not yet the native task-centre UI or a provider-query implementation.

Thirty new Rust tests include exact dispatch/journal identity, authority/source/deadline changes after claim, wrong worker scope/capability, claim replacement, generic API bypass attempts, effect-policy downgrades, key revocation, all evidence-binding mutations, canonical/duplicate/oversized evidence rejection, receipt idempotency/conflicts, original/compensation lineage, local/read effect semantics, stale recovery controls, bounded startup batches and injected transaction failure.

Actual child-process kill tests interrupt before attempt start, after durable start, after an independent external ledger accepts the effect and after transport acceptance. Recovery preserves the correct state without rewriting the independent ledger. A separate backup/restore test restores a pre-dispatch snapshot after a later external acceptance and verifies that reopening and fresh local authority still cannot silently repeat that write.

The full source must pass its own final read-only CI run; an initial contract or import workflow is not verification. CI source/lockfile artifacts identify the exact tested tree. Process kills, simulated adapter signatures and bounded SQLite tests are not VM/physical-power-loss, production-provider or all-platform qualification.

## Remaining acceptance

The trusted application must populate authority/source facts and authenticated worker registrations from their actual owners, not from untrusted wire declarations. Timestamps come from the trusted runtime clock. SQLite writer waits and filesystem sync are not preemptible disk deadlines; this owner must run off the UI/control executor. The next broker integration must connect supervisor revocation/admission and the durable dispatch boundary without giving workers direct state APIs or treating a process response as a provider receipt.

Still required: production key custody and read-only evidence adapters; independent external-history recovery after snapshot rollback; provider resume persistence and credential-isolated replay; the native task-centre; hostile-process and aggregate resource containment; all live artifact-path/root isolation; instrumented fuzzing; the platform/power-loss matrix; and independent acceptance reviews. The PR does not mark all CORE tasks complete, merge branches, close issues or weaken release gates.
