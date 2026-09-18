# Sync, mobile and collaboration

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 36

## Contents

- [#31 — EPIC: Sync, mobile and collaboration](#issue-31)
- [#94 — [P5][MESH-01] Encrypted sync, pairing and key recovery](#issue-94)
- [#95 — [P5][MESH-02] Device authority and remote/home workers](#issue-95)
- [#96 — [P5][MESH-03] Shared workspaces and enterprise contexts](#issue-96)
- [#97 — [P5][MESH-04] Mobile and additional presentation clients](#issue-97)
- [#283 — [TASK][EPIC-MESH.T01] Ratify device, sharing and authority boundaries](#issue-283)
- [#284 — [TASK][EPIC-MESH.T02] Integrate pairing, encrypted sync and recovery](#issue-284)
- [#285 — [TASK][EPIC-MESH.T03] Integrate remote workers, collaboration and clients](#issue-285)
- [#286 — [TASK][EPIC-MESH.T04] Qualify partitions, revocation and cross-client trust](#issue-286)
- [#715 — [TASK][MESH-01.T01] Specify sync eligibility and cryptographic threat model](#issue-715)
- [#716 — [TASK][MESH-01.T02] Implement explicit device pairing and identity](#issue-716)
- [#717 — [TASK][MESH-01.T03] Implement encrypted record transport and replay defense](#issue-717)
- [#718 — [TASK][MESH-01.T04] Implement type-specific conflict resolution](#issue-718)
- [#719 — [TASK][MESH-01.T05] Implement revocation and key rotation](#issue-719)
- [#720 — [TASK][MESH-01.T06] Implement recovery, export and bandwidth controls](#issue-720)
- [#721 — [TASK][MESH-01.T07] Qualify sync under partitions and malicious relay behavior](#issue-721)
- [#722 — [TASK][MESH-02.T01] Define worker capabilities and enrollment](#issue-722)
- [#723 — [TASK][MESH-02.T02] Implement explicit placement policy and preview](#issue-723)
- [#724 — [TASK][MESH-02.T03] Transfer minimal scoped task context](#issue-724)
- [#725 — [TASK][MESH-02.T04] Implement single-authority dispatch and fencing](#issue-725)
- [#726 — [TASK][MESH-02.T05] Implement remote progress, cancellation and results](#issue-726)
- [#727 — [TASK][MESH-02.T06] Implement relocation and recovery](#issue-727)
- [#728 — [TASK][MESH-02.T07] Qualify mesh execution and placement truthfulness](#issue-728)
- [#729 — [TASK][MESH-03.T01] Define roles, membership and policy boundaries](#issue-729)
- [#730 — [TASK][MESH-03.T02] Implement recipient-specific source projections](#issue-730)
- [#731 — [TASK][MESH-03.T03] Implement collaborative notes, layouts and tasks](#issue-731)
- [#732 — [TASK][MESH-03.T04] Implement approval ownership and delegated team actions](#issue-732)
- [#733 — [TASK][MESH-03.T05] Implement revocation and safe shared exports](#issue-733)
- [#734 — [TASK][MESH-03.T06] Qualify mixed-access collaboration](#issue-734)
- [#735 — [TASK][MESH-04.T01] Define mobile capability and platform support contracts](#issue-735)
- [#736 — [TASK][MESH-04.T02] Implement secure pairing and local storage](#issue-736)
- [#737 — [TASK][MESH-04.T03] Build reading, evidence and workspace navigation](#issue-737)
- [#738 — [TASK][MESH-04.T04] Implement voice capture and explicit handoff](#issue-738)
- [#739 — [TASK][MESH-04.T05] Implement trusted remote approval surfaces](#issue-739)
- [#740 — [TASK][MESH-04.T06] Implement notifications and client lifecycle recovery](#issue-740)
- [#741 — [TASK][MESH-04.T07] Qualify mobile and experimental clients](#issue-741)

---

<a id="issue-31"></a>
## #31 — EPIC: Sync, mobile and collaboration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/31
**Created:** 2026-09-15T12:08:47Z | **Updated:** 2026-09-15T14:26:16Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #1 | **Body-declared parent:** #1
**Native child issues:** #94, #95, #96, #97

### Original description

Programme: #1

Own encrypted multi-device state, pairing/key recovery, explicit device authority, home/remote workers, shared workspaces and additional clients.

#### Child issues
- [ ] #94 MESH-01 — Encrypted sync, pairing and key recovery
- [ ] #95 MESH-02 — Device authority and remote/home workers
- [ ] #96 MESH-03 — Shared workspaces and enterprise contexts
- [ ] #97 MESH-04 — Mobile and additional presentation clients

#### Cross-cutting gates
Revoked devices lose future access, risky actions are single-authority/fenced across partitions, shared views never confer the union of collaborators' source rights, execution location/data flow is visible, and mobile does not inherit desktop-control assumptions.

### Discussion (1 comments)

#### Comment 5681928128 — Jordan-Hall — 2026-09-15T14:26:16Z

Source: https://github.com/Jordan-Hall/browser/issues/31#issuecomment-5681928128 | Updated: 2026-09-15T14:26:16Z

<!-- intent-implementation-v1:EPIC-MESH -->
###### Workstream implementation and integration tasks

Integrate #94–#97 as explicit paired-device and collaboration capabilities, not blanket account replication.

- [ ] **EPIC-MESH.T01 — Ratify device/sharing/authority boundaries.** Separate replicated notes/layouts from credentials, approvals, reservations and commit authority. Define recipient-specific source rights. **Proof:** a shared workspace never implies union-of-permissions access.
- [ ] **EPIC-MESH.T02 — Integrate pairing/encrypted sync/recovery.** Connect per-device identity, eligible encrypted records, conflict handling, rotation and recovery keys. **Proof:** clean-device restore works and revoked devices cannot receive newly authorized state.
- [ ] **EPIC-MESH.T03 — Integrate workers/collaboration/clients.** Route scoped tasks to selected devices, enforce commit ownership, filter shared views and negotiate mobile capabilities. **Proof:** handoff preserves task/object identity without transferring unrelated private context.
- [ ] **EPIC-MESH.T04 — Qualify partitions/revocation/client trust.** Test stale workers, simultaneous actions, lost devices, shared summaries and mobile approval replay. **Proof:** no duplicated risky operation or cross-recipient source leak in the declared scenarios.

**Demonstration:** create a workspace on desktop, read it on mobile, delegate bounded work to a home machine, revoke a device, and reconcile a partitioned operation without a duplicate commit.

**Review correction:** fencing only works where dispatch is controlled; arbitrary services cannot reject an old request already sent. Prefer one authoritative commit broker initially. End-to-end encryption does not erase previously copied plaintext or hide all traffic metadata.


---

<a id="issue-94"></a>
## #94 — [P5][MESH-01] Encrypted sync, pairing and key recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/94
**Created:** 2026-09-15T12:20:06Z | **Updated:** 2026-09-15T21:04:58Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #31 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #31

#### Objective
Synchronize user-owned workspaces/preferences/artifacts across explicitly paired devices without giving the sync service plaintext authority or silently merging permissions/approvals.

#### Scope
- Device/user identity keys and explicit pairing flow.
- End-to-end encrypted sync envelopes for eligible metadata/artifacts.
- Per-record sync scope and exclusion of non-portable credentials/session state.
- Conflict handling appropriate to data type; CRDT/merge only where semantics are safe.
- Backup/export/recovery-key flow and key rotation.
- Device inventory, last-seen, revocation and key retirement.
- Incremental sync, resume and bandwidth/storage policy.
- Sync migration/version compatibility.

#### Security/correctness rules
- Permissions/approvals/financial reservations are not last-write-wins data.
- Revoked devices lose access to newly encrypted state.
- Credential/session transfer requires a separate explicit supported mechanism, not generic workspace sync.

#### Acceptance criteria
- [ ] Paired devices exchange eligible state encrypted end-to-end according to design.
- [ ] Revoked device cannot decrypt/fetch newly authorized state after key transition.
- [ ] Layout/note conflicts merge or surface safely without silently overwriting authority records.
- [ ] Credentials and blanket approvals are absent from ordinary sync payloads.
- [ ] Recovery-key exercise restores eligible workspace data to a clean device.
- [ ] Corrupt/old sync records cannot downgrade policy/schema silently.

#### Dependencies
- SEC-03
- UI-04

**First phase:** P5  
**Maturity target:** P6  
**Owner:** platform

### Discussion (2 comments)

#### Comment 5682419641 — Jordan-Hall — 2026-09-15T14:52:20Z

Source: https://github.com/Jordan-Hall/browser/issues/94#issuecomment-5682419641 | Updated: 2026-09-15T14:52:20Z

<!-- intent-implementation-v1:MESH-01 -->
###### Implementation proposal — MESH-01

Implement explicit paired-device encrypted sync over #8/#52. Replicate eligible user-owned content; keep approvals, financial state and commit authority outside generic CRDT/last-write-wins merging.

- [ ] **MESH-01.T01 — Eligibility/crypto threat model.** Inventory syncable views, notes, preferences, artifacts and sensitive metadata; define device/relay/backup threats using reviewed primitives/protocol design. **Verify:** every record type has an explicit sharing/retention rule.
- [ ] **MESH-01.T02 — Pairing/device identity.** Generate per-device keys in supported secure storage and bind pairing to a user-verifiable out-of-band flow, profile and permissions. **Verify:** relay or nearby-device impersonation cannot silently enroll a device.
- [ ] **MESH-01.T03 — Encrypted transport/replay defense.** Authenticate scope/type/version/epoch metadata and sender eligibility, persist cursors transactionally and reject malformed/replayed downgrade state. **Verify:** a malicious relay cannot alter contents or roll back accepted authority metadata.
- [ ] **MESH-01.T04 — Type-specific conflicts.** Merge suitable notes/layout operations causally, retain tombstones and surface irreconcilable edits; delegate authority records to their owning services. **Verify:** synchronization cannot resurrect deleted data or merge two financial approvals.
- [ ] **MESH-01.T05 — Revocation/rotation.** Exclude removed devices from future keys/state and independently disable task/approval authority. **Verify:** new epochs are available only to remaining eligible devices.
- [ ] **MESH-01.T06 — Recovery/transfer controls.** Provide encrypted recovery bundles under deliberate key custody, clean-device restore and resumable chunk transfer with quotas. **Verify:** key loss/error is explicit and support diagnostics minimize metadata.
- [ ] **MESH-01.T07 — Partition/relay qualification.** Test replay, reorder, missing chunks, offline devices, schema upgrades, revocation and recovery errors; obtain independent protocol review. **Verify:** eligible state converges without private-data or authority leakage.

**Boundary:** end-to-end encryption protects content from a relay, not all traffic metadata, and cannot erase copies a revoked device already read. Do not invent custom cryptographic constructions or assume sync automatically grants service access.

#### Comment 5688072854 — Jordan-Hall — 2026-09-15T21:04:58Z

Source: https://github.com/Jordan-Hall/browser/issues/94#issuecomment-5688072854 | Updated: 2026-09-15T21:04:58Z

###### Task issues

- [ ] #715 `MESH-01.T01` — Specify sync eligibility and cryptographic threat model
- [ ] #716 `MESH-01.T02` — Implement explicit device pairing and identity
- [ ] #717 `MESH-01.T03` — Implement encrypted record transport and replay defense
- [ ] #718 `MESH-01.T04` — Implement type-specific conflict resolution
- [ ] #719 `MESH-01.T05` — Implement revocation and key rotation
- [ ] #720 `MESH-01.T06` — Implement recovery, export and bandwidth controls
- [ ] #721 `MESH-01.T07` — Qualify sync under partitions and malicious relay behavior

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-95"></a>
## #95 — [P5][MESH-02] Device authority and remote/home workers

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/95
**Created:** 2026-09-15T12:20:16Z | **Updated:** 2026-09-15T21:05:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #31 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #31

#### Objective
Allow tasks to run on a selected home/remote device while keeping execution location, transferred context and risky-operation authority explicit and partition-safe.

#### Scope
- Worker/device capability advertisement and health.
- Explicit execution placement policy based on device availability, local model/hardware, data locality and grants.
- Scoped encrypted context/artifact transfer to the chosen worker.
- Operation leases/fencing tokens valid across devices.
- Single authority/reservation coordination for risky external commits.
- Partition/offline behavior and stale-worker result rejection.
- Remote worker revocation, key rotation and task migration/recovery.
- User-visible “where this ran / what left this device” trace.

#### Correctness rules
- Two devices must not independently commit the same risky operation.
- Late output from a partitioned/old worker cannot acquire fresh authority automatically.
- Remote execution never implies broader data access than local execution.

#### Acceptance criteria
- [ ] Concurrent-device fixture proves one risky commit authority at a time.
- [ ] Fencing rejects stale worker dispatch/results after lease replacement.
- [ ] Transferred context is limited to the task's grants and visible in audit UI.
- [ ] Device revocation terminates future task placement/authority.
- [ ] Offline/partition recovery does not duplicate transactions.
- [ ] User can identify whether work ran locally, on a home machine or another authorized worker.

#### Dependencies
- MESH-01
- AUTO-01
- TX-02

**First phase:** P5  
**Maturity target:** P6  
**Owner:** platform

### Discussion (2 comments)

#### Comment 5682425589 — Jordan-Hall — 2026-09-15T14:52:40Z

Source: https://github.com/Jordan-Hall/browser/issues/95#issuecomment-5682425589 | Updated: 2026-09-15T14:52:40Z

<!-- intent-implementation-v1:MESH-02 -->
###### Implementation proposal — MESH-02

Implement explicit placement and scoped remote/home execution over #94/#92/#79. Prefer one authoritative commit broker; remote workers propose consequential operations rather than receiving blanket merchant credentials.

- [ ] **MESH-02.T01 — Worker enrollment/capabilities.** Authenticate paired device advertisements for tools/models/OS/software versions/resource limits/trust class. **Verify:** stale last-seen or spoofed capabilities cannot count as current availability.
- [ ] **MESH-02.T02 — Placement preview/policy.** Select local/home/other approved workers from data locality, resources and grants; show required context transfer and reason. **Verify:** placement cannot silently change inference destination or profile scope.
- [ ] **MESH-02.T03 — Minimal encrypted context.** Transfer only eligible evidence/artifacts/constraints by hashed manifest and stage a clean environment. **Verify:** workers cannot request the user's whole graph or mount an unrestricted source directory.
- [ ] **MESH-02.T04 — Single-authority dispatch.** Allocate durable epochs/leases and reserve financial exposure at the authority; revalidate every consequential dispatch. **Verify:** partitions cannot create independent competing commit rights.
- [ ] **MESH-02.T05 — Progress/stop/results.** Bound streams, authenticate result/task versions and revoke leases before cancellation messaging. **Verify:** stale results may remain inspectable artifacts but cannot acquire executable authority.
- [ ] **MESH-02.T06 — Relocation/recovery.** Reassign safe unfinished work from checkpoints only after rechecking destinations/grants and reconciling previous external attempts; clean staging under retention rules. **Verify:** replacing a worker never blindly repeats uncertain writes.
- [ ] **MESH-02.T07 — Mesh qualification.** Test partitions, revocation, duplicate results, false advertisements and resource pressure. **Verify:** transferred-context manifests, budget conservation and actual where-ran reporting agree.

**Critical limitation:** fencing works at controlled dispatch points. An external site cannot retroactively reject a request already sent by an old worker because our local epoch changed. Keep credentials/commits at one controlled authority initially, block unsafe partition failover and reconcile accepted or unknown external state.

#### Comment 5688081391 — Jordan-Hall — 2026-09-15T21:05:43Z

Source: https://github.com/Jordan-Hall/browser/issues/95#issuecomment-5688081391 | Updated: 2026-09-15T21:05:43Z

###### Task issues

- [ ] #722 `MESH-02.T01` — Define worker capabilities and enrollment
- [ ] #723 `MESH-02.T02` — Implement explicit placement policy and preview
- [ ] #724 `MESH-02.T03` — Transfer minimal scoped task context
- [ ] #725 `MESH-02.T04` — Implement single-authority dispatch and fencing
- [ ] #726 `MESH-02.T05` — Implement remote progress, cancellation and results
- [ ] #727 `MESH-02.T06` — Implement relocation and recovery
- [ ] #728 `MESH-02.T07` — Qualify mesh execution and placement truthfulness

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-96"></a>
## #96 — [P5][MESH-03] Shared workspaces and enterprise contexts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/96
**Created:** 2026-09-15T12:20:25Z | **Updated:** 2026-09-15T21:06:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #31 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #31

#### Objective
Support collaborative workspaces without turning collaboration into accidental union-of-permissions access to every participant's connected services and private context.

#### Scope
- Workspace roles and membership lifecycle.
- Recipient-filtered shared state and redacted snapshots.
- Source/account access evaluated per viewer, not inherited from the workspace owner.
- Approval ownership/delegation rules for team operations.
- Team policy overlays and admin constraints while preserving personal/work separation.
- Collaborative annotations/tasks/layouts with conflict-safe sync.
- Shared artifact/evidence export rules and access revocation.
- Audit of who saw/changed/approved shared state.

#### Security rules
- Sharing a view never grants the union of collaborators' external-account rights.
- Personal memory/private-source data remains excluded unless explicitly shared and authorized for each recipient.
- Team policy cannot silently expose personal-profile sources.

#### Acceptance criteria
- [ ] Two collaborators with different source rights see only records/evidence they individually can access.
- [ ] Shared layout/annotations can collaborate without copying inaccessible source bodies.
- [ ] Approval ownership is explicit and cannot be forged by another role.
- [ ] Revoking membership removes future shared access while preserving required audit history.
- [ ] Work/personal profiles remain isolated under shared-workspace navigation.
- [ ] Redacted exports are tested against index/cache/metadata leakage.

#### Dependencies
- MESH-01
- UI-04
- SEC-03

**First phase:** P5  
**Maturity target:** P7  
**Owner:** platform

### Discussion (2 comments)

#### Comment 5682432485 — Jordan-Hall — 2026-09-15T14:53:01Z

Source: https://github.com/Jordan-Hall/browser/issues/96#issuecomment-5682432485 | Updated: 2026-09-15T14:53:01Z

<!-- intent-implementation-v1:MESH-03 -->
###### Implementation proposal — MESH-03

Implement collaboration over #94/#52/#8 with recipient-specific source rights, not the union of collaborators' connected accounts.

- [ ] **MESH-03.T01 — Roles/membership/policy.** Define owner/editor/viewer/approval roles, membership expiry and managed-work overlays separately from personal profiles. **Verify:** effective permissions are constrained by applicable rights and team policy cannot silently expose personal data.
- [ ] **MESH-03.T02 — Recipient projections.** Filter entities, evidence, snippets, summaries, media and indexes before synchronization/rendering for each viewer. **Verify:** hiding an original document cannot leave its private facts visible in a shared generated summary.
- [ ] **MESH-03.T03 — Collaborative overlays.** Merge eligible notes/layouts/tasks with actor/version history while preserving provider truth and private execution context. **Verify:** an editor cannot mutate an authoritative source record by editing a shared view.
- [ ] **MESH-03.T04 — Approval ownership/delegation.** Bind proposals and decisions to the accountable user/account with explicit limits; record proposer/reviewer/approver/dispatcher. **Verify:** another member cannot approve using someone else's source credentials or stale grant.
- [ ] **MESH-03.T05 — Revocation/exports.** Recompute eligibility and rotate keys where needed, deny future retrieval/dispatch and export only reviewed projections. **Verify:** embedded indexes/assets/metadata cannot reveal redacted records; retained audit is minimized.
- [ ] **MESH-03.T06 — Mixed-access qualification.** Test unequal rights, malicious members, work/personal crossings, conflicting edits, key changes and stale approvals. **Verify:** both direct source and generated derivative channels enforce the same constraints.

**Review boundary:** encryption cannot fix an overbroad sharing decision or retract plaintext already received. Sharing a layout is not permission to share the data bound to it. Enterprise policy applies to explicit managed work scope, not undisclosed access to the user's personal profile.

#### Comment 5688089108 — Jordan-Hall — 2026-09-15T21:06:22Z

Source: https://github.com/Jordan-Hall/browser/issues/96#issuecomment-5688089108 | Updated: 2026-09-15T21:06:22Z

###### Task issues

- [ ] #729 `MESH-03.T01` — Define roles, membership and policy boundaries
- [ ] #730 `MESH-03.T02` — Implement recipient-specific source projections
- [ ] #731 `MESH-03.T03` — Implement collaborative notes, layouts and tasks
- [ ] #732 `MESH-03.T04` — Implement approval ownership and delegated team actions
- [ ] #733 `MESH-03.T05` — Implement revocation and safe shared exports
- [ ] #734 `MESH-03.T06` — Qualify mixed-access collaboration

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-97"></a>
## #97 — [P5][MESH-04] Mobile and additional presentation clients

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/97
**Created:** 2026-09-15T12:20:35Z | **Updated:** 2026-09-15T21:07:06Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #31 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #31

#### Objective
Extend the personal runtime to mobile/additional clients for capture, reading, approvals and handoff without assuming desktop-level control capabilities.

#### Scope
- Mobile workspace browsing/reading, search, evidence inspection and notifications.
- Voice capture/dictation using the shared speech/context model.
- Approval/rejection for pending trusted actions with exact proposal context.
- Device-to-device handoff of workspace/object/task selection.
- Explicit client capability negotiation and support matrix.
- Mobile-specific local storage, biometric/device-key integration where appropriate and revocation.
- Later experimental clients (spatial/embedded) behind the same presentation contracts.

#### Security/product rules
- Mobile pairing does not inherit desktop filesystem or PC-control grants.
- Approval UI remains trusted native/runtime chrome, not generated content.
- Sensitive data sync follows MESH-01 scopes and device security state.

#### Acceptance criteria
- [ ] Mobile client can inspect synced workspaces/evidence and hand off context to a desktop/runtime worker.
- [ ] Voice capture is clearly indicated and follows configured audio retention.
- [ ] Approval action shows exact account/target/amount/audience and rejects stale proposals.
- [ ] Client capability matrix prevents desktop-only actions from appearing executable on unsupported devices.
- [ ] Revoked/lost device loses future sync/approval authority.
- [ ] Handoff preserves object/task identity without transferring unrelated private context.

#### Dependencies
- MESH-01
- VOICE-01

**First phase:** P5  
**Maturity target:** P7  
**Owner:** platform

### Discussion (2 comments)

#### Comment 5682439807 — Jordan-Hall — 2026-09-15T14:53:24Z

Source: https://github.com/Jordan-Hall/browser/issues/97#issuecomment-5682439807 | Updated: 2026-09-15T14:53:24Z

<!-- intent-implementation-v1:MESH-04 -->
###### Implementation proposal — MESH-04

Build mobile/additional clients over #94/#65 with explicit capability negotiation; reuse portable Rust domain logic, not assumptions that the desktop GUI or control permissions transfer unchanged.

- [ ] **MESH-04.T01 — Client/platform contracts.** Separate reading/capture/voice/sync/notifications/approvals from local inference and desktop execution; spike storage, accessibility, microphone and lifecycle behavior. **Verify:** unsupported desktop capabilities cannot appear locally executable.
- [ ] **MESH-04.T02 — Pairing/local storage.** Bind device keys to supported secure storage and selected eligible caches; implement lock/logout/lost-device behavior. **Verify:** revoked devices cannot receive new sync or approval authority.
- [ ] **MESH-04.T03 — Workspace/reader.** Render stable saved views, evidence/source navigation, local search and object deep links within client limits. **Verify:** offline cache and stale states are explicit and accessible.
- [ ] **MESH-04.T04 — Voice/handoff.** Show capture state, transcribe locally where supported or through an explicitly approved worker; transmit only scoped object/task identity to a chosen destination. **Verify:** handoff never silently uploads the full workspace or changes privacy mode.
- [ ] **MESH-04.T05 — Trusted remote approvals.** Fetch the current proposal with authenticated challenge, show exact material fields/expiry and bind the decision to digest/device/user identity. **Verify:** the authoritative broker revalidates before commit and rejects stale/replayed approvals.
- [ ] **MESH-04.T06 — Notifications/lifecycle.** Redact notifications, persist pending handoffs and recover sync/background suspension idempotently. **Verify:** a resumed mobile client cannot duplicate a task or falsely claim local execution.
- [ ] **MESH-04.T07 — Client qualification.** Test lost devices, offline reading, speech privacy, accessible approvals and different-capability handoffs. **Verify:** spatial/experimental clients stay explicitly gated until their own trust/interaction suites pass.

**Review decision:** choose native mobile shells only after platform spikes. Device pairing is not a desktop filesystem/input grant. Mobile background restrictions and available inference hardware must be reflected in actual support and execution-location reporting.

#### Comment 5688097267 — Jordan-Hall — 2026-09-15T21:07:06Z

Source: https://github.com/Jordan-Hall/browser/issues/97#issuecomment-5688097267 | Updated: 2026-09-15T21:07:06Z

###### Task issues

- [ ] #735 `MESH-04.T01` — Define mobile capability and platform support contracts
- [ ] #736 `MESH-04.T02` — Implement secure pairing and local storage
- [ ] #737 `MESH-04.T03` — Build reading, evidence and workspace navigation
- [ ] #738 `MESH-04.T04` — Implement voice capture and explicit handoff
- [ ] #739 `MESH-04.T05` — Implement trusted remote approval surfaces
- [ ] #740 `MESH-04.T06` — Implement notifications and client lifecycle recovery
- [ ] #741 `MESH-04.T07` — Qualify mobile and experimental clients

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-283"></a>
## #283 — [TASK][EPIC-MESH.T01] Ratify device, sharing and authority boundaries

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/283
**Created:** 2026-09-15T18:08:54Z | **Updated:** 2026-09-15T18:08:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #31

### Original description

Parent: #31

Task ID: `EPIC-MESH.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-284"></a>
## #284 — [TASK][EPIC-MESH.T02] Integrate pairing, encrypted sync and recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/284
**Created:** 2026-09-15T18:09:00Z | **Updated:** 2026-09-15T18:09:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #31

### Original description

Parent: #31

Task ID: `EPIC-MESH.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-285"></a>
## #285 — [TASK][EPIC-MESH.T03] Integrate remote workers, collaboration and clients

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/285
**Created:** 2026-09-15T18:09:04Z | **Updated:** 2026-09-15T18:09:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #31

### Original description

Parent: #31

Task ID: `EPIC-MESH.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-286"></a>
## #286 — [TASK][EPIC-MESH.T04] Qualify partitions, revocation and cross-client trust

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/286
**Created:** 2026-09-15T18:09:08Z | **Updated:** 2026-09-15T18:09:08Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #31

### Original description

Parent: #31

Task ID: `EPIC-MESH.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-715"></a>
## #715 — [TASK][MESH-01.T01] Specify sync eligibility and cryptographic threat model

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/715
**Created:** 2026-09-15T21:04:22Z | **Updated:** 2026-09-15T21:04:22Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #94

### Original description

Parent: #94

Task ID: `MESH-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-716"></a>
## #716 — [TASK][MESH-01.T02] Implement explicit device pairing and identity

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/716
**Created:** 2026-09-15T21:04:27Z | **Updated:** 2026-09-15T21:04:27Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #94

### Original description

Parent: #94

Task ID: `MESH-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-717"></a>
## #717 — [TASK][MESH-01.T03] Implement encrypted record transport and replay defense

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/717
**Created:** 2026-09-15T21:04:32Z | **Updated:** 2026-09-15T21:04:32Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #94

### Original description

Parent: #94

Task ID: `MESH-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-718"></a>
## #718 — [TASK][MESH-01.T04] Implement type-specific conflict resolution

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/718
**Created:** 2026-09-15T21:04:37Z | **Updated:** 2026-09-15T21:04:37Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #94

### Original description

Parent: #94

Task ID: `MESH-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-719"></a>
## #719 — [TASK][MESH-01.T05] Implement revocation and key rotation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/719
**Created:** 2026-09-15T21:04:42Z | **Updated:** 2026-09-15T21:04:42Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #94

### Original description

Parent: #94

Task ID: `MESH-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-720"></a>
## #720 — [TASK][MESH-01.T06] Implement recovery, export and bandwidth controls

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/720
**Created:** 2026-09-15T21:04:47Z | **Updated:** 2026-09-15T21:04:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #94

### Original description

Parent: #94

Task ID: `MESH-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-721"></a>
## #721 — [TASK][MESH-01.T07] Qualify sync under partitions and malicious relay behavior

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/721
**Created:** 2026-09-15T21:04:51Z | **Updated:** 2026-09-15T21:04:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #94

### Original description

Parent: #94

Task ID: `MESH-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-722"></a>
## #722 — [TASK][MESH-02.T01] Define worker capabilities and enrollment

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/722
**Created:** 2026-09-15T21:05:05Z | **Updated:** 2026-09-15T21:05:05Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #95

### Original description

Parent: #95

Task ID: `MESH-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-723"></a>
## #723 — [TASK][MESH-02.T02] Implement explicit placement policy and preview

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/723
**Created:** 2026-09-15T21:05:11Z | **Updated:** 2026-09-15T21:05:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #95

### Original description

Parent: #95

Task ID: `MESH-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-724"></a>
## #724 — [TASK][MESH-02.T03] Transfer minimal scoped task context

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/724
**Created:** 2026-09-15T21:05:15Z | **Updated:** 2026-09-15T21:05:15Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #95

### Original description

Parent: #95

Task ID: `MESH-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-725"></a>
## #725 — [TASK][MESH-02.T04] Implement single-authority dispatch and fencing

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/725
**Created:** 2026-09-15T21:05:20Z | **Updated:** 2026-09-15T21:05:20Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #95

### Original description

Parent: #95

Task ID: `MESH-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-726"></a>
## #726 — [TASK][MESH-02.T05] Implement remote progress, cancellation and results

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/726
**Created:** 2026-09-15T21:05:25Z | **Updated:** 2026-09-15T21:05:25Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #95

### Original description

Parent: #95

Task ID: `MESH-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-727"></a>
## #727 — [TASK][MESH-02.T06] Implement relocation and recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/727
**Created:** 2026-09-15T21:05:31Z | **Updated:** 2026-09-15T21:05:31Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #95

### Original description

Parent: #95

Task ID: `MESH-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-728"></a>
## #728 — [TASK][MESH-02.T07] Qualify mesh execution and placement truthfulness

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/728
**Created:** 2026-09-15T21:05:36Z | **Updated:** 2026-09-15T21:05:36Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #95

### Original description

Parent: #95

Task ID: `MESH-02.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-729"></a>
## #729 — [TASK][MESH-03.T01] Define roles, membership and policy boundaries

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/729
**Created:** 2026-09-15T21:05:49Z | **Updated:** 2026-09-15T21:05:49Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #96

### Original description

Parent: #96

Task ID: `MESH-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-730"></a>
## #730 — [TASK][MESH-03.T02] Implement recipient-specific source projections

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/730
**Created:** 2026-09-15T21:05:54Z | **Updated:** 2026-09-15T21:05:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #96

### Original description

Parent: #96

Task ID: `MESH-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-731"></a>
## #731 — [TASK][MESH-03.T03] Implement collaborative notes, layouts and tasks

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/731
**Created:** 2026-09-15T21:05:59Z | **Updated:** 2026-09-15T21:05:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #96

### Original description

Parent: #96

Task ID: `MESH-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-732"></a>
## #732 — [TASK][MESH-03.T04] Implement approval ownership and delegated team actions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/732
**Created:** 2026-09-15T21:06:06Z | **Updated:** 2026-09-15T21:06:06Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #96

### Original description

Parent: #96

Task ID: `MESH-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-733"></a>
## #733 — [TASK][MESH-03.T05] Implement revocation and safe shared exports

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/733
**Created:** 2026-09-15T21:06:11Z | **Updated:** 2026-09-15T21:06:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #96

### Original description

Parent: #96

Task ID: `MESH-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-734"></a>
## #734 — [TASK][MESH-03.T06] Qualify mixed-access collaboration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/734
**Created:** 2026-09-15T21:06:17Z | **Updated:** 2026-09-15T21:06:17Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #96

### Original description

Parent: #96

Task ID: `MESH-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-735"></a>
## #735 — [TASK][MESH-04.T01] Define mobile capability and platform support contracts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/735
**Created:** 2026-09-15T21:06:28Z | **Updated:** 2026-09-15T21:06:28Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #97

### Original description

Parent: #97

Task ID: `MESH-04.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-736"></a>
## #736 — [TASK][MESH-04.T02] Implement secure pairing and local storage

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/736
**Created:** 2026-09-15T21:06:33Z | **Updated:** 2026-09-15T21:06:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #97

### Original description

Parent: #97

Task ID: `MESH-04.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-737"></a>
## #737 — [TASK][MESH-04.T03] Build reading, evidence and workspace navigation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/737
**Created:** 2026-09-15T21:06:39Z | **Updated:** 2026-09-15T21:06:39Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #97

### Original description

Parent: #97

Task ID: `MESH-04.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-738"></a>
## #738 — [TASK][MESH-04.T04] Implement voice capture and explicit handoff

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/738
**Created:** 2026-09-15T21:06:44Z | **Updated:** 2026-09-15T21:06:44Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #97

### Original description

Parent: #97

Task ID: `MESH-04.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-739"></a>
## #739 — [TASK][MESH-04.T05] Implement trusted remote approval surfaces

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/739
**Created:** 2026-09-15T21:06:49Z | **Updated:** 2026-09-15T21:06:49Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #97

### Original description

Parent: #97

Task ID: `MESH-04.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-740"></a>
## #740 — [TASK][MESH-04.T06] Implement notifications and client lifecycle recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/740
**Created:** 2026-09-15T21:06:54Z | **Updated:** 2026-09-15T21:06:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #97

### Original description

Parent: #97

Task ID: `MESH-04.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-741"></a>
## #741 — [TASK][MESH-04.T07] Qualify mobile and experimental clients

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/741
**Created:** 2026-09-15T21:07:00Z | **Updated:** 2026-09-15T21:07:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #97

### Original description

Parent: #97

Task ID: `MESH-04.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

