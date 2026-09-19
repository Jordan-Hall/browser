# CORE native implementation checkpoint

This working-tree checkpoint extends commit `e33408ffa09dfd826715b25f063ec0849b1dde50`.
The changes are uncommitted on `main`. CORE acceptance remains open; no later workstream was started.

Implemented in the main working tree:

- Provider identities reject empty input through constructors and deserialization while preserving opaque Unicode values.
- Spending amounts require nonnegative values and known currency scales. Narrow integer conversions are checked. Generated tests exercise 4,096 full-range money values through the wire codec.
- IPC encoders enforce the same structural budgets and duplicate-key rejection as decoders.
- Windows artifact publication and collection use actual directory handles and flush results. This is native functionality, not power-loss qualification.
- Windows named-pipe tests authenticate both process IDs and reject incorrect launch identities. This does not supply a Windows supervisor.
- CI includes native Windows, Linux and macOS contract/storage jobs. The new remote matrix has not run.
- Acceptance-evidence traversal tests use actual Windows junctions without requiring symlink privileges.
- Duplicate queued cancellations and acknowledgements coalesce. A full acknowledgement queue reports that cancellation has already taken effect. Progress counts exclude artifact references.
- Migration 011 records provider recovery attempts before returning. Pending attempts consume the same durable budget after a crash or reopen. Failed or unfinished resumes select reseeding.
- Owned-child process sampling reports CPU durations and resident bytes on Windows and Linux. The Linux supervisor uses the normalized samples without reaping its process-group leader.
- Suppressed artifacts return `ArtifactNotFound` instead of a negative-size corruption error. Republishing the same ID fails before final blob publication and leaves the retained artifact unchanged.

Executed locally on Windows with Rust 1.98.1:

- All 60 state tests passed, including checkpoint process termination tests (`core-windows-state.log`).
- The portable contract/IPC suite passed (`core-portable-tests.log`).
- Integrated local-transport verification passed six unit tests, the parent/child named-pipe integration test and its child helper, and a compile-fail doctest.
- Strict Clippy passed for the portable packages and for integrated state/local-transport changes.
- All 20 Python evidence-validation tests passed.

The three patches left separate at the previous stop are now integrated:

| Original patch | Integrated scope | Limits |
| --- | --- | --- |
| `target/ipc-gates-work/ipc-gates.patch` | Duplicate cancellation coalescing, explicit pending acknowledgement, accurate progress count | `StreamEndpoint` still has no runtime caller. These are queue-level guarantees. |
| `target/provider-attempt-accounting.patch` | Durable recovery-attempt budget, crash and reopen accounting, migration 011 | Accounting has no runtime caller. Callers must bind the task and checkpoint and verify authority before contacting a provider. |
| `target/windows-peer-work/process-observation.patch` | Native Windows process CPU and memory sampling and normalized Linux metrics | Windows has owned-child sampling, but supervisor runtime ownership remains Linux-only. |

Verification during the resumed run:

- The combined Windows workspace tests passed after integrating all three patches (`target/resume-windows-tests.log`). Linux-only test targets execute zero tests on Windows.
- After the suppression fix, all 72 Windows state tests passed (`target/resume-state-after.log`). The regression previously failed with `InvalidStoredRecord("negative artifact byte size")` (`target/resume-suppression-before-local.log`).
- Strict Clippy passed for the portable CI package set (`target/resume-portable-clippy-final.log`). Full Windows workspace Clippy fails on 15 existing dead-code diagnostics in the Linux-only supervisor policy paths (`target/resume-windows-clippy.log`).
- All 20 Python evidence-validator tests passed (`target/resume-evidence-tests.log`). The ledger remains valid, and `--require-accepted` still fails with 37 unaccepted tasks.
- The first Linux container run compiled but hit broker-fixture timeouts with its build output on the Windows bind mount. That run was stopped. A serial diagnostic rerun uses a native Linux build volume (`target/resume-linux-broker-isolated.log`). Linux verification is still in progress.
- The native CI test matrix now includes `intent-supervisor`, covering Windows owned-child sampling and explicit unsupported sampling on macOS. Remote CI and native macOS verification have not run in this session.

Remaining acceptance gates include native supervisor/runtime ownership beyond Linux, full trusted-path coverage, platform power-loss qualification, provider/runtime integration, and independent task acceptance. Passing local tests does not satisfy those gates.

The previous run stopped after its requested deadline of 23:10 UTC on September 18 because Docker approval delayed execution. This resumed run begins from that checkpoint and preserves its acceptance limits.

## 2026-09-19 CORE-01.T01 baseline hardening

PR #817 merged as `252570ece2feb8089582c26518551ca4ae88298c` after exact-head CI run 35435686134 passed at `7baca2b8a7f02bc87453d3f03a7a611835566213` on Ubuntu 24.04, Windows 2025 and macOS 15. The run verified the compiler selected from `rust-toolchain.toml`, committed lockfiles, formatting, strict Clippy, workspace/native tests, doctests, fuzz-target compilation, the real architecture checker and contract conformance.

The architecture negative fixture now invokes the real checker against a target-specific Cargo dependency whose package is aliased to the allowed name `serde`; it fails because the underlying dependency is a path/workspace package. The checker scope is explicitly direct declared dependencies (normal, development, build and target-specific) plus reviewed source/name policy. It does not claim transitive supply-chain qualification; transitive resolution remains represented by the committed lockfile and separate review gates.

The CI checkout and artifact-upload actions are pinned to reviewed commit revisions, and CI/toolchain export derive Rust 1.98.1 from the checked-in toolchain file instead of duplicating the patch version in workflow commands. A drift regression rejects a floating toolchain and mismatched workspace `rust-version` major/minor. The requested `@codex review` could not run because the connector reported exhausted code-review quota, so no independent review or task acceptance is claimed. The existing `--require-accepted` gate and all remaining CORE acceptance criteria stay unchanged.

## 2026-09-19 CORE-01.T02 typed values and generated conformance

PR #819 adds the missing `ProfileId` and checked `ByteSize` conversions to signed persistence and address-sized consumers. Nil UUIDs remain legal authority-neutral identity values. The conversions reject out-of-range integers without authorizing an allocation. Existing monetary, timestamp, hash and provider encodings are unchanged.

Source `bdab42d95fd342954059fc2a7ca628cbaafb3bce` (tree `c4fa16b38aba7970ef9a83fafcc47b36ea78da2e`) passed CI run 35437459115, including native Ubuntu 24.04, Windows 2025 and macOS 15 jobs. The tested merge was `bf109242df0578bb05f75badbc8fa80489b4c150` against main `aedfc1411edde5d5e5fa94ab6a5c1988d15846d4`. Three new deterministic generated suites run 1,024 iterations each through direct JSON and the real envelope codec, with boundary and malformed-input checks. Two new compile-fail examples enforce profile/workspace and account/task separation. The local complete pinned/locked run passed 340 Rust test results including doctests, 25 Python tests, strict Clippy, both formatters, fuzz-target compilation, architecture and conformance checks. A deliberate truncating conversion mutation failed the new regression; restoring the implementation passed.

Instrumented fuzz run 35437459083 passed all three targets: 2,534,765 frame-decoder inputs, 3,873,675 control-envelope inputs and 2,153,930 record-codec inputs. These were bounded 60-second AddressSanitizer runs, not exhaustive parser or production qualification. The retained artifacts, exact source/lockfile hashes, test scope and limits are recorded in [the verification report](verification/core-typed-values-20260919.json).

The fresh `@codex review` request received an exhausted-review-quota response, not an approval. Independent task/compatibility acceptance remains open. No CORE task is accepted or closed by this increment; `--require-accepted` still rejects all 37 unaccepted tasks. The historical native-platform limitations above are not superseded by portable value tests.

## 2026-09-19 CORE-01.T03 record-binding increment

PR #820 adds `StateStore::stage_record_bound_outbox` plus read-only contract accessors needed to compare the proposal, approval record and durable operation before the existing atomic outbox transaction. This helper is explicitly a record-binding check, not executable authority or user consent: runtime-owner, current authority, policy and provider gates remain separate and must pass before any external effect.

The first green source `b9761a812d12a397c0fa6e0713086a94f8ab3ad8` was superseded after source review found a correctness gap: it compared stored argument digests but did not hash `NewOutboxMessage.payload` or compare its byte length with the canonical artifact. Green CI therefore did not establish the intended binding. The corrected source `579d12c94d336edb40239ffbfed7e0dcf33d7c89` derives the proposal fixture digest from its real canonical bytes, checks the canonical artifact hash against the proposal hash, checks the exact staged payload length, and hashes the exact staged bytes before outbox insertion. Same-size changed bytes and different-size changed bytes are both rejected before mutation.

The durable operation must still be approved and must match the proposal ID, task, account, capability and argument hash. The approval record must bind the same proposal and exact argument hash, be effective at the staging time and remain unexpired; proposal expiry is enforced too. Targeted proposals fail closed while `DurableOperation` lacks durable target-resource identity. Regressions verify successful exact-byte staging, payload rebinding, account rebinding, target rebinding and proposal/approval expiry. Every rejected case verifies no outbox row, no operation transition and no extra journal record.

Corrected source `579d12c94d336edb40239ffbfed7e0dcf33d7c89` passed CI run 35438741589: pinned formatting, strict workspace Clippy, workspace tests, doctests, fuzz-target compilation, architecture checks and contract conformance, plus native Ubuntu 24.04, Windows 2025 and macOS 15 contract/storage jobs. Instrumented fuzz run 35438741631 passed `frame_decoder`, `control_envelope` and `record_codec`; these bounded parser runs are not authorization or production qualification.

This increment does not complete CORE-01.T03 or CORE-02.T03. The lower-level `stage_outbox` primitive remains publicly callable, transport destination is not yet bound to an approved canonical action descriptor, target identity is not durable, and source preconditions plus a canonicalization-version binding are not represented. Claimed-message exposure, durable abandoned-attempt recovery and later outbox acceptance criteria also remain under CORE-02. The fresh `@codex review` request has been quota-blocked so far, so there is no independent review or task acceptance. `--require-accepted` remains unchanged and no task is marked accepted or closed.

## 2026-09-19 CORE-01.T03 staging time and rejection regressions

Source `24cca47273cca5b7e6480ff281a56af2bbc142e6` preserves the concurrent exact-payload fix and also requires the persisted staging timestamp to equal the validation timestamp and not predate the durable operation's latest transition. Two new regressions failed on the preceding implementation: validation at 120 could persist staging at expiry 200, and staging at 109 could precede the durable approval transition at 110. Oversized payloads now fail before hashing, using the existing outbox byte budget.

Seven additional test functions cover both temporal failures, every persisted proposal identity, approval identity/hash/state, stale revisions, oversized inputs, injected insertion rollback and file-backed dispatch denial after importing an `Approved` record. The complete record-binding suite passes 12 tests. The file-backed test preserves the separate runtime gate; it is not evidence that the imported record supplies consent or that the trusted provider path is qualified.

The pinned Rust 1.98.1 locked local checks passed 340 workspace test results, 12 doctests, 25 Python tests, strict Clippy, both formatters, architecture, conformance, fuzz-target compilation and the coverage inventory. The ignored owned-process child entry point is invoked by its passing parent and is not counted as a separate pass. `--require-accepted` still exits 1 with 37 unaccepted tasks.

Exact-head CI run 35439127274 passed all four jobs, including Ubuntu 24.04, Windows 2025 and macOS 15. The downloaded source archive reproduces tree `e1f42fb3bddcd00363a912ef9203d0cf17adda5f`; its tested merge is `9568b3d9e0a9ac2563b5e1777a4372bee224d43b` against main `06c8b5694e01105afc2b800476c98a9b5084dae1`. Fuzz run 35439127267 passed three instrumented 60-second parser targets: 2,461,521 frame-decoder inputs, 4,028,353 control-envelope inputs and 2,238,312 record-codec inputs. Archive hashes, job IDs, source hashes and execution limits are retained in [the verification report](verification/core-outbox-binding-20260919.json).

The fresh review request on this source returned a quota-exhaustion response, not an approval. The payload and staging-time findings are addressed by source and regression evidence, but independent acceptance and every remaining runtime, destination, target, source-version and platform qualification gate above remain open. No task is closed or marked accepted by this increment.
