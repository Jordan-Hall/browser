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
