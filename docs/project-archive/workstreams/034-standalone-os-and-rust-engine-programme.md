# Standalone OS and Rust-engine programme

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 11

## Contents

- [#34 — EPIC: Standalone OS and Rust-engine programme](#issue-34)
- [#105 — [P5][OS-01] Dedicated session and immutable distribution](#issue-105)
- [#106 — [P0][OS-02] Servo and alternative-engine compatibility track](#issue-106)
- [#107 — [K0][OS-03] Rust-kernel architecture and reuse assessment](#issue-107)
- [#108 — [K1][OS-04] Virtualized Rust-kernel substrate](#issue-108)
- [#109 — [K2][OS-05] Desktop, speech and inference OS ports](#issue-109)
- [#110 — [K3-K4][OS-06] Compatibility, hardware and supported Rust OS release](#issue-110)
- [#295 — [TASK][EPIC-OS.T01] Ratify separate engine, distribution and kernel roadmaps](#issue-295)
- [#296 — [TASK][EPIC-OS.T02] Integrate the dedicated session and engine experiments](#issue-296)
- [#297 — [TASK][EPIC-OS.T03] Coordinate K0-K2 substrate and workload ports](#issue-297)
- [#298 — [TASK][EPIC-OS.T04] Plan K3-K4 hardware and supported release](#issue-298)

---

<a id="issue-34"></a>
## #34 — EPIC: Standalone OS and Rust-engine programme

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/34
**Created:** 2026-09-15T12:09:09Z | **Updated:** 2026-09-15T14:27:21Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1
**Native child issues:** #105, #106, #107, #108, #109, #110

### Original description

Programme: #1

Own the dedicated desktop session/distribution path, alternative Rust engine track and separately staffed Rust-kernel research/delivery programme.

#### Child issues
- [ ] #105 OS-01 — Dedicated session and immutable distribution
- [ ] #106 OS-02 — Servo and alternative-engine compatibility track
- [ ] #107 OS-03 — Rust-kernel architecture and reuse assessment
- [ ] #108 OS-04 — Virtualized Rust-kernel substrate
- [ ] #109 OS-05 — Desktop, speech and inference OS ports
- [ ] #110 OS-06 — Compatibility, hardware and supported Rust OS release

#### Cross-cutting gates
User-space runtime remains portable, inference stays unprivileged, Chromium fallback stays truthful until alternatives pass compatibility, kernel work has explicit hardware/ABI/driver/browser/inference plans, and OS failure recovery works with inference disabled.

### Discussion (1 comments)

#### Comment 5681948543 — Jordan-Hall — 2026-09-15T14:27:21Z

Source: https://github.com/Jordan-Hall/browser/issues/34#issuecomment-5681948543 | Updated: 2026-09-15T14:27:21Z

<!-- intent-implementation-v1:EPIC-OS -->
###### Workstream implementation and integration tasks

Keep #105–#110 fully in scope as three related but distinct tracks: user-space OS distribution, Rust rendering-engine compatibility, and Rust-kernel delivery.

- [ ] **EPIC-OS.T01 — Ratify separate roadmaps.** Define reference hardware, reuse alternatives, ABI/driver dependencies, security boundaries and staffing for each track. **Proof:** a Linux-based distribution is not mislabeled a Rust kernel and browser delivery does not wait for kernel research.
- [ ] **EPIC-OS.T02 — Integrate session/engine experiments.** Package the existing runtime as a dedicated session with model-independent recovery; evaluate Servo behind the browser-engine contract. **Proof:** tested engine fallback and desktop workflows preserve workspace/account state.
- [ ] **EPIC-OS.T03 — Integrate K0–K2 substrate/ports.** Complete the kernel architecture decision, virtualized memory/process/IPC substrate and actual UI/audio/storage/network/speech/inference ports. **Proof:** isolated unprivileged workloads run; boot alone is not the milestone.
- [ ] **EPIC-OS.T04 — Qualify K3–K4 hardware/release.** Implement the chosen browser/toolchain compatibility path, physical drivers, power, encryption, accessibility, signed updates and recovery. **Proof:** the full declared product/hardware conformance suite passes with maintenance ownership.

**Review boundaries:** Rust source language does not supply an ABI, standard library, GPU driver or accelerator runtime automatically. CPU fallback/minimal test clients can validate early K2; full browser/accelerator compatibility still needs explicit K3/K4 evidence. AI reasoning and generated UI remain unprivileged user-space work.

**Closure:** supported hardware, workload and recovery evidence plus a patch/driver maintenance plan—not a one-off boot image or an unmeasured compatibility claim.


---

<a id="issue-105"></a>
## #105 — [P5][OS-01] Dedicated session and immutable distribution

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/105
**Created:** 2026-09-15T12:22:49Z | **Updated:** 2026-09-15T14:56:31Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #34 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #34

#### Objective
Package the same user-space personal runtime as a dedicated desktop session and then a recoverable Rust-heavy operating-system distribution without making inference a privileged boot dependency.

#### Scope
- Session/login/unlock integration and workspace-oriented shell startup.
- Window/application launching, file browsing, clipboard, notifications and background task runtime.
- Audio/microphone, networking, storage, display/input and power/session integration.
- Resource service coordinating browser, speech and local model workloads.
- Transactional/immutable system update strategy with A/B or equivalent rollback/recovery.
- Diagnostics/recovery mode independent of AI models.
- Secure credential/key storage integration and user data partitioning.
- Installer/image creation, reference hardware support matrix and migration from desktop-app profile.

#### Architecture rules
- Supervisor, connectors, model workers and generated applications remain unprivileged user-space services.
- A broken/missing model cannot prevent login, files, network repair or OS rollback.
- OS distribution reuses the same contracts/workspaces rather than creating a second incompatible product state model.

#### Acceptance criteria
- [ ] Reference hardware boots into the personal runtime and completes declared browser/workspace/file/voice workflows.
- [ ] Login, local files, network diagnostics and rollback work with inference entirely disabled/broken.
- [ ] Interrupted system update recovers transactionally without corrupting the personal workspace store.
- [ ] Desktop-app workspace export/migration is accepted by the OS runtime without losing declared portable state.
- [ ] Privileged OS components remain materially smaller than the AI/browser application stack.
- [ ] Hardware/driver limitations are documented in a tested support matrix.

#### Dependencies
- CORE-03
- EVAL-04
- PC-03

**First phase:** P5  
**Maturity target:** P8  
**Owner:** os-systems

### Discussion (1 comments)

#### Comment 5682498308 — Jordan-Hall — 2026-09-15T14:56:31Z

Source: https://github.com/Jordan-Hall/browser/issues/105#issuecomment-5682498308 | Updated: 2026-09-15T14:56:31Z

<!-- intent-implementation-v1:OS-01 -->
###### Implementation proposal — OS-01

Package the same user-space runtime as a dedicated desktop session and recoverable distribution, integrating #4/#104/#70. This is distinct from the Rust-kernel programme; AI remains unprivileged.

- [ ] **OS-01.T01 — Base/reference hardware.** Compare graphics/audio/networking, secure updates, packages/toolchains and browser/model support; declare exact devices and recovery assumptions. **Verify:** selected hardware runs the actual dependencies rather than only booting a minimal image.
- [ ] **OS-01.T02 — Dedicated session.** Start trusted shell/supervisor in a normal user session with application/window launch, clipboard, notifications and lock. **Verify:** conventional apps and generated workspaces coexist without giving agents ambient session authority.
- [ ] **OS-01.T03 — Device/settings brokers.** Expose narrow audio/microphone/network/display/storage/power operations via small privileged services where required. **Verify:** OS permission and per-task grant remain distinct and models cannot self-elevate.
- [ ] **OS-01.T04 — Resource/background services.** Coordinate browser/speech/inference and durable jobs through lock/suspend transitions. **Verify:** foreground control remains usable and stale work revalidates after resume.
- [ ] **OS-01.T05 — Immutable images/updates.** Build versioned signed images with separate user data, staged boot activation and known-good recovery tied to schema/security compatibility. **Verify:** failed update cannot strand the user or silently downgrade security.
- [ ] **OS-01.T06 — Model-independent recovery.** Provide network repair, storage diagnostics, safe mode, key recovery and rollback without inference. **Verify:** missing/broken weights cannot prevent login or access to recovery tools.
- [ ] **OS-01.T07 — Desktop-profile migration.** Import compatible workspaces/data while renewing grants/reauthenticating credentials; preserve layouts/evidence/task history and reconcile old effects. **Verify:** migration does not replay external actions or transfer blanket approval.
- [ ] **OS-01.T08 — Hardware/session qualification.** Run real browser/voice/coding/file/control workflows, suspend/resume and update-failure tests. **Verify:** publish supported devices, limitations and maintenance owners.

**Review distinction:** a Linux-based Rust-heavy distribution is not a Rust kernel. Reuse mature drivers/boot/audio/networking initially and keep build/product metadata honest. Distribution progress must preserve the same workspace and policy contracts instead of creating an incompatible second product.


---

<a id="issue-106"></a>
## #106 — [P0][OS-02] Servo and alternative-engine compatibility track

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/106
**Created:** 2026-09-15T12:23:00Z | **Updated:** 2026-09-15T14:56:46Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #34 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #34

#### Objective
Maintain a real Rust-native rendering-engine path behind the browser-engine abstraction without sacrificing website compatibility or security for language purity.

#### Scope
- Engine contract for navigation, rendering surfaces, source/origin identity, observations, input/focus and feature capability reporting.
- Integrate Servo or alternative Rust engine into isolated test/application workloads.
- Compatibility corpus covering public pages, forms, authentication handoff, accessibility, CSS/layout, media and common application patterns.
- Security/process isolation assessment and update cadence.
- Performance/resource comparison against Chromium/CEF on source-backed workspace/browser scenarios.
- Per-origin/workload engine-selection experiment only after compatibility/safety evidence.
- Explicit Chromium fallback and user/support diagnostics.

#### Product rules
- Alternative engine is enabled only where measured compatibility meets the declared workload gate.
- Original-view authenticity/functional requirements take precedence over Rust implementation purity.
- Engine differences remain visible to diagnostics/evaluation.

#### Acceptance criteria
- [ ] Browser-engine abstraction supports Chromium and at least one experimental alternative without workspace/data contract changes.
- [ ] Compatibility results are reproducible against EVAL-01 corpus.
- [ ] Unsupported workloads automatically/usefully fall back to the supported engine rather than silently breaking.
- [ ] Accessibility, focus/input and origin identity are independently evaluated.
- [ ] Security/update ownership is defined before any production enablement.
- [ ] No product claim presents experimental engine coverage as universal web compatibility.

#### Dependencies
- WEB-01
- WEB-04
- EVAL-01

**First phase:** P0  
**Maturity target:** P8 / opportunistic  
**Owner:** os-systems

### Discussion (1 comments)

#### Comment 5682502935 — Jordan-Hall — 2026-09-15T14:56:46Z

Source: https://github.com/Jordan-Hall/browser/issues/106#issuecomment-5682502935 | Updated: 2026-09-15T14:56:46Z

<!-- intent-implementation-v1:OS-02 -->
###### Implementation proposal — OS-02

Maintain the Rust-engine track behind #11/#36 contracts and #101 compatibility fixtures. Language choice is not a substitute for measured browser behavior or isolation.

- [ ] **OS-02.T01 — Minimum engine contract.** Extract navigation, surfaces, input/focus, origin/permissions and observation interfaces without CEF handles; declare optional capabilities and errors. **Verify:** workspace/data code can use adapters without engine-specific dependencies.
- [ ] **OS-02.T02 — Isolated Servo worker.** Pin a documented embedding build and implement lifecycle/render/input while retaining process/egress boundaries. **Verify:** failed or malformed content cannot escape to trusted runtime APIs.
- [ ] **OS-02.T03 — Compatibility corpus.** Test controlled and real workload pages for layout/forms/scripts/media/accessibility, extraction and authentication handoff. **Verify:** results identify exact engine/build/OS and unsupported features rather than claiming universal parity.
- [ ] **OS-02.T04 — Performance/resource comparison.** Measure navigation, memory, CPU/GPU and interaction with speech/inference on identical hardware/workloads. **Verify:** benefits and regressions use the same methodology, not selected demonstrations.
- [ ] **OS-02.T05 — Selection/fallback.** Enable only qualified workloads/origins, disclose diagnostics and preserve workspace continuity when using the supported engine. **Verify:** unsupported session portability triggers proper reauthentication instead of opaque cookie-database copying.
- [ ] **OS-02.T06 — Security/update qualification.** Review unsafe boundaries, fuzz interfaces/parsers, name patch owners and run held-out compatibility before a production cohort. **Verify:** experimental coverage remains visibly experimental until its gates pass.

**Reference:** [Servo](https://servo.org/). The experiment can begin with a narrow adapter before Original-view work is fully mature. Cross-engine account/session transfer is a separate security feature, not an automatic consequence of sharing a browser interface. Do not silently replay forms or consequential navigation during engine fallback.


---

<a id="issue-107"></a>
## #107 — [K0][OS-03] Rust-kernel architecture and reuse assessment

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/107
**Created:** 2026-09-15T12:23:13Z | **Updated:** 2026-09-15T14:57:21Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #34 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #34

#### Objective
Define the standalone Rust-kernel programme based on concrete product/isolation requirements and realistic browser/GPU/inference/application compatibility, not on language preference alone.

#### Scope
- Select initial CPU/firmware/reference hardware targets.
- Compare routes: new Rust kernel, Redox reuse/contribution, Linux/Rust-heavy distribution, Asterinas/other compatibility-oriented approaches where technically relevant.
- Choose/evaluate kernel architecture: capabilities, process model, address spaces, IPC, scheduler, filesystem/network abstractions and driver model.
- Define user/kernel ABI and compatibility strategy for required browser/model/toolchains.
- Inventory graphics, storage, networking, USB, audio, input, power and accelerator dependencies.
- Define secure boot/update/recovery requirements and threat boundary.
- Keep AI inference/UI generation unprivileged user-space work.
- Produce staffing/funding/maintenance plan separate from browser runtime delivery.

#### Decision rules
- Kernel work must solve a named systems/security/product requirement or advance an intentional OS research programme; it is not the next UI milestone.
- Reuse is preferred where it materially reduces driver/browser/inference burden without violating target architecture.
- Unknown compatibility work is explicitly costed/risked rather than assumed away.

#### Acceptance criteria
- [ ] Architecture decision names target CPU/firmware/reference hardware and required device classes.
- [ ] Browser/rendering and local-inference compatibility paths are explicit, including required ABI/runtime/toolchain work.
- [ ] Kernel/user-space privilege boundary keeps model reasoning and generated UI outside kernel privilege.
- [ ] Driver, GPU, power, accessibility and application-compatibility gaps are listed with owners/experiments.
- [ ] Reuse/new-kernel alternatives are compared against measurable criteria and documented tradeoffs.
- [ ] K1 entry criteria, separately staffed ownership and long-term security/update maintenance obligations are approved.

#### Dependencies
- CORE-01
- SEC-01

**First phase:** K0  
**Maturity target:** K0 decision gate  
**Owner:** os-systems

### Discussion (1 comments)

#### Comment 5682511773 — Jordan-Hall — 2026-09-15T14:57:21Z

Source: https://github.com/Jordan-Hall/browser/issues/107#issuecomment-5682511773 | Updated: 2026-09-15T14:57:21Z

<!-- intent-implementation-v1:OS-03 -->
###### Implementation proposal — OS-03 / K0

Run a separately owned kernel architecture programme alongside #2/#6. Compare alternatives against concrete product/research requirements, retaining the full new-kernel option without assuming browser/runtime dependencies disappear.

- [ ] **OS-03.T01 — Requirements/hypotheses.** Define capability isolation, recoverability, resource control and hardware goals; distinguish mandatory behavior from research hypotheses/language preference. **Verify:** each kernel decision has a measurable objective and rejection criterion.
- [ ] **OS-03.T02 — CPU/firmware/VM targets.** Choose one initial architecture/boot path and reproducible virtual hardware, then candidate physical devices and debug/recovery access. **Verify:** K1 has a concrete test target, not an unspecified universal platform.
- [ ] **OS-03.T03 — Reuse/compatibility comparison.** Evaluate Redox, Linux-ABI-oriented approaches such as Asterinas, a new kernel and the Rust-heavy Linux distribution using comparable spikes/evidence, licenses and maintenance costs. **Verify:** alternatives remain distinct rather than being treated as interchangeable components.
- [ ] **OS-03.T04 — Kernel objects/authority.** Specify processes/address spaces, capability transfer/revocation, IPC, scheduling, memory ownership and driver isolation. **Verify:** browser parsing, model inference and generated UI remain unprivileged user-space workloads.
- [ ] **OS-03.T05 — Port dependency inventory.** Trace libc/stdlib/threading/time/files/network/graphics/audio/sandbox/accelerator requirements for actual browser/speech/model runtimes. **Verify:** every requirement has a native-port, compatibility-layer or controlled-VM route and an owner.
- [ ] **OS-03.T06 — Boot/update/recovery trust.** Define signing/key custody, encrypted storage, rollback floors, crash diagnostics and non-model recovery. **Verify:** a failed inference component cannot prevent repair or rollback.
- [ ] **OS-03.T07 — Staffing/stages/entry evidence.** Assign systems/driver/security maintenance, K1–K4 gates and sustainable patch obligations separately from desktop delivery. **Verify:** K1 starts with approved architecture/targets and known compatibility risks, not just a language goal.

**References for evaluation:** [Redox](https://github.com/redox-os/redox), [Asterinas](https://github.com/asterinas/asterinas), [Linux Rust support](https://docs.kernel.org/rust/index.html). These are candidate reuse/reference directions, not tested compatibility claims. A new kernel can be chosen deliberately; driver, ABI, GPU and application obligations still require explicit implementation evidence.


---

<a id="issue-108"></a>
## #108 — [K1][OS-04] Virtualized Rust-kernel substrate

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/108
**Created:** 2026-09-15T12:23:25Z | **Updated:** 2026-09-15T14:57:42Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #34 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #34

#### Objective
Build the minimal capability-oriented Rust kernel substrate in a resettable VM before attempting physical-hardware desktop support.

#### Scope
- Boot path for selected architecture/firmware target under virtualization.
- Physical/virtual memory management, address spaces and page-fault handling.
- User processes, scheduler, timers and process lifecycle.
- Capability/handle model and explicit IPC primitives.
- Interrupt/exception handling and kernel diagnostics.
- Basic filesystem/storage abstraction and virtual block driver.
- Basic networking stack/interface and virtual NIC driver or carefully selected reusable components.
- Console/input and minimal user-space init/runtime.
- Kernel panic/crash capture, test harness and image build/reproducibility pipeline.

#### Architecture rules
- AI/browser code runs in unprivileged user processes, never kernel address space.
- Capability transfer is explicit; no ambient global object access by default.
- Every unsafe boundary and hardware-facing component has an owner/audit strategy.

#### Acceptance criteria
- [ ] Resettable VM boots deterministically into a user-space init/process.
- [ ] Two unprivileged processes are isolated and communicate only through declared IPC/capability transfer.
- [ ] Invalid memory/capability accesses are contained to the appropriate failure boundary.
- [ ] Basic file/network/timer workloads pass deterministic integration tests.
- [ ] Worker/process crash does not require full-system corruption/reinstall and diagnostic state is recoverable.
- [ ] Kernel images/tests are produced by an automated CI path with versioned artifacts.

#### Dependencies
- OS-03

**First phase:** K1  
**Maturity target:** K1  
**Owner:** os-systems

### Discussion (1 comments)

#### Comment 5682517218 — Jordan-Hall — 2026-09-15T14:57:42Z

Source: https://github.com/Jordan-Hall/browser/issues/108#issuecomment-5682517218 | Updated: 2026-09-15T14:57:42Z

<!-- intent-implementation-v1:OS-04 -->
###### Implementation proposal — OS-04 / K1

Implement the #107-selected virtualized kernel substrate in its own `no_std` build/unsafe boundary. Ordinary application crates do not become kernel-safe by moving directories.

- [ ] **OS-04.T01 — Boot/diagnostics.** Pin target compiler/linker, implement or reuse the selected boot protocol, validate memory/device handoff and emit reproducible VM images with serial panic output. **Verify:** reset images boot deterministically and malformed boot data rejects.
- [ ] **OS-04.T02 — Memory.** Implement frame allocation, page tables, kernel/user mappings, guard pages and exhaustion behavior with narrow ownership-aware unsafe APIs. **Verify:** user mappings cannot access kernel memory and allocation failures are contained.
- [ ] **OS-04.T03 — Interrupts/exceptions/time.** Install architecture-correct handlers and timers with explicit interrupt-context allocation/locking rules. **Verify:** faults and reentrancy cannot deadlock or corrupt scheduler state.
- [ ] **OS-04.T04 — Processes/threads/scheduler.** Load a minimal executable into isolated address spaces; implement context switching, fair scheduling, termination and accounting. **Verify:** crashing a user worker does not corrupt another process or require reinstall.
- [ ] **OS-04.T05 — Capabilities/IPC.** Implement typed rights-bearing handles, explicit transfer/duplication/revocation and bounded messages; validate every user pointer/length. **Verify:** stale handles, forged rights and invalid buffers cannot cross authority boundaries.
- [ ] **OS-04.T06 — Console/input services.** Add virtual console/input drivers with bounded buffers and explicit device-service access. **Verify:** unprivileged processes need declared capabilities rather than global device access.
- [ ] **OS-04.T07 — Virtual storage/filesystem.** Implement the selected block driver, bounded I/O and minimal file service with defined durability/corruption behavior. **Verify:** interrupted writes and malformed filesystem input fail predictably before real user data is trusted.
- [ ] **OS-04.T08 — Network/init.** Add the selected virtual NIC and bounded isolated packet handling; start/restart user-space workers via init/IPC. **Verify:** network parsers and applications do not gain kernel authority.
- [ ] **OS-04.T09 — Kernel/VM qualification.** Exercise malformed syscalls, stale handles, exhaustion, user faults, driver errors and concurrency with image/seed/crash artifacts. **Verify:** unsafe invariants, memory ordering and user-pointer handling receive separate audit.

**Closure:** deterministic boot plus real isolated user processes, IPC and basic file/network/timer workloads. A kernel boot screenshot alone is insufficient. Detailed implementation follows the selected architecture/hardware specifications; browser and AI logic remain outside kernel privilege.


---

<a id="issue-109"></a>
## #109 — [K2][OS-05] Desktop, speech and inference OS ports

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/109
**Created:** 2026-09-15T12:23:38Z | **Updated:** 2026-09-15T14:58:05Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #34 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #34

#### Objective
Port the real user-space personal runtime workloads onto the Rust-kernel substrate so kernel progress is measured by actual UI, speech, file and local-inference operation rather than boot demos alone.

#### Scope
- Graphics/display/compositor path sufficient for trusted shell/application surfaces.
- Keyboard/pointer/touch input stack and accessibility-facing primitives needed by the desktop runtime.
- Audio input/output path for local speech and read-aloud.
- Storage/filesystem services and content-addressed artifact store compatibility.
- Networking required by browser/connectors with explicit privilege separation.
- Port/adapt supervisor, local IPC, workspace state store and core Rust user-space services.
- Local inference runtime/toolchain support with CPU fallback first and accelerator path measured separately.
- Local speech runtime support using the same product contracts as desktop releases.
- Resource scheduling/telemetry for UI, browser, speech and inference competition.

#### Architecture rules
- Reuse product contracts; do not fork a kernel-specific task/workspace/memory model.
- GPU/accelerator work is measured independently from CPU functionality and may remain limited in early K2.
- Failure of speech/inference must not destabilize the desktop/session service.

#### Acceptance criteria
- [ ] Trusted shell renders and handles normal keyboard/pointer interaction on the K2 target.
- [ ] Workspace/state/artifact operations run through the same logical contracts used by the desktop application.
- [ ] Local speech captures/transcribes an offline test corpus on target hardware/VM configuration.
- [ ] At least one supported local inference workload runs with measured latency/resource use; CPU fallback is documented.
- [ ] Networked browser/connector worker can operate without gaining kernel privilege.
- [ ] Model/speech/browser worker crashes remain isolated from login/files/core session recovery.

#### Dependencies
- OS-04
- LOCAL-01
- VOICE-01

**First phase:** K2  
**Maturity target:** K2  
**Owner:** os-systems

### Discussion (1 comments)

#### Comment 5682522728 — Jordan-Hall — 2026-09-15T14:58:05Z

Source: https://github.com/Jordan-Hall/browser/issues/109#issuecomment-5682522728 | Updated: 2026-09-15T14:58:05Z

<!-- intent-implementation-v1:OS-05 -->
###### Implementation proposal — OS-05 / K2

Port real unprivileged runtime workloads onto #108, reusing #61/#65 and the same product contracts. Rust source compatibility does not automatically provide stdlib, threading, filesystem, database or accelerator support.

- [ ] **OS-05.T01 — User-space foundation.** Implement selected stdlib/libc/runtime interfaces for threads, time, files and networking; start with contract-only and storage tests. **Verify:** required semantics are implemented rather than hidden behind stubs.
- [ ] **OS-05.T02 — Durable storage/artifacts.** Adapt database synchronization/locking and filesystem flush/rename behavior while retaining migrations/recovery. **Verify:** the same durability failpoints produce correct states on the target OS.
- [ ] **OS-05.T03 — Graphics/input/shell.** Implement the selected display/compositor and keyboard/pointer path with trusted versus untrusted surface boundaries. **Verify:** ordinary navigation works and generated/browser content cannot issue privileged shell commands.
- [ ] **OS-05.T04 — Audio/speech.** Port device permissions, capture/output streams and local recognition, preserving visible microphone state, cancellation and default transient audio. **Verify:** offline speech runs on actual target interfaces and permission loss stops capture.
- [ ] **OS-05.T05 — Supervised inference.** Build a compatible CPU backend first with format validation/confinement, then measure optional acceleration. **Verify:** modes, budgets, cancellation and artifact ownership remain identical to the desktop product.
- [ ] **OS-05.T06 — Browser/connector networking.** Run the selected port/compatibility route with isolated networking and authenticated user-space IPC. **Verify:** source/account/task contracts stay unchanged; unsupported full-browser capabilities remain explicit.
- [ ] **OS-05.T07 — Resource/failure integration.** Measure simultaneous shell/browser/audio/inference loads and isolate worker failures under admission control. **Verify:** missing or crashed models cannot prevent session/file/network recovery.
- [ ] **OS-05.T08 — Shared product qualification.** Run core/storage/speech/research/bounded coding fixtures with recorded ABI/runtime/model/image versions. **Verify:** document exact passing workloads and remaining compatibility gaps.

**Stage clarification:** CPU inference/simple graphics and minimal network clients can establish early port progress. They do not establish full browser/GPU usability; those require their actual compatibility evidence and K3/K4 qualification. Keep one workspace/task/memory model rather than forking a kernel-specific product.


---

<a id="issue-110"></a>
## #110 — [K3-K4][OS-06] Compatibility, hardware and supported Rust OS release

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/110
**Created:** 2026-09-15T12:23:49Z | **Updated:** 2026-09-15T14:58:30Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #34 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #34

#### Objective
Move the Rust-kernel track from virtualized research to a supportable operating system on declared hardware with browser/application compatibility, secure updates, recovery and full product conformance.

#### Scope
- Browser/runtime compatibility strategy: native ports, ABI compatibility layer and/or isolated compatibility VM as selected by OS-03 evidence.
- Toolchains and runtime dependencies required by Chromium/alternative browser engines, local inference, speech and extension tooling.
- Physical drivers: storage, networking, USB, input, audio, display/GPU and required accelerator classes.
- Power management, suspend/resume, thermal reporting and battery support.
- Secure boot chain, signed system/application updates, rollback and recovery environment.
- Encrypted user storage, multi-user/session isolation, credential/key recovery and factory/reset flows.
- Accessibility primitives and supported assistive-technology path.
- Application/package distribution model and compatibility/support matrix.
- Long-term security response, driver ownership and hardware lifecycle policy.

#### Release rules
- Support is declared per tested hardware/configuration; unsupported hardware is not implied by architecture.
- Compatibility VMs/layers must preserve the runtime capability/security model rather than becoming an ambient-privilege escape hatch.
- Application crates are not moved into kernel privilege merely to increase the amount of Rust in kernel space.

#### Acceptance criteria
- [ ] Declared reference hardware boots, suspends/resumes and recovers reliably across the release test matrix.
- [ ] Full supported browser/workspace/speech/local-AI/file/desktop workflow suite passes on the declared hardware set.
- [ ] Browser/application compatibility path is measured and documented, including performance/isolation costs.
- [ ] Signed update failure and rollback cannot strand the user without a deterministic recovery path.
- [ ] Encrypted storage/key recovery and multi-user isolation pass threat-model/conformance tests.
- [ ] Security patch/driver maintenance ownership and hardware support lifetime are documented before general release.

#### Dependencies
- OS-05
- EVAL-02
- EVAL-04

**First phase:** K3  
**Maturity target:** K4 supported release  
**Owner:** os-systems

### Discussion (1 comments)

#### Comment 5682528838 — Jordan-Hall — 2026-09-15T14:58:29Z

Source: https://github.com/Jordan-Hall/browser/issues/110#issuecomment-5682528838 | Updated: 2026-09-15T14:58:29Z

<!-- intent-implementation-v1:OS-06 -->
###### Implementation proposal — OS-06 / K3–K4

Qualify a supported Rust OS on declared hardware over #109/#102/#104. A controlled compatibility VM is a valid explicit route; no route may bypass the runtime's account/data/input boundaries.

- [ ] **OS-06.T01 — Application compatibility.** Implement selected ABI/library ports or controlled compatibility VMs from K0 evidence; exercise threads/files/network/process isolation/toolchains with actual browser/model workloads. **Verify:** source-language similarity is not used as proof of binary/runtime compatibility.
- [ ] **OS-06.T02 — Storage/network/USB/input drivers.** Bring up declared devices with bounded queues, error recovery and explicit DMA/IOMMU assumptions. **Verify:** hotplug/removal cannot corrupt unrelated processes or confer ambient device authority.
- [ ] **OS-06.T03 — Graphics/audio/accelerators.** Port qualified stacks and optional inference acceleration with bounded untrusted shader/media/model inputs. **Verify:** measured browser/speech/model behavior and fallback limitations are published per device.
- [ ] **OS-06.T04 — Power/suspend.** Implement battery/thermal reporting, quiescence, reinitialization and clock/deadline handling. **Verify:** wake revalidates grants/auth and uncertain external effects rather than replaying expired work.
- [ ] **OS-06.T05 — Encrypted multi-user state.** Implement key provisioning/recovery, user/session separation and scoped credential services. **Verify:** browser profiles, memories and worker files cannot cross user boundaries.
- [ ] **OS-06.T06 — Boot/system updates.** Establish the selected boot trust chain, signed images/metadata, rotation, rollback floors and compatible activation. **Verify:** agents/runtime workers cannot access signing authority or install unreviewed privileged code.
- [ ] **OS-06.T07 — Recovery/accessibility.** Provide keyboard/assistive login, file/network repair and rollback with inference disabled. **Verify:** a failed AI component cannot lock the user out of essential recovery functions.
- [ ] **OS-06.T08 — Application/support operations.** Integrate package provenance/sandbox policy, compatibility disclosures, driver updates and incident handling with maintainers/support lifetime. **Verify:** supported hardware has sustainable patch and deprecation ownership.
- [ ] **OS-06.T09 — Full hardware/security qualification.** Run the supported browser/workspace/voice/local-AI/file/control suite, restore/update failures, multi-user and sustained-load tests on every reference configuration; commission independent review. **Verify:** release claims match actual passing workload/device evidence.

**Closure gate:** a small verified hardware set with documented compatibility, deterministic recovery and ongoing maintenance—not a one-time boot image or universal-device promise. All K0–K4 scope remains; every task is a reviewable proposal until code and qualification evidence exist.


---

<a id="issue-295"></a>
## #295 — [TASK][EPIC-OS.T01] Ratify separate engine, distribution and kernel roadmaps

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/295
**Created:** 2026-09-15T18:10:20Z | **Updated:** 2026-09-15T18:10:20Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #34

### Original description

Parent: #34

Task ID: `EPIC-OS.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-296"></a>
## #296 — [TASK][EPIC-OS.T02] Integrate the dedicated session and engine experiments

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/296
**Created:** 2026-09-15T18:10:24Z | **Updated:** 2026-09-15T18:10:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #34

### Original description

Parent: #34

Task ID: `EPIC-OS.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-297"></a>
## #297 — [TASK][EPIC-OS.T03] Coordinate K0-K2 substrate and workload ports

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/297
**Created:** 2026-09-15T18:10:32Z | **Updated:** 2026-09-15T18:10:32Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #34

### Original description

Parent: #34

Task ID: `EPIC-OS.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-298"></a>
## #298 — [TASK][EPIC-OS.T04] Plan K3-K4 hardware and supported release

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/298
**Created:** 2026-09-15T18:10:43Z | **Updated:** 2026-09-15T18:10:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #34

### Original description

Parent: #34

Task ID: `EPIC-OS.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

