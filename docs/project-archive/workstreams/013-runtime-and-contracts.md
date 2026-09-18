# Runtime and contracts

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 41

## Contents

- [#13 — EPIC: Runtime and contracts](#issue-13)
- [#2 — [P0][CORE-01] Versioned domain and IPC contracts](#issue-2)
- [#3 — [P0][CORE-02] Durable state, outbox and artifacts](#issue-3)
- [#4 — [P0][CORE-03] Worker supervision and resource scheduler](#issue-4)
- [#5 — [P1][CORE-04] Recovery and bounded replay](#issue-5)
- [#121 — [TASK][CORE-01.T01] Bootstrap the Rust workspace and architectural checks](#issue-121)
- [#122 — [TASK][CORE-01.T02] Define typed identities and value objects](#issue-122)
- [#123 — [TASK][CORE-01.T03] Specify durable core record schemas](#issue-123)
- [#124 — [TASK][CORE-01.T04] Build bounded wire framing and errors](#issue-124)
- [#125 — [TASK][CORE-01.T05] Authenticate locally launched worker channels](#issue-125)
- [#126 — [TASK][CORE-01.T06] Implement version negotiation and schema evolution](#issue-126)
- [#127 — [TASK][CORE-01.T07] Wire cancellation, deadlines and backpressure](#issue-127)
- [#128 — [TASK][CORE-01.T08] Publish contract conformance and fuzz targets](#issue-128)
- [#129 — [TASK][CORE-02.T01] Establish database ownership and migrations](#issue-129)
- [#130 — [TASK][CORE-02.T02] Model durable operation and journal records](#issue-130)
- [#131 — [TASK][CORE-02.T03] Implement transactional outbox dispatch](#issue-131)
- [#132 — [TASK][CORE-02.T04] Add inbox deduplication and consumer cursors](#issue-132)
- [#133 — [TASK][CORE-02.T05] Implement scoped immutable artifact storage](#issue-133)
- [#134 — [TASK][CORE-02.T06] Implement reference-safe retention and deletion](#issue-134)
- [#135 — [TASK][CORE-02.T07] Build backup, restore and integrity checks](#issue-135)
- [#136 — [TASK][CORE-02.T08] Run storage fault and power-loss qualification](#issue-136)
- [#137 — [TASK][CORE-03.T01] Define worker registry and launch specifications](#issue-137)
- [#138 — [TASK][CORE-03.T02] Implement process lifecycle and health](#issue-138)
- [#139 — [TASK][CORE-03.T03] Implement admission and foreground priorities](#issue-139)
- [#140 — [TASK][CORE-03.T04] Enforce platform resource constraints](#issue-140)
- [#141 — [TASK][CORE-03.T05] Implement lease-first cancellation and revocation](#issue-141)
- [#142 — [TASK][CORE-03.T06] Add crash-loop policy and degraded service](#issue-142)
- [#143 — [TASK][CORE-03.T07] Integrate resource observation and cooperative yields](#issue-143)
- [#144 — [TASK][CORE-03.T08] Verify supervisor robustness under concurrency](#issue-144)
- [#145 — [TASK][CORE-04.T01] Classify every recoverable operation](#issue-145)
- [#146 — [TASK][CORE-04.T02] Implement consistent checkpoints](#issue-146)
- [#147 — [TASK][CORE-04.T03] Plan startup recovery before dispatch](#issue-147)
- [#148 — [TASK][CORE-04.T04] Reconcile local writes and uncertain external state](#issue-148)
- [#149 — [TASK][CORE-04.T05] Build no-production replay contexts](#issue-149)
- [#150 — [TASK][CORE-04.T06] Implement provider resume and reseeding policy](#issue-150)
- [#151 — [TASK][CORE-04.T07] Expose recovery decisions in the task centre](#issue-151)
- [#152 — [TASK][CORE-04.T08] Qualify restart, suspend and corrupt-state cases](#issue-152)
- [#211 — [TASK][EPIC-CORE.T01] Agree runtime ownership and wire contracts](#issue-211)
- [#212 — [TASK][EPIC-CORE.T02] Integrate durable dispatch and worker supervision](#issue-212)
- [#213 — [TASK][EPIC-CORE.T03] Exercise crash and bounded replay end to end](#issue-213)
- [#214 — [TASK][EPIC-CORE.T04] Publish runtime operational readiness](#issue-214)

---

<a id="issue-13"></a>
## #13 — EPIC: Runtime and contracts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/13
**Created:** 2026-09-15T12:06:38Z | **Updated:** 2026-09-17T21:17:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #1 | **Body-declared parent:** #1
**Native child issues:** #2, #3, #4, #5
**PRs mentioning this issue:** [#785](https://github.com/Jordan-Hall/browser/pull/785), [#786](https://github.com/Jordan-Hall/browser/pull/786), [#787](https://github.com/Jordan-Hall/browser/pull/787), [#788](https://github.com/Jordan-Hall/browser/pull/788), [#789](https://github.com/Jordan-Hall/browser/pull/789), [#790](https://github.com/Jordan-Hall/browser/pull/790), [#791](https://github.com/Jordan-Hall/browser/pull/791), [#792](https://github.com/Jordan-Hall/browser/pull/792), [#793](https://github.com/Jordan-Hall/browser/pull/793), [#794](https://github.com/Jordan-Hall/browser/pull/794), [#795](https://github.com/Jordan-Hall/browser/pull/795), [#796](https://github.com/Jordan-Hall/browser/pull/796), [#797](https://github.com/Jordan-Hall/browser/pull/797), [#801](https://github.com/Jordan-Hall/browser/pull/801), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#803](https://github.com/Jordan-Hall/browser/pull/803), [#804](https://github.com/Jordan-Hall/browser/pull/804), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#808](https://github.com/Jordan-Hall/browser/pull/808), [#809](https://github.com/Jordan-Hall/browser/pull/809), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Programme: #1

Own the durable Rust runtime contracts, transactional state, worker supervision, scheduling and crash-safe recovery. This epic is complete only when provider/model sessions are replaceable details rather than the source of truth.

#### Child issues
- [ ] #2 CORE-01 — Versioned domain and IPC contracts
- [ ] #3 CORE-02 — Durable state, outbox and artifacts
- [ ] #4 CORE-03 — Worker supervision and resource scheduler
- [ ] #5 CORE-04 — Recovery and bounded replay

#### Cross-cutting gates
Schema/version compatibility, bounded IPC, deterministic cancellation, crash recovery and no blind replay of irreversible effects.

### Discussion (8 comments)

#### Comment 5681821882 — Jordan-Hall — 2026-09-15T14:20:41Z

Source: https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5681821882 | Updated: 2026-09-15T14:20:41Z

<!-- intent-implementation-v1:EPIC-CORE -->
###### Workstream implementation and integration tasks

Implement features in #2–#5; this epic owns their integration, not a duplicate runtime. Stable task IDs match handbook edition 1.0.

- [ ] **EPIC-CORE.T01 — Agree runtime ownership and wire contracts.** Ratify records, authenticated worker roles, state transitions, artifact references and one writer per authoritative state family. **Proof:** schema/ownership inventory and a real two-worker handshake.
- [ ] **EPIC-CORE.T02 — Integrate durable dispatch and worker supervision.** Connect journal/outbox, worker epochs, budgets and lease-first cancellation. **Proof:** a task crosses worker boundaries without losing durable state or gaining authority.
- [ ] **EPIC-CORE.T03 — Exercise crash and bounded replay end to end.** Kill workers around dispatch, restore checkpoints and replay only fixture/captured observations. **Proof:** safe reads resume; uncertain effects remain in reconciliation; no live write replay.
- [ ] **EPIC-CORE.T04 — Publish operational readiness.** Document platform resource enforcement, degraded modes, migration/backup recovery and measured control latency. **Proof:** pinned support matrix and fault/concurrency reports.

**Integration demonstration:** start a source-backed task, saturate progress/inference, revoke its lease, restart the supervisor and inspect the same workspace and operation history without provider conversation reconstruction.

**Closure gate:** children plus integration evidence, not merely four closed checkboxes. GPU limits and same-user IPC assumptions must be described at their actual enforcement strength; no exactly-once claim for arbitrary external actions.

#### Comment 5706192648 — Jordan-Hall — 2026-09-16T23:42:51Z

Source: https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5706192648 | Updated: 2026-09-16T23:42:51Z

<!-- intent-core-review:2026-09-17:epic-CORE -->
###### CORE epic review and implementation-readiness assessment

**Review outcome: CORE is not production-complete.** Fourteen task PRs exist, covering CORE-01.T01–T08 and CORE-02.T01–T06. The remaining eighteen feature tasks have no implementation PR in the reviewed inventory. Requirement/epic integration also remains outstanding. No code, branches, merges or issue closures were performed in this review.

###### Review navigation
- #2: CORE-01 task-by-task findings; #121–#128 map to PRs #785–#792.
- #3: CORE-02 storage findings; #129–#133 map to PRs #793–#797; #134 maps to #801; #135/#136 are unimplemented backup/fault-qualification work.
- #4: CORE-03 planning/acceptance refinements; #137–#144 require real supervisor/process implementation and evidence.
- #5: CORE-04 planning/acceptance refinements; #145–#152 require classification, checkpoints, startup gating, reconciliation, replay, provider resume, recovery presentation and restart qualification.
- #211: ownership/wire baseline; #212: dispatch/supervision integration; #213: crash/replay end to end; #214: operational readiness manifest. These are integration tasks, not duplicate feature implementations.
- #113 coordinates the programme's authority/contract baseline.

###### Highest-priority blockers
1. **Action identity/authority binding:** #787 can accept inconsistent argument hashes; #795 does not prove staged transport bytes/destination are the approved action. #794 can replace an attempt identity while recording its result and loses compensation lineage.
2. **Worker and protocol integration:** #789's inherited-socketpair identity needs actual spawned-process verification; #790 negotiates versions that the V1-only codec does not implement; #791's queue/cancellation primitives are not yet a qualified two-process control path.
3. **Crash-safe storage ownership:** #793 may modify an unrelated populated zero-ID database and does not enforce a profile-wide sole owner; migration selection/version consistency also needs correction. #795 leaves abandoned attempting messages without a recovery path.
4. **Retention correctness:** #801's suppression UPDATE goes through an INSTEAD OF trigger then treats the outer affected-row count as one. The reviewed CI head `fbfcfceb17540e5fe80bec555385e097f3787455` fails at tests (run `35086737317`, job `104763204222`). GC eligibility must be protected before unlink, and bounded scans must not starve later eligible blobs.
5. **Evidence quality:** #792's fuzz target lacks a direct dependency and the conformance report is smoke coverage. The workflow lacks explicit doctest/fuzz/platform qualification and does not retain conformance evidence on earlier failure. Green checks do not resolve the above findings.

The prior aggregate-effect read fix on #796 and the three artifact fixes on #797 are acknowledged; they are not being relabelled as unfixed. Their remaining integration, budget, filesystem and lifecycle risks are documented separately.

###### Recommended correction order within CORE
Repair the owning CORE-01 contracts/authentication/codec/queue gates first, then the CORE-02 identity/outbox/ownership/retention primitives. Carry those fixes into the stack and rerun integrated checks. Complete backup/fault qualification, then supervisor and recovery tasks in their existing stable-ID order. Each task needs its own implementation/test evidence, followed by the real integration scenarios in #211–#214.

###### Epic closure gate
Require a current reviewed/integrated commit matrix; all task acceptance criteria; every actionable PR finding dispositioned with a fix/test or a precise technical reason; successful permitted-work scenarios as well as rejection tests; real worker cancellation under load; interruption after fixture acceptance without duplicate effects; consistent backup/checkpoint restore; and the declared platform/durability matrix. A local stop, an external cancellation, and a verified rollback are different outcomes.

This review makes the missing work and defects explicit. It does not mark findings fixed, resolve threads, or claim that the browser/runtime has been implemented in full.

#### Comment 5706732410 — Jordan-Hall — 2026-09-17T00:50:34Z

Source: https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5706732410 | Updated: 2026-09-17T00:50:34Z

###### CORE epic follow-up — current-head review, new defects and integration gates

This follow-up rechecked all 14 existing CORE task PR heads, the four requirements, all 32 feature tasks, the four epic tasks and programme baseline #113. Existing valid fixes on #796/#797 were preserved; new findings were recorded separately. No code, branch, issue/PR state or review resolution was changed.

###### Navigate the follow-up
- [CORE-01 contract/codec/channel review](https://github.com/Jordan-Hall/browser/issues/2#issuecomment-5706716021): task comments on #121–#128 and PRs #785–#792.
- [CORE-02 storage/outcome review](https://github.com/Jordan-Hall/browser/issues/3#issuecomment-5706719472): #129–#136, PRs #793–#797 and #801.
- [CORE-03 supervisor acceptance refinements](https://github.com/Jordan-Hall/browser/issues/4#issuecomment-5706721638): #137–#144.
- [CORE-04 recovery acceptance refinements](https://github.com/Jordan-Hall/browser/issues/5#issuecomment-5706724466): #145–#152.
- #211 now requires executable producer/consumer contract fixtures; #212 requires the complete dispatch/result/receipt path; #213 includes lost acknowledgements and independent rollback-domain tests; #214 ties readiness to base/head/tested integration SHAs. #113 includes mutation return/outcome semantics in the baseline.

###### New findings requiring owners to act
The most urgent new defect is false-success artifact reference registration (#797/#133): broad `INSERT OR IGNORE` masks empty-field CHECK failures, so a claimed retention pin may not exist. This was reproduced in reduced SQLite tests and must become a repository regression before GC/backup/checkpoint integration.

Additional source findings cover missing store identity being regenerated on open (#793), errors after committed operation mutations (#794), encode/decode limit asymmetry (#788), no-op migration trust semantics (#790), and unbounded hold expiry/UTF-8 error truncation (#801). Detailed implementation options and tests are on the owning PRs and tasks.

###### Readiness remains blocked
The refreshed inventory still contains only CORE-01.T01–T08 and CORE-02.T01–T06 PRs. Eighteen feature tasks and the integration work lack implementation PRs. #801's inspected current-head CI fails three retention tests. Prior action-binding, real worker authentication, outbox recovery/fencing and storage ownership findings also remain open.

Correct existing primitives in stable-ID order, propagate fixes through the stack, then implement the missing tasks. Require successful authorized work as well as safe rejection and explicit uncertainty. Do not count a mock, an empty test set, a PR title, a comment or a previously green ancestor as completed production functionality. This pass completes review comments, not CORE implementation.

#### Comment 5711147203 — Jordan-Hall — 2026-09-17T08:10:38Z

Source: https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5711147203 | Updated: 2026-09-17T08:10:38Z

<!-- intent-core-review:2026-09-17:closure-register-link -->
###### Review continuation — complete task/PR navigation

The task-by-task remediation and verification register is now on the programme issue:

https://github.com/Jordan-Hall/browser/issues/1#issuecomment-5711140796

It maps all 32 CORE feature tasks, the four epic integration tasks #211–#214 and programme contract task #113 to their remaining acceptance evidence. It also links **14 newly submitted commit-anchored COMMENT reviews** on #785–#797 and #801; each links the earlier detailed findings without duplicating the complete review text.

**Disposition remains: CORE is not production-complete.** Eighteen feature tasks have no matching implementation PR in the retrieved inventory, and #801's retrieved head still has failing CI. The original corrected read-path/artifact findings on #796/#797 remain acknowledged as corrected; separate unresolved integration risks are not silently considered fixed.

For epic sign-off, record current reviewed heads, propagated prerequisite fixes, regression results, explicit dispositions for actionable comments, and the end-to-end evidence in #211–#214. Review publication is not implementation completion or merge approval.

This continuation changed comments/review records only: no code, branches, commits, merges, issue closures or thread resolutions.

#### Comment 5714280316 — Jordan-Hall — 2026-09-17T12:23:08Z

Source: https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5714280316 | Updated: 2026-09-17T12:23:08Z

<!-- intent-core-review:2026-09-17:coverage-verification -->
###### CORE review coverage verified — implementation sign-off remains blocked

I rechecked the repository-wide CORE PR inventory, all fourteen returned PR discussion timelines, the requirement/proposal reviews and task coverage. The detailed reviews already exist; this comment consolidates their navigation and acceptance gates rather than posting duplicate findings to every thread.

###### Coverage
Review comments are present on **43 issue resources**: programme #1; requirements #2–#5; this epic; baseline #113; all 32 feature tasks #121–#152; and integration tasks #211–#214. Every existing CORE task PR also has a detailed review: #785–#797 and #801. CORE-02.T07/T08 and CORE-03/CORE-04 have no implementation PR in the returned inventory; their reviews are implementation/acceptance recommendations, not claims of reviewed working code.

###### Reviewed implementation heads and correction gates
| Task / issue | PR / head | Required correction or evidence before sign-off |
| --- | --- | --- |
| CORE-01.T01 / #121 | #785 `fc7b62456f0d` | Fail-closed dependency policy; committed lockfile and locked CI; explicit doctests; declared platform coverage. |
| CORE-01.T02 / #122 | #786 `a9fbf6648214` | Lossless wide-integer money through the actual envelope; nonempty provider IDs; distinguish general signed money from executable spending values. |
| CORE-01.T03 / #123 | #787 `7c9d1e441b55` | Canonical argument artifact/digest consistency; explicit unknown-field handling; checked operation/receipt projections. |
| CORE-01.T04 / #124 | #788 `f76de0034641` | Bound encoding during emission; validate schema before incompatible payload decoding; make the advertised numeric error representation explicit. |
| CORE-01.T05 / #125 | #789 `45c4fef9dbb6` | Real launched-worker/channel authentication; globally one-use launch records; production evidence separated from synthetic fixtures. |
| CORE-01.T06 / #126 | #790 `9134df9db63b` | Authenticated negotiated session must use an implemented codec; validate actual migration output, including no-op paths; bound input work. |
| CORE-01.T07 / #127 | #791 `d1371e206495` | Unbypassable queue limits, cancellation retirement, accurate backlog metrics, byte budgets and actual cross-process cancellation tests. |
| CORE-01.T08 / #128 | #792 `f04d5d5b7998` | Build the excluded fuzz targets with declared dependencies; qualify real invariants beyond smoke checks; retain failure evidence. |
| CORE-02.T01 / #129 | #793 `c468a457da54` | Reject unrelated populated databases before mutation; coordinate migration selection and profile ownership; validate ledger/user_version agreement. |
| CORE-02.T02 / #130 | #794 `a01286c2acf4` | Enforce attempt continuity, compensation lineage and real source-precondition identity; keep journal and projection semantics aligned. |
| CORE-02.T03 / #131 | #795 `3c7c848014e9` | Gate dispatch bytes behind durable attempt start; fence leases; recover abandoned attempts; retire cancelled work; bind transport material to approved intent. |
| CORE-02.T04 / #132 | #796 `1fc177f601c6` | Preserve the existing aggregate-read fix; verify duplicate materialization, bound aggregate effect bytes and separate delivery order from source revision. |
| CORE-02.T05 / #133 | #797 `07936f3eecbb` | Preserve the three existing artifact fixes; prove reference insertion, root/permission/path boundaries, orphan cleanup and platform durability. |
| CORE-02.T06 / #134 | #801 `fbfcfceb1754` | Fix view-trigger suppression rollback; protect eligibility before unlink; prevent scan starvation; bound maintenance and safely format failure diagnostics. |

All fourteen PRs were open and unmerged in this inventory. Their existence does **not** establish fourteen completed production tasks.

###### Current integration result
Rechecked [CI run 35086737317, Rust checks job 104763204222](https://github.com/Jordan-Hall/browser/actions/runs/35086737317/job/104763204222): it targets #801 head `fbfcfceb17540e5fe80bec555385e097f3787455`, passes formatting and Clippy, fails the test step, and skips architecture/conformance/upload steps. The source still updates `artifact_handles` through the view and treats its affected-row count as exactly one. Do not describe this head as green or reviewed production completion.

###### Missing implementation and integration
- #135/#136: authenticated consistent database/blob backup and restore; storage interruption/fault qualification.
- #137–#144: actual worker registry/lifecycle, admission, platform constraints, lease-first cancellation, crash-loop handling, telemetry and concurrency qualification.
- #145–#152: recovery classification/checkpoints, startup dispatch barrier, evidence-bound reconciliation, production-free replay, provider resume/reseed, recovery presentation and restart qualification.
- #211–#214: shared ownership baseline, dispatch/supervisor integration, end-to-end crash/replay and operational readiness. #113 coordinates the programme authority baseline.

###### How each finding should be closed during implementation
For each actionable finding, record **the review comment/thread, owning task, fix commit, regression test and CI result on that commit**. A deliberate non-change needs a specific technical rationale and an explicit statement of any residual limitation. After a lower stacked PR changes, verify the fix is included in every affected descendant and rerun integrated checks. A response saying Fixed without that evidence is not a completed review disposition.

Keep two records distinct: (1) review coverage, which is present across the CORE backlog; (2) implementation and qualification, which remain incomplete. Existing Fixed replies on #796/#797 apply to their named earlier defects, not to every subsequent finding.

This verification pass makes no source changes, merges, closures or Fixed claims. Rust/fuzz binaries were not executed locally; current CI results, source inspection and proposed acceptance tests are distinguished above.

#### Comment 5717448842 — Jordan-Hall — 2026-09-17T16:03:22Z

Source: https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5717448842 | Updated: 2026-09-17T16:03:22Z

<!-- intent-core-inline-review:2026-09-17 -->
###### CORE implementation audit — inline amendments posted; epic NOT complete

Traced the original programme #1, this epic, CORE parents #2–#5, their task inventory and the 14 open implementation PRs. This pass advances the existing narrative reviews with **14 submitted PR reviews, 28 line-anchored comments and 21 GitHub suggestion blocks**. These are proposed changes, not applied commits or approvals. No issue was closed and no PR was merged.

###### Submitted reviews and concrete changes

| Task | PR / submitted review | Amendment supplied in this pass |
|---|---|---|
| #121 CORE-01.T01 | [#785 review](https://github.com/Jordan-Hall/browser/pull/785#pullrequestreview-5238159625) | Fail-closed direct dependency allowlist; explicit doctest CI invocation |
| #122 CORE-01.T02 | [#786 review](https://github.com/Jordan-Hall/browser/pull/786#pullrequestreview-5238170296) | Canonical decimal-string i128 money codec with boundary/invalid-input tests; requires explicit wire-format decision |
| #123 CORE-01.T03 | [#787 review](https://github.com/Jordan-Hall/browser/pull/787#pullrequestreview-5238187079) | Derive proposal hash at construction; reject inconsistent wire hash and unsupported fields; regression tests |
| #124 CORE-01.T04 | [#788 review](https://github.com/Jordan-Hall/browser/pull/788#pullrequestreview-5238203975) | Bound encoding while writing; reject schema mismatch before interpreting the typed payload |
| #125 CORE-01.T05 | [#789 review](https://github.com/Jordan-Hall/browser/pull/789#pullrequestreview-5238223303) | Remove implicit launch-record cloning; specify real-process credential/one-use-registry integration gate |
| #126 CORE-01.T06 | [#790 review](https://github.com/Jordan-Hall/browser/pull/790#pullrequestreview-5238234741) | Bound capability-iterator consumption; require selected versions to correspond to implemented codecs |
| #127 CORE-01.T07 | [#791 review](https://github.com/Jordan-Hall/browser/pull/791#pullrequestreview-5238245818) | Make QueueLimits fields private; require fenced request retirement rather than unsafe record deletion |
| #128 CORE-01.T08 | [#792 review](https://github.com/Jordan-Hall/browser/pull/792#pullrequestreview-5238256172) | Declare fuzz serde_json dependency; compile-check the excluded fuzz package in CI |
| #129 CORE-02.T01 | [#793 review](https://github.com/Jordan-Hall/browser/pull/793#pullrequestreview-5238283070) | Reject populated zero-application-ID databases; select/apply migrations under a writer transaction |
| #130 CORE-02.T02 | [#794 review](https://github.com/Jordan-Hall/browser/pull/794#pullrequestreview-5238326845) | Preserve attempt identity on outcome transitions; keep compensation-origin reconciliation as a separate gate |
| #131 CORE-02.T03 | [#795 review](https://github.com/Jordan-Hall/browser/pull/795#pullrequestreview-5238334549) | Filter non-dispatchable operations before claim LIMIT; keep abandoned-attempt recovery and opaque/fenced dispatch material blocked |
| #132 CORE-02.T04 | [#796 review](https://github.com/Jordan-Hall/browser/pull/796#pullrequestreview-5238343182) | Aggregate inbox/effect payload admission budget; acknowledge the existing aggregate read-integrity fix |
| #133 CORE-02.T05 | [#797 review](https://github.com/Jordan-Hall/browser/pull/797#pullrequestreview-5238353038) | Repeat target-directory durability barrier on deduplication/retry; review existing-directory retry barriers |
| #134 CORE-02.T06 | [#801 review](https://github.com/Jordan-Hall/browser/pull/801#pullrequestreview-5238367830) | Update underlying suppression table; filter GC eligibility/already-queued keys before LIMIT; UTF-8-safe diagnostics; retry unlink durability barrier |

###### CI evidence, not a green-build assumption

Inspected the actual [run 35086737317 / job 104763204222 log](https://github.com/Jordan-Hall/browser/actions/runs/35086737317/job/104763204222). It tests the merge of `fbfcfceb17540e5fe80bec555385e097f3787455` into `07936f3eecbba10d608b520427ab13360cf79eee`. Formatting and Clippy pass. The intent-state suite reports **21 passed, 3 failed**: `active_hold_blocks_gc_until_released`, `suppression_is_immediate_for_normal_retrieval`, and `durable_reference_blocks_gc_until_removed`, all with `ConcurrentSuppression`. The later architecture/conformance/report-upload steps are skipped. The suppression suggestion addresses SQLite's INSTEAD OF-view affected-row semantics; passing those three tests afterward would not establish the missing integration properties.

Rust, cargo and the fuzz binaries were not available for local execution in this review environment. None of the suggested Rust changes has been compiled here, and no passing rerun is claimed. Apply compatible suggestions to their owning PRs, update dependent API callers/fixtures, rebase the stack and run the full gates before resolving the threads. In particular, the money representation, descriptor API and tighter admission budgets are explicit contract decisions, not silent compatibility-preserving edits.

###### Remaining feature implementation inventory

The inventory contains 32 CORE feature tasks: 14 have open PRs, and **18 have no implementation PR in the reviewed repository inventory**. An open PR is not completed acceptance evidence.

- [ ] #135 — consistent backup/restore, authenticated inventory and fresh restore with dispatch disabled.
- [ ] #136 — storage fault injection and actual process/VM power-loss qualification.
- [ ] #137 — worker registry and verified launch specifications.
- [ ] #138 — worker lifecycle/readiness/health handling.
- [ ] #139 — bounded resource admission and priority/control reservations.
- [ ] #140 — qualified platform constraints, distinguishing OS-enforced limits from cooperative GPU accounting.
- [ ] #141 — lease-first cancellation/revocation and stale-generation rejection.
- [ ] #142 — bounded crash-loop/restart and degraded operation.
- [ ] #143 — resource metrics, accounting and cooperative yield behavior.
- [ ] #144 — concurrent real-process scheduler qualification.
- [ ] #145 — durable recovery classification.
- [ ] #146 — checkpoints and immutable recovery inputs.
- [ ] #147 — startup recovery barrier before dispatch resumes.
- [ ] #148 — read-only external reconciliation preserving uncertainty and attempt lineage.
- [ ] #149 — bounded replay without production-effect credentials or production dispatch.
- [ ] #150 — provider resume/reseed handling.
- [ ] #151 — recovery state and actions exposed accurately to users.
- [ ] #152 — restart/recovery qualification, including independently observed external effects.

Also retain baseline #113 and this epic's integration tasks #211, #212, #213 and #214 as open evidence gates; the 18 count above does not include those five tasks.

###### Required integration work that cannot be replaced by local suggestions

**Authority and dispatch:** real spawned-worker authentication, a registry-owned one-use launch binding, implemented negotiated codecs, approval/account/capability/target/source-precondition binding, immutable argument bytes, opaque dispatch material and worker-generation fencing. A hash is not authorization and an in-memory two-endpoint test is not a two-worker integration test.

**State and artifact lifecycle:** a single trusted profile owner or equivalent shared exclusion protocol; user_version/ledger agreement; one operation/attempt/journal transition contract across the operation and outbox APIs; explicit compensation origin; abandoned-attempt discovery without blind resend; atomic publication/reference/hold/backup/GC lifecycle coordination. A post-unlink check cannot recover bytes already deleted by a concurrent collector.

**Qualification:** committed application/fuzz lockfiles, explicit doctests, real excluded-package/fuzzer build and bounded runs, complete failure-report artifacts, real-process cancellation/deadline/epoch tests, schema fixtures for every claimed family, scoped filesystem and platform durability tests, backup/restore evidence, and crash/replay tests with an independent external-effect ledger. The current smoke report is not operational approval.

###### Closure gate

Keep #2, #3, #4, #5 and this epic open until their task-specific implementation evidence and the integrated acceptance scenario exist: run a source-backed operation, saturate progress/resource work, revoke authority, crash/restart the supervisor, recover the same durable history without reconstructing it from the provider, and prove that replay/reconciliation does not silently repeat an uncertain external effect. Attach the tested commit, lockfile identity, platform/filesystem, executed gates and residual limitations. **Review submitted is not code applied; code applied is not task accepted; unit tests passing is not epic completion.**

#### Comment 5718036472 — Jordan-Hall — 2026-09-17T16:46:23Z

Source: https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5718036472 | Updated: 2026-09-17T16:46:23Z

###### Implementation follow-through: PR #802 is committed and CI passes

[PR #802](https://github.com/Jordan-Hall/browser/pull/802) now contains actual source changes and regression tests, not just review suggestions: **33 changed files, 34 new runtime regressions and 3 new compile-fail checks**, plus both committed lockfiles.

Verified [CI run 35248222708](https://github.com/Jordan-Hall/browser/actions/runs/35248222708) at head `1023b7fd6055ef95520478f93dbf45ffc99b13a7` (tested merge `7cfd4f7c9668a074a606a1d3ddd1a6548f78e484`). All gates pass: **115 unit/integration tests and 4 doctests**, formatting, Clippy with warnings denied, committed-lockfile checks, excluded fuzz-target compilation, actual architecture checks, smoke conformance and evidence upload. The three retention failures reported in the earlier audit now pass.

Implemented contract/wire bounds and hash consistency, atomic migration validation, attempt identity continuity across operations and outbox, cancelled-claim filtering, aggregate inbox limits and duplicate integrity checks. Artifact publication and GC now share a writer boundary through final filesystem operations and metadata commit, with two-connection publication/reference/hold-versus-unlink regressions. Retry directory barriers, suppression row counts, bounded GC selection and UTF-8-safe error recording are covered.

The PR remains **draft and not production-qualified**. It is stacked on #801; upstream branches were not rewritten or merged. The 18 missing feature tasks #135–#152, baseline #113 and integration/readiness gates #211–#214 remain open. Real worker authentication/supervision, generation fencing, uncertain external-effect recovery, backup/restore, instrumented fuzzing and platform/power-loss qualification are not supplied by these unit/integration tests. The money wire representation and proposal descriptor API changes require explicit compatibility review.

[Committed release-gate and compatibility document](https://github.com/Jordan-Hall/browser/blob/1023b7fd6055ef95520478f93dbf45ffc99b13a7/docs/core-production-readiness.md). [Exact verification artifact](https://github.com/Jordan-Hall/browser/actions/runs/35248222708/artifacts/10507954423). No issue was closed and no production-ready claim is made.

#### Comment 5721341683 — Jordan-Hall — 2026-09-17T21:17:00Z

Source: https://github.com/Jordan-Hall/browser/issues/13#issuecomment-5721341683 | Updated: 2026-09-17T21:17:00Z

###### All existing CORE PRs updated; every task individually referenced

Updated source branches **and descriptions** for #785, #786, #787, #788, #789, #790, #791, #792, #793, #794, #795, #796, #797, #801 and #802. The hardening fixes are now in the owning task PRs, with updated predecessors, stage-specific lockfiles/tests, exact head commits, passing CI links and explicit remaining acceptance. Every branch update preserves history without force-pushing.

[PR #802 now contains the complete individual-task matrix](https://github.com/Jordan-Hall/browser/pull/802): all **32 feature tasks**, baseline **#113**, and each of **#211, #212, #213 and #214**. It also links this epic, programme #1 and all four CORE parents #2, #3, #4 and #5. The same **37-task inventory** is committed as [Markdown](https://github.com/Jordan-Hall/browser/blob/84d2dd0512fb8e6ae02e4b41562676f4b5e8a215/docs/core-task-coverage.md) and [JSON](https://github.com/Jordan-Hall/browser/blob/84d2dd0512fb8e6ae02e4b41562676f4b5e8a215/docs/core-task-coverage.json), with a CI coverage check that rejects missing/duplicate task references and a production-ready flag while tasks remain unaccepted.

###### Verified implementation

All 15 PRs have passing CI runs linked in their descriptions and in #802. The final integration [run 35275548059](https://github.com/Jordan-Hall/browser/actions/runs/35275548059) passes after every original task branch was updated: **117 runtime tests and 4 compile-fail doctests**, formatting, warnings-denied Clippy, both committed lockfiles, actual architecture checks, excluded fuzz compilation, smoke conformance, task-coverage validation and evidence upload. Head `84d2dd0512fb8e6ae02e4b41562676f4b5e8a215`; tested merge `dba74c4e387995b360ed8876f80d32fc46365b29`; updated #801 base `96e66e0b977b3ad4c716368b0baa430a266cb686`. [Retained source/lockfile/verification artifact](https://github.com/Jordan-Hall/browser/actions/runs/35275548059/artifacts/10520023171).

Each original task-stage source tree was also compiled/tested locally using the verified offline Rust 1.98.1 toolchain. Two additional regressions now pass: #788 rejects duplicate JSON keys before typed parsing, and #797 rejects invalid artifact-reference pins instead of reporting false success.

###### Completion status — still open

**This is not completion of CORE or a production-ready release.** The 14 feature tasks with PRs still have task-specific acceptance/integration gates. The following 18 tasks still have no implementation in this stack: #135, #136, #137, #138, #139, #140, #141, #142, #143, #144, #145, #146, #147, #148, #149, #150, #151 and #152. The five baseline/integration tasks above also remain unaccepted.

In particular, real worker authentication/supervision and generation fencing, backup/restore, durable recovery/read-only reconciliation, credential-isolated replay, complete authority enforcement and platform/process/power-loss qualification are not supplied by passing unit tests. Fuzz-target compilation is not an instrumented campaign. The matrix records these gaps explicitly rather than closing issues based on references.

No PR was merged or approved; no issue was closed; review findings were not blanket-resolved. #802 remains draft. **All existing PRs updated: yes. Every individual CORE task referenced in #802: yes. All CORE tasks done: no.**


---

<a id="issue-2"></a>
## #2 — [P0][CORE-01] Versioned domain and IPC contracts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/2
**Created:** 2026-09-15T12:03:40Z | **Updated:** 2026-09-17T00:48:28Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #13 | **Body-declared parent:** #1
**PRs mentioning this issue:** [#785](https://github.com/Jordan-Hall/browser/pull/785), [#786](https://github.com/Jordan-Hall/browser/pull/786), [#787](https://github.com/Jordan-Hall/browser/pull/787), [#788](https://github.com/Jordan-Hall/browser/pull/788), [#789](https://github.com/Jordan-Hall/browser/pull/789), [#790](https://github.com/Jordan-Hall/browser/pull/790), [#791](https://github.com/Jordan-Hall/browser/pull/791), [#792](https://github.com/Jordan-Hall/browser/pull/792), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #1

#### Objective
Define the stable Rust contracts that every trusted process, worker, connector, model host, UI renderer and external-agent adapter uses. This is the foundation for replacing chat-centric state with durable product state.

#### Scope
- Define typed IDs and versioned schemas for `GoalContract`, `Workspace`, `Task`, `Capability`, `Observation`, `Evidence`, `ActionProposal`, `Approval`, `Operation`, `Receipt`, `ViewDefinition`, `MemoryRecord` and artifact references.
- Define explicit request/response/event envelopes with request IDs, deadlines, cancellation IDs, trace IDs and schema version.
- Authenticate local IPC peers and bind process identity/role to allowed message families.
- Enforce message size, collection size, recursion/depth and backpressure limits.
- Support version negotiation plus forward/backward-compatible migrations where explicitly declared.
- Keep raw browser/OS/provider handles out of general contracts; use broker-resolved typed IDs.

#### Architecture requirements
- Rust + Serde first; schemas must be serializable for golden fixtures and protocol recordings.
- OS-local IPC preferred over publicly reachable localhost services.
- Unknown fields/versions must have defined handling rather than accidental permissiveness.
- Contracts must not encode a model provider as the durable owner of task/workspace state.

#### Acceptance criteria
- [ ] All core records have schema versions and migration tests.
- [ ] Malformed, oversized and unsupported-version messages are rejected safely.
- [ ] Negotiated compatible versions round-trip without semantic loss.
- [ ] Cancellation and deadlines propagate across at least two worker boundaries.
- [ ] IPC peer authentication prevents an untrusted local process from impersonating a privileged broker.
- [ ] Golden contract fixtures run in CI.

#### Tests
Property tests for parsing/serialization; fuzz malformed envelopes; migration round-trips; replay recorded older-version fixtures; authentication-negative tests.

#### Dependencies
None. This issue is intentionally first.

**First phase:** P0  
**Maturity target:** P1  
**Workstream:** Runtime and contracts

### Discussion (3 comments)

#### Comment 5681731406 — Jordan-Hall — 2026-09-15T14:16:04Z

Source: https://github.com/Jordan-Hall/browser/issues/2#issuecomment-5681731406 | Updated: 2026-09-15T14:16:04Z

<!-- intent-implementation-v1:CORE-01 -->
###### Implementation proposal — CORE-01

Build the small Rust contract kernel before GUI or agent-specific code. Proposed locations: `crates/contracts`, `crates/ipc`, `schemas/v1`, `tests/contracts`, and the workspace/toolchain manifests. Domain records must not depend on Tokio, CEF, GUI handles or provider SDKs.

###### Reviewable tasks
- [ ] **CORE-01.T01 — Bootstrap the workspace and dependency checks.** Pin the compiler and lockfile; create real contracts/IPC/fixture-worker packages. Add a dependency-direction test rejecting GUI/OS/provider imports into contracts. **Verify:** clean build plus a deliberately forbidden dependency fails CI.
- [ ] **CORE-01.T02 — Define identities and validated values.** Implement non-interchangeable profile/workspace/task/account/operation IDs, scoped provider references, bounded strings and explicit money/time types. IDs are not authorization. **Verify:** compile-time ID separation and property tests for invalid currency, size and timestamp values.
- [ ] **CORE-01.T03 — Specify version-one records.** Define GoalContract, Workspace, Task, Capability, Observation, Evidence, ActionProposal, Approval, Operation, Receipt, ViewDefinition and MemoryRecord. Keep provider sessions secondary and inferred/source facts separate. **Verify:** golden round-trips including missing optional fields and unresolved writes.
- [ ] **CORE-01.T04 — Implement bounded framing and errors.** Start with length-prefixed JSON envelopes carrying correlation, deadline, role and payload type; reject lengths before allocation, cap nesting, and separate control traffic from artifacts. **Verify:** fragmented/oversized/malformed/repeated-request fixtures remain bounded.
- [ ] **CORE-01.T05 — Authenticate launched workers.** Use inherited private endpoints or one-use bootstrap handles, tied to the supervisor's worker instance/role; use socket permissions/pipe ACLs as defense in depth. **Verify:** unrelated clients and a browser claiming broker authority are rejected.
- [ ] **CORE-01.T06 — Negotiate versions and migrate records.** Separate live protocol negotiation from durable schema migrations; security-semantic incompatibility is not silently downgraded. **Verify:** documented old versions work; unsupported majors and unknown privileged fields fail explicitly.
- [ ] **CORE-01.T07 — Propagate cancellation and backpressure.** Use bounded queues/credits, reserved stop capacity and artifact references for large payloads. **Verify:** cancellation crosses two busy worker boundaries without waiting for a large stream.
- [ ] **CORE-01.T08 — Publish conformance and fuzzing.** Combine codec fuzz targets, old-schema fixtures, negative authentication and invalid action bindings into CI. **Verify:** reproducible compatibility report and reviewed golden changes.

**Order:** T01 → T02 → T03 → T04; T05/T06 can proceed together; T07 integrates both; T08 qualifies the result. Coordinate the threat model with #6 rather than waiting for every security feature to finish.

**Review correction:** PID/UID matching alone is not strong isolation against an unrestricted same-user process. Keep that limit explicit and require #9 containment for hostile workers. No implementations or passing tests are claimed by this comment.

#### Comment 5706164060 — Jordan-Hall — 2026-09-16T23:39:45Z

Source: https://github.com/Jordan-Hall/browser/issues/2#issuecomment-5706164060 | Updated: 2026-09-16T23:39:45Z

<!-- intent-core-review:2026-09-17:requirement-CORE-01 -->
###### CORE-01 review — PR existence and green smoke CI are not contract sign-off

Reviewed this requirement/proposal, #121–#128, the corresponding PRs #785–#792 and their discussions, with relevant source checks. All eight task PRs exist; material contract/integration findings remain. No code or review-thread resolution was changed in this pass.

| Task / PR | Required improvement |
| --- | --- |
| #121 / #785 | Commit/test the dependency lock; use locked resolution; strengthen the architecture guard beyond a finite denylist and test the real checker with renamed/unlisted/indirect dependencies. Run doctests explicitly and qualify advertised platforms. |
| #122 / #786 | Test money/ID/value types through the actual envelope, not only direct Serde. The i128 money domain needs a lossless JSON representation through serde_json::Value; define non-empty provider identifiers and explicit nil-ID policy. |
| #123 / #787 | Make inconsistent canonical-argument and supplied argument hashes unrepresentable/rejected on deserialization. Define per-record unknown-field and bounds policies; require account/capability/source-precondition binding before authority-bearing use. |
| #124 / #788 | Enforce serialization budgets before allocation growth, route schema before typed payload decoding and specify real stable wire-error encoding. Test fragmented, oversized, deeply nested and cross-version inputs through the whole codec. |
| #125 / #789 | Authenticate a real launched worker, not a same-process fixture or inherited socketpair creator PID. Bind bootstrap use, process instance, role and channel; make synthetic evidence unavailable to production authorization. |
| #126 / #790 | Connect negotiated versions to implemented envelope codecs. Bound proposal/migration inputs and validate migrated target records; migration metadata alone is not proof that bytes match the target schema. |
| #127 / #791 | Prevent QueueLimits construction bypass, retire completed cancellation records safely, separate progress metrics, bound bytes as well as item counts, and wire epoch/deadline checks into dispatch. Real subprocess cancellation is still required. |
| #128 / #792 | Fix the undeclared serde_json fuzz dependency, compile/smoke fuzz targets in CI, expand beyond synthetic smoke cases, and retain structured failing reports rather than uploading evidence only after success. |

###### Cross-contract improvements
Use one action-binding and operation/receipt vocabulary across contracts, state and outbox. Negotiation does not grant permission, IDs do not grant access, a hash does not authenticate a caller, and a syntactically valid record is not an approved action. Unknown security semantics must not be silently discarded.

The current CI command is `cargo test --workspace --all-targets`; Cargo defines that as lib/bins/tests/benches/examples, not documentation tests. Add a separate doctest invocation so compile-fail ID-separation examples actually become a release gate. Reference: https://doc.rust-lang.org/cargo/commands/cargo-test.html#target-selection . The workflow also needs a reproducible lock and separate fuzz/platform coverage; it should not imply those passed today.

###### Requirement-level acceptance
A clean pinned build must demonstrate: lossless supported values through the negotiated codec; malformed authoritative records rejected; authenticated real worker exchange; cancellation under saturated bulk I/O; actual migration compatibility and fuzz builds; and regression failures when each claimed invariant is deliberately broken. Record reviewed/integrated SHAs and fixture/test IDs for each child.

Preserve the Rust-first, narrow-dependency design and safe-code defaults. Document the small OS FFI boundary accurately rather than claiming the whole dependency/platform stack contains no unsafe code. Link remaining blockers through the task/PR table before marking CORE-01 complete.

#### Comment 5706716021 — Jordan-Hall — 2026-09-17T00:48:28Z

Source: https://github.com/Jordan-Hall/browser/issues/2#issuecomment-5706716021 | Updated: 2026-09-17T00:48:28Z

###### CORE-01 review follow-up — contract closure before integration

Reviewed all eight task PRs #785–#792, their source, existing findings and task requirements. Follow-up comments are on every PR and every child #121–#128. This is review coverage, not implementation approval.

###### Priority implementation corrections
The authorization boundary must bind canonical action bytes, account, capability, target, source preconditions, expiry and canonicalization version (#123/#787). An imported `Approved` enum is data, not a broker-issued grant. The same binding must survive the codec and outbox rather than being rebuilt from unrelated inputs.

Require encode/decode closure under the same validated limits (#124/#788): encoding currently enforces byte size while decoding also imposes structural budgets. Connect negotiated versions to implemented codecs and the authenticated channel (#125/#126); range intersection and synthetic peer evidence do not qualify a real worker session. Equal-version migration also needs an explicit validated/unvalidated output contract.

###### Task-level follow-ups
| Task | Added review requirement |
| --- | --- |
| #121 T01 | One toolchain source of truth; locked resolution, actual negative architecture fixtures and explicit doctests. |
| #122 T02 | Lossless money through the real envelope; signed adjustments separate from executable budgets; actual Unknown-scale coverage. |
| #123 T03 | Constructor/import binding checks; identical bytes with a changed account/capability must not inherit approval. |
| #124 T04 | Same-policy encode/decode boundaries, bounded encoding and explicit malformed-stream disposition. |
| #125 T05 | Channel-bound launch authentication with consumed registry entry, generation and lifecycle direction/instance checks. |
| #126 T06 | Real negotiated message exchange and validation of no-op as well as transformed migrations. |
| #127 T07 | Current-generation positive activity, safe retirement and queue-full cancellation acknowledgement semantics. |
| #128 T08 | Requirement-to-test coverage manifest and mutation-tested gates with failure artifacts. |

Preserve safe Rust in authority-neutral contracts/orchestration and audit narrow platform adapters separately; do not describe the whole stack as zero-unsafe without checking those adapters and dependencies. Keep source review, unit tests, subprocess tests and platform qualification distinct. Prior green smoke checks do not discharge these findings.

No code, issue state, merge status or existing review resolution was changed.


---

<a id="issue-3"></a>
## #3 — [P0][CORE-02] Durable state, outbox and artifacts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/3
**Created:** 2026-09-15T12:03:52Z | **Updated:** 2026-09-17T00:48:55Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #13 | **Body-declared parent:** #1
**PRs mentioning this issue:** [#793](https://github.com/Jordan-Hall/browser/pull/793), [#794](https://github.com/Jordan-Hall/browser/pull/794), [#795](https://github.com/Jordan-Hall/browser/pull/795), [#796](https://github.com/Jordan-Hall/browser/pull/796), [#797](https://github.com/Jordan-Hall/browser/pull/797), [#801](https://github.com/Jordan-Hall/browser/pull/801), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#803](https://github.com/Jordan-Hall/browser/pull/803), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #1

#### Objective
Make task/workspace state durable enough that crashes, restarts and external side effects never reduce the system to guessing from a chat transcript.

#### Scope
- SQLite transactional state for workspaces, tasks, grants, operations and migrations.
- Event journal plus transactional outbox/inbox/consumer cursors.
- Unique operation IDs before externally significant dispatch.
- Content-addressed artifact store for evidence, patches, files, snapshots and protocol fixtures.
- Retention/GC that respects references, privacy scope and legal deletion.
- Atomic state + pending-dispatch persistence where required.

#### Design requirements
- A worker must be able to resume from durable state without provider conversation history being authoritative.
- Side-effect intent is persisted before dispatch; observations/results are appended, not retroactively invented.
- Artifact hashes are immutable references; mutable metadata lives separately.

#### Acceptance criteria
- [ ] Crash between state transition and dispatch leaves an unambiguous recoverable operation record.
- [ ] Crash after dispatch but before result recording produces an explicit uncertain/reconciliation state where appropriate.
- [ ] Database migrations are reversible/restore-tested for supported upgrade paths.
- [ ] Artifact integrity is checked by hash and corrupt/missing blobs fail visibly.
- [ ] Concurrent consumers cannot double-apply one journal event.
- [ ] Retention does not delete referenced evidence or leak deleted private derivatives.

#### Tests
Crash/fault injection around every transaction boundary; duplicate-delivery tests; migration fixtures; disk-full/corruption simulations; artifact hash mismatch cases.

#### Dependencies
- CORE-01

**First phase:** P0  
**Maturity target:** P1  
**Workstream:** Runtime and contracts

### Discussion (3 comments)

#### Comment 5681738388 — Jordan-Hall — 2026-09-15T14:16:26Z

Source: https://github.com/Jordan-Hall/browser/issues/3#issuecomment-5681738388 | Updated: 2026-09-15T14:16:26Z

<!-- intent-implementation-v1:CORE-02 -->
###### Implementation proposal — CORE-02

Use one authoritative SQLite writer per profile, current-state projections plus a durable event journal, and a transactional outbox. Proposed modules: `state-store`, `journal`, `artifacts`; prerequisite: #2. Do not distribute writable database connections to untrusted workers.

- [ ] **CORE-02.T01 — Database ownership and migrations.** Configure foreign keys, bounded contention and tested durability; store migration checksums and expose typed repository methods. **Verify:** concurrent requests serialize and interrupted/modified migrations fail or recover correctly.
- [ ] **CORE-02.T02 — Operation/journal records.** Separate immutable intent/argument digest/account/source revision from attempts and outcome observations. Update projection and append event in one transaction with expected-revision checks. **Verify:** racing updates produce one success and one explicit conflict.
- [ ] **CORE-02.T03 — Transactional outbox.** Persist pending dispatch with the approved transition; claim using worker epoch/lease and record the attempt before network I/O. **Verify:** failpoints before commit, after commit and after send recover as unsent, acknowledged or uncertain—not inferred success.
- [ ] **CORE-02.T04 — Inbox and cursors.** Apply event effects, dedup identity and cursor advancement atomically. Separate delivery order from source revision. **Verify:** duplicates cannot double-apply; newer out-of-order revisions remain processable.
- [ ] **CORE-02.T05 — Scoped immutable artifacts.** Stream through bounded quarantine, hash and flush, publish atomically, then register references. Resolve opaque handles only after access checks. **Verify:** corrupted blobs fail and cross-profile hashes do not confer access.
- [ ] **CORE-02.T06 — Retention and deletion.** Suppress retrieval immediately, mark eligible unreferenced blobs, then run resumable GC respecting active operations/holds. **Verify:** deleted private records disappear from retrieval without deleting retained receipts.
- [ ] **CORE-02.T07 — Backup and restore.** Snapshot database consistently, enumerate referenced blobs, authenticate the manifest, and restore into a fresh directory. Keep external dispatch disabled during recovery review. **Verify:** missing blobs fail visibly and uncertain writes remain uncertain.
- [ ] **CORE-02.T08 — Fault qualification.** Exercise disk full, denied writes, corrupt manifests, busy readers, cursor failures and abrupt VM power loss. **Verify:** every designed failpoint converges to a documented state with no duplicate fixture effect.

**Important boundaries:** WAL is for tested local-filesystem configurations; database transactions do not make remote operations atomic. Scope plaintext deduplication by privacy zone to prevent existence leaks. Never hold a database transaction across external network I/O. Review tasks separately; closure requires commit, migration and fault-test evidence.

#### Comment 5706155786 — Jordan-Hall — 2026-09-16T23:38:54Z

Source: https://github.com/Jordan-Hall/browser/issues/3#issuecomment-5706155786 | Updated: 2026-09-16T23:38:54Z

<!-- intent-core-review:2026-09-17:requirement-CORE-02 -->
###### CORE-02 review — storage needs stronger cross-operation invariants

Reviewed the requirement/proposal, #129–#136, storage implementation in the current stack, and PR discussions #793–#797/#801. Backup/restore and storage fault qualification still have no implementation PR in the reviewed inventory. This is a review, not a completion or fix claim.

###### Findings and owning tasks
| Task / PR | Required correction before sign-off |
| --- | --- |
| #129 / #793 | Reject populated unrelated zero-application-ID databases without modifying them; choose migrations under the writer lock; compare user_version with the validated ledger; enforce a profile-wide owner/coordination boundary rather than relying on a private Connection field. |
| #130 / #794 | Preserve the exact dispatch attempt identity through acceptance/verification/reconciliation; retain compensation origin and its success path; distinguish source schema version from the actual source revision/preconditions. |
| #131 / #795 | Keep dispatch bytes behind durable attempt start; fence leases by worker instance/generation; discover abandoned attempts; stop cancelled outboxes starving live work; bind staged bytes/destination to the authorized action rather than merely hashing them for corruption checks. |
| #132 / #796 | Keep the fixed aggregate-effect read verification. Also validate/repair missing materialization on duplicate delivery, bound total effect bytes, and distinguish delivery sequence from source revision. |
| #133 / #797 | Keep the three existing fixes: retry-byte comparison, scoped reference counts and newly created parent synchronization. Still qualify profile-root/OS-permission/symlink boundaries, orphan publication cleanup and platform durability; a non-Unix directory-sync no-op is not equal durability evidence. |
| #134 / #801 | Fix affected-row handling through the INSTEAD OF view trigger; make suppression an explicit lifecycle result rather than byte_size = -1; avoid starvation in bounded GC scans; prevent new holds/handles racing irreversible unlink. |
| #135 | Implement an authenticated consistent DB/blob manifest, fresh-directory restore, verified publication and dispatch-disabled recovery review. A checkpoint of the DB alone does not protect referenced blobs from concurrent GC. |
| #136 | Add failpoint/process/VM qualification with an independent external effect ledger and reproducible evidence. Current smoke CI is not the stated power-loss qualification. |

###### Architectural improvement
Define one lifecycle/ownership protocol spanning publication, references, suppression, holds, backup pins and deletion generations. A database transaction protects local rows; it does not make an earlier filesystem unlink or remote send reversible. Every public mutation should state what is committed when an error is returned and how a retry proves it is the same request.

Prefer immutable validated types, typed state/attempt transitions and small shared persistence helpers over three independently maintained SQL state machines. Add bounded pagination and aggregate byte budgets to journals, inbox effects, claim batches and maintenance work; keep expensive file hashing off the control/UI executor.

###### Integration gates
Test concurrent owner startup, migration interruption, projection/journal atomicity, cancelled claim starvation, accepted-but-unrecorded sends, duplicate inbox corruption, reference/hold preservation, publish-before-metadata interruption, GC/backup races, missing blobs and repeated recovery. Preserve verified receipts and explicit uncertainty through all paths.

The original read-path/artifact fixes are acknowledged rather than re-reported as unfixed. All remaining findings require fix commits and regression evidence on their owning PRs. Child/PR links above are the review navigation; none of these issues should be closed merely because a PR exists.

#### Comment 5706719472 — Jordan-Hall — 2026-09-17T00:48:55Z

Source: https://github.com/Jordan-Hall/browser/issues/3#issuecomment-5706719472 | Updated: 2026-09-17T00:48:55Z

###### CORE-02 review follow-up — storage success must mean a durable, recoverable result

Reviewed the six existing task PRs #793–#797 and #801, and added linked follow-ups to all eight tasks #129–#136. No implementation PR was found for backup/restore T07 or storage qualification T08. This is not storage completion or approval.

###### New concrete findings
- **P1 — reference registration can report success without a pin.** #797 uses `INSERT OR IGNORE`; empty reference identifiers pass `BoundedText` but violate table CHECK constraints, so insertion is ignored and the Rust method returns `Ok`. A reduced SQLite reproduction confirmed this. Fix details: https://github.com/Jordan-Hall/browser/pull/797#issuecomment-5706481921 . A backup/checkpoint/receipt must not rely on a pin that never existed.
- **Existing-store identity can silently change.** #793 initializes missing metadata with a fresh UUID even for an existing database. Separate creation from validation/recovery (#129).
- **Committed mutation and returned outcome can disagree.** #794 commits before reloading its result; a failed or interleaved readback can misrepresent the outcome of the completed transaction (#130). Preserve exact committed revision and safe retry semantics.
- **Maintenance is not fully bounded.** #801 prunes all expired holds before checking its batch limit, including limit zero. Its error logger can also panic when truncating a non-ASCII message at byte 2048 (#134).

###### Existing blockers remain
#794 still needs attempt continuity and compensation-origin handling. #795 still needs approved-action binding, opaque claims, generation fencing, abandoned-attempt recovery and cancelled-row retirement. #796's aggregate read fix is present, but duplicate acknowledgements, total effect bytes and delivery-versus-source revision semantics need the recorded corrections.

At the inspected #801 head `fbfcfceb17540e5fe80bec555385e097f3787455`, run `35086737317` fails all three retention tests with `ConcurrentSuppression`; formatting/Clippy passing does not make retention correct. Fix the suppression/view and GC coordination defects before layering backup over them.

###### Closure requirements
Use one declared profile/state/artifact coordination boundary; distinguish rejected, committed, already-recorded and uncertain outcomes; verify references actually exist; bound maintenance work and preserve recovery evidence after secondary failures. Add these counterexamples to #136, then qualify backup pins and complete inventories in #135. SQL reductions are evidence for the identified SQL behavior, not repository Rust or power-loss qualification.

No code, issue states or review dispositions were changed.


---

<a id="issue-4"></a>
## #4 — [P0][CORE-03] Worker supervision and resource scheduler

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/4
**Created:** 2026-09-15T12:04:03Z | **Updated:** 2026-09-17T00:49:12Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #13 | **Body-declared parent:** #1
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #1

#### Objective
Provide a Rust-owned supervisor for browser, connector, agent, inference, speech and desktop workers so failures or resource pressure cannot freeze the trusted UI or silently orphan authority.

#### Scope
- Worker lifecycle: spawn, handshake, capability registration, health, restart, drain and terminate.
- Priority queues for deterministic UI/control, speech, foreground agents and background work.
- CPU/GPU/memory/concurrency budgets and per-task deadlines.
- Cancellation propagation and lease revocation.
- Backpressure between supervisor and workers.
- Crash-loop detection and degraded-mode operation.
- Resource telemetry needed for local-model scheduling.

#### Design requirements
- Trusted shell remains responsive even when inference/browser/agent workers stall.
- Stop/cancel is a privileged fast path and is not queued behind model work.
- Workers receive only task-scoped state/capabilities.

#### Acceptance criteria
- [ ] A stalled or OOM worker cannot prevent deterministic navigation or local stop acknowledgement.
- [ ] Crashed workers restart according to policy without duplicating externally significant work.
- [ ] CPU/GPU/memory budgets are observable and enforceable.
- [ ] Cancellation invalidates worker leases before any later dispatch.
- [ ] Priority inversion tests show control/speech responsiveness under heavy inference load.
- [ ] Crash loops enter a visible degraded state instead of infinite restart.

#### Dependencies
- CORE-01
- CORE-02

**First phase:** P0  
**Maturity target:** P3  
**Workstream:** Runtime and contracts

### Discussion (3 comments)

#### Comment 5681748232 — Jordan-Hall — 2026-09-15T14:16:56Z

Source: https://github.com/Jordan-Hall/browser/issues/4#issuecomment-5681748232 | Updated: 2026-09-15T14:16:56Z

<!-- intent-implementation-v1:CORE-03 -->
###### Implementation proposal — CORE-03

Implement a Rust supervisor over typed worker instances, not an LLM-owned process loop. Proposed locations: `apps/supervisor`, `crates/supervisor`, `scheduler`, `platform-process`; dependencies #2 and #3.

- [ ] **CORE-03.T01 — Worker registry/launch specifications.** Define WorkerSpec with verified executable identity, role, protocol, resources and restart class; allocate a fresh epoch per launch. **Verify:** changed executable/role cannot reuse an old launch authorization.
- [ ] **CORE-03.T02 — Process lifecycle/health.** Implement Starting → Ready → Draining → Stopped/Failed; require handshake before Ready and observe OS exits as well as heartbeats. **Verify:** stuck, silent and dead processes yield distinct bounded diagnostics.
- [ ] **CORE-03.T03 — Admission/priorities.** Bound worker count, memory and CPU admission; reserve queues for stop, interactive work and speech; age background jobs without stealing stop capacity. **Verify:** load cannot starve cancellation or foreground input.
- [ ] **CORE-03.T04 — Platform limits.** Translate leases to tested process/container/VM limits; measure effective enforcement and reject unsupported unattended profiles. **Verify:** resource-abuse probes fail within the advertised boundary.
- [ ] **CORE-03.T05 — Lease-first cancellation.** Revoke the epoch before notifying the worker; deny later dispatch, then attempt graceful cancellation and bounded process-tree termination. **Verify:** late buffered output cannot execute under a revoked lease; in-flight external effects remain tracked.
- [ ] **CORE-03.T06 — Crash-loop/degraded mode.** Bound restarts/backoff, isolate broken integrations and restore only from runtime-owned checkpoints with a fresh handshake. **Verify:** cached workspaces remain usable after repeated model/provider failures.
- [ ] **CORE-03.T07 — Resource observation/yield.** Collect content-free queue, memory, CPU and available accelerator metrics; coordinate model unload/KV eviction and speech reservations. **Verify:** measured interaction/speech responsiveness under sustained coding load.
- [ ] **CORE-03.T08 — Concurrency qualification.** Stress rapid launch/stop, worker churn, suspend/resume and permission revocation with race-focused tests. **Verify:** logging and saturated progress streams cannot block the control plane.

**Review correction:** distinguish hard OS limits, admission estimates and cooperative GPU scheduling. Do not promise universal fine-grained GPU preemption or VRAM enforcement. Local stop acknowledgement is different from confirmed provider termination or external rollback. Task completion requires actual race/resource test evidence.

#### Comment 5706147272 — Jordan-Hall — 2026-09-16T23:38:09Z

Source: https://github.com/Jordan-Hall/browser/issues/4#issuecomment-5706147272 | Updated: 2026-09-16T23:38:09Z

<!-- intent-core-review:2026-09-17:requirement-CORE-03 -->
###### CORE-03 review — define the supervisor's enforceable boundary

Reviewed the requirement/proposal, task breakdown #137–#144 and the IPC/state prerequisites. No CORE-03 implementation PR was found. The current in-memory queue/authentication helpers are prerequisites, not evidence of a working process supervisor.

###### Task-specific requirements
| Task | Review improvement and required proof |
| --- | --- |
| #137 Worker registry | Immutable launch descriptor binds verified executable, role, protocol, account/task scope, resources and restart policy. Fresh generation per launch; globally one-use bootstrap; no authorization reused after executable/role replacement. |
| #138 Lifecycle/health | Require authenticated readiness; distinguish handshake timeout, missed heartbeat, blocked progress and actual OS exit. Bound diagnostics and reap children; heartbeat alone cannot declare useful progress or completion. |
| #139 Admission/priorities | Reserve control/interactive/speech capacity by both count and bytes; enforce aggregate admission and fair background aging. Oversized head-of-line work and a progress/ack flood must not block Stop. |
| #140 Platform limits | Report hard OS limits, estimates and cooperative limits separately. Install containment before untrusted execution; verify process-tree coverage and reject unsupported unattended profiles. Compile-time availability is not runtime enforcement evidence. |
| #141 Cancellation | Revoke current epoch before requesting termination; atomically gate dispatch against revocation. Preserve already-admitted uncertain external attempts. Test late output, reused IDs and graceful-timeout escalation. |
| #142 Crash loops | Bound restart count/backoff with explicit reset conditions, circuit-break unhealthy integrations and require fresh auth/checkpoints. Restart must not duplicate outbox effects or reset lifetime resource limits. |
| #143 Observation/yield | Content-free metrics with bounded collection cost; define unavailable/stale observations; hysteresis for unload/yield requests. Measure foreground and speech latency under load rather than promising universal GPU preemption. |
| #144 Qualification | Real subprocess fixtures, rapid launch/stop, saturation, suspend/resume, PID/epoch replacement, revocation races and descendant-process escape/termination cases. Model tests complement, not replace, platform tests. |

###### Fix prerequisites instead of assuming them
#789 still has the inherited-socketpair creator-PID authentication problem. #790's negotiated protocol is not wired into the V1-only codec. #791 allows invalid queue-limit construction, lacks cancellation-record retirement on StreamEndpoint and calls an in-memory pair a two-worker test. #793 does not enforce a profile-wide sole owner. #795 needs opaque/fenced claims and abandoned-attempt recovery.

Define the ownership/dispatch linearization boundary with #211/#212. Keep blocking database/filesystem work and optional metrics away from the control executor. A safe Rust API should make stale/unvalidated states hard to express; add unsafe code only in narrow platform wrappers with explicit lifetime and OS preconditions.

**Closure gate:** successful permitted work plus rejection/race tests, real platform-enforcement evidence and measured control responsiveness. The native shell/cached workspace should remain usable when model workers repeatedly fail. No implementation or passing supervisor qualification is claimed by this review.

#### Comment 5706721638 — Jordan-Hall — 2026-09-17T00:49:12Z

Source: https://github.com/Jordan-Hall/browser/issues/4#issuecomment-5706721638 | Updated: 2026-09-17T00:49:12Z

###### CORE-03 planning review follow-up — one owner for lifecycle, authority and accounting

Reviewed this requirement, its implementation proposal, all eight child tasks and the current CORE PR inventory. No CORE-03 implementation PR was found. The following refines acceptance; it is not a list of defects in an existing supervisor or a claim of completed implementation.

| Task | Additional implementation/test refinement |
| --- | --- |
| #137 T01 | Fingerprint security-relevant launch configuration, not only executable bytes: role, arguments, cwd policy, inheritance, confinement and protocol. Changing configuration invalidates prior launch authority. |
| #138 T02 | A single-owner epoch-tagged lifecycle reducer defines precedence for Ready/Exit/Cancel and releases resources once across every event ordering. |
| #139 T03 | Reserve the complete admissible resource vector atomically; queued demand is not committed capacity. Test competing scarce-resource requests and duplicate release. |
| #140 T04 | Enforcement evidence is bound to backend/configuration/instance and revalidated after changes. Partial setup must not produce an unconfined fallback. |
| #141 T05 | A failed cancellation journal write must not preserve in-memory dispatch authority. Distinguish local revocation, durable acknowledgement, termination and external uncertainty. |
| #142 T06 | Specify circuit reset authority and retry deduplication. A fresh epoch or supervisor restart must not accidentally reset the crash budget. |
| #143 T07 | Correlate yield acknowledgements to request, resource instance and reservation; telemetry and acknowledgements cannot credit the same release twice. |
| #144 T08 | Keep the test oracle independent of production transitions and mutation-test it; add real process/transport cases rather than relying solely on in-memory endpoints. |

###### Integration prerequisites
Use the repaired actual-channel authentication from #125/#789, codec negotiation from #126/#790, cancellation lifetime from #127/#791, and durable attempt/outcome APIs from #130/#131. Process health, executable authority and durable task outcome remain different facts. A heartbeat is not verified task completion, and killing a worker does not undo an accepted external request.

Keep control execution responsive under storage and telemetry saturation; reserved queue slots alone do not prevent a blocked control executor. Distinguish hard platform limits from admission estimates and cooperative accelerator yielding. Unsupported profiles must fail honestly, but that rejection is not evidence of full platform implementation.

Follow-ups are posted on every child task. No code, states or completion claims were changed.


---

<a id="issue-5"></a>
## #5 — [P1][CORE-04] Recovery and bounded replay

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/5
**Created:** 2026-09-15T12:04:13Z | **Updated:** 2026-09-17T00:49:34Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #13 | **Body-declared parent:** #1
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#804](https://github.com/Jordan-Hall/browser/pull/804), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#808](https://github.com/Jordan-Hall/browser/pull/808), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #1

#### Objective
Make recovery deterministic after crashes, suspend/resume, network loss and partial external completion without ever replaying irreversible production actions blindly.

#### Scope
- Durable checkpoints and task snapshots.
- Restart/resume policy per operation effect class.
- Fixture/captured-observation replay for debugging and regression.
- Handling expired auth, stale observations, partial provider sessions and missing artifacts.
- Explicit `NeedsReconciliation` path for ambiguous side effects.
- Recovery UX describing what resumed, what was cancelled and what remains uncertain.

#### Design requirements
- Production replay must never resubmit purchases, bids, messages, deletions or other irreversible writes merely because the local process missed the response.
- Trace replay uses captured observations or resettable fixtures.
- Recovery revalidates deadlines, grants and freshness rather than assuming pre-crash authority remains valid.

#### Acceptance criteria
- [ ] Captured traces replay without any live irreversible side effect.
- [ ] Unknown write outcomes enter reconciliation instead of automatic retry.
- [ ] Suspend/resume and restart recover supported read-only tasks from durable state.
- [ ] Expired grants/auth are surfaced and require the correct recovery path.
- [ ] Recovery after partial artifact persistence is deterministic and tested.
- [ ] User-facing task history clearly distinguishes resumed, failed, cancelled and uncertain operations.

#### Dependencies
- CORE-02
- CORE-03

**First phase:** P1  
**Maturity target:** P4  
**Workstream:** Runtime and contracts

### Discussion (3 comments)

#### Comment 5681755881 — Jordan-Hall — 2026-09-15T14:17:20Z

Source: https://github.com/Jordan-Hall/browser/issues/5#issuecomment-5681755881 | Updated: 2026-09-15T14:17:20Z

<!-- intent-implementation-v1:CORE-04 -->
###### Implementation proposal — CORE-04

Implement recovery as an explicit planning step before worker dispatch. Proposed locations: `crates/recovery`, `crates/replay`, `apps/recovery-cli`, `tests/recovery`; dependencies #3/#4, with transaction reconciliation integrated through #79.

- [ ] **CORE-04.T01 — Recovery classification.** Define a matrix for safe reads, reversible local writes, provider-idempotent writes and ambiguous external effects, including evidence required to resume/reconcile/abandon. **Verify:** every registered operation has a recovery policy.
- [ ] **CORE-04.T02 — Consistent checkpoints.** Persist graph revision, committed artifacts, consumed event positions, provider secondary IDs and worker epoch transactionally. Exclude live OS/CEF pointers and credentials. **Verify:** checkpoint restoration resolves all declared dependencies or reports missing state.
- [ ] **CORE-04.T03 — Startup recovery planner.** Load unfinished tasks, current policy/clocks/auth and generate RecoveryPlan records before starting work. **Verify:** revoked or expired authority cannot resume merely because it existed before the crash.
- [ ] **CORE-04.T04 — Local/external reconciliation.** Compare before/after local versions; use connector-specific read-only provider queries for uncertain external attempts. **Verify:** success, proven non-commit and still-unknown remain distinct; no blind replay.
- [ ] **CORE-04.T05 — Production-free replay.** Replay captured observations/fixtures in a process without production credentials or side-effect dispatcher; inject time, random inputs and provider events. **Verify:** a purchase trace cannot contact a live merchant.
- [ ] **CORE-04.T06 — Provider resume/reseed.** Use actual negotiated resume where supported; otherwise create a new scoped session from shared goal/evidence/artifacts and revalidate repository bases. **Verify:** provider replacement retains workspace state without pretending to preserve private session internals.
- [ ] **CORE-04.T07 — Recovery UX.** Show resumed, stopped, login-required and uncertain states with inspect/resume/reconcile controls. **Verify:** every user action resolves current policy and task state.
- [ ] **CORE-04.T08 — Restart qualification.** Test process kills, sleep/wake, unavailable models, revoked accounts, missing blobs and incompatible schemas. **Verify:** each failpoint has a recorded convergence result.

**Critical rule:** a timeout after dispatch may represent an accepted transaction. Never interpret unknown as safe-to-repeat, and never let restoring a VM or file snapshot imply rollback of an external service. Approve these tasks independently before converting them into native sub-issues.

#### Comment 5706139701 — Jordan-Hall — 2026-09-16T23:37:26Z

Source: https://github.com/Jordan-Hall/browser/issues/5#issuecomment-5706139701 | Updated: 2026-09-16T23:37:26Z

<!-- intent-core-review:2026-09-17:requirement-CORE-04 -->
###### CORE-04 review — recovery is not implemented by the current stack

Reviewed this requirement, its implementation proposal, task breakdown #145–#152, and the existing storage/IPC PR discussions. No CORE-04 implementation PR was found. Treat the following as a tightened implementation/review contract, not a list of fixes already made.

###### Task-by-task closure requirements
| Task | Required implementation/evidence |
| --- | --- |
| #145 CORE-04.T01 | Exhaustive operation/recovery classification; unsupported or ambiguous writes default to reconciliation/blocking, not retry. Provider idempotency includes account, key, payload, operation and validity window. |
| #146 CORE-04.T02 | One consistent checkpoint revision with task/graph state, artifact references, cursors, attempt/provider references and lineage; no credentials or live handles. Publication and retention must coordinate so checkpoint blobs cannot disappear mid-commit. |
| #147 CORE-04.T03 | Startup dispatch barrier; current authority/account/deadline/source checks; durable revisioned recovery plans; repeated startup must converge rather than issue fresh effects. |
| #148 CORE-04.T04 | Evidence-bound local/provider reconciliation; authoritative non-commit distinct from missing/stale query results; original and compensation attempts remain separate. |
| #149 CORE-04.T05 | Replay environment structurally lacks production write/credential adapters; missing capture cannot trigger live fallback. Versioned, bounded traces with injected nondeterminism and explicit divergence. |
| #150 CORE-04.T06 | Actual negotiated resume or explicit reseed/block; fresh runtime epoch; source/access/repository-base revalidation; preserve workspace identity without claiming provider-internal continuity. |
| #151 CORE-04.T07 | Typed recovery view model and current-state-checked actions. Stopped, unknown, verified, login-required and compensated remain distinct. Do not offer blind Retry for uncertain writes. |
| #152 CORE-04.T08 | Independent fixture effect ledger; process/VM interruption matrix; revoked accounts, missing/suppressed blobs, unsupported schemas, unavailable models and recovery-under-disk-failure tests. |

###### Existing defects that must not be hidden by the planner
#794 permits an attempt identity to change when recording acceptance and loses compensation origin during reconciliation. #795 has no public recovery path for abandoned attempting messages and retains cancelled pending work in its claim population. #801's retention failure and cross-writer deletion race affect checkpoint availability. Fix the owning primitives and add regressions before building a planner that assumes they are correct.

###### Cross-cutting design improvements
A recovery plan is a proposal, not a grant. Store expected revisions and revalidate when the user or supervisor executes it. Retain evidence even when an old worker's authority is revoked, but only through a checked outcome/reconciliation path. Separate wall-clock deadlines from process-local monotonic timers and define sleep/wake/backwards-clock handling. Backup restore and VM rollback do not reverse external service actions.

**Acceptance:** demonstrate a safe read resumes, a conflicting local write blocks, an accepted-but-unrecorded external effect reconciles without duplication, and the same workspace remains inspectable without inference. Keep fixture, subprocess, platform and real-provider evidence separate. A CLI/state-model increment is valuable but does not silently satisfy the complete native recovery-UI requirement.

Review comments and proposed tests are attached to the child tasks. No code, merge, closure or passing CORE-04 qualification is asserted.

#### Comment 5706724466 — Jordan-Hall — 2026-09-17T00:49:34Z

Source: https://github.com/Jordan-Hall/browser/issues/5#issuecomment-5706724466 | Updated: 2026-09-17T00:49:34Z

###### CORE-04 planning review follow-up — recover facts, not assumptions about success

Reviewed the requirement, both the original proposal and existing review refinements, all eight task issues and the current PR inventory. No CORE-04 implementation PR was found. Follow-up comments are on every child; these remain implementation/acceptance guidance, not completed recovery functionality.

| Task | Additional acceptance requirement |
| --- | --- |
| #145 T01 | Classify known local commit with lost acknowledgement/readback separately from rollback and uncertain external effects. Later missing information must not erase stronger commit evidence. |
| #146 T02 | Commit to the complete checkpoint dependency set; validating surviving blobs alone cannot detect a missing reference or cursor row. |
| #147 T03 | Enforce the startup/recovery barrier at every dispatch ingress, including old scheduler events and direct connector entry points. Inspection is separate from execution authority. |
| #148 T04 | Deduplicate outcome observations and define reservation consequences. Unknown external effects do not free commitments merely because a worker stopped. |
| #149 T05 | Validate capture completeness and expected terminal disposition; a truncated but syntactically valid trace must not vacuously pass. |
| #150 T06 | Handle crashes during provider attach/session creation before persisting the secondary reference. Do not create multiple write-enabled sessions after a lost acknowledgement. |
| #151 T07 | Show command accepted versus outcome observed. Reconnection resolves the same request identity before offering another execution. |
| #152 T08 | Keep the external fixture ledger outside the runtime snapshot/rollback domain so VM restore cannot erase duplicate-effect evidence. |

###### Prerequisites and evidence
Repair the owning storage/dispatch primitives first: #130's attempt and committed-result semantics, #131's abandoned attempts/result fencing, #133's false-success reference registration, and #134's failing suppression/GC behavior. Checkpoint/backup protection must use actual durable pins, not successful-looking return values.

A recovery plan is a proposal, not a grant. Revalidate current account rights, source preconditions, deadlines, suppression and runtime epoch when applying it. Original and compensating attempts retain separate histories. Never infer external rollback from restoring a VM or killing a process.

Run positive safe-read and permitted-write cases as well as negative cases; permanently blocking everything is not a working runtime. Keep fixture policy, real provider behavior, subprocess tests, native UI accessibility and VM/power-loss evidence separately labelled. No code, issue state, merge or review resolution changed in this pass.


---

<a id="issue-121"></a>
## #121 — [TASK][CORE-01.T01] Bootstrap the Rust workspace and architectural checks

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/121
**Created:** 2026-09-15T15:11:53Z | **Updated:** 2026-09-17T00:27:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #2
**PRs mentioning this issue:** [#785](https://github.com/Jordan-Hall/browser/pull/785), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #2

Task ID: `CORE-01.T01`

#### Proposed implementation
Create the Cargo workspace, pinned toolchain, formatting/lint configuration and minimal contracts/ipc/fixture-worker packages. Add a dependency-direction check so domain contracts cannot import GUI, CEF, OS brokers or provider SDKs.

#### Done when
A clean checkout builds on the initial CI host and a deliberately forbidden dependency makes the architecture check fail.

#### Guardrail
Keep optional engine/model builds outside default CI until artifacts/licenses are pinned; do not create empty crates for future concepts.

#### Prerequisites
None

Review: pending · Implementation: not started.

### Discussion (3 comments)

#### Comment 5690003441 — Jordan-Hall — 2026-09-16T00:16:32Z

Source: https://github.com/Jordan-Hall/browser/issues/121#issuecomment-5690003441 | Updated: 2026-09-16T00:16:32Z

Implementation started in stacked PR #785 (`core-01-t01-bootstrap`). GitHub CI run 35039256974 completed successfully: formatting, Clippy with warnings denied, workspace tests, and the architecture dependency check all passed. This PR is the verified base for `CORE-01.T02`.

#### Comment 5705926415 — Jordan-Hall — 2026-09-16T23:18:14Z

Source: https://github.com/Jordan-Hall/browser/issues/121#issuecomment-5705926415 | Updated: 2026-09-16T23:18:14Z

<!-- intent-core-review:2026-09-16:issue-121 -->
###### Review — CORE-01.T01

Parent: #2. Implementation reviewed: #785 at `fc7b62456f0d5feaa4f4c98d5be1953604328264`. The earlier green-CI comment demonstrates the configured checks passed; it does not establish that the dependency boundary is complete.

**Changes required before acceptance:** replace the fail-open forbidden-name prefix list with a reviewed dependency policy; test the real checker with renamed/unlisted packages, not only its predicate. Specify direct/transitive/build-dependency scope. Commit the application lockfile and use `--locked`; add explicit doctest execution because the current `--all-targets` command omits it. Pin CI actions to reviewed revisions.

**Task-level verification:** a clean checkout reproduces dependency resolution; each intended forbidden dependency makes CI fail; a broken compile-fail example fails the doctest job; compiler/target/lockfile identity is captured in evidence. Keep the initial-host acceptance separate from cross-platform transport qualification.

The minimal contracts/IPC crates and `forbid(unsafe_code)` boundaries are useful. The fixture worker can remain small, but #125/#127 need an actual protocol-speaking worker before claiming process-boundary tests. Do not create unrelated future crates as part of this repair.

Full source findings and existing review-thread references are posted on #785. Review only: no code or issue status changed.

#### Comment 5706550054 — Jordan-Hall — 2026-09-17T00:27:47Z

Source: https://github.com/Jordan-Hall/browser/issues/121#issuecomment-5706550054 | Updated: 2026-09-17T00:27:47Z

###### Review follow-up — CORE-01.T01

Parent: #2. Code and acceptance follow-up: https://github.com/Jordan-Hall/browser/pull/785#issuecomment-5706416708 . The earlier review remains open at the same implementation head.

Add a **toolchain-drift regression**: installation configuration and `rust-toolchain.toml` must select the same compiler after an upgrade. Prefer one checked-in source of truth.

For the dependency-boundary repair, run the actual `arch-check` executable against a fixture Cargo workspace containing a deliberately disallowed dependency, including an alias/target-specific case. A predicate unit test alone cannot prove the command examines the relevant Cargo graph. Keep the committed lockfile, locked clean build and explicit doctest gates in the acceptance checklist.

Completion should link the fixing commit, exact CI run and negative-fixture result. This comment records review guidance only; it does not supersede unresolved PR findings or mark the task implemented/approved.


---

<a id="issue-122"></a>
## #122 — [TASK][CORE-01.T02] Define typed identities and value objects

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/122
**Created:** 2026-09-15T15:12:00Z | **Updated:** 2026-09-17T00:28:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #2
**PRs mentioning this issue:** [#786](https://github.com/Jordan-Hall/browser/pull/786), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #2

Task ID: `CORE-01.T02`

#### Proposed implementation
Implement non-interchangeable ID newtypes and validated resource references. Define money, timestamps, bounded text, content hashes and account-qualified provider IDs with explicit validation.

#### Done when
Property tests reject swapped account/task types where possible and malformed monetary, timestamp and size values at runtime.

#### Guardrail
IDs must never imply permission; unknown currency scale or source timestamp remains explicitly unknown.

#### Prerequisites
`CORE-01.T01`

Review: pending · Implementation: not started.

### Discussion (3 comments)

#### Comment 5690116156 — Jordan-Hall — 2026-09-16T00:23:09Z

Source: https://github.com/Jordan-Hall/browser/issues/122#issuecomment-5690116156 | Updated: 2026-09-16T00:23:09Z

Implemented in stacked PR #786. After fixing formatter/Clippy findings without suppressions, CI run 35039705670 completed successfully on Rust 1.98.1. The typed-ID compile-fail doctest, deserialization validation tests, workspace tests, and architecture checks all pass. This verified head is the base for `CORE-01.T03`.

#### Comment 5705931359 — Jordan-Hall — 2026-09-16T23:18:36Z

Source: https://github.com/Jordan-Hall/browser/issues/122#issuecomment-5705931359 | Updated: 2026-09-16T23:18:36Z

<!-- intent-core-review:2026-09-16:issue-122 -->
###### Review — CORE-01.T02

Parent: #2. Reviewed implementation: #786 at `a9fbf6648214552dd4ab4431ad8d2fde07f2a5ff`, including its downstream envelope integration.

**Required corrections:** the public money type accepts i128 amounts, but #788 decodes via `serde_json::Value` without an exact wide-integer representation. Test the actual envelope path, not just direct serialization. Specify a canonical lossless representation and checked conversions for narrower persistence/consumer domains. Retain Unknown currency scale; do not silently use a guessed exponent.

Provider/account/resource wrappers currently accept empty identifiers because `BoundedText` checks only maximum length. Add non-empty identifier invariants on both constructors and deserialization, without changing legitimate provider case/Unicode semantics. Decide nil-UUID policy explicitly; possession of any valid identifier still grants no authority.

**Evidence correction:** the previous comment says the compile-fail doctest passed, but the reviewed CI uses `cargo test --workspace --all-targets`, which does not run doctests. Add an explicit documentation-test job and link its actual result before repeating that claim.

**Acceptance cases:** swapped ID types fail compilation; malformed identifiers/currencies/timestamps/size values fail; i128 boundary money survives `Envelope<Money>` and `GoalContract`; account-qualified identical resource strings remain distinct; serialization is canonical where used for hashes. Property tests should be genuinely generated rather than only example tests.

Details and proposed regression cases are on #786. No fixes or state changes were made in this review.

#### Comment 5706554829 — Jordan-Hall — 2026-09-17T00:28:23Z

Source: https://github.com/Jordan-Hall/browser/issues/122#issuecomment-5706554829 | Updated: 2026-09-17T00:28:23Z

###### Review follow-up — CORE-01.T02

Parent: #2. Updated value-boundary recommendations: https://github.com/Jordan-Hall/browser/pull/786#issuecomment-5706419337 .

Do not fix the money invariant by banning all negative `Money`: signed adjustments/refunds remain valid. Use a separate checked executable budget/spend type that requires nonnegative value and resolved currency scale. Test `Unknown` explicitly—the existing test with “unknown_or_known” in its name exercises only a known scale.

Acceptance should include the same extreme amount through construction → direct JSON → `Envelope<Money>` → checked persistence conversion, with no floating-point fallback. Non-empty provider identifiers need constructor and deserialize checks, but opaque provider values must not be silently trimmed or case-normalized.

These are additions to the existing review, not a passing test report. Link actual doctest and boundary-test evidence before accepting the task.


---

<a id="issue-123"></a>
## #123 — [TASK][CORE-01.T03] Specify durable core record schemas

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/123
**Created:** 2026-09-15T15:12:29Z | **Updated:** 2026-09-17T00:28:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #2
**PRs mentioning this issue:** [#787](https://github.com/Jordan-Hall/browser/pull/787), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #2

Task ID: `CORE-01.T03`

#### Proposed implementation
Write v1 records for GoalContract, Workspace, Task, Capability, Observation, Evidence, ActionProposal, Approval, Operation, Receipt, ViewDefinition and MemoryRecord. Keep provider sessions secondary and source observations distinct from model-derived facts.

#### Done when
Schemas round-trip fixtures with missing optional fields, distinct account scopes and unresolved external outcomes.

#### Guardrail
Use explicit unknown/unsupported states; reject records that cannot preserve authority-relevant meaning.

#### Prerequisites
`CORE-01.T02`

Review: pending · Implementation: not started.

### Discussion (3 comments)

#### Comment 5690207575 — Jordan-Hall — 2026-09-16T00:31:23Z

Source: https://github.com/Jordan-Hall/browser/issues/123#issuecomment-5690207575 | Updated: 2026-09-16T00:31:23Z

Implemented in stacked PR #787. CI run 35040305377 is green on Rust 1.98.1: formatting, Clippy, workspace tests, doctests, and architecture checks all pass. The durable v1 contracts reject unsupported record versions, preserve account scope, keep model provenance secondary, and represent uncertain external outcomes as `NeedsReconciliation`. This verified head is the base for `CORE-01.T04`.

#### Comment 5705939182 — Jordan-Hall — 2026-09-16T23:19:13Z

Source: https://github.com/Jordan-Hall/browser/issues/123#issuecomment-5705939182 | Updated: 2026-09-16T23:19:13Z

<!-- intent-core-review:2026-09-16:issue-123 -->
###### Review — CORE-01.T03

Parent: #2. Reviewed PR: #787, head `7c9d1e441b55216d3f5f0249d5605fd1b8dd0bf9`.

**P1 acceptance blocker:** `ActionProposal::new` and derived deserialization allow the canonical artifact's content hash and `arguments_hash` to disagree. The existing review finding remains present. Enforce this invariant through validated construction and equivalent decoding; dispatch must use the exact artifact/account/capability authorized by the approval.

**Schema improvements:** specify which records reject unknown fields and which retain explicit extensions; reject unsupported authority semantics rather than silently dropping them. Add per-family bounded collection/time/state invariants and canonical fixtures for every listed record, not only Workspace/Capability/Operation. Keep Source versus ModelDerived provenance explicit and provider sessions secondary.

Coordinate the wire OperationState with the different durable state machine introduced in #794, including original versus compensation attempts, accepted versus independently verified outcomes, and required receipt/evidence references. Align artifact media-type bounds with #797.

**Acceptance evidence:** valid and invalid complete proposal/approval fixtures; mismatched hashes/account bindings rejected; every record round-trips; newer-schema handling matches #126; unresolved writes retain uncertainty. The earlier comment's doctest claim is not established by the reviewed `--all-targets` workflow; require an explicit doctest result.

Full findings are on #787. This review does not mark the task fixed, production-ready or merged.

#### Comment 5706556796 — Jordan-Hall — 2026-09-17T00:28:38Z

Source: https://github.com/Jordan-Hall/browser/issues/123#issuecomment-5706556796 | Updated: 2026-09-17T00:28:38Z

###### Review follow-up — CORE-01.T03

Parent: #2. Detailed binding review: https://github.com/Jordan-Hall/browser/pull/787#issuecomment-5706421555 .

Make the action-binding fixture shared with #131, not just a record serialization fixture. Acceptance must reject both mismatched argument bytes and **identical bytes rebound to a different account/capability/target**. Include source preconditions, expiry and canonicalization version in the authorization contract; specify when a trusted transformation changes transport bytes.

Separate an imported proposal/approval DTO from validated records and the broker's executable authorization. Deserializing an `Approved` enum is not proof of user consent. Removing redundant digest constructor arguments is preferable to trusting callers to keep them synchronized.

For task closure, provide constructor/import negative cases and a proposal → approval → outbox integration test showing rejection leaves no pending dispatch or journal residue. Preserve distinct Accepted, Verified, Uncertain and compensation outcomes across #130/#145. Review only; code and states are unchanged.


---

<a id="issue-124"></a>
## #124 — [TASK][CORE-01.T04] Build bounded wire framing and errors

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/124
**Created:** 2026-09-15T15:12:36Z | **Updated:** 2026-09-17T00:29:26Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #2
**PRs mentioning this issue:** [#788](https://github.com/Jordan-Hall/browser/pull/788), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #2

Task ID: `CORE-01.T04`

#### Proposed implementation
Add request/response/event envelopes, length-prefix decoding, request correlation, deadline propagation and stable error codes. Validate lengths before allocation, cap nesting/collections and reserve a control lane separate from large artifacts.

#### Done when
Fragmented, oversized, malformed and deeply nested payload fixtures remain memory-bounded and return stable errors.

#### Guardrail
Never parse a partially received action as executable.

#### Prerequisites
`CORE-01.T03`

Review: pending · Implementation: not started.

### Discussion (4 comments)

#### Comment 5690348214 — Jordan-Hall — 2026-09-16T00:49:58Z

Source: https://github.com/Jordan-Hall/browser/issues/124#issuecomment-5690348214 | Updated: 2026-09-16T00:49:58Z

Implementation evidence for `CORE-01.T04`: PR #788 (`core-01-t04-wire-framing`) is green on CI run 35041541874. Rust 1.98.1 formatting, Clippy with warnings denied, full workspace tests, and architecture-boundary checks all pass. The implementation includes bounded length-prefixed control/artifact framing, pre-allocation size validation, incremental complete-frame gating, typed envelopes with trace/request/cancellation/deadline metadata, stable wire error codes, and JSON depth/collection/node limits. Implementation status: implemented in stacked PR; pending review/merge.

#### Comment 5694607883 — Jordan-Hall — 2026-09-16T08:40:54Z

Source: https://github.com/Jordan-Hall/browser/issues/124#issuecomment-5694607883 | Updated: 2026-09-16T08:40:54Z

Implementation complete in stacked PR #788 (`CORE-01.T04`). Current head `f76de003464148e179fd908aced770956a7d1352` passes CI: rustfmt, Clippy with warnings denied, tests, and architecture checks. This task is now the verified base for `CORE-01.T05`.

#### Comment 5705944376 — Jordan-Hall — 2026-09-16T23:19:36Z

Source: https://github.com/Jordan-Hall/browser/issues/124#issuecomment-5705944376 | Updated: 2026-09-16T23:19:36Z

<!-- intent-core-review:2026-09-16:issue-124 -->
###### Review — CORE-01.T04

Parent: #2. Reviewed implementation: #788 at `f76de003464148e179fd908aced770956a7d1352`. The existing complete/green comments need to be read alongside these unresolved codec findings.

**Required work:** perform version/header validation before decoding an incompatible typed payload; stop encoding at a byte budget rather than allocating the full JSON Vec first; define numeric versus textual wire error representation and test the serialized form. Derived Serde does not use the enum's repr(u16) discriminants automatically.

Reject duplicate authority-bearing JSON keys before normalization into Value. Preserve wide integer money exactly. Define what happens when a feed contains a valid frame followed by a malformed one: either return an explicit partial batch/error or poison the connection; do not silently lose preceding frames and resume at an unknown offset.

**Acceptance additions:** invalid version with invalid-for-current payload; duplicate account/schema/cancellation fields; i128 money boundaries; valid+invalid+valid batches; truncated EOF; encoding a too-large payload with measured bounded allocation; many simultaneous connections under an aggregate byte budget. Per-frame limits alone are insufficient.

Keep complete-frame gating and length-before-reserve. However, a lane tag on the same blocked byte stream does not make stop traffic independent of a large artifact; #127 must prove the required separation with real transport backpressure.

Existing inline findings and repair guidance are recorded on #788. Review only: no code, status or thread resolution changed.

#### Comment 5706562757 — Jordan-Hall — 2026-09-17T00:29:26Z

Source: https://github.com/Jordan-Hall/browser/issues/124#issuecomment-5706562757 | Updated: 2026-09-17T00:29:26Z

###### Review follow-up — CORE-01.T04

Parent: #2. Additional codec invariant: https://github.com/Jordan-Hall/browser/pull/788#issuecomment-5706423641 .

Add an **encode/decode closure test under the same validated limits**: every supported envelope accepted by the encoder must be decodable without loss by its matching decoder. Currently encoding enforces bytes while decoding also enforces structural budgets, so small over-deep/over-entry payloads can be locally emitted and then rejected.

Test exact boundaries and one-over cases for bytes, depth, nodes and collections, including the envelope overhead and wide-integer payloads. Share validation policy between version-specific codecs, enforce encoding size while writing, and keep error/framing failures explicit. Do not resolve this by removing the defensive decode limits.

The existing review remains applicable; this task needs the linked regressions and real transport integration evidence, not just the older green smoke run. Review only.


---

<a id="issue-125"></a>
## #125 — [TASK][CORE-01.T05] Authenticate locally launched worker channels

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/125
**Created:** 2026-09-15T15:12:58Z | **Updated:** 2026-09-17T00:29:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #2
**PRs mentioning this issue:** [#789](https://github.com/Jordan-Hall/browser/pull/789), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #2

Task ID: `CORE-01.T05`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5690421546 — Jordan-Hall — 2026-09-16T00:59:49Z

Source: https://github.com/Jordan-Hall/browser/issues/125#issuecomment-5690421546 | Updated: 2026-09-16T00:59:49Z

Implementation evidence for `CORE-01.T05`: PR #789 (`core-01-t05-worker-auth`) is green on CI run 35042238653. Rust 1.98.1 formatting, Clippy with warnings denied, full workspace tests, and architecture checks all pass. The implementation binds worker channels to supervisor-issued 256-bit one-shot secrets, typed worker-instance IDs and roles, checks OS peer evidence, denies browser-worker policy messages, and verifies live Linux `SO_PEERCRED` evidence over an anonymous Unix socket pair. Same-user peer credentials remain documented defense in depth, not a sandbox. Implementation status: implemented in stacked PR; pending review/merge.

#### Comment 5705949481 — Jordan-Hall — 2026-09-16T23:20:01Z

Source: https://github.com/Jordan-Hall/browser/issues/125#issuecomment-5705949481 | Updated: 2026-09-16T23:20:01Z

<!-- intent-core-review:2026-09-16:issue-125 -->
###### Review — CORE-01.T05

Parent: #2. Reviewed PR: #789 at `45c4fef9dbb6e17bf3ec06d2458d0545f679d390`.

**P1 blocker:** the live socketpair test verifies credentials of its creator, not a newly launched worker. On Linux I independently reproduced an inherited socketpair with an actual subprocess: the peer PID remains the creator's, while the child reports a different PID. This confirms the existing review concern; it is not a Rust-crate test run.

Choose an explicit bootstrap model: post-spawn protected connection with observed peer identity, or inherited-channel possession plus a correctly specified corroboration mechanism. Bind the supervisor-owned launch record to role, instance, epoch and channel. Consume it atomically in a registry; cloning a launch record into two OneShotAuthenticators must not permit reuse.

**Hardening:** derive credential evidence inside the transport boundary, keep Synthetic fixtures out of production identity construction, avoid permissive AnyLocal defaults, expire bootstrap material and constrain message direction/target scope. PID/UID equality alone is not containment against hostile same-user software. Qualify Windows endpoint ACL/principal/server checks separately from a PID helper.

**Required tests:** real worker exec/handshake; wrong process/token/role; duplicate use; expiration; revoked launch; fresh epoch on restart; no authority before handshake; platform-specific tests for every advertised OS. Keep unavoidable unsafe FFI narrow, documented and independently tested rather than weakening the trusted crates' safe-Rust boundary.

Full findings are on #789. The prior green unit run does not satisfy the launched-worker authentication acceptance criterion.

#### Comment 5706565055 — Jordan-Hall — 2026-09-17T00:29:43Z

Source: https://github.com/Jordan-Hall/browser/issues/125#issuecomment-5706565055 | Updated: 2026-09-17T00:29:43Z

###### Review follow-up — CORE-01.T05

Parent: #2. Channel-bound authentication review: https://github.com/Jordan-Hall/browser/pull/789#issuecomment-5706425677 .

Make the accepted result an authenticated connection bound to a consumed launch record and worker generation, not caller-supplied/deserialized peer evidence. Add a direction/instance check for lifecycle messages: a worker's own heartbeat is not authority to launch or stop another process.

The positive acceptance test must launch a real child and successfully complete the selected transport handshake; pair it with wrong-process/role, repeated launch-token, expired bootstrap and previous-generation failures. A same-process socketpair credential test or synthetic peer fixture alone cannot qualify the launch model.

Expose platform-specific support honestly and keep low-level platform wrappers isolated. Keep earlier unresolved findings open until the corrected source and subprocess evidence are linked; this review makes no code or status changes.


---

<a id="issue-126"></a>
## #126 — [TASK][CORE-01.T06] Implement version negotiation and schema evolution

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/126
**Created:** 2026-09-15T15:13:02Z | **Updated:** 2026-09-17T00:29:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #2
**PRs mentioning this issue:** [#790](https://github.com/Jordan-Hall/browser/pull/790), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #2

Task ID: `CORE-01.T06`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5690479124 — Jordan-Hall — 2026-09-16T01:07:42Z

Source: https://github.com/Jordan-Hall/browser/issues/126#issuecomment-5690479124 | Updated: 2026-09-16T01:07:42Z

Implementation evidence for `CORE-01.T06`: PR #790 (`core-01-t06-version-negotiation`) is green on CI run 35042801958. Rust 1.98.1 formatting, Clippy with warnings denied, full workspace tests, and architecture checks all pass. Live IPC negotiation is bounded and selects an exact mutually supported version/capability intersection; negotiated sessions reject version mismatch. Durable migrations are separate, explicit, bounded, preserve source version, reject downgrade/ambiguous paths, and prevent older writers from rewriting newer schema documents. Authority-affecting protocol changes require a major version bump. Implementation status: implemented in stacked PR; pending review/merge.

#### Comment 5705954775 — Jordan-Hall — 2026-09-16T23:20:28Z

Source: https://github.com/Jordan-Hall/browser/issues/126#issuecomment-5705954775 | Updated: 2026-09-16T23:20:28Z

<!-- intent-core-review:2026-09-16:issue-126 -->
###### Review — CORE-01.T06

Parent: #2. Reviewed implementation: #790 at `9134df9db63be066b549de30b178c1efb494f841`.

**Integration blocker:** negotiation can select 1.1/2.0, but the actual Envelope codec still emits and accepts only 1.0. The previous comment's “live IPC” description is stronger than the demonstrated helper tests. Bind negotiation to an authenticated session and its real encoder/decoder, or advertise only the implemented version.

**Bounds:** validate offer input while reading, not after collecting arbitrary iterators/Vecs; cap duplicate input entries as well as unique capabilities. Bound migration input/output bytes, and validate each callback's output against the declared record family/target schema before returning success.

**Forward compatibility:** `ReadOnlyNewerMinor` needs a real opaque/preserving read path; the existing record deserializers reject non-1.0 schemas. An assessment enum alone does not prevent an older writer from dropping unknown fields. Keep live protocol negotiation separate from durable migration semantics and never reinterpret capability negotiation as permission.

**Acceptance evidence:** for every advertised version, offer → handshake → actual wire round-trip; unsupported semantic version rejection before payload interpretation; complete record migrations with invalid-output tests; newer-document preservation and write refusal; bounded repeated-capability offers. Coordinate with #124 and #123 before freezing the schema baseline.

Full findings are on #790. Review only; implementation remains subject to these acceptance corrections.

#### Comment 5706566562 — Jordan-Hall — 2026-09-17T00:29:54Z

Source: https://github.com/Jordan-Hall/browser/issues/126#issuecomment-5706566562 | Updated: 2026-09-17T00:29:54Z

###### Review follow-up — CORE-01.T06

Parent: #2. Additional migration/session checks: https://github.com/Jordan-Hall/browser/pull/790#issuecomment-5706433095 .

Do not advertise a negotiated version unless an authenticated session can encode and decode it. Test the actual 1.1 negotiation → message exchange path against the current V1-only codec, not only range intersection.

The migration no-op path needs a documented trust contract: equal source/target versions currently return supplied bytes without family validation. Either identify the output as unvalidated bytes, or validate that path as rigorously as a transformed document. Invalid input must not gain a valid-document label because no version change was needed.

Acceptance adds malformed equal-version input, a wrong-family result, an expanding migration and preserved newer-minor read-only data. Retain original bytes on failure and keep schema compatibility separate from authorization. Review only; no fixes are claimed.


---

<a id="issue-127"></a>
## #127 — [TASK][CORE-01.T07] Wire cancellation, deadlines and backpressure

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/127
**Created:** 2026-09-15T15:13:09Z | **Updated:** 2026-09-17T00:30:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #2
**PRs mentioning this issue:** [#791](https://github.com/Jordan-Hall/browser/pull/791), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #2

Task ID: `CORE-01.T07`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5694674723 — Jordan-Hall — 2026-09-16T08:46:27Z

Source: https://github.com/Jordan-Hall/browser/issues/127#issuecomment-5694674723 | Updated: 2026-09-16T08:46:27Z

Implementation complete in stacked PR #791 (`CORE-01.T07`). Current head `d1371e2064956cf2b4e3441281119701d3f31861` passes rustfmt, Clippy with warnings denied, unit/integration tests, and architecture checks. The two-worker fixture verifies cancellation acknowledgements preempt saturated progress queues.

#### Comment 5705959253 — Jordan-Hall — 2026-09-16T23:20:52Z

Source: https://github.com/Jordan-Hall/browser/issues/127#issuecomment-5705959253 | Updated: 2026-09-16T23:20:52Z

<!-- intent-core-review:2026-09-16:issue-127 -->
###### Review — CORE-01.T07

Parent: #2. Reviewed PR: #791 at `d1371e2064956cf2b4e3441281119701d3f31861`.

**Acceptance correction:** the existing “two-worker fixture” uses two in-memory StreamEndpoints, not two processes or transport boundaries. It proves queue ordering, not cancellation during blocked IO. Keep it as a unit test and add the actual two-worker test required by the parent.

**Unresolved findings:** public QueueLimits bypass constructor validation; StreamEndpoint cannot retire completed cancellation records and eventually exhausts its registry; progress_backlog includes artifact entries. Fix the bounded configuration and lifecycle APIs without allowing a retired/unknown ID to regain execution authority.

**Integration required:** associate queued work with task/epoch/deadline, revalidate current authority before dispatch, and discard or explicitly fail stale work. A cancellation flag plus an ACK does not revoke an external worker or undo an already-sent action. Use byte as well as count budgets, deduplicate cancel storms, and define behavior when even reserved control capacity is full.

**Tests:** real spawned workers with bulk streams blocked; stop deadline measurement; queued work after expiry/revocation; repeated sequential tasks beyond registry capacity; duplicate cancel/ACK pressure; stale-epoch output after restart; external in-flight effects remain reconciliable. Include sleep/wall-clock changes in deadline semantics through CORE-03/04.

Details are on #791. Current green helper tests do not establish this task's full acceptance criteria; no implementation or status changes were made by this review.

#### Comment 5706568222 — Jordan-Hall — 2026-09-17T00:30:06Z

Source: https://github.com/Jordan-Hall/browser/issues/127#issuecomment-5706568222 | Updated: 2026-09-17T00:30:06Z

###### Review follow-up — CORE-01.T07

Parent: #2. Cancellation lifetime and transport acceptance: https://github.com/Jordan-Hall/browser/pull/791#issuecomment-5706445404 .

When implementing retirement, require positive current-generation activity for dispatch. `!is_cancelled(id)` is insufficient because an unknown/removed ID returns false, and removing then re-registering an ID makes it Active again.

Add one end-to-end lifecycle case: saturate both progress and reserved queues → accept cancellation → acknowledgement enqueue fails but revocation remains applied → drain/retry acknowledgement → retire completed work → reject late old-generation messages → accept a fresh generation. Repeat beyond registry capacity without unbounded tombstones.

Required result/artifact delivery must not be accidentally treated as disposable progress. Preserve ownership on enqueue errors and test actual subprocess backpressure. Coordinate this with lease-first cancellation #141 rather than inventing a second definition of stop completion. Review only.


---

<a id="issue-128"></a>
## #128 — [TASK][CORE-01.T08] Publish contract conformance and fuzz targets

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/128
**Created:** 2026-09-15T15:13:13Z | **Updated:** 2026-09-17T00:30:26Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #2
**PRs mentioning this issue:** [#792](https://github.com/Jordan-Hall/browser/pull/792), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #2

Task ID: `CORE-01.T08`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5694768646 — Jordan-Hall — 2026-09-16T08:54:26Z

Source: https://github.com/Jordan-Hall/browser/issues/128#issuecomment-5694768646 | Updated: 2026-09-16T08:54:26Z

Implementation complete in stacked PR #792 (`CORE-01.T08`). Head `f04d5d5b799823c3d5ad28d8f1412b657d1b0269` passes rustfmt, Clippy, all workspace tests, architecture checks, and the executable core conformance suite; CI also uploads the machine-readable conformance report. Fuzz entrypoints are published under the excluded `fuzz/` package.

#### Comment 5705966032 — Jordan-Hall — 2026-09-16T23:21:28Z

Source: https://github.com/Jordan-Hall/browser/issues/128#issuecomment-5705966032 | Updated: 2026-09-16T23:21:28Z

<!-- intent-core-review:2026-09-16:issue-128 -->
###### Review — CORE-01.T08

Parent: #2. Reviewed PR: #792 at `f04d5d5b799823c3d5ad28d8f1412b657d1b0269`.

**P2 blocker:** the published control-envelope fuzz target imports `serde_json::Value` without declaring `serde_json` in the excluded fuzz package. The existing inline finding remains. Exclusion from production dependencies is correct; omission from build validation is not. Add a separate pinned fuzz-build/smoke gate.

**Coverage correction:** the current conformance binary is a useful smoke suite. Its synthetic authentication, byte-appending migration, one offer fixture and incomplete-approval rejection do not qualify real worker authentication, all durable schemas, approval bindings, negotiated codecs or process cancellation. It remains green in the presence of the concrete defects recorded on #787–#791.

**Required improvement:** map every CORE-01 acceptance invariant to an executable check ID and fixture; add complete valid/invalid records, actual process-boundary tests and regressions for this review. Emit failure reports as well as success reports, preserve non-zero exit status, and upload available evidence even on failure.

**Completion evidence:** both fuzz targets compile and execute within stated budgets; failures are minimized into deterministic regressions; doctests run; report includes commit, target, compiler, dependency-lock/corpus identity and actual results. Golden changes require review rather than merely replacing expected strings.

Full review is on #792. The prior green smoke report is not sufficient evidence to close CORE-01 or this qualification task.

#### Comment 5706570897 — Jordan-Hall — 2026-09-17T00:30:26Z

Source: https://github.com/Jordan-Hall/browser/issues/128#issuecomment-5706570897 | Updated: 2026-09-17T00:30:26Z

###### Review follow-up — CORE-01.T08

Parent: #2. Gate design and evidence schema: https://github.com/Jordan-Hall/browser/pull/792#issuecomment-5706450338 .

Require a machine-checkable mapping from each claimed contract invariant to a test/fixture, supported target, result and exact tested revision. Missing, skipped and unsupported cases must not become passes through an empty check list. Keep unit, subprocess and platform evidence distinct.

Test the gate itself: intentionally omit a required case, permit a mismatched action hash, bypass queue limits and force a check failure. Each should produce a non-zero result plus a retained structured failure report. A report emitted only on success hides the evidence needed to diagnose failures.

The excluded fuzz package still needs its own build/smoke job and declared direct dependencies. The six current smoke checks remain useful but cannot certify the unresolved properties on #123–#127. No test execution or fix is claimed by this comment.


---

<a id="issue-129"></a>
## #129 — [TASK][CORE-02.T01] Establish database ownership and migrations

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/129
**Created:** 2026-09-15T15:13:23Z | **Updated:** 2026-09-17T00:30:48Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #3
**PRs mentioning this issue:** [#793](https://github.com/Jordan-Hall/browser/pull/793), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #3

Task ID: `CORE-02.T01`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5694849050 — Jordan-Hall — 2026-09-16T09:01:04Z

Source: https://github.com/Jordan-Hall/browser/issues/129#issuecomment-5694849050 | Updated: 2026-09-16T09:01:04Z

Implementation complete in stacked PR #793 (`CORE-02.T01`). Head `c468a457da543806ae2558a081d98d8ee5a2f9cd` passes rustfmt, Clippy, all tests, architecture checks and core conformance. File stores enforce WAL/FULL durability, checksummed migrations, application identity, foreign keys and integrity checks.

#### Comment 5705996369 — Jordan-Hall — 2026-09-16T23:24:22Z

Source: https://github.com/Jordan-Hall/browser/issues/129#issuecomment-5705996369 | Updated: 2026-09-16T23:24:22Z

<!-- intent-core-review:2026-09-16:issue-129 -->
###### Review — CORE-02.T01

Parent: #3. Reviewed implementation: #793 at `c468a457da543806ae2558a081d98d8ee5a2f9cd`. The earlier green-CI comment does not establish the following ownership and migration invariants.

**P1:** a populated unrelated database with the default application_id of zero is adopted and modified. Check the existing file/schema identity before persistent configuration changes; initialize only an explicitly new/empty store. Refusal must leave the unrelated database unchanged.

**P2:** migration selection occurs before the writer lock, so simultaneous startup can select stale migrations. Validate/recheck the ledger under the appropriate lock. Also compare user_version with the validated ledger maximum; a valid checksum ledger currently does not detect an independently changed user_version.

**Cross-task prerequisite:** a private Connection field does not enforce one owner per profile. Multiple StateStore::open calls are possible, while artifact ingestion and GC operate outside database transactions. Establish a lifetime owner guard tied to the canonical provisioned profile, or specify an equally strong coordination protocol before #133/#134 rely on single-writer behavior.

**Acceptance additions:** unchanged foreign zero-ID database on refusal; simultaneous initialization; altered user_version above/below the ledger; interrupted migration rollback; duplicate owner startup and crash release; separate foreign-key and domain-integrity checks. Keep blocking database contention off the stop/UI path and publish the tested filesystem/durability matrix.

All original inline findings and detailed improvements are recorded on #793. Review only: no code, status or thread resolution changed.

#### Comment 5706573724 — Jordan-Hall — 2026-09-17T00:30:47Z

Source: https://github.com/Jordan-Hall/browser/issues/129#issuecomment-5706573724 | Updated: 2026-09-17T00:30:47Z

###### Review follow-up — CORE-02.T01

Parent: #3. New existing-store identity case and related fixes: https://github.com/Jordan-Hall/browser/pull/793#issuecomment-5706493426 .

Separate new-store bootstrap from opening an existing store. The current metadata initialization can silently generate a fresh UUID when an existing store's metadata row is missing. Treat missing/malformed existing identity as recovery-required; do not automatically re-identify the profile.

Add missing-row, malformed-UUID, populated foreign zero-application-ID, inconsistent ledger/user_version, and two-process startup fixtures. Verify rejected opens do not claim or mutate an unrelated database.

A private Rust connection is not an exclusive profile owner. Establish that ownership contract before relying on it for artifact publication, GC and backup coordination. Link the fixing commit and actual process/migration evidence; the older green CI result does not discharge these newly reviewed invariants. Review only.


---

<a id="issue-130"></a>
## #130 — [TASK][CORE-02.T02] Model durable operation and journal records

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/130
**Created:** 2026-09-15T15:13:28Z | **Updated:** 2026-09-17T00:31:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #3
**PRs mentioning this issue:** [#794](https://github.com/Jordan-Hall/browser/pull/794), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #3

Task ID: `CORE-02.T02`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5695095474 — Jordan-Hall — 2026-09-16T09:18:43Z

Source: https://github.com/Jordan-Hall/browser/issues/130#issuecomment-5695095474 | Updated: 2026-09-16T09:18:43Z

Implemented in PR #794 (`CORE-02.T02`). Durable operation identity/state, optimistic revision checks, append-only journal entries, and snapshot+journal atomicity are implemented. Final CI on head `a01286c2` is green: rustfmt, Clippy with warnings denied, workspace tests, architecture checks, and core conformance all pass.

#### Comment 5705999706 — Jordan-Hall — 2026-09-16T23:24:45Z

Source: https://github.com/Jordan-Hall/browser/issues/130#issuecomment-5705999706 | Updated: 2026-09-16T23:24:45Z

<!-- intent-core-review:2026-09-16:issue-130 -->
###### Review — CORE-02.T02

Parent: #3. Reviewed PR: #794 at `a01286c2acf45ec7a905eb3ec8de6ee5f3ea555f`.

**P1 blockers:** an allowed transition can replace the active attempt ID, for example Attempting(A) → Accepted(B), without a dispatch for B. Enforce attempt continuity and model any new compensation attempt explicitly. Also preserve reconciliation origin: Compensating → NeedsReconciliation currently cannot reach a correctly classified Compensated outcome after confirmation.

The parent requires immutable source preconditions/revision, but source_schema_major/minor identifies a schema, not the cart/quote/document revision against which the user authorized action. Add a distinct typed precondition/evidence binding and verify it at dispatch.

**Integration:** consolidate general operation, journal and outbox changes behind coherent transactional commands. A cancellation must not leave an endlessly claimable pending outbox. Define the checked projection into the different wire OperationState and require independent evidence/receipt identity for Verified, rather than letting an enum transition manufacture success.

**Tests before acceptance:** exhaustive state/attempt matrix; rejection of substituted attempt IDs; compensation timeout followed by read-only reconciliation; stale source precondition; Prepared abandonment; racing revisions; journal/projection consistency; timestamp ordering and bounded journal pagination. Preserve the existing atomic snapshot+journal commit.

The previous green CI is evidence for its existing test suite, not resolution of these findings. Detailed source findings and original thread IDs are on #794; this review does not mark them fixed.

#### Comment 5706575315 — Jordan-Hall — 2026-09-17T00:31:00Z

Source: https://github.com/Jordan-Hall/browser/issues/130#issuecomment-5706575315 | Updated: 2026-09-17T00:31:00Z

###### Review follow-up — CORE-02.T02

Parent: #3. Additional transaction-outcome review: https://github.com/Jordan-Hall/browser/pull/794#issuecomment-5706500421 .

The current mutation commits and then reloads outside its transaction. Add a fixture for readback failure after a successful commit, and another where a second writer advances the record before readback. The returned result must not imply rollback or falsely describe another writer's revision as this command's result.

Prefer validating/building the result snapshot inside the transaction, then returning it after commit. Otherwise provide an explicit committed-but-readback-unavailable result and lookup by operation/revision. Keep ambiguous commit errors distinct from confirmed rollback.

This complements—not replaces—the open attempt-continuity, compensation-origin, source-precondition and Prepared-cancellation findings. Require journal/projection agreement across all transitions and make retries incapable of duplicating a committed event. Coordinate result semantics with #131 and recovery #145–#148. Review only.


---

<a id="issue-131"></a>
## #131 — [TASK][CORE-02.T03] Implement transactional outbox dispatch

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/131
**Created:** 2026-09-15T15:13:34Z | **Updated:** 2026-09-17T00:31:16Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #3
**PRs mentioning this issue:** [#795](https://github.com/Jordan-Hall/browser/pull/795), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#809](https://github.com/Jordan-Hall/browser/pull/809), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #3

Task ID: `CORE-02.T03`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5695211646 — Jordan-Hall — 2026-09-16T09:28:10Z

Source: https://github.com/Jordan-Hall/browser/issues/131#issuecomment-5695211646 | Updated: 2026-09-16T09:28:10Z

Implemented in PR #795 (`CORE-02.T03`). Transactional outbox staging, bounded leases, durable pre-dispatch attempt recording, payload integrity checks, and accepted/ambiguous/rejected outcome handling are implemented. CI on head `3c7c8480` is green across rustfmt, Clippy, workspace tests, architecture checks, and core conformance.

#### Comment 5706003190 — Jordan-Hall — 2026-09-16T23:25:09Z

Source: https://github.com/Jordan-Hall/browser/issues/131#issuecomment-5706003190 | Updated: 2026-09-16T23:25:09Z

<!-- intent-core-review:2026-09-16:issue-131 -->
###### Review — CORE-02.T03

Parent: #3. Reviewed PR: #795 at `3c7c848014e9ce9c3b234be72ef1d19ea597d15e`.

**P1 blockers:** claimed messages expose dispatch bytes before durable start; a crash after begin_dispatch leaves an attempting row without a recovery enumeration/path; staging hashes arbitrary new payloads but does not bind their bytes/destination to the approved operation's arguments, account and capability. Storage-integrity hashing is not proof of authorization.

Return opaque claim records and make the real dispatcher require a validated started-attempt token. Bind transport material through an explicit canonical-action descriptor or trusted versioned transformation. Persist enough worker epoch/attempt/recovery information to find abandoned sends and classify them as uncertain rather than resend them.

**Correctness improvements:** use claim generations/epochs instead of a reusable owner string; require the active attempt identity when recording results; retire cancelled pending outboxes atomically so they cannot starve subsequent work. Align planned versus active attempt identity in the snapshot and journal.

**Acceptance tests:** changed payload/destination/account rejected; stale lease holder cannot start or report; cancellation with batch size one does not starve work; process death immediately before/after send; lost response becomes NeedsReconciliation; restart discovers every unfinished attempt; duplicate deliveries do not duplicate the fixture effect. Bound total claim bytes, not only message count, and revalidate current policy at dispatch.

The original three inline findings are still applicable; detailed source review is on #795. No fixes, production sends, or issue state changes were performed in this review.

#### Comment 5706577331 — Jordan-Hall — 2026-09-17T00:31:16Z

Source: https://github.com/Jordan-Hall/browser/issues/131#issuecomment-5706577331 | Updated: 2026-09-17T00:31:16Z

###### Review follow-up — CORE-02.T03

Parent: #3. Result-recording and acknowledgement review: https://github.com/Jordan-Hall/browser/pull/795#issuecomment-5706529974 .

Add an idempotent **result observation** contract separate from action dispatch. A repeated identical acceptance after a lost local acknowledgement should be recognized as already recorded; a conflicting result or stale generation must be rejected or explicitly reconciled. Neither case authorizes repeating the external action.

The acceptance fixture should cover claim expiry/replacement, durable attempt creation, one provider fixture effect, acceptance commit, lost acknowledgement, duplicate result and restart. Retain exact attempt/generation and provider evidence. Preserve uncertainty for the after-send/before-record crash.

The earlier payload-exposure, missing abandoned-attempt recovery, cancelled-row starvation and approval-binding findings remain blockers. Claims must not expose executable dispatch material prematurely; a matching owner string is not an epoch. Cross-link completed fixes to #130/#141/#147 rather than layering a separate retry policy over broken primitives. Review only.


---

<a id="issue-132"></a>
## #132 — [TASK][CORE-02.T04] Add inbox deduplication and consumer cursors

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/132
**Created:** 2026-09-15T15:13:42Z | **Updated:** 2026-09-17T00:31:31Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #3
**PRs mentioning this issue:** [#796](https://github.com/Jordan-Hall/browser/pull/796), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #3

Task ID: `CORE-02.T04`

Review: addressed in PR #796
Implementation: implemented in PR #796; pending merge

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5695713019 — Jordan-Hall — 2026-09-16T10:06:37Z

Source: https://github.com/Jordan-Hall/browser/issues/132#issuecomment-5695713019 | Updated: 2026-09-16T10:06:37Z

Implementation is in stacked PR #796 (`core-02-t04-inbox`). CI is green on head `1fc177f601c62acf6fa4fddffa23b902034663b2`. Codex review finding about truncated materialized effect sets was addressed and replied to inline; reads now verify the complete aggregate effects hash and include a missing-row regression test. Parent: #3.

#### Comment 5706006163 — Jordan-Hall — 2026-09-16T23:25:29Z

Source: https://github.com/Jordan-Hall/browser/issues/132#issuecomment-5706006163 | Updated: 2026-09-16T23:25:29Z

<!-- intent-core-review:2026-09-16:issue-132 -->
###### Review — CORE-02.T04

Parent: #3. Reviewed PR: #796 at `1fc177f601c62acf6fa4fddffa23b902034663b2`.

The previous aggregate-effect read fix is present: missing rows are detected by recomputing the complete effect-set hash, with a regression and explicit reply. Preserve that fix; it is not being reported as still absent.

**Remaining improvements:** the duplicate-apply path returns Duplicate without checking that the stored effect rows still match the saved aggregate. After corruption, redelivery can acknowledge an event as applied while the separate read fails. Verify before such acknowledgement or surface a repair-required result; repair only local projections, never external effects.

A call can contain 1,024 effects of 1 MiB each. Add an aggregate byte/work budget before hashing or opening the transaction, plus bounded materialization reads. Use artifact references for large values.

**Requirement clarification:** separate delivery cursor from provider/entity revision and define the initial watermark. The single contiguous sequence currently does not implement the parent's explicit requirement that newer out-of-order source revisions remain processable. Bind source identity to the connector/account, and version the consumer projection so intentional upgrades have a controlled rebuild path.

**Acceptance additions:** corrupted effect followed by identical redelivery; legitimate zero-effect versus absent state; aggregate oversize with individually valid effects; out-of-order source revisions; identical event IDs from distinct accounts; rollback between effects, dedup record and cursor writes; versioned projector rebuild.

Full review is posted on #796. Existing task text saying review addressed applies to the earlier finding, not automatic acceptance of these additional review points. No code or state was changed.

#### Comment 5706579274 — Jordan-Hall — 2026-09-17T00:31:31Z

Source: https://github.com/Jordan-Hall/browser/issues/132#issuecomment-5706579274 | Updated: 2026-09-17T00:31:31Z

###### Review follow-up — CORE-02.T04

Parent: #3. Cursor/bootstrap fixture and acknowledgement semantics: https://github.com/Jordan-Hall/browser/pull/796#issuecomment-5706535373 .

The aggregate-read repair is present; retain its test. Tighten the remaining contract with explicit stream generation, starting delivery watermark and projector version. Ordered delivery, importing a snapshot and rebuilding a new projector version should not share an ambiguous bootstrap path.

Test watermark 40 → receive 42 (no apply) → 41 → 42 → duplicate 41; then a newer entity revision on a later delivery index and an identical event ID from another account. Delivery positions and provider/entity revisions are separate values.

A `Duplicate` acknowledgement must state whether healthy materialization was verified. Missing rows must fail or enter a local-only repair path, never replay external actions. Add aggregate byte limits and a legitimate zero-effect case. Earlier “review addressed” status covers the previous read-path fix, not these remaining acceptance gaps. No code or status changed.


---

<a id="issue-133"></a>
## #133 — [TASK][CORE-02.T05] Implement scoped immutable artifact storage

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/133
**Created:** 2026-09-15T15:13:46Z | **Updated:** 2026-09-17T00:31:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #3
**PRs mentioning this issue:** [#797](https://github.com/Jordan-Hall/browser/pull/797), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #3

Task ID: `CORE-02.T05`

Review: addressed in PR #797
Implementation: implemented in PR #797; pending merge

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5695920634 — Jordan-Hall — 2026-09-16T10:22:23Z

Source: https://github.com/Jordan-Hall/browser/issues/133#issuecomment-5695920634 | Updated: 2026-09-16T10:22:23Z

Implementation is in stacked PR #797 (`core-02-t05-artifacts`). CI is green on head `07936f3eecbba10d608b520427ab13360cf79eee`. Three Codex P1 findings were fixed and replied to inline: retry-byte identity verification, scope-qualified reference counts, and durable parent-directory fsync before metadata commit. Parent: #3.

#### Comment 5706010320 — Jordan-Hall — 2026-09-16T23:25:54Z

Source: https://github.com/Jordan-Hall/browser/issues/133#issuecomment-5706010320 | Updated: 2026-09-16T23:25:54Z

<!-- intent-core-review:2026-09-16:issue-133 -->
###### Review — CORE-02.T05

Parent: #3. Reviewed PR: #797 at `07936f3eecbba10d608b520427ab13360cf79eee`.

The three previous fixes are present: retry-byte identity, scope-qualified reference counting and synchronization of newly created directory ancestry. This review distinguishes them from the additional requirements below.

**Filesystem authority:** root is still caller-supplied on each operation, not bound once to the profile/store. Existing-directory checks and a separate symlink_metadata/open pair do not prevent parent substitution when another actor can modify that namespace. Provision a private root with verified permissions/ownership and use directory-relative, no-follow/beneath operations through safe platform interfaces. This risk is conditional on the storage namespace being writable/untrusted; no remote exploit is claimed.

**Durability:** non-Unix directory sync is a no-op returning success. Expose the supported durability profile or provide a tested backend; do not infer non-Linux guarantees from Ubuntu CI. Test repeated publication after a failed flush/link/sync.

**Crash cleanup:** publication before metadata can leave unregistered blobs; process death also bypasses Drop-based quarantine cleanup. Add bounded orphan reconciliation with active-ingestion fencing, grace periods and profile quotas. Coordinate with #134 so GC cannot delete a publication another live owner is registering.

**Acceptance additions:** wrong-root/profile rejection; ancestor/leaf path substitution; supported-OS flush failures; crash at each publication boundary; missing/corrupt/deduplicated blobs; bounded cancellable ingestion. Document that suppression stops new retrieval, not bytes already transferred through into_file. Broker-derived scope must come from current grants, not a worker-selected string.

Detailed findings are on #797. Existing fixes remain acknowledged; no implementation or issue state changed in this review.

#### Comment 5706581384 — Jordan-Hall — 2026-09-17T00:31:47Z

Source: https://github.com/Jordan-Hall/browser/issues/133#issuecomment-5706581384 | Updated: 2026-09-17T00:31:47Z

###### New review blocker — CORE-02.T05 reference registration

Parent: #3. New P1 finding and SQL reproduction: https://github.com/Jordan-Hall/browser/pull/797#issuecomment-5706481921 .

`register_artifact_reference` can return success without inserting a reference: `BoundedText` permits empty reference kind/ID, table CHECK constraints reject them, and broad `INSERT OR IGNORE` suppresses the error. The reduced SQLite reproduction confirmed zero inserted rows with no error. This is SQL-level evidence, not a locally executed Rust test.

Require non-empty validated identifiers, targeted primary-key conflict handling, and a distinction between a newly inserted pin and an identical existing pin. Test that every successful reference registration really protects its artifact from GC; pair this with retained receipt/evidence cases in #134 and checkpoint/backup pins in #135/#146.

The three earlier fixes in #797 remain present. This is a separate finding, so the issue's prior “review addressed” wording must not be read as all findings resolved. No code, issue state or existing thread resolution was changed.


---

<a id="issue-134"></a>
## #134 — [TASK][CORE-02.T06] Implement reference-safe retention and deletion

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/134
**Created:** 2026-09-15T15:13:51Z | **Updated:** 2026-09-17T00:32:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #3
**PRs mentioning this issue:** [#801](https://github.com/Jordan-Hall/browser/pull/801), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #3

Task ID: `CORE-02.T06`

Review: in progress
Implementation: in progress on branch `core-02-t06-retention`

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

Implementation PR: pending

Current scope: immediate retrieval suppression, scoped holds, resumable GC queue, reference-safe blob deletion.

### Discussion (2 comments)

#### Comment 5706013729 — Jordan-Hall — 2026-09-16T23:26:12Z

Source: https://github.com/Jordan-Hall/browser/issues/134#issuecomment-5706013729 | Updated: 2026-09-16T23:26:12Z

<!-- intent-core-review:2026-09-16:issue-134 -->
###### Review — CORE-02.T06

Parent: #3. The implementation PR is **#801**, reviewed at `fbfcfceb17540e5fe80bec555385e097f3787455`; the issue body's PR-pending text is stale. This review does not alter that body.

**Confirmed P1 blocker:** suppression updates an INSTEAD OF-triggered view, then expects one affected row. SQLite reports zero for the outer statement, so the method returns ConcurrentSuppression and rolls the transaction back. Latest inspected CI run `35086737317`, job `104763204222`, passes formatting/Clippy but fails all three retention tests with this error. An independent SQLite reproduction confirms the trigger/row-count behavior. Update the scoped base table under transaction or verify the logical result without relying on view changes counts; do not weaken the tests.

**Additional blockers:** selecting a limited sorted prefix before checking eligibility can indefinitely starve later eligible blobs. Use an eligibility-filtered/keyset scan and bounded retries. GC also unlinks before its second eligibility check; without a real profile-wide owner or deletion-generation exclusion, another writer can add a hold/live handle between the first check and unlink. Detecting that afterward cannot restore the file.

**Representation:** suppression is encoded as a fake negative byte_size in a view, not an explicit lifecycle result. Keep normal hidden data, administrative retention access and actual corruption distinct, with scoped tombstone semantics.

**Acceptance:** all three current tests green; retained-prefix scan beyond 64 records; live handles/references/holds; ingestion or hold addition racing GC; restart before/after unlink; failure backoff; missing-file reconciliation; active-reader and ID-reuse policy. Full findings and original review-thread references are on #801.

This task is not complete while suppression fails and the latest tested head is red. Review only; no fix or status change claimed.

#### Comment 5706584169 — Jordan-Hall — 2026-09-17T00:32:07Z

Source: https://github.com/Jordan-Hall/browser/issues/134#issuecomment-5706584169 | Updated: 2026-09-17T00:32:07Z

###### Review follow-up — CORE-02.T06 remains blocked

Parent: #3. Implementation PR is #801; the issue body's “PR: pending” is stale. Current reviewed head `fbfcfceb17540e5fe80bec555385e097f3787455` has three failing retention tests, not a completed retention implementation.

Additional review: https://github.com/Jordan-Hall/browser/pull/801#issuecomment-5706541001 . Besides the existing first-suppression rollback, starvation and GC/hold race findings, expiry pruning currently processes all expired holds before checking the batch limit; even limit zero mutates state. Bound this maintenance work separately.

The GC failure logger also truncates a String at byte 2048 without a UTF-8-boundary check. Add a Unicode error regression so recording a deletion failure cannot itself panic. Keep explicit known-committed versus rollback outcomes when suppression/reference/hold changes commit before enqueue.

Acceptance must also include #133's false-reference-success case: GC may only rely on pins proven to exist. Preserve failing tests, fix the underlying transactional/lifecycle behavior, and link rerun evidence. This comment does not implement or mark any finding fixed.


---

<a id="issue-135"></a>
## #135 — [TASK][CORE-02.T07] Build backup, restore and integrity checks

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/135
**Created:** 2026-09-15T15:13:59Z | **Updated:** 2026-09-17T00:32:48Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #3
**PRs mentioning this issue:** [#801](https://github.com/Jordan-Hall/browser/pull/801), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#803](https://github.com/Jordan-Hall/browser/pull/803), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#808](https://github.com/Jordan-Hall/browser/pull/808), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #3

Task ID: `CORE-02.T07`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706020836 — Jordan-Hall — 2026-09-16T23:26:51Z

Source: https://github.com/Jordan-Hall/browser/issues/135#issuecomment-5706020836 | Updated: 2026-09-16T23:26:51Z

<!-- intent-core-review:2026-09-16:issue-135 -->
###### Planning review — CORE-02.T07

Parent: #3. No implementation PR was found for this task in the reviewed CORE inventory. These are proposed acceptance refinements, not defects in code that exists.

**Snapshot contract:** take a consistent database snapshot using the database's supported snapshot/backup mechanism, then enumerate artifacts from that exact snapshot. Coordinate a backup lease/reference with #134 so GC cannot remove a required blob between enumeration and copying. Include retained evidence/receipts even when ordinary retrieval is suppressed; preserve their suppression flags.

**Manifest:** version the format and bind store/profile identity, snapshot schema, artifact identities/scopes/hashes/sizes and a complete inventory. Authenticate it against a trust key obtained independently of the bundle. A hash stored alongside attacker-editable content does not authenticate the backup. Bound manifest size/counts and reject duplicate identities, traversal paths and unsupported schemas. Scope any portable plaintext export separately from an encrypted backup.

**Restore:** stage into a new restricted directory; verify manifest, database, references and every required blob before publishing. Failure leaves the active profile untouched. Restore a durable dispatch-disabled recovery state and a new runtime incarnation; do not restore old process leases as current authority. Revalidate credentials, grants, deadlines, source freshness and deletion policy before resuming. An old snapshot does not undo an external transaction or justify another send.

**Required tests:** backup concurrent with writes/GC; missing/corrupt/extra/duplicated artifacts; wrong manifest key; interrupted copy/restore; unsupported migration; withheld credentials; old suppressed data stays suppressed; stale attempting writes remain discoverable as uncertain. Record the supported crash-consistency and platform limits.

Coordinate #129 ownership, #133 artifact publication, #134 retention and #145–148 recovery before declaring this independently usable.

#### Comment 5706589214 — Jordan-Hall — 2026-09-17T00:32:48Z

Source: https://github.com/Jordan-Hall/browser/issues/135#issuecomment-5706589214 | Updated: 2026-09-17T00:32:48Z

###### Planning review follow-up — CORE-02.T07

Parent: #3. No implementation PR for this task was found in the current inventory; these are acceptance refinements to the existing backup plan.

Add an explicit distinction between **durable retention requirements** and **temporary backup pins**. Backing up must acquire and verify its own pins before GC can delete the snapshot's blobs. Restoring that snapshot must preserve user/legal retention and suppression, but must not resurrect an abandoned backup worker's lease as permanent authority or an immortal GC blocker.

The new reference-registration defect in #797 / #133 matters directly: a returned `Ok` currently does not always prove that a pin was inserted. Require the fixed API and a backup-versus-GC test that asserts actual reference rows, not mocked success.

Add failpoints after pin acquisition, during blob copy, after bundle completion and before pin release. Recovery should reclaim abandoned temporary pins safely without exposing suppressed data. Authenticate and verify the complete snapshot before activation; activation remains dispatch-disabled until recovery review.

Keep backup consistency, confidentiality, authenticity and dispatch safety as separate reported checks. No backup implementation or passing qualification is claimed.


---

<a id="issue-136"></a>
## #136 — [TASK][CORE-02.T08] Run storage fault and power-loss qualification

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/136
**Created:** 2026-09-15T15:14:04Z | **Updated:** 2026-09-17T00:33:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #3
**PRs mentioning this issue:** [#801](https://github.com/Jordan-Hall/browser/pull/801), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#803](https://github.com/Jordan-Hall/browser/pull/803), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#808](https://github.com/Jordan-Hall/browser/pull/808), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #3

Task ID: `CORE-02.T08`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706023771 — Jordan-Hall — 2026-09-16T23:27:09Z

Source: https://github.com/Jordan-Hall/browser/issues/136#issuecomment-5706023771 | Updated: 2026-09-16T23:27:09Z

<!-- intent-core-review:2026-09-16:issue-136 -->
###### Planning review — CORE-02.T08

Parent: #3. No task implementation PR was found. This is a qualification plan; no power-loss testing is claimed.

**Build the test oracle first:** define invariants for each state boundary: no bytes dispatched before recorded authorization/attempt; no completed operation without matching evidence; snapshot and journal revisions agree; inbox effects/dedup/cursor commit together; no live reference points to a physically removed artifact; restoration does not revive expired authority. Check authoritative fixture state, not the worker's success message.

**Failure matrix:** instrument named boundaries before/after database commit, outbox start/send/result, artifact write/fsync/link/register, suppression/unlink/metadata removal, and backup publication. Exercise process kill, partial writes, disk full, permission denial, busy readers, corrupted metadata/blob/manifest, stale lease holders, clock changes and repeated restart. Each row needs a documented converged state and reproducible seed/trace.

**Important evidence distinction:** process termination tests do not prove storage survives machine power loss. Use a controlled VM/storage fault setup for actual power-loss qualification, identify filesystem/mount/cache/platform assumptions, and label untested platforms explicitly. Never substitute repeated unit tests for those results.

**Regressions from this review:** include the actual #801 suppression failure, retained-prefix starvation, GC versus new hold/reference, #795 abandoned attempts/cancelled claims, #794 attempt substitution, #793 migration concurrency/user_version, and #796 incomplete persisted effects. Do not bypass these failures to make a green report.

**Deliverable:** machine-readable case IDs, tested commit/toolchain/lock/corpus/platform, expected versus observed outcomes, failure artifacts uploaded even when the job fails, and a bounded retry policy. Live commerce/messages must be replaced by resettable fixtures with externally observable commit counts. Cross-link runtime restart coverage with #152 and integrated coverage with #213.

#### Comment 5706592142 — Jordan-Hall — 2026-09-17T00:33:11Z

Source: https://github.com/Jordan-Hall/browser/issues/136#issuecomment-5706592142 | Updated: 2026-09-17T00:33:11Z

###### Planning review follow-up — CORE-02.T08 regression seeds

Parent: #3. No implementation PR for this qualification task was found; the existing fault matrix remains the baseline.

Promote the new review counterexamples into named deterministic Rust regressions before running the larger fault campaign:

- #133/#797: successful-looking empty reference registration must not leave the durable pin absent.
- #129/#793: missing existing-store metadata must not silently create a new identity.
- #130/#794: post-commit readback failure must not report the command as rolled back.
- #134/#801: Unicode error truncation must not panic; zero maintenance budget must not perform unbounded expiry deletion.

Keep the SQL reductions as explanatory fixtures only. The actual gate must execute the repository implementation and the pinned SQLite build. Include a failure while recording another failure, and capture both primary and secondary outcomes without losing the recovery marker.

Every case should record expected state, observed state, durable reference/attempt identities and the independent fixture effect count. A missing report is failure/not-run, not success. Process-kill coverage and VM/power-loss evidence remain separate; none is claimed as executed by this planning review.


---

<a id="issue-137"></a>
## #137 — [TASK][CORE-03.T01] Define worker registry and launch specifications

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/137
**Created:** 2026-09-15T15:14:13Z | **Updated:** 2026-09-17T00:33:46Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #4
**PRs mentioning this issue:** [#789](https://github.com/Jordan-Hall/browser/pull/789), [#791](https://github.com/Jordan-Hall/browser/pull/791), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#809](https://github.com/Jordan-Hall/browser/pull/809), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #4

Task ID: `CORE-03.T01`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706031754 — Jordan-Hall — 2026-09-16T23:27:58Z

Source: https://github.com/Jordan-Hall/browser/issues/137#issuecomment-5706031754 | Updated: 2026-09-16T23:27:58Z

<!-- intent-core-review:2026-09-16:issue-137 -->
###### Planning review — CORE-03.T01

Parent: #4. No implementation PR was found for this task. The parent's WorkerSpec/registry direction is sound; specify these contracts before launch code is added.

**Proposed structure:** keep immutable worker specifications separate from live instances and task grants. A specification identifies executable/package digest, role, supported protocol, required confinement profile, resource requests and restart class. An instance records its fresh launch epoch, owned OS process handle, one-use bootstrap/channel identity, negotiated protocol and lifecycle state. Registration must not itself confer tool authority.

**Executable integrity:** avoid a hash-path-then-exec race. Execute verified immutable bytes through a supported handle/private publication mechanism, or state the platform boundary and deny the unsupported profile. Restrict inherited environment, descriptors, working directory and argument construction; a provider's project configuration is untrusted input, not part of the supervisor's policy.

**Cross-PR prerequisites:** fix #789's inherited-socketpair identity assumption and cloned bootstrap reuse; connect actual negotiated codecs from #790. The registry, rather than each worker, must consume launch authorization exactly once. Bind all later health/results/cancellation to instance and epoch so PID reuse or an old process cannot impersonate its replacement.

**Acceptance tests:** executable swapped between verification and launch; role/protocol mismatch; reused bootstrap; stale epoch; unexpected inherited handle; failed launch rollback; duplicate registry registration; supported-platform process identity. Preserve safe Rust in orchestration; isolate any necessary FFI in a small audited platform crate.

Deliver an ownership/state-transition document and actual fixture-worker launch evidence. An in-memory registry test alone will not qualify the process boundary.

#### Comment 5706597079 — Jordan-Hall — 2026-09-17T00:33:46Z

Source: https://github.com/Jordan-Hall/browser/issues/137#issuecomment-5706597079 | Updated: 2026-09-17T00:33:46Z

###### Planning review follow-up — CORE-03.T01

Parent: #4. No matching implementation PR was found. Retain the existing separation between WorkerSpec, live instance and task authority.

Make the **launch-spec fingerprint** cover all security-relevant configuration, not only executable bytes: role, argument template, working-directory policy, allowed environment/descriptor inheritance, confinement requirements, requested resources and protocol capabilities. Store references to credentials/grants, never their secret values in the fingerprinted manifest or journal.

A regression should keep the executable hash unchanged while changing a shell flag, working-directory policy or confinement profile. An old authorization must not remain valid merely because the binary is identical. Conversely, an approved identical retry should still allocate a fresh instance/epoch and consume new launch authority.

Link the actual-channel fixes from #125/#789 and maintain a typed unsupported-platform outcome rather than launching unconfined as fallback. This is a contract refinement for implementation, not a claim of an existing runtime defect or completed worker launch.


---

<a id="issue-138"></a>
## #138 — [TASK][CORE-03.T02] Implement process lifecycle and health

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/138
**Created:** 2026-09-15T15:14:19Z | **Updated:** 2026-09-17T00:34:17Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #4
**PRs mentioning this issue:** [#789](https://github.com/Jordan-Hall/browser/pull/789), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #4

Task ID: `CORE-03.T02`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706035104 — Jordan-Hall — 2026-09-16T23:28:16Z

Source: https://github.com/Jordan-Hall/browser/issues/138#issuecomment-5706035104 | Updated: 2026-09-16T23:28:16Z

<!-- intent-core-review:2026-09-16:issue-138 -->
###### Planning review — CORE-03.T02

Parent: #4. No implementation PR was found. Build lifecycle management around #137's owned instance/epoch, not an LLM-owned process loop.

**State contract:** define legal Starting → Ready → Draining → Stopped/Failed transitions, including timeout, launch failure, requested shutdown and unexpected exit. Ready requires a successful authenticated handshake and capability/protocol validation. Heartbeats are observations, not proof that a task completed or that a dead process may regain authority.

**Implementation boundary:** retain owned child/process handles, observe OS exits, and separately track handshake deadline, heartbeat freshness and task-progress freshness. Use bounded monotonic timers in a live session and an explicit suspend/resume policy. A delayed Ready/heartbeat from an old epoch must not resurrect a revoked instance.

Drain stdout/stderr/progress through bounded channels so a verbose worker cannot deadlock process exit or cancellation. Decide which diagnostics may be dropped or summarized, and keep secrets/content out of routine health telemetry. Cleanup must close handles, reap the process and release admission reservations exactly once; distinguish a signalled shutdown from confirmed process-tree termination.

**Acceptance tests:** actual subprocesses that never handshake, stop heartbeating, flood logs, exit during handshake, ignore graceful shutdown, fork descendants and race shutdown with exit. Verify bounded diagnostics, no leaked resources, no false Ready transition and no lost record of an already-started external attempt.

Keep worker restart decisions in #142 and side-effect recovery in CORE-04. This task should supply trustworthy lifecycle facts to them, not automatically retry external work when a process dies.

#### Comment 5706601440 — Jordan-Hall — 2026-09-17T00:34:17Z

Source: https://github.com/Jordan-Hall/browser/issues/138#issuecomment-5706601440 | Updated: 2026-09-17T00:34:17Z

###### Planning review follow-up — CORE-03.T02

Parent: #4. No implementation PR was found. Keep the existing lifecycle and health distinctions.

Add a **single-owner lifecycle reducer** over epoch-tagged observations. Define deterministic precedence when OS exit, handshake completion, heartbeat expiry and user cancellation arrive in different orders. For example, an observed exit or revoked epoch must not be overwritten by a queued Ready event; duplicate exit notifications must not release reservations twice.

Test every ordering of Ready/Exit/Cancel for the same instance, including a process that exits immediately after spawn and before ordinary registration callbacks complete. Assert one terminal lifecycle result, one resource release, no live dispatch authority and preserved external-attempt history.

Represent heartbeat freshness, progress freshness and OS-process existence separately in the health result. A process that is alive but blocked draining logs is not equivalent to a missing heartbeat caused by supervisor suspend. Feed facts to #142/#147; do not restart or replay tasks inside the health reducer. These are proposed acceptance refinements, not implemented behavior.


---

<a id="issue-139"></a>
## #139 — [TASK][CORE-03.T03] Implement admission and foreground priorities

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/139
**Created:** 2026-09-15T15:14:27Z | **Updated:** 2026-09-17T00:34:49Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #4
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #4

Task ID: `CORE-03.T03`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706038582 — Jordan-Hall — 2026-09-16T23:28:37Z

Source: https://github.com/Jordan-Hall/browser/issues/139#issuecomment-5706038582 | Updated: 2026-09-16T23:28:37Z

<!-- intent-core-review:2026-09-16:issue-139 -->
###### Planning review — CORE-03.T03

Parent: #4. No implementation PR was found. Preserve the parent's foreground/control/speech priorities, but separate admission, scheduling and authorization.

**Admission:** use validated immutable limit types and checked accounting for worker count, committed/reserved memory, CPU allocation estimates and queue bytes. Reuse the lessons from #791: public fields must not bypass validation, and bounded item count does not bound payload memory. A failed launch, cancellation or duplicate completion must release a reservation once, not twice.

**Scheduling:** reserve execution capacity as well as queue slots for stop and interactive control. If a worker holding a global lock or blocking SQLite call prevents the control executor from running, a priority queue alone does not solve inversion. Age background jobs within a defined ceiling; aging must never outrank cancellation. Add per-workspace/account fairness so one large agent run cannot occupy all admitted capacity.

Distinguish hard limits enforced by #140 from admission estimates and cooperative accelerator scheduling. Unknown capacity is not zero usage or unlimited capacity. Resource admission never supplies a missing tool grant.

**Acceptance tests:** saturate count and byte limits; cancellation under large artifact streams; speech/interactive latency under sustained work; background starvation bounds; overflow and invalid configuration; launch failure/exit races; duplicate reservation release; unavailable GPU metrics; task deadline expiration while queued. Report measured distributions and the exact machine/profile rather than an unqualified latency promise.

Deliver a deterministic scheduler simulation plus real-process load tests. Integrate #127's repaired queues and #141's revocation-before-dispatch checks before calling this a production supervisor.

#### Comment 5706607175 — Jordan-Hall — 2026-09-17T00:34:49Z

Source: https://github.com/Jordan-Hall/browser/issues/139#issuecomment-5706607175 | Updated: 2026-09-17T00:34:49Z

###### Planning review follow-up — CORE-03.T03

Parent: #4. No implementation PR was found. Add **atomic multi-resource reservation** to the existing admission plan: a request requiring worker slots, memory, CPU budget and accelerator capacity should reserve its whole admissible vector or reserve nothing.

Do not hold scarce worker/memory reservations indefinitely while waiting for an unavailable accelerator. Represent queued demand separately from committed admission; release/resize by a unique reservation identity so launch failure, timeout and duplicate completion cannot leak or double-free budget.

Test two competing jobs with complementary scarce resources, a permanently impossible request, cancellation during reservation, capacity reduction and duplicate release. Check the accounting invariant after every event, not only at the end of the test. Foreground/control reserves remain unavailable to background aging.

Keep non-blocking control processing separate from storage/metrics work, including #134's newly identified unbounded expiry pruning. This refines the plan and tests; it is not evidence that a scheduler or measured latency guarantee already exists.


---

<a id="issue-140"></a>
## #140 — [TASK][CORE-03.T04] Enforce platform resource constraints

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/140
**Created:** 2026-09-15T15:14:32Z | **Updated:** 2026-09-17T00:35:17Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #4
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #4

Task ID: `CORE-03.T04`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706043238 — Jordan-Hall — 2026-09-16T23:29:04Z

Source: https://github.com/Jordan-Hall/browser/issues/140#issuecomment-5706043238 | Updated: 2026-09-16T23:29:04Z

<!-- intent-core-review:2026-09-16:issue-140 -->
###### Planning review — CORE-03.T04

Parent: #4. No implementation PR was found. The parent's correction distinguishing hard OS limits, admission estimates and cooperative GPU scheduling should become an explicit capability contract.

**Proposed backend result:** report requested limits, effective enforced limits, backend identity, process-tree coverage, unsupported controls and evidence/probe status. Fail closed for unattended profiles whose required enforcement is unavailable. Never return an all-success structure merely because the process spawned.

Establish confinement/limits before untrusted worker code can run; avoid a spawn-then-attach window. Bind lifetime control to owned process/session handles and cover descendants, not only the original PID. Define cleanup and resource release after supervisor death. Filesystem/network authority remains a separate SEC boundary: memory/CPU limits are not a complete sandbox.

**Platform coverage:** maintain a tested backend per supported OS. Keep orchestration safe Rust, with unavoidable platform FFI isolated and its ownership/lifetime obligations documented. Cooperative model unload or a requested GPU budget must not be labelled hard VRAM isolation or universal preemption.

**Acceptance tests:** a worker attempts excessive allocation, process proliferation, sustained CPU use, descendant escape and ignored shutdown; observe whether each advertised limit actually constrains it. Verify the trusted control path remains responsive and the unsupported profile is rejected before launch. Capture backend/OS/hardware configuration and both requested and measured effective results.

Coordinate #137 launch, #139 admission, #141 tree termination and the security containment work. This task is not done by implementing interfaces plus unsupported stubs, nor by passing Linux-only unit tests while claiming all desktop platforms.

#### Comment 5706611320 — Jordan-Hall — 2026-09-17T00:35:17Z

Source: https://github.com/Jordan-Hall/browser/issues/140#issuecomment-5706611320 | Updated: 2026-09-17T00:35:17Z

###### Planning review follow-up — CORE-03.T04

Parent: #4. No implementation PR was found. Keep requested limits, effective enforcement and cooperative hints as different fields.

Add an **enforcement-capability freshness rule**: a previous successful probe must not qualify a changed backend, changed permissions or a resumed/recreated containment session automatically. Bind evidence to the backend/configuration and instance it tested, and revalidate required controls before an unattended launch.

Test backend setup partially succeeding, a required control becoming unavailable, supervisor death during setup and a descendant attempting work before setup completes. The observable result must be no unconfined execution, released reservations and an explicit unsupported/failed profile—not a silently weakened launch.

For supported platforms, provide real abuse probes and recovery evidence. For unsupported profiles, test rejection and document the remaining implementation work; a correct rejection does not make that platform fully implemented. Preserve safe orchestration and small audited platform adapters. This comment refines acceptance only.


---

<a id="issue-141"></a>
## #141 — [TASK][CORE-03.T05] Implement lease-first cancellation and revocation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/141
**Created:** 2026-09-15T15:14:40Z | **Updated:** 2026-09-17T00:35:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #4
**PRs mentioning this issue:** [#791](https://github.com/Jordan-Hall/browser/pull/791), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#809](https://github.com/Jordan-Hall/browser/pull/809), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #4

Task ID: `CORE-03.T05`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706046769 — Jordan-Hall — 2026-09-16T23:29:24Z

Source: https://github.com/Jordan-Hall/browser/issues/141#issuecomment-5706046769 | Updated: 2026-09-16T23:29:24Z

<!-- intent-core-review:2026-09-16:issue-141 -->
###### Planning review — CORE-03.T05

Parent: #4. No implementation PR was found. Treat lease-first cancellation as an authority transition, not just sending a Cancel message.

**Required ordering:** invalidate the current task/worker epoch at the broker's dispatch boundary first; record cancellation intent and affected operation identities; then notify the worker and start bounded graceful/forced termination. All later dispatch requests require a positively valid current lease, not merely an ID that is absent from a cancelled set. Restarted workers get fresh epochs and must not inherit executable authority from serialized old leases.

Specify the linearization point precisely. A request already committed as attempting or bytes already released to a dispatcher may have reached an external service; cancellation must retain it for reconciliation. Local acknowledgement, worker acknowledgement, OS process-tree termination and external reversal are different outcomes and should be separately observable.

**Prerequisites from reviewed code:** #791's queue/registry lifecycle needs correction; #795 needs generation-fenced claims/results, cancelled-outbox retirement and abandoned-attempt recovery. Revoke-before-notify is ineffective if callers can dispatch directly from a claim's exposed bytes.

**Acceptance tests:** dispatch racing cancellation; a previously buffered tool request arriving after revocation; old worker returning after replacement; duplicate cancels; full progress/control queues; unresponsive descendants; cancellation during a database stall; supervisor restart; external send with lost response. Stale results may be retained as evidence but must not authorize new actions or overwrite a newer attempt.

Deliver explicit cancellation/reconciliation status and audit events with bounded diagnostics. Do not report an uncertain purchase/message as successfully undone because its local process was killed.

#### Comment 5706616099 — Jordan-Hall — 2026-09-17T00:35:52Z

Source: https://github.com/Jordan-Hall/browser/issues/141#issuecomment-5706616099 | Updated: 2026-09-17T00:35:52Z

###### Planning review follow-up — CORE-03.T05

Parent: #4. No implementation PR was found. Add an explicit **cancellation persistence failure** outcome to the existing revocation-before-notification plan.

If the database is full or unavailable, inability to journal cancellation must not leave in-memory dispatch authority active or wait indefinitely before stopping the local worker. Fail closed for new dispatch, perform bounded local shutdown, expose that persistence/termination status separately, and ensure the next runtime incarnation cannot revive the old lease from a stale checkpoint.

Test cancellation with a full/stalled journal, acknowledgement queue exhaustion, process death before cancellation persistence, and a restart loading the older checkpoint. Verify no old-generation dispatch is accepted; an already-started external attempt stays discoverable/uncertain rather than being reported undone.

Use the result-observation path from #131 to retain verifiable late evidence without re-granting execution rights. Include #127's retire/re-register counterexample. These are proposed failure contracts; no cancellation implementation or recovery evidence is claimed.


---

<a id="issue-142"></a>
## #142 — [TASK][CORE-03.T06] Add crash-loop policy and degraded service

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/142
**Created:** 2026-09-15T15:14:45Z | **Updated:** 2026-09-17T00:36:28Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #4
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #4

Task ID: `CORE-03.T06`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706051221 — Jordan-Hall — 2026-09-16T23:29:49Z

Source: https://github.com/Jordan-Hall/browser/issues/142#issuecomment-5706051221 | Updated: 2026-09-16T23:29:49Z

<!-- intent-core-review:2026-09-16:issue-142 -->
###### Planning review — CORE-03.T06

Parent: #4. No implementation PR was found. Preserve the parent's bounded restart/degraded-mode requirement and make restart eligibility independent of task replay eligibility.

**Proposed policy:** classify exit/health failures; apply bounded restart counts, capped backoff and an explicit circuit-open state per integration/version. Reset the failure budget only after a defined stable interval, not immediately after a successful spawn. Retain enough history that restarting the supervisor cannot trivially reset a crash loop. Inject time/randomness in tests so backoff is reproducible.

A replacement worker always gets a new epoch and handshake. Load only a runtime-owned, verified checkpoint and rerun current authorization/recovery classification; do not silently restore the failed worker's tool permissions or rerun its last external command. Persist unresolved operation lineage separately from process health.

**Degraded behavior:** cached workspace data and deterministic controls should remain usable when optional model/provider workers fail. A failed policy/state authority, however, must disable dependent execution rather than fall back to permissive operation. Surface the affected capability, reason, available safe operations and an explicit retry/reset action. Do not hide failures by switching to an unapproved remote provider.

**Acceptance tests:** repeated immediate exit; intermittent healthy periods; supervisor restart during backoff; poisoned checkpoint; unavailable local model; failed security-critical worker; concurrent integration failures; recovered old process sending late output; no duplicate fixture side effect after restart.

Coordinate #138 health facts, #141 revocation, #146 checkpoint integrity and #147 recovery planning. Runtime fixture evidence and later real workspace/UI evidence should be labelled separately rather than claiming a not-yet-built shell has been tested.

#### Comment 5706620714 — Jordan-Hall — 2026-09-17T00:36:28Z

Source: https://github.com/Jordan-Hall/browser/issues/142#issuecomment-5706620714 | Updated: 2026-09-17T00:36:28Z

###### Planning review follow-up — CORE-03.T06

Parent: #4. No implementation PR was found. The existing restart-versus-replay separation is essential.

Specify **who can reset an open circuit and what a reset changes**. Fresh worker epochs must not reset a crash budget accidentally; otherwise every relaunch escapes the limit. An explicit user retry should be deduplicated and still obey admission, current grants and quarantine decisions. Installing a new verified integration version may use a separate budget, while retaining the previous version's diagnostic history.

Test rapid repeated Retry clicks, supervisor restart during cooldown, wall-clock rollback, identical binary with changed launch configuration, and a replacement worker failing before handshake. Assert bounded launches, no duplicate task dispatch, a retained actionable diagnostic and usable deterministic workspace controls.

A reset should reopen eligibility for a fresh attempt, not erase unresolved external-operation history or automatically authorize cloud fallback. This is an implementation-plan refinement; it does not claim that degraded mode or restart policy has been built.


---

<a id="issue-143"></a>
## #143 — [TASK][CORE-03.T07] Integrate resource observation and cooperative yields

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/143
**Created:** 2026-09-15T15:14:49Z | **Updated:** 2026-09-17T00:36:57Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #4
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #4

Task ID: `CORE-03.T07`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706054438 — Jordan-Hall — 2026-09-16T23:30:06Z

Source: https://github.com/Jordan-Hall/browser/issues/143#issuecomment-5706054438 | Updated: 2026-09-16T23:30:06Z

<!-- intent-core-review:2026-09-16:issue-143 -->
###### Planning review — CORE-03.T07

Parent: #4. No implementation PR was found. The resource-observation contract should distinguish measured usage, reserved capacity, estimates, stale samples and unsupported metrics.

**Proposed implementation:** associate bounded telemetry with a current worker instance/epoch and monotonic sample age. Export CPU/memory/queue counters and available accelerator data without prompt text, file content, credentials or high-cardinality user identifiers. Missing GPU data must remain unknown, not report zero usage. Bound collection overhead so metrics cannot starve the control path.

**Cooperative yielding:** issue an epoch-scoped request to pause optional work, unload a model or release eligible caches; observe an acknowledgement and actual resource change before admitting work that depends on that release. Protect state required by active tasks/checkpoints. Apply hysteresis and minimum residency/backoff to avoid repeated unload/reload thrashing. A request to release memory is not hard enforcement; escalate according to the tested platform profile rather than claiming universal accelerator preemption.

**Acceptance tests:** stale/reordered telemetry; worker replacement; missing counters; a worker falsely acknowledging yield without releasing capacity; concurrent speech and coding load; repeated memory-pressure oscillation; telemetry floods; cancellation during unload; no disclosure of task content in diagnostics.

Report foreground/speech latency and throughput on declared hardware with the exact workload. Keep admission accounting in #139, effective limits in #140 and the content-free observation layer here; do not create three independent sources of truth for available capacity.

#### Comment 5706624456 — Jordan-Hall — 2026-09-17T00:36:57Z

Source: https://github.com/Jordan-Hall/browser/issues/143#issuecomment-5706624456 | Updated: 2026-09-17T00:36:57Z

###### Planning review follow-up — CORE-03.T07

Parent: #4. No implementation PR was found. Refine the existing yield/telemetry plan with a **correlated release acknowledgement**: bind each yield request to request ID, worker epoch, resource/model instance and the reservation it may affect.

A late acknowledgement of an earlier unload must not release a new model's reservation, and the same observed memory reduction must not be credited once by telemetry and again by an acknowledgement. Admission owns the accounting; telemetry provides evidence, not an independent budget mutation path.

Test two overlapping yield requests, cancelled unload followed by reload, duplicated acknowledgement, worker replacement, counter reset and stale low-usage samples. Assert no double release or over-admission and retain Unknown for unavailable measurements.

Keep the existing hysteresis/minimum-residency requirements and record controller decisions without prompts, paths or account identifiers. This is a proposed interface/test refinement, not a claim of a completed measurement backend or performance benchmark.


---

<a id="issue-144"></a>
## #144 — [TASK][CORE-03.T08] Verify supervisor robustness under concurrency

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/144
**Created:** 2026-09-15T15:14:57Z | **Updated:** 2026-09-17T00:37:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #4
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #4

Task ID: `CORE-03.T08`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706058294 — Jordan-Hall — 2026-09-16T23:30:26Z

Source: https://github.com/Jordan-Hall/browser/issues/144#issuecomment-5706058294 | Updated: 2026-09-16T23:30:26Z

<!-- intent-core-review:2026-09-16:issue-144 -->
###### Planning review — CORE-03.T08

Parent: #4. No implementation PR was found. Qualify the integrated supervisor, not only the independent queue/state-machine helpers.

**Required harness:** real fixture subprocesses with deterministic modes for handshake failure, delayed/reordered events, log flooding, hung IO, deliberate exit, ignored shutdown and descendant creation. Drive concurrent launch/stop/revoke/restart sequences with recorded seeds and bounded deadlines. Use a deterministic model of registry/lease/reservation state as the oracle, then compare actual process and database state.

**Invariants to assert:** at most one current epoch per instance; no Ready before authentication; no dispatch after revocation; no double release or resource leak; stale events never resurrect a process; uncertain external effects remain recorded; stop/control processing survives progress/log saturation. Check owned handles/descendants, not just a PID no longer appearing.

**Evidence distinction:** the current #791 test uses two in-memory endpoints and does not meet the parent's worker-boundary requirement. Add real blocked-transport and process tests. Race-focused model checking can supplement them but cannot prove the OS backend works. Run stress/soak and platform-specific suites with effective confinement probes from #140.

**Failure evidence:** preserve minimal event traces, head SHA, platform/profile, scheduler seed, timing distributions and state snapshots. Upload diagnostics on failure, redact secrets, and distinguish timeouts from proof of termination. A flaky test should be investigated and reduced, not retried until a green result hides it.

Coordinate storage crash tests #136 and end-to-end recovery #152/#213. Define explicit pass criteria for the initially supported profile and mark other platform/hardware combinations unqualified.

#### Comment 5706627902 — Jordan-Hall — 2026-09-17T00:37:23Z

Source: https://github.com/Jordan-Hall/browser/issues/144#issuecomment-5706627902 | Updated: 2026-09-17T00:37:23Z

###### Planning review follow-up — CORE-03.T08

Parent: #4. No implementation PR was found. Add a **test-oracle independence gate** to the existing concurrency plan: the expected-state model must not simply call the production transition functions it is meant to verify.

Validate the harness with deliberate mutations: accept Ready after Exit, skip epoch revocation, double-release a reservation, reuse a retired cancellation ID, or credit the same yield twice. Each mutation should produce a small failing trace that identifies the violated invariant.

Add metamorphic checks: duplicating harmless progress messages or changing their interleaving must not change authorization, reservation totals or external effect counts. Use explicit synchronization points for targeted races rather than sleep-based guesses, then complement them with real process/transport stress.

Record all failing seeds and distinguish harness timeout from observed worker termination. Include the new event-ordering, atomic-reservation, persistence-failure and yield-correlation cases from #138/#139/#141/#143. This specifies stronger qualification; it does not claim those tests have run.


---

<a id="issue-145"></a>
## #145 — [TASK][CORE-04.T01] Classify every recoverable operation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/145
**Created:** 2026-09-15T15:15:04Z | **Updated:** 2026-09-17T00:37:58Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #5
**PRs mentioning this issue:** [#795](https://github.com/Jordan-Hall/browser/pull/795), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#804](https://github.com/Jordan-Hall/browser/pull/804), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #5

Task ID: `CORE-04.T01`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706065357 — Jordan-Hall — 2026-09-16T23:31:04Z

Source: https://github.com/Jordan-Hall/browser/issues/145#issuecomment-5706065357 | Updated: 2026-09-16T23:31:04Z

<!-- intent-core-review:2026-09-16:issue-145 -->
###### Planning review — CORE-04.T01

Parent: #5. No implementation PR was found. Make recovery classification an exhaustive, versioned policy over durable operation facts—not a model's judgement or a retry boolean.

**Proposed matrix inputs:** effect class; recorded stage and attempt identity; original versus compensation phase; current account/capability authority; source preconditions; deadline; local before/after version evidence; and provider idempotency guarantees where explicitly supported. Missing or contradictory facts must produce a blocked/uncertain disposition, not default to retry.

Distinguish safe read re-execution, verified local completion, conflict requiring user choice, provider-idempotent continuation, read-only reconciliation, fresh approval required and unsupported/manual recovery. A provider's idempotency key must retain its original account, payload binding, operation and validity window. An expired key or changed payload cannot be treated as proof that another send is safe.

**Current blockers to resolve:** #794 loses compensation origin and allows attempt replacement; #795 lacks abandoned-attempt discovery and approved-payload binding. Classification cannot be reliable while the durable record can misidentify the attempted request. Map the wire and persisted state machines explicitly.

**Acceptance tests:** every registered executable capability has a recovery policy; unknown versions/effects fail closed; timeout after send remains uncertain; compensation success is not original-action success; revoked/expired grants block continuation; proven non-commit differs from an inconclusive provider query. Test the full state/effect cross-product and property invariants.

Keep policy computation pure and testable; the later executor must revalidate it immediately before acting. This task should define what evidence is required, not generate missing evidence.

#### Comment 5706632316 — Jordan-Hall — 2026-09-17T00:37:58Z

Source: https://github.com/Jordan-Hall/browser/issues/145#issuecomment-5706632316 | Updated: 2026-09-17T00:37:58Z

###### Planning review follow-up — CORE-04.T01

Parent: #5. No implementation PR was found. Extend the recovery matrix with **known local commit but failed acknowledgement/readback** as a separate input, using the source finding on #794 / #130.

Do not collapse clean rollback, durable local commit, externally accepted effect and unknown external effect into a generic `Error → Retry` branch. A later timeout or missing acknowledgement must not erase stronger recorded commit evidence. Permission to continue is a separate current-authority check, not inferred from prior success.

Add table cases for committed journal revision + failed readback; accepted provider effect + missing local receipt; compensation timeout + later confirmation; conflicting evidence; and a matching but expired idempotency key. Require the classifier to name the evidence needed for its next permitted step.

Share these cases with #148/#152 and validate persisted attempt lineage first. This is a planning/acceptance refinement, not a new implementation or a claim that provider idempotency is universally available.


---

<a id="issue-146"></a>
## #146 — [TASK][CORE-04.T02] Implement consistent checkpoints

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/146
**Created:** 2026-09-15T15:15:14Z | **Updated:** 2026-09-17T00:38:22Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #5
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#808](https://github.com/Jordan-Hall/browser/pull/808), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #5

Task ID: `CORE-04.T02`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706068129 — Jordan-Hall — 2026-09-16T23:31:20Z

Source: https://github.com/Jordan-Hall/browser/issues/146#issuecomment-5706068129 | Updated: 2026-09-16T23:31:20Z

<!-- intent-core-review:2026-09-16:issue-146 -->
###### Planning review — CORE-04.T02

Parent: #5. No implementation PR was found. The parent's checkpoint contents are appropriate, but consistency needs a commit contract spanning database state and durable artifacts.

**Proposed record:** checkpoint/version ID, task and graph revision, operation/attempt references, consumed event positions, committed artifact handles/digests, provider secondary session identifiers, repository/base revisions where relevant, and the producing worker epoch. Record the old epoch as historical evidence; restore under a fresh epoch after authorization, never revive it as a live lease.

Publish and verify blobs before atomically registering checkpoint references and its database revision. Coordinate retention pins with #134 so a checkpoint cannot name a blob GC removes during creation or restore. Reject a checkpoint if any required dependency is missing, suppressed for this use, corrupt or from an unsupported schema. Do not silently replace it with transcript-derived guesses.

Exclude raw OS/CEF handles, sockets, credentials, reusable approval tokens and in-memory model/provider internals. Keep application-owned workspace/task state authoritative; a provider session ID is only an optional resume hint.

**Acceptance tests:** crash at artifact publication/reference registration/checkpoint commit; graph revision changes during capture; missing/corrupt blob; cursor mismatch; stale source/repository base; revoked account; restored old worker epoch; retained checkpoint racing GC; unsupported schema. Demonstrate that the published checkpoint resolves to one coherent state or fails visibly.

Coordinate #135's authenticated backup format rather than inventing a conflicting bundle format. Provide a typed storage API and bounded dependency enumeration; a serializable struct alone is not consistent checkpointing.

#### Comment 5706635716 — Jordan-Hall — 2026-09-17T00:38:21Z

Source: https://github.com/Jordan-Hall/browser/issues/146#issuecomment-5706635716 | Updated: 2026-09-17T00:38:21Z

###### Planning review follow-up — CORE-04.T02

Parent: #5. No implementation PR was found. Add a **complete dependency-set commitment** to checkpoint validation, using the lesson from #132/#796: verifying only the dependency rows that still exist cannot detect a missing row.

Bind the checkpoint revision to the complete canonical set of required artifact handles/digests, cursors, operation/attempt references and graph revision. Validate set membership/completeness as well as each surviving blob. A valid zero-dependency checkpoint must remain distinguishable from a damaged non-empty checkpoint.

Add regressions deleting one reference, all references, or one cursor after checkpoint publication. Restore must fail visibly, not succeed with a truncated context. Also use #133's repaired reference API so a successful checkpoint pin is proven to exist and remains protected during GC.

This integrity commitment does not grant access: current suppression, account rights and lease checks still apply before reseeding a worker. Keep transient backup pins distinct from permanent checkpoint retention. Planning review only.


---

<a id="issue-147"></a>
## #147 — [TASK][CORE-04.T03] Plan startup recovery before dispatch

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/147
**Created:** 2026-09-15T15:15:18Z | **Updated:** 2026-09-17T00:39:08Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #5
**PRs mentioning this issue:** [#795](https://github.com/Jordan-Hall/browser/pull/795), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#803](https://github.com/Jordan-Hall/browser/pull/803), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#808](https://github.com/Jordan-Hall/browser/pull/808), [#809](https://github.com/Jordan-Hall/browser/pull/809), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #5

Task ID: `CORE-04.T03`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706071503 — Jordan-Hall — 2026-09-16T23:31:38Z

Source: https://github.com/Jordan-Hall/browser/issues/147#issuecomment-5706071503 | Updated: 2026-09-16T23:31:38Z

<!-- intent-core-review:2026-09-16:issue-147 -->
###### Planning review — CORE-04.T03

Parent: #5. No implementation PR was found. Startup recovery must run behind a dispatch-disabled barrier, including after backup restoration and suspend/resume.

**Proposed sequence:** acquire exclusive profile ownership; validate database/migrations/artifacts; enumerate unfinished tasks and attempts; load current policy/account/auth/clock state; generate and persist revision-bound RecoveryPlan decisions; expose blocked/uncertain states; then admit only the operations currently eligible to resume. Do not start workers with external write authority merely because their last persisted state was Running.

Each plan should identify its checkpoint, relevant operation/attempt, source/task revisions, current authority inputs and disposition. Applying a plan must compare those revisions and revalidate grants/deadlines immediately before dispatch. A plan is a proposal derived from state, not a transferable authorization token. Invalidate it after policy/account/source changes.

**Current integration gap:** #795's attempting rows need a public bounded enumeration/recovery path, and the state store currently lacks the complete durable task/workspace/grant repositories named in #3. Specify where those records live before implementing a planner that only scans a subset of operation rows.

**Acceptance tests:** crash after plan persistence but before execution; concurrent cancellation/revocation; expired login; wall-clock change/suspend; a missing artifact; a prior external send with no result; multiple competing startup owners; stale plan application; repeated startup converges without duplicate dispatch. Unsupported schemas must open only an explicitly safe recovery path, not be rewritten opportunistically.

Coordinate #129 ownership, #141 revocation, #145 classification, #146 checkpoints and #151 task-centre actions. Keep unknown outcomes unknown until supported evidence resolves them.

#### Comment 5706642199 — Jordan-Hall — 2026-09-17T00:39:08Z

Source: https://github.com/Jordan-Hall/browser/issues/147#issuecomment-5706642199 | Updated: 2026-09-17T00:39:08Z

###### Planning review follow-up — CORE-04.T03

Parent: #5. No implementation PR was found. Make the recovery barrier an **enforced runtime state at every dispatch ingress**, not a UI flag or a convention followed only by startup code.

Provide separate inspection/reconciliation access while the barrier is closed. A serialized `Running` task, an old lease, a queued scheduler event or a direct connector call must not bypass it. Opening the barrier should require a current validated recovery revision plus live authority checks, not a caller-provided boolean.

Test direct dispatch entry points while startup validation is incomplete, failed, or invalidated by a new revocation. Race barrier opening with cancellation and policy changes; repeated startup should not create fresh executable actions. Allow safe inspection even when inference is unavailable.

Integrate #129's existing-store identity checks, #131's abandoned-attempt enumeration, #141's cancellation-persistence failure and #146's complete checkpoint dependency validation. A planner cannot compensate for omitted records or missing pins. These are acceptance requirements for the proposed implementation, not claims that the barrier already exists.


---

<a id="issue-148"></a>
## #148 — [TASK][CORE-04.T04] Reconcile local writes and uncertain external state

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/148
**Created:** 2026-09-15T15:15:32Z | **Updated:** 2026-09-17T00:39:39Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #5
**PRs mentioning this issue:** [#795](https://github.com/Jordan-Hall/browser/pull/795), [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#809](https://github.com/Jordan-Hall/browser/pull/809), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #5

Task ID: `CORE-04.T04`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706075417 — Jordan-Hall — 2026-09-16T23:32:00Z

Source: https://github.com/Jordan-Hall/browser/issues/148#issuecomment-5706075417 | Updated: 2026-09-16T23:32:00Z

<!-- intent-core-review:2026-09-16:issue-148 -->
###### Planning review — CORE-04.T04

Parent: #5. No implementation PR was found. Reconciliation should append independently sourced evidence to the exact attempt, not replace uncertainty with an inferred success flag.

**Local writes:** persist the intended change and relevant before/after content/version identities. On recovery, inspect through the authorized filesystem boundary and compare actual state. Distinguish already-applied, proven-not-applied, user-modified/conflicting and still-unknown. Never overwrite a user's intervening edit to make the expected result appear true.

**External writes:** expose connector-specific read-only reconciliation capabilities bound to the original account, provider request/idempotency identity, target and attempt. A query returning no result may reflect stale indexing, pagination, an unavailable service or insufficient permissions; only documented authoritative evidence may prove non-commit. Retain observation time/source and the precise evidence behind each conclusion.

Keep original execution separate from compensation. A refund/cancel/delete intended to compensate for an earlier action is a new side effect with its own authorization, attempt and receipt; it does not erase the original ledger. Repair #794's compensation-origin and attempt-continuity gaps first.

**Acceptance tests:** committed send with lost response; delayed visibility; wrong account; partial order/page results; provider downtime; conflicting evidence; local user edit; stale reconciliation result after another result is recorded; compensation uncertainty later confirmed. The result must be confirmed success, proven non-commit, conflict or still-unknown—not an unqualified boolean.

Coordinate the existing transaction reconciliation requirement #79 and #145's policy matrix. Fixture tests can qualify generic behavior; live connector support must be demonstrated separately before advertising that provider's recovery capability.

#### Comment 5706646218 — Jordan-Hall — 2026-09-17T00:39:39Z

Source: https://github.com/Jordan-Hall/browser/issues/148#issuecomment-5706646218 | Updated: 2026-09-17T00:39:39Z

###### Planning review follow-up — CORE-04.T04

Parent: #5. No implementation PR was found. Extend the existing evidence-bound reconciliation plan with **duplicate observation and downstream reservation semantics**.

An uncertain external operation must not free its spending/resource commitment merely because its worker stopped or one lookup returned no result. Define which independently verified outcome permits downstream consumers to release, retain or convert that reservation. A duplicate acceptance/refund observation must not apply the accounting effect twice.

Test original acceptance arriving late after cancellation, a compensation result arriving before the original receipt is recovered, repeated reconciliation observations and conflicting provider observations. Preserve both original and compensating histories; never overwrite one attempt's result with another or automatically launch compensation from a late callback.

Use the observation identity and already-recorded result contract proposed on #131/#795, plus revision-checked local effects from #132. Keep any compensating write behind a fresh appropriate authorization. This is a cross-component acceptance contract for #79 and CORE recovery, not a claim of implemented transaction support.


---

<a id="issue-149"></a>
## #149 — [TASK][CORE-04.T05] Build no-production replay contexts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/149
**Created:** 2026-09-15T15:15:37Z | **Updated:** 2026-09-17T00:40:10Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #5
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#804](https://github.com/Jordan-Hall/browser/pull/804), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #5

Task ID: `CORE-04.T05`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5706079615 — Jordan-Hall — 2026-09-16T23:32:23Z

Source: https://github.com/Jordan-Hall/browser/issues/149#issuecomment-5706079615 | Updated: 2026-09-16T23:32:23Z

<!-- intent-core-review:2026-09-17:issue-149 -->
###### Review — CORE-04.T05: production-free replay

Parent: #5. This is a design/acceptance review; no implementation PR was found for this task.

**Make replay a different execution environment, not a boolean on the production dispatcher.** A ReplayContext should expose captured observations, fixture-local storage, an injected clock/random source and deterministic event ordering. It should have no production credential resolver, connector dispatcher, browser profile or desktop-control capability. Do not load arbitrary recorded commands through a shell. Prefer a separate process and narrowly selected dependencies; use tested OS isolation for the stronger no-network/no-host-write guarantee.

**Trace contract:** version the trace; bind it to fixture/model/adapter/schema revisions, sanitized input digests and expected outputs. Preserve task/operation/attempt identity relationships without making recorded grants executable. Record nondeterministic inputs needed for replay, not private model reasoning. Missing events, unknown versions or unavailable captured payloads should yield ReplayIncomplete, not silently fall back to a live provider.

**Evidence semantics:** replay can prove that the deterministic runtime behaves consistently for the supplied observations. It cannot prove the model would regenerate identical text, that the external service currently has the same state, or that a historical transaction was rolled back. Keep those assertions out of conformance reports.

**Required tests:** load a purchase/message/deletion trace and attempt a live network request, credential lookup, host-file mutation and desktop action; each forbidden path must be absent or denied. Repeat with hostile URLs/tool names embedded in captured content. Test missing artifacts, oversized/malformed traces, shuffled/duplicated events and schema skew. The same seed and captured inputs should produce the same runtime decisions and explicit trace divergences.

Link #128 conformance/fuzzing, #136 storage qualification, #146 checkpoint format, #148 reconciliation and #152 restart tests. Reuse fixtures, but never reuse a production write credential just to make a replay convenient. No replay isolation or tests are claimed as implemented by this comment.

#### Comment 5706080247 — Jordan-Hall — 2026-09-16T23:32:26Z

Source: https://github.com/Jordan-Hall/browser/issues/149#issuecomment-5706080247 | Updated: 2026-09-16T23:32:26Z

<!-- intent-core-review:2026-09-16:issue-149 -->
###### Planning review — CORE-04.T05

Parent: #5. No implementation PR was found. Make no-production replay a structurally different execution context, not a runtime flag on the live dispatcher.

**Proposed boundary:** a replay executable/library wiring that has no credential-vault access and no production side-effect dispatcher. Supply captured observations or explicitly resettable fixture connectors through narrow interfaces. Enforce the selected network boundary at the process/container level; an offline label or empty environment alone is not proof that no inherited credential, socket or fallback provider can be used.

Replay time, random inputs, external observations and event ordering from versioned records. Captured model outputs may be replayed deterministically; rerunning a model/provider is a separate evaluation mode and must not be described as reproducing identical internal behavior. Keep replay output in a fresh namespace with no live task grants or outbox inheritance.

**Input safety:** traces and attachments are untrusted, bounded data. Verify referenced artifacts and schema/manifest integrity; reject path traversal, huge collections and trace instructions that attempt to enable tools or load project hooks. Preserve privacy labels and use redacted fixtures by default; replay is not an excuse to export private prompts/content.

**Acceptance tests:** a purchase/message/deletion trace cannot reach a production endpoint or obtain credentials even when it asks to; environment and inherited-handle checks; malicious URLs/configuration; missing/corrupt artifacts; unsupported schema; deterministic repeat output for captured runs; strict limits on time/memory/output.

Coordinate #128 conformance, #136 fault fixtures and #213 end-to-end replay. Publish actual negative egress/side-effect evidence, not just a test that a `production = false` field was set.

#### Comment 5706650172 — Jordan-Hall — 2026-09-17T00:40:10Z

Source: https://github.com/Jordan-Hall/browser/issues/149#issuecomment-5706650172 | Updated: 2026-09-17T00:40:10Z

###### Planning review follow-up — CORE-04.T05

Parent: #5. No implementation PR was found. The two existing replay reviews already cover isolation and hostile inputs; retain those requirements without duplicating their implementation paths.

Add **capture completeness and terminal-state checks**. A syntactically valid trace with its final acknowledgement, cancellation or commit observation removed must not report a successful replay merely because all remaining events were consumed. Bind the expected event/dependency inventory and terminal disposition to the capture manifest; return Incomplete or Diverged when they cannot be established.

Mutation tests should remove the final event, one artifact reference and one nondeterministic input, then replay an empty-but-valid trace. None may vacuously pass a non-empty scenario. Distinguish replaying captured model output from regenerating it.

Use a separately observed fixture effect ledger for expected external outcomes, and keep output in a fresh non-production namespace. These additions strengthen the acceptance gate; they do not claim a replay implementation or isolation test has run.


---

<a id="issue-150"></a>
## #150 — [TASK][CORE-04.T06] Implement provider resume and reseeding policy

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/150
**Created:** 2026-09-15T15:15:45Z | **Updated:** 2026-09-17T00:40:42Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #5
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#804](https://github.com/Jordan-Hall/browser/pull/804), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #5

Task ID: `CORE-04.T06`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5706074351 — Jordan-Hall — 2026-09-16T23:31:55Z

Source: https://github.com/Jordan-Hall/browser/issues/150#issuecomment-5706074351 | Updated: 2026-09-16T23:31:55Z

<!-- intent-core-review:2026-09-17:issue-150 -->
###### Review — CORE-04.T06: provider resume and reseeding

Parent: #5. Planning review: no implementation PR for this task was found in the current CORE stack.

The proposed distinction between actual provider resume and creating a new session is essential. Make it an explicit result such as ResumeExisting / ReseedNew / RequiresLogin / Blocked / ReconcileFirst, with a reason and evidence. A provider session ID is an opaque secondary reference, not proof of ownership, compatibility or authorization.

###### Clarify the resume contract before implementation
Bind the resume reference to provider identity, adapter/protocol version, account, task, workspace and checkpoint revision. Negotiate the actual optional capability; unsupported resume must choose reseeding or a visible block, never a guessed CLI command or silent cloud fallback. Create a fresh runtime worker epoch even when the provider session survives.

For reseeding, assemble only the permitted shared goal, unresolved constraints, evidence, artifacts, code diff and verified test outcomes. Recheck artifact access/suppression and inference destinations at the time of reseeding. Exclude credentials and undocumented internal session state. Revalidate repository base/worktree status before applying or continuing a patch.

**Side-effect gate:** an old session may already have issued an external action. Inspect durable operations/outbox first and route uncertain attempts through #148; neither successful session recovery nor provider replacement grants permission to repeat them. A repeated provider event must use the inbox deduplication path, with a source namespace that survives a session restart without conflating accounts.

###### Acceptance evidence
Test supported and unsupported resume, expired account/session, changed adapter version, changed repository base, suppressed artifact, provider substitution and local-only mode. Assert stable task/workspace IDs but a fresh execution epoch; no increase in grants or data destinations; late old-session events cannot dispatch. Record whether each test used a protocol fixture or a real supported adapter.

Dependencies: #125/#126 authentication/version integration, #131/#132 durable events, #146 checkpoints, #147 current-authority planning, and the AGENT provider-adapter issues. The core policy can be tested with typed fixtures, but real provider support remains separately qualified.

#### Comment 5706084083 — Jordan-Hall — 2026-09-16T23:32:46Z

Source: https://github.com/Jordan-Hall/browser/issues/150#issuecomment-5706084083 | Updated: 2026-09-16T23:32:46Z

<!-- intent-core-review:2026-09-16:issue-150 -->
###### Planning review — CORE-04.T06

Parent: #5. No implementation PR was found. Preserve the parent's distinction between negotiated provider resume and runtime-owned reseeding.

**Proposed contract:** an adapter reports supported resume capabilities and a versioned, account-scoped secondary session reference. The supervisor owns the task/checkpoint/artifacts, checks current policy and launches a fresh worker epoch. Resume is permitted only when the adapter can validate the session's provider/account/protocol compatibility and the required evidence remains available.

When resume is unsupported, expired or invalid, construct a new session from the shared goal, explicit constraints, verified evidence and artifacts. Record that it is a reseed/new session, not a continuation of private provider internals. Do not manufacture cross-provider equivalence by copying an opaque transcript or restoring credentials from a checkpoint.

**Coding-specific checks:** validate repository identity, worktree/base revision, uncommitted changes and proposed patch ancestry before applying resumed output. Isolate project hooks/configuration and tool execution before launching the CLI. Retain any uncertain external attempt from the previous session; provider replacement does not authorize retrying it.

**Privacy and fallback:** honor the original offline/local/hybrid mode and allowed data destinations. Unavailable local inference must not silently escalate to a remote provider; a capability declaration is not a user grant.

**Acceptance tests:** supported resume; expired session; wrong account/provider; unsupported version; reseed with unchanged workspace; conflicting repository changes; missing evidence; revoked tool grants; old epoch output; local-only policy with remote fallback configured. Include adapter conformance fixtures, then provider-specific evidence when those adapters exist.

Dependencies: #137/#141 worker authority, #145/#147 recovery decisions and #146 checkpoints. This task defines policy/integration, not four assumed provider implementations.

#### Comment 5706654172 — Jordan-Hall — 2026-09-17T00:40:42Z

Source: https://github.com/Jordan-Hall/browser/issues/150#issuecomment-5706654172 | Updated: 2026-09-17T00:40:42Z

###### Planning review follow-up — CORE-04.T06

Parent: #5. No implementation PR was found. Add a **resume/attach crash boundary** to the existing native-resume versus reseed contract.

Persist the runtime's resume intent and current task/checkpoint binding; keep executable tool authority gated while a provider session is being attached or created. If the provider accepts that operation but the runtime crashes before persisting the returned reference, recovery must identify an orphan/uncertain session rather than blindly create another write-enabled worker.

Test interruption before attach, after provider acknowledgement, before reference commit and before granting the fresh epoch. Repeated attach/creation acknowledgements must not allocate multiple executable task instances. Late old-session events can be retained as checked observations, not new authority.

Use only capabilities the adapter actually negotiates; unsupported lookup/idempotent attach must remain a visible limitation. Core fixtures can validate policy, while real adapter behavior belongs to its separately qualified implementation. Preserve offline/local/hybrid destination restrictions and repository-base checks. Planning review only.


---

<a id="issue-151"></a>
## #151 — [TASK][CORE-04.T07] Expose recovery decisions in the task centre

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/151
**Created:** 2026-09-15T15:15:50Z | **Updated:** 2026-09-17T00:41:16Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #5
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #5

Task ID: `CORE-04.T07`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5706069625 — Jordan-Hall — 2026-09-16T23:31:27Z

Source: https://github.com/Jordan-Hall/browser/issues/151#issuecomment-5706069625 | Updated: 2026-09-16T23:31:27Z

<!-- intent-core-review:2026-09-17:issue-151 -->
###### Review — CORE-04.T07: recovery decisions in the task centre

Parent: #5. No implementation PR was found for this task; these are proposed requirements, not claims of an existing UI defect.

The parent correctly separates resumed, stopped, login-required and uncertain work. Preserve that distinction in a typed recovery view model rather than generating free-form agent status text. Include task/operation identity, plan revision, reason, evidence, last observation time, required user action and currently permitted controls. A stopped worker and a reversed external transaction are different facts.

**Recommended implementation boundary:** the task centre reads runtime-owned recovery records; clicking Resume/Reconcile/Abandon submits an intent with the expected task/plan revision. The supervisor rechecks current policy, account, deadline, source preconditions and operation state. A stale screen or copied control cannot itself authorize a dispatch. Abandon means stop further automatic work, not erase receipts or pretend an uncertain purchase did not happen.

**Important UX cases:** show 'outcome unknown' until provider evidence settles it; distinguish reauthentication from granting new authority; provide original-source/receipt inspection; present pending confirmation in trusted chrome, not generated content. Do not offer a generic Retry control for unknown external writes. Any compensating action is separately described and authorized. Keep diagnostics useful without exposing raw credentials, confidential payloads or unnecessary provider identifiers.

**Acceptance tests:** stale recovery action rejected after revocation or a newer plan; double-click does not start two attempts; login does not automatically approve a purchase; stopped-but-reconciling remains visible; missing evidence is not rendered as success; keyboard/screen-reader access and no-inference operation work. Verify the displayed state survives restart without reusing the provider conversation.

Coordinate #147/#148 for authoritative transitions and the workspace/UI workstream for presentation. A CORE CLI/view-model milestone can precede the native task centre, but it must be labelled as such rather than counted as the complete UI acceptance.

#### Comment 5706088038 — Jordan-Hall — 2026-09-16T23:33:06Z

Source: https://github.com/Jordan-Hall/browser/issues/151#issuecomment-5706088038 | Updated: 2026-09-16T23:33:06Z

<!-- intent-core-review:2026-09-16:issue-151 -->
###### Planning review — CORE-04.T07

Parent: #5. No implementation PR was found. Treat the recovery task centre as a deterministic view over runtime-owned decisions and evidence, not a generated explanation that controls execution.

**Proposed view contract:** stable task/operation/plan IDs, expected revision, status, reason code, evidence references, last observation time and currently available actions. Distinguish resumed, locally stopped, worker termination pending, login required, blocked by policy, failed and uncertain external outcome. A local Cancel acknowledgement must not display a purchase or message as externally reversed.

**Action binding:** Inspect, Resume, Reconcile, Abandon and any compensating action must resolve the current task/plan/account and revalidate authority when invoked. Use expected revisions and deduplication so stale views or double clicks cannot trigger repeated operations. Unknown outcomes should offer read-only reconciliation, not a convenient blind Retry. New side effects/compensation retain their own approval flow.

Keep trusted confirmation controls outside untrusted source/generated content. Explanations may be assisted by AI, but factual status, evidence and allowed controls must remain usable without inference or the originating chat. Preserve source inspection and raw bounded diagnostics without exposing secrets.

**Acceptance tests:** state changes while the view is open; revoked account; double-click/stale action; delayed old worker result; no evidence available; local cancel with an externally accepted request; offline mode; keyboard/screen-reader access and status announcements. A user action must never increase authority just because the screen offered it.

Deliver the view model and host integration tests separately. The current CORE tree has no completed task-centre UI, so a serialized view model alone is not evidence that the finished interface meets this task.

#### Comment 5706658534 — Jordan-Hall — 2026-09-17T00:41:16Z

Source: https://github.com/Jordan-Hall/browser/issues/151#issuecomment-5706658534 | Updated: 2026-09-17T00:41:16Z

###### Planning review follow-up — CORE-04.T07

Parent: #5. No implementation PR was found. Add **accepted command versus observed outcome** to the task-centre state model.

When Resume/Stop/Reconcile is durably accepted but its acknowledgement or refreshed view is lost, the interface should expose an operation/request identity and an observation-pending state, not falsely report that nothing happened. Reconnect/retry must query that command's outcome before offering another execution.

Test a click whose server-side transition commits while the UI connection closes; reopening the workspace must recover the same command and current plan revision. A repeated click must not create another attempt, and a newer result must invalidate stale offered controls. Keep this usable without inference.

Coordinate #130/#131's transaction/result contracts and #147's barrier rather than inventing UI-only success flags. Preserve the existing separation among locally stopped, externally uncertain and compensated. These are additional acceptance cases; neither a serialized view model nor this review proves that the native recovery UI is implemented.


---

<a id="issue-152"></a>
## #152 — [TASK][CORE-04.T08] Qualify restart, suspend and corrupt-state cases

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/152
**Created:** 2026-09-15T15:15:56Z | **Updated:** 2026-09-17T00:41:46Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #5
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #5

Task ID: `CORE-04.T08`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (3 comments)

#### Comment 5706062029 — Jordan-Hall — 2026-09-16T23:30:46Z

Source: https://github.com/Jordan-Hall/browser/issues/152#issuecomment-5706062029 | Updated: 2026-09-16T23:30:46Z

<!-- intent-core-review:2026-09-17:issue-152 -->
###### Review — CORE-04.T08: restart, suspend and corrupt-state qualification

Parent: #5. This is a planning/acceptance review: no implementation PR for this task was found in the current CORE stack. The parent proposal correctly requires convergence evidence rather than a successful restart demonstration.

###### Make the test oracle independent of the runtime
Use a resettable external-service fixture with its own durable effect ledger. Compare that ledger with the runtime's operation, attempt, receipt and checkpoint records after every interruption. An agent reporting success, a surviving process, or an unchanged local database is not the oracle for an external purchase/message.

###### Required interruption matrix
- Kill the supervisor/worker before outbox commit, after commit, after durable attempt creation, after the fixture accepts the request, and before the result is persisted. Accepted-but-unrecorded effects must remain discoverable and reconcile; they must not be blindly resent.
- Resume after account revocation, expired approval, changed source preconditions, elapsed deadlines and backwards/forwards wall-clock movement. Re-evaluate current authority before creating a fresh worker epoch.
- Restore with a missing or suppressed artifact, modified migration ledger, unsupported schema, unavailable local model and unavailable provider resume capability. Distinguish blocked, login-required, repair-required and still-uncertain states.
- Interrupt local writes between staging/publication/metadata commits and test disk-full/denied-write conditions during recovery itself. Recovery failure must not enable ordinary dispatch.
- Test repeated recovery of the same checkpoint and late responses from the pre-crash worker. Neither may manufacture a second operation or regain revoked authority.

###### Evidence and scope
Persist fixture seed, exact tested commit, dependency/toolchain/OS/filesystem identity, interruption point, pre/post state, external effect count and expected/actual disposition. Run deterministic state-machine cases on each PR and real process/platform cases separately. Process termination is not evidence of power-loss durability; report actual VM/power-loss runs distinctly from simulated failures.

Link coverage to #136 (storage), #144 (supervisor), #147 (startup planner), #148 (reconciliation) and #149 (production-free replay). The current outbox findings on #795 and failing retention behavior on #801 should become regression seeds. Keep this issue open until the implemented paths and those failure cases actually pass; this comment does not claim qualification or fixes.

#### Comment 5706091580 — Jordan-Hall — 2026-09-16T23:33:25Z

Source: https://github.com/Jordan-Hall/browser/issues/152#issuecomment-5706091580 | Updated: 2026-09-16T23:33:25Z

<!-- intent-core-review:2026-09-16:issue-152 -->
###### Planning review — CORE-04.T08

Parent: #5. No implementation PR was found. This should be the restart/recovery release gate over #145–151 and the repaired storage/supervisor stack.

**Scenario matrix:** process death before/after checkpoint commit, dispatch start, external fixture commit and result recording; supervisor death during cancellation; sleep/wake with elapsed deadlines; changed wall clock; unavailable model/provider; revoked account; missing/corrupt/suppressed blobs; migration mismatch; restored backup with stale authority; and repeated recovery of the same interrupted task.

**Oracle:** compare durable task/graph/operation/attempt records, artifact references, current grants, worker epochs and authoritative fixture effects. Assert no duplicate irreversible fixture action, no restored stale grant, no false success/rollback, and no loss of uncertainty. Include the compensating-attempt lineage from #130 and stale-plan application from #147.

Run deterministic failpoint tests for coverage, real subprocess tests for lifecycle, and declared VM/platform tests for actual suspend/power-loss behavior. A fake clock test is not evidence that OS suspend/resume works; a process kill is not storage power-loss qualification. Reuse #136 and #144 fixtures rather than maintaining contradictory oracles.

**Evidence requirements:** case ID/seed, tested head and dependency lock, OS/hardware/filesystem profile, expected/actual convergence, resource/latency observations, and preserved failure artifacts. Emit reports on failure and never convert repeated flaky retries into a passing release claim.

Gate supported scenarios individually. When a required connector or UI is not implemented, report that integration as not qualified rather than substituting a mock and labelling the whole product production-ready. Cross-link the final integrated result to #213/#214 and retain review findings until tested fixes or explicit scoped decisions exist.

#### Comment 5706662355 — Jordan-Hall — 2026-09-17T00:41:46Z

Source: https://github.com/Jordan-Hall/browser/issues/152#issuecomment-5706662355 | Updated: 2026-09-17T00:41:46Z

###### Planning review follow-up — CORE-04.T08

Parent: #5. No implementation PR was found. Tighten the independent-oracle requirement: the **external fixture ledger must remain outside the snapshot/rollback domain of the runtime being tested**.

If a VM restore rolls back both the runtime and the simulated merchant/message ledger, a duplicate-send bug can disappear from the evidence. Keep the fixture's accepted effect durable across runtime restore, then verify that the restored client reconciles it instead of issuing it again.

Add repeated restore of the same checkpoint, lost result acknowledgement, late original/compensation observations, an old UI command replayed after restore, and missing checkpoint-reference rows. Compare the unchanged external ledger with fresh runtime epochs and current authority, not only the restored local database.

Retain minimal counterexample traces and a coverage manifest that fails on omitted required cases. Label simulated faults, real process kills and actual VM/power-loss runs separately. The review's local SQL reductions are regression seeds, not evidence that this qualification is complete. Link eventual results to #213/#214.


---

<a id="issue-211"></a>
## #211 — [TASK][EPIC-CORE.T01] Agree runtime ownership and wire contracts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/211
**Created:** 2026-09-15T15:24:07Z | **Updated:** 2026-09-17T00:43:58Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #13
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #13

Task ID: `EPIC-CORE.T01`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706124394 — Jordan-Hall — 2026-09-16T23:36:09Z

Source: https://github.com/Jordan-Hall/browser/issues/211#issuecomment-5706124394 | Updated: 2026-09-16T23:36:09Z

<!-- intent-core-review:2026-09-17:issue-211 -->
###### Review — EPIC-CORE.T01: ownership and wire-contract agreement

Parent: #13. This should ratify the implemented boundaries from #2–#5, not freeze inconsistent drafts merely because the types compile.

###### Contract conflicts to settle
- #787 permits an independently supplied arguments hash on deserialization; the authorized proposal, stored argument artifact and eventual dispatch must agree on exact bytes and account/capability/target.
- #790 negotiates versions not used by the actual V1-only envelope codec. Advertise only implemented formats and bind the negotiated version to the authenticated session.
- #794's durable operation states and the domain Operation/Receipt schema need an explicit lossless mapping. Accepted is not verified; original and compensating attempts must retain their identity and evidence.
- #793's private Connection is not a profile-wide ownership lock. State, artifact publication, holds, GC and future backups must share a declared coordination boundary.
- #789's creator-process socketpair evidence does not authenticate an inherited child endpoint. Require real launched-worker evidence, fresh one-use bootstrap and role/instance scope.

###### Required inventory
For every state family and message, specify the sole authoritative writer, readers, trust class, accepted schema versions, byte/count limits, credential/data exposure, lifetime and invalidation rule. Keep resource identifiers separate from grants, protocol support separate from authority, and provider session IDs separate from durable task identity. Document where the small platform FFI boundary is unavoidable rather than making a blanket zero-unsafe claim.

###### Acceptance
A real two-worker handshake and bounded framed exchange must exercise those contracts end to end. Include wrong role/account, stale epoch, reused bootstrap, incompatible payload version, unknown authority-bearing field and cross-scope artifact cases. Add a schema/ownership compatibility matrix and actual codec fixtures, not just trait signatures or synthetic authentication values.

Coordinate #113 and the security ownership/threat-model work without creating duplicate truths. The agreement should identify unresolved decisions explicitly; it is not a sign-off while the referenced blockers remain. Review only; no implementation was changed.

#### Comment 5706679352 — Jordan-Hall — 2026-09-17T00:43:58Z

Source: https://github.com/Jordan-Hall/browser/issues/211#issuecomment-5706679352 | Updated: 2026-09-17T00:43:58Z

###### Review follow-up — EPIC-CORE.T01

Parent: #13. Extend the ownership agreement with a **producer/consumer contract matrix generated from executable fixtures** rather than a second prose-only schema inventory.

For each record/message, identify who creates it, which codec/version transports it, who validates it, which authority transition it may request, and who owns the durable outcome. Check encode/decode closure (#124), no-op migration validation (#126), immutable action binding (#123), launch-spec/channel binding (#125/#137), and committed-result semantics (#130/#131) across those boundaries.

Require each advertised version/error representation and important value boundary to appear in a fixture exercised by both ends. Successful deserialization remains data acceptance—not approval or dispatch authority. A hash mismatch or unavailable version must not be normalized away to make the integration work.

Coordinate this inventory with #113; do not create two independent sources of truth. The current PR findings still need fixes and integration evidence before ratification. This is a review follow-up, not sign-off or implementation.


---

<a id="issue-212"></a>
## #212 — [TASK][EPIC-CORE.T02] Integrate durable dispatch and worker supervision

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/212
**Created:** 2026-09-15T15:24:15Z | **Updated:** 2026-09-17T00:44:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #13
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#807](https://github.com/Jordan-Hall/browser/pull/807), [#809](https://github.com/Jordan-Hall/browser/pull/809), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #13

Task ID: `EPIC-CORE.T02`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706119870 — Jordan-Hall — 2026-09-16T23:35:46Z

Source: https://github.com/Jordan-Hall/browser/issues/212#issuecomment-5706119870 | Updated: 2026-09-16T23:35:46Z

<!-- intent-core-review:2026-09-17:issue-212 -->
###### Review — EPIC-CORE.T02: durable dispatch and worker supervision

Parent: #13. No integration PR was found. The individual libraries should not be called an integrated runtime until their authority and durability boundaries are exercised together.

###### Define one dispatch gate
The proposed integration should resolve the task, action proposal, exact account/capability/target, immutable argument artifact/hash, source preconditions, current approval, deadline, budget and live worker epoch before granting an external attempt. Negotiated protocol capabilities are not authorization. The durable outbox's free-form lease-owner string must not substitute for the supervisor's instance/generation identity.

Make a committed attempt the only way to obtain dispatch material. #795 currently exposes destination/payload on the earlier claim/load object, and #794 permits changing an attempt identity while recording its result. Close these API gaps rather than adding a second 'correct' orchestration path alongside bypassable methods.

###### Concurrency and cancellation
Specify the linearization point shared by cancellation/revocation and dispatch admission. A lease revoked before that point denies new dispatch; an effect admitted before it may already be external and must retain its attempt/reconciliation state. Do not claim that process termination can undo a request accepted by a provider. Authenticate result events to the corresponding instance/attempt and reject late events as authority while retaining useful outcome evidence through a controlled reconciliation path.

###### Tests and observability
Use actual subprocesses plus the fixture service. Saturate progress and storage work; issue Stop; verify bounded local acknowledgement, revoked future dispatch, eventual worker handling and exact external effect count. Include expired/reclaimed leases, a stale worker with the same display name, account mismatch, approval change, duplicate events, budget exhaustion and a crash after send. Demonstrate successful permitted work too.

Link results to #125/#127, #130/#131/#132, #137–#144 and #147/#148. Publish task/PR/head/test links on the parent. A roll-up should add integration wiring and evidence, not duplicate the contracts or journal ownership. Review only; no code, permissions or issue state changed.

#### Comment 5706683959 — Jordan-Hall — 2026-09-17T00:44:33Z

Source: https://github.com/Jordan-Hall/browser/issues/212#issuecomment-5706683959 | Updated: 2026-09-17T00:44:33Z

###### Review follow-up — EPIC-CORE.T02

Parent: #13. Require a **closed-loop result path**, not just successful launch and dispatch wiring: validated action → durable attempt → one fixture effect → correlated result observation → local projection/receipt → stable task-centre outcome.

Add a lost-acknowledgement variant at each boundary. Repeating a command or result must resolve the same identity rather than creating another effect. Include two accounts using identical argument bytes so a successful result from one cannot complete the other's operation. Keep stale worker evidence distinguishable from current execution authority.

The new follow-ups on #130/#131/#138/#139/#141/#143/#147 define the needed committed-outcome, lifecycle ordering, atomic reservation, revocation-failure, yield-correlation and startup-barrier contracts. Enforce those in the owning primitives instead of implementing an integration-only workaround.

Qualification must show both one successful permitted effect and zero forbidden effects; a runtime that rejects everything is not a passing integration. No integration PR was found, and this review does not claim one exists or that the stack is production-ready.


---

<a id="issue-213"></a>
## #213 — [TASK][EPIC-CORE.T03] Exercise crash and bounded replay end to end

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/213
**Created:** 2026-09-15T15:24:30Z | **Updated:** 2026-09-17T00:45:10Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #13
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#809](https://github.com/Jordan-Hall/browser/pull/809), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #13

Task ID: `EPIC-CORE.T03`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706113005 — Jordan-Hall — 2026-09-16T23:35:13Z

Source: https://github.com/Jordan-Hall/browser/issues/213#issuecomment-5706113005 | Updated: 2026-09-16T23:35:13Z

<!-- intent-core-review:2026-09-17:issue-213 -->
###### Review — EPIC-CORE.T03: crash and bounded replay end to end

Parent: #13. This epic task should qualify the integrated path from #3/#4/#5, not create another recovery implementation.

**Required demonstration:** create a runtime-owned task with a source artifact; persist the operation and outbox; launch/authenticate a worker; interrupt it at a controlled dispatch boundary; restart the supervisor; inspect the same workspace, task and attempt history; reconcile against a separate fixture service; replay only captured/fixture observations.

Use an external fixture with a durable ledger and controllable response loss. Kill points must include (1) before local commit, (2) after outbox commit but before dispatch, (3) after attempt commit, (4) after the fixture accepts the effect but before the response, and (5) after response receipt but before result commit. The last three cannot be called safely unsent merely because the worker died.

**Cross-component assertions:** one stable operation identity and explicit attempt lineage; no duplicate fixture effect; no authority resurrected from the checkpoint; late old-worker output denied; missing/suppressed artifacts reported; pending/accepted/verified/unknown stay distinct; repeated recovery converges without relying on conversation history. A successful compensation adds a new authorized action and receipt rather than erasing the original action.

Run a second scenario with a safe read and a third with a conflicting local file edit. This prevents an implementation from 'passing' by permanently blocking everything. Run replay in an environment without production credentials or write adapters and assert network/desktop/host-file escape attempts are denied.

**Current blockers to seed as regressions:** abandoned attempting outboxes and cancelled-row starvation (#795), substituted attempt identities/compensation reconciliation (#794), incomplete real process authentication (#789), and retention failure (#801). Keep Rust subprocess, simulated fault and actual VM/power-loss results clearly labelled.

Acceptance evidence belongs in this issue and its future integration PR: exact commit, fixtures, kill point, expected/actual durable state, external ledger and replay result. No end-to-end implementation or passing result is claimed here.

#### Comment 5706688902 — Jordan-Hall — 2026-09-17T00:45:10Z

Source: https://github.com/Jordan-Hall/browser/issues/213#issuecomment-5706688902 | Updated: 2026-09-17T00:45:10Z

###### Review follow-up — EPIC-CORE.T03

Parent: #13. Add the new cross-cutting regressions to the existing end-to-end scenario, without creating another recovery implementation.

Run the same accepted-but-unrecorded fixture action through: a lost local acknowledgement (#130/#131), supervisor cancellation with failed persistence (#141), a checkpoint missing one dependency row (#146), and restoration of an older runtime snapshot while the external fixture ledger remains current (#152). Expected outcomes differ; none may silently produce a second action.

Validate replay completeness (#149): dropping the capture's final event must make the scenario incomplete/divergent, not green. Validate artifact protection using real reference rows (#133), not a mocked successful pin. Keep the receipt/task state inspectable with inference disabled.

The test ledger and its oracle must not roll back with the runtime VM. Record the exact head, kill point, local committed revision, worker epoch, expected versus observed result and external effect count. No integration PR or passing end-to-end run was found; this is a qualification refinement, not completion evidence.


---

<a id="issue-214"></a>
## #214 — [TASK][EPIC-CORE.T04] Publish runtime operational readiness

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/214
**Created:** 2026-09-15T15:24:34Z | **Updated:** 2026-09-17T00:46:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #13
**PRs mentioning this issue:** [#802](https://github.com/Jordan-Hall/browser/pull/802), [#805](https://github.com/Jordan-Hall/browser/pull/805), [#806](https://github.com/Jordan-Hall/browser/pull/806), [#810](https://github.com/Jordan-Hall/browser/pull/810) (text references, not verified implementation or acceptance)

### Original description

Parent: #13

Task ID: `EPIC-CORE.T04`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (2 comments)

#### Comment 5706106593 — Jordan-Hall — 2026-09-16T23:34:41Z

Source: https://github.com/Jordan-Hall/browser/issues/214#issuecomment-5706106593 | Updated: 2026-09-16T23:34:41Z

<!-- intent-core-review:2026-09-17:issue-214 -->
###### Review — EPIC-CORE.T04: operational readiness

Parent: #13. The epic proposal calls for a pinned support matrix and fault/concurrency reports. Make this a verifiable release gate, not a documentation-only completion task.

**Current evidence gap:** the CORE stack has PRs for CORE-01.T01–T08 and CORE-02.T01–T06, but no PRs were found for CORE-02.T07/T08 or CORE-03/04. Existing review findings on #787–#795 remain material; #801's recorded CI fails retention tests. A green conformance smoke report cannot approve the whole runtime.

###### Proposed readiness manifest
For each task, record issue/PR, reviewed head SHA, integrated head SHA, acceptance-test IDs, CI run, platform/filesystem, dependency-lock and fixture digests, result, and unresolved findings. Separate source review, unit tests, subprocess tests, VM/power-loss qualification and live-provider evidence. An earlier green head is not evidence for a later changed head.

Publish actual support for profile ownership, IPC authentication, process-tree termination, resource limits, file publication/durability, backup restore, cancellation latency and degraded operation. Distinguish hard OS enforcement from admission estimates/cooperative GPU yields. List unsupported combinations explicitly; do not report an untested platform as supported because its code is conditionally compiled.

###### Acceptance additions
- Every CORE task maps to code plus executable evidence; roll-up PRs add real integration coverage rather than duplicate feature implementations.
- Re-run the full integrated stack after review fixes and verify no stacked PR silently lost a prerequisite.
- Every actionable top-level/inline finding has a disposition with a fix commit and test, or a precise reason for not changing it. A reply alone does not make a defect fixed.
- Exercise recovery/operator runbooks from clean installations and damaged fixtures; retain failure reports as well as passing reports.
- Provide local stop acknowledgement and eventual process/provider termination as separate measurements.

Coordinate #128/#136/#144/#152 and #211–#213. No approval, CI success, implementation completion or thread resolution is asserted by this review comment.

#### Comment 5706695627 — Jordan-Hall — 2026-09-17T00:45:59Z

Source: https://github.com/Jordan-Hall/browser/issues/214#issuecomment-5706695627 | Updated: 2026-09-17T00:45:59Z

###### Review follow-up — EPIC-CORE.T04

Parent: #13. Add a **base/head integration identity** to the readiness manifest. A stacked PR's source review and CI evidence apply to particular commits; changing an ancestor/base can change the integrated result even when the child's head or headline looks unchanged.

For each task record base SHA, head SHA, tested merge/integration SHA, fixing commits, acceptance-case results and unresolved finding dispositions. When repairing an earlier PR, propagate the correction through affected descendants and rerun their applicable tests. Verify that a roll-up does not silently retain the old vulnerable implementation.

A `Fixed` reply requires matching source plus regression evidence. `Not changed` requires a technical reason and an explicit affected support/completion claim; it is not automatic acceptance of a release blocker. Preserve earlier legitimate fixes on #796/#797 while tracking the additional findings separately.

The current inventory still has no CORE-02.T07/T08 or CORE-03/04 implementation PRs, and #801's inspected CI fails retention tests. Review completion must therefore remain separate from implementation, merge and operational-readiness completion. This pass adds comments only.


---

