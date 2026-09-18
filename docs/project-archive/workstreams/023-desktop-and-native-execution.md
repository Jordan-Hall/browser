# Desktop and native execution

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 38

## Contents

- [#23 — EPIC: Desktop and native execution](#issue-23)
- [#68 — [P0][PC-01] DesktopSession and physical-input leases](#issue-68)
- [#69 — [P0][PC-02] Windows native-control adapter](#issue-69)
- [#70 — [P0][PC-03] macOS and Linux native-control adapters](#issue-70)
- [#71 — [P3][PC-04] Controlled unattended desktop sessions](#issue-71)
- [#251 — [TASK][EPIC-PC.T01] Ratify DesktopSession and input ownership](#issue-251)
- [#252 — [TASK][EPIC-PC.T02] Integrate native platform backends](#issue-252)
- [#253 — [TASK][EPIC-PC.T03] Integrate isolated unattended sessions](#issue-253)
- [#254 — [TASK][EPIC-PC.T04] Qualify takeover, safety and recovery](#issue-254)
- [#536 — [TASK][PC-01.T01] Define scoped desktop observations and actions](#issue-536)
- [#537 — [TASK][PC-01.T02] Implement session and target identity](#issue-537)
- [#538 — [TASK][PC-01.T03] Implement exclusive input leases](#issue-538)
- [#539 — [TASK][PC-01.T04] Implement human takeover and reserved stop](#issue-539)
- [#540 — [TASK][PC-01.T05] Build preflight and postcondition verification](#issue-540)
- [#541 — [TASK][PC-01.T06] Add bounded visual fallback](#issue-541)
- [#542 — [TASK][PC-01.T07] Integrate brokered files/processes and handoffs](#issue-542)
- [#543 — [TASK][PC-01.T08] Qualify desktop safety and responsiveness](#issue-543)
- [#544 — [TASK][PC-02.T01] Implement Windows app/window inventory](#issue-544)
- [#545 — [TASK][PC-02.T02] Implement UIA extraction with bounded calls](#issue-545)
- [#546 — [TASK][PC-02.T03] Implement semantic patterns and text input](#issue-546)
- [#547 — [TASK][PC-02.T04] Integrate input leases and user takeover](#issue-547)
- [#548 — [TASK][PC-02.T05] Implement protected-surface handoffs](#issue-548)
- [#549 — [TASK][PC-02.T06] Implement bounded capture and app/file launching](#issue-549)
- [#550 — [TASK][PC-02.T07] Verify outcomes and qualify app coverage](#issue-550)
- [#551 — [TASK][PC-03.T01] Define backend capability matrices](#issue-551)
- [#552 — [TASK][PC-03.T02] Implement macOS permission and app identity](#issue-552)
- [#553 — [TASK][PC-03.T03] Implement macOS semantic observation and actions](#issue-553)
- [#554 — [TASK][PC-03.T04] Implement Linux semantic accessibility](#issue-554)
- [#555 — [TASK][PC-03.T05] Implement portal/compositor input and capture](#issue-555)
- [#556 — [TASK][PC-03.T06] Integrate leases, takeover and protected handoffs](#issue-556)
- [#557 — [TASK][PC-03.T07] Qualify native workflows independently](#issue-557)
- [#558 — [TASK][PC-04.T01] Choose session isolation and image strategy](#issue-558)
- [#559 — [TASK][PC-04.T02] Implement guest lifecycle and scoped channels](#issue-559)
- [#560 — [TASK][PC-04.T03] Implement file and credential injection](#issue-560)
- [#561 — [TASK][PC-04.T04] Implement independent input and human inspection](#issue-561)
- [#562 — [TASK][PC-04.T05] Implement snapshots, recovery and reconciliation](#issue-562)
- [#563 — [TASK][PC-04.T06] Implement recordings, retention and secure teardown](#issue-563)
- [#564 — [TASK][PC-04.T07] Qualify parallelism and guest isolation](#issue-564)

---

<a id="issue-23"></a>
## #23 — EPIC: Desktop and native execution

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/23
**Created:** 2026-09-15T12:07:56Z | **Updated:** 2026-09-15T14:23:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1

Own semantic computer control, platform accessibility backends, scoped filesystem/process APIs and isolated unattended desktop sessions.

#### Child issues
- [ ] #68 PC-01 — DesktopSession and physical-input leases
- [ ] #69 PC-02 — Windows native-control adapter
- [ ] #70 PC-03 — macOS and Linux native-control adapters
- [ ] #71 PC-04 — Controlled unattended desktop sessions

#### Cross-cutting gates
APIs/accessibility before pixels, fresh observations before actions, one physical-input lease, immediate human takeover, no privilege escalation/MFA/CAPTCHA bypass, scoped files/processes and independently verified postconditions.

### Discussion (1 comments)

#### Comment 5681881443 — Jordan-Hall — 2026-09-15T14:23:47Z

Source: https://github.com/Jordan-Hall/browser/issues/23#issuecomment-5681881443 | Updated: 2026-09-15T14:23:47Z

<!-- intent-implementation-v1:EPIC-PC -->
###### Workstream implementation and integration tasks

Implement #68–#71 as a scoped native-control service, with file/process operations supplied by #73.

- [ ] **EPIC-PC.T01 — Ratify DesktopSession/input ownership.** Agree target identities, observation freshness, grants, physical-input leases, takeover and postconditions with harness/security owners. **Proof:** one live input stream has one owner and stale targets cannot authorize buffered actions.
- [ ] **EPIC-PC.T02 — Integrate platform backends.** Implement Windows UIA, macOS accessibility and Linux accessibility/portal paths with separate consent/support matrices. **Proof:** semantic workflows verify effects; permission denial/revocation stops visibly.
- [ ] **EPIC-PC.T03 — Integrate isolated unattended sessions.** Allocate genuine VM/separate-session displays, scoped files, credentials and egress, plus explicit inspection/takeover. **Proof:** parallel sessions neither share credentials nor compete for physical input.
- [ ] **EPIC-PC.T04 — Qualify takeover/safety/recovery.** Exercise stale windows/frames, user intervention, protected prompts, crashes and unknown outcomes. **Proof:** every action has current scope/target evidence and a verified or honestly uncertain postcondition.

**Demonstration:** operate a permitted native app, reject a stale target, yield to manual input, then run an isolated unattended task without affecting the live desktop.

**Hard boundaries:** APIs/accessibility precede pixels; virtual-desktop organization is not security isolation; portals do not inherently provide semantic targets; OS login/MFA/CAPTCHA/elevation barriers require user handoff. No autonomous self-approval of accessibility or privilege grants.


---

<a id="issue-68"></a>
## #68 — [P0][PC-01] DesktopSession and physical-input leases

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/68
**Created:** 2026-09-15T12:15:07Z | **Updated:** 2026-09-15T19:40:26Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #23

#### Objective
Define the safe cross-platform computer-control contract: scoped observation, semantic actions, exclusive physical input and independent postcondition verification.

#### Scope
- `DesktopSession` with allowed applications/windows/regions and execution mode.
- Observation primitives: window inventory, accessibility subtree, selected content, focus/state and bounded screenshots.
- Action primitives: focus, semantic invoke, enter text, scroll, bounded pointer/key action and permitted file/app operations.
- Freshness/version metadata on observations.
- Exclusive physical keyboard/mouse lease with fencing token.
- User-activity detection and immediate yield/takeover.
- Pre-action revalidation and postcondition observation.

#### Safety rules
- Prefer application API → accessibility semantics → visual grounding, in that order.
- Stale screenshots/targets cannot authorize action sequences.
- One live physical input stream has one agent owner at a time.
- MFA/CAPTCHA/secure-desktop/privilege prompts require user handoff.

#### Acceptance criteria
- [ ] Manual input pauses/yields agent control immediately.
- [ ] Changed/stale windows/elements prevent dispatch until re-observed.
- [ ] Every supported action records target, grant, observation version and postcondition.
- [ ] Two workers cannot simultaneously hold the live physical-input lease.
- [ ] Cancellation invalidates the lease before later buffered output can act.
- [ ] Visual-coordinate fallback requires a fresh bounded frame and is traceable separately.

#### Dependencies
- SEC-02
- CORE-03

**First phase:** P0  
**Maturity target:** P3  
**Owner:** harness-pc-providers

### Discussion (2 comments)

#### Comment 5682231682 — Jordan-Hall — 2026-09-15T14:42:30Z

Source: https://github.com/Jordan-Hall/browser/issues/68#issuecomment-5682231682 | Updated: 2026-09-15T14:42:30Z

<!-- intent-implementation-v1:PC-01 -->
###### Implementation proposal — PC-01

Implement DesktopSession as a trusted scoped broker over #7/#4. Agents receive opaque WindowIdentity/SemanticTarget/VisualTarget references, not raw OS handles or global input authority.

- [ ] **PC-01.T01 — Observation/action contracts.** Define permitted apps/windows/regions, bounded semantic trees/text/screenshots with timestamps/epochs and typed focus/invoke/value/scroll/input operations. **Verify:** requests cannot observe outside their grant.
- [ ] **PC-01.T02 — Session/target identity.** Bind OS process/window handles to creation/generation and application identity; re-resolve after restarts/focus changes. **Verify:** recycled handles cannot make a stale target valid again.
- [ ] **PC-01.T03 — Exclusive input lease.** Acquire one fenced lease per physical input channel and serialize dispatch; distinguish read-only observation from input ownership. **Verify:** competing workers cannot simultaneously control the live desktop.
- [ ] **PC-01.T04 — Human takeover/stop.** Detect real user interaction, invalidate queued authority and yield before acknowledging takeover; keep stop independent of model/speech/browser work. **Verify:** late buffered actions cannot run after user intervention.
- [ ] **PC-01.T05 — Preflight/postconditions.** Check app/window/focus/element/grant/approval immediately before action and independently inspect the expected result afterward. **Verify:** attempted and verified outcomes remain separate.
- [ ] **PC-01.T06 — Visual fallback.** Use a fresh granted region, current display scale and window generation to resolve one bounded target. **Verify:** stale images reject; uncertain consequential steps require supervision rather than blind click sequences.
- [ ] **PC-01.T07 — Files/processes/protected handoff.** Delegate file/app/process operations to #73 instead of simulating everything with clicks; pause protected prompts for the user. **Verify:** no autonomous path approves its own OS permissions or elevates privilege.
- [ ] **PC-01.T08 — Safety/responsiveness qualification.** Test stale targets, concurrent workers, takeover races, app crashes, permission loss and model stalls against fixtures and supported OSes. **Verify:** identity, scope and lease assertions hold across failures.

**Control order:** authorized application APIs, then accessibility, then bounded vision. Broad OS accessibility consent does not remove the broker's narrower task restrictions. MFA/CAPTCHA/login/secure-desktop barriers require human handoff. Resource and observation races must be reported honestly, not treated as atomic external state.

#### Comment 5687049582 — Jordan-Hall — 2026-09-15T19:40:26Z

Source: https://github.com/Jordan-Hall/browser/issues/68#issuecomment-5687049582 | Updated: 2026-09-15T19:40:26Z

###### Task issues
- [ ] #536 `PC-01.T01` — Define scoped desktop observations and actions
- [ ] #537 `PC-01.T02` — Implement session and target identity
- [ ] #538 `PC-01.T03` — Implement exclusive input leases
- [ ] #539 `PC-01.T04` — Implement human takeover and reserved stop
- [ ] #540 `PC-01.T05` — Build preflight and postcondition verification
- [ ] #541 `PC-01.T06` — Add bounded visual fallback
- [ ] #542 `PC-01.T07` — Integrate brokered files/processes and handoffs
- [ ] #543 `PC-01.T08` — Qualify desktop safety and responsiveness


---

<a id="issue-69"></a>
## #69 — [P0][PC-02] Windows native-control adapter

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/69
**Created:** 2026-09-15T12:15:19Z | **Updated:** 2026-09-15T19:41:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #23

#### Objective
Implement the first production desktop-control backend on Windows using UI Automation and platform APIs, with bounded visual fallback only where necessary.

#### Scope
- Window/process enumeration and identity mapping.
- UI Automation tree extraction and stable element references where possible.
- Invoke/select/set-value/text/focus/scroll patterns.
- Application/file launch and handoff through scoped runtime capabilities.
- Bounded screenshot/visual grounding fallback for unsupported controls.
- User takeover detection and physical-input lease integration.
- Handling elevation boundaries, UAC/secure desktop, permission prompts and inaccessible controls.
- Reference workflow corpus across common native/Win32/UWP/Chromium/Electron apps.

#### Safety rules
- Never automate secure-desktop approval, UAC elevation, login protection, MFA or CAPTCHA bypass.
- Re-observe state after focus/window changes.
- Visual fallback does not inherit semantic confidence.

#### Acceptance criteria
- [ ] Reference Windows workflows complete using semantic UIA where supported and verify postconditions.
- [ ] UAC/secure-desktop and protected auth flows stop for user handoff.
- [ ] Changed/closed/replaced windows invalidate stale targets.
- [ ] Visual fallback is bounded and separately logged/evaluated.
- [ ] Manual user input relinquishes agent control.
- [ ] Accessibility/control failures return structured diagnostics rather than blind retries.

#### Dependencies
- PC-01

**First phase:** P0  
**Maturity target:** P3  
**Owner:** harness-pc-providers

### Discussion (2 comments)

#### Comment 5682237584 — Jordan-Hall — 2026-09-15T14:42:49Z

Source: https://github.com/Jordan-Hall/browser/issues/69#issuecomment-5682237584 | Updated: 2026-09-15T14:42:49Z

<!-- intent-implementation-v1:PC-02 -->
###### Implementation proposal — PC-02

Implement a Windows adapter behind #68 using supported UI Automation/application APIs. Keep COM/native handles inside an audited helper; return only scoped observations and action results.

- [ ] **PC-02.T01 — App/window inventory.** Enumerate authorized windows/processes with generation, session/integrity and application identity; treat titles as untrusted labels. **Verify:** scope filtering occurs before observations reach agents.
- [ ] **PC-02.T02 — Bounded UIA extraction.** Read roles/names/states/pattern support in an isolated helper with timeouts, subtree limits and protected-field redaction. **Verify:** hung/inaccessible controls cannot stall global stop or expose password fields.
- [ ] **PC-02.T03 — Semantic patterns/text.** Implement invoke, value/text, selection, expand/collapse, focus and scrolling with immediate state revalidation. **Verify:** unsupported patterns return structured failures; fallback input preserves IME/key semantics.
- [ ] **PC-02.T04 — Leases/takeover.** Enforce #68 ownership, detect manual input and invalidate queued actions before yielding. **Verify:** user focus changes require fresh observation before any continuation.
- [ ] **PC-02.T05 — Protected-surface handoff.** Detect access denial, elevated contexts, UAC and secure-desktop boundaries; provide trusted user guidance. **Verify:** agents cannot click their own privilege/permission/authentication approvals.
- [ ] **PC-02.T06 — Capture/launching.** Capture only granted regions with correct DPI mapping; route open/launch through #73. **Verify:** visual targeting is used only when semantics are unavailable and never grants broader filesystem access.
- [ ] **PC-02.T07 — Outcome/application qualification.** Test Win32, supported modern-native and embedded-browser apps with independent postconditions. **Verify:** publish OS/app/driver versions, inaccessible controls and semantic-versus-visual outcomes.

**Reference:** [Windows UI Automation](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-uiautomationoverview). Do not inject arbitrary code into target applications or disable OS protection to improve task coverage. UIA semantic identity reduces ambiguity but does not make a multi-step native workflow atomic; recheck state before each consequential step.

#### Comment 5687059502 — Jordan-Hall — 2026-09-15T19:41:14Z

Source: https://github.com/Jordan-Hall/browser/issues/69#issuecomment-5687059502 | Updated: 2026-09-15T19:41:14Z

###### Task issues
- [ ] #544 `PC-02.T01` — Implement Windows app/window inventory
- [ ] #545 `PC-02.T02` — Implement UIA extraction with bounded calls
- [ ] #546 `PC-02.T03` — Implement semantic patterns and text input
- [ ] #547 `PC-02.T04` — Integrate input leases and user takeover
- [ ] #548 `PC-02.T05` — Implement protected-surface handoffs
- [ ] #549 `PC-02.T06` — Implement bounded capture and app/file launching
- [ ] #550 `PC-02.T07` — Verify outcomes and qualify app coverage


---

<a id="issue-70"></a>
## #70 — [P0][PC-03] macOS and Linux native-control adapters

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/70
**Created:** 2026-09-15T12:15:31Z | **Updated:** 2026-09-15T19:42:01Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #23

#### Objective
Implement macOS and Linux backends for the common DesktopSession contract while respecting each platform's real accessibility, consent and input-control model.

#### Scope
- macOS AXUIElement-based semantic observation/action adapter and accessibility permission lifecycle.
- Linux accessibility integration plus user-authorized desktop/input portals where appropriate.
- Window/application identity, semantic trees, text/value/action patterns and focus state.
- Platform-specific consent/permission UX and revocation detection.
- Bounded visual fallback using the same PC-01 freshness/postcondition rules.
- Reference application/workflow corpus per OS/display stack.
- Structured capability matrix for operations that differ by platform/session type.

#### Design rules
- A remote-desktop/input portal is not treated as a semantic accessibility API.
- Permission grants are never self-approved by the general agent.
- Advertised capabilities are platform-specific and tested independently.

#### Acceptance criteria
- [ ] Declared macOS workflows pass using AX semantics where available.
- [ ] Declared Linux workflows pass on supported desktop/session configurations.
- [ ] Permission denial/revocation fails visibly and cannot be bypassed by the agent.
- [ ] Input portal access is not misrepresented as semantic target verification.
- [ ] Freshness, user takeover and postcondition behavior matches PC-01 contract.
- [ ] Unsupported desktop environments/operations are visible in the support matrix.

#### Dependencies
- PC-01

**First phase:** P0  
**Maturity target:** P5  
**Owner:** harness-pc-providers

### Discussion (2 comments)

#### Comment 5682242314 — Jordan-Hall — 2026-09-15T14:43:05Z

Source: https://github.com/Jordan-Hall/browser/issues/70#issuecomment-5682242314 | Updated: 2026-09-15T14:43:05Z

<!-- intent-implementation-v1:PC-03 -->
###### Implementation proposal — PC-03

Implement separate macOS and Linux backends behind #68. Semantic observation, input transport, capture permission and app launching remain distinct capabilities.

- [ ] **PC-03.T01 — Platform capability matrices.** Specify supported OS/display/session configurations and required user consent for semantic, visual, input and launch operations. **Verify:** unavailable capabilities cannot be presented as executable.
- [ ] **PC-03.T02 — macOS identity/permissions.** Use supported SDK interfaces to detect accessibility/screen-capture authorization and app/window identity; observe revocation and provide user-controlled setup. **Verify:** denial cannot be bypassed or self-approved by an agent.
- [ ] **PC-03.T03 — macOS semantic actions.** Bound AX tree reads and supported action/value calls in a helper with timeouts and generation/focus tracking. **Verify:** stale targets reject and postconditions map to the common DesktopSession result.
- [ ] **PC-03.T04 — Linux accessibility.** Integrate supported accessibility bus/toolkit semantics, bind app/window/node identity and bound subtree reads. **Verify:** missing semantics are explicit rather than guessed from available input access.
- [ ] **PC-03.T05 — Portal/compositor input and capture.** Negotiate authorized sessions, track revocation and current display transforms. **Verify:** captured coordinates remain bound to the current session; input permission does not count as semantic target verification.
- [ ] **PC-03.T06 — Lease/takeover/handoff integration.** Apply exclusive physical-input ownership and immediate human yield; reobserve after intervention and stop at protected prompts. **Verify:** neither platform can continue from queued stale input after revocation.
- [ ] **PC-03.T07 — Independent OS qualification.** Test supported macOS versions and Linux desktop/session combinations for keyboard/accessibility/capture/revocation/restart. **Verify:** report per-configuration results and gaps, not a single cross-platform compatibility claim.

**References:** [AXUIElement SDK reference](https://developer.apple.com/documentation/applicationservices/axuielement), [XDG RemoteDesktop portal](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html). Validate exact SDK behavior with executable spikes. Do not rely on undisclosed private APIs or circumvent privacy consent; a portal is not itself an accessibility tree.

#### Comment 5687069388 — Jordan-Hall — 2026-09-15T19:42:00Z

Source: https://github.com/Jordan-Hall/browser/issues/70#issuecomment-5687069388 | Updated: 2026-09-15T19:42:00Z

###### Task issues
- [ ] #551 `PC-03.T01` — Define backend capability matrices
- [ ] #552 `PC-03.T02` — Implement macOS permission and app identity
- [ ] #553 `PC-03.T03` — Implement macOS semantic observation and actions
- [ ] #554 `PC-03.T04` — Implement Linux semantic accessibility
- [ ] #555 `PC-03.T05` — Implement portal/compositor input and capture
- [ ] #556 `PC-03.T06` — Integrate leases, takeover and protected handoffs
- [ ] #557 `PC-03.T07` — Qualify native workflows independently


---

<a id="issue-71"></a>
## #71 — [P3][PC-04] Controlled unattended desktop sessions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/71
**Created:** 2026-09-15T12:15:41Z | **Updated:** 2026-09-15T19:42:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #23

#### Objective
Support unattended/native application work in isolated desktop sessions or VMs without sharing the user's live physical input stream or broad host credentials.

#### Scope
- Dedicated local/VM desktop session lifecycle: create, boot/login where authorized, execute, suspend, snapshot, recover, destroy.
- Scoped file mounts and credential handles; no ambient host home/profile access.
- Separate virtual display/input channel per session.
- Session recording/screenshot retention controls and privacy metadata.
- Network/egress policy per session.
- Crash/reboot recovery and task reconciliation.
- Human inspection/takeover path that clearly indicates this is not the live desktop.

#### Isolation rules
- OS virtual desktops alone do not count as independent security/input boundaries.
- Parallel unattended jobs require genuinely isolated sessions.
- Session secrets/state must not become visible to other sessions/accounts.

#### Acceptance criteria
- [ ] Two parallel sessions can execute without sharing one keyboard/mouse stream.
- [ ] Each session sees only its scoped files, credentials and destinations.
- [ ] Session crash/restart preserves task state without blindly repeating external writes.
- [ ] Captured visual data follows explicit retention/sensitivity policy.
- [ ] Leaked state from one fixture session cannot access another account/session.
- [ ] UI clearly distinguishes live co-pilot control from isolated unattended execution.

#### Dependencies
- PC-01
- SEC-04

**First phase:** P3  
**Maturity target:** P5  
**Owner:** harness-pc-providers

### Discussion (2 comments)

#### Comment 5682246731 — Jordan-Hall — 2026-09-15T14:43:21Z

Source: https://github.com/Jordan-Hall/browser/issues/71#issuecomment-5682246731 | Updated: 2026-09-15T14:43:21Z

<!-- intent-implementation-v1:PC-04 -->
###### Implementation proposal — PC-04

Run unattended native work in genuinely isolated sessions/VMs over #68/#9, initially using resettable versioned images. Virtual desktops alone do not provide independent authority/input boundaries.

- [ ] **PC-04.T01 — Isolation/image choice.** Evaluate app compatibility, independent input, graphics, licensing and containment; pin signed base-image manifests and separate writable task overlays. **Verify:** reusable images contain no live credentials or user data.
- [ ] **PC-04.T02 — Guest lifecycle/channels.** Create/start/health/suspend/destroy through #4 with authenticated host/guest IPC and unique epochs. **Verify:** guests receive no broad shared home folder or universal forwarded control port.
- [ ] **PC-04.T03 — Files/credentials.** Transfer approved inputs through bounded validated channels, inject narrowly scoped short-lived credentials where supported and quarantine outputs. **Verify:** other sessions cannot read task secrets; teardown removes eligible credentials.
- [ ] **PC-04.T04 — Independent input/inspection.** Attach a separate #68 channel to each guest; label guest takeover distinctly from live-desktop control and limit concurrency by host resources. **Verify:** parallel tasks cannot share one input stream accidentally.
- [ ] **PC-04.T05 — Checkpoints/reconciliation.** Snapshot safe guest state with versions and revalidate grants/deadlines before restore; retain external operation truth outside guest snapshots. **Verify:** snapshot rollback cannot resubmit an already accepted purchase/message.
- [ ] **PC-04.T06 — Recording/retention/teardown.** Encrypt and bound authorized screenshots/recordings; preserve only necessary evidence before deleting overlays. **Verify:** snapshots and captures follow source sensitivity and retention rather than being unrestricted debug logs.
- [ ] **PC-04.T07 — Isolation qualification.** Test parallel guests, hostile probes, egress denial, host pressure and snapshot/crash recovery on pinned images/apps. **Verify:** no cross-session credential/file leakage and clear supported-session limits.

**Important:** a guest snapshot may contain plaintext credentials and private screens; encryption/retention must cover the entire snapshot. Guest destruction does not cancel external commitments. Preserve unresolved transaction evidence until reconciliation and make the execution location visible throughout.

#### Comment 5687080050 — Jordan-Hall — 2026-09-15T19:42:54Z

Source: https://github.com/Jordan-Hall/browser/issues/71#issuecomment-5687080050 | Updated: 2026-09-15T19:42:54Z

###### Task issues
- [ ] #558 `PC-04.T01` — Choose session isolation and image strategy
- [ ] #559 `PC-04.T02` — Implement guest lifecycle and scoped channels
- [ ] #560 `PC-04.T03` — Implement file and credential injection
- [ ] #561 `PC-04.T04` — Implement independent input and human inspection
- [ ] #562 `PC-04.T05` — Implement snapshots, recovery and reconciliation
- [ ] #563 `PC-04.T06` — Implement recordings, retention and secure teardown
- [ ] #564 `PC-04.T07` — Qualify parallelism and guest isolation


---

<a id="issue-251"></a>
## #251 — [TASK][EPIC-PC.T01] Ratify DesktopSession and input ownership

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/251
**Created:** 2026-09-15T15:28:36Z | **Updated:** 2026-09-15T15:28:36Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #23

### Original description

Parent: #23

Task ID: `EPIC-PC.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-252"></a>
## #252 — [TASK][EPIC-PC.T02] Integrate native platform backends

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/252
**Created:** 2026-09-15T15:28:43Z | **Updated:** 2026-09-15T15:28:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #23

### Original description

Parent: #23

Task ID: `EPIC-PC.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-253"></a>
## #253 — [TASK][EPIC-PC.T03] Integrate isolated unattended sessions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/253
**Created:** 2026-09-15T15:28:49Z | **Updated:** 2026-09-15T15:28:49Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #23

### Original description

Parent: #23

Task ID: `EPIC-PC.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-254"></a>
## #254 — [TASK][EPIC-PC.T04] Qualify takeover, safety and recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/254
**Created:** 2026-09-15T15:28:55Z | **Updated:** 2026-09-15T15:28:55Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #23

### Original description

Parent: #23

Task ID: `EPIC-PC.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-536"></a>
## #536 — [TASK][PC-01.T01] Define scoped desktop observations and actions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/536
**Created:** 2026-09-15T19:39:29Z | **Updated:** 2026-09-15T19:39:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #68

### Original description

Parent: #68

Task ID: `PC-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-537"></a>
## #537 — [TASK][PC-01.T02] Implement session and target identity

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/537
**Created:** 2026-09-15T19:39:34Z | **Updated:** 2026-09-15T19:39:34Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #68

### Original description

Parent: #68

Task ID: `PC-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-538"></a>
## #538 — [TASK][PC-01.T03] Implement exclusive input leases

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/538
**Created:** 2026-09-15T19:39:43Z | **Updated:** 2026-09-15T19:39:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #68

### Original description

Parent: #68

Task ID: `PC-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-539"></a>
## #539 — [TASK][PC-01.T04] Implement human takeover and reserved stop

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/539
**Created:** 2026-09-15T19:39:53Z | **Updated:** 2026-09-15T19:39:53Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #68

### Original description

Parent: #68

Task ID: `PC-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-540"></a>
## #540 — [TASK][PC-01.T05] Build preflight and postcondition verification

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/540
**Created:** 2026-09-15T19:40:00Z | **Updated:** 2026-09-15T19:40:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #68

### Original description

Parent: #68

Task ID: `PC-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-541"></a>
## #541 — [TASK][PC-01.T06] Add bounded visual fallback

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/541
**Created:** 2026-09-15T19:40:09Z | **Updated:** 2026-09-15T19:40:09Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #68

### Original description

Parent: #68

Task ID: `PC-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-542"></a>
## #542 — [TASK][PC-01.T07] Integrate brokered files/processes and handoffs

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/542
**Created:** 2026-09-15T19:40:14Z | **Updated:** 2026-09-15T19:40:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #68

### Original description

Parent: #68

Task ID: `PC-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-543"></a>
## #543 — [TASK][PC-01.T08] Qualify desktop safety and responsiveness

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/543
**Created:** 2026-09-15T19:40:19Z | **Updated:** 2026-09-15T19:40:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #68

### Original description

Parent: #68

Task ID: `PC-01.T08`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-544"></a>
## #544 — [TASK][PC-02.T01] Implement Windows app/window inventory

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/544
**Created:** 2026-09-15T19:40:32Z | **Updated:** 2026-09-15T19:40:32Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #69

### Original description

Parent: #69

Task ID: `PC-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-545"></a>
## #545 — [TASK][PC-02.T02] Implement UIA extraction with bounded calls

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/545
**Created:** 2026-09-15T19:40:38Z | **Updated:** 2026-09-15T19:40:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #69

### Original description

Parent: #69

Task ID: `PC-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-546"></a>
## #546 — [TASK][PC-02.T03] Implement semantic patterns and text input

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/546
**Created:** 2026-09-15T19:40:42Z | **Updated:** 2026-09-15T19:40:42Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #69

### Original description

Parent: #69

Task ID: `PC-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-547"></a>
## #547 — [TASK][PC-02.T04] Integrate input leases and user takeover

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/547
**Created:** 2026-09-15T19:40:49Z | **Updated:** 2026-09-15T19:40:49Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #69

### Original description

Parent: #69

Task ID: `PC-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-548"></a>
## #548 — [TASK][PC-02.T05] Implement protected-surface handoffs

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/548
**Created:** 2026-09-15T19:40:54Z | **Updated:** 2026-09-15T19:40:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #69

### Original description

Parent: #69

Task ID: `PC-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-549"></a>
## #549 — [TASK][PC-02.T06] Implement bounded capture and app/file launching

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/549
**Created:** 2026-09-15T19:41:02Z | **Updated:** 2026-09-15T19:41:02Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #69

### Original description

Parent: #69

Task ID: `PC-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-550"></a>
## #550 — [TASK][PC-02.T07] Verify outcomes and qualify app coverage

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/550
**Created:** 2026-09-15T19:41:07Z | **Updated:** 2026-09-15T19:41:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #69

### Original description

Parent: #69

Task ID: `PC-02.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-551"></a>
## #551 — [TASK][PC-03.T01] Define backend capability matrices

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/551
**Created:** 2026-09-15T19:41:21Z | **Updated:** 2026-09-15T19:41:21Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #70

### Original description

Parent: #70

Task ID: `PC-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-552"></a>
## #552 — [TASK][PC-03.T02] Implement macOS permission and app identity

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/552
**Created:** 2026-09-15T19:41:26Z | **Updated:** 2026-09-15T19:41:26Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #70

### Original description

Parent: #70

Task ID: `PC-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-553"></a>
## #553 — [TASK][PC-03.T03] Implement macOS semantic observation and actions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/553
**Created:** 2026-09-15T19:41:30Z | **Updated:** 2026-09-15T19:41:30Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #70

### Original description

Parent: #70

Task ID: `PC-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-554"></a>
## #554 — [TASK][PC-03.T04] Implement Linux semantic accessibility

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/554
**Created:** 2026-09-15T19:41:35Z | **Updated:** 2026-09-15T19:41:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #70

### Original description

Parent: #70

Task ID: `PC-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-555"></a>
## #555 — [TASK][PC-03.T05] Implement portal/compositor input and capture

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/555
**Created:** 2026-09-15T19:41:39Z | **Updated:** 2026-09-15T19:41:39Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #70

### Original description

Parent: #70

Task ID: `PC-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-556"></a>
## #556 — [TASK][PC-03.T06] Integrate leases, takeover and protected handoffs

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/556
**Created:** 2026-09-15T19:41:48Z | **Updated:** 2026-09-15T19:41:48Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #70

### Original description

Parent: #70

Task ID: `PC-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-557"></a>
## #557 — [TASK][PC-03.T07] Qualify native workflows independently

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/557
**Created:** 2026-09-15T19:41:53Z | **Updated:** 2026-09-15T19:41:53Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #70

### Original description

Parent: #70

Task ID: `PC-03.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-558"></a>
## #558 — [TASK][PC-04.T01] Choose session isolation and image strategy

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/558
**Created:** 2026-09-15T19:42:13Z | **Updated:** 2026-09-15T19:42:13Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #71

### Original description

Parent: #71

Task ID: `PC-04.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-559"></a>
## #559 — [TASK][PC-04.T02] Implement guest lifecycle and scoped channels

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/559
**Created:** 2026-09-15T19:42:18Z | **Updated:** 2026-09-15T19:42:18Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #71

### Original description

Parent: #71

Task ID: `PC-04.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-560"></a>
## #560 — [TASK][PC-04.T03] Implement file and credential injection

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/560
**Created:** 2026-09-15T19:42:24Z | **Updated:** 2026-09-15T19:42:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #71

### Original description

Parent: #71

Task ID: `PC-04.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-561"></a>
## #561 — [TASK][PC-04.T04] Implement independent input and human inspection

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/561
**Created:** 2026-09-15T19:42:30Z | **Updated:** 2026-09-15T19:42:30Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #71

### Original description

Parent: #71

Task ID: `PC-04.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-562"></a>
## #562 — [TASK][PC-04.T05] Implement snapshots, recovery and reconciliation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/562
**Created:** 2026-09-15T19:42:36Z | **Updated:** 2026-09-15T19:42:36Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #71

### Original description

Parent: #71

Task ID: `PC-04.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-563"></a>
## #563 — [TASK][PC-04.T06] Implement recordings, retention and secure teardown

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/563
**Created:** 2026-09-15T19:42:42Z | **Updated:** 2026-09-15T19:42:42Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #71

### Original description

Parent: #71

Task ID: `PC-04.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-564"></a>
## #564 — [TASK][PC-04.T07] Qualify parallelism and guest isolation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/564
**Created:** 2026-09-15T19:42:48Z | **Updated:** 2026-09-15T19:42:48Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #71

### Original description

Parent: #71

Task ID: `PC-04.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

