# Implementation run

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

Completion requires all 37 CORE tasks to satisfy their individual criteria and the acceptance ledger gate before subsequent workstreams. The existing ledger reports zero accepted tasks. Platform, independent review, real-provider and power-loss criteria require their own evidence; unit test success cannot replace them.

## Cross-platform architecture

The user explicitly requires cross-platform support. Native Windows tests are required alongside Linux and macOS qualification. Docker execution alone cannot establish this.

- [ ] Ground. Trace the existing storage and supervisor OS boundaries.
- [ ] Sketch. Compare platform backend designs against authentication, durability and native testability.
- [ ] Agree. Proceed autonomously as requested.
- [ ] Implement. Build and test independently verifiable native platform units.
- [ ] Scrap. Revisit the design if OS-specific assumptions leak into shared policy.

Design comparison phases: Frame, Fan out, Cross-judge, Pick, Graft, Verify. Prefer a small internal backend over duplicating state machines. Reject any candidate that substitutes successful no-ops for unavailable durability or authentication guarantees.

Current implementation and verification status: docs/core-native-progress.md. The three saved patches are integrated. Native Windows verification passed within the recorded scope; Linux/macOS/Windows CI for the CORE-01.T01 baseline passed on PR #817. CORE acceptance remains open, and no later workstream has started.
