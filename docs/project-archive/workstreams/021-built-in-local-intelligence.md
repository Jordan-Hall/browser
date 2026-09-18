# Built-in local intelligence

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 37

## Contents

- [#21 — EPIC: Built-in local intelligence](#issue-21)
- [#61 — [P0][LOCAL-01] Offline/local/hybrid inference policies](#issue-61)
- [#62 — [P1][LOCAL-02] Role-specific model packs](#issue-62)
- [#63 — [P1][LOCAL-03] Native local coding and tool agent](#issue-63)
- [#64 — [P0][LOCAL-04] Hardware/resource profiling and optimization](#issue-64)
- [#243 — [TASK][EPIC-LOCAL.T01] Define local capability and hardware profiles](#issue-243)
- [#244 — [TASK][EPIC-LOCAL.T02] Integrate inference modes and signed model packs](#issue-244)
- [#245 — [TASK][EPIC-LOCAL.T03] Integrate the brokered local agent and speech scheduling](#issue-245)
- [#246 — [TASK][EPIC-LOCAL.T04] Qualify quality, energy and graceful degradation](#issue-246)
- [#487 — [TASK][LOCAL-01.T01] Define inference contracts and mode policy](#issue-487)
- [#488 — [TASK][LOCAL-01.T02] Implement the first supervised local runtime](#issue-488)
- [#489 — [TASK][LOCAL-01.T03] Enforce offline and destination restrictions](#issue-489)
- [#490 — [TASK][LOCAL-01.T04] Implement structured output and tool-call proposals](#issue-490)
- [#491 — [TASK][LOCAL-01.T05] Implement cancellation and model lifecycle](#issue-491)
- [#492 — [TASK][LOCAL-01.T06] Implement explicit escalation and useful degradation](#issue-492)
- [#493 — [TASK][LOCAL-01.T07] Qualify inference policy across failure modes](#issue-493)
- [#494 — [TASK][LOCAL-02.T01] Define role and manifest schemas](#issue-494)
- [#495 — [TASK][LOCAL-02.T02] Build candidate evaluation and selection](#issue-495)
- [#496 — [TASK][LOCAL-02.T03] Implement verified resumable installation](#issue-496)
- [#497 — [TASK][LOCAL-02.T04] Implement compatibility and storage management](#issue-497)
- [#498 — [TASK][LOCAL-02.T05] Implement update, rollback and revocation](#issue-498)
- [#499 — [TASK][LOCAL-02.T06] Integrate role-aware scheduling](#issue-499)
- [#500 — [TASK][LOCAL-02.T07] Publish support and regression evidence](#issue-500)
- [#501 — [TASK][LOCAL-03.T01] Implement local AgentSession and planner loop](#issue-501)
- [#502 — [TASK][LOCAL-03.T02] Build repository retrieval and code context](#issue-502)
- [#503 — [TASK][LOCAL-03.T03] Implement structured edits and patch artifacts](#issue-503)
- [#504 — [TASK][LOCAL-03.T04] Integrate isolated build, test and diagnostics tools](#issue-504)
- [#505 — [TASK][LOCAL-03.T05] Implement bounded repair and verification](#issue-505)
- [#506 — [TASK][LOCAL-03.T06] Add UI/workflow composition and scoped tool tasks](#issue-506)
- [#507 — [TASK][LOCAL-03.T07] Qualify local coding on declared hardware](#issue-507)
- [#508 — [TASK][LOCAL-04.T01] Implement hardware and backend detection](#issue-508)
- [#509 — [TASK][LOCAL-04.T02] Build workload measurement instrumentation](#issue-509)
- [#510 — [TASK][LOCAL-04.T03] Define hardware cohorts and admission estimates](#issue-510)
- [#511 — [TASK][LOCAL-04.T04] Implement residency, eviction and cooperative priority](#issue-511)
- [#512 — [TASK][LOCAL-04.T05] Implement OOM and thermal/battery degradation](#issue-512)
- [#513 — [TASK][LOCAL-04.T06] Publish performance diagnostics and support guidance](#issue-513)
- [#514 — [TASK][LOCAL-04.T07] Run mixed-workload and regression qualification](#issue-514)

---

<a id="issue-21"></a>
## #21 — EPIC: Built-in local intelligence

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/21
**Created:** 2026-09-15T12:07:46Z | **Updated:** 2026-09-15T14:23:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1

Own local inference runtime abstraction, model packs, hardware-aware routing and the built-in local coding/planning agent. Local capability is a measured product guarantee, not a model-name checkbox.

#### Child issues
- [ ] #61 LOCAL-01 — Offline/local/hybrid inference policies
- [ ] #62 LOCAL-02 — Role-specific model packs
- [ ] #63 LOCAL-03 — Native local coding and tool agent
- [ ] #64 LOCAL-04 — Hardware/resource profiling and optimization

#### Cross-cutting gates
Offline/local/hybrid modes are explicit, no hidden cloud fallback, model packages are signed/licensed/tested, resource scheduling preserves UI/speech responsiveness, and coding quality is measured by verified outcomes.

### Discussion (1 comments)

#### Comment 5681867404 — Jordan-Hall — 2026-09-15T14:23:04Z

Source: https://github.com/Jordan-Hall/browser/issues/21#issuecomment-5681867404 | Updated: 2026-09-15T14:23:04Z

<!-- intent-implementation-v1:EPIC-LOCAL -->
###### Workstream implementation and integration tasks

Integrate #61–#64 with speech, retrieval, UI and coding. Local intelligence is a measured capability set, not a single model-name promise.

- [ ] **EPIC-LOCAL.T01 — Define capability/hardware profiles.** Select role-specific workloads and reference devices; record runtime, weights, quantization, context and usable accelerator memory. **Proof:** each support claim names tested hardware and verified tasks.
- [ ] **EPIC-LOCAL.T02 — Integrate modes/model packs.** Connect offline/local-connectors/hybrid policy, supervised inference, signed pack installation and explicit destination grants. **Proof:** network-denied operation works and model failure cannot silently invoke a cloud provider.
- [ ] **EPIC-LOCAL.T03 — Integrate brokered tools and speech scheduling.** Route local-agent actions through policy, run code tests in isolation and prioritize control/speech over background generation. **Proof:** a useful offline patch passes independent tests while UI/stop remain responsive.
- [ ] **EPIC-LOCAL.T04 — Qualify quality/energy/degradation.** Measure verified outcomes, latency, memory, thermal/energy telemetry where available and smaller-task fallback. **Proof:** resource pressure causes visible degradation, not OOM loops or hidden cloud use.

**Demonstration:** disconnect networking, transcribe an intent, inspect a repository, produce a scoped tested change, and reopen its workspace after model shutdown.

**Review boundaries:** active parameters are not installed-weight memory; system RAM is not usable VRAM; local inference does not make online connector requests anonymous. Separate measured support from proposed targets.


---

<a id="issue-61"></a>
## #61 — [P0][LOCAL-01] Offline/local/hybrid inference policies

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/61
**Created:** 2026-09-15T12:13:54Z | **Updated:** 2026-09-15T19:34:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #21

#### Objective
Implement supervised local inference with enforceable offline/local-with-connectors/hybrid modes and no hidden cloud fallback.

#### Scope
- Inference worker adapter with model load/unload, request/cancel, health and resource reporting.
- `Offline`: deny network for inference and use only installed/cached data.
- `Local inference + online connectors`: keep model local while separately permitting granted connector/web network.
- `Hybrid`: explicitly named remote providers with destination-scoped context grants.
- Failover policy that requests a mode change rather than silently escalating remotely.
- Per-request destination/telemetry trace.
- Runtime adapters initially compatible with llama.cpp-class local runtimes without tying product state to one backend.

#### Security / product rules
- Local inference is not synonymous with anonymous browsing; network activity is reported separately.
- Remote provider permission is scoped to selected context/destinations.
- Model workers have no direct authority over tools.

#### Acceptance criteria
- [ ] Offline mode technically prevents inference-worker network egress.
- [ ] Local-model failure cannot trigger remote inference without a new applicable grant/user-visible mode change.
- [ ] Connector networking remains independently controllable from inference networking.
- [ ] Cancellation works while generation/model loading is active.
- [ ] UI exposes active mode/provider/model and where inference runs.
- [ ] Network-denial and hidden-fallback regression tests run in CI.

#### Dependencies
- CORE-03
- SEC-02
- SEC-04

**First phase:** P0  
**Maturity target:** P1  
**Owner:** local-ai-speech

### Discussion (2 comments)

#### Comment 5682183365 — Jordan-Hall — 2026-09-15T14:39:54Z

Source: https://github.com/Jordan-Hall/browser/issues/61#issuecomment-5682183365 | Updated: 2026-09-15T14:39:54Z

<!-- intent-implementation-v1:LOCAL-01 -->
###### Implementation proposal — LOCAL-01

Implement a supervised inference worker over #4/#7/#9. InferenceRequest includes model/runtime identity, eligible context, budget, destination and cancellation ID; model output has no direct tool authority.

- [ ] **LOCAL-01.T01 — Contracts and modes.** Define generation/embedding/streaming contracts and separate offline, local-inference-with-connectors and hybrid policy. **Verify:** connector networking and inference networking have independent grants; UI mode is not the enforcement mechanism.
- [ ] **LOCAL-01.T02 — First local backend.** Integrate a pinned llama.cpp-class runtime in a separate worker with bounded requests, health and resource reporting. **Verify:** tensor/FFI failure cannot crash trusted chrome or corrupt task state.
- [ ] **LOCAL-01.T03 — Enforce destinations.** Apply actual sandbox/egress denial offline; allow named hybrid endpoints only for context eligible for that destination. **Verify:** packet/egress probes distinguish advertised mode from actual connections, including failed requests and downloads.
- [ ] **LOCAL-01.T04 — Structured outputs.** Validate generated schemas/tool proposals and return them to the supervisor; bound repair attempts. **Verify:** malformed output cannot execute a tool or acquire a grant.
- [ ] **LOCAL-01.T05 — Model/request lifecycle.** Implement cancellation, load/unload, cache eviction and timeout through supervisor scheduling. **Verify:** local acknowledgement and actual worker completion remain separate; cached context carries no authority.
- [ ] **LOCAL-01.T06 — Useful degradation/escalation.** On OOM/unavailable or inadequate models, preserve artifacts and propose a smaller task/model or explicit hybrid transition with a context preview. **Verify:** no hidden cloud fallback occurs.
- [ ] **LOCAL-01.T07 — Failure qualification.** Test missing/corrupt weights, oversized context, worker crashes, denied downloads, malformed output and backend drift. **Verify:** each result records model/runtime hashes, hardware and effective mode.

**Reference:** [llama.cpp](https://github.com/ggml-org/llama.cpp). A loopback model HTTP service still needs authentication, request limits and network/process containment. Offline means relevant networking is denied, not merely that a local model is selected. Keep backend integration replaceable without moving workspace state into it.

#### Comment 5686967606 — Jordan-Hall — 2026-09-15T19:34:04Z

Source: https://github.com/Jordan-Hall/browser/issues/61#issuecomment-5686967606 | Updated: 2026-09-15T19:34:04Z

###### Task issues
- [ ] #487 `LOCAL-01.T01` — Define inference contracts and mode policy
- [ ] #488 `LOCAL-01.T02` — Implement the first supervised local runtime
- [ ] #489 `LOCAL-01.T03` — Enforce offline and destination restrictions
- [ ] #490 `LOCAL-01.T04` — Implement structured output and tool-call proposals
- [ ] #491 `LOCAL-01.T05` — Implement cancellation and model lifecycle
- [ ] #492 `LOCAL-01.T06` — Implement explicit escalation and useful degradation
- [ ] #493 `LOCAL-01.T07` — Qualify inference policy across failure modes


---

<a id="issue-62"></a>
## #62 — [P1][LOCAL-02] Role-specific model packs

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/62
**Created:** 2026-09-15T12:14:08Z | **Updated:** 2026-09-15T19:35:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #21

#### Objective
Package local models by product role and measured hardware support instead of treating one large model as the answer to speech, retrieval, UI and coding.

#### Scope
- Model-pack manifest: role, model/runtime IDs, hashes, tokenizer/templates, license, quantization, context limits, hardware requirements and evaluation results.
- Roles: coding/planning, fast extraction/UI planning, embeddings/reranking, speech-related support where applicable, and visual grounding.
- Signed downloads, resumable installation, disk management, rollback and version pinning.
- Disable loaders that execute untrusted repository/custom model code.
- Compatibility matrix per OS/accelerator/runtime.
- Benchmark/regression gate before default-pack promotion.

#### Product rules
- Installed weights and active parameters are distinct concepts; publish realistic storage/RAM/VRAM requirements.
- Candidate model names are not permanent product promises.
- Privacy mode cannot change silently because a pack/runtime changes.

#### Acceptance criteria
- [ ] Every installed pack exposes hashes, license, hardware/context limits and measured task results.
- [ ] Corrupt/unsigned/untrusted-code model packages are rejected.
- [ ] Rollback restores the last known-good compatible pack.
- [ ] Unsupported hardware receives a useful smaller-pack/degraded path rather than crash/OOM loops.
- [ ] Pack updates run regression suites before becoming defaults.
- [ ] Storage cleanup never deletes an actively required model without explicit state transition.

#### Dependencies
- LOCAL-01
- SEC-03

**First phase:** P1  
**Maturity target:** P5  
**Owner:** local-ai-speech

### Discussion (2 comments)

#### Comment 5682189612 — Jordan-Hall — 2026-09-15T14:40:13Z

Source: https://github.com/Jordan-Hall/browser/issues/62#issuecomment-5682189612 | Updated: 2026-09-15T14:40:13Z

<!-- intent-implementation-v1:LOCAL-02 -->
###### Implementation proposal — LOCAL-02

Create signed, versioned role-specific execution packs over #61/#8. A pack includes weights, tokenizer, template, runtime compatibility, license, quantization/context limits and measured hardware results—not just a model filename.

- [ ] **LOCAL-02.T01 — Manifest and role schemas.** Define coding/planning, extraction/UI planning, embeddings/reranking and visual-grounding roles, with hashes for every execution-relevant asset. **Verify:** template/tokenizer changes create a distinct versioned pack.
- [ ] **LOCAL-02.T02 — Candidate evaluation.** Run fixed-budget product fixtures per role/hardware cohort; measure verified quality, latency, memory and available energy metrics. **Verify:** defaults follow repeatable task evidence rather than an assumed leaderboard winner.
- [ ] **LOCAL-02.T03 — Verified resumable installation.** Obtain download/storage consent, quarantine chunks, check size/disk capacity, verify hashes/signatures and activate atomically. **Verify:** corrupt or incomplete downloads cannot load; untrusted repository code is never executed by the loader.
- [ ] **LOCAL-02.T04 — Compatibility and storage.** Detect supported backends/accelerators and account for complete weights, context/KV cache and application overhead. Reference-count active packs and offer qualified smaller alternatives. **Verify:** cleanup cannot remove a live required pack or cause reload loops.
- [ ] **LOCAL-02.T05 — Update/rollback/revocation.** Stage versions, run regressions, preserve a compatible prior pack and support signed revocation. **Verify:** updates cannot silently change privacy mode, selected model or data destinations.
- [ ] **LOCAL-02.T06 — Role-aware routing.** Schedule lightweight and heavy roles under #4 admission, sharing residency only when beneficial. **Verify:** unavailable offline roles are visible and background coding cannot starve speech/control.
- [ ] **LOCAL-02.T07 — Publish support evidence.** Export OS/accelerator/runtime/pack/quantization/context/workload results with limitations and license/download information in model settings. **Verify:** a support designation is traceable to actual evaluation artifacts.

**Review boundaries:** activated parameters are not installed weight memory; signatures establish provenance rather than safe behavior; downloaded model formats are untrusted parser inputs. Keep large downloads opt-in and choose model defaults only after current runtime/license and product-task review.

#### Comment 5686980602 — Jordan-Hall — 2026-09-15T19:35:00Z

Source: https://github.com/Jordan-Hall/browser/issues/62#issuecomment-5686980602 | Updated: 2026-09-15T19:35:00Z

###### Task issues
- [ ] #494 `LOCAL-02.T01` — Define role and manifest schemas
- [ ] #495 `LOCAL-02.T02` — Build candidate evaluation and selection
- [ ] #496 `LOCAL-02.T03` — Implement verified resumable installation
- [ ] #497 `LOCAL-02.T04` — Implement compatibility and storage management
- [ ] #498 `LOCAL-02.T05` — Implement update, rollback and revocation
- [ ] #499 `LOCAL-02.T06` — Integrate role-aware scheduling
- [ ] #500 `LOCAL-02.T07` — Publish support and regression evidence


---

<a id="issue-63"></a>
## #63 — [P1][LOCAL-03] Native local coding and tool agent

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/63
**Created:** 2026-09-15T12:14:18Z | **Updated:** 2026-09-15T19:35:53Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #21

#### Objective
Deliver a built-in local agent that can perform useful coding and bounded computer tasks entirely through our harness/tool broker, including fully offline coding workflows.

#### Scope
- Native `AgentSession` implementation using LOCAL-01 inference worker.
- Repository search/navigation, scoped file reads, patch generation and structured edits.
- Brokered compile/test/lint/format/process tools in isolated workspaces.
- Failure inspection and bounded repair loop with budgets/stopping criteria.
- UI/workflow composition using trusted schemas.
- Evidence/artifact publication and deterministic verification hooks.
- Tool policy tuned for offline operation and local reference material.

#### Security/product rules
- Every tool call routes through our broker; local model has no ambient shell/filesystem/network authority.
- Offline mode must remain truly network-denied.
- Model-generated changes to policy/supervisor/security repos require ordinary reviewed development flow.

#### Acceptance criteria
- [ ] Declared offline coding fixture produces a reviewable patch and passes independent tests without network/cloud use.
- [ ] Agent cannot read/write files outside its granted project scope.
- [ ] Tool failures are surfaced and bounded repair stops at budget/deadline.
- [ ] Generated dependency/network requests require explicit applicable capabilities.
- [ ] Artifacts and verification survive inference-worker restart.
- [ ] Performance is reported as time-to-verified-useful-change, not token throughput alone.

#### Dependencies
- LOCAL-01
- AGENT-01
- SEC-04

**First phase:** P1  
**Maturity target:** P3  
**Owner:** local-ai-speech

### Discussion (2 comments)

#### Comment 5682197543 — Jordan-Hall — 2026-09-15T14:40:39Z

Source: https://github.com/Jordan-Hall/browser/issues/63#issuecomment-5682197543 | Updated: 2026-09-15T14:40:39Z

<!-- intent-implementation-v1:LOCAL-03 -->
###### Implementation proposal — LOCAL-03

Build the native local AgentSession from #61/#54/#9 with broker-controlled tools. The supervisor owns goals, grants and budgets; the model proposes bounded next steps.

- [ ] **LOCAL-03.T01 — Session and planner loop.** Connect inference to durable task nodes, validate structured tool proposals and bound turns. **Verify:** trusted constraints survive model context compaction and inference-worker restart.
- [ ] **LOCAL-03.T02 — Repository retrieval.** Select scoped paths/symbols/text/tests/docs using exact search and optional local embeddings; include base/file revisions. **Verify:** unrelated files are not loaded and context remains within token/access budgets.
- [ ] **LOCAL-03.T03 — Revision-bound edits.** Produce narrow patches against file hashes/base commits; validate paths and syntax where available. **Verify:** stale bases create conflicts instead of silently overwriting user changes.
- [ ] **LOCAL-03.T04 — Isolated tools.** Integrate compile/test/lint/format/process tools from #72/#73 with explicit filesystem/dependency/network policy. **Verify:** offline runs use available local dependencies or request permission rather than secretly downloading them.
- [ ] **LOCAL-03.T05 — Bounded repair and verification.** Feed structured failures back under fixed attempt/time limits; stop on repeated no-progress or required scope expansion. **Verify:** protected acceptance tests independently judge success and are not replaced by agent-authored tests.
- [ ] **LOCAL-03.T06 — UI/workflow and computer tasks.** Generate validated GoalContract/UI IR and scoped file/desktop requests using the same broker and postconditions. **Verify:** a local model cannot bypass authorization simply because data stays local.
- [ ] **LOCAL-03.T07 — Hardware-specific qualification.** Run repeated offline coding tasks with hidden tests, scope probes and concurrent browser/speech load. **Verify:** publish verified success/interventions and time-to-useful-change by pack/hardware, with truthful limitations.

**Definition of done:** a local agent can inspect an approved repository, produce a reviewable patch, execute isolated tests, repair within bounds and publish verified evidence with networking disabled. Preserve all required coding capability, but never equate locality with safe unrestricted shell access or frontier-quality performance on every device.

#### Comment 5686991496 — Jordan-Hall — 2026-09-15T19:35:53Z

Source: https://github.com/Jordan-Hall/browser/issues/63#issuecomment-5686991496 | Updated: 2026-09-15T19:35:53Z

###### Task issues
- [ ] #501 `LOCAL-03.T01` — Implement local AgentSession and planner loop
- [ ] #502 `LOCAL-03.T02` — Build repository retrieval and code context
- [ ] #503 `LOCAL-03.T03` — Implement structured edits and patch artifacts
- [ ] #504 `LOCAL-03.T04` — Integrate isolated build, test and diagnostics tools
- [ ] #505 `LOCAL-03.T05` — Implement bounded repair and verification
- [ ] #506 `LOCAL-03.T06` — Add UI/workflow composition and scoped tool tasks
- [ ] #507 `LOCAL-03.T07` — Qualify local coding on declared hardware


---

<a id="issue-64"></a>
## #64 — [P0][LOCAL-04] Hardware/resource profiling and optimization

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/64
**Created:** 2026-09-15T12:14:27Z | **Updated:** 2026-09-15T19:36:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #21

#### Objective
Make local AI usable across realistic laptops/workstations through measured hardware detection, model/context selection and cooperative resource scheduling.

#### Scope
- Detect CPU, RAM, accelerator type, usable VRAM/shared memory and storage.
- Hardware cohorts and recommended model/quantization/context profiles.
- Runtime telemetry: cold/warm start, TTFT, task latency, memory, power, thermal throttling and browser/speech responsiveness.
- Explicit cache eviction and model residency policy.
- Battery/thermal-aware background scheduling.
- GPU/CPU priority coordination between coding, speech, browser rendering and UI.
- OOM prediction/recovery and smaller-pack fallback suggestions.

#### Product rules
- Installed RAM does not equal usable accelerator memory.
- Speech/stop/direct interaction take priority over optional background reasoning.
- Product support claims are tied to declared reference hardware measurements.

#### Acceptance criteria
- [ ] Reference hardware is classified reproducibly and selects a supported profile.
- [ ] Speech and deterministic UI remain usable during declared coding workloads.
- [ ] OOM/thermal pressure degrades gracefully without crash loops or hidden cloud fallback.
- [ ] Cache/model eviction is explicit and bounded.
- [ ] Performance dashboard separates model latency from deterministic UI/connector latency.
- [ ] Battery/thermal policies are testable and user-overridable within safe bounds.

#### Dependencies
- LOCAL-01
- CORE-03

**First phase:** P0  
**Maturity target:** P5  
**Owner:** local-ai-speech

### Discussion (2 comments)

#### Comment 5682204424 — Jordan-Hall — 2026-09-15T14:41:01Z

Source: https://github.com/Jordan-Hall/browser/issues/64#issuecomment-5682204424 | Updated: 2026-09-15T14:41:01Z

<!-- intent-implementation-v1:LOCAL-04 -->
###### Implementation proposal — LOCAL-04

Build privacy-minimized hardware/resource profiling coordinated with #61/#4. HardwareProfile, ResidencyPlan, PressureEvent and PerformanceSample must distinguish measured, estimated and unavailable values.

- [ ] **LOCAL-04.T01 — Hardware/backend detection.** Enumerate CPU, RAM, accelerator support, usable memory and storage; record driver/backend versions without unnecessary device identifiers. **Verify:** unsupported or unknown accelerators are not silently treated as supported GPUs.
- [ ] **LOCAL-04.T02 — Workload instrumentation.** Measure cold/warm load, first useful output, complete task duration, peak memory and available utilization/thermal/energy data. **Verify:** deterministic UI, retrieval and inference timing remain separately attributable.
- [ ] **LOCAL-04.T03 — Cohorts/admission.** Establish compact/developer/workstation profiles using measured packs, context/KV cache and browser/speech headroom. **Verify:** predicted admission is checked against actual peaks and uncertainty margins.
- [ ] **LOCAL-04.T04 — Residency/eviction/priority.** Coordinate load/unload, cache eviction and background yield; reserve foreground speech/control capacity rather than loading every role simultaneously. **Verify:** pressure produces explicit queue/wait states and bounded memory growth.
- [ ] **LOCAL-04.T05 — OOM/thermal/battery behavior.** Stop optional work, release caches and offer smaller contexts/packs without changing privacy mode. **Verify:** sustained load degrades visibly instead of crash-looping or silently moving inference to cloud.
- [ ] **LOCAL-04.T06 — Diagnostics/support guidance.** Show current residency, queue pressure and qualified choices; provide redacted hardware/version exports. **Verify:** diagnostics contain no prompts, private paths or unique identifiers beyond explicit consent.
- [ ] **LOCAL-04.T07 — Mixed-load qualification.** Run browsing/video, speech, coding, indexing and desktop control together across reference configurations. **Verify:** publish distributions and sustained thermal results, not only short warm benchmarks.

**Dependency correction:** hardware detection can start in P0 before full inference support; measured admission integrates later. Distinguish hard OS limits, estimates and cooperative GPU scheduling—do not promise universal accelerator preemption or treat system RAM as freely available VRAM. Proposed latency/energy targets become claims only after measurement.

#### Comment 5687002430 — Jordan-Hall — 2026-09-15T19:36:41Z

Source: https://github.com/Jordan-Hall/browser/issues/64#issuecomment-5687002430 | Updated: 2026-09-15T19:36:41Z

###### Task issues
- [ ] #508 `LOCAL-04.T01` — Implement hardware and backend detection
- [ ] #509 `LOCAL-04.T02` — Build workload measurement instrumentation
- [ ] #510 `LOCAL-04.T03` — Define hardware cohorts and admission estimates
- [ ] #511 `LOCAL-04.T04` — Implement residency, eviction and cooperative priority
- [ ] #512 `LOCAL-04.T05` — Implement OOM and thermal/battery degradation
- [ ] #513 `LOCAL-04.T06` — Publish performance diagnostics and support guidance
- [ ] #514 `LOCAL-04.T07` — Run mixed-workload and regression qualification


---

<a id="issue-243"></a>
## #243 — [TASK][EPIC-LOCAL.T01] Define local capability and hardware profiles

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/243
**Created:** 2026-09-15T15:27:46Z | **Updated:** 2026-09-15T15:27:46Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #21

### Original description

Parent: #21

Task ID: `EPIC-LOCAL.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-244"></a>
## #244 — [TASK][EPIC-LOCAL.T02] Integrate inference modes and signed model packs

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/244
**Created:** 2026-09-15T15:27:52Z | **Updated:** 2026-09-15T15:27:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #21

### Original description

Parent: #21

Task ID: `EPIC-LOCAL.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-245"></a>
## #245 — [TASK][EPIC-LOCAL.T03] Integrate the brokered local agent and speech scheduling

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/245
**Created:** 2026-09-15T15:27:59Z | **Updated:** 2026-09-15T15:27:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #21

### Original description

Parent: #21

Task ID: `EPIC-LOCAL.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-246"></a>
## #246 — [TASK][EPIC-LOCAL.T04] Qualify quality, energy and graceful degradation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/246
**Created:** 2026-09-15T15:28:04Z | **Updated:** 2026-09-15T15:28:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #21

### Original description

Parent: #21

Task ID: `EPIC-LOCAL.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-487"></a>
## #487 — [TASK][LOCAL-01.T01] Define inference contracts and mode policy

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/487
**Created:** 2026-09-15T19:33:22Z | **Updated:** 2026-09-15T19:33:22Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #61

### Original description

Parent: #61

Task ID: `LOCAL-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-488"></a>
## #488 — [TASK][LOCAL-01.T02] Implement the first supervised local runtime

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/488
**Created:** 2026-09-15T19:33:30Z | **Updated:** 2026-09-15T19:33:30Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #61

### Original description

Parent: #61

Task ID: `LOCAL-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-489"></a>
## #489 — [TASK][LOCAL-01.T03] Enforce offline and destination restrictions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/489
**Created:** 2026-09-15T19:33:34Z | **Updated:** 2026-09-15T19:33:34Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #61

### Original description

Parent: #61

Task ID: `LOCAL-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-490"></a>
## #490 — [TASK][LOCAL-01.T04] Implement structured output and tool-call proposals

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/490
**Created:** 2026-09-15T19:33:41Z | **Updated:** 2026-09-15T19:33:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #61

### Original description

Parent: #61

Task ID: `LOCAL-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-491"></a>
## #491 — [TASK][LOCAL-01.T05] Implement cancellation and model lifecycle

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/491
**Created:** 2026-09-15T19:33:46Z | **Updated:** 2026-09-15T19:33:46Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #61

### Original description

Parent: #61

Task ID: `LOCAL-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-492"></a>
## #492 — [TASK][LOCAL-01.T06] Implement explicit escalation and useful degradation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/492
**Created:** 2026-09-15T19:33:51Z | **Updated:** 2026-09-15T19:33:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #61

### Original description

Parent: #61

Task ID: `LOCAL-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-493"></a>
## #493 — [TASK][LOCAL-01.T07] Qualify inference policy across failure modes

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/493
**Created:** 2026-09-15T19:33:56Z | **Updated:** 2026-09-15T19:33:56Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #61

### Original description

Parent: #61

Task ID: `LOCAL-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-494"></a>
## #494 — [TASK][LOCAL-02.T01] Define role and manifest schemas

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/494
**Created:** 2026-09-15T19:34:10Z | **Updated:** 2026-09-15T19:34:10Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #62

### Original description

Parent: #62

Task ID: `LOCAL-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-495"></a>
## #495 — [TASK][LOCAL-02.T02] Build candidate evaluation and selection

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/495
**Created:** 2026-09-15T19:34:16Z | **Updated:** 2026-09-15T19:34:16Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #62

### Original description

Parent: #62

Task ID: `LOCAL-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-496"></a>
## #496 — [TASK][LOCAL-02.T03] Implement verified resumable installation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/496
**Created:** 2026-09-15T19:34:24Z | **Updated:** 2026-09-15T19:34:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #62

### Original description

Parent: #62

Task ID: `LOCAL-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-497"></a>
## #497 — [TASK][LOCAL-02.T04] Implement compatibility and storage management

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/497
**Created:** 2026-09-15T19:34:32Z | **Updated:** 2026-09-15T19:34:32Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #62

### Original description

Parent: #62

Task ID: `LOCAL-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-498"></a>
## #498 — [TASK][LOCAL-02.T05] Implement update, rollback and revocation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/498
**Created:** 2026-09-15T19:34:38Z | **Updated:** 2026-09-15T19:34:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #62

### Original description

Parent: #62

Task ID: `LOCAL-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-499"></a>
## #499 — [TASK][LOCAL-02.T06] Integrate role-aware scheduling

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/499
**Created:** 2026-09-15T19:34:43Z | **Updated:** 2026-09-15T19:34:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #62

### Original description

Parent: #62

Task ID: `LOCAL-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-500"></a>
## #500 — [TASK][LOCAL-02.T07] Publish support and regression evidence

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/500
**Created:** 2026-09-15T19:34:51Z | **Updated:** 2026-09-15T19:34:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #62

### Original description

Parent: #62

Task ID: `LOCAL-02.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-501"></a>
## #501 — [TASK][LOCAL-03.T01] Implement local AgentSession and planner loop

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/501
**Created:** 2026-09-15T19:35:06Z | **Updated:** 2026-09-15T19:35:06Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #63

### Original description

Parent: #63

Task ID: `LOCAL-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-502"></a>
## #502 — [TASK][LOCAL-03.T02] Build repository retrieval and code context

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/502
**Created:** 2026-09-15T19:35:14Z | **Updated:** 2026-09-15T19:35:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #63

### Original description

Parent: #63

Task ID: `LOCAL-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-503"></a>
## #503 — [TASK][LOCAL-03.T03] Implement structured edits and patch artifacts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/503
**Created:** 2026-09-15T19:35:20Z | **Updated:** 2026-09-15T19:35:20Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #63

### Original description

Parent: #63

Task ID: `LOCAL-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-504"></a>
## #504 — [TASK][LOCAL-03.T04] Integrate isolated build, test and diagnostics tools

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/504
**Created:** 2026-09-15T19:35:26Z | **Updated:** 2026-09-15T19:35:26Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #63

### Original description

Parent: #63

Task ID: `LOCAL-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-505"></a>
## #505 — [TASK][LOCAL-03.T05] Implement bounded repair and verification

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/505
**Created:** 2026-09-15T19:35:32Z | **Updated:** 2026-09-15T19:35:32Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #63

### Original description

Parent: #63

Task ID: `LOCAL-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-506"></a>
## #506 — [TASK][LOCAL-03.T06] Add UI/workflow composition and scoped tool tasks

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/506
**Created:** 2026-09-15T19:35:38Z | **Updated:** 2026-09-15T19:35:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #63

### Original description

Parent: #63

Task ID: `LOCAL-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-507"></a>
## #507 — [TASK][LOCAL-03.T07] Qualify local coding on declared hardware

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/507
**Created:** 2026-09-15T19:35:47Z | **Updated:** 2026-09-15T19:35:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #63

### Original description

Parent: #63

Task ID: `LOCAL-03.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-508"></a>
## #508 — [TASK][LOCAL-04.T01] Implement hardware and backend detection

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/508
**Created:** 2026-09-15T19:36:00Z | **Updated:** 2026-09-15T19:36:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #64

### Original description

Parent: #64

Task ID: `LOCAL-04.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-509"></a>
## #509 — [TASK][LOCAL-04.T02] Build workload measurement instrumentation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/509
**Created:** 2026-09-15T19:36:07Z | **Updated:** 2026-09-15T19:36:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #64

### Original description

Parent: #64

Task ID: `LOCAL-04.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-510"></a>
## #510 — [TASK][LOCAL-04.T03] Define hardware cohorts and admission estimates

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/510
**Created:** 2026-09-15T19:36:12Z | **Updated:** 2026-09-15T19:36:12Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #64

### Original description

Parent: #64

Task ID: `LOCAL-04.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-511"></a>
## #511 — [TASK][LOCAL-04.T04] Implement residency, eviction and cooperative priority

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/511
**Created:** 2026-09-15T19:36:17Z | **Updated:** 2026-09-15T19:36:17Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #64

### Original description

Parent: #64

Task ID: `LOCAL-04.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-512"></a>
## #512 — [TASK][LOCAL-04.T05] Implement OOM and thermal/battery degradation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/512
**Created:** 2026-09-15T19:36:23Z | **Updated:** 2026-09-15T19:36:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #64

### Original description

Parent: #64

Task ID: `LOCAL-04.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-513"></a>
## #513 — [TASK][LOCAL-04.T06] Publish performance diagnostics and support guidance

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/513
**Created:** 2026-09-15T19:36:30Z | **Updated:** 2026-09-15T19:36:30Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #64

### Original description

Parent: #64

Task ID: `LOCAL-04.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-514"></a>
## #514 — [TASK][LOCAL-04.T07] Run mixed-workload and regression qualification

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/514
**Created:** 2026-09-15T19:36:35Z | **Updated:** 2026-09-15T19:36:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #64

### Original description

Parent: #64

Task ID: `LOCAL-04.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

