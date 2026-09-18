# CORE task-by-task coverage

Programme #1; epic #13; requirements #2, #3, #4 and #5; integration tracker #802.

**CORE remains incomplete and unaccepted.** This candidate is based on PR #804 at `5c6bbb4cc7af28fe1a20d0921409749bc74f2487`. The new supervisor source and this corrected ledger are **local/unpublished**; no new implementation PR is invented. #803 and #804 are existing published PRs with partial acceptance still open.

A PR link names a source contribution, not completion. The 37-row ledger separates partial source from accepted tasks. All acceptance states remain open. The normal CI check validates the recorded inventory; `python3 scripts/check_core_coverage.py --require-accepted` intentionally fails until every task has reviewed acceptance evidence.

See [local worker acceptance and limits](core-worker-supervision-acceptance.md) for executed scenarios and remaining boundaries. A process result is not an external receipt; a yield acknowledgement does not establish GPU eviction.

| Task | Issue | Source PRs / local source | Status and remaining acceptance |
|---|---|---|---|
| `CORE-01.T01` — Bootstrap the Rust workspace and architectural checks | #121 | #785 | implemented_pending_acceptance: Initial host checks pass; independent review and merge remain. |
| `CORE-01.T02` — Define typed identities and value objects | #122 | #786 | implemented_pending_acceptance: Wire representation and provider-resource validation require baseline sign-off. |
| `CORE-01.T03` — Specify durable core record schemas | #123 | #787 | implemented_pending_acceptance: Complete record-family fixtures and authority semantics remain unaccepted. |
| `CORE-01.T04` — Build bounded wire framing and errors | #124 | #788 | implemented_pending_acceptance: Stable error-wire representation and full parser/fuzz qualification remain. |
| `CORE-01.T05` — Authenticate locally launched worker channels | #125 | #789; unpublished supervisor candidate | implemented_pending_acceptance: Real post-spawn peer credentials, one-use bootstrap and role binding are tested locally. Independent threat-boundary review, hostile same-user isolation and platform qualification remain; source is unpublished. |
| `CORE-01.T06` — Implement version negotiation and schema evolution | #126 | #790; unpublished supervisor candidate | implemented_pending_acceptance: The local supervisor owns the negotiated implemented v1 codec. Durable migration validation, complete version/record-family fixtures and accepted product integration remain; source is unpublished. |
| `CORE-01.T07` — Wire cancellation, deadlines and backpressure | #127 | #791; unpublished supervisor candidate | implemented_pending_acceptance: Local real-worker cancellation, deadline checks and bounded generation retirement exist. Trusted durable-dispatch integration, byte/accounting baseline and product control-latency acceptance remain; source is unpublished. |
| `CORE-01.T08` — Publish contract conformance and fuzz targets | #128 | #792 | implemented_pending_acceptance: Instrumented fuzz execution, complete schema fixtures and process conformance are missing. |
| `CORE-02.T01` — Establish database ownership and migrations | #129 | #793 | implemented_pending_acceptance: Transactional migration hardening and authenticated restore primitives exist. Lifetime profile ownership, shared-root isolation and complete platform qualification remain. |
| `CORE-02.T02` — Model durable operation and journal records | #130 | #794 | implemented_pending_acceptance: Evidence-bound state transitions and original/compensation lineage are missing. |
| `CORE-02.T03` — Implement transactional outbox dispatch | #131 | #795 | implemented_pending_acceptance: Authority-bound opaque dispatch, fencing and abandoned-attempt recovery are missing. |
| `CORE-02.T04` — Add inbox deduplication and consumer cursors | #132 | #796 | implemented_pending_acceptance: Source-precondition integration and repair/recovery acceptance are missing. |
| `CORE-02.T05` — Implement scoped immutable artifact storage | #133 | #797 | implemented_pending_acceptance: Artifact lifecycle is serialized; #803 adds descriptor-relative snapshot traversal. Exclusive root ownership and all live-artifact path boundaries, plus platform durability, remain unaccepted. |
| `CORE-02.T06` — Implement reference-safe retention and deletion | #134 | #801 | implemented_pending_acceptance: Snapshot/GC writer exclusion exists in #803. Lifetime profile coordination and process/VM power-loss qualification remain. |
| `CORE-02.T07` — Build backup, restore and integrity checks | #135 | #803 | implemented_pending_acceptance: Authenticated plaintext snapshots and fail-closed restore are implemented in #803. Key custody/product integration, lifetime profile ownership, activation and platform/power-loss qualification remain. |
| `CORE-02.T08` — Run storage fault and power-loss qualification | #136 | #803 | partially_implemented: Snapshot interruption/corruption/writer-exclusion tests exist in #803; storage/VM power-loss, disk-full and filesystem qualification are not complete. |
| `CORE-03.T01` — Define worker registry and launch specifications | #137 | No published implementation; unpublished supervisor candidate | partially_implemented: Verified sealed image/specification and real kernel-peer handshake exist locally. Launch registry is not wired to the durable broker/ownership baseline; hostile-process and multi-platform qualification remain. |
| `CORE-03.T02` — Implement process lifecycle and health | #138 | No published implementation; unpublished supervisor candidate | partially_implemented: Local real-process readiness, heartbeat/progress/OS-exit classification and cooperative cleanup exist. Production broker/app integration, noncooperative descendant containment and kernel-uninterruptible cleanup are not qualified. |
| `CORE-03.T03` — Implement admission and foreground priorities | #139 | No published implementation; unpublished supervisor candidate | partially_implemented: Local bounded count/byte queues and CPU/memory admission reservations exist. Reservations are estimates, not aggregate OS containment; native foreground/speech responsiveness and integrated load acceptance remain. |
| `CORE-03.T04` — Enforce platform resource constraints | #140 | No published implementation; unpublished supervisor candidate | partially_implemented: Linux per-process hard AS/CPU/file limits, NO_NEW_PRIVS and descriptor hygiene are locally tested. Aggregate cgroup/VM/sandbox containment, descendant escape prevention and other platforms remain unsupported; unattended-untrusted launch is rejected. |
| `CORE-03.T05` — Implement lease-first cancellation and revocation | #141 | No published implementation; unpublished supervisor candidate | partially_implemented: Local generation revocation precedes worker notification; late responses and post-revocation admission are rejected. Actual provider/outbox send is not in this atomic boundary and durable uncertain attempts are not integrated. |
| `CORE-03.T06` — Add crash-loop policy and degraded service | #142 | No published implementation; unpublished supervisor candidate | partially_implemented: Local bounded backoff/circuit and preserved lifetime with fresh generation are tested. Restart does not replay outstanding requests. Durable checkpoint, budget persistence and production degraded-service integration remain. |
| `CORE-03.T07` — Integrate resource observation and cooperative yields | #143 | No published implementation; unpublished supervisor candidate | partially_implemented: Bounded content-free process sampling and coalesced yield messages are implemented locally. GPU metrics/enforcement, actual model unload/KV eviction and speech/frontend latency qualification remain absent. |
| `CORE-03.T08` — Verify supervisor robustness under concurrency | #144 | No published implementation; unpublished supervisor candidate | partially_implemented: Real subprocess churn, saturation, SIGSTOP/SIGCONT, scoped cancellation, late results and cooperative descendants are tested locally. Hostile process-tree escape, supervisor-crash integration, system suspend and cross-platform qualification remain. |
| `CORE-04.T01` — Classify every recoverable operation | #145 | #804 | partially_implemented: Pure deterministic recovery policy exists in #804; trusted durable-fact/evidence adapters, persisted original/compensation lineage and live integration remain. |
| `CORE-04.T02` — Implement consistent checkpoints | #146 | No published implementation | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-04.T03` — Plan startup recovery before dispatch | #147 | #803 | partially_implemented: Restored stores deny dispatch in #803; startup classification, authoritative revalidation and safe activation are not implemented. |
| `CORE-04.T04` — Reconcile local writes and uncertain external state | #148 | No published implementation | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-04.T05` — Build no-production replay contexts | #149 | #804 | partially_implemented: Captured-only replay validation exists in #804; credential-isolated worker execution, production dispatch separation and end-to-end recovery qualification remain. |
| `CORE-04.T06` — Implement provider resume and reseeding policy | #150 | #804 | partially_implemented: Read-only provider resume/reseed planning exists in #804; provider SDK/credential integration and durable resume-attempt accounting remain. |
| `CORE-04.T07` — Expose recovery decisions in the task centre | #151 | No published implementation | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-04.T08` — Qualify restart, suspend and corrupt-state cases | #152 | No published implementation | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `PROGRAMME.T03` — Freeze core contracts and process authority inventory | #113 | No published implementation | blocked: Required complete ownership, durable-dispatch/recovery integration or operational qualification has not been supplied. Local supervisor and reference ledgers do not constitute acceptance. |
| `EPIC-CORE.T01` — Agree runtime ownership and wire contracts | #211 | No published implementation | blocked: Required complete ownership, durable-dispatch/recovery integration or operational qualification has not been supplied. Local supervisor and reference ledgers do not constitute acceptance. |
| `EPIC-CORE.T02` — Integrate durable dispatch and worker supervision | #212 | No published implementation | blocked: Required complete ownership, durable-dispatch/recovery integration or operational qualification has not been supplied. Local supervisor and reference ledgers do not constitute acceptance. |
| `EPIC-CORE.T03` — Exercise crash and bounded replay end to end | #213 | No published implementation | blocked: Required complete ownership, durable-dispatch/recovery integration or operational qualification has not been supplied. Local supervisor and reference ledgers do not constitute acceptance. |
| `EPIC-CORE.T04` — Publish runtime operational readiness | #214 | No published implementation | blocked: Required complete ownership, durable-dispatch/recovery integration or operational qualification has not been supplied. Local supervisor and reference ledgers do not constitute acceptance. |

## Evidence interpretation

The release-mode check validates required evidence shape and refuses unfinished tasks; it does not authenticate reports or certify live behaviour from links. Acceptance needs an independent review of actual source, test outcomes and task-specific limits. Local findings have not been pushed into #802 or any owning PR.

## Restacked branch verification

Each existing task branch preserves its original tip and merges the updated predecessor without force-pushing. CI and lockfiles reflect only packages present at that point in the stack. The stage-specific source trees were locally compiled with the pinned Rust 1.98.1 toolchain and verified vendored dependencies. This is Linux container evidence, not Windows/macOS or power-loss qualification.

| PR | Task | Runtime tests | Doctests | Source tree |
|---|---|---:|---:|---|
| #785 | `CORE-01.T01` | 5 | 0 | `4e1e159db53cf16d8f4c630609b869df850a54bf` |
| #786 | `CORE-01.T02` | 17 | 1 | `1a27d8fefc6a957ba9f81ff5e3b9eb46a46deecb` |
| #787 | `CORE-01.T03` | 23 | 1 | `c1f3821745de3d887b259c9b831e18fbf82cf009` |
| #788 | `CORE-01.T04` | 44 | 1 | `72f3776228972a5011c40917c3e9ac4321ec8548` |
| #789 | `CORE-01.T05` | 51 | 2 | `43c3490bd5d371f27c2fec758c737f0dab524f6e` |
| #790 | `CORE-01.T06` | 64 | 2 | `28c4aeb5a4c437c7c873675389575d8960f3016e` |
| #791 | `CORE-01.T07` | 73 | 4 | `a612f2b5b9a44ac1f3c93369f6df5a1af1a65bf9` |
| #792 | `CORE-01.T08` | 74 | 4 | `18a0794fcca56395901d7eecc03e5a41f8e51bf6` |
| #793 | `CORE-02.T01` | 82 | 4 | `61380078f9e66461ba7c7ff6bd922ef690cb61fb` |
| #794 | `CORE-02.T02` | 87 | 4 | `6967fe3ae134f248f6cc36f13eff08bb769fbb56` |
| #795 | `CORE-02.T03` | 93 | 4 | `4ca3646a92958231250043225d24b16374486ae3` |
| #796 | `CORE-02.T04` | 100 | 4 | `859f21a310a130ec46fc29cf823cf3ef8cc4e029` |
| #797 | `CORE-02.T05` | 107 | 4 | `0ba40a2a376380ef42b46e5419da134dc2c24540` |
| #801 | `CORE-02.T06` | 117 | 4 | `4a8f77752c374252617a0d71955a01bb75c89c04` |

Formatting, Clippy with warnings denied, unit/integration tests, doctests and the actual architecture checker passed on every listed tree. Fuzz-target compilation and smoke conformance passed where introduced (#792 onward). These totals are cumulative within each branch, not distinct tests to sum across branches.

The two additional fixes in this pass reject duplicate JSON keys (including equivalent escaped keys) before typed payload parsing, and reject invalid artifact-reference inserts instead of reporting a retention pin that does not exist. They are owned by #788/#124 and #797/#133 respectively.

No production release, merge, issue closure or blanket review resolution is authorized by this matrix.

## Checkpoint implementation follow-through

CORE-04.T02 / #146 now has local source in `crates/intent-state/src/checkpoints.rs`, additive migration 008, typed checkpoint records and a 21-test regression suite including process death and authenticated backup restore. This change records implementation, not acceptance. Publication identity and the exact remote CI run will be recorded on its owning PR. The previously preserved supervisor source is published in #805 at `3bfdbc0f679e37368f5d8e04975390bb4d862295`; its Linux CI run 35336466160 passes. Earlier source snapshots and their counts above are historical, not an all-tasks completion declaration.
