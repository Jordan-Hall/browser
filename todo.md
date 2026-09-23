# Implementation run

Standing instruction, 2026-09-19: commit verified work and continue until the user explicitly stops execution or every epic and subtask is done. Finish CORE implementation and acceptance before starting the next epic. Keep incomplete checks and platform limits visible; do not substitute source publication for task acceptance.

Standing instruction, 2026-09-20: fetch and incorporate `origin/main` before each new implementation unit and before publication. Check merged work before implementing a reported gap. Close GitHub issues as their full task criteria are verified, and proceed to the next epic only after CORE completion.

- [x] Merge main through PRs #840 and #841; retain the distinct local outbound-cancellation and fixture-observation fixes.
- [x] Commit the broker startup deadline separation and outbound cancellation state fix.
- [x] Reproduce and fix final-message observation races in the newly merged process fixtures without extending stop deadlines.
- [x] Publish the verified cancellation changes and inspect exact-head CI; follow-through is merged through #861 with exact-head native CI.
- [x] Complete task-level acceptance audit of CORE-01.T01; CORE-01.T01 through T04 are accepted on the live ledger.
- [x] Merge bounded Windows CORE-01.T05 authentication-lifecycle follow-through through #872: real child spawn/two-lane authentication, pre-auth revocation/stale-generation fencing, and post-auth ownership revocation/fresh-generation recovery; keep T05 open/partial.

- [x] Read the Principles section of the poteto-mode skill.
- [x] Phase A: Frame. Compare the consolidated implementation with the CORE task ledger and live issues.
- [x] Phase B: Design the workflow. Work in stable CORE task order, verify each correction, then continue to later workstreams only after CORE acceptance.
- [ ] Phase C: Run the loop.
  - [x] Establish the pinned toolchain and baseline checks for CORE-01.T01.
  - [x] Reject empty provider identities through constructors and deserialization for CORE-01.T02.
  - [x] Add checked spending amounts and exact narrowing conversions; verify generated wire round trips.
  - [x] Enforce symmetric structural budgets during IPC encoding and decoding.
  - [x] Integrate pending IPC cancellation, durable recovery accounting and process observation patches.
  - [x] Fix suppressed-artifact lookup errors and verify same-ID republishing remains blocked.
  - [ ] Complete remaining CORE-01 contract, migration, channel and conformance criteria.
  - [ ] Complete CORE-02 storage and fault qualification criteria.
  - [ ] Complete CORE-03 supervision and resource criteria.
  - [ ] Complete CORE-04 recovery and replay criteria.
  - [ ] Complete programme and epic integration acceptance.
  - [ ] Proceed to the next workstream in the programme.
- [ ] Phase D: Keep the audit trail. Append decisions and executed evidence to decisions.tsv.
- [ ] Phase E: Verify and hand back. Require actual task evidence and the unchanged require-accepted gate.

Completion requires all 37 CORE tasks to satisfy their individual criteria and the acceptance ledger gate before subsequent workstreams. The live ledger has 4 accepted tasks (CORE-01.T01-T04) and 33 unaccepted tasks; `production_ready=false` and `--require-accepted` remain fail-closed. Platform, independent review, real-provider and power-loss criteria require their own evidence; unit test success cannot replace them.

## 2026-09-19 cancellation transport continuation

- [x] Check PR #824 and preserve its active owner's changes. The user subsequently merged it; the verification baseline is now main `ce06a3aa0364b65b2c1fe4e4d8a05f0721abb09e`.
- [x] Trace the real worker cancellation path. The existing flood test assumes saturation after a sleep and ignores progress admission failures.
- [x] Add a progress-only fixture that reports measured socket backpressure after a parent-controlled start.
- [x] Prove both real workers acknowledge cancellation before their measured progress backlog drains, and an unrelated worker still completes work.
- [x] Run Linux process tests and strict checks; preserve the CORE acceptance gate and record actual evidence. All 64 relevant Linux results passed serially, along with strict Clippy and formatting. Earlier concurrent broker timeout remains unqualified.

The throughput constraint is Linux process verification from Windows. Use the pinned Rust container and native Linux Cargo/build caches. This unit changes only fixture/test code and evidence, on top of merged PR #824. The data shape is a fixture progress producer with waiting, filling and saturated states. Marker files synchronize readiness and record admitted frame counts without adding a runtime API. A timed flood alone cannot prove socket pressure, and runtime metrics would unnecessarily expand the public interface. Both flood fixtures now wait for parent-observed cancellation before exiting; a controlled 30 ms delay reproduced the old fixed-sleep race. Concurrent broker qualification remains open after an existing crash-test timeout; final fixture verification runs serially.

## Cross-platform architecture

The user explicitly requires cross-platform support. Native Windows tests are required alongside Linux and macOS qualification. Docker execution alone cannot establish this.

- [ ] Ground. Trace the existing storage and supervisor OS boundaries.
- [ ] Sketch. Compare platform backend designs against authentication, durability and native testability.
- [ ] Agree. Proceed autonomously as requested.
- [ ] Implement. Build and test independently verifiable native platform units.
- [ ] Scrap. Revisit the design if OS-specific assumptions leak into shared policy.

Design comparison phases: Frame, Fan out, Cross-judge, Pick, Graft, Verify. Prefer a small internal backend over duplicating state machines. Reject any candidate that substitutes successful no-ops for unavailable durability or authentication guarantees.

Current implementation and verification status: docs/core-native-progress.md. CORE-01.T01-T04 are accepted; T05 remains partial and T06-T08 remain implemented_pending_acceptance. CORE-01.T05 Windows follow-through #870-#872 is merged with exact-head native CI for bounded real-child launch/authentication, pre-auth generation fencing and post-auth ownership revocation; it does not establish complete Windows lifecycle parity or any macOS supervisor. CORE-01.T07 #860/#861, CORE-01.T08 #862, and CORE-02.T01 #863-#868 are also merged with exact-head native CI in their documented scopes. CORE-02.T01 remains partial for controlled filesystem/power-loss qualification and independent acceptance. No later workstream starts until CORE closes.
