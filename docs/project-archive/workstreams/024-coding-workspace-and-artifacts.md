# Coding workspace and artifacts

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 28

## Contents

- [#24 — EPIC: Coding workspace and artifacts](#issue-24)
- [#72 — [P2][CODE-01] Repositories, worktrees, tests and patch review](#issue-72)
- [#73 — [P1][CODE-02] Safe files, process tools and artifact generation](#issue-73)
- [#74 — [P2][CODE-03] Action journal, undo and compensation UI](#issue-74)
- [#255 — [TASK][EPIC-CODE.T01] Ratify project and artifact ownership](#issue-255)
- [#256 — [TASK][EPIC-CODE.T02] Integrate isolated edits, builds and review](#issue-256)
- [#257 — [TASK][EPIC-CODE.T03] Integrate generated files and action recovery](#issue-257)
- [#258 — [TASK][EPIC-CODE.T04] Qualify project integrity and cleanup](#issue-258)
- [#565 — [TASK][CODE-01.T01] Register projects and protected paths](#issue-565)
- [#566 — [TASK][CODE-01.T02] Allocate isolated worktrees and snapshots](#issue-566)
- [#567 — [TASK][CODE-01.T03] Implement patch capture and conflict detection](#issue-567)
- [#568 — [TASK][CODE-01.T04] Build isolated test and tool execution](#issue-568)
- [#569 — [TASK][CODE-01.T05] Review dependencies and protected changes](#issue-569)
- [#570 — [TASK][CODE-01.T06] Implement code/diff/test review and apply](#issue-570)
- [#571 — [TASK][CODE-01.T07] Qualify multi-agent coding and cleanup](#issue-571)
- [#572 — [TASK][CODE-02.T01] Implement path and file-handle authorization](#issue-572)
- [#573 — [TASK][CODE-02.T02] Implement scoped read/create/write operations](#issue-573)
- [#574 — [TASK][CODE-02.T03] Implement rename/move/copy/delete with previews](#issue-574)
- [#575 — [TASK][CODE-02.T04] Implement bounded process sessions](#issue-575)
- [#576 — [TASK][CODE-02.T05] Build format-aware generation and validation](#issue-576)
- [#577 — [TASK][CODE-02.T06] Implement safe previews and opening](#issue-577)
- [#578 — [TASK][CODE-02.T07] Integrate recovery, auditing and file workflows](#issue-578)
- [#579 — [TASK][CODE-03.T01] Define action history and reversibility metadata](#issue-579)
- [#580 — [TASK][CODE-03.T02] Implement local file and layout restoration](#issue-580)
- [#581 — [TASK][CODE-03.T03] Implement external compensation planning](#issue-581)
- [#582 — [TASK][CODE-03.T04] Route compensation through trusted transactions](#issue-582)
- [#583 — [TASK][CODE-03.T05] Build the history and recovery UI](#issue-583)
- [#584 — [TASK][CODE-03.T06] Qualify undo across concurrency and restart](#issue-584)

---

<a id="issue-24"></a>
## #24 — EPIC: Coding workspace and artifacts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/24
**Created:** 2026-09-15T12:08:02Z | **Updated:** 2026-09-15T14:24:03Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #1 | **Body-declared parent:** #1
**Native child issues:** #72, #73, #74

### Original description

Programme: #1

Own repository/project workspaces, worktree isolation, code diff/review/test flows and user-facing artifact/file creation.

#### Child issues
- [ ] #72 CODE-01 — Repositories, worktrees, tests and patch review
- [ ] #73 CODE-02 — Safe files, process tools and artifact generation
- [ ] #74 CODE-03 — Action journal, undo and compensation UI

#### Cross-cutting gates
Agents operate on staged/scoped projects, independent tests verify changes, dependencies/network are reviewed, protected runtime/policy code cannot self-deploy agent-generated changes, and generated files remain untrusted until opened/executed safely.

### Discussion (1 comments)

#### Comment 5681886580 — Jordan-Hall — 2026-09-15T14:24:03Z

Source: https://github.com/Jordan-Hall/browser/issues/24#issuecomment-5681886580 | Updated: 2026-09-15T14:24:03Z

<!-- intent-implementation-v1:EPIC-CODE -->
###### Workstream implementation and integration tasks

Integrate #72–#74 as the common coding/file workflow for local and external agents.

- [ ] **EPIC-CODE.T01 — Ratify project/artifact ownership.** Agree allowed roots, protected paths, base revisions, patch provenance and format validators. **Proof:** selecting a repository does not expose unrelated home data or writable shared Git metadata.
- [ ] **EPIC-CODE.T02 — Integrate edits/builds/review.** Connect isolated worktrees, bounded process runners, dependency diffs, independent tests and revision-checked apply. **Proof:** concurrent agents cannot overwrite each other or apply a patch to a changed base without conflict review.
- [ ] **EPIC-CODE.T03 — Integrate files/action recovery.** Generate validated artifacts, preview safely, restore local versions and prepare external compensation through the transaction service. **Proof:** generated documents/scripts cannot execute through privileged previews; compensation is a new action.
- [ ] **EPIC-CODE.T04 — Qualify integrity/cleanup.** Test malicious build scripts, traversal/reparse races, concurrent user edits, interrupted writes and temporary-state cleanup. **Proof:** source projects and unresolved evidence survive failures while staged secrets are removed.

**Demonstration:** two providers edit separate project copies; the user changes the base; tests run independently; conflicts are reviewed; an accepted local patch can be restored.

**Closure gate:** code, file and recovery integration evidence. Agent-authored tests alone do not replace protected acceptance tests, and stopping a task cannot undo a remote publication or purchase.


---

<a id="issue-72"></a>
## #72 — [P2][CODE-01] Repositories, worktrees, tests and patch review

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/72
**Created:** 2026-09-15T12:15:52Z | **Updated:** 2026-09-15T19:43:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #24 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #24

#### Objective
Provide a safe shared coding workspace for local/external agents: scoped repositories, isolated worktrees/builds, deterministic tests and human-reviewable patches.

#### Scope
- Repository/project registration with allowed roots and protected paths.
- Per-agent/task worktrees or equivalent isolated mutable copies.
- Editor/diff/patch artifact model and merge/rebase conflict workflow.
- Isolated build/test/lint/typecheck runners with resource/network/dependency policy.
- Dependency change detection and review.
- Independent test execution after provider reports completion.
- Patch review UI with provenance by agent/task/provider.
- Merge/apply operation through scoped write capability.

#### Security rules
- Protected runtime/policy/security paths can be configured read-only or require stronger review.
- Builds never inherit ambient host secrets/network.
- Agent output is untrusted until verified.

#### Acceptance criteria
- [ ] Patch applies cleanly to the intended base or produces explicit conflicts.
- [ ] Independent tests execute in the permitted isolated environment.
- [ ] Parallel agents cannot silently overwrite each other's worktrees.
- [ ] Protected paths remain unchanged without an explicit stronger grant.
- [ ] New dependencies/network requests are visible and policy-controlled.
- [ ] Review shows diff, tests, provenance and verification state before merge.

#### Dependencies
- CORE-02
- SEC-04

**First phase:** P2  
**Maturity target:** P3  
**Owner:** harness-pc-providers

### Discussion (2 comments)

#### Comment 5682255234 — Jordan-Hall — 2026-09-15T14:43:48Z

Source: https://github.com/Jordan-Hall/browser/issues/72#issuecomment-5682255234 | Updated: 2026-09-15T14:43:48Z

<!-- intent-implementation-v1:CODE-01 -->
###### Implementation proposal — CODE-01

Implement a common project/patch/test/review service over #3/#9. ProjectGrant, RepositorySnapshot, WorktreeLease, PatchArtifact and ApplyProposal preserve exact roots, base revisions and provenance.

- [ ] **CODE-01.T01 — Project registration/protection.** Grant explicit roots/repositories and protected paths; resolve actual objects with safe handle-based paths rather than string prefixes. **Verify:** nested links or similarly named directories cannot extend access.
- [ ] **CODE-01.T02 — Isolated worktrees/snapshots.** Allocate per-task/agent copies from a known base while preserving uncommitted user work and scoping shared Git metadata. **Verify:** agents cannot mutate another worktree or the user's main checkout through `.git` references.
- [ ] **CODE-01.T03 — Patch/conflict artifacts.** Capture hashes, rename/delete operations, base and provenance; compare with current user state before apply. **Verify:** stale or competing patches surface conflicts rather than overwrite.
- [ ] **CODE-01.T04 — Isolated builds/tests.** Run explicit executable/args/cwd/environment with bounded resources and dependency/egress policy; store bounded output artifacts. **Verify:** build scripts cannot use ambient host secrets or unapproved networking.
- [ ] **CODE-01.T05 — Dependency/protected-change review.** Detect lockfile/package/build-script and protected-runtime changes, showing new execution/network implications. **Verify:** agent modifications to policy/harness require the normal reviewed development path.
- [ ] **CODE-01.T06 — Review/apply.** Display code diff, base, provider provenance, independent tests and unresolved risks; apply via a revision-checked capability and retain a reversible checkpoint. **Verify:** passing tests for an old base cannot authorize a different patch.
- [ ] **CODE-01.T07 — Multi-agent/cleanup qualification.** Test conflicts, failures, crashes, cancellation and provider switches; retain needed evidence while cleaning disposable state. **Verify:** cleanup never removes the user's changes or unresolved artifacts.

**Review correction:** Git worktrees separate working files but may share writable metadata. Use confinement or independent copies where stronger isolation is needed. Agent-authored tests supplement protected independent acceptance tests; a provider reporting success is not a verified change.

#### Comment 5687090227 — Jordan-Hall — 2026-09-15T19:43:40Z

Source: https://github.com/Jordan-Hall/browser/issues/72#issuecomment-5687090227 | Updated: 2026-09-15T19:43:40Z

###### Task issues
- [ ] #565 `CODE-01.T01` — Register projects and protected paths
- [ ] #566 `CODE-01.T02` — Allocate isolated worktrees and snapshots
- [ ] #567 `CODE-01.T03` — Implement patch capture and conflict detection
- [ ] #568 `CODE-01.T04` — Build isolated test and tool execution
- [ ] #569 `CODE-01.T05` — Review dependencies and protected changes
- [ ] #570 `CODE-01.T06` — Implement code/diff/test review and apply
- [ ] #571 `CODE-01.T07` — Qualify multi-agent coding and cleanup


---

<a id="issue-73"></a>
## #73 — [P1][CODE-02] Safe files, process tools and artifact generation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/73
**Created:** 2026-09-15T12:16:03Z | **Updated:** 2026-09-15T19:44:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #24 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #24

#### Objective
Give agents and users first-class file/process capabilities now—without waiting for an OS—through scoped, typed and recoverable operations.

#### Scope
- File capabilities: read/write/create/open/rename/move/copy/delete with project/directory/object scope.
- Atomic save where possible, version history and preview before destructive changes.
- Format-aware artifact workers for common documents, archives, code/config and media metadata.
- Bounded process/terminal execution with cwd, env, executable, args, timeout, network and filesystem policy.
- Generated artifact validation and MIME/type sniffing.
- Safe opening/launching of generated/downloaded artifacts.
- Process output/artifact capture into task journal.

#### Security rules
- Terminal availability does not imply universal shell authority.
- Generated files, macros, scripts and packages are untrusted even when created by our model.
- Dependency installation/network access is a separate capability.

#### Acceptance criteria
- [ ] File operations outside granted roots fail closed.
- [ ] Destructive operations are versioned/previewed where supported and correctly classified for undo.
- [ ] Process runners receive clean scoped environments and enforce timeout/resource/network policy.
- [ ] Generated artifacts validate before publication/opening.
- [ ] Macro/script/package content cannot execute through a privileged preview path.
- [ ] File/process actions survive crash recovery with unambiguous journal state.

#### Dependencies
- SEC-02
- SEC-04
- CORE-02

**First phase:** P1  
**Maturity target:** P4  
**Owner:** harness-pc-providers

### Discussion (2 comments)

#### Comment 5682260905 — Jordan-Hall — 2026-09-15T14:44:06Z

Source: https://github.com/Jordan-Hall/browser/issues/73#issuecomment-5682260905 | Updated: 2026-09-15T14:44:06Z

<!-- intent-implementation-v1:CODE-02 -->
###### Implementation proposal — CODE-02

Build scoped file/process brokers over #7/#9/#3. File creation is part of the desktop product now, not dependent on OS work. A terminal displays a scoped ProcessSpec; it does not grant universal shell access.

- [ ] **CODE-02.T01 — Paths/handles.** Resolve selected roots using platform-safe handles, enforce symlink/reparse/hardlink policies and validate archive/output paths. **Verify:** time-of-check/time-of-use races and traversal cannot redirect writes outside the grant.
- [ ] **CODE-02.T02 — Read/create/write.** Stream bounded reads; write temporary same-filesystem files, flush and atomically replace where supported, recording before/after versions. **Verify:** interruption and concurrent user edits do not produce silent partial overwrite.
- [ ] **CODE-02.T03 — Rename/move/copy/delete.** Validate both endpoint grants, preview destructive effects and retain history/trash according to policy. **Verify:** destination conflicts and cross-volume limitations are explicit rather than claimed atomic.
- [ ] **CODE-02.T04 — Process sessions.** Launch approved executables with exact args/cwd/sanitized environment, time/resource/egress constraints and child-process tracking. **Verify:** bounded I/O and termination prevent orphan processes or log-exhaustion attacks.
- [ ] **CODE-02.T05 — Format-aware artifacts.** Generate supported documents/archives/config/code/media through isolated workers; validate MIME/structure and record generator/source versions. **Verify:** invalid outputs remain quarantined and cannot be advertised as valid deliverables.
- [ ] **CODE-02.T06 — Preview/open.** Use restricted viewers for active/untrusted formats; separate viewing from external launch/execution grants and disclose origin/type. **Verify:** scripts/macros/packages cannot execute through a privileged preview path.
- [ ] **CODE-02.T07 — Journal/recovery integration.** Record operation/artifact versions, recover interrupted writes and expose restore/compensation via #74. **Verify:** a source-backed research artifact can be created in an approved folder and reopened after failure without broader file access.

**Implementation boundaries:** string-prefix checks are insufficient path authorization; generated content is still untrusted; a package install requires separate dependency/network authority. Closure needs platform-specific race tests, artifact validation and crash recovery evidence, not a successful file-write demonstration alone.

#### Comment 5687098733 — Jordan-Hall — 2026-09-15T19:44:23Z

Source: https://github.com/Jordan-Hall/browser/issues/73#issuecomment-5687098733 | Updated: 2026-09-15T19:44:23Z

###### Task issues
- [ ] #572 `CODE-02.T01` — Implement path and file-handle authorization
- [ ] #573 `CODE-02.T02` — Implement scoped read/create/write operations
- [ ] #574 `CODE-02.T03` — Implement rename/move/copy/delete with previews
- [ ] #575 `CODE-02.T04` — Implement bounded process sessions
- [ ] #576 `CODE-02.T05` — Build format-aware generation and validation
- [ ] #577 `CODE-02.T06` — Implement safe previews and opening
- [ ] #578 `CODE-02.T07` — Integrate recovery, auditing and file workflows


---

<a id="issue-74"></a>
## #74 — [P2][CODE-03] Action journal, undo and compensation UI

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/74
**Created:** 2026-09-15T12:16:15Z | **Updated:** 2026-09-15T19:45:03Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #24 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #24

#### Objective
Give users one coherent history and recovery model across local edits and external actions while accurately distinguishing true undo from compensating operations and irreversible effects.

#### Scope
- Unified action journal referencing operation IDs, artifacts, source state, approvals and receipts.
- Local reversible actions: file/layout/version restoration.
- External compensatable actions: cancellation, deletion, refund/reversal requests as new transactions.
- Irreversible/uncertain action states with explicit next steps.
- User-facing diff/history timeline and “what can be undone?” metadata.
- Compensation authorization/verification through TX-01/TX-02.
- Preserve immutable receipts/audit where required while reverting local presentation/state.

#### Product rules
- Compensation is not rollback and may fail or require new approval.
- Old approvals cannot be reused for materially different compensation actions.
- UI must never imply that stopping a task reversed an accepted external action.

#### Acceptance criteria
- [ ] Every supported action declares reversible / compensatable / irreversible-or-uncertain behavior.
- [ ] Local file/layout restore reproduces a prior valid version.
- [ ] External compensation creates a new proposal/approval/receipt flow.
- [ ] Stale approval cannot authorize compensation automatically.
- [ ] Uncertain states remain visibly unresolved until verified/reconciled.
- [ ] Journal remains coherent across crash/restart and source refresh.

#### Dependencies
- CORE-02
- TX-01

**First phase:** P2  
**Maturity target:** P4  
**Owner:** harness-pc-providers

### Discussion (2 comments)

#### Comment 5682267426 — Jordan-Hall — 2026-09-15T14:44:27Z

Source: https://github.com/Jordan-Hall/browser/issues/74#issuecomment-5682267426 | Updated: 2026-09-15T14:44:27Z

<!-- intent-implementation-v1:CODE-03 -->
###### Implementation proposal — CODE-03

Implement one action history with truthful recovery semantics over #3/#78/#79. Distinguish a reversible local change, externally compensatable action, irreversible effect and unknown outcome.

- [ ] **CODE-03.T01 — History/reversibility records.** Store operation/account/source, before/after artifact versions, approvals, receipts and recovery classification; separate original time from current recovery availability. **Verify:** every operation has explicit rather than guessed undo semantics.
- [ ] **CODE-03.T02 — Local restore.** Preview and restore retained file/layout versions under current access and conflict checks, recording the restoration as a new action. **Verify:** undo cannot silently overwrite subsequent user edits or restore revoked data access.
- [ ] **CODE-03.T03 — Compensation planning.** Discover current provider cancel/delete/refund capabilities and prepare a fresh proposal linked to the original receipt. **Verify:** eligibility, account, fees, destination and time limits are rechecked.
- [ ] **CODE-03.T04 — Trusted compensation execution.** Use a new operation identity, policy/approval, dispatch, verification and reconciliation. **Verify:** both original and compensating receipts survive and an old approval cannot authorize a changed remedy.
- [ ] **CODE-03.T05 — Recovery UI.** Show diffs, exact identities, attempted/verified/uncertain state and available options in one timeline. **Verify:** stopping execution, restoring a local version and requesting an external refund are visibly different actions.
- [ ] **CODE-03.T06 — Concurrency/restart qualification.** Test edited bases, missing artifacts, expired grants, uncertain provider state and recovery while preserving minimum retained receipts. **Verify:** failures do not create a fictitious undo or erase the original outcome.

**Dependency refinement:** local file/layout undo can begin before full transaction/commerce maturity; external compensation waits for the relevant transaction contracts and actual provider capability. A requested refund is not a completed refund, cancellation is not rollback, and a stopped task may still have an accepted external effect.

#### Comment 5687107028 — Jordan-Hall — 2026-09-15T19:45:03Z

Source: https://github.com/Jordan-Hall/browser/issues/74#issuecomment-5687107028 | Updated: 2026-09-15T19:45:03Z

###### Task issues
- [ ] #579 `CODE-03.T01` — Define action history and reversibility metadata
- [ ] #580 `CODE-03.T02` — Implement local file and layout restoration
- [ ] #581 `CODE-03.T03` — Implement external compensation planning
- [ ] #582 `CODE-03.T04` — Route compensation through trusted transactions
- [ ] #583 `CODE-03.T05` — Build the history and recovery UI
- [ ] #584 `CODE-03.T06` — Qualify undo across concurrency and restart


---

<a id="issue-255"></a>
## #255 — [TASK][EPIC-CODE.T01] Ratify project and artifact ownership

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/255
**Created:** 2026-09-15T15:29:00Z | **Updated:** 2026-09-15T15:29:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #24

### Original description

Parent: #24

Task ID: `EPIC-CODE.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-256"></a>
## #256 — [TASK][EPIC-CODE.T02] Integrate isolated edits, builds and review

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/256
**Created:** 2026-09-15T15:29:06Z | **Updated:** 2026-09-15T15:29:06Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #24

### Original description

Parent: #24

Task ID: `EPIC-CODE.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-257"></a>
## #257 — [TASK][EPIC-CODE.T03] Integrate generated files and action recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/257
**Created:** 2026-09-15T15:29:14Z | **Updated:** 2026-09-15T15:29:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #24

### Original description

Parent: #24

Task ID: `EPIC-CODE.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-258"></a>
## #258 — [TASK][EPIC-CODE.T04] Qualify project integrity and cleanup

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/258
**Created:** 2026-09-15T15:29:19Z | **Updated:** 2026-09-15T15:29:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #24

### Original description

Parent: #24

Task ID: `EPIC-CODE.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-565"></a>
## #565 — [TASK][CODE-01.T01] Register projects and protected paths

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/565
**Created:** 2026-09-15T19:43:00Z | **Updated:** 2026-09-15T19:43:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #72

### Original description

Parent: #72

Task ID: `CODE-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-566"></a>
## #566 — [TASK][CODE-01.T02] Allocate isolated worktrees and snapshots

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/566
**Created:** 2026-09-15T19:43:07Z | **Updated:** 2026-09-15T19:43:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #72

### Original description

Parent: #72

Task ID: `CODE-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-567"></a>
## #567 — [TASK][CODE-01.T03] Implement patch capture and conflict detection

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/567
**Created:** 2026-09-15T19:43:13Z | **Updated:** 2026-09-15T19:43:13Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #72

### Original description

Parent: #72

Task ID: `CODE-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-568"></a>
## #568 — [TASK][CODE-01.T04] Build isolated test and tool execution

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/568
**Created:** 2026-09-15T19:43:18Z | **Updated:** 2026-09-15T19:43:18Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #72

### Original description

Parent: #72

Task ID: `CODE-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-569"></a>
## #569 — [TASK][CODE-01.T05] Review dependencies and protected changes

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/569
**Created:** 2026-09-15T19:43:23Z | **Updated:** 2026-09-15T19:43:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #72

### Original description

Parent: #72

Task ID: `CODE-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-570"></a>
## #570 — [TASK][CODE-01.T06] Implement code/diff/test review and apply

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/570
**Created:** 2026-09-15T19:43:29Z | **Updated:** 2026-09-15T19:43:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #72

### Original description

Parent: #72

Task ID: `CODE-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-571"></a>
## #571 — [TASK][CODE-01.T07] Qualify multi-agent coding and cleanup

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/571
**Created:** 2026-09-15T19:43:34Z | **Updated:** 2026-09-15T19:43:34Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #72

### Original description

Parent: #72

Task ID: `CODE-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-572"></a>
## #572 — [TASK][CODE-02.T01] Implement path and file-handle authorization

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/572
**Created:** 2026-09-15T19:43:46Z | **Updated:** 2026-09-15T19:43:46Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #73

### Original description

Parent: #73

Task ID: `CODE-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-573"></a>
## #573 — [TASK][CODE-02.T02] Implement scoped read/create/write operations

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/573
**Created:** 2026-09-15T19:43:52Z | **Updated:** 2026-09-15T19:43:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #73

### Original description

Parent: #73

Task ID: `CODE-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-574"></a>
## #574 — [TASK][CODE-02.T03] Implement rename/move/copy/delete with previews

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/574
**Created:** 2026-09-15T19:43:57Z | **Updated:** 2026-09-15T19:43:57Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #73

### Original description

Parent: #73

Task ID: `CODE-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-575"></a>
## #575 — [TASK][CODE-02.T04] Implement bounded process sessions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/575
**Created:** 2026-09-15T19:44:02Z | **Updated:** 2026-09-15T19:44:02Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #73

### Original description

Parent: #73

Task ID: `CODE-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-576"></a>
## #576 — [TASK][CODE-02.T05] Build format-aware generation and validation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/576
**Created:** 2026-09-15T19:44:07Z | **Updated:** 2026-09-15T19:44:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #73

### Original description

Parent: #73

Task ID: `CODE-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-577"></a>
## #577 — [TASK][CODE-02.T06] Implement safe previews and opening

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/577
**Created:** 2026-09-15T19:44:14Z | **Updated:** 2026-09-15T19:44:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #73

### Original description

Parent: #73

Task ID: `CODE-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-578"></a>
## #578 — [TASK][CODE-02.T07] Integrate recovery, auditing and file workflows

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/578
**Created:** 2026-09-15T19:44:18Z | **Updated:** 2026-09-15T19:44:18Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #73

### Original description

Parent: #73

Task ID: `CODE-02.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-579"></a>
## #579 — [TASK][CODE-03.T01] Define action history and reversibility metadata

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/579
**Created:** 2026-09-15T19:44:29Z | **Updated:** 2026-09-15T19:44:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #74

### Original description

Parent: #74

Task ID: `CODE-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-580"></a>
## #580 — [TASK][CODE-03.T02] Implement local file and layout restoration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/580
**Created:** 2026-09-15T19:44:34Z | **Updated:** 2026-09-15T19:44:34Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #74

### Original description

Parent: #74

Task ID: `CODE-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-581"></a>
## #581 — [TASK][CODE-03.T03] Implement external compensation planning

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/581
**Created:** 2026-09-15T19:44:39Z | **Updated:** 2026-09-15T19:44:39Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #74

### Original description

Parent: #74

Task ID: `CODE-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-582"></a>
## #582 — [TASK][CODE-03.T04] Route compensation through trusted transactions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/582
**Created:** 2026-09-15T19:44:45Z | **Updated:** 2026-09-15T19:44:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #74

### Original description

Parent: #74

Task ID: `CODE-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-583"></a>
## #583 — [TASK][CODE-03.T05] Build the history and recovery UI

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/583
**Created:** 2026-09-15T19:44:50Z | **Updated:** 2026-09-15T19:44:50Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #74

### Original description

Parent: #74

Task ID: `CODE-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-584"></a>
## #584 — [TASK][CODE-03.T06] Qualify undo across concurrency and restart

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/584
**Created:** 2026-09-15T19:44:55Z | **Updated:** 2026-09-15T19:44:55Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #74

### Original description

Parent: #74

Task ID: `CODE-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

