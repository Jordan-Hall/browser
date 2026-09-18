# Evaluation, release and daily-driver quality

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 31

## Contents

- [#33 — EPIC: Evaluation, release and daily-driver quality](#issue-33)
- [#101 — [P0][EVAL-01] Resettable fixtures and protocol tests](#issue-101)
- [#102 — [P0][EVAL-02] Security, chaos and performance suites](#issue-102)
- [#103 — [P1][EVAL-03] Outcome benchmarks and anti-wrapper tests](#issue-103)
- [#104 — [P1][EVAL-04] Signed distribution, updates and operations](#issue-104)
- [#291 — [TASK][EPIC-EVAL.T01] Ratify the oracle and environment recording architecture](#issue-291)
- [#292 — [TASK][EPIC-EVAL.T02] Integrate deterministic and adversarial CI tiers](#issue-292)
- [#293 — [TASK][EPIC-EVAL.T03] Integrate outcome and hardware qualification](#issue-293)
- [#294 — [TASK][EPIC-EVAL.T04] Integrate signed release and recovery operations](#issue-294)
- [#763 — [TASK][EVAL-01.T01] Define reproducible scenario manifests](#issue-763)
- [#764 — [TASK][EVAL-01.T02] Implement seeded resettable service fixtures](#issue-764)
- [#765 — [TASK][EVAL-01.T03] Implement repository and native desktop fixtures](#issue-765)
- [#766 — [TASK][EVAL-01.T04] Implement fake clocks and controlled event delivery](#issue-766)
- [#767 — [TASK][EVAL-01.T05] Build golden protocol and schema tests](#issue-767)
- [#768 — [TASK][EVAL-01.T06] Implement safe captured-observation replay](#issue-768)
- [#769 — [TASK][EVAL-01.T07] Build evaluator assertions and result artifacts](#issue-769)
- [#770 — [TASK][EVAL-01.T08] Integrate fast CI and expensive scenario tiers](#issue-770)
- [#771 — [TASK][EVAL-02.T01] Map security invariants to attack fixtures](#issue-771)
- [#772 — [TASK][EVAL-02.T02] Build indirect injection and confused-deputy suites](#issue-772)
- [#773 — [TASK][EVAL-02.T03] Instrument durable failure boundaries](#issue-773)
- [#774 — [TASK][EVAL-02.T04] Test sandbox, IPC and supply-chain controls](#issue-774)
- [#775 — [TASK][EVAL-02.T05] Test browser and desktop observation races](#issue-775)
- [#776 — [TASK][EVAL-02.T06] Build accessibility and hardware-load suites](#issue-776)
- [#777 — [TASK][EVAL-02.T07] Test upgrades, backups and recovery semantics](#issue-777)
- [#778 — [TASK][EVAL-02.T08] Integrate external suites and release-blocking gates](#issue-778)
- [#779 — [TASK][EVAL-03.T01] Define outcome scenarios and objective oracles](#issue-779)
- [#780 — [TASK][EVAL-03.T02] Implement the anti-wrapper continuity suite](#issue-780)
- [#781 — [TASK][EVAL-03.T03] Build matched comparison baselines](#issue-781)
- [#782 — [TASK][EVAL-03.T04] Implement repeated trials and failure taxonomy](#issue-782)
- [#783 — [TASK][EVAL-03.T05] Add human usefulness and authority-comprehension review](#issue-783)
- [#784 — [TASK][EVAL-03.T06] Build coverage-aware release dashboards](#issue-784)

---

<a id="issue-33"></a>
## #33 — EPIC: Evaluation, release and daily-driver quality

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/33
**Created:** 2026-09-15T12:09:02Z | **Updated:** 2026-09-15T14:26:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1

Own resettable fixtures, protocol/security/chaos/performance suites, outcome/anti-wrapper benchmarks, signed distribution, updates, backup/restore, diagnostics and operational readiness.

#### Child issues
- [ ] #101 EVAL-01 — Resettable fixtures and protocol tests
- [ ] #102 EVAL-02 — Security, chaos and performance suites
- [ ] #103 EVAL-03 — Outcome benchmarks and anti-wrapper tests
- [ ] #104 EVAL-04 — Signed distribution, updates and operations

#### Cross-cutting gates
Fixture truth is independent of model self-grading; nondeterministic tasks use repeated trials; security/recovery regressions block release; deterministic UI latency is measured separately from model latency; irreversible live actions are never replayed by eval tooling.

### Discussion (1 comments)

#### Comment 5681939212 — Jordan-Hall — 2026-09-15T14:26:51Z

Source: https://github.com/Jordan-Hall/browser/issues/33#issuecomment-5681939212 | Updated: 2026-09-15T14:26:51Z

<!-- intent-implementation-v1:EPIC-EVAL -->
###### Workstream implementation and integration tasks

Integrate #101–#104 continuously from P0. The product harness controls agents; this workstream independently verifies that control.

- [ ] **EPIC-EVAL.T01 — Ratify oracle/environment recording.** Define resettable state, hidden evaluator oracles, fake clocks and complete OS/engine/model/provider/connector manifests. **Proof:** an agent cannot read or alter its success oracle and runs are reproducible at the declared boundary.
- [ ] **EPIC-EVAL.T02 — Integrate deterministic/adversarial CI.** Run contract/property/fuzz tests, injection/egress attacks, dispatch failpoints and migrations. **Proof:** security/recovery regressions block release; captured replay cannot invoke production writes.
- [ ] **EPIC-EVAL.T03 — Integrate outcomes/hardware qualification.** Compare ordinary browsing, chat assistance and persistent workspaces using controlled models/tools; repeat stochastic trials. **Proof:** verified success, interventions, uncertainty, latency and energy are reported separately.
- [ ] **EPIC-EVAL.T04 — Integrate signed release/recovery.** Qualify installers, engine-security updates, backup/restore, safe mode, revocation and redacted support bundles. **Proof:** interrupted upgrades recover without lost workspace state or repeated transactions.

**Acceptance demonstration:** close the chat, restart, disable inference and switch providers while retaining the same usable workspace; inject a failure around an external commit and verify honest reconciliation.

**Review rule:** no aggregate benchmark score overrides an authorization/privacy/duplicate-commit failure. Zero observed attacks succeeding in a finite suite is not a safety proof. Report proposed performance targets separately from measured results and retain independent review.


---

<a id="issue-101"></a>
## #101 — [P0][EVAL-01] Resettable fixtures and protocol tests

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/101
**Created:** 2026-09-15T12:21:19Z | **Updated:** 2026-09-15T21:10:22Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #33

#### Objective
Build the evaluation world alongside the product so every agent/browser/transaction feature has reproducible authoritative truth instead of relying on model self-report or dangerous live actions.

#### Scope
- Resettable fixtures for merchant/catalog/checkout, auctions, social, mail/calendar, project systems, repositories and native desktop apps.
- Seeded users/accounts/balances/permissions/files/source revisions.
- Fake clock/timezone controls and deterministic source events.
- Authoritative fixture APIs for expected postconditions.
- Golden IPC/connector/AgentSession/provider protocol recordings.
- Captured observation replay with all production write routes disabled.
- Scenario manifest recording browser engine, OS image, model hash/quantization, provider version, connector version, permissions and initial state.

#### Evaluation rules
- Fixture truth—not agent narrative—determines externally checkable success.
- Replay can never submit live purchases/bids/messages/deletions.
- Test state must be resettable and isolated per run.

#### Acceptance criteria
- [ ] Core fixture families reset to identical seeded state reproducibly.
- [ ] Fixture APIs expose authoritative expected outcomes for evaluators.
- [ ] Production irreversible actions are technically unavailable in replay mode.
- [ ] Golden protocol tests cover malformed/version-drift/startup/cancellation cases.
- [ ] Fake clock supports auction/schedule/expiry/timeout testing deterministically.
- [ ] Every benchmark run records the complete declared environment/configuration.

#### Dependencies
- CORE-01
- SEC-01

**First phase:** P0  
**Maturity target:** P3  
**Owner:** security-evaluation-release

### Discussion (2 comments)

#### Comment 5682467829 — Jordan-Hall — 2026-09-15T14:54:52Z

Source: https://github.com/Jordan-Hall/browser/issues/101#issuecomment-5682467829 | Updated: 2026-09-15T14:54:52Z

<!-- intent-implementation-v1:EVAL-01 -->
###### Implementation proposal — EVAL-01

Build the evaluation harness alongside contracts/security in #2/#6. Separate agent-visible tasks from protected fixture oracles and disable production credentials/egress in replay.

- [ ] **EVAL-01.T01 — Scenario manifests.** Record initial objects/accounts/permissions, goal, tools, versions, budgets, clocks, expected outcomes and failure injections. **Verify:** public task input contains no hidden oracle state.
- [ ] **EVAL-01.T02 — Seeded service fixtures.** Implement resettable merchant/checkout/auction, social, mail/calendar and project services with realistic public APIs and separately protected truth endpoints. **Verify:** identical seeds restore identical authoritative state.
- [ ] **EVAL-01.T03 — Repository/desktop fixtures.** Supply code tasks with independent tests and pinned VM/session apps, locale, display and files. **Verify:** agents cannot edit protected acceptance tests or fixture evaluator state.
- [ ] **EVAL-01.T04 — Clocks/events.** Inject time/timeout interfaces and deterministically simulate DST, skew, auction extension, duplicates and reordering. **Verify:** race/expiry scenarios reproduce without real waiting or production side effects.
- [ ] **EVAL-01.T05 — Golden protocols.** Capture sanitized versioned IPC/connector/provider exchanges, including malformed, delayed and capability-changing responses. **Verify:** normalization, version negotiation and cancellation regressions fail CI.
- [ ] **EVAL-01.T06 — Safe replay.** Resolve every tool to recorded data or resettable services and reconstruct durable decisions with no production dispatcher or credentials. **Verify:** replaying a purchase/message trace cannot contact the live service.
- [ ] **EVAL-01.T07 — Independent assertions/artifacts.** Query hidden truth for exact files/accounts/receipts and publish bounded traces, environment hashes and failure classes. **Verify:** task success, safety and usability are separate results rather than model self-grading.
- [ ] **EVAL-01.T08 — CI tiers.** Run fast contract/golden/unit suites per change, expensive repeated model/desktop tests on pinned runners and live canaries under separate grants. **Verify:** only deterministic build assets are reused as caches; test initial state resets correctly.

**Review boundary:** fake models validate deterministic plumbing, not actual model quality. Record/replay reproduces selected observations/event order, not arbitrary hidden model internals or already-executed external effects. Keep fixture oracles inaccessible to the evaluated agent.

#### Comment 5688136835 — Jordan-Hall — 2026-09-15T21:10:22Z

Source: https://github.com/Jordan-Hall/browser/issues/101#issuecomment-5688136835 | Updated: 2026-09-15T21:10:22Z

###### Task issues

- [ ] #763 `EVAL-01.T01` — Define reproducible scenario manifests
- [ ] #764 `EVAL-01.T02` — Implement seeded resettable service fixtures
- [ ] #765 `EVAL-01.T03` — Implement repository and native desktop fixtures
- [ ] #766 `EVAL-01.T04` — Implement fake clocks and controlled event delivery
- [ ] #767 `EVAL-01.T05` — Build golden protocol and schema tests
- [ ] #768 `EVAL-01.T06` — Implement safe captured-observation replay
- [ ] #769 `EVAL-01.T07` — Build evaluator assertions and result artifacts
- [ ] #770 `EVAL-01.T08` — Integrate fast CI and expensive scenario tiers

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-102"></a>
## #102 — [P0][EVAL-02] Security, chaos and performance suites

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/102
**Created:** 2026-09-15T12:22:16Z | **Updated:** 2026-09-15T21:11:18Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #33

#### Objective
Continuously prove the runtime fails safely under hostile content, broken dependencies, crashes, resource pressure and real hardware limits.

#### Scope
- Prompt-injection and indirect-injection corpora across pages, email, PDFs, repositories, tool descriptions and connector data.
- Exfiltration, confused-deputy, forged-approval, scope-escalation, malicious-extension/model/package and local IPC abuse scenarios.
- Chaos/fault injection: worker crash, process kill, OOM, disk full, corrupted artifact, network loss, timeout, duplicate event, expired auth, rate limit, schema drift and partial provider response.
- Stale DOM/accessibility/screenshot and changed-account/recipient/quote scenarios.
- Browser/local-AI/speech/desktop sustained-load and thermal/resource tests on reference hardware.
- Accessibility and generated-layout regression suite.
- Database/schema/package migration and rollback tests.
- External suites such as AgentDojo/OSWorld used as supplements where useful.

#### Release rules
- Security and recovery gates are blocking, not dashboard-only metrics.
- Zero observed failures is not proof of safety; preserve adversarial review/red-team work.
- Deterministic UI/control latency is measured separately from model/provider latency.

#### Acceptance criteria
- [ ] No release-suite scenario gains authority from hostile content or tool metadata.
- [ ] Designed crash/timeout/duplicate-delivery cases converge to a correct durable state without blind side-effect replay.
- [ ] Egress/credential/scope attack fixtures fail closed and produce useful audit state.
- [ ] Accessibility gates cover trusted shell and generated component layouts.
- [ ] Reference-hardware tests report latency, memory, energy/thermal and degraded-mode behavior.
- [ ] Migration/rollback fixtures preserve supported durable state and detect incompatible downgrade.

#### Dependencies
- EVAL-01
- SEC-01
- SEC-02

**First phase:** P0  
**Maturity target:** P7 (continuous)  
**Owner:** security-evaluation-release

### Discussion (2 comments)

#### Comment 5682474386 — Jordan-Hall — 2026-09-15T14:55:13Z

Source: https://github.com/Jordan-Hall/browser/issues/102#issuecomment-5682474386 | Updated: 2026-09-15T14:55:13Z

<!-- intent-implementation-v1:EVAL-02 -->
###### Implementation proposal — EVAL-02

Turn #6/#7 security invariants into release-blocking tests on #101 fixtures. Keep performance and task-success scores separate from authority/privacy/duplicate-effect failures.

- [ ] **EVAL-02.T01 — Invariant/attack mapping.** Map every trust boundary to no-escalation/no-egress/no-spoofing/no-duplicate-effect assertions for pages, messages, docs, repos, tool metadata and packages. **Verify:** every shipped authority path has an attack case and legitimate control case.
- [ ] **EVAL-02.T02 — Injection/confused deputy.** Seed account swaps, memory poisoning, secret reads, destination changes and forged approvals in untrusted inputs. **Verify:** deterministic brokers/sandboxes block forbidden effects regardless of model compliance claims.
- [ ] **EVAL-02.T03 — Durable failpoints.** Instrument test-only boundaries around commits/outbox/dispatch/acknowledgements/artifact publication/reservation settlement; schedule kills/timeouts/disk failure. **Verify:** all designed failure windows produce correct recoverable state without blind replay.
- [ ] **EVAL-02.T04 — Sandbox/IPC/supply chain.** Test impersonated workers, inherited descriptors/env, DNS/redirect pivots, undeclared imports, traversal and model/package loaders; fuzz parsers/unsafe boundaries. **Verify:** seeded private resources remain inaccessible.
- [ ] **EVAL-02.T05 — Browser/desktop races.** Replace frames/DOM/AX targets, switch accounts/windows and inject human takeover between observation and action. **Verify:** stale observations and revoked leases cannot authorize subsequent input.
- [ ] **EVAL-02.T06 — Accessibility/mixed load.** Run keyboard/screen-reader/IME/zoom/contrast flows under sustained browser/speech/inference/desktop load. **Verify:** report p50/p95/p99 and resource/thermal/energy metrics where measured, separately from model latency.
- [ ] **EVAL-02.T07 — Updates/backups/recovery.** Test interrupted installs, old backups, incompatible rollback and revoked packages with unknown external operations present. **Verify:** journals, security floors and current authority survive correctly.
- [ ] **EVAL-02.T08 — External suites/gates.** Pin relevant [AgentDojo](https://github.com/ethz-spylab/agentdojo) and [OSWorld](https://github.com/xlang-ai/OSWorld-V2) configurations alongside held-out product cases. **Verify:** supplemental benchmark success cannot override a product security failure.

**Review rule:** performance thresholds remain proposed until measured on declared hardware. Zero observed successful attacks is not proof of universal safety; retain independent review and adversarial testing. Experimental non-security exceptions need named owners and expiry rather than silently relaxing invariants.

#### Comment 5688147419 — Jordan-Hall — 2026-09-15T21:11:18Z

Source: https://github.com/Jordan-Hall/browser/issues/102#issuecomment-5688147419 | Updated: 2026-09-15T21:11:18Z

###### Task issues

- [ ] #771 `EVAL-02.T01` — Map security invariants to attack fixtures
- [ ] #772 `EVAL-02.T02` — Build indirect injection and confused-deputy suites
- [ ] #773 `EVAL-02.T03` — Instrument durable failure boundaries
- [ ] #774 `EVAL-02.T04` — Test sandbox, IPC and supply-chain controls
- [ ] #775 `EVAL-02.T05` — Test browser and desktop observation races
- [ ] #776 `EVAL-02.T06` — Build accessibility and hardware-load suites
- [ ] #777 `EVAL-02.T07` — Test upgrades, backups and recovery semantics
- [ ] #778 `EVAL-02.T08` — Integrate external suites and release-blocking gates

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-103"></a>
## #103 — [P1][EVAL-03] Outcome benchmarks and anti-wrapper tests

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/103
**Created:** 2026-09-15T12:22:27Z | **Updated:** 2026-09-15T14:55:40Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #33

#### Objective
Measure whether the product actually delivers a persistent user-owned computing experience that outperforms ordinary browsing/chat assistance on verified outcomes—not merely a better-looking agent demo.

#### Scope
- Scenario suites for research, shopping comparison, synthesized reading, social, coding, files, browser automation and native desktop tasks.
- Compare ordinary browsing, chat-based assistance using the same underlying tools/models, and persistent workspace mode.
- Anti-wrapper tests: close chat, restart app, disable inference, change provider/model and continue using the workspace.
- Repeated trials for nondeterministic tasks with confidence intervals and failure taxonomy.
- Human review for relevance, usability, evidence quality and corrective intervention burden.
- Metrics: verified completion, elapsed time, interventions, forced Original-mode handoffs, source/evidence quality, user understanding of authority, latency/cost/energy.
- Held-out private scenarios plus staged low-risk production canaries.

#### Evaluation rules
- The model cannot be the sole grader of its own completion.
- Provider/model comparisons use pinned versions/configurations and disclose them.
- Do not use time-spent-in-product as the north-star metric; forced source inspection and healthy provenance use must not be penalized.

#### Acceptance criteria
- [ ] Workspace remains directly usable after chat closure/restart with inference disabled for supported deterministic interactions.
- [ ] Provider switch preserves GoalContract, workspace, evidence and artifacts for declared scenarios.
- [ ] Benchmarks report verified completion separately from attempted/self-reported completion.
- [ ] Nondeterministic results include repeated trials, confidence intervals, interventions and failure classes.
- [ ] Comparisons use the same underlying tools/models where the goal is to isolate the interaction/runtime advantage.
- [ ] Release dashboards expose cost, latency, energy and correctness together rather than optimizing a single benchmark score.

#### Dependencies
- EVAL-01
- WS-01
- WS-02

**First phase:** P1  
**Maturity target:** P7 (continuous)  
**Owner:** security-evaluation-release

### Discussion (1 comments)

#### Comment 5682482301 — Jordan-Hall — 2026-09-15T14:55:39Z

Source: https://github.com/Jordan-Hall/browser/issues/103#issuecomment-5682482301 | Updated: 2026-09-15T14:55:39Z

<!-- intent-implementation-v1:EVAL-03 -->
###### Implementation proposal — EVAL-03

Build coverage-aware outcome benchmarks over #101/#37/#38. Measure the value of persistent user-owned interfaces separately from simply choosing a stronger model.

- [ ] **EVAL-03.T01 — Outcomes/oracles.** Define representative research, commerce, reading, social, coding, file and desktop tasks with explicit supported scope and exact success predicates. **Verify:** unsafe completion cannot count as a successful verified outcome.
- [ ] **EVAL-03.T02 — Anti-wrapper continuity.** Create saved apps, close chat, restart, disable inference and switch providers. **Verify:** layout, evidence, deterministic controls and durable task/artifact state remain available without conversation reconstruction.
- [ ] **EVAL-03.T03 — Matched baselines.** Compare ordinary browsing, chat-with-tools and persistent workspace conditions using the same models/sources/permissions and comparable budgets where relevant. **Verify:** unavoidable differences are recorded rather than attributed to the interface.
- [ ] **EVAL-03.T04 — Repeated trials/failures.** Run independent trials across pinned environments, recording interventions, elapsed time, compute/cost and measured energy; compute intervals and failure classes. **Verify:** one lucky run does not establish a support claim.
- [ ] **EVAL-03.T05 — Human usefulness/authority review.** Assess relevance, evidence quality, UI stability, approval comprehension and correction burden with structured/blinded review where practical. **Verify:** disagreement and reviewer context remain visible.
- [ ] **EVAL-03.T06 — Release dashboards.** Break out verified outcomes, safety, latency/cost/energy and forced versus voluntary Original use by feature/hardware/provider. **Verify:** unsupported coverage cannot disappear inside an aggregate score.
- [ ] **EVAL-03.T07 — Live canaries/promotion.** Use separately granted low-risk accounts/budgets/teardown, compare with fixtures and retain private held-out cases. **Verify:** production reliability is checked without exposing unrestricted live writes to benchmark replay.

**Product test:** the second visit matters as much as first-run generation. Do not penalize voluntary source inspection or reward trapping users inside the application. Proposed success/latency thresholds become claims only with versioned task coverage and measured evidence; a model cannot be its own sole judge.


---

<a id="issue-104"></a>
## #104 — [P1][EVAL-04] Signed distribution, updates and operations

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/104
**Created:** 2026-09-15T12:22:38Z | **Updated:** 2026-09-15T14:56:10Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #33

#### Objective
Make the browser/runtime safely installable, patchable, recoverable and supportable as everyday software, with an independent fast lane for browser/security updates.

#### Scope
- Signed installers/packages for declared desktop platforms.
- Signed update manifests, staged rollout channels, rollback and emergency revocation.
- Separate browser-engine security patch pipeline from feature releases.
- Dependency inventory/SBOM, binary/model/package provenance and reproducible-build goals where feasible.
- Database/schema/artifact backup, restore and migration recovery.
- Crash diagnostics and privacy-preserving telemetry with explicit consent/settings.
- Safe mode with inference/extensions/connectors disabled for recovery.
- Incident-response integration, support bundles and operational runbooks.
- Release support matrix for OS/browser/model/connectors and known limitations.

#### Release rules
- Failed update must preserve or recover user-owned state without replaying consequential operations.
- Rollback cannot silently downgrade to a known-vulnerable or schema-incompatible build.
- Security patching cannot wait for unrelated feature readiness.

#### Acceptance criteria
- [ ] Signed install/update/rollback paths are tested on every declared supported platform.
- [ ] Interrupted/corrupt update returns to a bootable safe prior/current build without workspace loss.
- [ ] Browser-engine emergency update can ship independently of the main feature train.
- [ ] Backup/restore exercise recovers declared workspace/artifact state and preserves transaction/audit correctness.
- [ ] Safe mode can start with local inference, extensions and third-party connectors disabled.
- [ ] Support bundle redacts credentials/private content according to policy and records versions needed for reproduction.

#### Dependencies
- SEC-01
- SEC-05
- CORE-02

**First phase:** P1  
**Maturity target:** P7  
**Owner:** security-evaluation-release

### Discussion (1 comments)

#### Comment 5682491926 — Jordan-Hall — 2026-09-15T14:56:10Z

Source: https://github.com/Jordan-Hall/browser/issues/104#issuecomment-5682491926 | Updated: 2026-09-15T14:56:10Z

<!-- intent-implementation-v1:EVAL-04 -->
###### Implementation proposal — EVAL-04

Build the signed release/recovery pipeline over #6/#10/#3. Keep build/test/promotion/signing permissions distinct and browser security patching independent from feature delivery.

- [ ] **EVAL-04.T01 — Distribution/signing boundary.** Select supported OS/architecture package formats, signing/notarization routes and accountable owners. **Verify:** ordinary build agents cannot access production signing keys or promote untested artifacts.
- [ ] **EVAL-04.T02 — Packages/provenance.** Pin compiler/engine/dependencies, generate installers and SBOM/build-input records, test reproducibility where feasible and review redistribution rights. **Verify:** shipped binaries map to tested source/dependencies/licenses.
- [ ] **EVAL-04.T03 — Signed discovery/staging.** Verify metadata signatures, expiry, hashes, compatibility and security floors; quarantine downloads before channel-based rollout. **Verify:** stale/frozen metadata or substituted content cannot silently install.
- [ ] **EVAL-04.T04 — Migration-safe activation.** Snapshot compatible state, run reviewed migrations and atomically switch executable sets where supported; retain recovery checkpoints. **Verify:** executable rollback cannot open an incompatible database or bypass a security floor.
- [ ] **EVAL-04.T05 — Browser patch lane.** Track engine security releases and qualify the browser corpus independently of unrelated features; maintain emergency revocation. **Verify:** a critical compatible engine patch can ship without waiting for the full feature train.
- [ ] **EVAL-04.T06 — Backup/restore/safe mode.** Create consistent encrypted metadata/artifact backups, validate clean-device restore and start without inference/extensions/connectors. **Verify:** unknown external effects remain reconcilable, not automatically repeated after restore.
- [ ] **EVAL-04.T07 — Diagnostics/support.** Collect controlled version/error/resource metadata and preview redacted support bundles; document triage and revocation. **Verify:** default support exports exclude credentials/private content.
- [ ] **EVAL-04.T08 — Lifecycle qualification.** Test clean install, upgrade, corruption, low disk, interrupted update, expired metadata, restore and safe mode on every advertised platform. **Verify:** recovery works independently of models and limitations/maintenance ownership are published.

**Review boundary:** a checksum URL is not a complete update trust design. A/B executable rollback must respect data/schema compatibility and revoked-version policy. A backup cannot revoke a transaction already accepted by a provider, and restore must not resurrect stale approval authority.


---

<a id="issue-291"></a>
## #291 — [TASK][EPIC-EVAL.T01] Ratify the oracle and environment recording architecture

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/291
**Created:** 2026-09-15T18:10:02Z | **Updated:** 2026-09-15T18:10:02Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #33

### Original description

Parent: #33

Task ID: `EPIC-EVAL.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-292"></a>
## #292 — [TASK][EPIC-EVAL.T02] Integrate deterministic and adversarial CI tiers

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/292
**Created:** 2026-09-15T18:10:07Z | **Updated:** 2026-09-15T18:10:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #33

### Original description

Parent: #33

Task ID: `EPIC-EVAL.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-293"></a>
## #293 — [TASK][EPIC-EVAL.T03] Integrate outcome and hardware qualification

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/293
**Created:** 2026-09-15T18:10:12Z | **Updated:** 2026-09-15T18:10:12Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #33

### Original description

Parent: #33

Task ID: `EPIC-EVAL.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-294"></a>
## #294 — [TASK][EPIC-EVAL.T04] Integrate signed release and recovery operations

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/294
**Created:** 2026-09-15T18:10:16Z | **Updated:** 2026-09-15T18:10:16Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #33

### Original description

Parent: #33

Task ID: `EPIC-EVAL.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-763"></a>
## #763 — [TASK][EVAL-01.T01] Define reproducible scenario manifests

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/763
**Created:** 2026-09-15T21:09:38Z | **Updated:** 2026-09-15T21:09:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #101

### Original description

Parent: #101

Task ID: `EVAL-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-764"></a>
## #764 — [TASK][EVAL-01.T02] Implement seeded resettable service fixtures

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/764
**Created:** 2026-09-15T21:09:42Z | **Updated:** 2026-09-15T21:09:42Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #101

### Original description

Parent: #101

Task ID: `EVAL-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-765"></a>
## #765 — [TASK][EVAL-01.T03] Implement repository and native desktop fixtures

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/765
**Created:** 2026-09-15T21:09:50Z | **Updated:** 2026-09-15T21:09:50Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #101

### Original description

Parent: #101

Task ID: `EVAL-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-766"></a>
## #766 — [TASK][EVAL-01.T04] Implement fake clocks and controlled event delivery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/766
**Created:** 2026-09-15T21:09:57Z | **Updated:** 2026-09-15T21:09:57Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #101

### Original description

Parent: #101

Task ID: `EVAL-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-767"></a>
## #767 — [TASK][EVAL-01.T05] Build golden protocol and schema tests

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/767
**Created:** 2026-09-15T21:10:02Z | **Updated:** 2026-09-15T21:10:02Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #101

### Original description

Parent: #101

Task ID: `EVAL-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-768"></a>
## #768 — [TASK][EVAL-01.T06] Implement safe captured-observation replay

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/768
**Created:** 2026-09-15T21:10:06Z | **Updated:** 2026-09-15T21:10:06Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #101

### Original description

Parent: #101

Task ID: `EVAL-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-769"></a>
## #769 — [TASK][EVAL-01.T07] Build evaluator assertions and result artifacts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/769
**Created:** 2026-09-15T21:10:11Z | **Updated:** 2026-09-15T21:10:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #101

### Original description

Parent: #101

Task ID: `EVAL-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-770"></a>
## #770 — [TASK][EVAL-01.T08] Integrate fast CI and expensive scenario tiers

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/770
**Created:** 2026-09-15T21:10:17Z | **Updated:** 2026-09-15T21:10:17Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #101

### Original description

Parent: #101

Task ID: `EVAL-01.T08`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-771"></a>
## #771 — [TASK][EVAL-02.T01] Map security invariants to attack fixtures

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/771
**Created:** 2026-09-15T21:10:28Z | **Updated:** 2026-09-15T21:10:28Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #102

### Original description

Parent: #102

Task ID: `EVAL-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-772"></a>
## #772 — [TASK][EVAL-02.T02] Build indirect injection and confused-deputy suites

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/772
**Created:** 2026-09-15T21:10:35Z | **Updated:** 2026-09-15T21:10:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #102

### Original description

Parent: #102

Task ID: `EVAL-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-773"></a>
## #773 — [TASK][EVAL-02.T03] Instrument durable failure boundaries

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/773
**Created:** 2026-09-15T21:10:40Z | **Updated:** 2026-09-15T21:10:40Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #102

### Original description

Parent: #102

Task ID: `EVAL-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-774"></a>
## #774 — [TASK][EVAL-02.T04] Test sandbox, IPC and supply-chain controls

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/774
**Created:** 2026-09-15T21:10:45Z | **Updated:** 2026-09-15T21:10:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #102

### Original description

Parent: #102

Task ID: `EVAL-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-775"></a>
## #775 — [TASK][EVAL-02.T05] Test browser and desktop observation races

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/775
**Created:** 2026-09-15T21:10:51Z | **Updated:** 2026-09-15T21:10:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #102

### Original description

Parent: #102

Task ID: `EVAL-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-776"></a>
## #776 — [TASK][EVAL-02.T06] Build accessibility and hardware-load suites

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/776
**Created:** 2026-09-15T21:10:57Z | **Updated:** 2026-09-15T21:10:57Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #102

### Original description

Parent: #102

Task ID: `EVAL-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-777"></a>
## #777 — [TASK][EVAL-02.T07] Test upgrades, backups and recovery semantics

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/777
**Created:** 2026-09-15T21:11:02Z | **Updated:** 2026-09-15T21:11:02Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #102

### Original description

Parent: #102

Task ID: `EVAL-02.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-778"></a>
## #778 — [TASK][EVAL-02.T08] Integrate external suites and release-blocking gates

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/778
**Created:** 2026-09-15T21:11:10Z | **Updated:** 2026-09-15T21:11:10Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #102

### Original description

Parent: #102

Task ID: `EVAL-02.T08`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-779"></a>
## #779 — [TASK][EVAL-03.T01] Define outcome scenarios and objective oracles

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/779
**Created:** 2026-09-15T21:11:25Z | **Updated:** 2026-09-15T21:11:25Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #103

### Original description

Parent: #103

Task ID: `EVAL-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-780"></a>
## #780 — [TASK][EVAL-03.T02] Implement the anti-wrapper continuity suite

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/780
**Created:** 2026-09-15T21:11:30Z | **Updated:** 2026-09-15T21:11:30Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #103

### Original description

Parent: #103

Task ID: `EVAL-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-781"></a>
## #781 — [TASK][EVAL-03.T03] Build matched comparison baselines

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/781
**Created:** 2026-09-15T21:11:38Z | **Updated:** 2026-09-15T21:11:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #103

### Original description

Parent: #103

Task ID: `EVAL-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-782"></a>
## #782 — [TASK][EVAL-03.T04] Implement repeated trials and failure taxonomy

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/782
**Created:** 2026-09-15T21:11:43Z | **Updated:** 2026-09-15T21:11:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #103

### Original description

Parent: #103

Task ID: `EVAL-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-783"></a>
## #783 — [TASK][EVAL-03.T05] Add human usefulness and authority-comprehension review

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/783
**Created:** 2026-09-15T21:11:48Z | **Updated:** 2026-09-15T21:11:48Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #103

### Original description

Parent: #103

Task ID: `EVAL-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-784"></a>
## #784 — [TASK][EVAL-03.T06] Build coverage-aware release dashboards

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/784
**Created:** 2026-09-15T21:11:54Z | **Updated:** 2026-09-15T21:11:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #103

### Original description

Parent: #103

Task ID: `EVAL-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

