# Conventional browser and compatibility

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 41

## Contents

- [#15 — EPIC: Conventional browser and compatibility](#issue-15)
- [#11 — [P0][WEB-01] Isolated Chromium embedding](#issue-11)
- [#12 — [P1][WEB-02] Daily browser features and account profiles](#issue-12)
- [#35 — [P1][WEB-03] Semantic web observation and actuation](#issue-35)
- [#36 — [P1][WEB-04] Original-view continuity and engine abstraction](#issue-36)
- [#193 — [TASK][WEB-01.T01] Pin CEF artifacts and build provenance](#issue-193)
- [#194 — [TASK][WEB-01.T02] Prove shell and rendering integration](#issue-194)
- [#195 — [TASK][WEB-01.T03] Create the isolated browser process boundary](#issue-195)
- [#196 — [TASK][WEB-01.T04] Implement audited Rust FFI ownership](#issue-196)
- [#197 — [TASK][WEB-01.T05] Add navigation and identity events](#issue-197)
- [#198 — [TASK][WEB-01.T06] Implement profile/context isolation](#issue-198)
- [#199 — [TASK][WEB-01.T07] Add crash, GPU and update recovery](#issue-199)
- [#200 — [TASK][WEB-01.T08] Qualify isolation and production embedding](#issue-200)
- [#201 — [TASK][WEB-02.T01] Build tabs, windows and navigation history](#issue-201)
- [#202 — [TASK][WEB-02.T02] Implement profiles and private sessions](#issue-202)
- [#203 — [TASK][WEB-02.T03] Build bookmarks, imports and local history search](#issue-203)
- [#204 — [TASK][WEB-02.T04] Implement authentication and identity handoffs](#issue-204)
- [#205 — [TASK][WEB-02.T05] Implement site permissions and trusted prompts](#issue-205)
- [#206 — [TASK][WEB-02.T06] Build download and upload workflows](#issue-206)
- [#207 — [TASK][WEB-02.T07] Qualify media, popups, notifications and printing](#issue-207)
- [#208 — [TASK][WEB-02.T08] Implement browser crash and session restoration](#issue-208)
- [#209 — [TASK][WEB-02.T09] Define and implement extension compatibility](#issue-209)
- [#210 — [TASK][WEB-02.T10] Publish the browser feature support matrix](#issue-210)
- [#219 — [TASK][EPIC-WEB.T01] Qualify shell and engine integration](#issue-219)
- [#220 — [TASK][EPIC-WEB.T02] Integrate profiles and daily browser operations](#issue-220)
- [#221 — [TASK][EPIC-WEB.T03] Integrate semantic control and Original continuity](#issue-221)
- [#222 — [TASK][EPIC-WEB.T04] Qualify compatibility and patch readiness](#issue-222)
- [#299 — [TASK][WEB-03.T01] Model scoped web observations](#issue-299)
- [#300 — [TASK][WEB-03.T02] Implement DOM and accessibility extraction](#issue-300)
- [#301 — [TASK][WEB-03.T03] Implement restricted private control transport](#issue-301)
- [#302 — [TASK][WEB-03.T04] Resolve and revalidate action targets](#issue-302)
- [#303 — [TASK][WEB-03.T05] Implement bounded semantic actions](#issue-303)
- [#304 — [TASK][WEB-03.T06] Plan bounded visual fallback](#issue-304)
- [#305 — [TASK][WEB-03.T07] Verify postconditions and separate contexts](#issue-305)
- [#306 — [TASK][WEB-03.T08] Build semantic actuation regression corpus](#issue-306)
- [#307 — [TASK][WEB-04.T01] Define stable source and return locators](#issue-307)
- [#308 — [TASK][WEB-04.T02] Bind Original handoff to profile and account](#issue-308)
- [#309 — [TASK][WEB-04.T03] Implement continuity without unsafe replay](#issue-309)
- [#310 — [TASK][WEB-04.T04] Define the engine capability boundary](#issue-310)
- [#311 — [TASK][WEB-04.T05] Implement engine routing and safe fallback](#issue-311)
- [#312 — [TASK][WEB-04.T06] Qualify deep links and cross-view navigation](#issue-312)

---

<a id="issue-15"></a>
## #15 — EPIC: Conventional browser and compatibility

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/15
**Created:** 2026-09-15T12:06:49Z | **Updated:** 2026-09-15T14:21:18Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1

Own Chromium/CEF integration, daily browser behavior, semantic web observation/actuation and correct Original-view handoff. Chromium is a compatibility substrate, not the product's state model.

#### Child issues
- [ ] #11 WEB-01 — Isolated Chromium embedding
- [ ] #12 WEB-02 — Daily browser features and account profiles
- [ ] #35 WEB-03 — Semantic web observation and actuation
- [ ] #36 WEB-04 — Original-view continuity and engine abstraction

#### Cross-cutting gates
Renderer isolation, account/profile isolation, private control interfaces, origin freshness, postcondition verification and truthful compatibility support.

### Discussion (1 comments)

#### Comment 5681834469 — Jordan-Hall — 2026-09-15T14:21:18Z

Source: https://github.com/Jordan-Hall/browser/issues/15#issuecomment-5681834469 | Updated: 2026-09-15T14:21:18Z

<!-- intent-implementation-v1:EPIC-WEB -->
###### Workstream implementation and integration tasks

Integrate #11, #12, #35 and #36 behind the browser-engine contract; remote websites never own the trusted runtime.

- [ ] **EPIC-WEB.T01 — Qualify shell and engine integration.** Prove CEF surface composition, FFI ownership, sandboxing, focus, IME and accessibility on reference OS/GPU profiles. **Proof:** executable spike plus lifecycle/isolation tests before freezing the shell choice.
- [ ] **EPIC-WEB.T02 — Integrate profiles and daily operations.** Connect tabs, history, downloads/uploads, permissions, auth handoffs, media and printing to profile-scoped trusted UI. **Proof:** two same-origin accounts remain isolated through normal browsing and crash restore.
- [ ] **EPIC-WEB.T03 — Integrate semantic control and Original continuity.** Bind observations/actions to current origin/frame/account epochs and support authentic source navigation from workspace objects. **Proof:** stale elements fail, postconditions verify, and return navigation preserves workspace state.
- [ ] **EPIC-WEB.T04 — Qualify compatibility and patch readiness.** Publish feature coverage by engine/OS and run an independent engine-security update/recovery exercise. **Proof:** browser patching does not wait for unrelated feature releases.

**Demonstration:** inspect a fact in Personal, open its evidence and authentic source in the correct account, perform a bounded semantic action, crash the renderer and recover without losing the workspace.

**Decisions:** do not assume Chrome extension/DRM/password-manager parity from embedding CEF. Alternative engines must pass their own measured coverage; handoff must not pretend an unverified cart/session transfer succeeded.


---

<a id="issue-11"></a>
## #11 — [P0][WEB-01] Isolated Chromium embedding

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/11
**Created:** 2026-09-15T12:05:11Z | **Updated:** 2026-09-15T14:19:46Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Parent: #1

#### Objective
Provide production-grade legacy web compatibility without making remote web content part of the trusted application runtime.

#### Scope
- CEF/Chromium worker/process boundary and minimal audited Rust FFI.
- Window/surface embedding, navigation, crashes, renderer lifecycle and source/origin identity.
- Separate browser profiles/partitions; no privileged shell bridge exposed to pages.
- Authenticated private control channel owned by browser host, not remote JS.
- Browser engine version reporting and security-update hooks.

#### Acceptance criteria
- [ ] Remote content cannot invoke privileged native APIs or shell IPC.
- [ ] Renderer/browser-worker crash preserves trusted shell/workspace state.
- [ ] Origin/account identity is available to trusted UI and action verification.
- [ ] Debug/control interfaces are inaccessible from ordinary pages/local network.
- [ ] CEF version/update process is automated enough for emergency security patching.
- [ ] Integration spike proves text input, focus, accessibility and GPU rendering on target OS.

#### Dependencies
- CORE-01
- SEC-01

**First phase:** P0  
**Maturity target:** P1  
**Workstream:** Conventional browser and compatibility

### Discussion (1 comments)

#### Comment 5681803035 — Jordan-Hall — 2026-09-15T14:19:46Z

Source: https://github.com/Jordan-Hall/browser/issues/11#issuecomment-5681803035 | Updated: 2026-09-15T14:19:46Z

<!-- intent-implementation-v1:WEB-01 -->
###### Implementation proposal — WEB-01

Use an isolated CEF/Chromium compatibility worker; the Rust runtime—not remote page code—owns browser commands and workspace state. Proposed modules: `browser-host`, audited `cef-bridge`, `apps/browser-worker`, embedding fixtures. Prerequisites #2/#6; integrate lifecycle with #4.

- [ ] **WEB-01.T01 — Pin CEF/build provenance.** Record engine/native versions, checksums, licenses and packaging; first launch a minimal standalone host. **Verify:** clean-machine build with reproducible dependency identification.
- [ ] **WEB-01.T02 — Prove rendering/shell integration.** Compare native embedding and offscreen composition on reference GPUs/OSes; test DPI, resize, multiple windows, IME, accessibility, focus and video. **Verify:** no input misrouting during rapid lifecycle changes.
- [ ] **WEB-01.T03 — Isolate browser authority.** Launch through the supervisor, preserve Chromium sandboxing and expose only narrow private host commands. **Verify:** remote JavaScript cannot invoke files/policy/supervisor APIs; renderer death does not kill trusted chrome.
- [ ] **WEB-01.T04 — Audit Rust FFI ownership.** Wrap reference counts, callback lifetimes and thread affinity; prevent unwind across FFI; marshal events into owned queues. **Verify:** lifecycle stress/sanitizers catch stale callbacks, double release and use-after-free.
- [ ] **WEB-01.T05 — Navigation/identity events.** Track committed origin, frame, profile, security status, redirects and navigation epochs. **Verify:** redirects/frame replacement invalidate old observations before actuation.
- [ ] **WEB-01.T06 — Profile/context isolation.** Partition cookies/storage/cache; create explicit private and automation contexts; broker profile selection instead of arbitrary path arguments. **Verify:** same-origin accounts remain separated.
- [ ] **WEB-01.T07 — Crash/GPU/update recovery.** Persist safe navigation outside the worker; recreate surfaces on crash and retain uncertain actions for reconciliation. **Verify:** trusted controls/workspaces survive renderer or GPU-helper termination.
- [ ] **WEB-01.T08 — Production embedding qualification.** Run control-port/native-bridge probes plus input/accessibility suites; connect an independent CEF security-patch lane. **Verify:** every advertised configuration passes its support gate.

**Review decision:** freeze Iced versus another trusted shell only after the executable embedding spike. Never disable the Chromium sandbox to make a demo work, and do not promise extension/codec/passkey parity without #12 qualification. Reference: [CEF](https://github.com/chromiumembedded/cef).


---

<a id="issue-12"></a>
## #12 — [P1][WEB-02] Daily browser features and account profiles

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/12
**Created:** 2026-09-15T12:05:20Z | **Updated:** 2026-09-15T14:20:09Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Parent: #1

#### Objective
Make Original mode a dependable daily browser rather than a demo webview, while keeping profiles/account state isolated from AI and connector authority.

#### Scope
- Tabs/windows, back/forward, history, bookmarks, restore and private sessions.
- Authentication, OAuth handoff, passkeys/WebAuthn, MFA handoff and password-manager interoperability.
- Site permissions, downloads/uploads, media, popups, printing, file choosers and notifications.
- Import from common browsers where legally/technically feasible.
- Profile partitions for cookies/storage/cache and clear account identity in trusted chrome.
- Explicit extension-compatibility policy and feature support matrix.

#### Acceptance criteria
- [ ] Declared browser feature matrix passes on every advertised OS.
- [ ] Two profiles/accounts cannot leak cookies, cache, history or authenticated actions across boundaries.
- [ ] Passkey/MFA flows hand back correctly to the authenticated Original view.
- [ ] Download/upload/permission prompts cannot be spoofed by generated workspace UI.
- [ ] Crash/session restore returns to the correct profile and navigation state.
- [ ] Unsupported extension/browser features are disclosed rather than silently degraded.

#### Dependencies
- WEB-01
- SEC-03

**First phase:** P1  
**Maturity target:** P7  
**Workstream:** Conventional browser and compatibility

### Discussion (1 comments)

#### Comment 5681811496 — Jordan-Hall — 2026-09-15T14:20:08Z

Source: https://github.com/Jordan-Hall/browser/issues/12#issuecomment-5681811496 | Updated: 2026-09-15T14:20:08Z

<!-- intent-implementation-v1:WEB-02 -->
###### Implementation proposal — WEB-02

Implement Original mode as a real browser product over #11, with identity/secret handling from #8. Proposed records: TabState, WindowState, HistoryEntry, Bookmark, DownloadRecord, SitePermission, AuthHandoff and FeatureSupport.

- [ ] **WEB-02.T01 — Tabs/windows/navigation.** Persist stable IDs, profile ownership, selected/pinned tabs and navigation descriptors. **Verify:** moving/reopening tabs and restart retain the intended profile and target.
- [ ] **WEB-02.T02 — Profiles/private sessions.** Implement create/switch/lock/delete and ephemeral contexts with explicit retention. **Verify:** identical origins cannot share cookies/history across profiles; private sessions respect the declared non-persistence policy.
- [ ] **WEB-02.T03 — Bookmarks/import/history search.** Stage imports, preserve order, report malformed rows and deduplicate carefully; exclude password/session transfer. **Verify:** Unicode/duplicate/corrupt fixtures import without destroying user data.
- [ ] **WEB-02.T04 — Authentication handoffs.** Bind OAuth/custom-scheme callbacks and WebAuthn/MFA flows to profile, origin, state/nonce and initiating surface. **Verify:** replay/cross-profile callbacks fail and successful auth returns correctly.
- [ ] **WEB-02.T05 — Site permissions.** Render origin/profile-scoped camera, microphone, location, clipboard and notification prompts in trusted chrome; process live revocation. **Verify:** revoked capture stops and untrusted content cannot create an actionable substitute prompt.
- [ ] **WEB-02.T06 — Downloads/uploads.** Quarantine bounded downloads, handle safe resume/conflicts, and grant uploads through selected file handles. **Verify:** interruptions do not overwrite unrelated files; uploads cannot browse arbitrary paths.
- [ ] **WEB-02.T07 — Media/popups/printing.** Integrate supported playback, popup ownership, platform notifications and print-to-file with correct focus/origin behavior. **Verify:** required fixtures pass per OS/engine combination.
- [ ] **WEB-02.T08 — Crash/session restore.** Restore safe navigation and download state independently; revalidate interrupted auth/forms. **Verify:** restart never repeats consequential submissions automatically.
- [ ] **WEB-02.T09 — Browser-extension compatibility.** Implement and test the explicitly supported browser-extension API subset, separately from Intent extensions. **Verify:** permission updates/private-profile access fail closed when unsupported.
- [ ] **WEB-02.T10 — Support matrix.** Publish tested login, input, accessibility, permissions, media, printing and extension coverage by platform/CEF version. **Verify:** failing required features block a supported designation.

**Decisions to review:** proprietary codecs/DRM, password-manager integration and browser-extension coverage need named technical/licensing investigations. Do not treat CEF embedding as automatic Chrome parity. Preserve all scope; qualify support per operation instead of silently omitting difficult features.


---

<a id="issue-35"></a>
## #35 — [P1][WEB-03] Semantic web observation and actuation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/35
**Created:** 2026-09-15T12:09:30Z | **Updated:** 2026-09-15T14:27:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #15

#### Objective
Expose authenticated web state to agents/connectors as current semantic observations and bounded actions, using DOM/accessibility meaning before visual coordinates.

#### Scope
- DOM and accessibility-tree extraction with origin, frame, element identity and freshness metadata.
- Bounded actions: navigate, focus, invoke/click, input, select, scroll and submit where authorized.
- Isolated browser contexts for parallel read-only work.
- Private brokered CDP/debug transport; never expose a universal page-accessible control port.
- Revalidate origin/account/target immediately before actuation.
- Capture task-specific postconditions after actions.
- Permit bounded visual fallback only when semantic targeting is unavailable.

#### Security / correctness rules
- Page text, DOM attributes and tool descriptions are untrusted data, not authority.
- A stale element/frame handle cannot authorize a later action.
- Account/profile identity is part of every consequential web action.
- Coordinates require a fresh frame and bounded target; no blind click sequences.

#### Acceptance criteria
- [ ] Every action records current origin, profile/account, target, grant and observation version.
- [ ] Stale element/frame observations fail closed.
- [ ] Ordinary pages cannot discover or invoke the private debug/control interface.
- [ ] Postconditions distinguish attempted from verified completion.
- [ ] Parallel read-only browser sessions do not share authenticated mutable state unless explicitly configured.
- [ ] Visual fallback is visible in traces and separately measurable.

#### Tests
Navigation races, iframe/origin changes, DOM replacement, account switching, malicious page instructions, stale screenshot actions, renderer restarts and hidden-control-port probes.

#### Dependencies
- WEB-01
- SEC-02

**First phase:** P1  
**Maturity target:** P3  
**Owner:** platform

### Discussion (1 comments)

#### Comment 5681958434 — Jordan-Hall — 2026-09-15T14:27:52Z

Source: https://github.com/Jordan-Hall/browser/issues/35#issuecomment-5681958434 | Updated: 2026-09-15T14:27:52Z

<!-- intent-implementation-v1:WEB-03 -->
###### Implementation proposal — WEB-03

Build `browser-observation` and `browser-actions` over the private browser host. Prerequisites #11/#7. WebObservation must carry profile/context/origin/frame/navigation epoch/time; WebAction includes a current target, dispatch permit and expected postcondition.

- [ ] **WEB-03.T01 — Scoped observations.** Capture only authorized tabs/frames and redact password/out-of-scope fields before model context. **Verify:** cross-profile/private-field fixtures disclose nothing.
- [ ] **WEB-03.T02 — DOM/accessibility extraction.** Extract bounded roles, names, states and form semantics; refresh relevant subtrees incrementally. **Verify:** huge or malicious trees cannot exhaust the shell or become authority.
- [ ] **WEB-03.T03 — Private control transport.** Authenticate the worker channel and allowlist host operations; never forward arbitrary client-supplied CDP JSON. **Verify:** pages/local-network clients cannot discover an actionable universal debug endpoint.
- [ ] **WEB-03.T04 — Target resolution/revalidation.** Resolve within current account/origin/frame, reject ambiguous matches and recheck epochs immediately before action. **Verify:** replaced DOM/frame/account invalidates stale targets.
- [ ] **WEB-03.T05 — Typed actions.** Implement navigate/focus/invoke/value/select/scroll and authorized submit; files resolve through the file broker and consequential submits through transactions. **Verify:** no direct submit bypasses approval policy.
- [ ] **WEB-03.T06 — Visual fallback.** Capture an authorized region with scale/time/origin, ground one bounded target and revalidate before acting. **Verify:** stale frames reject and traces disclose visual mode/verification limits.
- [ ] **WEB-03.T07 — Postconditions/context isolation.** Check source/DOM/provider state independently and isolate parallel reads; lock shared authenticated mutation. **Verify:** attempted and verified outcomes remain distinct.
- [ ] **WEB-03.T08 — Regression corpus.** Test duplicate labels, hidden controls, races, account switches, crashes and stop/takeover. **Verify:** semantic and visual success/failure rates are reported separately.

**Review boundary:** fresh observations reduce races but cannot make arbitrary website state atomic. Higher-risk uncertain visual actions require supervision or authentic handoff, not blind coordinate sequences. Task IDs match handbook edition 1.0; preserve all original acceptance criteria.


---

<a id="issue-36"></a>
## #36 — [P1][WEB-04] Original-view continuity and engine abstraction

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/36
**Created:** 2026-09-15T12:09:38Z | **Updated:** 2026-09-15T14:28:13Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #15

#### Objective
Make switching between personalized workspace UI and authentic service UI reliable, while keeping the rendering engine replaceable behind a compatibility contract.

#### Scope
- Stable source/object deep links from Personal/Evidence views to Original view.
- Correct account/profile selection for handoff.
- Preserve safe navigation continuity: back/forward, workspace return point, relevant source location where supported.
- Bookmark/deep-link representation that can target workspace objects and original URLs.
- Browser-engine abstraction for navigation, observation, rendering surfaces and feature capability reporting.
- Explicit handoff metadata when state cannot be transferred between generated UI and original service.
- Servo/other engines plug into the same interface only for tested workloads.

#### Correctness rules
- Never claim cart/form/session state was transferred unless verified.
- Account mismatch must block or visibly require the correct profile.
- Source links remain authentic URLs/IDs; generated UI never impersonates the source.

#### Acceptance criteria
- [ ] A Personal-view entity can open the correct source object in the correct account context.
- [ ] Returning from Original view restores the workspace location and user layout state.
- [ ] Unsupported state transfer is explicitly disclosed rather than inferred.
- [ ] Engine capability differences are queryable and used for truthful fallback.
- [ ] Deep links survive application restart where source/account permissions still allow them.

#### Dependencies
- WEB-01
- WS-01

**First phase:** P1  
**Maturity target:** P5  
**Owner:** platform

### Discussion (1 comments)

#### Comment 5681964935 — Jordan-Hall — 2026-09-15T14:28:13Z

Source: https://github.com/Jordan-Hall/browser/issues/36#issuecomment-5681964935 | Updated: 2026-09-15T14:28:13Z

<!-- intent-implementation-v1:WEB-04 -->
###### Implementation proposal — WEB-04

Implement `source-navigation`, browser-engine adapters and the shell's Original surface. Dependencies #11/#37. Use SourceLocator, AccountBinding, HandoffIntent/Result, ReturnPoint and EngineCapabilities; transfer status must distinguish location-only from verified state transfer.

- [ ] **WEB-04.T01 — Source/return locators.** Keep canonical provider IDs/URLs separate from workspace/view/selection IDs; store semantic return anchors and layout revision. **Verify:** links survive restart and missing objects fail explicitly.
- [ ] **WEB-04.T02 — Account-bound handoff.** Resolve a source account to an authorized profile and verify identity where supported; surface ambiguous mappings. **Verify:** another logged-in account cannot silently receive the continuation.
- [ ] **WEB-04.T03 — Safe continuity.** Open authentic URLs/provider session links and record exactly which state transferred. Never replay forms to reconstruct a cart. **Verify:** return restores the workspace while unsupported transfer remains labelled.
- [ ] **WEB-04.T04 — Engine capability contract.** Define rendering, navigation, input, accessibility, observation and profile operations with explicit Unsupported responses. **Verify:** engine handles never escape adapters and unsupported methods cannot appear successful.
- [ ] **WEB-04.T05 — Routing/fallback.** Select engines by tested workload/configuration and recreate contexts only through a safe authentication path. **Verify:** fallback does not copy opaque cookie stores or resubmit an operation.
- [ ] **WEB-04.T06 — Continuity qualification.** Test redirects, deletion, expired auth, multiple profiles, restart, deep-link handlers and assistive-tech focus restoration. **Verify:** correct account/object/return context and no private-data leakage.

**Decision:** this abstraction enables #106 research before full browser maturity, but alternative-engine production routing needs its own compatibility evidence. A working source link is not evidence that checkout/session state was transferred. Proposed paths and contracts require review before implementation.


---

<a id="issue-193"></a>
## #193 — [TASK][WEB-01.T01] Pin CEF artifacts and build provenance

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/193
**Created:** 2026-09-15T15:21:08Z | **Updated:** 2026-09-15T15:21:08Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #11

### Original description

Parent: #11

Task ID: `WEB-01.T01`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-194"></a>
## #194 — [TASK][WEB-01.T02] Prove shell and rendering integration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/194
**Created:** 2026-09-15T15:21:14Z | **Updated:** 2026-09-15T15:21:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #11

### Original description

Parent: #11

Task ID: `WEB-01.T02`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-195"></a>
## #195 — [TASK][WEB-01.T03] Create the isolated browser process boundary

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/195
**Created:** 2026-09-15T15:21:22Z | **Updated:** 2026-09-15T15:21:22Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #11

### Original description

Parent: #11

Task ID: `WEB-01.T03`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-196"></a>
## #196 — [TASK][WEB-01.T04] Implement audited Rust FFI ownership

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/196
**Created:** 2026-09-15T15:21:28Z | **Updated:** 2026-09-15T15:21:28Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #11

### Original description

Parent: #11

Task ID: `WEB-01.T04`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-197"></a>
## #197 — [TASK][WEB-01.T05] Add navigation and identity events

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/197
**Created:** 2026-09-15T15:21:36Z | **Updated:** 2026-09-15T15:21:36Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #11

### Original description

Parent: #11

Task ID: `WEB-01.T05`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-198"></a>
## #198 — [TASK][WEB-01.T06] Implement profile/context isolation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/198
**Created:** 2026-09-15T15:21:41Z | **Updated:** 2026-09-15T15:21:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #11

### Original description

Parent: #11

Task ID: `WEB-01.T06`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-199"></a>
## #199 — [TASK][WEB-01.T07] Add crash, GPU and update recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/199
**Created:** 2026-09-15T15:21:47Z | **Updated:** 2026-09-15T15:21:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #11

### Original description

Parent: #11

Task ID: `WEB-01.T07`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-200"></a>
## #200 — [TASK][WEB-01.T08] Qualify isolation and production embedding

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/200
**Created:** 2026-09-15T15:21:54Z | **Updated:** 2026-09-15T15:21:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #11

### Original description

Parent: #11

Task ID: `WEB-01.T08`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-201"></a>
## #201 — [TASK][WEB-02.T01] Build tabs, windows and navigation history

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/201
**Created:** 2026-09-15T15:22:01Z | **Updated:** 2026-09-15T15:22:01Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #12

### Original description

Parent: #12

Task ID: `WEB-02.T01`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-202"></a>
## #202 — [TASK][WEB-02.T02] Implement profiles and private sessions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/202
**Created:** 2026-09-15T15:22:07Z | **Updated:** 2026-09-15T15:22:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #12

### Original description

Parent: #12

Task ID: `WEB-02.T02`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-203"></a>
## #203 — [TASK][WEB-02.T03] Build bookmarks, imports and local history search

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/203
**Created:** 2026-09-15T15:22:14Z | **Updated:** 2026-09-15T15:22:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #12

### Original description

Parent: #12

Task ID: `WEB-02.T03`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-204"></a>
## #204 — [TASK][WEB-02.T04] Implement authentication and identity handoffs

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/204
**Created:** 2026-09-15T15:22:21Z | **Updated:** 2026-09-15T15:22:21Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #12

### Original description

Parent: #12

Task ID: `WEB-02.T04`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-205"></a>
## #205 — [TASK][WEB-02.T05] Implement site permissions and trusted prompts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/205
**Created:** 2026-09-15T15:22:26Z | **Updated:** 2026-09-15T15:22:26Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #12

### Original description

Parent: #12

Task ID: `WEB-02.T05`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-206"></a>
## #206 — [TASK][WEB-02.T06] Build download and upload workflows

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/206
**Created:** 2026-09-15T15:22:36Z | **Updated:** 2026-09-15T15:22:36Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #12

### Original description

Parent: #12

Task ID: `WEB-02.T06`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-207"></a>
## #207 — [TASK][WEB-02.T07] Qualify media, popups, notifications and printing

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/207
**Created:** 2026-09-15T15:22:43Z | **Updated:** 2026-09-15T15:22:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #12

### Original description

Parent: #12

Task ID: `WEB-02.T07`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-208"></a>
## #208 — [TASK][WEB-02.T08] Implement browser crash and session restoration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/208
**Created:** 2026-09-15T15:22:49Z | **Updated:** 2026-09-15T15:22:49Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #12

### Original description

Parent: #12

Task ID: `WEB-02.T08`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-209"></a>
## #209 — [TASK][WEB-02.T09] Define and implement extension compatibility

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/209
**Created:** 2026-09-15T15:23:58Z | **Updated:** 2026-09-15T15:23:58Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #12

### Original description

Parent: #12

Task ID: `WEB-02.T09`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-210"></a>
## #210 — [TASK][WEB-02.T10] Publish the browser feature support matrix

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/210
**Created:** 2026-09-15T15:24:03Z | **Updated:** 2026-09-15T15:24:03Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #12

### Original description

Parent: #12

Task ID: `WEB-02.T10`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-219"></a>
## #219 — [TASK][EPIC-WEB.T01] Qualify shell and engine integration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/219
**Created:** 2026-09-15T15:25:08Z | **Updated:** 2026-09-15T15:25:08Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #15

### Original description

Parent: #15

Task ID: `EPIC-WEB.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-220"></a>
## #220 — [TASK][EPIC-WEB.T02] Integrate profiles and daily browser operations

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/220
**Created:** 2026-09-15T15:25:14Z | **Updated:** 2026-09-15T15:25:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #15

### Original description

Parent: #15

Task ID: `EPIC-WEB.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-221"></a>
## #221 — [TASK][EPIC-WEB.T03] Integrate semantic control and Original continuity

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/221
**Created:** 2026-09-15T15:25:20Z | **Updated:** 2026-09-15T15:25:20Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #15

### Original description

Parent: #15

Task ID: `EPIC-WEB.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-222"></a>
## #222 — [TASK][EPIC-WEB.T04] Qualify compatibility and patch readiness

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/222
**Created:** 2026-09-15T15:25:27Z | **Updated:** 2026-09-15T15:25:27Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #15

### Original description

Parent: #15

Task ID: `EPIC-WEB.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-299"></a>
## #299 — [TASK][WEB-03.T01] Model scoped web observations

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/299
**Created:** 2026-09-15T18:10:51Z | **Updated:** 2026-09-15T18:10:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #35

### Original description

Parent: #35

Task ID: `WEB-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-300"></a>
## #300 — [TASK][WEB-03.T02] Implement DOM and accessibility extraction

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/300
**Created:** 2026-09-15T18:10:56Z | **Updated:** 2026-09-15T18:10:56Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #35

### Original description

Parent: #35

Task ID: `WEB-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-301"></a>
## #301 — [TASK][WEB-03.T03] Implement restricted private control transport

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/301
**Created:** 2026-09-15T18:11:01Z | **Updated:** 2026-09-15T18:11:01Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #35

### Original description

Parent: #35

Task ID: `WEB-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-302"></a>
## #302 — [TASK][WEB-03.T04] Resolve and revalidate action targets

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/302
**Created:** 2026-09-15T18:11:06Z | **Updated:** 2026-09-15T18:11:06Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #35

### Original description

Parent: #35

Task ID: `WEB-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-303"></a>
## #303 — [TASK][WEB-03.T05] Implement bounded semantic actions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/303
**Created:** 2026-09-15T18:11:11Z | **Updated:** 2026-09-15T18:11:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #35

### Original description

Parent: #35

Task ID: `WEB-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-304"></a>
## #304 — [TASK][WEB-03.T06] Plan bounded visual fallback

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/304
**Created:** 2026-09-15T18:11:20Z | **Updated:** 2026-09-15T18:11:20Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #35

### Original description

Parent: #35

Task ID: `WEB-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-305"></a>
## #305 — [TASK][WEB-03.T07] Verify postconditions and separate contexts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/305
**Created:** 2026-09-15T18:11:24Z | **Updated:** 2026-09-15T18:11:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #35

### Original description

Parent: #35

Task ID: `WEB-03.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-306"></a>
## #306 — [TASK][WEB-03.T08] Build semantic actuation regression corpus

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/306
**Created:** 2026-09-15T18:11:29Z | **Updated:** 2026-09-15T18:11:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #35

### Original description

Parent: #35

Task ID: `WEB-03.T08`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-307"></a>
## #307 — [TASK][WEB-04.T01] Define stable source and return locators

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/307
**Created:** 2026-09-15T18:11:36Z | **Updated:** 2026-09-15T18:11:36Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #36

### Original description

Parent: #36

Task ID: `WEB-04.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-308"></a>
## #308 — [TASK][WEB-04.T02] Bind Original handoff to profile and account

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/308
**Created:** 2026-09-15T18:11:41Z | **Updated:** 2026-09-15T18:11:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #36

### Original description

Parent: #36

Task ID: `WEB-04.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-309"></a>
## #309 — [TASK][WEB-04.T03] Implement continuity without unsafe replay

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/309
**Created:** 2026-09-15T18:11:49Z | **Updated:** 2026-09-15T18:11:49Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #36

### Original description

Parent: #36

Task ID: `WEB-04.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-310"></a>
## #310 — [TASK][WEB-04.T04] Define the engine capability boundary

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/310
**Created:** 2026-09-15T18:11:53Z | **Updated:** 2026-09-15T18:11:53Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #36

### Original description

Parent: #36

Task ID: `WEB-04.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-311"></a>
## #311 — [TASK][WEB-04.T05] Implement engine routing and safe fallback

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/311
**Created:** 2026-09-15T18:11:56Z | **Updated:** 2026-09-15T18:11:56Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #36

### Original description

Parent: #36

Task ID: `WEB-04.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-312"></a>
## #312 — [TASK][WEB-04.T06] Qualify deep links and cross-view navigation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/312
**Created:** 2026-09-15T18:12:04Z | **Updated:** 2026-09-15T18:12:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #36

### Original description

Parent: #36

Task ID: `WEB-04.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

