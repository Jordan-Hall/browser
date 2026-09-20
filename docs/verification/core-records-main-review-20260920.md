# CORE-01.T03 refreshed main acceptance review

## Verdict

**Pass.** Commit `15ef9cc1ed7a4576d1e3b8f2603e6a6fdc6eca8a` satisfies the implementation, independent whole-task review, and exact-main CI gates for issue #123 / CORE-01.T03. I found no remaining source or review blocker.

This refresh uses the actual published independent-review baseline, `37c996fa28a9f04707817ddd0f17cae37b4d5b30`. It does not rely on the later unpublished `3e2b499` review draft as acceptance evidence. The original review is `target/base-record-validation-design/task123-acceptance-review.md`, referenced by the published `docs/verification/core-records-combined-20260920.json`.

Issue closure still requires publishing this refreshed decision and updating the stale task ledger. Those are evidence-publication steps, not implementation gaps.

## Scope and method

I independently reconciled:

- issue #123's canonical record, optional-field, account-scope, unresolved-outcome, strict-import, and authority-preservation criteria;
- every criterion and scope boundary in the original whole-task acceptance review at `37c996f`;
- immutable Git source for `37c996f..15ef9cc` in every changed T03 file;
- the exact-main run and retained artifact-verification record.

The T03 delta contains exactly six changed files:

- `crates/intent-contracts/src/execution.rs`;
- `crates/intent-contracts/tests/execution.rs`;
- `crates/intent-contracts/tests/execution_failed.rs`;
- `crates/intent-state/src/durable_recovery/evidence.rs`;
- `crates/intent-state/src/durable_recovery/projection.rs`;
- `crates/intent-state/src/durable_recovery/tests/projection.rs`.

The base record definitions, per-family validators, collection bounds, proposal validation, state accessors, conformance fixtures, IPC codec, record invariant tests, and migration-validation tests are unchanged from the approved baseline. I did not build, modify source, post to GitHub, or close the issue.

## Review of all six changed files

### Execution contract

`execution.rs` adds three fail-closed constraints:

1. A `Failed` record, like every other sent state, must identify an attempt whose `started_at` is present.
2. A verified compensation-phase observation requires `LocalCommitted` or `ExternalCommitted` write evidence. A read capture is still a valid committed reference for an original read, but is not compensation proof.
3. A `Compensated` record requires committed write evidence for both the original effect and its nested compensation. Compensation timing, distinct attempt identity, and distinct evidence identity checks remain in force.

These constraints narrow invalid records without changing valid JSON shapes or adding authority. `tests/execution.rs` covers verified compensation evidence, rejects compensation of a read completion, and rejects read completion as the nested compensation result. The new `tests/execution_failed.rs` rejects absent and unstarted attempts for `Failed` and accepts a started attempt.

### Durable evidence admission

`evidence.rs` now loads the prior decisive evidence payload and compares the complete typed verdict as well as denormalized verdict and receipt columns. Consequently:

- a second decisive result for one attempt is rejected even if it reuses a receipt while changing local before/after revision data;
- an exact evidence-ID replay remains governed by the existing idempotent replay path;
- a later inconclusive report after a decisive result is a non-persistent no-op, leaving the decision and revision unchanged;
- a managed compensation cannot be completed with `ReadCompleted` evidence;
- all rejection paths occur before transaction commit and do not append evidence or advance projection state.

This makes decisive evidence immutable per attempt and preserves uncertainty honestly; it does not turn inconclusive evidence into a conflicting outcome or create a synthetic receipt.

### Projection and lineage

`projection.rs` replaces the broad committed-reference check for a compensation origin with an explicit write-receipt check. Only `LocalCommitted` and `ExternalCommitted` may supply the original receipt for compensation. `ReadCompleted`, `ProvenNotCommitted`, and `Inconclusive` cannot establish write authority.

The expanded `tests/projection.rs` checks all observable effects of the new rules: altered local commit metadata with the same receipt is rejected without evidence-count or projection change; creating compensation freezes the original decisive evidence before and after compensation; and read completion cannot complete a managed compensation, leaves its evidence count at zero, leaves the compensation attempt `Attempting`, and preserves the original as `Verified`.

Together, the six-file delta closes inconsistencies identified after `37c996f`. It strengthens the original review's exact-outcome and compensation-lineage conclusions and does not weaken any other T03 criterion.

## Whole-task acceptance trace at main

| Requirement | Main finding | Result |
| --- | --- | --- |
| Canonical v1 DTO families and typed round trips | The approved 13-family full/minimal fixture corpus and typed IPC import/round-trip paths are unchanged. | Pass |
| Missing optionals, account-scoped references, unresolved external outcomes | The approved fixtures and strict typed imports are unchanged. | Pass |
| Unsupported schema/state and authority ambiguity | Strict deserialization, opaque/read-only newer supported-minor handling, and rejection of unknown nested authority variants are unchanged. | Pass |
| Exact proposal, approval, operation, and outbox binding | Artifact bytes/hash, task, account, capability, provider target, source revision, effect, approval identity, and expiry binding are unchanged. Imported DTOs still cannot mint consent. | Pass |
| Source preconditions and legacy records | Bound execution still requires the immutable binding marker; corrupt, missing, retrofitted, and replacement bindings fail closed. Legacy unbound records remain readable and non-executable. | Pass |
| Operation attempts, outcomes, and evidence | Main preserves exact recorded facts and now enforces one immutable decisive outcome per attempt. It does not invent an idempotency key or provider receipt. | Pass |
| Compensation | Contract admission, runtime evidence admission, and projection all require committed write evidence, distinct lineage, and valid ordering. Read results cannot be relabelled as compensation authority. | Pass |
| Per-family intrinsic validation | The 16,384-entry collection bound, timestamp/expiry/state relations, deterministic evidence-origin checks, and constructor/import parity are unchanged from the approved baseline. | Pass |
| Compatibility | Existing v1 fixture JSON remains stable. Under the documented pre-freeze 1.0 rule, earlier strict readers may reject new Execution records and cannot silently reinterpret them. The delta adds no wire fields. | Pass |
| Scope boundary | Live trusted provider-target execution remains downstream dispatch work. Refusing execution without that mapping preserves the guardrail and is not a criterion in issue #123. | Pass |

## Exact-main verification

GitHub Actions run [`35524771552`](https://github.com/Jordan-Hall/browser/actions/runs/35524771552) is a completed successful push run whose exact `head_sha` is `15ef9cc1ed7a4576d1e3b8f2603e6a6fdc6eca8a`. All four jobs passed:

- Rust checks;
- native contracts and storage on Ubuntu 24.04;
- native contracts and storage on Windows 2025;
- native contracts and storage on macOS 15.

`target/base-record-main-artifact-verification.json` records an independent retained-artifact check: all 270 archived source files matched Git at `15ef9cc`; compiler `rustc 1.98.1 (48a229cea 2026-09-01)` and both lockfile hashes matched; the source archive SHA-256 is `f0e207a5eabd14131d3ded054544767f20548c8628822e53ebe77dbcd2b96735`; and the verification JSON SHA-256 is `23a2ba08af035d187c8c809a285c0070bce84076863b6781d792c88f8d66fe70`.

This exact-main evidence satisfies the execution gate separately from the source review above.

## Closure disposition

No T03 implementation, test, compatibility, or independent-review work remains. Before closing #123:

1. publish this refreshed main acceptance decision in the acceptance documentation and issue record;
2. update `docs/core-task-coverage.json` and the rendered Markdown, which still say PRs #847 and #849 must merge and still mark the task `implemented_pending_acceptance`;
3. link the accepted main commit, exact-main CI run, retained artifact verification, and refreshed review from the closure comment.

Those publication changes may mark CORE-01.T03 accepted. They must not claim that CORE as a whole is production-ready or that downstream trusted provider execution is complete.

Review provenance: separate Codex acceptance reviewer (`/root/authentication_api_design_b`).
