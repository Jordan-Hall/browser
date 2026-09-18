# Programme and ungrouped issues

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 14

## Contents

- [#1 — PROGRAMME: Intent Browser — full implementation roadmap](#issue-1)
- [#111 — [TASK][PROGRAMME.T01] Ratify product invariants and review vocabulary](#issue-111)
- [#112 — [TASK][PROGRAMME.T02] Bootstrap the Rust repository and contribution boundary](#issue-112)
- [#113 — [TASK][PROGRAMME.T03] Freeze core contracts and process authority inventory](#issue-113)
- [#114 — [TASK][PROGRAMME.T04] Establish the executable feasibility programme](#issue-114)
- [#115 — [TASK][PROGRAMME.T05] Build the cross-workstream dependency and ownership graph](#issue-115)
- [#116 — [TASK][PROGRAMME.T06] Establish continuous evaluation and release evidence](#issue-116)
- [#117 — [TASK][PROGRAMME.T07] Integrate the complete desktop platform through P1-P5](#issue-117)
- [#118 — [TASK][PROGRAMME.T08] Integrate device mesh and daily-driver qualification](#issue-118)
- [#119 — [TASK][PROGRAMME.T09] Run the OS distribution and Rust-kernel programme](#issue-119)
- [#120 — [TASK][PROGRAMME.T10] Review this task register and stage later GitHub updates](#issue-120)
- [#798 — Accidental connector test — ignore](#issue-798)
- [#799 — Accidental connector test — ignore](#issue-799)
- [#800 — Accidental connector test — ignore](#issue-800)

---

<a id="issue-1"></a>
## #1 — PROGRAMME: Intent Browser — full implementation roadmap

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/1
**Created:** 2026-09-15T12:03:20Z | **Updated:** 2026-09-17T21:17:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded
**Native child issues:** #13, #14, #16, #24, #25, #26, #28, #30, #31, #32
**PRs mentioning this issue:** [#785](https://github.com/Jordan-Hall/browser/pull/785), [#786](https://github.com/Jordan-Hall/browser/pull/786), [#787](https://github.com/Jordan-Hall/browser/pull/787), [#788](https://github.com/Jordan-Hall/browser/pull/788), [#789](https://github.com/Jordan-Hall/browser/pull/789), [#790](https://github.com/Jordan-Hall/browser/pull/790), [#791](https://github.com/Jordan-Hall/browser/pull/791), [#792](https://github.com/Jordan-Hall/browser/pull/792), [#793](https://github.com/Jordan-Hall/browser/pull/793), [#794](https://github.com/Jordan-Hall/browser/pull/794), [#795](https://github.com/Jordan-Hall/browser/pull/795), [#796](https://github.com/Jordan-Hall/browser/pull/796), [#797](https://github.com/Jordan-Hall/browser/pull/797), [#801](https://github.com/Jordan-Hall/browser/pull/801), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#803](https://github.com/Jordan-Hall/browser/pull/803), [#804](https://github.com/Jordan-Hall/browser/pull/804), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#808](https://github.com/Jordan-Hall/browser/pull/808), [#809](https://github.com/Jordan-Hall/browser/pull/809), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

### Intent Browser implementation programme

#### Product thesis
Services provide authenticated data, domain rules, and executable capabilities. The browser/runtime owns the user's presentation, personal context, orchestration, and authorization. AI proposes representations and plans; deterministic software owns permissions, dispatch, verification, receipts, and recovery.

This repository should **not** become another browser with an AI chat sidebar. The primary product object is a persistent workspace that remains useful after the originating conversation is closed and when inference is unavailable.

#### Product invariants
- Persistent workspace, not conversation, is the durable unit of state.
- Personal / Original / Evidence views remain synchronized but distinct.
- Every rendered action maps to an installed, authorized capability.
- Models and external agents never authorize themselves.
- Local/offline/hybrid modes are explicit; no hidden cloud fallback.
- Source observations, user overlays, and inferred/derived claims remain distinct.
- Risky writes are prepared, approved, committed, independently verified, and reconciled when ambiguous.
- Computer control prefers APIs/semantic accessibility over pixels/coordinates.
- Generated UI is declarative and bounded by default; generated executable code runs only through the extension review/sandbox path.
- Direct UI interactions such as sorting/filtering/navigation do not require an LLM.
- The user can stop execution, inspect accessed data/destinations, correct memory, export data, and return to the authentic source UI.

#### Programme phases
- **P0** — Feasibility, contracts, security model, fixtures
- **P1** — Persistent local-first runtime
- **P2** — Connector-centric personalized interfaces
- **P3** — Complete harness, coding agents, PC execution
- **P4** — Full core feature alpha, trusted writes, automation
- **P5** — Adaptive reusable desktop beta
- **P6** — Multi-device platform, SDK, ecosystem
- **P7** — Daily-driver general release
- **P8** — Dedicated desktop session / OS distribution
- **K0–K4** — Separate Rust-kernel research and delivery programme

#### Engineering strategy
Rust owns the trusted supervisor, contracts, policy/capability broker, semantic/evidence model, workspace state, transaction journal, connector broker, desktop broker, extension host, and evaluation harness. Chromium/CEF, local inference runtimes, speech engines, and external coding agents remain replaceable workers/adapters behind explicit contracts.

#### Release definition
A capability is not "done" because a demo or model says it worked. It is done when its stated acceptance criteria, conformance tests, security constraints, recovery behavior, and supported-platform matrix pass.

#### Anti-wrapper release test
For supported workflows the user must be able to:
1. Create a useful source-backed workspace.
2. Close the conversation and restart.
3. Continue using deterministic workspace interactions.
4. Inspect original sources and evidence.
5. Disable inference without losing the application.
6. Change agent/model providers without losing workspace/task state.
7. Execute only explicitly authorized capabilities.
8. Receive a verified result or an honest unresolved/reconciliation state.

#### Workstream epics
- [ ] #13 Runtime and contracts
- [ ] #14 Authority and security
- [ ] #15 Conventional browser and compatibility
- [ ] #16 Workspace experience
- [ ] #17 Semantic and evidence substrate
- [ ] #18 Connector platform and domain packs
- [ ] #19 Intent-driven UI and portable applications
- [ ] #20 Harness and required provider integrations
- [ ] #21 Built-in local intelligence
- [ ] #22 Speech and contextual interaction
- [ ] #23 Desktop and native execution
- [ ] #24 Coding workspace and artifacts
- [ ] #25 Research and synthesized publications
- [ ] #26 Trusted transactions
- [ ] #27 Shopping, bidding and post-purchase
- [ ] #28 Social, communications and calendars
- [ ] #29 Inspectable memory and learned workflows
- [ ] #30 Schedules, events and attention
- [ ] #31 Sync, mobile and collaboration
- [ ] #32 Extensions, SDKs and ecosystem
- [ ] #33 Evaluation, release and daily-driver quality
- [ ] #34 Standalone OS and Rust-engine programme

#### Backlog structure
Implementation issues use stable IDs (`CORE-*`, `SEC-*`, `WEB-*`, `WS-*`, `DATA-*`, `CONN-*`, `UI-*`, `AGENT-*`, `LOCAL-*`, `VOICE-*`, `PC-*`, `CODE-*`, `RES-*`, `TX-*`, `SHOP-*`, `SOC-*`, `MEM-*`, `AUTO-*`, `MESH-*`, `SDK-*`, `EVAL-*`, `OS-*`). IDs are part of the architecture and should not be recycled.

### Discussion (8 comments)

#### Comment 5681713260 — Jordan-Hall — 2026-09-15T14:15:12Z

Source: https://github.com/Jordan-Hall/browser/issues/1#issuecomment-5681713260 | Updated: 2026-09-15T14:15:12Z

<!-- intent-implementation-v1:PROGRAMME -->
###### Task-by-task implementation proposal — programme

This is the implementation companion to the full plan and `Intent_Browser_Task_by_Task_Implementation_Handbook.md` (review edition 1.0): 110 issue sections and 730 stable task IDs. The repository currently has no committed code/docs; all paths, APIs and tests described are proposed. Original issue descriptions and acceptance criteria remain authoritative.

###### Programme tasks
- [ ] **PROGRAMME.T01 — Ratify product invariants and review vocabulary.** Approve workspace ownership, Personal/Original/Evidence, explicit inference modes and deterministic authorization. Keep `review approved` separate from `implementation verified`. Proof: every workstream maps its behavior to these invariants.
- [ ] **PROGRAMME.T02 — Bootstrap the Rust repository and contribution boundary.** Establish a pinned Cargo workspace, real minimal shell/supervisor/worker entry points, lint/test automation and ADRs. Keep kernel research in a separate build boundary. Proof: clean build without private credentials; no speculative empty-crate forest.
- [ ] **PROGRAMME.T03 — Freeze core contracts and process authority inventory.** Coordinate #2/#6 ownership for workspaces, grants, evidence and external operations. Proof: one authoritative owner per state family and explicit peer/role validation.
- [ ] **PROGRAMME.T04 — Establish the executable feasibility programme.** Prove CEF/shell integration, local speech/inference, all four agent protocols, semantic PC control and confinement on declared hardware. Proof: runnable spikes with failure findings, not vendor claims.
- [ ] **PROGRAMME.T05 — Build the cross-workstream dependency and ownership graph.** Classify dependencies as contract prerequisites, implementation prerequisites, integration gates or external access. Proof: no unexplained cycles and explicit parallel starts.
- [ ] **PROGRAMME.T06 — Establish continuous evaluation and release evidence.** Build resettable fixtures, hostile-content tests, recovery tests and anti-wrapper continuity checks. Proof: independent fixture oracles and production writes technically excluded from replay.
- [ ] **PROGRAMME.T07 — Integrate the complete desktop platform through P1–P5.** Runtime → source-backed interfaces → agents/PC/files → controlled writes/automation → reusable adaptive applications. Proof: saved applications survive chat closure and provider changes; no feature family silently removed.
- [ ] **PROGRAMME.T08 — Integrate device mesh and daily-driver qualification.** Add pairing/sync/home workers/collaboration/mobile/SDK distribution, then browser compatibility and recovery qualification. Proof: no cross-device duplicate commit or source-rights leakage.
- [ ] **PROGRAMME.T09 — Run the OS distribution and Rust-kernel programme.** Maintain P8 and K0–K4 with distinct hardware/ABI/driver/inference milestones and owners. Proof: real workloads and deterministic recovery with models disabled.
- [ ] **PROGRAMME.T10 — Review and maintain the implementation backlog.** Review individual stable task IDs before promoting them to native sub-issues; preserve existing hierarchy and concurrent edits. Proof: coverage and dependency validation plus a reviewed change record.

###### Rules for subsequent issue updates
Treat task IDs as stable references. Preserve original scope; add reviewed implementation details instead of replacing requirements with a summary. Each completed task needs its PR/commit, test evidence, tested versions/platforms and limitations. Native parent/child relationships and blocking dependencies are separate concepts.

###### Cross-cutting corrections from the handbook review
Do not claim same-user IPC checks are a sandbox, GPU budgets are universally hard-enforceable, local CLIs imply local inference, signatures prove safety, or outboxes provide exactly-once merchant actions. Reconciliation must retain unresolved financial commitments. Credentials required by a confined provider are an explicit scoped exception—not a reason to expose the user's vault. All implementation decisions remain reviewable; nothing here marks software as completed.

#### Comment 5706197431 — Jordan-Hall — 2026-09-16T23:43:30Z

Source: https://github.com/Jordan-Hall/browser/issues/1#issuecomment-5706197431 | Updated: 2026-09-16T23:43:30Z

<!-- intent-core-review:2026-09-17:programme-summary -->
###### CORE review summary — defects, missing implementation and review gates

**CORE is not fully implemented or production-ready.** This review covers the CORE-01–CORE-04 requirements, their 32 task issues, the runtime epic/integration tasks, the programme contract-baseline task and the 14 existing CORE task PRs. Review comments are not implementation, fixes, merge approval or passing qualification.

###### Start here
- [Runtime epic assessment and correction order](https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5706192648)
- [CORE-01: contracts, IPC, authentication and conformance](https://github.com/Jordan-Hall/browser/issues/2#issuecomment-5706164060)
- [CORE-02: state, outbox, artifacts, retention and backups](https://github.com/Jordan-Hall/browser/issues/3#issuecomment-5706155786)
- [CORE-03: supervisor and resource enforcement](https://github.com/Jordan-Hall/browser/issues/4#issuecomment-5706147272)
- [CORE-04: checkpoints, reconciliation, replay and recovery](https://github.com/Jordan-Hall/browser/issues/5#issuecomment-5706139701)

###### Actual implementation coverage
| Requirement | Task issues | Existing task PRs | Review status |
| --- | --- | --- | --- |
| CORE-01 | #121–#128 | #785–#792 | Code exists; material contract/authentication/codec/cancellation and evidence gaps remain. |
| CORE-02 | #129–#136 | #793–#797 and #801 | T01–T06 have code; T07/T08 have no PR in the reviewed inventory. Retention #801 is failing tests. |
| CORE-03 | #137–#144 | None found | Planning/acceptance review, not review of an implemented supervisor. |
| CORE-04 | #145–#152 | None found | Planning/acceptance review, not review of an implemented recovery subsystem. |

All fourteen PRs were open in the reviewed inventory. That is fourteen of thirty-two feature tasks with PRs, **not fourteen completed production tasks**. Integration tasks #211–#214 and baseline #113 also have explicit review requirements.

###### Most consequential findings
- Approved action bytes/account/capability/target are not yet bound consistently through contract → operation → outbox; attempt identity and compensation lineage can be lost (#787/#794/#795).
- Real launched-worker authentication and negotiated wire codecs are not yet integrated correctly (#789/#790). The two-worker cancellation test in #791 actually uses in-memory values.
- A populated unrelated zero-application-ID database can be adopted/modified; migrations and profile ownership need stronger coordination (#793).
- Abandoned attempting outboxes have no recovery path; cancelled pending rows can starve later work (#795).
- Retention suppression relies on the affected-row count of an INSTEAD OF view update, causing rollback; post-unlink eligibility checks cannot protect against a competing writer (#801). CI run 35086737317 at head fbfcfceb17540e5fe80bec555385e097f3787455 fails at tests.
- The published fuzz target has an undeclared dependency; current conformance/CI coverage does not establish the advertised runtime invariants (#792). Add locked dependencies, explicit doctests, fuzz builds, actual subprocess tests and failure artifacts.

Previously fixed inbox aggregate-read and artifact retry/scope/directory-sync findings are acknowledged rather than duplicated as unfixed. Remaining defects and improvements have owning task/PR references, suggested implementation changes and acceptance tests in the review comments.

###### Next implementation gate
Address blockers in the existing CORE stack in stable-ID order; preserve each task's PR/evidence links; then implement missing backup, fault qualification, supervision and recovery. Validate the full integrated head and record a disposition for every actionable review finding with its fix commit/test or specific reason not changed. Do not close a task solely because its title appears on a PR or an earlier head was green.

**Scope of this action:** source/issue/discussion review and GitHub comments only. No source commits, branches, PR state changes, merges, issue closures or false Fixed replies were made. Rust/fuzz binaries were not executed locally; existing CI and source evidence are distinguished from proposed future tests.

#### Comment 5706739120 — Jordan-Hall — 2026-09-17T00:51:24Z

Source: https://github.com/Jordan-Hall/browser/issues/1#issuecomment-5706739120 | Updated: 2026-09-17T00:51:24Z

###### CORE review follow-up completed — comment index and remaining blockers

**This pass adds 57 review comments across 43 issues and 14 PRs, including this index.** Scope: CORE-01–CORE-04, all 32 feature tasks, runtime epic #13 and its four integration tasks, programme baseline #113, and this programme issue. Findings are tied to the reviewed code or explicitly labelled as planning/acceptance refinements where implementation does not yet exist.

**CORE is still not fully implemented or production-approved.** The refreshed PR inventory contains 14 open task PRs, covering CORE-01.T01–T08 and CORE-02.T01–T06; no implementation PRs were found for the other 18 feature tasks or the integration work. A PR's existence is not proof that its acceptance criteria pass.

###### Requirement and integration navigation
| Review area | Follow-up |
| --- | --- |
| CORE-01, #121–#128 | [Contracts, IPC, authentication, conformance](https://github.com/Jordan-Hall/browser/issues/2#issuecomment-5706716021) |
| CORE-02, #129–#136 | [Storage, durable outcomes, artifacts, retention, backups](https://github.com/Jordan-Hall/browser/issues/3#issuecomment-5706719472) |
| CORE-03, #137–#144 | [Supervisor, resources, cancellation and qualification](https://github.com/Jordan-Hall/browser/issues/4#issuecomment-5706721638) |
| CORE-04, #145–#152 | [Checkpoints, recovery, reconciliation, replay and UX](https://github.com/Jordan-Hall/browser/issues/5#issuecomment-5706724466) |
| Runtime epic | [Integration priorities and readiness](https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5706732410) |
| Baseline/integration tasks | #113 and #211–#214 each have separate follow-up comments. |

###### Every existing CORE PR has a follow-up
| Task / issue | PR review |
| --- | --- |
| CORE-01.T01 / #121 | [#785: executable build/architecture gates](https://github.com/Jordan-Hall/browser/pull/785#issuecomment-5706416708) |
| CORE-01.T02 / #122 | [#786: value and executable-money boundaries](https://github.com/Jordan-Hall/browser/pull/786#issuecomment-5706419337) |
| CORE-01.T03 / #123 | [#787: exact authorization binding](https://github.com/Jordan-Hall/browser/pull/787#issuecomment-5706421555) |
| CORE-01.T04 / #124 | [#788: encode/decode closure and budgets](https://github.com/Jordan-Hall/browser/pull/788#issuecomment-5706423641) |
| CORE-01.T05 / #125 | [#789: channel-bound worker identity](https://github.com/Jordan-Hall/browser/pull/789#issuecomment-5706425677) |
| CORE-01.T06 / #126 | [#790: negotiated codecs and no-op migration validation](https://github.com/Jordan-Hall/browser/pull/790#issuecomment-5706433095) |
| CORE-01.T07 / #127 | [#791: cancellation lifetime and reliable outcomes](https://github.com/Jordan-Hall/browser/pull/791#issuecomment-5706445404) |
| CORE-01.T08 / #128 | [#792: coverage manifest and mutation-tested gates](https://github.com/Jordan-Hall/browser/pull/792#issuecomment-5706450338) |
| CORE-02.T01 / #129 | [#793: existing-store identity and ownership](https://github.com/Jordan-Hall/browser/pull/793#issuecomment-5706493426) |
| CORE-02.T02 / #130 | [#794: committed mutation versus returned outcome](https://github.com/Jordan-Hall/browser/pull/794#issuecomment-5706500421) |
| CORE-02.T03 / #131 | [#795: duplicate result acknowledgements without redispatch](https://github.com/Jordan-Hall/browser/pull/795#issuecomment-5706529974) |
| CORE-02.T04 / #132 | [#796: cursor bootstrap, rebuild and materialization](https://github.com/Jordan-Hall/browser/pull/796#issuecomment-5706535373) |
| CORE-02.T05 / #133 | [#797: P1 false-success reference registration](https://github.com/Jordan-Hall/browser/pull/797#issuecomment-5706481921) |
| CORE-02.T06 / #134 | [#801: failing retention, maintenance budget and UTF-8 failure handling](https://github.com/Jordan-Hall/browser/pull/801#issuecomment-5706541001) |

###### Highest-priority findings
**New P1:** artifact reference registration can return success with no reference inserted because broad `INSERT OR IGNORE` suppresses CHECK violations for empty identifiers. This can invalidate the assumptions used to protect receipts, checkpoints and backups from GC. A reduced SQLite reproduction confirmed the SQL behavior; the fix needs a Rust regression in the repository.

**Existing blockers remain:** exact approved-action/account/attempt binding; real launched-worker authentication and negotiated codec integration; abandoned outbox attempts and generation fencing; profile-wide storage ownership; suppression/GC coordination. The previous aggregate inbox-read fix and three artifact fixes are present and were not incorrectly relabelled as unfixed.

**Current failing evidence:** #801 head `fbfcfceb17540e5fe80bec555385e097f3787455`, CI run `35086737317`, job `104763204222`: formatting and Clippy pass, but all three retention tests fail with `ConcurrentSuppression`. This is not a green retention implementation.

**Additional source findings:** missing existing-store metadata silently regenerates identity; operation mutations reload after commit and can return misleading errors/results; encoder/decoder structural limits differ; no-op migrations return unvalidated bytes; hold pruning ignores the requested maintenance budget; error truncation can panic at a non-UTF-8 boundary.

###### Next implementation gate
Repair owning tasks in the existing CORE order and propagate fixes through affected stacked PRs. Then implement backup/fault qualification, supervisor and recovery, followed by #211–#214 integration. Each actionable finding needs a fixing commit and regression evidence, or a precise reason not changed with its affected support/completion claim. A reply alone does not fix a defect.

**Method and boundaries:** source, issue, PR discussion and existing CI review, plus reduced in-memory SQLite reproductions. Rust, fuzzing, subprocess and power-loss suites were not executed locally in this review. No code commits, branches, new PRs, merges, issue closures or false `Fixed` replies were made. Review completion is separate from implementation completion.

#### Comment 5711140796 — Jordan-Hall — 2026-09-17T08:10:07Z

Source: https://github.com/Jordan-Hall/browser/issues/1#issuecomment-5711140796 | Updated: 2026-09-17T08:10:07Z

<!-- intent-core-review:2026-09-17:closure-register-v1 -->
###### CORE review continuation — task-by-task remediation and verification register

**The review is published; CORE implementation is not complete.** This register links the existing detailed issue findings to the newly submitted commit-anchored PR review dispositions. It does not replace the issue requirements, assert fixes, or approve merging.

###### Coverage and evidence

The review set contains **43 issues and 14 existing task PRs**: programme #1, requirements #2–#5, epic #13, programme task #113, feature tasks #121–#152, and epic integration tasks #211–#214. Detailed review comments already exist across those issues; this continuation avoids reposting their full text.

Fourteen of the 32 feature tasks have implementation PRs. **The other 18 still have no matching implementation PR in the retrieved inventory:** #135–#152. PR existence and a green lint/test job are not task completion. The retrieved CORE tip is `fbfcfceb17540e5fe80bec555385e097f3787455` on #801; its latest retrieved CI run [35086737317](https://github.com/Jordan-Hall/browser/actions/runs/35086737317) failed.

This continuation used repository source, current PR heads, existing discussions and CI evidence. It did not run Rust locally, modify source, resolve findings as fixed, change issue states, or merge anything.

###### CORE-01 — contracts and IPC; parent #2

Full requirement review: https://github.com/Jordan-Hall/browser/issues/2#issuecomment-5706164060.

| Task | Issue / implementation PR | Required correction and acceptance evidence |
|---|---|---|
| CORE-01.T01 | #121 / #785 | Fail-closed dependency policy exercised through the actual checker; lockfile and locked CI; explicit doctests; documented platform coverage. |
| CORE-01.T02 | #122 / #786 | Exact money through the real codec, including wide integers and unknown scale; non-empty opaque provider identifiers; checked persistence/display conversions. Preserve signed generic amounts, with separate executable-spend validation. |
| CORE-01.T03 | #123 / #787 | Reject canonical-artifact/approved-hash disagreement on construction and import; explicit authority-field policy; validated state/time constraints; checked wire/durable-state projection; per-family fixtures. |
| CORE-01.T04 | #124 / #788 | Bounded serialization during emission; schema check before version-specific payload decode; explicit error-code codec; duplicate-key rejection/exact-number policy; deterministic malformed-stream behavior; matching encode/decode limits. |
| CORE-01.T05 | #125 / #789 | Real launched-worker authentication, not socketpair creator-PID evidence; registry-owned one-use bootstrap; fresh epochs; negative process/channel tests on each supported OS. |
| CORE-01.T06 | #126 / #790 | Authenticated sessions negotiate only implemented codecs; bounded offers; family/version validation on transformed and no-op migrations; lossless newer-document handling. |
| CORE-01.T07 | #127 / #791 | Non-bypassable queue limits; aggregate byte accounting; cancellation-record retirement; positively active epoch checks; actual two-process cancellation under saturated IO. |
| CORE-01.T08 | #128 / #792 | Fix/build both fuzz targets; validate all claimed invariants rather than smoke coverage alone; preserve structured failure reports; identify commit, target and fixture/lock digests. |

###### CORE-02 — durable state, artifacts and retention; parent #3

Full requirement review: https://github.com/Jordan-Hall/browser/issues/3#issuecomment-5706155786.

| Task | Issue / implementation PR | Required correction and acceptance evidence |
|---|---|---|
| CORE-02.T01 | #129 / #793 | Refuse unrelated populated zero-ID databases without modifying them; select migrations under ownership/transaction exclusion; reconcile user_version with the ledger; enforce profile-wide ownership/coordination. |
| CORE-02.T02 | #130 / #794 | Preserve started-attempt identity and compensation lineage; store actual source preconditions separately from schema versions; one transactional transition owner; exhaustive state/attempt and journal-consistency tests. |
| CORE-02.T03 | #131 / #795 | Bind the authorized action to staged transport; require a durable started-attempt dispatch token; fence stale claims/results; enumerate abandoned attempts; retire cancelled outboxes; verify interruption without duplicate fixture effects. |
| CORE-02.T04 | #132 / #796 | Preserve the fixed aggregate-effect read check. Also detect missing materialization on duplicate delivery, cap total effect bytes, separate delivery cursor from source revision, and qualify account/projector identity. |
| CORE-02.T05 | #133 / #797 | Preserve the three original fixes. Bind/protect the profile root; qualify path/permission and platform-durability boundaries; coordinate bounded orphan cleanup with ingestion/GC/backup; document active-reader semantics. |
| CORE-02.T06 | #134 / #801 | Fix suppression rollback caused by view affected-row assumptions; explicit tombstone/lifecycle semantics; non-starving GC scans; exclusion before irreversible unlink; holds/references/live-handle and crash-race tests. **Current retrieved CI fails.** |
| CORE-02.T07 | #135 / no PR found | Implement consistent authenticated DB/blob backup, retention pins, verified fresh-directory restore and dispatch-disabled recovery review; test missing/corrupt blobs and concurrent GC. |
| CORE-02.T08 | #136 / no PR found | Implement fault injection and process/VM interruption qualification with an independent effect ledger. Record failure convergence; distinguish process-kill evidence from actual power-loss evidence. |

###### CORE-03 — supervision and scheduling; parent #4

Full requirement review: https://github.com/Jordan-Hall/browser/issues/4#issuecomment-5706147272. No CORE-03 implementation PR was found; the following are implementation gates, not already-fixed bugs.

| Task | Issue | Required implementation and evidence |
|---|---|---|
| CORE-03.T01 | #137 | Immutable verified launch descriptor and registry-owned fresh worker generation; changed executable/role cannot reuse authorization. |
| CORE-03.T02 | #138 | Spawn/handshake/readiness/drain/exit lifecycle; distinct bounded diagnostics for timeout, silence, blocked progress and OS exit; child reaping. |
| CORE-03.T03 | #139 | Count-and-byte admission, reserved control/interactive/speech capacity, bounded fair background work; prove Stop remains serviceable under saturation. |
| CORE-03.T04 | #140 | Install supported containment before untrusted execution; distinguish hard limits, estimates and cooperative GPU controls; test descendant coverage and unsupported-profile refusal. |
| CORE-03.T05 | #141 | Revoke epoch before cancellation delivery; atomic dispatch/revocation ordering; reject late authority while retaining already-started uncertain external outcomes. |
| CORE-03.T06 | #142 | Bounded restart/backoff and explicit reset conditions; fresh handshake/checkpoint after replacement; visible degraded service without duplicate effects. |
| CORE-03.T07 | #143 | Content-free, bounded-cost resource observation with stale/unavailable states; cooperative yield/unload hysteresis; measured foreground/speech latency. |
| CORE-03.T08 | #144 | Real subprocess concurrency, saturated channels, launch/stop churn, suspend/resume, replacement/revocation races and process-tree termination evidence. |

###### CORE-04 — recovery and replay; parent #5

Full requirement review: https://github.com/Jordan-Hall/browser/issues/5#issuecomment-5706139701. No CORE-04 implementation PR was found.

| Task | Issue | Required implementation and evidence |
|---|---|---|
| CORE-04.T01 | #145 | Exhaustive recovery classification with evidence requirements; unknown writes never become safe retries by default; idempotency scoped to provider/account/action/key/validity. |
| CORE-04.T02 | #146 | One consistent checkpoint revision across task/graph/artifact/cursor/attempt references, with retention coordination and no credentials/live handles. |
| CORE-04.T03 | #147 | Startup dispatch barrier and revisioned recovery plans; current policy/account/deadline/freshness checks; repeated startup converges without fresh unintended effects. |
| CORE-04.T04 | #148 | Evidence-bound local/provider reconciliation; authoritative non-commit distinct from absent/stale query results; original and compensation attempts remain distinguishable. |
| CORE-04.T05 | #149 | Replay structurally lacks production credentials/write adapters; missing capture fails rather than enabling live fallback; injected time/random/provider events and explicit divergence. |
| CORE-04.T06 | #150 | Actual negotiated provider resume or explicit reseed/block; fresh epoch and access/repository-base validation; durable workspace identity preserved independently of provider internals. |
| CORE-04.T07 | #151 | Typed recovery presentation and state-checked actions; distinguish stopped, uncertain, login-required, verified and compensated; no blind Retry for unknown writes. |
| CORE-04.T08 | #152 | Independent fixture-effect ledger plus interruption/revocation/missing-blob/schema/model/disk-failure matrix. Keep fixture, process, platform and real-provider evidence separate. |

###### Integration tasks remain necessary

| Task | Issue | Closure evidence |
|---|---|---|
| PROGRAMME.T03 | #113 | Explicit authoritative owners for contracts, grants, workspaces, operations and evidence; resolve contract disagreements before freezing the baseline. |
| EPIC-CORE.T01 | #211 | Ratified ownership and wire baseline with a real authenticated two-worker handshake. |
| EPIC-CORE.T02 | #212 | Durable dispatch, worker epochs, budgets and cancellation connected end to end without lost state or increased authority. |
| EPIC-CORE.T03 | #213 | Crash after fixture acceptance, checkpoint restore and production-free replay; no duplicate irreversible effect. |
| EPIC-CORE.T04 | #214 | Current reviewed-commit/support matrix, operational recovery/degraded-mode documentation and measured resource/control qualification. |

###### Exact PR review anchors

| PR | Reviewed head | Commit-anchored disposition |
|---|---|---|
| #785 | `fc7b62456f0d` | https://github.com/Jordan-Hall/browser/pull/785#pullrequestreview-5232826013 |
| #786 | `a9fbf6648214` | https://github.com/Jordan-Hall/browser/pull/786#pullrequestreview-5232829195 |
| #787 | `7c9d1e441b55` | https://github.com/Jordan-Hall/browser/pull/787#pullrequestreview-5232833081 |
| #788 | `f76de0034641` | https://github.com/Jordan-Hall/browser/pull/788#pullrequestreview-5232835894 |
| #789 | `45c4fef9dbb6` | https://github.com/Jordan-Hall/browser/pull/789#pullrequestreview-5232838430 |
| #790 | `9134df9db63b` | https://github.com/Jordan-Hall/browser/pull/790#pullrequestreview-5232841028 |
| #791 | `d1371e206495` | https://github.com/Jordan-Hall/browser/pull/791#pullrequestreview-5232843903 |
| #792 | `f04d5d5b7998` | https://github.com/Jordan-Hall/browser/pull/792#pullrequestreview-5232846202 |
| #793 | `c468a457da54` | https://github.com/Jordan-Hall/browser/pull/793#pullrequestreview-5232849484 |
| #794 | `a01286c2acf4` | https://github.com/Jordan-Hall/browser/pull/794#pullrequestreview-5232853828 |
| #795 | `3c7c848014e9` | https://github.com/Jordan-Hall/browser/pull/795#pullrequestreview-5232856724 |
| #796 | `1fc177f601c6` | https://github.com/Jordan-Hall/browser/pull/796#pullrequestreview-5232858794 |
| #797 | `07936f3eecbb` | https://github.com/Jordan-Hall/browser/pull/797#pullrequestreview-5232861422 |
| #801 | `fbfcfceb1754` | https://github.com/Jordan-Hall/browser/pull/801#pullrequestreview-5232863573 |

###### How to work this register one task at a time

Preserve the existing stable-ID order: repair the owning CORE-01 tasks, then CORE-02; complete the missing backup/fault work, CORE-03 and CORE-04, then qualify the integrated runtime. Carry prerequisite fixes forward through dependent branches; a fix in a later branch does not automatically repair an earlier PR.

For each actionable review finding, the implementation PR should record **Fixed — commit + regression evidence**, **Not changed — precise technical reason**, or **Blocked — named dependency/evidence still required**. A blocked finding is not resolved merely because it was acknowledged. Keep existing verified fixes and add regressions for the remaining failure paths.

Only sign off a task against its current head after its real acceptance criteria, positive permitted-work cases, negative/race cases and supported-platform evidence pass. Refresh stale issue-body status/PR links as a separate housekeeping action; do not infer completion from those labels. The ultimate integration gate remains #13, not a count of open PRs or successful smoke checks.

#### Comment 5714287323 — Jordan-Hall — 2026-09-17T12:23:29Z

Source: https://github.com/Jordan-Hall/browser/issues/1#issuecomment-5714287323 | Updated: 2026-09-17T12:23:29Z

<!-- intent-core-review:2026-09-17:verified-index -->
###### CORE review index — all issues and existing PRs covered

The detailed review comments are present across **43 issue resources and all 14 existing CORE task PRs**. I verified that coverage and added a [consolidated task → PR → reviewed-head → correction-gate matrix on the runtime epic](https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5714280316).

Use these requirement reviews for the task-by-task implementation corrections:
- [CORE-01: contracts, wire protocol, authentication and conformance](https://github.com/Jordan-Hall/browser/issues/2#issuecomment-5706164060), tasks #121–#128, PRs #785–#792.
- [CORE-02: durable state, outbox/inbox, artifacts, retention and backup](https://github.com/Jordan-Hall/browser/issues/3#issuecomment-5706155786), tasks #129–#136, existing PRs #793–#797 and #801.
- [CORE-03: supervisor, scheduler, platform limits and cancellation](https://github.com/Jordan-Hall/browser/issues/4#issuecomment-5706147272), tasks #137–#144; implementation still absent from the reviewed PR inventory.
- [CORE-04: checkpoints, reconciliation, replay and recovery](https://github.com/Jordan-Hall/browser/issues/5#issuecomment-5706139701), tasks #145–#152; implementation still absent from the reviewed PR inventory.

**Review coverage is complete; CORE implementation is not.** Eighteen of the 32 feature tasks have no implementation PR, and the fourteen existing PRs still contain documented blockers or qualification gaps. The inspected integrated head #801 (`fbfcfceb17540e5fe80bec555385e097f3787455`) still fails its test step in [run 35086737317](https://github.com/Jordan-Hall/browser/actions/runs/35086737317).

Priorities remain the existing stable-ID order: repair contract/authentication/codec/queue correctness, then operation identity/outbox/storage/retention invariants, then complete backup/fault qualification, supervision and recovery. Each fix needs an owning task, commit, regression evidence and a disposition on the relevant review finding; carry fixes through the stacked branches before claiming integrated completion.

This action was review and GitHub comments only. It did not change implementation code, merge PRs, close issues or mark outstanding findings Fixed.

#### Comment 5717454436 — Jordan-Hall — 2026-09-17T16:03:45Z

Source: https://github.com/Jordan-Hall/browser/issues/1#issuecomment-5717454436 | Updated: 2026-09-17T16:03:45Z

<!-- intent-core-inline-review-programme:2026-09-17 -->
###### CORE review follow-through — concrete PR suggestions posted

The [new CORE epic audit and task-by-task completion ledger](https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5717448842) links **14 newly submitted PR reviews, 28 line-anchored comments and 21 GitHub suggestion blocks** across #785–#797 and #801. It distinguishes already-fixed findings from remaining defects and includes actual replacement code for contract/IPC validation, CI coverage, migration locking, attempt continuity, outbox/inbox bounds and artifact/retention durability.

**CORE is not complete and must not be represented as complete.** Of the 32 feature tasks, 14 have open PRs; #135–#152 are 18 tasks without implementation PRs in the reviewed inventory. Baseline #113 and integration/readiness tasks #211–#214 also remain evidence gates. The posted audit retains the exact implementation and acceptance requirements rather than closing planning items to inflate completion.

The inspected [CI run 35086737317](https://github.com/Jordan-Hall/browser/actions/runs/35086737317/job/104763204222) fails three retention tests with ConcurrentSuppression. Formatting and Clippy pass, but later architecture/conformance/report-upload steps are skipped. The one-line underlying-table suggestion addresses that immediate failure; it does not resolve the concurrent publisher/reference/hold-versus-unlink race, worker authentication/supervision, abandoned-attempt recovery, backup/restore or platform/restart qualification.

These were review/comment writes only: **no source commits, approvals, merges or issue closures**. Suggested Rust code was not compiled locally because Rust/cargo were unavailable. Apply and validate the compatible amendments on their owning branches, integrate/rebase the stack, complete the missing implementation tasks, and attach executable acceptance evidence before closing the CORE parents or epic. Proposal approval and review submission are not implementation completion.

#### Comment 5718040515 — Jordan-Hall — 2026-09-17T16:46:42Z

Source: https://github.com/Jordan-Hall/browser/issues/1#issuecomment-5718040515 | Updated: 2026-09-17T16:46:42Z

###### CORE implementation PR created and verified

[PR #802](https://github.com/Jordan-Hall/browser/pull/802) implements the review follow-through on an isolated branch stacked above #801: 33 changed files, contract/IPC and durable-state fixes, coordinated artifact publication/GC locking, regression tests and committed application/fuzz lockfiles.

[CI run 35248222708 passes](https://github.com/Jordan-Hall/browser/actions/runs/35248222708): 115 unit/integration tests, 4 compile-fail doctests, formatting, Clippy, lockfile enforcement, fuzz-target compilation, architecture checks, smoke conformance and evidence upload. This replaces the earlier limitation of unapplied/uncompiled suggestions with committed, CI-compiled and tested changes. Head: `1023b7fd6055ef95520478f93dbf45ffc99b13a7`.

[Full epic implementation update and remaining gates](https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5718036472). **CORE remains incomplete and the PR is draft, not a production release.** Missing tasks #135–#152 and #113/#211–#214, authenticated supervision/fencing, recovery, backup/restore and platform qualification remain required. No existing implementation branch was rewritten, no PR was merged or approved, and no issue was closed.

#### Comment 5721344110 — Jordan-Hall — 2026-09-17T21:17:14Z

Source: https://github.com/Jordan-Hall/browser/issues/1#issuecomment-5721344110 | Updated: 2026-09-17T21:17:14Z

###### CORE stack propagation and task-reference audit

All **15 existing CORE PRs** now have updated source branches and descriptions: #785, #786, #787, #788, #789, #790, #791, #792, #793, #794, #795, #796, #797, #801 and #802. Applicable hardening changes have been moved into their owning PRs, preserving every original branch and predecessor history without force-pushing. Each PR description records its task/parent/epic references, exact head, passing CI and outstanding acceptance requirements.

[PR #802](https://github.com/Jordan-Hall/browser/pull/802) now explicitly references **all 37 individual tasks**: 32 feature tasks, baseline #113 and epic-integration tasks #211, #212, #213 and #214. It includes the complete task-to-PR/remaining-work matrix and links all 15 passing CI runs. [Full epic update](https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5721341683).

The final [integration CI run 35275548059](https://github.com/Jordan-Hall/browser/actions/runs/35275548059) passes against the updated stack: **117 runtime tests and 4 compile-fail doctests**, formatting, Clippy, lockfiles, architecture checks, excluded fuzz compilation, smoke conformance, task-coverage validation and evidence upload. Head `84d2dd0512fb8e6ae02e4b41562676f4b5e8a215`. This includes new duplicate-JSON-key and invalid-artifact-pin regressions, in addition to the earlier hardening fixes.

**CORE remains incomplete and not production-qualified.** Fourteen feature tasks have implementation PRs with unresolved acceptance/integration work; #135 through #152 are 18 feature tasks without implementation in this stack. Baseline/integration gates remain open too. The [committed 37-task ledger](https://github.com/Jordan-Hall/browser/blob/84d2dd0512fb8e6ae02e4b41562676f4b5e8a215/docs/core-task-coverage.md) distinguishes these states. No PR was merged or approved, no issue was closed, and #802 remains draft; task references and passing smoke tests are not completion evidence for missing runtime features.


---

<a id="issue-111"></a>
## #111 — [TASK][PROGRAMME.T01] Ratify product invariants and review vocabulary

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/111
**Created:** 2026-09-15T15:09:59Z | **Updated:** 2026-09-15T15:09:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Parent: #1

Task ID: `PROGRAMME.T01`

#### Objective
Ratify product invariants and review vocabulary

#### Implementation
Approve persistent workspace ownership, Personal/Original/Evidence separation, explicit local/hybrid modes, bounded agents, trusted authorization and independent verification as non-negotiable integration constraints. Define review-approved separately from implemented and verified.

#### Deliverable
Versioned product charter and review-state definitions.

#### Verification
Every workstream can identify how its implementation preserves user-owned state and control when inference is unavailable.

#### Failure / guardrail
Scope decisions must not quietly remove required feature families or replace persistent applications with transient chat outputs.

#### Prerequisite task IDs
None

#### Review state
- Review: pending
- Implementation: not started
- No code or PR is part of this issue-creation pass.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-112"></a>
## #112 — [TASK][PROGRAMME.T02] Bootstrap the Rust repository and contribution boundary

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/112
**Created:** 2026-09-15T15:10:06Z | **Updated:** 2026-09-15T15:10:06Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Parent: #1

Task ID: `PROGRAMME.T02`

#### Implementation
Propose a Cargo workspace, pinned toolchain, minimal shell/supervisor/worker entrypoints, formatting/lint/test automation, architecture decision records and ownership files. Keep kernel research in its own build boundary and do not create dozens of empty crates before interfaces justify them.

#### Deliverable
Repository bootstrap pull-request plan and developer setup.

#### Verification
A clean environment can build and test the initial real modules with no private credentials or undocumented machine dependencies.

#### Guardrail
Repository bootstrap remains proposed work; this issue-creation pass creates no code or PR.

#### Prerequisites
- `PROGRAMME.T01`

Review: pending · Implementation: not started.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-113"></a>
## #113 — [TASK][PROGRAMME.T03] Freeze core contracts and process authority inventory

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/113
**Created:** 2026-09-15T15:10:33Z | **Updated:** 2026-09-17T00:43:12Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #1

Task ID: `PROGRAMME.T03`

#### Implementation
Coordinate CORE-01/SEC-01 records, IPC roles, persistence ownership and threat boundaries before parallel teams implement incompatible variants. Assign one authoritative owner for grants, workspace state, external operations and evidence.

#### Deliverable
Contract baseline and process/data ownership register.

#### Verification
No two services independently own the same approval or transaction truth, and model/provider sessions remain secondary references.

#### Guardrail
Version skew and unknown capabilities fail explicitly rather than being guessed by adapters.

#### Prerequisites
- `PROGRAMME.T01`
- `PROGRAMME.T02`

Review: pending · Implementation: not started.

### Discussion (2 comments)

#### Comment 5706173441 — Jordan-Hall — 2026-09-16T23:40:31Z

Source: https://github.com/Jordan-Hall/browser/issues/113#issuecomment-5706173441 | Updated: 2026-09-16T23:40:31Z

<!-- intent-core-review:2026-09-17:issue-113 -->
###### Review — PROGRAMME.T03: contract and process-authority baseline

Parent: #1; coordinates #2, #6 and #211. Treat 'freeze' as a reviewed, versioned baseline with compatibility rules, not a prohibition on correcting an unsafe contract.

**Do not ratify the current drafts unchanged.** The CORE review found independent argument hashes accepted in authoritative records (#787), negotiated versions disconnected from the codec (#790), launch authentication relying on inappropriate inherited-socketpair peer identity (#789), divergent operation/attempt semantics (#794), and no enforced profile-wide state owner (#793). These affect the authority inventory, not just implementation details.

**Required baseline:** for each durable record/message, record its owner, trust level, valid state transitions, identity/epoch, supported schema versions, byte/count limits, provenance and deletion policy. For each process, record its actual filesystem/network/desktop capabilities, allowed peers and methods, credentials/destinations, restart authority and revocation mechanism. 'Rust', 'local', a typed ID, a valid hash, or a supported protocol does not itself establish permission.

Make architectural tests exercise the actual dependency checker, codec and process handshake. Require explicit mappings between domain Operation/Receipt and durable dispatch/reconciliation states. Keep provider sessions secondary and describe the boundary where application-level role checks stop and OS containment begins.

**Acceptance:** ownership/compatibility ADRs link to test IDs; wrong role/account/epoch and mismatched action bytes are rejected end to end; no two services are authoritative for the same state family; old/unknown versions have intentional dispositions; every remaining discrepancy has an owner and blocks the affected claim. Store the reviewed commit/schema baseline and require a migration/security review when it changes.

This comment records review requirements only. It does not freeze an unsafe interface, grant new permissions, or mark CORE complete.

#### Comment 5706673471 — Jordan-Hall — 2026-09-17T00:43:11Z

Source: https://github.com/Jordan-Hall/browser/issues/113#issuecomment-5706673471 | Updated: 2026-09-17T00:43:11Z

###### Review follow-up — PROGRAMME.T03

Parent: #1. The current ownership-baseline review remains valid. Add **return/outcome semantics** to the contract freeze, not only record fields and process permissions.

This pass found both directions of ambiguity: #133/#797 can return successful reference registration with no reference inserted; #130/#794 can return an error after a mutation committed because readback happens outside the transaction. Standardize the distinction among rejected/no-effect, locally committed, already recorded, externally accepted, verified and still-unknown outcomes.

For every mutating repository/broker method, name the commit/authority boundary, idempotency identity, failure-after-commit behavior and recovery lookup. For every imported DTO, distinguish validated data from a runtime-issued execution grant. Hash validity, schema compatibility and a typed ID remain insufficient authorization.

Require the repaired cross-component fixtures from #123/#124/#125/#130/#131/#133 to accompany the versioned baseline. Preserve the intended CORE → SEC implementation order, but do not enable real external authority before the required broker/containment controls are implemented. This comment proposes baseline corrections; it neither freezes the current unsafe gaps nor marks CORE complete.


---

<a id="issue-114"></a>
## #114 — [TASK][PROGRAMME.T04] Establish the executable feasibility programme

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/114
**Created:** 2026-09-15T15:10:45Z | **Updated:** 2026-09-15T15:10:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Parent: #1

Task ID: `PROGRAMME.T04`

#### Implementation
Run shell/CEF integration, local speech plus inference, all four provider protocols, semantic PC control and resource-profile spikes on declared hardware. Capture alternatives and reject unworkable combinations with evidence.

#### Deliverable
P0 feasibility dossier and architecture decisions.

#### Verification
Critical embedding, accessibility, cancellation and confinement assumptions are demonstrated by executable spikes before commitments are frozen.

#### Guardrail
A failed candidate toolkit or provider surface changes the implementation choice, not the full product vision.

#### Prerequisites
- `PROGRAMME.T02`
- `PROGRAMME.T03`

Review: pending · Implementation: not started.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-115"></a>
## #115 — [TASK][PROGRAMME.T05] Build the cross-workstream dependency and ownership graph

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/115
**Created:** 2026-09-15T15:10:52Z | **Updated:** 2026-09-15T15:10:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Parent: #1

Task ID: `PROGRAMME.T05`

#### Implementation
Classify each dependency as contract prerequisite, implementation prerequisite, integration gate or external access condition. Assign task owners and identify parallel start opportunities without treating maturity phases as blanket blockers.

#### Deliverable
Reviewable dependency graph and staffing allocation.

#### Verification
The graph has no unexplained cycle and partnership/hardware research can begin before full SDK/runtime maturity.

#### Guardrail
Do not solve scheduling conflicts by weakening authorization or pretending fixture-only capabilities are live.

#### Prerequisites
- `PROGRAMME.T03`
- `PROGRAMME.T04`

Review: pending · Implementation: not started.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-116"></a>
## #116 — [TASK][PROGRAMME.T06] Establish continuous evaluation and release evidence

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/116
**Created:** 2026-09-15T15:11:04Z | **Updated:** 2026-09-15T15:11:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Parent: #1

Task ID: `PROGRAMME.T06`

#### Proposed implementation
Build EVAL-01 fixtures, blocking security/recovery checks, model/provider version pinning and the anti-wrapper continuity suite. Define evidence artifacts linked to issue/task IDs.

#### Done when
CI/evaluation architecture and release evidence schema are reviewable, and verified outcomes are determined independently of agent self-report.

#### Guardrail
A high aggregate task score cannot override a failed permission, duplication or privacy invariant.

#### Prerequisites
`PROGRAMME.T02`, `PROGRAMME.T03`

Review: pending · Implementation: not started.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-117"></a>
## #117 — [TASK][PROGRAMME.T07] Integrate the complete desktop platform through P1-P5

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/117
**Created:** 2026-09-15T15:11:13Z | **Updated:** 2026-09-15T15:11:13Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Parent: #1

Task ID: `PROGRAMME.T07`

#### Proposed implementation
Deliver persistent runtime, connector UI, agents/PC/files, supported writes/automation and inspectable learning/custom extensions as successive integrated releases. Retain every feature family and expose supported capability coverage honestly.

#### Done when
P1-P5 integration demonstrations show the user can speak an intent, keep the resulting application, inspect sources, change providers and perform only bounded verified actions.

#### Guardrail
Missing production access is a named dependency, not an excuse to claim unsupported Buy or Send actions.

#### Prerequisites
`PROGRAMME.T04`, `PROGRAMME.T05`, `PROGRAMME.T06`

Review: pending · Implementation: not started.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-118"></a>
## #118 — [TASK][PROGRAMME.T08] Integrate device mesh and daily-driver qualification

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/118
**Created:** 2026-09-15T15:11:29Z | **Updated:** 2026-09-15T15:11:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Parent: #1

Task ID: `PROGRAMME.T08`

#### Proposed implementation
Add encrypted portability, remote/home execution, collaboration, mobile and SDK distribution, then qualify browser compatibility, updates, recovery, accessibility and support across the declared matrix.

#### Done when
P6/P7 release evidence shows cross-device operation preserves source rights and single-authority transactions while update failure remains recoverable without AI.

#### Guardrail
Do not transfer credentials/blanket approvals as ordinary synced workspace data or market untested device parity.

#### Prerequisites
`PROGRAMME.T05`, `PROGRAMME.T06`, `PROGRAMME.T07`

Review: pending · Implementation: not started.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-119"></a>
## #119 — [TASK][PROGRAMME.T09] Run the OS distribution and Rust-kernel programme

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/119
**Created:** 2026-09-15T15:11:37Z | **Updated:** 2026-09-15T15:11:37Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Parent: #1

Task ID: `PROGRAMME.T09`

#### Proposed implementation
Package the user-space runtime as a session/distribution and staff K0-K4 separately with explicit hardware, ABI, driver, browser and inference compatibility gates. Keep reasoning and generated interfaces unprivileged.

#### Done when
P8 and K0-K4 stage plans are owned, and OS progress is measured by real supported workflows and deterministic recovery rather than a boot demo or Rust line count.

#### Guardrail
Kernel ambitions must not consume browser security-patch and connector-maintenance capacity required for current users.

#### Prerequisites
`PROGRAMME.T03`, `PROGRAMME.T04`, `PROGRAMME.T05`, `PROGRAMME.T06`

Review: pending · Implementation: not started.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-120"></a>
## #120 — [TASK][PROGRAMME.T10] Review this task register and stage later GitHub updates

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/120
**Created:** 2026-09-15T15:11:45Z | **Updated:** 2026-09-15T15:11:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Parent: #1

Task ID: `PROGRAMME.T10`

#### Proposed implementation
Review stable task IDs, assumptions, dependencies and acceptance evidence; record approve/revise/split/defer decisions. Later re-read live issues and stage only approved implementation work.

#### Done when
The reviewed change set is idempotent, preserves existing edits and has explicit mutation scope.

#### Guardrail
This issue only defines review/planning. It does not authorize implementation, PRs or code changes.

#### Prerequisites
`PROGRAMME.T01`, `PROGRAMME.T05`, `PROGRAMME.T06`

Review: pending · Implementation: not started.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-798"></a>
## #798 — Accidental connector test — ignore

**GitHub state:** closed | **State reason:** not_planned
**Source:** https://github.com/Jordan-Hall/browser/issues/798
**Created:** 2026-09-16T10:37:51Z | **Updated:** 2026-09-16T10:38:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

Created accidentally while switching GitHub write paths during CORE implementation. This is not a product or implementation task and is intentionally not part of the CORE hierarchy.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-799"></a>
## #799 — Accidental connector test — ignore

**GitHub state:** closed | **State reason:** not_planned
**Source:** https://github.com/Jordan-Hall/browser/issues/799
**Created:** 2026-09-16T10:38:07Z | **Updated:** 2026-09-16T10:38:18Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

Created accidentally by a mis-selected GitHub action during CORE implementation. This is not a product or implementation task and is intentionally excluded from the CORE hierarchy.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-800"></a>
## #800 — Accidental connector test — ignore

**GitHub state:** closed | **State reason:** not_planned
**Source:** https://github.com/Jordan-Hall/browser/issues/800
**Created:** 2026-09-16T10:38:29Z | **Updated:** 2026-09-16T10:38:42Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

Created accidentally by a mis-selected GitHub action during CORE implementation. This is not a product or implementation task and is intentionally excluded from the CORE hierarchy.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

