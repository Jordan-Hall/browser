# Inspectable memory and learned workflows

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 28

## Contents

- [#29 — EPIC: Inspectable memory and learned workflows](#issue-29)
- [#89 — [P1][MEM-01] Explicit preferences and memory inspector](#issue-89)
- [#90 — [P3][MEM-02] Opt-in adaptive patterns and declared lenses](#issue-90)
- [#91 — [P3][MEM-03] Verified workflow recipes and procedural learning](#issue-91)
- [#275 — [TASK][EPIC-MEM.T01] Ratify memory scopes and correction precedence](#issue-275)
- [#276 — [TASK][EPIC-MEM.T02] Integrate inspector, adaptation and deletion](#issue-276)
- [#277 — [TASK][EPIC-MEM.T03] Integrate verified procedural learning](#issue-277)
- [#278 — [TASK][EPIC-MEM.T04] Qualify agency, privacy and poisoning resistance](#issue-278)
- [#682 — [TASK][MEM-01.T01] Define scoped memory records and precedence](#issue-682)
- [#683 — [TASK][MEM-01.T02] Implement deliberate capture and provenance](#issue-683)
- [#684 — [TASK][MEM-01.T03] Implement memory retrieval and use accounting](#issue-684)
- [#685 — [TASK][MEM-01.T04] Build the memory inspector and correction UI](#issue-685)
- [#686 — [TASK][MEM-01.T05] Implement forget, expiry and derivative invalidation](#issue-686)
- [#687 — [TASK][MEM-01.T06] Implement safe memory export and import](#issue-687)
- [#688 — [TASK][MEM-01.T07] Qualify memory correction and isolation](#issue-688)
- [#689 — [TASK][MEM-02.T01] Define opt-in observation and allowed pattern classes](#issue-689)
- [#690 — [TASK][MEM-02.T02] Extract candidate patterns with bounded history](#issue-690)
- [#691 — [TASK][MEM-02.T03] Implement sensitivity and precedence gates](#issue-691)
- [#692 — [TASK][MEM-02.T04] Generate reviewable adaptation proposals](#issue-692)
- [#693 — [TASK][MEM-02.T05] Implement declared lenses and reset controls](#issue-693)
- [#694 — [TASK][MEM-02.T06] Qualify adaptation usefulness and reversibility](#issue-694)
- [#695 — [TASK][MEM-03.T01] Define a versioned workflow recipe language](#issue-695)
- [#696 — [TASK][MEM-03.T02] Identify candidate repeatable procedures](#issue-696)
- [#697 — [TASK][MEM-03.T03] Compile and minimize required capabilities](#issue-697)
- [#698 — [TASK][MEM-03.T04] Simulate changed inputs and failure paths](#issue-698)
- [#699 — [TASK][MEM-03.T05] Build user review, editing and promotion](#issue-699)
- [#700 — [TASK][MEM-03.T06] Integrate runtime execution and version lifecycle](#issue-700)
- [#701 — [TASK][MEM-03.T07] Qualify procedural learning and portability](#issue-701)

---

<a id="issue-29"></a>
## #29 — EPIC: Inspectable memory and learned workflows

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/29
**Created:** 2026-09-15T12:08:34Z | **Updated:** 2026-09-15T14:25:21Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1

Own explicit preferences, scoped and inspectable memory, optional adaptive patterns, and promotion of successful work into verified reusable workflow recipes.

#### Child issues
- [ ] #89 MEM-01 — Explicit preferences and memory inspector
- [ ] #90 MEM-02 — Opt-in adaptive patterns and declared lenses
- [ ] #91 MEM-03 — Verified workflow recipes and procedural learning

#### Cross-cutting gates
Origin, scope, confidence and expiry are visible; deletion removes future context eligibility; inferred preferences remain distinct from confirmed preferences; personalization cannot rewrite evidence; repeated success never auto-installs arbitrary scripts.

### Discussion (1 comments)

#### Comment 5681910560 — Jordan-Hall — 2026-09-15T14:25:21Z

Source: https://github.com/Jordan-Hall/browser/issues/29#issuecomment-5681910560 | Updated: 2026-09-15T14:25:21Z

<!-- intent-implementation-v1:EPIC-MEM -->
###### Workstream implementation and integration tasks

Integrate #89–#91 as user-owned, inspectable memory and tested reusable workflows.

- [ ] **EPIC-MEM.T01 — Ratify scopes/correction precedence.** Distinguish session, project, explicit preference, episode and optional inferred pattern records with origin, confidence, sensitivity, expiry and allowed uses. **Proof:** explicit corrections/locks outrank inference and memory never grants execution authority.
- [ ] **EPIC-MEM.T02 — Integrate inspector/adaptation/deletion.** Expose source, usage, correction, export and forget; connect reviewed layout changes and derivative cleanup. **Proof:** deleted or private-session material cannot re-enter future context through caches/indexes.
- [ ] **EPIC-MEM.T03 — Integrate procedural learning.** Propose recipes from repeated verified traces, with input schemas, preconditions, bounded agent slots, permissions and verifiers. **Proof:** user review and fixture tests precede installation; credentials/old approvals are excluded.
- [ ] **EPIC-MEM.T04 — Qualify agency/privacy/poisoning.** Test malicious source instructions, conflicting inferred preferences, sensitive patterns and recipe updates. **Proof:** source content cannot poison durable authority/preferences, and wider capabilities require fresh review.

**Demonstration:** correct a comparison preference, inspect why the next view changed, lock the layout, approve a reusable research recipe, then forget the underlying optional memory and verify future eligibility.

**Review boundary:** declared viewpoints are visible presentation/relevance lenses, not instructions to alter evidence. A successful one-off click sequence is not an automatically trusted connector.


---

<a id="issue-89"></a>
## #89 — [P1][MEM-01] Explicit preferences and memory inspector

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/89
**Created:** 2026-09-15T12:19:19Z | **Updated:** 2026-09-15T21:01:17Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #29

#### Objective
Implement inspectable, scoped user memory where preferences/context remain user-owned data rather than an opaque ever-growing prompt.

#### Scope
- Separate scopes: session context, project knowledge, episodic summaries, explicit durable preferences and optional learned patterns.
- `MemoryRecord` metadata: origin, evidence/source, confirmation, confidence, sensitivity, scope, expiry and allowed destinations/uses.
- Memory inspector with search, source, usage history, correction, lock, expiry, export and forget.
- Private-session mode that prevents unintended durable carryover.
- Work/personal profile separation.
- Deletion propagation into retrieval indexes, cached summaries and future context eligibility.

#### Product rules
- Explicit user corrections beat inferred preferences.
- Sensitive inferences do not become durable facts by default.
- Memory does not grant capabilities/permissions.

#### Acceptance criteria
- [ ] User can locate the origin and current scope of a durable preference/memory.
- [ ] Correction immediately affects future eligible retrieval/use.
- [ ] Forget/delete removes future context eligibility and configured searchable derivatives.
- [ ] Private-session material does not enter durable memory without explicit action.
- [ ] Work/personal memory remains isolated under provider/context switches.
- [ ] Export produces human- and machine-readable memory records without credentials.

#### Dependencies
- DATA-01
- SEC-03

**First phase:** P1  
**Maturity target:** P5  
**Owner:** workspace-ui-data

### Discussion (2 comments)

#### Comment 5682385175 — Jordan-Hall — 2026-09-15T14:50:33Z

Source: https://github.com/Jordan-Hall/browser/issues/89#issuecomment-5682385175 | Updated: 2026-09-15T14:50:33Z

<!-- intent-implementation-v1:MEM-01 -->
###### Implementation proposal — MEM-01

Implement inspectable local-first memory over #41/#8, separating explicit preferences, project/session knowledge, summaries and optional inferred patterns. Memory influences presentation/context; it never grants capabilities.

- [ ] **MEM-01.T01 — Records/scopes/precedence.** Store origin, stated/inferred/summarized status, profile/project/session scope, confidence, sensitivity, expiry and permitted uses. **Verify:** locked explicit preferences outrank optional inference and private scopes remain separate.
- [ ] **MEM-01.T02 — Deliberate capture.** Save explicit preferences through user actions; stage proposed task-derived memories with evidence and conservative retention/destination defaults. **Verify:** source instructions cannot promote themselves into durable preferences.
- [ ] **MEM-01.T03 — Retrieval/use accounting.** Filter by current task/profile/source rights and destination before context construction; record memory revision use without unnecessary prompt retention. **Verify:** a revoked record cannot leak through a previously built cache or remote-context serialization.
- [ ] **MEM-01.T04 — Inspector/correction.** Expose origin, scope, confidence, usage, expiry and confirmation; support edit/lock/reject/disable/conflict resolution. **Verify:** corrections change future eligible behavior and their history is inspectable with minimized payloads.
- [ ] **MEM-01.T05 — Forget/expiry.** Tombstone and deny retrieval immediately, invalidate indexes/caches and schedule eligible artifact cleanup with progress. **Verify:** forgotten data cannot re-enter new context through summaries/embeddings.
- [ ] **MEM-01.T06 — Export/import.** Provide readable and machine-readable scoped records without credentials/grants; import as untrusted proposed data with local scope assignment. **Verify:** imported memory cannot reactivate permissions or overwrite locked preferences silently.
- [ ] **MEM-01.T07 — Isolation/poisoning tests.** Exercise private sessions, expiry, conflicts, deletion, profile/provider changes and malicious records. **Verify:** authorization precedes every exposure and explicit correction stays authoritative.

**Review boundary:** logical deletion can block new retrieval immediately; physical storage/backups have documented cleanup constraints. Do not claim erasure from remote providers or people already given plaintext. The memory inspector must remain usable when inference is disabled.

#### Comment 5688024989 — Jordan-Hall — 2026-09-15T21:01:17Z

Source: https://github.com/Jordan-Hall/browser/issues/89#issuecomment-5688024989 | Updated: 2026-09-15T21:01:17Z

###### Task issues

- [ ] #682 `MEM-01.T01` — Define scoped memory records and precedence
- [ ] #683 `MEM-01.T02` — Implement deliberate capture and provenance
- [ ] #684 `MEM-01.T03` — Implement memory retrieval and use accounting
- [ ] #685 `MEM-01.T04` — Build the memory inspector and correction UI
- [ ] #686 `MEM-01.T05` — Implement forget, expiry and derivative invalidation
- [ ] #687 `MEM-01.T06` — Implement safe memory export and import
- [ ] #688 `MEM-01.T07` — Qualify memory correction and isolation

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-90"></a>
## #90 — [P3][MEM-02] Opt-in adaptive patterns and declared lenses

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/90
**Created:** 2026-09-15T12:19:28Z | **Updated:** 2026-09-15T21:02:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #29

#### Objective
Let the runtime learn presentation/relevance patterns and declared viewpoints without turning inferred beliefs into hidden truth filters.

#### Scope
- Opt-in pattern inference from repeated explicit corrections/layout choices/workflows.
- Separate inferred pattern records from confirmed preferences.
- Sensitivity classifier/policy preventing durable sensitive inferences without deliberate consent.
- “Why did this adapt?” explanations and one-click correction/disable.
- Declared lenses/viewpoints as explicit user-selected ranking/presentation inputs.
- Stable layout adaptation through UI-03 review/lock mechanics.
- Conflict handling when a new pattern contradicts a locked/explicit preference.

#### Product rules
- Personalization changes presentation/relevance, not source evidence.
- A declared lens is labeled and cannot silently remove contradictory credible evidence.
- Sensitive political/health/religious/etc. inferences must not become durable hidden preferences by default.

#### Acceptance criteria
- [ ] Every adaptive change can identify the preference/pattern that caused it.
- [ ] Inferred patterns remain distinguishable from user-confirmed preferences.
- [ ] Sensitive inferred beliefs cannot silently become durable facts/preferences.
- [ ] User correction/lock overrides future adaptation.
- [ ] Declared lenses retain contradictory evidence/source access.
- [ ] Disabling adaptive patterns leaves explicit preferences and workspace functionality intact.

#### Dependencies
- MEM-01
- UI-03

**First phase:** P3  
**Maturity target:** P5  
**Owner:** workspace-ui-data

### Discussion (2 comments)

#### Comment 5682391962 — Jordan-Hall — 2026-09-15T14:50:54Z

Source: https://github.com/Jordan-Hall/browser/issues/90#issuecomment-5682391962 | Updated: 2026-09-15T14:50:54Z

<!-- intent-implementation-v1:MEM-02 -->
###### Implementation proposal — MEM-02

Implement optional learned adaptation over #89/#51. Learn bounded presentation/relevance patterns, not hidden sensitive profiles or new authority.

- [ ] **MEM-02.T01 — Consent/allowed observations.** Define which explicit local interactions can inform learning, such as repeated layout/density corrections; exclude private sessions and sensitive sources by default. **Verify:** consent is scoped by profile/purpose and disabling it stops capture.
- [ ] **MEM-02.T02 — Candidate extraction.** Aggregate repeated stable choices within a limited retention window and attach evidence/uncertainty. **Verify:** one transient action does not become a durable inferred preference or unrelated identity/belief inference.
- [ ] **MEM-02.T03 — Sensitivity/precedence gates.** Allowlist adaptation targets, reject prohibited capture and compare candidates with locked explicit choices; require deliberate review for sensitive use. **Verify:** a classifier's low sensitivity score alone cannot authorize a forbidden memory.
- [ ] **MEM-02.T04 — Reviewable proposals.** Translate eligible patterns into specific layout/ranking changes with before/after diff and evidence-linked explanation, applied through #51. **Verify:** no silent control movement or overwrite of a locked preference.
- [ ] **MEM-02.T05 — Declared lenses/reset.** Let users choose and visibly label a viewpoint/source set; preserve factual support, credible contradictions and source inspection; provide disable/reset/export. **Verify:** a lens cannot alter evidence or transfer source permissions.
- [ ] **MEM-02.T06 — Utility/reversibility tests.** Test noise, poisoning, sensitive sources, explicit conflicts and resets; measure useful accepted adaptations, corrections and user understanding. **Verify:** goals are not replaced by engagement maximization.

**Implementation boundary:** combine restricted learned-feature classes, deliberate promotion and deterministic policy; do not rely on a sensitivity model as the sole gate. Learned preferences remain distinct from confirmed preferences. Full personalization stays in scope, with visible origins and user override rather than an opaque personality prompt.

#### Comment 5688033732 — Jordan-Hall — 2026-09-15T21:02:00Z

Source: https://github.com/Jordan-Hall/browser/issues/90#issuecomment-5688033732 | Updated: 2026-09-15T21:02:00Z

###### Task issues

- [ ] #689 `MEM-02.T01` — Define opt-in observation and allowed pattern classes
- [ ] #690 `MEM-02.T02` — Extract candidate patterns with bounded history
- [ ] #691 `MEM-02.T03` — Implement sensitivity and precedence gates
- [ ] #692 `MEM-02.T04` — Generate reviewable adaptation proposals
- [ ] #693 `MEM-02.T05` — Implement declared lenses and reset controls
- [ ] #694 `MEM-02.T06` — Qualify adaptation usefulness and reversibility

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-91"></a>
## #91 — [P3][MEM-03] Verified workflow recipes and procedural learning

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/91
**Created:** 2026-09-15T12:19:37Z | **Updated:** 2026-09-15T21:02:48Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #29

#### Objective
Convert repeated successful work into reusable tested procedures so the runtime becomes more deterministic and efficient over time rather than merely accumulating prompt history.

#### Scope
- Detect candidate repeated workflows from successful verified task traces.
- Propose a `WorkflowRecipe` with inputs, preconditions, deterministic steps, bounded agent slots, required capabilities, budgets, stopping conditions and verification.
- User review/edit/approve before promotion to reusable automation.
- Versioning, migration, tests and rollback for recipes.
- Distinguish provider-independent logical steps from provider-specific adapters.
- Permission manifest and least-authority recomputation per run.
- Recipe simulator against EVAL-01 fixtures.

#### Safety rules
- One successful click sequence does not become an unattended connector automatically.
- Promoting a recipe never promotes captured credentials/session cookies/old approvals.
- Recipe updates that widen capabilities require fresh review.

#### Acceptance criteria
- [ ] Repeated verified task can produce a reviewable recipe proposal.
- [ ] Approved recipe declares exact inputs/capabilities/preconditions/verifiers.
- [ ] Recipe executes against resettable fixtures with bounded permissions.
- [ ] Captured secrets/approvals are absent from portable recipe definition.
- [ ] Update requiring broader authority stops for review.
- [ ] Failed recipe version can roll back without corrupting workspace/task state.

#### Dependencies
- AGENT-01
- MEM-01
- SDK-01

**First phase:** P3  
**Maturity target:** P5  
**Owner:** workspace-ui-data

### Discussion (2 comments)

#### Comment 5682396970 — Jordan-Hall — 2026-09-15T14:51:09Z

Source: https://github.com/Jordan-Hall/browser/issues/91#issuecomment-5682396970 | Updated: 2026-09-15T14:51:09Z

<!-- intent-implementation-v1:MEM-03 -->
###### Implementation proposal — MEM-03

Implement reusable verified WorkflowRecipe definitions over #54/#89/#98. Generalize successful semantic structure, not captured credentials or one fragile click sequence.

- [ ] **MEM-03.T01 — Recipe language.** Define typed inputs/outputs, deterministic steps, bounded agent slots, preconditions, budgets, required capabilities and independent verifiers. **Verify:** ordinary expressions cannot evaluate arbitrary privileged code.
- [ ] **MEM-03.T02 — Repeatable candidates.** Analyze verified traces for repeated operations, distinguish variables from fixed logic and retain failures/uncertainty. **Verify:** one successful run cannot automatically promote a workflow into unattended production.
- [ ] **MEM-03.T03 — Compile/minimize authority.** Produce semantic steps and least-required capabilities; revalidate new inputs and strip secrets/session IDs/approval tokens. **Verify:** replayed definitions cannot inherit captured authority.
- [ ] **MEM-03.T04 — Simulate changes/failures.** Vary records, stale state, missing tools, revoked grants and interrupted steps in resettable fixtures. **Verify:** independent postconditions expose unsupported assumptions before installation.
- [ ] **MEM-03.T05 — Review/edit/promote.** Show steps, inputs, resource limits, effects, verification and permission differences. Approve a reusable version separately from any unattended grant. **Verify:** recipe approval alone cannot authorize future purchases/messages.
- [ ] **MEM-03.T06 — Runtime/version lifecycle.** Execute through durable harness/scheduler with fresh input/grant binding and versioned artifacts; support migration, rollback and pause on capability changes. **Verify:** interrupted or upgraded runs cannot silently use a different recipe or wider authority.
- [ ] **MEM-03.T07 — Portability/learning qualification.** Test provider substitution, offline operation, account rebinding, secret stripping and failed updates. **Verify:** reduced inference does not sacrifice verification or user control.

**Dependency refinement:** deterministic recipe authoring/testing can begin before optional learned-pattern extraction or marketplace maturity. A saved successful browser trace is not inherently a reliable connector. Keep source freshness, current permissions and verification explicit on every execution.

#### Comment 5688044211 — Jordan-Hall — 2026-09-15T21:02:48Z

Source: https://github.com/Jordan-Hall/browser/issues/91#issuecomment-5688044211 | Updated: 2026-09-15T21:02:48Z

###### Task issues

- [ ] #695 `MEM-03.T01` — Define a versioned workflow recipe language
- [ ] #696 `MEM-03.T02` — Identify candidate repeatable procedures
- [ ] #697 `MEM-03.T03` — Compile and minimize required capabilities
- [ ] #698 `MEM-03.T04` — Simulate changed inputs and failure paths
- [ ] #699 `MEM-03.T05` — Build user review, editing and promotion
- [ ] #700 `MEM-03.T06` — Integrate runtime execution and version lifecycle
- [ ] #701 `MEM-03.T07` — Qualify procedural learning and portability

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-275"></a>
## #275 — [TASK][EPIC-MEM.T01] Ratify memory scopes and correction precedence

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/275
**Created:** 2026-09-15T15:31:19Z | **Updated:** 2026-09-15T15:31:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #29

### Original description

Parent: #29

Task ID: `EPIC-MEM.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-276"></a>
## #276 — [TASK][EPIC-MEM.T02] Integrate inspector, adaptation and deletion

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/276
**Created:** 2026-09-15T15:31:27Z | **Updated:** 2026-09-15T15:31:27Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #29

### Original description

Parent: #29

Task ID: `EPIC-MEM.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-277"></a>
## #277 — [TASK][EPIC-MEM.T03] Integrate verified procedural learning

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/277
**Created:** 2026-09-15T15:31:34Z | **Updated:** 2026-09-15T15:31:34Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #29

### Original description

Parent: #29

Task ID: `EPIC-MEM.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-278"></a>
## #278 — [TASK][EPIC-MEM.T04] Qualify agency, privacy and poisoning resistance

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/278
**Created:** 2026-09-15T15:31:41Z | **Updated:** 2026-09-15T15:31:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #29

### Original description

Parent: #29

Task ID: `EPIC-MEM.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-682"></a>
## #682 — [TASK][MEM-01.T01] Define scoped memory records and precedence

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/682
**Created:** 2026-09-15T21:00:33Z | **Updated:** 2026-09-15T21:00:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #89

### Original description

Parent: #89

Task ID: `MEM-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-683"></a>
## #683 — [TASK][MEM-01.T02] Implement deliberate capture and provenance

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/683
**Created:** 2026-09-15T21:00:40Z | **Updated:** 2026-09-15T21:00:40Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #89

### Original description

Parent: #89

Task ID: `MEM-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-684"></a>
## #684 — [TASK][MEM-01.T03] Implement memory retrieval and use accounting

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/684
**Created:** 2026-09-15T21:00:45Z | **Updated:** 2026-09-15T21:00:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #89

### Original description

Parent: #89

Task ID: `MEM-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-685"></a>
## #685 — [TASK][MEM-01.T04] Build the memory inspector and correction UI

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/685
**Created:** 2026-09-15T21:00:51Z | **Updated:** 2026-09-15T21:00:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #89

### Original description

Parent: #89

Task ID: `MEM-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-686"></a>
## #686 — [TASK][MEM-01.T05] Implement forget, expiry and derivative invalidation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/686
**Created:** 2026-09-15T21:00:56Z | **Updated:** 2026-09-15T21:00:56Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #89

### Original description

Parent: #89

Task ID: `MEM-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-687"></a>
## #687 — [TASK][MEM-01.T06] Implement safe memory export and import

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/687
**Created:** 2026-09-15T21:01:05Z | **Updated:** 2026-09-15T21:01:05Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #89

### Original description

Parent: #89

Task ID: `MEM-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-688"></a>
## #688 — [TASK][MEM-01.T07] Qualify memory correction and isolation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/688
**Created:** 2026-09-15T21:01:11Z | **Updated:** 2026-09-15T21:01:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #89

### Original description

Parent: #89

Task ID: `MEM-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-689"></a>
## #689 — [TASK][MEM-02.T01] Define opt-in observation and allowed pattern classes

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/689
**Created:** 2026-09-15T21:01:23Z | **Updated:** 2026-09-15T21:01:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #90

### Original description

Parent: #90

Task ID: `MEM-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-690"></a>
## #690 — [TASK][MEM-02.T02] Extract candidate patterns with bounded history

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/690
**Created:** 2026-09-15T21:01:27Z | **Updated:** 2026-09-15T21:01:27Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #90

### Original description

Parent: #90

Task ID: `MEM-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-691"></a>
## #691 — [TASK][MEM-02.T03] Implement sensitivity and precedence gates

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/691
**Created:** 2026-09-15T21:01:35Z | **Updated:** 2026-09-15T21:01:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #90

### Original description

Parent: #90

Task ID: `MEM-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-692"></a>
## #692 — [TASK][MEM-02.T04] Generate reviewable adaptation proposals

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/692
**Created:** 2026-09-15T21:01:41Z | **Updated:** 2026-09-15T21:01:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #90

### Original description

Parent: #90

Task ID: `MEM-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-693"></a>
## #693 — [TASK][MEM-02.T05] Implement declared lenses and reset controls

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/693
**Created:** 2026-09-15T21:01:47Z | **Updated:** 2026-09-15T21:01:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #90

### Original description

Parent: #90

Task ID: `MEM-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-694"></a>
## #694 — [TASK][MEM-02.T06] Qualify adaptation usefulness and reversibility

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/694
**Created:** 2026-09-15T21:01:54Z | **Updated:** 2026-09-15T21:01:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #90

### Original description

Parent: #90

Task ID: `MEM-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-695"></a>
## #695 — [TASK][MEM-03.T01] Define a versioned workflow recipe language

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/695
**Created:** 2026-09-15T21:02:07Z | **Updated:** 2026-09-15T21:02:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #91

### Original description

Parent: #91

Task ID: `MEM-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-696"></a>
## #696 — [TASK][MEM-03.T02] Identify candidate repeatable procedures

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/696
**Created:** 2026-09-15T21:02:12Z | **Updated:** 2026-09-15T21:02:12Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #91

### Original description

Parent: #91

Task ID: `MEM-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-697"></a>
## #697 — [TASK][MEM-03.T03] Compile and minimize required capabilities

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/697
**Created:** 2026-09-15T21:02:18Z | **Updated:** 2026-09-15T21:02:18Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #91

### Original description

Parent: #91

Task ID: `MEM-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-698"></a>
## #698 — [TASK][MEM-03.T04] Simulate changed inputs and failure paths

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/698
**Created:** 2026-09-15T21:02:23Z | **Updated:** 2026-09-15T21:02:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #91

### Original description

Parent: #91

Task ID: `MEM-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-699"></a>
## #699 — [TASK][MEM-03.T05] Build user review, editing and promotion

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/699
**Created:** 2026-09-15T21:02:28Z | **Updated:** 2026-09-15T21:02:28Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #91

### Original description

Parent: #91

Task ID: `MEM-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-700"></a>
## #700 — [TASK][MEM-03.T06] Integrate runtime execution and version lifecycle

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/700
**Created:** 2026-09-15T21:02:34Z | **Updated:** 2026-09-15T21:02:34Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #91

### Original description

Parent: #91

Task ID: `MEM-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-701"></a>
## #701 — [TASK][MEM-03.T07] Qualify procedural learning and portability

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/701
**Created:** 2026-09-15T21:02:40Z | **Updated:** 2026-09-15T21:02:40Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #91

### Original description

Parent: #91

Task ID: `MEM-03.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

