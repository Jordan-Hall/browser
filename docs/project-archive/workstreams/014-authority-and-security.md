# Authority and security

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 50

## Contents

- [#14 — EPIC: Authority and security](#issue-14)
- [#6 — [P0][SEC-01] Threat model and conformance policy](#issue-6)
- [#7 — [P0][SEC-02] Capability broker and scoped grants](#issue-7)
- [#8 — [P1][SEC-03] Vault, encryption and information-flow controls](#issue-8)
- [#9 — [P0][SEC-04] Sandbox and egress broker](#issue-9)
- [#10 — [P0][SEC-05] Privacy governance and incident process](#issue-10)
- [#153 — [TASK][SEC-01.T01] Inventory assets and execution principals](#issue-153)
- [#154 — [TASK][SEC-01.T02] Draw trust boundaries and dataflow diagrams](#issue-154)
- [#155 — [TASK][SEC-01.T03] Specify effect classes and required controls](#issue-155)
- [#156 — [TASK][SEC-01.T04] Construct adversarial scenarios](#issue-156)
- [#157 — [TASK][SEC-01.T05] Define agent and extension trust classes](#issue-157)
- [#158 — [TASK][SEC-01.T06] Translate policy into release conformance](#issue-158)
- [#159 — [TASK][SEC-01.T07] Publish incident assumptions and residual risks](#issue-159)
- [#160 — [TASK][SEC-01.T08] Keep the model current through change review](#issue-160)
- [#161 — [TASK][SEC-02.T01] Implement the capability registry](#issue-161)
- [#162 — [TASK][SEC-02.T02] Build scoped grant value types](#issue-162)
- [#163 — [TASK][SEC-02.T03] Implement deterministic policy decisions](#issue-163)
- [#164 — [TASK][SEC-02.T04] Bind approvals to exact proposals](#issue-164)
- [#165 — [TASK][SEC-02.T05] Implement dispatch-time revalidation](#issue-165)
- [#166 — [TASK][SEC-02.T06] Add revocation and cancellation fencing](#issue-166)
- [#167 — [TASK][SEC-02.T07] Implement audit records without secret leakage](#issue-167)
- [#168 — [TASK][SEC-02.T08] Prove the authority lattice and attack resistance](#issue-168)
- [#169 — [TASK][SEC-03.T01] Design key hierarchy and unlock lifecycle](#issue-169)
- [#170 — [TASK][SEC-03.T02] Implement OS secret-store adapters](#issue-170)
- [#171 — [TASK][SEC-03.T03] Encrypt private stores and artifacts](#issue-171)
- [#172 — [TASK][SEC-03.T04] Implement access and destination labels](#issue-172)
- [#173 — [TASK][SEC-03.T05] Secure credential release and diagnostic paths](#issue-173)
- [#174 — [TASK][SEC-03.T06] Implement rotation and recovery](#issue-174)
- [#175 — [TASK][SEC-03.T07] Implement forget and retention jobs](#issue-175)
- [#176 — [TASK][SEC-03.T08] Test privacy modes and cross-scope leakage](#issue-176)
- [#177 — [TASK][SEC-04.T01] Define the isolation contract and adversarial probe](#issue-177)
- [#178 — [TASK][SEC-04.T02] Build staged filesystem and environment preparation](#issue-178)
- [#179 — [TASK][SEC-04.T03] Implement Linux confinement backend](#issue-179)
- [#180 — [TASK][SEC-04.T04] Implement Windows containment backend](#issue-180)
- [#181 — [TASK][SEC-04.T05] Implement macOS containment backend](#issue-181)
- [#182 — [TASK][SEC-04.T06] Implement egress and redirect enforcement](#issue-182)
- [#183 — [TASK][SEC-04.T07] Integrate WASM limits and capability handles](#issue-183)
- [#184 — [TASK][SEC-04.T08] Add launch-before-config and escape regressions](#issue-184)
- [#185 — [TASK][SEC-05.T01] Create the processing and rights inventory](#issue-185)
- [#186 — [TASK][SEC-05.T02] Implement consent and privacy-mode receipts](#issue-186)
- [#187 — [TASK][SEC-05.T03] Define enforceable retention and export policies](#issue-187)
- [#188 — [TASK][SEC-05.T04] Review distribution, model and provider terms](#issue-188)
- [#189 — [TASK][SEC-05.T05] Constrain payment and sensitive-domain handling](#issue-189)
- [#190 — [TASK][SEC-05.T06] Implement telemetry and support-bundle minimization](#issue-190)
- [#191 — [TASK][SEC-05.T07] Build incident response and emergency revocation](#issue-191)
- [#192 — [TASK][SEC-05.T08] Run launch and lifecycle governance exercises](#issue-192)
- [#215 — [TASK][EPIC-SEC.T01] Unify authority and data-flow inventories](#issue-215)
- [#216 — [TASK][EPIC-SEC.T02] Integrate policy with all brokers and sandboxes](#issue-216)
- [#217 — [TASK][EPIC-SEC.T03] Validate hostile-content and privacy scenarios](#issue-217)
- [#218 — [TASK][EPIC-SEC.T04] Exercise revocation and incident response](#issue-218)

---

<a id="issue-14"></a>
## #14 — EPIC: Authority and security

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/14
**Created:** 2026-09-15T12:06:43Z | **Updated:** 2026-09-15T14:20:56Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #1 | **Body-declared parent:** #1
**Native child issues:** #6, #7, #8, #9, #10

### Original description

Programme: #1

Own the threat model, capability authorization, credential/data protection, process/egress confinement, privacy governance and incident response. Security rules are product semantics: model/provider output can propose work but cannot authorize itself.

#### Child issues
- [ ] #6 SEC-01 — Threat model and conformance policy
- [ ] #7 SEC-02 — Capability broker and scoped grants
- [ ] #8 SEC-03 — Vault, encryption and information-flow controls
- [ ] #9 SEC-04 — Sandbox and egress broker
- [ ] #10 SEC-05 — Privacy governance and incident process

#### Cross-cutting gates
No ambient authority, scoped credentials, trusted approval surfaces, hostile content never grants privilege, explicit egress, revocation, reconciliation and auditable decisions.

### Discussion (1 comments)

#### Comment 5681826893 — Jordan-Hall — 2026-09-15T14:20:56Z

Source: https://github.com/Jordan-Hall/browser/issues/14#issuecomment-5681826893 | Updated: 2026-09-15T14:20:56Z

<!-- intent-implementation-v1:EPIC-SEC -->
###### Workstream implementation and integration tasks

Feature implementation lives in #6–#10. This epic proves the complete authorization/dataflow boundary across actual workers.

- [ ] **EPIC-SEC.T01 — Unify authority and data-flow inventories.** Agree principal, asset, effect, destination, retention and secret ownership with every workstream. **Proof:** every privileged dispatch and sensitive transformation maps to one reviewed owner/policy.
- [ ] **EPIC-SEC.T02 — Integrate policy with all brokers and sandboxes.** Connect grants, scoped credentials, launch-before-config confinement, egress and trusted approvals to browser/connector/agent/file/desktop execution. **Proof:** no worker can invoke an alternate unmediated privileged path in its advertised trust class.
- [ ] **EPIC-SEC.T03 — Validate hostile-content and privacy scenarios.** Run injection, spoofing, cross-account, export, retrieval and credential-leak tests with legitimate controls. **Proof:** forbidden effects fail; false positives and residual risks are reported separately.
- [ ] **EPIC-SEC.T04 — Exercise revocation and incident response.** Revoke a connector/package/device while tasks run, patch safely and recover with models disabled. **Proof:** future authority is stopped while accepted/unknown external operations remain reconcilable.

**Important review corrections:** matching a host user is not strong isolation; some confined provider CLIs require scoped credential release; signatures do not prove package safety; encryption cannot erase already copied data. Publish these boundaries honestly rather than asserting absolute security.

**Closure evidence:** independent security review, adversarial fixture results, privacy/export exercises and tested incident runbooks. Every task remains proposed until implementation evidence exists.


---

<a id="issue-6"></a>
## #6 — [P0][SEC-01] Threat model and conformance policy

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/6
**Created:** 2026-09-15T12:04:26Z | **Updated:** 2026-09-15T14:17:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #14 | **Body-declared parent:** #1

### Original description

Parent: #1

#### Objective
Define the security model before connectors, agents and generated UI acquire capabilities. Every process, data class and effect path must have a named trust boundary and release policy.

#### Scope
- Trust zones for shell, supervisor, browser renderer, connectors, external agents, local models, extensions, desktop broker and OS services.
- Threat inventory: prompt injection, confused deputy, credential theft, malicious MCP/tool metadata, hostile pages/files/repos, generated-UI spoofing, supply chain, egress, data poisoning, stale observations and local IPC abuse.
- Effect taxonomy: read, local reversible write, external compensatable write, irreversible/uncertain write.
- Required controls by trust/effect class: sandbox, grants, approvals, verification, audit, reconciliation.
- Supported execution modes and explicitly excluded adversaries (for example fully compromised host kernel).
- Security-conformance requirements for new connectors/providers/components.

#### Deliverables
- Versioned threat-model document/dataflow diagrams.
- Trust-boundary inventory with owning crate/process/team.
- Conformance checklist consumed by code review and evaluation suites.
- Security assumptions and non-claims suitable for product documentation.

#### Acceptance criteria
- [ ] Every runtime process has declared authority, readable resources and allowed egress.
- [ ] Every connector operation maps to an effect class and required authorization path.
- [ ] Prompt/content cannot be treated as authority by policy definition.
- [ ] Trusted confirmation surfaces and generated/provider content have explicit isolation rules.
- [ ] External coding agents have separate mediated/confined/unconfined trust classes.
- [ ] Threat-model fixtures are represented in EVAL-01/EVAL-02.

#### Dependencies
- CORE-01

**First phase:** P0  
**Maturity target:** P7 (continuous)  
**Workstream:** Authority and security

### Discussion (1 comments)

#### Comment 5681764405 — Jordan-Hall — 2026-09-15T14:17:47Z

Source: https://github.com/Jordan-Hall/browser/issues/6#issuecomment-5681764405 | Updated: 2026-09-15T14:17:47Z

<!-- intent-implementation-v1:SEC-01 -->
###### Implementation proposal — SEC-01

Make the threat model executable review policy. Proposed locations: `docs/security`, `policy/conformance`, `fixtures/adversarial`, `crates/security-model`. Coordinate contracts with #2 and fixtures with #101/#102.

- [ ] **SEC-01.T01 — Asset/principal inventory.** Enumerate credentials, source records, browser profiles, files/code, money/messages, microphone data and approvals. Assign one owner and enumerate permitted readers/transformers/exporters. **Verify:** every capability and data path has a named principal and asset.
- [ ] **SEC-01.T02 — Trust/dataflow diagrams.** Model shell, supervisor, brokers, renderers, connectors, agents, models and extensions, including clipboard, attachments, drag/drop, logs and crash reports. **Verify:** transport authentication is never treated as proof that content is trustworthy.
- [ ] **SEC-01.T03 — Effect/control taxonomy.** Map read/local reversible/external compensatable/irreversible-or-uncertain effects to grants, freshness, approval, idempotency, verification and audit. **Verify:** each installed operation satisfies its required controls.
- [ ] **SEC-01.T04 — Adversarial scenarios.** Pair injection, account-confusion, approval-replay, exfiltration, spoofing, traversal and stale-target attacks with legitimate counterparts. **Verify:** unauthorized effects are denied without concealing false-positive rates.
- [ ] **SEC-01.T05 — Agent/extension trust classes.** Define mediated, confined and manual/unconfined modes using measurable file/network/desktop limits. **Verify:** native agents cannot receive autonomous sensitive authority without confinement evidence.
- [ ] **SEC-01.T06 — Release conformance.** Turn controls into manifest validation, tests and release artifacts; make waiver ownership/expiry explicit. **Verify:** model-generated plans cannot waive security invariants.
- [ ] **SEC-01.T07 — Residual risks/incident assumptions.** Document host compromise, same-user access limits, physical threats, key loss and uncertain provider state, with detection/recovery owners. **Verify:** product claims match enforceable boundaries.
- [ ] **SEC-01.T08 — Change-triggered review.** Require new connectors, native APIs, destinations and background behaviors to update threat scenarios and release associations. **Verify:** CI/review rejects undocumented authority expansion.

**Implementation principle:** denying an unauthorized effect deterministically is the security boundary; an LLM promising to ignore injected instructions is not. Source data and tool metadata never confer authority. This is proposed work, not a completed security assessment.


---

<a id="issue-7"></a>
## #7 — [P0][SEC-02] Capability broker and scoped grants

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/7
**Created:** 2026-09-15T12:04:38Z | **Updated:** 2026-09-15T14:18:12Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #14 | **Body-declared parent:** #1

### Original description

Parent: #1

#### Objective
Make authorization deterministic and independent of model/provider output. Every tool/action dispatch must resolve through a Rust capability broker with explicit scope.

#### Scope
- Capability registry keyed by provider, account, operation and version.
- Grants scoped by resource/object, account, recipient/audience, data destination, time, task, financial ceiling and effect class.
- Approval binding to proposal digest, exact arguments, source/resource version, expiry and user identity.
- Revocation and lease invalidation.
- Policy evaluation API for browser, connector, agent, desktop, file and transaction workers.
- Decision/audit records with stable reason codes.

#### Architecture requirements
- Models may propose but never mint/expand grants.
- A generic tool name is insufficient authority; provider/account/target semantics are part of the decision.
- Changed material arguments require a fresh authorization decision.

#### Acceptance criteria
- [ ] Revoked, expired or stale grants cannot dispatch new work.
- [ ] Forged proposals and materially changed arguments fail authorization.
- [ ] Cross-account/cross-recipient confusion tests fail closed.
- [ ] Every privileged dispatch has a traceable policy decision and capability ID.
- [ ] Worker compromise does not reveal unrelated grants.
- [ ] Stop/cancel can invalidate execution leases immediately.

#### Tests
Property tests for scope intersection; approval replay attacks; account/recipient swaps; expiry races; stale resource versions; malicious model/tool descriptions.

#### Dependencies
- CORE-01
- SEC-01

**First phase:** P0  
**Maturity target:** P4  
**Workstream:** Authority and security

### Discussion (1 comments)

#### Comment 5681772125 — Jordan-Hall — 2026-09-15T14:18:12Z

Source: https://github.com/Jordan-Hall/browser/issues/7#issuecomment-5681772125 | Updated: 2026-09-15T14:18:12Z

<!-- intent-implementation-v1:SEC-02 -->
###### Implementation proposal — SEC-02

Put deterministic authorization in `crates/policy`, `capabilities`, and `approvals`, with one authority owner. Prerequisites: #2/#6. Installation/discovery, grant eligibility, approval and dispatch permission are separate states.

- [ ] **SEC-02.T01 — Capability registry.** Persist provider/account-qualified operations, schemas, effect class, version and verifier. **Verify:** conflicting descriptors fail; installation creates no grant.
- [ ] **SEC-02.T02 — Scope types.** Model resources, file roots, recipients/audiences, destinations, task, expiry and spending limits with explicit intersections. **Verify:** delegation cannot increase authority; missing scope denies by default.
- [ ] **SEC-02.T03 — Policy evaluator.** Evaluate caller, active grants, descriptor version, inference mode and exact arguments against a trusted snapshot. Return stable reason codes and approval requirements. **Verify:** model assurances cannot influence allow/deny.
- [ ] **SEC-02.T04 — Bound approvals.** Canonicalize material fields; bind digest to user/profile/task/account/target/resource version/limits/expiry and one operation intent. Only trusted confirmation can mint approval. **Verify:** argument, account, amount and recipient swaps reject reused approval.
- [ ] **SEC-02.T05 — Dispatch revalidation.** Resolve current handles, epochs, expiry, source preconditions and budget reservations at the controlled dispatch boundary. Issue a single-operation permit. **Verify:** a stale displayed button or earlier policy decision cannot authorize changed state.
- [ ] **SEC-02.T06 — Revocation fencing.** Atomically advance epochs, invalidate decisions and deny queued permits before signalling workers. **Verify:** delayed worker output cannot regain authority; already-sent attempts remain for reconciliation.
- [ ] **SEC-02.T07 — Redacted decision audit.** Store reason/policy version/principal/operation/resource references without raw secrets. **Verify:** diagnostics retain useful explanations but omit seeded private markers.
- [ ] **SEC-02.T08 — Property/adversarial tests.** Test scope monotonicity, canonicalization, expiry races, account confusion and forged approvals through the real broker. **Verify:** all denied effects remain denied regardless of injected tool descriptions.

**Race boundary:** define the exact linearization point for dispatch versus revocation. Broker fencing controls future local dispatch, not requests already accepted by an external service. Keep those outcomes in #79 reconciliation. Each task needs its own code and negative-test evidence before closure.


---

<a id="issue-8"></a>
## #8 — [P1][SEC-03] Vault, encryption and information-flow controls

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/8
**Created:** 2026-09-15T12:04:47Z | **Updated:** 2026-09-15T14:18:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #14 | **Body-declared parent:** #1

### Original description

Parent: #1

#### Objective
Protect credentials and private context across profiles, workspaces, connectors, agents, sync and derived data. Local-first must be an enforceable data-flow property, not a UI badge.

#### Scope
- OS-backed secret vault abstraction and scoped credential handles.
- Profile/workspace encryption keys, key rotation and recovery hooks.
- Access labels on observations, evidence, artifacts, embeddings, summaries and memory.
- Destination restrictions for local inference, named cloud providers, exports and shared workspaces.
- Conservative propagation of restrictions through deterministic/model-derived outputs.
- Deletion/forget workflows covering indexes, caches and future context eligibility.

#### Design requirements
- Secrets never appear in prompts, ordinary worker environments or diagnostic traces.
- Cloud-provider permission does not imply all readable local context can be uploaded.
- Work/personal/private-session scopes remain isolated.

#### Acceptance criteria
- [ ] Private inputs cannot reach an unauthorized provider, connector, export or shared workspace.
- [ ] Credentials are brokered by handle and remain absent from model context/logs.
- [ ] Derived summaries/embeddings inherit conservative restrictions from sensitive inputs.
- [ ] Deletion removes searchable derivatives and future retrieval eligibility.
- [ ] Profile/workspace key rotation preserves authorized data and invalidates retired keys.
- [ ] Privacy-mode transitions are explicit and auditable; no hidden cloud fallback.

#### Dependencies
- SEC-02
- CORE-02

**First phase:** P1  
**Maturity target:** P6  
**Workstream:** Authority and security

### Discussion (1 comments)

#### Comment 5681778751 — Jordan-Hall — 2026-09-15T14:18:33Z

Source: https://github.com/Jordan-Hall/browser/issues/8#issuecomment-5681778751 | Updated: 2026-09-15T14:18:33Z

<!-- intent-implementation-v1:SEC-03 -->
###### Implementation proposal — SEC-03

Implement privacy as dataflow enforcement, not a local-mode badge. Use a vault/key service plus labelled storage/retrieval boundaries; dependencies #7/#3. Proposed areas: `vault`, encrypted state/artifacts, destination policy and deletion jobs.

- [ ] **SEC-03.T01 — Key/unlock lifecycle.** Separate device/profile wrapping keys from data keys; version algorithms and keys; bind authenticated ciphertext to object/profile/version. **Verify:** wrong-profile or modified ciphertext fails and locked profiles expose no private plaintext.
- [ ] **SEC-03.T02 — OS secret stores.** Implement Windows, Keychain and supported Linux keyring adapters with explicit unavailable/locked/error states and opaque handles. **Verify:** failures never fall back silently to plaintext files.
- [ ] **SEC-03.T03 — Encrypt stores/artifacts.** Choose and test field/database encryption plus encrypted blobs; minimize plaintext metadata and scope deduplication. **Verify:** offline inspection cannot recover protected bodies or infer cross-profile object existence through public hashes.
- [ ] **SEC-03.T04 — Access/destination labels.** Carry readers, sensitivity, permitted destinations and retention into summaries, embeddings and memory. Derivations conservatively inherit all contributing restrictions. **Verify:** denied inputs cannot leak through snippets, rerankers, exports or cloud context.
- [ ] **SEC-03.T05 — Credential/diagnostic release.** Resolve exact provider/account/audience in the broker; redact errors, prompts, artifacts and normal worker environments. **Verify:** seeded secrets do not appear in logs or model context.
- [ ] **SEC-03.T06 — Rotation/recovery.** Implement resumable re-encryption and atomic key-version changes; test model-independent recovery and backup compatibility. **Verify:** interruption cannot strand data or reactivate retired keys.
- [ ] **SEC-03.T07 — Forget/retention.** Revoke retrieval eligibility first, then remove derivatives/eligible blobs and propagate tombstones; minimize legally retained receipts. **Verify:** forgotten records cannot return through indexes, caches or new context.
- [ ] **SEC-03.T08 — Cross-scope qualification.** Test identical content across work/personal/private sessions, different accounts and named remote destinations. **Verify:** compaction, export, diagnostics and sync preserve current restrictions.

**Review correction to the original absolute wording:** some provider CLIs require a scoped credential inside their confined process. Document that narrow exception and its lifetime; never expose the full vault or unrelated credentials. Also, deletion cannot retract plaintext already exported to another party; state the actual retention/revocation boundary.


---

<a id="issue-9"></a>
## #9 — [P0][SEC-04] Sandbox and egress broker

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/9
**Created:** 2026-09-15T12:04:55Z | **Updated:** 2026-09-15T14:18:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #14 | **Body-declared parent:** #1

### Original description

Parent: #1

#### Objective
Confine native agents, generated extensions, connectors and model tooling so a compromised worker cannot turn task context into host-wide filesystem, credential or network access.

#### Scope
- Cross-platform sandbox abstraction with staged workspaces and clean environments.
- Native CLI confinement via OS sandbox/container/VM strategy per platform.
- WASM extension host with narrow imports, CPU/memory/time budgets and capability handles.
- Network egress broker with allowlists, redirects, DNS/IP revalidation and private-address controls.
- File mounts/read-write scopes and disposable build/test environments.
- Explicit mediated/confined/unconfined adapter classification.

#### Design requirements
- External coding agents are not assumed to route their internal tools through our broker.
- Localhost/private-network access is a separate grant, not implicit network permission.
- Sandboxes must start before agent hooks/project config/MCP configuration can execute.

#### Acceptance criteria
- [ ] Hostile repo/model/connector fixtures cannot read disallowed home files or credentials.
- [ ] Unapproved outbound destinations and redirect pivots are blocked and recorded.
- [ ] Generated WASM/extensions cannot call host APIs outside declared imports.
- [ ] Confined agents see only staged files and approved environment variables.
- [ ] Sandbox escape/egress regression fixtures run in CI on supported platforms.
- [ ] Unconfined integrations are visibly manual-only and cannot receive autonomous personal-account/desktop authority.

#### Dependencies
- SEC-01
- SEC-02

**First phase:** P0  
**Maturity target:** P3  
**Workstream:** Authority and security

### Discussion (1 comments)

#### Comment 5681785176 — Jordan-Hall — 2026-09-15T14:18:54Z

Source: https://github.com/Jordan-Hall/browser/issues/9#issuecomment-5681785176 | Updated: 2026-09-15T14:18:54Z

<!-- intent-implementation-v1:SEC-04 -->
###### Implementation proposal — SEC-04

Define isolation as a tested execution profile before starting any repository-controlled configuration. Prerequisites #6/#7; use separate native-worker and WASM paths, not one misleading sandbox flag.

- [ ] **SEC-04.T01 — Isolation contract/probe.** Declare allowed files, roots, executables, subprocesses, environment, network and IPC. Run a harmless forbidden-marker/destination probe before real tasks. **Verify:** unavailable controls reject sensitive unattended execution.
- [ ] **SEC-04.T02 — Staging.** Copy only approved inputs into disposable roots, sanitize config/environment search paths and expose bounded outputs. Handle symlinks, hardlinks, reparse points and archive traversal. **Verify:** staged paths cannot resolve back into the host home/vault.
- [ ] **SEC-04.T03 — Linux backend.** Combine supported namespaces, syscall/filesystem restrictions, descriptor closure and dropped capabilities; establish brokered or denied networking. **Verify:** concrete kernel/profile combinations pass escape and egress probes.
- [ ] **SEC-04.T04 — Windows backend.** Evaluate restricted tokens/AppContainer, ACLs and Job Objects; stage with matching ACLs. **Verify:** process-tree/resource limits work; incompatible native agents use a verified VM rather than losing isolation.
- [ ] **SEC-04.T05 — macOS backend.** Use supported signed helper/sandbox profiles where applicable and a controlled VM for unrestricted third-party toolchains. Keep accessibility grants outside untrusted workers. **Verify:** child processes cannot inherit broad desktop/home authority.
- [ ] **SEC-04.T06 — Egress broker.** Revalidate destinations, DNS results and redirects; enforce private-network grants, request/response limits and socket restrictions. **Verify:** redirect/DNS pivots and alternate direct sockets cannot bypass policy.
- [ ] **SEC-04.T07 — WASM host constraints.** Restrict imports, memory, fuel/time and storage; resolve host calls through the capability broker. **Verify:** guest limits also bound host-side I/O and parsing.
- [ ] **SEC-04.T08 — Startup/escape regressions.** Test malicious hooks, project settings, MCP configuration, build scripts and model loaders. **Verify:** containment is active before those inputs can execute, not added after process startup.

**Do not substitute:** a selected working directory for filesystem isolation; Job Objects alone for all Windows security; an HTTPS host allowlist for permission to upload every readable file; or WASM containment for native CLI confinement. Advertise mediated/confined/manual-only status per adapter and tested OS profile.


---

<a id="issue-10"></a>
## #10 — [P0][SEC-05] Privacy governance and incident process

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/10
**Created:** 2026-09-15T12:05:03Z | **Updated:** 2026-09-15T14:19:12Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #14 | **Body-declared parent:** #1

### Original description

Parent: #1

#### Objective
Treat privacy, legal access, data lifecycle, publisher/payment obligations and incident response as engineering constraints from the first release rather than post-launch paperwork.

#### Scope
- Data inventory and data-flow map across local, connector, cloud-agent and sync paths.
- DPIA/privacy-by-design review, retention/deletion schedules and telemetry consent.
- Publisher/content caching, quotation, attribution and redistribution policy hooks.
- Model/license, extension license and third-party binary redistribution review.
- Payment/PCI boundary documentation; avoid handling raw card data where provider/tokenized flows exist.
- Security/privacy incident classification, revocation, disclosure and recovery playbooks.
- User export/deletion exercises and provider-account revocation behavior.

#### Acceptance criteria
- [ ] Each shipped data flow has a declared purpose, retention rule and destination.
- [ ] Privacy mode/telemetry choices are enforceable and testable.
- [ ] User deletion removes eligible local derivatives and schedules/requests applicable remote deletion.
- [ ] Provider/publisher/payment agreements required for advertised capabilities are recorded before launch.
- [ ] Incident exercise demonstrates credential/connector/package revocation and safe client recovery.
- [ ] Product claims accurately distinguish local inference, network privacy and cloud processing.

#### Dependencies
- SEC-01

**First phase:** P0  
**Maturity target:** P7 (continuous)  
**Workstream:** Authority and security

### Discussion (1 comments)

#### Comment 5681791115 — Jordan-Hall — 2026-09-15T14:19:12Z

Source: https://github.com/Jordan-Hall/browser/issues/10#issuecomment-5681791115 | Updated: 2026-09-15T14:19:12Z

<!-- intent-implementation-v1:SEC-05 -->
###### Implementation proposal — SEC-05

Treat privacy, distribution rights and incident handling as enforceable product requirements. Start alongside #6; legal/provider reviews are dependencies, not claims established by this plan.

- [ ] **SEC-05.T01 — Processing/rights inventory.** Record purpose, source, reviewed controller/processor roles, destinations, region assumptions, retention and deletion for local indexes, agents, sync and exports. **Verify:** every shipped data path has an owner and policy.
- [ ] **SEC-05.T02 — Consent/mode receipts.** Separate networking, remote inference, diagnostics, audio retention and learned-pattern choices; version disclosures and revocation. **Verify:** upgrades do not broaden choices silently.
- [ ] **SEC-05.T03 — Retention/export enforcement.** Attach caching, quotation, sharing and expiry rules to derived records; preview restricted omissions on export. **Verify:** deletion jobs and source-policy tests prevent inappropriate redistribution.
- [ ] **SEC-05.T04 — Binary/model/provider rights.** Inventory licenses and approvals for CEF/codecs, weights, native CLIs, packages and service integrations by version. **Verify:** advertised distribution/authentication routes have actual approvals where needed.
- [ ] **SEC-05.T05 — Payment/sensitive data boundaries.** Prefer merchant/PSP tokenized handoff; exclude raw payment authentication data from prompts and traces; document sensitive-domain controls. **Verify:** seeded payment data is absent from diagnostics/artifacts.
- [ ] **SEC-05.T06 — Telemetry/support minimization.** Separate counters from content and provide a previewable redacted support export. **Verify:** private URLs, recipients, prompts, audio and credentials are omitted from default bundles.
- [ ] **SEC-05.T07 — Incident/revocation workflow.** Define severity, owner, containment, credential/package revocation, patch and recovery; implement signed deny-lists and model-independent safe mode. **Verify:** a compromised connector can be disabled without deleting uncertain transactions.
- [ ] **SEC-05.T08 — Lifecycle exercises.** Rehearse export, deletion, account revocation, lost devices, malicious updates and provider termination on release candidates. **Verify:** product claims match implemented controls and recorded rights.

**Additional review item:** assess foreseeable children's use where applicable, sensitive inferred preferences and support operations in the privacy assessment. Current [ICO privacy-by-design guidance](https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/accountability-and-governance/guide-to-accountability-and-governance/data-protection-by-design-and-by-default/) is a reference; obtain product-specific legal review rather than treating a checklist as a compliance certificate.

Approve each task separately; do not close this continuous-governance issue merely because a policy document exists.


---

<a id="issue-153"></a>
## #153 — [TASK][SEC-01.T01] Inventory assets and execution principals

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/153
**Created:** 2026-09-15T15:16:11Z | **Updated:** 2026-09-15T15:16:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #6

### Original description

Parent: #6

Task ID: `SEC-01.T01`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-154"></a>
## #154 — [TASK][SEC-01.T02] Draw trust boundaries and dataflow diagrams

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/154
**Created:** 2026-09-15T15:16:17Z | **Updated:** 2026-09-15T15:16:17Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #6

### Original description

Parent: #6

Task ID: `SEC-01.T02`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-155"></a>
## #155 — [TASK][SEC-01.T03] Specify effect classes and required controls

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/155
**Created:** 2026-09-15T15:16:24Z | **Updated:** 2026-09-15T15:16:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #6

### Original description

Parent: #6

Task ID: `SEC-01.T03`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-156"></a>
## #156 — [TASK][SEC-01.T04] Construct adversarial scenarios

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/156
**Created:** 2026-09-15T15:16:29Z | **Updated:** 2026-09-15T15:16:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #6

### Original description

Parent: #6

Task ID: `SEC-01.T04`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-157"></a>
## #157 — [TASK][SEC-01.T05] Define agent and extension trust classes

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/157
**Created:** 2026-09-15T15:16:33Z | **Updated:** 2026-09-15T15:16:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #6

### Original description

Parent: #6

Task ID: `SEC-01.T05`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-158"></a>
## #158 — [TASK][SEC-01.T06] Translate policy into release conformance

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/158
**Created:** 2026-09-15T15:16:41Z | **Updated:** 2026-09-15T15:16:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #6

### Original description

Parent: #6

Task ID: `SEC-01.T06`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-159"></a>
## #159 — [TASK][SEC-01.T07] Publish incident assumptions and residual risks

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/159
**Created:** 2026-09-15T15:16:47Z | **Updated:** 2026-09-15T15:16:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #6

### Original description

Parent: #6

Task ID: `SEC-01.T07`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-160"></a>
## #160 — [TASK][SEC-01.T08] Keep the model current through change review

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/160
**Created:** 2026-09-15T15:16:55Z | **Updated:** 2026-09-15T15:16:55Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #6

### Original description

Parent: #6

Task ID: `SEC-01.T08`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-161"></a>
## #161 — [TASK][SEC-02.T01] Implement the capability registry

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/161
**Created:** 2026-09-15T15:17:03Z | **Updated:** 2026-09-15T15:17:03Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #7

### Original description

Parent: #7

Task ID: `SEC-02.T01`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-162"></a>
## #162 — [TASK][SEC-02.T02] Build scoped grant value types

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/162
**Created:** 2026-09-15T15:17:11Z | **Updated:** 2026-09-15T15:17:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #7

### Original description

Parent: #7

Task ID: `SEC-02.T02`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-163"></a>
## #163 — [TASK][SEC-02.T03] Implement deterministic policy decisions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/163
**Created:** 2026-09-15T15:17:19Z | **Updated:** 2026-09-15T15:17:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #7

### Original description

Parent: #7

Task ID: `SEC-02.T03`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-164"></a>
## #164 — [TASK][SEC-02.T04] Bind approvals to exact proposals

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/164
**Created:** 2026-09-15T15:17:27Z | **Updated:** 2026-09-15T15:17:27Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #7

### Original description

Parent: #7

Task ID: `SEC-02.T04`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-165"></a>
## #165 — [TASK][SEC-02.T05] Implement dispatch-time revalidation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/165
**Created:** 2026-09-15T15:17:33Z | **Updated:** 2026-09-15T15:17:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #7

### Original description

Parent: #7

Task ID: `SEC-02.T05`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-166"></a>
## #166 — [TASK][SEC-02.T06] Add revocation and cancellation fencing

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/166
**Created:** 2026-09-15T15:17:38Z | **Updated:** 2026-09-15T15:17:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #7

### Original description

Parent: #7

Task ID: `SEC-02.T06`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-167"></a>
## #167 — [TASK][SEC-02.T07] Implement audit records without secret leakage

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/167
**Created:** 2026-09-15T15:17:44Z | **Updated:** 2026-09-15T15:17:44Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #7

### Original description

Parent: #7

Task ID: `SEC-02.T07`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-168"></a>
## #168 — [TASK][SEC-02.T08] Prove the authority lattice and attack resistance

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/168
**Created:** 2026-09-15T15:17:52Z | **Updated:** 2026-09-15T15:17:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #7

### Original description

Parent: #7

Task ID: `SEC-02.T08`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-169"></a>
## #169 — [TASK][SEC-03.T01] Design key hierarchy and unlock lifecycle

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/169
**Created:** 2026-09-15T15:18:00Z | **Updated:** 2026-09-15T15:18:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #8

### Original description

Parent: #8

Task ID: `SEC-03.T01`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-170"></a>
## #170 — [TASK][SEC-03.T02] Implement OS secret-store adapters

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/170
**Created:** 2026-09-15T15:18:09Z | **Updated:** 2026-09-15T15:18:09Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #8

### Original description

Parent: #8

Task ID: `SEC-03.T02`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-171"></a>
## #171 — [TASK][SEC-03.T03] Encrypt private stores and artifacts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/171
**Created:** 2026-09-15T15:18:15Z | **Updated:** 2026-09-15T15:18:15Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #8

### Original description

Parent: #8

Task ID: `SEC-03.T03`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-172"></a>
## #172 — [TASK][SEC-03.T04] Implement access and destination labels

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/172
**Created:** 2026-09-15T15:18:20Z | **Updated:** 2026-09-15T15:18:20Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #8

### Original description

Parent: #8

Task ID: `SEC-03.T04`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-173"></a>
## #173 — [TASK][SEC-03.T05] Secure credential release and diagnostic paths

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/173
**Created:** 2026-09-15T15:18:28Z | **Updated:** 2026-09-15T15:18:28Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #8

### Original description

Parent: #8

Task ID: `SEC-03.T05`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-174"></a>
## #174 — [TASK][SEC-03.T06] Implement rotation and recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/174
**Created:** 2026-09-15T15:18:36Z | **Updated:** 2026-09-15T15:18:36Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #8

### Original description

Parent: #8

Task ID: `SEC-03.T06`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-175"></a>
## #175 — [TASK][SEC-03.T07] Implement forget and retention jobs

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/175
**Created:** 2026-09-15T15:18:43Z | **Updated:** 2026-09-15T15:18:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #8

### Original description

Parent: #8

Task ID: `SEC-03.T07`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-176"></a>
## #176 — [TASK][SEC-03.T08] Test privacy modes and cross-scope leakage

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/176
**Created:** 2026-09-15T15:18:51Z | **Updated:** 2026-09-15T15:18:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #8

### Original description

Parent: #8

Task ID: `SEC-03.T08`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-177"></a>
## #177 — [TASK][SEC-04.T01] Define the isolation contract and adversarial probe

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/177
**Created:** 2026-09-15T15:18:59Z | **Updated:** 2026-09-15T15:18:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #9

### Original description

Parent: #9

Task ID: `SEC-04.T01`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-178"></a>
## #178 — [TASK][SEC-04.T02] Build staged filesystem and environment preparation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/178
**Created:** 2026-09-15T15:19:06Z | **Updated:** 2026-09-15T15:19:06Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #9

### Original description

Parent: #9

Task ID: `SEC-04.T02`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-179"></a>
## #179 — [TASK][SEC-04.T03] Implement Linux confinement backend

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/179
**Created:** 2026-09-15T15:19:13Z | **Updated:** 2026-09-15T15:19:13Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #9

### Original description

Parent: #9

Task ID: `SEC-04.T03`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-180"></a>
## #180 — [TASK][SEC-04.T04] Implement Windows containment backend

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/180
**Created:** 2026-09-15T15:19:19Z | **Updated:** 2026-09-15T15:19:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #9

### Original description

Parent: #9

Task ID: `SEC-04.T04`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-181"></a>
## #181 — [TASK][SEC-04.T05] Implement macOS containment backend

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/181
**Created:** 2026-09-15T15:19:24Z | **Updated:** 2026-09-15T15:19:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #9

### Original description

Parent: #9

Task ID: `SEC-04.T05`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-182"></a>
## #182 — [TASK][SEC-04.T06] Implement egress and redirect enforcement

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/182
**Created:** 2026-09-15T15:19:29Z | **Updated:** 2026-09-15T15:19:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #9

### Original description

Parent: #9

Task ID: `SEC-04.T06`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-183"></a>
## #183 — [TASK][SEC-04.T07] Integrate WASM limits and capability handles

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/183
**Created:** 2026-09-15T15:19:38Z | **Updated:** 2026-09-15T15:19:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #9

### Original description

Parent: #9

Task ID: `SEC-04.T07`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-184"></a>
## #184 — [TASK][SEC-04.T08] Add launch-before-config and escape regressions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/184
**Created:** 2026-09-15T15:19:45Z | **Updated:** 2026-09-15T15:19:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #9

### Original description

Parent: #9

Task ID: `SEC-04.T08`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-185"></a>
## #185 — [TASK][SEC-05.T01] Create the processing and rights inventory

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/185
**Created:** 2026-09-15T15:19:52Z | **Updated:** 2026-09-15T15:19:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #10

### Original description

Parent: #10

Task ID: `SEC-05.T01`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-186"></a>
## #186 — [TASK][SEC-05.T02] Implement consent and privacy-mode receipts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/186
**Created:** 2026-09-15T15:19:56Z | **Updated:** 2026-09-15T15:19:56Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #10

### Original description

Parent: #10

Task ID: `SEC-05.T02`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-187"></a>
## #187 — [TASK][SEC-05.T03] Define enforceable retention and export policies

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/187
**Created:** 2026-09-15T15:20:03Z | **Updated:** 2026-09-15T15:20:03Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #10

### Original description

Parent: #10

Task ID: `SEC-05.T03`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-188"></a>
## #188 — [TASK][SEC-05.T04] Review distribution, model and provider terms

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/188
**Created:** 2026-09-15T15:20:19Z | **Updated:** 2026-09-15T15:20:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #10

### Original description

Parent: #10

Task ID: `SEC-05.T04`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-189"></a>
## #189 — [TASK][SEC-05.T05] Constrain payment and sensitive-domain handling

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/189
**Created:** 2026-09-15T15:20:24Z | **Updated:** 2026-09-15T15:20:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #10

### Original description

Parent: #10

Task ID: `SEC-05.T05`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-190"></a>
## #190 — [TASK][SEC-05.T06] Implement telemetry and support-bundle minimization

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/190
**Created:** 2026-09-15T15:20:42Z | **Updated:** 2026-09-15T15:20:42Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #10

### Original description

Parent: #10

Task ID: `SEC-05.T06`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-191"></a>
## #191 — [TASK][SEC-05.T07] Build incident response and emergency revocation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/191
**Created:** 2026-09-15T15:20:47Z | **Updated:** 2026-09-15T15:20:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #10

### Original description

Parent: #10

Task ID: `SEC-05.T07`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-192"></a>
## #192 — [TASK][SEC-05.T08] Run launch and lifecycle governance exercises

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/192
**Created:** 2026-09-15T15:20:53Z | **Updated:** 2026-09-15T15:20:53Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #10

### Original description

Parent: #10

Task ID: `SEC-05.T08`

Review: pending
Implementation: not started

Source: task breakdown from the implementation handbook and the parent issue implementation proposal.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-215"></a>
## #215 — [TASK][EPIC-SEC.T01] Unify authority and data-flow inventories

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/215
**Created:** 2026-09-15T15:24:43Z | **Updated:** 2026-09-15T15:24:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #14

### Original description

Parent: #14

Task ID: `EPIC-SEC.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-216"></a>
## #216 — [TASK][EPIC-SEC.T02] Integrate policy with all brokers and sandboxes

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/216
**Created:** 2026-09-15T15:24:48Z | **Updated:** 2026-09-15T15:24:48Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #14

### Original description

Parent: #14

Task ID: `EPIC-SEC.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-217"></a>
## #217 — [TASK][EPIC-SEC.T03] Validate hostile-content and privacy scenarios

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/217
**Created:** 2026-09-15T15:24:54Z | **Updated:** 2026-09-15T15:24:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #14

### Original description

Parent: #14

Task ID: `EPIC-SEC.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-218"></a>
## #218 — [TASK][EPIC-SEC.T04] Exercise revocation and incident response

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/218
**Created:** 2026-09-15T15:25:03Z | **Updated:** 2026-09-15T15:25:03Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #14

### Original description

Parent: #14

Task ID: `EPIC-SEC.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

