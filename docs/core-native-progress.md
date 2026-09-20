# CORE native implementation checkpoint

## 2026-09-20 fixture final-message observation

Main through PRs #840 and #841 is merged at `ca712c2`. The blocked-progress and stale-result fixtures now wait for the parent to observe their final messages before exiting. Both former fixed-sleep races were reproduced with a 30 ms parent delay. The stale-result test also seals both executable images before launch, so replacement setup cannot consume cancellation grace. Stop deadlines and stale-result rejection assertions remain unchanged.

Strict Clippy and formatting passed. The combined four-thread Linux run recorded 159 passes and one timeout in the existing 300 ms control-notification test. All three tests in that target passed on a separate four-thread rerun with unchanged limits. The two corrected exit-race scenarios passed in the combined run. Python validation ran 51 tests with one platform-specific skip and passed after a separate rerun; its earlier overlapping run had one subprocess timeout. [The verification report](verification/core-fixture-observation-20260920.json) retains source hashes and both Linux outcomes. These results do not qualify production latency or establish a fully green combined load run.

## 2026-09-20 outbound cancellation under reserved queue pressure

The merged stream API could leave a registration active when its outbound Cancel failed to enter a full reserved queue. It now revokes local dispatch before queue insertion and distinguishes a notification pending send from one awaiting acknowledgement. Retries preserve that state, including simultaneous incoming cancellation and failed retransmission. Queue errors retain the rejected event, and retirement remains blocked until the handshake completes.

The regression failed before the fix with `is_active=true`. All 85 native Windows IPC tests, formatting and strict Clippy passed afterward. [The verification report](verification/core-outbound-cancellation-20260920.json) records hashes and the remaining scope limits. This queue API still has no production runtime caller, and identical-registration reuse after retirement remains a separate issue. No CORE acceptance status changed.

## 2026-09-20 broker crash setup deadline

Latest main through PR #839 is merged at `f258485`. The broker crash test now waits for actual worker readiness before starting its existing eight-second crash-phase deadline. Setup has a separate thirty-second bound. A nine-second setup delay passes both crash phases; the same delay after the parent start still fails the crash-phase deadline. The independent effect and recovery assertions are unchanged.

Formatting, strict Clippy and all 16 broker tests passed on the merged code. The broader four-thread run finished with 73 passing tests and two failures in the newly merged blocked-progress and stale-result fixtures. Those failures remain open; this is not a green full-suite result. Exact source and log hashes are in [the verification report](verification/core-broker-startup-deadline-20260920.json). No task acceptance changed.

## 2026-09-19 measured cancellation backpressure

Local fixture and regression changes build on main `ce06a3aa0364b65b2c1fe4e4d8a05f0721abb09e`, including merged PR #824. A parent-controlled start lets two real cooperative workers fill their progress sockets without supervisor reads. Each reports repeated refused progress admissions and its admitted frame count. The test requires cancellation acknowledgements before that measured backlog drains, immediate lease revocation, successful exit without escalation, and a completed request through an independent worker.

The final run admitted 168 progress frames per worker and observed each acknowledgement after consuming only 8 frames. Its 34.640 ms acknowledgement interval includes an intentional 30 ms parent delay and is not production latency qualification. A container-only mutation that drained all 168 frames failed the new assertion. A separate controlled-delay run reproduced the fixture's former exit-before-acknowledgement race. Both this fixture and the older stdout/stderr flood fixture now wait for a parent release marker after acknowledgement observation.

Pinned Rust 1.98.1 formatting, strict Clippy, 64 Linux supervisor/worker test results and 25 Python validator tests passed. The Linux suite ran serially; one process-observation helper is marked ignored and invoked by its passing parent. An earlier four-thread run hit the existing broker crash-test deadline while executable sealing took 8.8 seconds. Serial success does not qualify that concurrent case. The unchanged release gate still rejects all 37 unaccepted tasks.

At verification time, these were unpublished working-tree changes. Exact source hashes, commands, negative checks and limitations are in [the verification report](verification/core-progress-backpressure-20260919.json). Native Windows/macOS supervision, remote CI, full workspace qualification and independent task acceptance are not claimed. No later epic has started. Earlier entries below retain their original historical scope.

## Earlier checkpoint

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

## 2026-09-19 CORE-01.T01 resolved dependency identity

PR #822 repairs a source-policy bypass in the architecture checker. `cargo metadata --no-deps` described the crates.io declaration even when `[patch]` or `[replace]` selected local code with the same allowed package name. Six new real-command regressions failed on main `85acafd92d12dd43e6bfb6b2fd0d5cc8b2496469`. The corrected command resolves the locked all-feature graph, requires the actual contracts workspace member, and validates both declared and resolved direct package identities. Missing graph/package data fails closed; the existing dependency allowlist is unchanged.

Code `f24f7f08d8b9b0e1e1cbbd88b7ae640c0418cefd` (tree `ee550e7acc4a9d41eb377d2587257d8de2ef7632`) passed 353 local workspace test results, 17 doctests, strict Clippy, both formatters, fuzz-target compilation, architecture/conformance and 25 Python tests with pinned Rust 1.98.1 and locked inputs. All six normal/dev/build/inactive-target/optional/replacement command regressions and four metadata tests passed. A deliberately valid replacement for an ID-separation compile-fail example made the explicit doctest command fail; restoring the exact original source passed.

CI run 35450308409 passed Rust checks and native Ubuntu 24.04, Windows 2025 and macOS 15. Downloaded artifact 10586204809 matched all 209 source files in the tested index and retained the unchanged lockfile hashes. The tested merge was `5ca89429ea9ff4f5fe446234289f216c61833fcf` against the above main commit. Exact hashes, job IDs and test limits are recorded in [the verification report](verification/core-architecture-resolution-20260919.json).

The initial local clean compilation was interrupted by an execution timeout; the complete baseline rerun passed before the fix. Parser-fuzz execution was not triggered by these tooling-only paths, and only fuzz compilation is claimed. Fresh Codex review completed on the code head without additional findings (comment 5742882218); it is not independent task acceptance. This increment does not establish independent task acceptance, transitive supply-chain qualification or runtime/platform safety acceptance. No issue is closed by this checkpoint; `--require-accepted` still exits 1 with 37 unaccepted tasks.

## 2026-09-19 CORE-01.T04 malformed-stream and EOF fail-closed increment

PR #823 makes framing errors terminal for a `FrameDecoder` instance and adds explicit EOF finalization. A valid+invalid+valid byte sequence can no longer return an error and then resume decoding at an ambiguous offset: malformed framing poisons the decoder and clears buffered frame state. `finish()` rejects partial headers and partial payloads with the existing stable `InvalidEnvelope` error class while clean EOF remains successful. The supervisor's nonblocking `FramedSocket` now calls this finalizer before reporting a closed peer, so disconnecting mid-frame is a protocol failure rather than a clean closure. The frame fuzz target finalizes every otherwise-consumed input to cover truncated EOF state.

Exact code `693869fc660ba4dd8e4925e0cb6ef8c2d44affe4` (tree `e8842d65b452b1e3939ba14c78263b6bdc291584`) passed CI run 35452845220. Rust checks passed formatting, strict workspace Clippy, 357 passing workspace tests with one owned-process helper ignored by default, 17 doctests, fuzz-target compilation, architecture/conformance and all 25 Python evidence tests. Native Ubuntu 24.04, Windows 2025 and macOS 15 jobs also passed. The retained artifact 10587655937 has SHA-256 `35b5ffd96c6de19cc656f2bb9d758f6103ba9adbd92a9e164d30d2b1629a6179`; its `source.tar` is the synthetic merge `eca89de6182c4c721649109b6d4f5177460a9867`, whose parents are base `aeb2e1f3b5cdeb77ae183cb828c252376da3f04b` and the exact code head. Lockfile hashes remain unchanged.

Instrumented fuzz run 35452845205 passed all three bounded AddressSanitizer targets for 60 seconds each: `frame_decoder` executed 3,325,714 units with maximum reported coverage 145 (artifact 10587531117, SHA-256 `177847668b0ec9a3c085f3cbfe83df564a477434e6c407502727b6d511d6ab7d`), `control_envelope` executed 4,289,836 units with coverage 1,818 (artifact 10587258024, SHA-256 `1fbc366d6a3e8fe4d5a17100ec0ea735d06c4db3f266e5e578a6e3923f12d71b`), and `record_codec` executed 2,111,393 units with coverage 6,768 (artifact 10586888402, SHA-256 `2b19bd2f121e2be4e2e4bafa5057030cedd0475ec29b55f0550d7318f1ed3654`). These are bounded parser runs, not production qualification.

The first exact-head CI attempt exposed only a rustfmt failure in one new assertion; no pass was claimed from that run. The formatting-only follow-up produced the exact tested source above. Fresh Codex review of `693869fc660ba4dd8e4925e0cb6ef8c2d44affe4` completed without major findings (comment 5743222523), and there are no inline review threads. Local repository execution was unavailable because this automation container could not resolve GitHub, so no local test pass is claimed for this increment.

This does not complete CORE-01.T04. The supervisor still applies per-entry read budgets independently while iterating workers, so aggregate bytes read in one supervisor poll can grow with the number of simultaneous connections; the issue's many-connection aggregate-budget criterion remains open. CORE-01.T07's real transport backpressure separation and independent task acceptance also remain open. No issue is closed and `--require-accepted` remains unchanged with 37 unaccepted CORE tasks.

## 2026-09-20 CORE-03.T05 cancellation persistence failure contract

PR #835 adds an explicit fail-closed result for cancellation that cannot complete its required persistence/termination-request sequence. `RuntimeBroker::cancel_worker` revokes the in-memory lease and latches the worker record revoked before fallible database work. Successful durable revocation is distinguished from an unregistered generation that needed no persistence. If durable cancellation-intent persistence fails, the broker fences all future dispatches, still requests local supervisor cancellation for every owned worker, and reports the target worker's persistence and termination-request outcomes separately. `CancellationTerminationStatus::Requested` means only that the supervisor accepted the cancellation request; it does not claim worker acknowledgement, process-tree exit, external reversal or effect non-commit.

The real-worker regression `cancellation_persistence_failure_is_explicit_fenced_and_recoverable` waits until an independent external ledger proves an external write occurred while the durable operation remains `Attempting` revision 3. It then installs a deterministic SQLite trigger that aborts only the cancellation-intent journal insert. The call returns `persistence=Failed` and `termination=Requested`, the broker is fenced, subsequent dispatch is denied, and the failed transaction leaves the durable operation at the prior `Attempting` revision rather than falsely claiming cancellation. After dropping the broker and reopening the same profile, startup planning classifies that started attempt as `NeedsReconciliation`; independently signed read-only evidence for the exact retained attempt verifies the effect, and the external ledger still contains exactly one row, proving that recovery did not resend the write.

Code head `8dad4f760eb13a055761425c5180602d8e5c0560` (tree `5b28115ff6b60ff92fea328ea518fdf0bd213f77`) passed pinned Rust 1.98.1 locked local checks: 409 workspace test results with one owned-process helper ignored, 17 doctests, strict workspace Clippy, workspace/fuzz formatting, fuzz-target compilation, architecture/conformance and all 25 Python evidence tests. `--require-accepted` continues to exit 1 with all 37 CORE tasks unaccepted.

Exact-head CI run 35486289328 passed Rust checks plus native Ubuntu 24.04, Windows 2025 and macOS 15. Artifact 10597680846 has SHA-256 `5b5fcde29b5999e97265c12fc71547cdd5b0564e8400ac19c74b039d39c537a3`; its tested synthetic merge `cde471a012c20cef4a28a7225058934e2ed6bff6` has base `3d1f14c5dc15afad73e4302b0c3f60345a3c98c7` and exact code head as parents. The retained source matches the changed source and lockfile blobs, with workspace `Cargo.lock` SHA-256 `e4c40cbcdf2e6e9799656518579f7122fc3a74f8efca66b31c23e7912fc66571`. Instrumented parser fuzz run 35486289269 passed `frame_decoder`, `control_envelope` and `record_codec`; these bounded parser jobs do not qualify cancellation correctness or production operation. Fresh Codex review completed on the exact code head without an inline finding. Automated code review is not independent whole-task acceptance.

This increment does not close #141 or #127. The SQLite trigger is deterministic fault injection into the exact cancellation journal path, not qualification of real filesystem-full, storage-stall, power-loss or platform-specific durability behavior. End-to-end acknowledgement/control-queue exhaustion and the required independent task acceptance also remain open. Existing real process-death and lost-response tests plus this exact external-effect reconciliation regression improve the evidence, but no task is marked accepted or production-ready.

## 2026-09-20 CORE-01.T07 / CORE-03.T05 reserved cancellation control

PR #837 addresses a concrete control-queue starvation case that remains after the blocked-progress and durable cancellation increments. A max-sized immediate `Execute` can be partially written within the supervisor's 4 KiB flush budget and retain the single normal control output slot. On main `1d39dc61e17c34fff4f07bed8aea866e406fbb8f`, revoking that worker and running one supervisor poll could not queue `Cancel` because the control socket remained occupied; cancellation notification therefore depended on a later supervisor poll rather than the already-established revoke-before-notify path.

The real launched-worker regression `cancellation_notification_is_not_starved_by_an_occupied_control_slot` performs exactly one supervisor poll after revocation and then waits without further supervisor polling. It failed on the preceding source with `cancel notification remained behind the occupied control slot until another supervisor poll`. The corrected `FramedSocket` keeps the supervisor runtime path unchanged. When stop-time `discard_unstarted` encounters an already-partially-written frame it refuses to splice or discard those bytes, but exposes exactly one bounded queue reservation behind that frame. Because the worker has already entered `Draining` and its lease is revoked, normal application dispatch cannot consume the reservation. The next existing cancellation queue operation occupies it, closes the reservation, and any second queued frame still returns `QueueFull`. `flush()` completes the current frame first and may spend only the remaining bytes from the same caller-supplied budget on the reserved frame.

A wire-level regression proves the partially written frame is delivered first, the reserved control frame follows without byte splicing, and a second queue attempt fails. The real-worker regression observes the actual cancellation marker after the single poll, receives cancellation acknowledgement without signal escalation, and preserves the unresolved request for reconciliation.

Exact code `10273419b9829fece6d445cfbbc8908df089aebe` changes only `crates/intent-supervisor/src/wire.rs` and `crates/intent-fixture-worker/tests/blocked_progress_cancellation.rs`. Pinned Rust 1.98.1 locked local verification passed 413 workspace test results with one owned-process helper ignored, 17 doctests, strict workspace Clippy, workspace/fuzz formatting, fuzz-target compilation, architecture/conformance and all 25 Python evidence tests. `--require-accepted` still exits 1 with all 37 CORE tasks unaccepted. Final source SHA-256 values are `09f3f391f509807be18ae78ebd35ae15a8f1169be0ad9ac8ec67aac66c249c8d` for `wire.rs` and `cf631a55c85220e3bcc9ebed40e663cbcf00c9dd132814e359834f65f270fccc` for the real-worker regression.

Exact-head CI run 35491279021 passed strict Rust checks and native Ubuntu 24.04, Windows 2025 and macOS 15. Artifact 10599545326 has SHA-256 `9c54207e2279c4c30c2edccefacf556e27a0ea0549995fe0e1d14c09580665dd`; its verification pins synthetic merge `620a98f3d8e951325f535859126d7bc4f2de9d42` against base `1d39dc61e17c34fff4f07bed8aea866e406fbb8f`, Rust 1.98.1 and unchanged lockfiles. The retained source archive reproduces both changed-file hashes above. Fresh Codex review completed on the exact code head with no major issues (comment 5747836217), and there are no inline review threads. Automated code review is not independent whole-task acceptance.

This increment does not qualify filesystem-full or physical power-loss behavior, production-provider cancellation, product latency targets, or independent task acceptance, and it does not close #127 or #141. The following evidence-only commit records these results; its own exact-head CI remains a separate pre-merge gate.
