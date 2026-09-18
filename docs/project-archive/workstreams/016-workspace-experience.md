# Workspace experience

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 40

## Contents

- [#16 — EPIC: Workspace experience](#issue-16)
- [#37 — [P1][WS-01] Persistent workspace store and navigation](#issue-37)
- [#38 — [P1][WS-02] Personal, Original and Evidence views](#issue-38)
- [#39 — [P1][WS-03] Trusted control surfaces and task centre](#issue-39)
- [#40 — [P1][WS-04] Command palette, semantic selection and accessibility](#issue-40)
- [#223 — [TASK][EPIC-WS.T01] Specify workspace navigation and trusted chrome](#issue-223)
- [#224 — [TASK][EPIC-WS.T02] Integrate Personal, Original and Evidence representations](#issue-224)
- [#225 — [TASK][EPIC-WS.T03] Integrate direct, keyboard and contextual controls](#issue-225)
- [#226 — [TASK][EPIC-WS.T04] Validate workspace independence and recovery](#issue-226)
- [#313 — [TASK][WS-01.T01] Define workspace schema and revision ownership](#issue-313)
- [#314 — [TASK][WS-01.T02] Implement workspace lifecycle commands](#issue-314)
- [#315 — [TASK][WS-01.T03] Build workspace home and deterministic navigation](#issue-315)
- [#316 — [TASK][WS-01.T04] Persist layouts, filters and user overlays separately](#issue-316)
- [#317 — [TASK][WS-01.T05] Implement navigation history and restoration](#issue-317)
- [#318 — [TASK][WS-01.T06] Add artifact/task/source links and local search](#issue-318)
- [#319 — [TASK][WS-01.T07] Implement archive, export and retention handoff](#issue-319)
- [#320 — [TASK][WS-01.T08] Prove workspace independence from conversation](#issue-320)
- [#321 — [TASK][WS-02.T01] Define shared selection and view context](#issue-321)
- [#322 — [TASK][WS-02.T02] Implement source-backed Personal view bindings](#issue-322)
- [#323 — [TASK][WS-02.T03] Integrate authentic Original navigation](#issue-323)
- [#324 — [TASK][WS-02.T04] Implement field-level Evidence inspection](#issue-324)
- [#325 — [TASK][WS-02.T05] Add contradiction and revision navigation](#issue-325)
- [#326 — [TASK][WS-02.T06] Preserve focus, scroll and history across views](#issue-326)
- [#327 — [TASK][WS-02.T07] Test source identity and privacy consistency](#issue-327)
- [#328 — [TASK][WS-03.T01] Define trusted surface ownership](#issue-328)
- [#329 — [TASK][WS-03.T02] Build task projections and outcome labels](#issue-329)
- [#330 — [TASK][WS-03.T03] Implement pause, stop and takeover controls](#issue-330)
- [#331 — [TASK][WS-03.T04] Render exact approval proposals](#issue-331)
- [#332 — [TASK][WS-03.T05] Show inference mode and execution location](#issue-332)
- [#333 — [TASK][WS-03.T06] Implement audit and receipt drill-down](#issue-333)
- [#334 — [TASK][WS-03.T07] Handle stale notifications and concurrent decisions](#issue-334)
- [#335 — [TASK][WS-03.T08] Qualify trusted-control accessibility and stress](#issue-335)
- [#336 — [TASK][WS-04.T01] Define command descriptors and routing](#issue-336)
- [#337 — [TASK][WS-04.T02] Implement explicit semantic selection](#issue-337)
- [#338 — [TASK][WS-04.T03] Build command palette and local search](#issue-338)
- [#339 — [TASK][WS-04.T04] Implement focus and input composition](#issue-339)
- [#340 — [TASK][WS-04.T05] Expose accessible component semantics](#issue-340)
- [#341 — [TASK][WS-04.T06] Implement zoom, density and preference controls](#issue-341)
- [#342 — [TASK][WS-04.T07] Integrate contextual voice targets](#issue-342)
- [#343 — [TASK][WS-04.T08] Run keyboard and assistive-technology conformance](#issue-343)

---

<a id="issue-16"></a>
## #16 — EPIC: Workspace experience

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/16
**Created:** 2026-09-15T12:06:56Z | **Updated:** 2026-09-15T14:21:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #1 | **Body-declared parent:** #1
**Native child issues:** #37, #38, #39, #40

### Original description

Programme: #1

Own the persistent workspace as the primary product object: home/navigation, synchronized Personal/Original/Evidence views, trusted task/approval surfaces, command palette, semantic selection and accessibility.

#### Child issues
- [ ] #37 WS-01 — Persistent workspace store and navigation
- [ ] #38 WS-02 — Personal, Original and Evidence views
- [ ] #39 WS-03 — Trusted control surfaces and task centre
- [ ] #40 WS-04 — Command palette, semantic selection and accessibility

#### Cross-cutting gates
Workspace survives chat/model/provider changes, trusted controls cannot be spoofed, deterministic interactions work without inference, and core flows are keyboard/assistive-tech usable.

### Discussion (1 comments)

#### Comment 5681838631 — Jordan-Hall — 2026-09-15T14:21:32Z

Source: https://github.com/Jordan-Hall/browser/issues/16#issuecomment-5681838631 | Updated: 2026-09-15T14:21:32Z

<!-- intent-implementation-v1:EPIC-WS -->
###### Workstream implementation and integration tasks

Implement #37–#40 around persistent workspace state; conversation is only one input channel.

- [ ] **EPIC-WS.T01 — Specify navigation and trusted chrome.** Agree stable workspace/view/selection IDs, independent layout/data/task state, account identity and non-provider-controlled approval/stop surfaces. **Proof:** a saved workspace can be navigated without a conversation.
- [ ] **EPIC-WS.T02 — Integrate Personal, Original and Evidence.** Connect generated views to source-backed entities, provenance spans and authenticated original-page handoff. **Proof:** a selected fact retains its identity and context across all three views.
- [ ] **EPIC-WS.T03 — Integrate direct, keyboard and contextual controls.** Wire deterministic sort/filter/navigation, command palette, screen-reader semantics and explicit selection for voice. **Proof:** core flows work without inference or a microphone.
- [ ] **EPIC-WS.T04 — Validate independence and recovery.** Close chat, disable models, replace the provider and restart during a task. **Proof:** layout, annotations, sources, files and durable task state remain usable; unavailable operations are labelled honestly.

**End-to-end test:** create a comparison, pin columns/exclude an item, inspect evidence, switch to Original, return, restart and continue sorting with inference disabled.

**Review gate:** no silent structural regeneration during interaction, no credential-bearing workspace exports, and no generated content capable of issuing trusted approvals. Epic closure requires an integrated accessibility and anti-wrapper demonstration, not only closed child issues.


---

<a id="issue-37"></a>
## #37 — [P1][WS-01] Persistent workspace store and navigation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/37
**Created:** 2026-09-15T12:09:51Z | **Updated:** 2026-09-15T14:28:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #16 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #16

#### Objective
Make the persistent workspace—not a conversation or tab—the durable user-facing unit of state.

#### Scope
- Workspace/project home with stable IDs and addresses.
- Persist goal, source-backed object references, layouts, user overlays, saved filters, navigation history, files/artifacts and task links.
- Separate layout state from source data and execution/task state.
- Stable panel/view identity, back/forward navigation and restart restoration.
- Workspace archive/export hooks and redacted snapshot support.
- Local deterministic search/navigation over existing workspace state.

#### Product rules
- Closing a chat must not remove the application the user created.
- Sorting/filtering/navigation must continue when inference is unavailable.
- Provider session/thread IDs remain optional references, never primary ownership.

#### Acceptance criteria
- [ ] Create a workspace, change layout/filter state, close the conversation/app, restart and recover the same usable workspace.
- [ ] Workspace navigation works with inference disabled.
- [ ] Layout state can change without mutating source observations.
- [ ] Task/artifact/source links remain resolvable after restart and migrations.
- [ ] Archived/deleted workspaces follow retention and data-deletion rules.
- [ ] Stable workspace URLs/IDs support deep links from notifications/history.

#### Dependencies
- CORE-02

**First phase:** P1  
**Maturity target:** P2  
**Owner:** workspace-ui-data

### Discussion (1 comments)

#### Comment 5681972717 — Jordan-Hall — 2026-09-15T14:28:37Z

Source: https://github.com/Jordan-Hall/browser/issues/37#issuecomment-5681972717 | Updated: 2026-09-15T14:28:37Z

<!-- intent-implementation-v1:WS-01 -->
###### Implementation proposal — WS-01

Build `workspaces`, `navigation`, shell workspace surfaces and migrations over #3. Persist goal/source/view/overlay/artifact/task references independently; provider conversation IDs are optional references only.

- [ ] **WS-01.T01 — Schema/revision ownership.** Define separately revisioned workspace, bindings, layouts and overlays with profile ownership. **Verify:** source changes and layout edits cannot overwrite each other's state.
- [ ] **WS-01.T02 — Lifecycle commands.** Implement create/rename/pin/duplicate-definition/archive/restore/delete using access checks and expected revisions. **Verify:** duplication neither copies credentials nor grants source access.
- [ ] **WS-01.T03 — Home/navigation.** Render persisted projects, pinned/recent workspaces and stable routes to views/entities/tasks/artifacts. **Verify:** home and navigation work without an LLM.
- [ ] **WS-01.T04 — Layout/filter/overlay persistence.** Use deterministic reducers for columns, density, filters and annotations; retain layout versions. **Verify:** restore reproduces a prior layout without modifying source observations.
- [ ] **WS-01.T05 — History/restore.** Persist semantic routes, selection and bounded scroll anchors; recover the nearest surviving anchor. **Verify:** restart or source deletion does not leave unusable navigation.
- [ ] **WS-01.T06 — Links/local search.** Index permitted metadata and resolve resources through responsible brokers; distinguish missing from inaccessible. **Verify:** denied objects do not leak through previews/search.
- [ ] **WS-01.T07 — Archive/export/retention.** Preserve definitions, explicitly handle active workflows, invoke deletion policy and preview export inclusions. **Verify:** archiving does not silently cancel a still-authorized task; exports contain no blanket authority.
- [ ] **WS-01.T08 — Anti-wrapper fixture.** Close chat, restart, disable inference and change provider after creating a comparison application. **Verify:** layout, exclusions, files and source/evidence links remain directly usable.

**Closure evidence:** migrations, concurrency tests and the recorded second-visit fixture. Do not use a transcript replay to recreate the user's application. Export integrates with #52; evidence and action controls remain governed by their brokers.


---

<a id="issue-38"></a>
## #38 — [P1][WS-02] Personal, Original and Evidence views

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/38
**Created:** 2026-09-15T12:10:00Z | **Updated:** 2026-09-15T18:14:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #16 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #16

#### Objective
Implement the three synchronized product views: the user's generated Personal interface, the authentic Original service/page, and an inspectable Evidence/provenance view.

#### Scope
- Personal view renders user-owned reusable interfaces over normalized/source-backed state.
- Original view opens the authentic service/page in the correct profile/account.
- Evidence view shows source object/URI, observation time, source version, extractor/transform, supporting spans, freshness and contradiction state.
- Shared selection/navigation identity across the three views.
- Display badges for inferred/derived vs directly observed fields.
- One-click transition from a displayed fact/action to its evidence and original context.

#### Product rules
- Generated synthesis must never impersonate a publisher/service page.
- Evidence is attached to claims/fields, not merely dumped as a bibliography.
- Authentic source state remains authoritative for provider-specific account/transaction semantics.

#### Acceptance criteria
- [ ] A critical displayed fact can be traced from Personal view to evidence and source observation time.
- [ ] Original view opens the correct source/account for the selected object.
- [ ] Derived or inferred values are visually distinguishable from direct source facts.
- [ ] Contradictory credible evidence remains visible.
- [ ] Switching views does not lose selection, workspace navigation or layout state.
- [ ] Evidence access respects the same source/privacy permissions as the underlying record.

#### Dependencies
- WS-01
- DATA-01
- WEB-01

**First phase:** P1  
**Maturity target:** P2  
**Owner:** workspace-ui-data

#### Task issues
- [ ] #321 `WS-02.T01` — Define shared selection and view context
- [ ] #322 `WS-02.T02` — Implement source-backed Personal view bindings
- [ ] #323 `WS-02.T03` — Integrate authentic Original navigation
- [ ] #324 `WS-02.T04` — Implement field-level Evidence inspection
- [ ] #325 `WS-02.T05` — Add contradiction and revision navigation
- [ ] #326 `WS-02.T06` — Preserve focus, scroll and history across views
- [ ] #327 `WS-02.T07` — Test source identity and privacy consistency

### Discussion (1 comments)

#### Comment 5681980850 — Jordan-Hall — 2026-09-15T14:29:05Z

Source: https://github.com/Jordan-Hall/browser/issues/38#issuecomment-5681980850 | Updated: 2026-09-15T14:29:05Z

<!-- intent-implementation-v1:WS-02 -->
###### Implementation proposal — WS-02

Implement three surfaces over a shared ViewContext, not three divergent stores. Proposed locations: `apps/shell/views/{personal,original,evidence}` and `crates/view-context`; dependencies #37/#41/#11, with handoff from #36.

- [ ] **WS-02.T01 — Shared context.** Bind workspace selection to stable entity/claim/observation/source IDs and versions. **Verify:** switching surfaces does not rewrite or lose another surface's state.
- [ ] **WS-02.T02 — Personal bindings.** Render approved components from authorized projections; label observed/derived/inferred/stale/unknown fields. **Verify:** action proposals and source facts remain distinct.
- [ ] **WS-02.T03 — Authentic Original.** Resolve source/account locators through the browser host, retaining a return point and trusted origin/profile display. **Verify:** page titles cannot spoof the account/source identity.
- [ ] **WS-02.T04 — Field-level evidence.** Display supporting spans, observation/source times, revisions and transformation versions. **Verify:** evidence for a claim is distinguishable from unrelated pages visited.
- [ ] **WS-02.T05 — Contradictions/revisions.** Show support and contradiction within entity/time scope; connect invalidation reasons and historical versions. **Verify:** new evidence cannot silently erase credible disagreement.
- [ ] **WS-02.T06 — Focus/history continuity.** Retain per-surface reading/focus anchors while synchronizing object selection intentionally. **Verify:** keyboard/screen-reader return behavior remains predictable.
- [ ] **WS-02.T07 — Privacy/source tests.** Exercise account changes, deletion, private evidence, malicious rich text and outages across all caches/surfaces. **Verify:** no more permissive evidence/original path leaks restricted data.

**Definition of done:** a material displayed field can be traced to its actual observation and original source; all three views preserve access restrictions and workspace continuity. Clearly label synthesized content; never imitate a publisher's authoritative page.


---

<a id="issue-39"></a>
## #39 — [P1][WS-03] Trusted control surfaces and task centre

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/39
**Created:** 2026-09-15T12:10:11Z | **Updated:** 2026-09-15T18:15:44Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #16 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #16

#### Objective
Provide trusted, non-spoofable controls for identity, inference mode, task execution, approvals and human takeover.

#### Scope
- Persistent trusted chrome for active profile/account, inference mode, execution location and active tasks.
- Task centre showing lifecycle state, budget, provider, accessed resources, current step, failures and receipts.
- Pause, stop, take-over, approve, reject, revise and resume controls.
- Trusted confirmation surfaces rendered outside generated/provider content.
- Explicit state labels for proposed, attempted, committed, verified, failed and needs-reconciliation.
- Notification/approval queue hooks.

#### Security rules
- Generated UI/web content/provider UI cannot render or overlay a substitute trusted approval surface.
- Stop revokes future authority but cannot falsely claim already-committed external effects were undone.
- Account/provider/model location is visible before consequential work.

#### Acceptance criteria
- [ ] Generated/provider content cannot impersonate trusted approvals or global stop controls.
- [ ] User can stop/cancel while inference/browser workers are stalled.
- [ ] Attempted vs externally verified completion is visibly distinct.
- [ ] Take-over revokes or pauses the appropriate execution lease before user control begins.
- [ ] Task centre links to relevant evidence, actions, policy decisions and receipts.
- [ ] Execution location (`this device`, isolated session, home worker, cloud/provider) is visible.

#### Dependencies
- WS-01
- SEC-02
- CORE-03

**First phase:** P1  
**Maturity target:** P4  
**Owner:** workspace-ui-data

#### Task issues
- [ ] #328 `WS-03.T01` — Define trusted surface ownership
- [ ] #329 `WS-03.T02` — Build task projections and outcome labels
- [ ] #330 `WS-03.T03` — Implement pause, stop and takeover controls
- [ ] #331 `WS-03.T04` — Render exact approval proposals
- [ ] #332 `WS-03.T05` — Show inference mode and execution location
- [ ] #333 `WS-03.T06` — Implement audit and receipt drill-down
- [ ] #334 `WS-03.T07` — Handle stale notifications and concurrent decisions
- [ ] #335 `WS-03.T08` — Qualify trusted-control accessibility and stress

### Discussion (1 comments)

#### Comment 5681987679 — Jordan-Hall — 2026-09-15T14:29:27Z

Source: https://github.com/Jordan-Hall/browser/issues/39#issuecomment-5681987679 | Updated: 2026-09-15T14:29:27Z

<!-- intent-implementation-v1:WS-03 -->
###### Implementation proposal — WS-03

Create trusted chrome/task-centre modules and journal-derived projections, separate from generated/provider surfaces. Dependencies #37/#7/#4. Task state comes from the supervisor, not streamed provider prose.

- [ ] **WS-03.T01 — Trusted surface ownership.** Define protected composition/focus boundaries and accepted event sources. Generated content may propose an action, never mint an approval. **Verify:** hostile content cannot issue an actionable substitute confirmation.
- [ ] **WS-03.T02 — Task projections.** Derive state/provider/budget/resources/location/receipts from durable events. **Verify:** progress text cannot overwrite authoritative completed/uncertain state.
- [ ] **WS-03.T03 — Pause/stop/takeover.** Use reserved broker/supervisor channels and accessible keyboard controls; display local acknowledgement separately from process termination. **Verify:** busy or hung workers cannot prevent revocation.
- [ ] **WS-03.T04 — Exact approval views.** Render canonical broker fields: account, target, audience/recipient, destination, amount/ceiling, revision, expiry and consequences. **Verify:** material edits invalidate the displayed approval and require refresh.
- [ ] **WS-03.T05 — Mode/location disclosure.** Show offline/local/hybrid, selected provider/model and device/isolated-session/remote placement. **Verify:** permitted data destinations are visible before release.
- [ ] **WS-03.T06 — Audit/receipt drill-down.** Link decisions, accessed resources, evidence, artifacts, tests and receipts through authorized references. **Verify:** useful explanations require neither secret exposure nor private model reasoning.
- [ ] **WS-03.T07 — Stale/concurrent decisions.** Resolve notification actions against current proposal versions; handle multiwindow/device races with one-time intent. **Verify:** expired or consumed approvals cannot commit twice.
- [ ] **WS-03.T08 — Accessibility/stress.** Test screen readers, focus, zoom, overlapping windows, generated-content abuse and resource saturation. **Verify:** trusted controls remain reachable and measured separately from model latency.

**Review boundary:** distinguish proposed, prepared, attempted, accepted, verified and NeedsReconciliation. Stop is not undo. Anti-spoofing requires controlled event/composition paths plus recognizable trusted UI; do not promise immunity to all visual imitation by arbitrary host processes.


---

<a id="issue-40"></a>
## #40 — [P1][WS-04] Command palette, semantic selection and accessibility

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/40
**Created:** 2026-09-15T12:10:20Z | **Updated:** 2026-09-15T18:16:55Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #16 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #16

#### Objective
Make keyboard, direct manipulation, accessibility and semantic selection equal first-class interaction modes alongside voice/chat.

#### Scope
- Global command palette over workspaces, objects, actions, files, tasks and providers.
- Stable semantic selection model for rows/cards/entities/evidence/files/panels.
- Keyboard navigation, focus management, shortcuts and discoverability.
- IME/composition, screen-reader semantics, zoom/density, high-contrast/reduced-motion hooks and localization-ready text.
- Context handoff so commands such as “explain this” resolve explicit selection IDs.
- Deterministic global/local search over already indexed material.

#### Product rules
- Core workflows must not require speech or inference.
- Selection/context is explicit and scoped; no implicit unrestricted desktop capture.
- Generated layouts must preserve accessible names/roles/order.

#### Acceptance criteria
- [ ] Core workspace/research/browser/task flows complete using keyboard and assistive technology.
- [ ] Selection survives view refresh where the underlying object still exists.
- [ ] “this/these/current” commands resolve to explicit semantic targets and fail when ambiguous.
- [ ] IME and text composition work correctly in trusted and embedded browser surfaces.
- [ ] Zoom/density changes do not break trusted controls or action visibility.
- [ ] Accessibility regression tests cover generated component layouts.

#### Dependencies
- WS-01

**First phase:** P1  
**Maturity target:** P5  
**Owner:** workspace-ui-data

#### Task issues
- [ ] #336 `WS-04.T01` — Define command descriptors and routing
- [ ] #337 `WS-04.T02` — Implement explicit semantic selection
- [ ] #338 `WS-04.T03` — Build command palette and local search
- [ ] #339 `WS-04.T04` — Implement focus and input composition
- [ ] #340 `WS-04.T05` — Expose accessible component semantics
- [ ] #341 `WS-04.T06` — Implement zoom, density and preference controls
- [ ] #342 `WS-04.T07` — Integrate contextual voice targets
- [ ] #343 `WS-04.T08` — Run keyboard and assistive-technology conformance

### Discussion (1 comments)

#### Comment 5681992782 — Jordan-Hall — 2026-09-15T14:29:44Z

Source: https://github.com/Jordan-Hall/browser/issues/40#issuecomment-5681992782 | Updated: 2026-09-15T14:29:44Z

<!-- intent-implementation-v1:WS-04 -->
###### Implementation proposal — WS-04

Build `commands`, `selection`, accessibility adapters and the command palette over #37. CommandContext references explicit selected IDs/revisions; it is not an unrestricted view of the desktop.

- [ ] **WS-04.T01 — Command descriptors.** Register IDs, argument schemas, required scope, discoverability and shortcuts; route to typed services. **Verify:** command text cannot become arbitrary shell execution.
- [ ] **WS-04.T02 — Semantic selection.** Track single/multiple selected entities, claims, files, tasks and focus ownership with revisions. **Verify:** refresh preserves surviving identities and invalidates removed targets.
- [ ] **WS-04.T03 — Palette/local search.** Combine installed commands with permission-filtered exact object search and clear context labels. **Verify:** core navigation/discovery works with models disabled.
- [ ] **WS-04.T04 — Focus/IME.** Implement traversal, text selection and shell/browser transitions; defer command activation during composition. **Verify:** composed input and platform shortcuts do not trigger unintended actions.
- [ ] **WS-04.T05 — Accessible semantics.** Expose names/roles/states/values/relationships and meaningful announcements, including uncertainty/evidence labels. **Verify:** generated layouts preserve screen-reader order and stable identities.
- [ ] **WS-04.T06 — Zoom/density/preferences.** Apply scaling, contrast and motion tokens while keeping controls reachable. **Verify:** high zoom/narrow windows do not hide approval/stop paths.
- [ ] **WS-04.T07 — Contextual voice targets.** Give #66 a bounded current selection context; resolve pronouns deterministically and surface material ambiguity. **Verify:** “these” cannot silently target a stale selection or another account.
- [ ] **WS-04.T08 — Accessibility conformance.** Run navigation/source/download/code-review/approval flows with keyboard and supported assistive technology. **Verify:** report toolkit/OS/engine combinations and regression failures.

**Acceptance:** keyboard, direct manipulation and accessibility are peers of speech/chat, not fallbacks added later. The [WCAG 2.2 reference](https://www.w3.org/TR/WCAG22/) informs relevant web surfaces; native toolkit/OS behavior still needs independent tests. Proposed task plans do not claim conformance already exists.


---

<a id="issue-223"></a>
## #223 — [TASK][EPIC-WS.T01] Specify workspace navigation and trusted chrome

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/223
**Created:** 2026-09-15T15:25:33Z | **Updated:** 2026-09-15T15:25:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #16

### Original description

Parent: #16

Task ID: `EPIC-WS.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-224"></a>
## #224 — [TASK][EPIC-WS.T02] Integrate Personal, Original and Evidence representations

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/224
**Created:** 2026-09-15T15:25:38Z | **Updated:** 2026-09-15T15:25:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #16

### Original description

Parent: #16

Task ID: `EPIC-WS.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-225"></a>
## #225 — [TASK][EPIC-WS.T03] Integrate direct, keyboard and contextual controls

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/225
**Created:** 2026-09-15T15:25:42Z | **Updated:** 2026-09-15T15:25:42Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #16

### Original description

Parent: #16

Task ID: `EPIC-WS.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-226"></a>
## #226 — [TASK][EPIC-WS.T04] Validate workspace independence and recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/226
**Created:** 2026-09-15T15:25:50Z | **Updated:** 2026-09-15T15:25:50Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #16

### Original description

Parent: #16

Task ID: `EPIC-WS.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-313"></a>
## #313 — [TASK][WS-01.T01] Define workspace schema and revision ownership

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/313
**Created:** 2026-09-15T18:12:16Z | **Updated:** 2026-09-15T18:12:16Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #37

### Original description

Parent: #37

Task ID: `WS-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-314"></a>
## #314 — [TASK][WS-01.T02] Implement workspace lifecycle commands

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/314
**Created:** 2026-09-15T18:12:22Z | **Updated:** 2026-09-15T18:12:22Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #37

### Original description

Parent: #37

Task ID: `WS-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-315"></a>
## #315 — [TASK][WS-01.T03] Build workspace home and deterministic navigation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/315
**Created:** 2026-09-15T18:12:27Z | **Updated:** 2026-09-15T18:12:27Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #37

### Original description

Parent: #37

Task ID: `WS-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-316"></a>
## #316 — [TASK][WS-01.T04] Persist layouts, filters and user overlays separately

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/316
**Created:** 2026-09-15T18:12:32Z | **Updated:** 2026-09-15T18:12:32Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #37

### Original description

Parent: #37

Task ID: `WS-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-317"></a>
## #317 — [TASK][WS-01.T05] Implement navigation history and restoration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/317
**Created:** 2026-09-15T18:12:47Z | **Updated:** 2026-09-15T18:12:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #37

### Original description

Parent: #37

Task ID: `WS-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-318"></a>
## #318 — [TASK][WS-01.T06] Add artifact/task/source links and local search

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/318
**Created:** 2026-09-15T18:12:51Z | **Updated:** 2026-09-15T18:12:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #37

### Original description

Parent: #37

Task ID: `WS-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-319"></a>
## #319 — [TASK][WS-01.T07] Implement archive, export and retention handoff

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/319
**Created:** 2026-09-15T18:12:55Z | **Updated:** 2026-09-15T18:12:55Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #37

### Original description

Parent: #37

Task ID: `WS-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-320"></a>
## #320 — [TASK][WS-01.T08] Prove workspace independence from conversation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/320
**Created:** 2026-09-15T18:13:00Z | **Updated:** 2026-09-15T18:13:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #37

### Original description

Parent: #37

Task ID: `WS-01.T08`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-321"></a>
## #321 — [TASK][WS-02.T01] Define shared selection and view context

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/321
**Created:** 2026-09-15T18:13:09Z | **Updated:** 2026-09-15T18:13:09Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #38

### Original description

Parent: #38

Task ID: `WS-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-322"></a>
## #322 — [TASK][WS-02.T02] Implement source-backed Personal view bindings

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/322
**Created:** 2026-09-15T18:13:14Z | **Updated:** 2026-09-15T18:13:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #38

### Original description

Parent: #38

Task ID: `WS-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-323"></a>
## #323 — [TASK][WS-02.T03] Integrate authentic Original navigation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/323
**Created:** 2026-09-15T18:14:00Z | **Updated:** 2026-09-15T18:14:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #38

### Original description

Parent: #38

Task ID: `WS-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-324"></a>
## #324 — [TASK][WS-02.T04] Implement field-level Evidence inspection

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/324
**Created:** 2026-09-15T18:14:04Z | **Updated:** 2026-09-15T18:14:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #38

### Original description

Parent: #38

Task ID: `WS-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-325"></a>
## #325 — [TASK][WS-02.T05] Add contradiction and revision navigation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/325
**Created:** 2026-09-15T18:14:11Z | **Updated:** 2026-09-15T18:14:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #38

### Original description

Parent: #38

Task ID: `WS-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-326"></a>
## #326 — [TASK][WS-02.T06] Preserve focus, scroll and history across views

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/326
**Created:** 2026-09-15T18:14:14Z | **Updated:** 2026-09-15T18:14:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #38

### Original description

Parent: #38

Task ID: `WS-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-327"></a>
## #327 — [TASK][WS-02.T07] Test source identity and privacy consistency

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/327
**Created:** 2026-09-15T18:14:20Z | **Updated:** 2026-09-15T18:14:20Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #38

### Original description

Parent: #38

Task ID: `WS-02.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-328"></a>
## #328 — [TASK][WS-03.T01] Define trusted surface ownership

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/328
**Created:** 2026-09-15T18:14:41Z | **Updated:** 2026-09-15T18:14:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #39

### Original description

Parent: #39

Task ID: `WS-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-329"></a>
## #329 — [TASK][WS-03.T02] Build task projections and outcome labels

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/329
**Created:** 2026-09-15T18:14:46Z | **Updated:** 2026-09-15T18:14:46Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #39

### Original description

Parent: #39

Task ID: `WS-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-330"></a>
## #330 — [TASK][WS-03.T03] Implement pause, stop and takeover controls

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/330
**Created:** 2026-09-15T18:14:52Z | **Updated:** 2026-09-15T18:14:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #39

### Original description

Parent: #39

Task ID: `WS-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-331"></a>
## #331 — [TASK][WS-03.T04] Render exact approval proposals

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/331
**Created:** 2026-09-15T18:15:09Z | **Updated:** 2026-09-15T18:15:09Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #39

### Original description

Parent: #39

Task ID: `WS-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-332"></a>
## #332 — [TASK][WS-03.T05] Show inference mode and execution location

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/332
**Created:** 2026-09-15T18:15:14Z | **Updated:** 2026-09-15T18:15:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #39

### Original description

Parent: #39

Task ID: `WS-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-333"></a>
## #333 — [TASK][WS-03.T06] Implement audit and receipt drill-down

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/333
**Created:** 2026-09-15T18:15:19Z | **Updated:** 2026-09-15T18:15:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #39

### Original description

Parent: #39

Task ID: `WS-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-334"></a>
## #334 — [TASK][WS-03.T07] Handle stale notifications and concurrent decisions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/334
**Created:** 2026-09-15T18:15:24Z | **Updated:** 2026-09-15T18:15:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #39

### Original description

Parent: #39

Task ID: `WS-03.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-335"></a>
## #335 — [TASK][WS-03.T08] Qualify trusted-control accessibility and stress

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/335
**Created:** 2026-09-15T18:15:30Z | **Updated:** 2026-09-15T18:15:30Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #39

### Original description

Parent: #39

Task ID: `WS-03.T08`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-336"></a>
## #336 — [TASK][WS-04.T01] Define command descriptors and routing

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/336
**Created:** 2026-09-15T18:15:57Z | **Updated:** 2026-09-15T18:15:57Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #40

### Original description

Parent: #40

Task ID: `WS-04.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-337"></a>
## #337 — [TASK][WS-04.T02] Implement explicit semantic selection

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/337
**Created:** 2026-09-15T18:16:08Z | **Updated:** 2026-09-15T18:16:08Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #40

### Original description

Parent: #40

Task ID: `WS-04.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-338"></a>
## #338 — [TASK][WS-04.T03] Build command palette and local search

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/338
**Created:** 2026-09-15T18:16:12Z | **Updated:** 2026-09-15T18:16:12Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #40

### Original description

Parent: #40

Task ID: `WS-04.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-339"></a>
## #339 — [TASK][WS-04.T04] Implement focus and input composition

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/339
**Created:** 2026-09-15T18:16:19Z | **Updated:** 2026-09-15T18:16:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #40

### Original description

Parent: #40

Task ID: `WS-04.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-340"></a>
## #340 — [TASK][WS-04.T05] Expose accessible component semantics

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/340
**Created:** 2026-09-15T18:16:24Z | **Updated:** 2026-09-15T18:16:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #40

### Original description

Parent: #40

Task ID: `WS-04.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-341"></a>
## #341 — [TASK][WS-04.T06] Implement zoom, density and preference controls

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/341
**Created:** 2026-09-15T18:16:29Z | **Updated:** 2026-09-15T18:16:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #40

### Original description

Parent: #40

Task ID: `WS-04.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-342"></a>
## #342 — [TASK][WS-04.T07] Integrate contextual voice targets

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/342
**Created:** 2026-09-15T18:16:35Z | **Updated:** 2026-09-15T18:16:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #40

### Original description

Parent: #40

Task ID: `WS-04.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-343"></a>
## #343 — [TASK][WS-04.T08] Run keyboard and assistive-technology conformance

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/343
**Created:** 2026-09-15T18:16:39Z | **Updated:** 2026-09-15T18:16:39Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #40

### Original description

Parent: #40

Task ID: `WS-04.T08`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

