# CORE task-by-task coverage

Programme #1; epic #13; requirements #2, #3, #4 and #5; integration tracker #802.

**CORE is not complete.** This matrix separates source implementation from accepted task completion. All 32 feature tasks, baseline #113 and each of #211, #212, #213 and #214 are listed individually. PR references do not close tasks.

| Task | Issue | Implementation PR | Status / remaining acceptance |
|---|---|---|---|
| `CORE-01.T01` — Bootstrap the Rust workspace and architectural checks | #121 | #785 | implemented_pending_acceptance: Initial host checks pass; independent review and merge remain. |
| `CORE-01.T02` — Define typed identities and value objects | #122 | #786 | implemented_pending_acceptance: Wire representation and provider-resource validation require baseline sign-off. |
| `CORE-01.T03` — Specify durable core record schemas | #123 | #787 | implemented_pending_acceptance: Complete record-family fixtures and authority semantics remain unaccepted. |
| `CORE-01.T04` — Build bounded wire framing and errors | #124 | #788 | implemented_pending_acceptance: Stable error-wire representation and full parser/fuzz qualification remain. |
| `CORE-01.T05` — Authenticate locally launched worker channels | #125 | #789 | implemented_pending_acceptance: Actual spawned-process authentication and registry-owned one-use launch bindings are missing. |
| `CORE-01.T06` — Implement version negotiation and schema evolution | #126 | #790 | implemented_pending_acceptance: Authenticated production session and validated durable migration integration are missing. |
| `CORE-01.T07` — Wire cancellation, deadlines and backpressure | #127 | #791 | implemented_pending_acceptance: Real worker cancellation, deadlines, generation fencing and bounded retirement are missing. |
| `CORE-01.T08` — Publish contract conformance and fuzz targets | #128 | #792 | implemented_pending_acceptance: Instrumented fuzz execution, complete schema fixtures and process conformance are missing. |
| `CORE-02.T01` — Establish database ownership and migrations | #129 | #793 | implemented_pending_acceptance: Profile-wide ownership, filesystem trust boundary and restore validation are missing. |
| `CORE-02.T02` — Model durable operation and journal records | #130 | #794 | implemented_pending_acceptance: Evidence-bound state transitions and original/compensation lineage are missing. |
| `CORE-02.T03` — Implement transactional outbox dispatch | #131 | #795 | implemented_pending_acceptance: Authority-bound opaque dispatch, fencing and abandoned-attempt recovery are missing. |
| `CORE-02.T04` — Add inbox deduplication and consumer cursors | #132 | #796 | implemented_pending_acceptance: Source-precondition integration and repair/recovery acceptance are missing. |
| `CORE-02.T05` — Implement scoped immutable artifact storage | #133 | #797 | implemented_pending_acceptance: Exclusive root ownership, hostile-path protection and platform durability qualification are missing. |
| `CORE-02.T06` — Implement reference-safe retention and deletion | #134 | #801 | implemented_pending_acceptance: Profile/backup coordination and process/power-loss qualification are missing. |
| `CORE-02.T07` — Build backup, restore and integrity checks | #135 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-02.T08` — Run storage fault and power-loss qualification | #136 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-03.T01` — Define worker registry and launch specifications | #137 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-03.T02` — Implement process lifecycle and health | #138 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-03.T03` — Implement admission and foreground priorities | #139 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-03.T04` — Enforce platform resource constraints | #140 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-03.T05` — Implement lease-first cancellation and revocation | #141 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-03.T06` — Add crash-loop policy and degraded service | #142 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-03.T07` — Integrate resource observation and cooperative yields | #143 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-03.T08` — Verify supervisor robustness under concurrency | #144 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-04.T01` — Classify every recoverable operation | #145 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-04.T02` — Implement consistent checkpoints | #146 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-04.T03` — Plan startup recovery before dispatch | #147 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-04.T04` — Reconcile local writes and uncertain external state | #148 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-04.T05` — Build no-production replay contexts | #149 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-04.T06` — Implement provider resume and reseeding policy | #150 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-04.T07` — Expose recovery decisions in the task centre | #151 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `CORE-04.T08` — Qualify restart, suspend and corrupt-state cases | #152 | None | not_implemented: No implementation or task acceptance evidence in the reviewed stack. |
| `PROGRAMME.T03` — Freeze core contracts and process authority inventory | #113 | None | blocked: Required integration/ownership/qualification evidence has not been supplied. |
| `EPIC-CORE.T01` — Agree runtime ownership and wire contracts | #211 | None | blocked: Required integration/ownership/qualification evidence has not been supplied. |
| `EPIC-CORE.T02` — Integrate durable dispatch and worker supervision | #212 | None | blocked: Required integration/ownership/qualification evidence has not been supplied. |
| `EPIC-CORE.T03` — Exercise crash and bounded replay end to end | #213 | None | blocked: Required integration/ownership/qualification evidence has not been supplied. |
| `EPIC-CORE.T04` — Publish runtime operational readiness | #214 | None | blocked: Required integration/ownership/qualification evidence has not been supplied. |

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
