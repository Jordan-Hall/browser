# Harness and required provider integrations

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 60

## Contents

- [#20 — EPIC: Harness and required provider integrations](#issue-20)
- [#54 — [P1][AGENT-01] Task graph and scoped orchestration](#issue-54)
- [#55 — [P0][AGENT-02] Common AgentSession adapter interface](#issue-55)
- [#56 — [P0][AGENT-03] Codex integration](#issue-56)
- [#57 — [P0][AGENT-04] Claude Code integration](#issue-57)
- [#58 — [P0][AGENT-05] Cursor integration](#issue-58)
- [#59 — [P0][AGENT-06] Grok integration](#issue-59)
- [#60 — [P3][AGENT-07] Cross-agent delegation and independent verification](#issue-60)
- [#239 — [TASK][EPIC-AGENT.T01] Ratify task and provider capability semantics](#issue-239)
- [#240 — [TASK][EPIC-AGENT.T02] Integrate all provider workers under containment](#issue-240)
- [#241 — [TASK][EPIC-AGENT.T03] Integrate bounded delegation and independent verification](#issue-241)
- [#242 — [TASK][EPIC-AGENT.T04] Qualify interruption, switching and protocol drift](#issue-242)
- [#439 — [TASK][AGENT-01.T01] Define graph nodes, effects and state transitions](#issue-439)
- [#440 — [TASK][AGENT-01.T02] Implement durable task submission and planning](#issue-440)
- [#441 — [TASK][AGENT-01.T03] Build the ready-node scheduler](#issue-441)
- [#442 — [TASK][AGENT-01.T04] Implement scoped specialist delegation](#issue-442)
- [#443 — [TASK][AGENT-01.T05] Implement conflicting-resource locks and fencing](#issue-443)
- [#444 — [TASK][AGENT-01.T06] Implement bounded tool execution and context management](#issue-444)
- [#445 — [TASK][AGENT-01.T07] Add independent verification nodes](#issue-445)
- [#446 — [TASK][AGENT-01.T08] Implement checkpoints, stop and recovery](#issue-446)
- [#447 — [TASK][AGENT-01.T09] Expose progress, budgets and revisions](#issue-447)
- [#448 — [TASK][AGENT-01.T10] Run graph race, budget and recovery tests](#issue-448)
- [#449 — [TASK][AGENT-02.T01] Define the common lifecycle and optional capabilities](#issue-449)
- [#450 — [TASK][AGENT-02.T02] Implement supervised transport adapters](#issue-450)
- [#451 — [TASK][AGENT-02.T03] Normalize progress, artifacts and termination](#issue-451)
- [#452 — [TASK][AGENT-02.T04] Implement permission translation and trust classification](#issue-452)
- [#453 — [TASK][AGENT-02.T05] Build session context and destination policy](#issue-453)
- [#454 — [TASK][AGENT-02.T06] Implement interruption and resume semantics](#issue-454)
- [#455 — [TASK][AGENT-02.T07] Create golden protocol and shared workflow fixtures](#issue-455)
- [#456 — [TASK][AGENT-03.T01] Pin and probe the supported Codex build](#issue-456)
- [#457 — [TASK][AGENT-03.T02] Implement App Server session lifecycle](#issue-457)
- [#458 — [TASK][AGENT-03.T03] Translate approvals and tool events](#issue-458)
- [#459 — [TASK][AGENT-03.T04] Stage context and collect artifacts](#issue-459)
- [#460 — [TASK][AGENT-03.T05] Implement interrupt, crash and resume handling](#issue-460)
- [#461 — [TASK][AGENT-03.T06] Qualify the shared coding workflow](#issue-461)
- [#462 — [TASK][AGENT-04.T01] Pin CLI/SDK versions and choose integration mode](#issue-462)
- [#463 — [TASK][AGENT-04.T02] Confine before project discovery](#issue-463)
- [#464 — [TASK][AGENT-04.T03] Implement structured event decoding](#issue-464)
- [#465 — [TASK][AGENT-04.T04] Bridge permissions and scoped tool exposure](#issue-465)
- [#466 — [TASK][AGENT-04.T05] Implement interruption, resume and artifact review](#issue-466)
- [#467 — [TASK][AGENT-04.T06] Qualify hooks, MCP and coding outcomes](#issue-467)
- [#468 — [TASK][AGENT-05.T01] Pin and initialize the Cursor ACP transport](#issue-468)
- [#469 — [TASK][AGENT-05.T02] Implement authentication and session creation](#issue-469)
- [#470 — [TASK][AGENT-05.T03] Normalize updates and permission requests](#issue-470)
- [#471 — [TASK][AGENT-05.T04] Implement artifacts and optional extension support](#issue-471)
- [#472 — [TASK][AGENT-05.T05] Implement cancellation and session recovery](#issue-472)
- [#473 — [TASK][AGENT-05.T06] Run ACP and coding conformance](#issue-473)
- [#474 — [TASK][AGENT-06.T01] Probe official version and structured mode](#issue-474)
- [#475 — [TASK][AGENT-06.T02] Prepare confined configuration and authentication](#issue-475)
- [#476 — [TASK][AGENT-06.T03] Implement session and stream normalization](#issue-476)
- [#477 — [TASK][AGENT-06.T04] Map permissions, tools and artifacts](#issue-477)
- [#478 — [TASK][AGENT-06.T05] Implement stop, failure and supported resume](#issue-478)
- [#479 — [TASK][AGENT-06.T06] Qualify provider and shared coding behavior](#issue-479)
- [#480 — [TASK][AGENT-07.T01] Define specialist jobs and bounded delegation](#issue-480)
- [#481 — [TASK][AGENT-07.T02] Allocate isolated mutable workspaces](#issue-481)
- [#482 — [TASK][AGENT-07.T03] Implement result contracts and independent verifiers](#issue-482)
- [#483 — [TASK][AGENT-07.T04] Reconcile conflicting outputs and patches](#issue-483)
- [#484 — [TASK][AGENT-07.T05] Implement provider switching from shared state](#issue-484)
- [#485 — [TASK][AGENT-07.T06] Implement cascading cancellation and orphan cleanup](#issue-485)
- [#486 — [TASK][AGENT-07.T07] Evaluate multi-agent benefit against a single agent](#issue-486)

---

<a id="issue-20"></a>
## #20 — EPIC: Harness and required provider integrations

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/20
**Created:** 2026-09-15T12:07:26Z | **Updated:** 2026-09-15T14:22:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1

Own durable task graphs, scoped orchestration, the common AgentSession contract, built-in local agent integration and required Codex / Claude Code / Cursor / Grok adapters.

#### Child issues
- [ ] #54 AGENT-01 — Task graph and scoped orchestration
- [ ] #55 AGENT-02 — Common AgentSession adapter interface
- [ ] #56 AGENT-03 — Codex integration
- [ ] #57 AGENT-04 — Claude Code integration
- [ ] #58 AGENT-05 — Cursor integration
- [ ] #59 AGENT-06 — Grok integration
- [ ] #60 AGENT-07 — Cross-agent delegation and independent verification

#### Cross-cutting gates
Providers never own durable product state; subagents cannot expand authority; cancellation/leases/checkpoints are enforced; provider-specific differences remain explicit; external CLIs are confined before project hooks/tools can execute.

### Discussion (1 comments)

#### Comment 5681862665 — Jordan-Hall — 2026-09-15T14:22:47Z

Source: https://github.com/Jordan-Hall/browser/issues/20#issuecomment-5681862665 | Updated: 2026-09-15T14:22:47Z

<!-- intent-implementation-v1:EPIC-AGENT -->
###### Workstream implementation and integration tasks

Implement #54–#60 alongside the built-in agent #63. The supervisor owns tasks; providers are replaceable workers.

- [ ] **EPIC-AGENT.T01 — Ratify task/provider semantics.** Agree GoalContract, DAG/state transitions, AgentSession events, capability negotiation, budgets, epochs and success predicates. **Proof:** missing provider capabilities remain explicit rather than silently emulated.
- [ ] **EPIC-AGENT.T02 — Integrate all providers under containment.** Connect Codex, Claude Code, Cursor and Grok structured interfaces; stage approved context before any project config/hooks can run. **Proof:** shared bounded fixtures run through all available pinned adapters without home/vault access.
- [ ] **EPIC-AGENT.T03 — Integrate bounded delegation/verification.** Give subjobs scoped context, output schemas and resource budgets; isolate mutable worktrees; verify with independent tests/receipts. **Proof:** subagents cannot expand authority or mark unverified results complete.
- [ ] **EPIC-AGENT.T04 — Qualify interruption/switching/drift.** Cancel at every lifecycle state, reject stale worker output, switch providers from shared state and exercise protocol incompatibility. **Proof:** workspace/artifacts survive and unsupported resume is honestly reseeded, not called exact continuation.

**Demonstration:** one provider researches a repository, another proposes a patch in a separate worktree, the local verifier runs tests, and the user reviews the result; stop during a later turn and show no post-revocation dispatch.

**Review rules:** ACP is Agent Client Protocol here, not the commerce protocol. Local CLI execution does not establish local inference. Provider-specific authorization UI never supersedes the runtime broker; unconfined integrations remain manual-only.


---

<a id="issue-54"></a>
## #54 — [P1][AGENT-01] Task graph and scoped orchestration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/54
**Created:** 2026-09-15T12:12:47Z | **Updated:** 2026-09-15T18:31:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #20

#### Objective
Implement the core harness as a durable supervisor-owned task graph, not an open-ended LLM loop.

#### Scope
- Task lifecycle: Draft → Scoped → Planned → Running → AwaitingApproval → Verifying → Completed plus Suspended/Cancelled/Failed/Blocked/NeedsReconciliation.
- DAG/state-machine representation with dependencies, inputs, outputs and explicit success predicates.
- Specialist subjobs with scoped context bundles, allowed capabilities, deadlines and budgets.
- Execution leases/fencing tokens, checkpoints and resumability.
- Concurrency control: parallel safe reads; serialize conflicts over mutable accounts/carts/repos/budgets/desktops.
- Plan revision proposals that cannot expand authority.
- Cancellation cascading and orphan-job cleanup.

#### Harness rules
- The supervisor owns durable state; provider thread/session IDs are secondary references.
- Subagents cannot recursively spawn unbounded trees or inherit ambient authority.
- Completion requires verification, not a model saying “done”.

#### Acceptance criteria
- [ ] Subagents cannot expand their grant/tool/resource scope.
- [ ] Conflicting access to one mutable external resource is serialized/fenced.
- [ ] Cancellation revokes leases before later worker output can dispatch new effects.
- [ ] Task resumes from persisted state after worker/app restart.
- [ ] Budget/deadline exhaustion produces explicit bounded failure/degraded state.
- [ ] Success predicates and verification outcomes are durable and inspectable.

#### Dependencies
- CORE-02
- CORE-03
- SEC-02

**First phase:** P1  
**Maturity target:** P3  
**Owner:** harness-pc-providers

#### Task issues
- [ ] #439 `AGENT-01.T01` — Define graph nodes, effects and state transitions
- [ ] #440 `AGENT-01.T02` — Implement durable task submission and planning
- [ ] #441 `AGENT-01.T03` — Build the ready-node scheduler
- [ ] #442 `AGENT-01.T04` — Implement scoped specialist delegation
- [ ] #443 `AGENT-01.T05` — Implement conflicting-resource locks and fencing
- [ ] #444 `AGENT-01.T06` — Implement bounded tool execution and context management
- [ ] #445 `AGENT-01.T07` — Add independent verification nodes
- [ ] #446 `AGENT-01.T08` — Implement checkpoints, stop and recovery
- [ ] #447 `AGENT-01.T09` — Expose progress, budgets and revisions
- [ ] #448 `AGENT-01.T10` — Run graph race, budget and recovery tests

### Discussion (1 comments)

#### Comment 5682090577 — Jordan-Hall — 2026-09-15T14:34:57Z

Source: https://github.com/Jordan-Hall/browser/issues/54#issuecomment-5682090577 | Updated: 2026-09-15T14:34:57Z

<!-- intent-implementation-v1:AGENT-01 -->
###### Implementation proposal — AGENT-01

Implement a durable DAG plus a state machine per node over #3/#4/#7. Repair loops have explicit iteration/deadline/budget bounds. Trusted limits stay outside prompt compaction.

- [ ] **AGENT-01.T01 — Nodes/effects/transitions.** Define inputs/outputs, dependencies, resource claims, capabilities and success predicates. **Verify:** illegal transitions and conflated Running/AwaitingApproval/Verifying/NeedsReconciliation states reject.
- [ ] **AGENT-01.T02 — Durable submission/planning.** Persist GoalContract, validate bounded planner output against installed operations and store plan revisions. **Verify:** missing capabilities remain diagnostics, not invented tools.
- [ ] **AGENT-01.T03 — Ready-node scheduling.** Admit nodes only after required outputs are verified and grants/resources available. **Verify:** safe independent reads parallelize; blocked reasons/deadlines remain visible.
- [ ] **AGENT-01.T04 — Scoped delegation.** Narrow child context/tools/output schema, bound depth/fan-out and reserve budget from the parent. **Verify:** no child expands authority or creates unbounded descendants.
- [ ] **AGENT-01.T05 — Locks/fencing.** Canonicalize mutable cart/account/worktree/budget/desktop resource IDs and fence leases by epoch. **Verify:** stale workers and competing owners cannot dispatch conflicting effects.
- [ ] **AGENT-01.T06 — Tools/context.** Mediate tool calls, append evidence/artifacts, track usage and rebuild bounded authorized context. **Verify:** compaction never removes trusted account/grant/deadline/success constraints.
- [ ] **AGENT-01.T07 — Verification nodes.** Run independent tests, hashes, source-state and receipt checks; label subjective review separately. **Verify:** a provider saying “done” cannot complete an objectively unverified node.
- [ ] **AGENT-01.T08 — Checkpoint/stop/recovery.** Persist progress, revoke parent/child epochs before cancellation and recover under current scope. **Verify:** late output cannot regain authority; uncertain writes remain unresolved.
- [ ] **AGENT-01.T09 — User controls/projections.** Expose active nodes, waits, usage and plan changes; permit bounded revision/rejection/takeover. **Verify:** changes preserve original intent/history and invalidate stale plans/approvals.
- [ ] **AGENT-01.T10 — Race/budget tests.** Exercise orphan children, exhaustion, invalid verifier output, conflicts and provider loss. **Verify:** recovered projections agree with durable state.

**Review priority:** define the dispatch/revocation linearization point and resource-lock ordering before multi-agent concurrency. Keep task ownership independent from every provider session. All ten tasks need separate review and test evidence.


---

<a id="issue-55"></a>
## #55 — [P0][AGENT-02] Common AgentSession adapter interface

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/55
**Created:** 2026-09-15T12:12:55Z | **Updated:** 2026-09-15T18:31:50Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #20

#### Objective
Create one internal protocol for local and external coding/agent providers while preserving provider-specific capability differences.

#### Scope
- Session capability negotiation and version pinning.
- Start/resume/input/event/progress/artifact/approval/interruption/completion/diagnostics primitives.
- Structured error and termination reasons.
- Provider/model/inference-location disclosure.
- Artifact publication into the shared workspace store.
- Approval requests translated into trusted runtime proposals—not provider-owned confirmations.
- Golden protocol recordings and compatibility fixtures per provider/version.

#### Trust rules
- Adapter integration is not proof the provider's internal file/shell/network tools are mediated.
- Session IDs are not durable task state.
- Prefer machine-readable provider protocols; interactive terminal parsing is not an acceptable primary embedding contract.

#### Acceptance criteria
- [ ] Every required provider can report its actual capabilities through the common interface.
- [ ] Missing optional capabilities remain missing instead of being simulated misleadingly.
- [ ] Start/progress/artifacts/cancel/complete normalize into stable runtime events.
- [ ] Provider version/schema mismatch fails early with actionable diagnostics.
- [ ] Golden fixtures detect protocol drift.
- [ ] Provider session termination does not erase shared task/artifact state.

#### Dependencies
- CORE-01
- SEC-04

**First phase:** P0  
**Maturity target:** P3  
**Owner:** harness-pc-providers

#### Task issues
- [ ] #449 `AGENT-02.T01` — Define the common lifecycle and optional capabilities
- [ ] #450 `AGENT-02.T02` — Implement supervised transport adapters
- [ ] #451 `AGENT-02.T03` — Normalize progress, artifacts and termination
- [ ] #452 `AGENT-02.T04` — Implement permission translation and trust classification
- [ ] #453 `AGENT-02.T05` — Build session context and destination policy
- [ ] #454 `AGENT-02.T06` — Implement interruption and resume semantics
- [ ] #455 `AGENT-02.T07` — Create golden protocol and shared workflow fixtures

### Discussion (1 comments)

#### Comment 5682096464 — Jordan-Hall — 2026-09-15T14:35:16Z

Source: https://github.com/Jordan-Hall/browser/issues/55#issuecomment-5682096464 | Updated: 2026-09-15T14:35:16Z

<!-- intent-implementation-v1:AGENT-02 -->
###### Implementation proposal — AGENT-02

Define one internal AgentSession lifecycle with optional capabilities, not false provider parity. Dependencies #2/#9. Use supervised bounded transports and a disclosed mediated/confined/manual trust class.

- [ ] **AGENT-02.T01 — Lifecycle/capabilities.** Specify initialize/start/input/events/artifacts/approval/interrupt/end and separately negotiate resume, streaming, model selection and tool mediation. **Verify:** provider session IDs cannot substitute for runtime task IDs.
- [ ] **AGENT-02.T02 — Transport.** Bound stdio/RPC frames, correlate requests, separate protocol stdout from diagnostic stderr and enforce deadlines/backpressure. **Verify:** malformed logs/oversized frames cannot become tool messages or freeze control.
- [ ] **AGENT-02.T03 — Event normalization.** Map progress, artifacts, errors and stop reasons while preserving safe protocol references. **Verify:** provider termination is not automatically verified task completion.
- [ ] **AGENT-02.T04 — Permission/trust mapping.** Convert requests to canonical broker proposals; explicitly classify unmediated internal file/shell/network tools. **Verify:** autonomous use is impossible without the advertised mediation or tested confinement.
- [ ] **AGENT-02.T05 — Context/destinations.** Stage only approved files/evidence, declare provider destinations and disclose unknown inference location. **Verify:** home/vault/unrelated context cannot enter a session through inherited environment/config.
- [ ] **AGENT-02.T06 — Interrupt/resume.** Revoke runtime epochs first, send supported cancellation and terminate process trees after bounded grace. Resume only compatible supported state; otherwise reseed explicitly. **Verify:** late provider events cannot dispatch after cancellation.
- [ ] **AGENT-02.T07 — Protocol/workflow fixtures.** Pin startup/auth/permission/artifact/cancel/error/drift recordings and run one shared repository task with an independent verifier. **Verify:** optional-capability differences remain observable.

**Implementation boundary:** structured transport is mandatory for supported integration; do not scrape terminal paint. An adapter does not automatically control every internal provider tool. A provider's “allow always” choice must not be translated into a broader runtime grant.


---

<a id="issue-56"></a>
## #56 — [P0][AGENT-03] Codex integration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/56
**Created:** 2026-09-15T12:13:03Z | **Updated:** 2026-09-15T18:32:31Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #20

#### Objective
Integrate OpenAI Codex through its documented structured application/server surface behind `AgentSession`, with containment, artifacts and independent verification owned by our runtime.

#### Scope
- Pin supported Codex/App Server versions and generated/matched schemas.
- Start/resume/cancel session lifecycle through structured protocol.
- Translate Codex progress, tool/approval events, artifacts and errors into AgentSession events.
- Stage only the approved repository/workspace into the provider environment.
- Map trusted runtime approvals rather than delegating authorization to provider UI.
- Persist provider session/thread IDs only as secondary references.
- Collect patches/artifacts into CODE-01 review flow.

#### Security / product rules
- Codex does not receive the user's home directory, browser profile, credential vault or unrelated personal graph.
- Provider completion never bypasses independent tests/verification.
- Model/provider execution location and data destination are visible.

#### Acceptance criteria
- [ ] A bounded repository fixture starts, makes a scoped change, publishes artifacts and terminates cleanly.
- [ ] Cancellation stops the provider process/session and prevents later dispatch under revoked leases.
- [ ] Independent tests determine verified completion.
- [ ] Unsupported/changed protocol versions fail honestly with diagnostics.
- [ ] No unapproved workspace files/secrets appear in provider context.
- [ ] Crash/restart preserves runtime task/artifact state even if provider session cannot resume.

#### Dependencies
- AGENT-02

**First phase:** P0  
**Maturity target:** P3  
**Owner:** harness-pc-providers

#### Task issues
- [ ] #456 `AGENT-03.T01` — Pin and probe the supported Codex build
- [ ] #457 `AGENT-03.T02` — Implement App Server session lifecycle
- [ ] #458 `AGENT-03.T03` — Translate approvals and tool events
- [ ] #459 `AGENT-03.T04` — Stage context and collect artifacts
- [ ] #460 `AGENT-03.T05` — Implement interrupt, crash and resume handling
- [ ] #461 `AGENT-03.T06` — Qualify the shared coding workflow

### Discussion (1 comments)

#### Comment 5682146540 — Jordan-Hall — 2026-09-15T14:37:55Z

Source: https://github.com/Jordan-Hall/browser/issues/56#issuecomment-5682146540 | Updated: 2026-09-15T14:37:55Z

<!-- intent-implementation-v1:AGENT-03 -->
###### Implementation proposal — AGENT-03

Implement `agent-codex` behind #55 using the documented structured App Server, not terminal rendering. Codex sessions remain secondary references to supervisor-owned tasks; code artifacts enter #72 for independent verification.

- [ ] **AGENT-03.T01 — Pin and probe the supported build.** Record executable provenance/hash/version, generate or pin compatible schemas and run a confined no-project handshake. **Verify:** unsupported protocol/build combinations fail before receiving private context.
- [ ] **AGENT-03.T02 — App Server lifecycle.** Implement initialization, thread/turn creation, input, structured progress and termination with bounded framing and request correlation. **Verify:** provider thread IDs never become the sole durable task state; malformed events cannot advance state.
- [ ] **AGENT-03.T03 — Approval/tool translation.** Convert server-initiated permission requests into canonical runtime proposals with exact resource/action semantics; deny unknown requests and classify unmediated internal tools. **Verify:** provider permission responses cannot expand grants or bypass confinement.
- [ ] **AGENT-03.T04 — Context/artifact pipeline.** Stage only approved project/evidence inputs, constrain environment/egress and publish diffs/artifacts with base revision and provenance. **Verify:** unrelated files, browser profiles and vault contents are inaccessible; outputs enter independent test/review.
- [ ] **AGENT-03.T05 — Interrupt, crash and resume.** Revoke runtime epochs first, invoke documented interruption and terminate the process tree after bounded grace. Resume only compatible supported state or explicitly reseed from shared artifacts. **Verify:** late events cannot dispatch after cancellation and failed resume does not erase the workspace.
- [ ] **AGENT-03.T06 — Shared coding qualification.** Run the common repository fixture plus auth failure, protocol drift, malformed output, cancellation and containment cases. **Verify:** independent tests—not Codex completion text—determine verified success.

**Reference:** [Codex App Server](https://openai.com/index/unlocking-the-codex-harness/). Pin the actual interface at implementation time and publish supported versions. Running the CLI on the PC does not itself establish local inference; show permitted destinations and actual/unknown model execution location. All tasks remain proposed until code, protocol fixtures and containment evidence are reviewed.


---

<a id="issue-57"></a>
## #57 — [P0][AGENT-04] Claude Code integration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/57
**Created:** 2026-09-15T12:13:12Z | **Updated:** 2026-09-15T18:33:34Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #20

#### Objective
Integrate Claude Code through its supported structured CLI/SDK path while containing project hooks/configuration and normalizing its lifecycle into `AgentSession`.

#### Scope
- Pin supported Claude Code versions and structured non-interactive invocation.
- Evaluate/use Agent SDK sidecar only where it materially improves lifecycle/tool control.
- Start from a clean confined environment before project hooks, commands or MCP config can run.
- Normalize progress, outputs, permission requests, artifacts, termination and diagnostics.
- Translate provider permissions to runtime proposals without granting ambient host access.
- Stage approved repository/worktree only.
- Version-specific compatibility tests for config/permission behavior.

#### Security rules
- Project hooks, settings, skills and MCP configuration are untrusted repository inputs.
- Provider-internal shell/file/network capabilities must remain inside OS confinement unless fully mediated.
- No host home directory, vault, browser profile or unrelated context mounts.

#### Acceptance criteria
- [ ] Malicious project hooks/config fixtures cannot escape the staged workspace/sandbox.
- [ ] Shared coding fixture produces normalized progress/artifacts and independently tested output.
- [ ] Cancellation reliably terminates provider execution and invalidates later authority.
- [ ] Provider permission prompts map to trusted runtime state or fail closed.
- [ ] Version drift/config changes are caught by golden/startup fixtures.
- [ ] No terminal-paint scraping is required for the supported path.

#### Dependencies
- AGENT-02

**First phase:** P0  
**Maturity target:** P3  
**Owner:** harness-pc-providers

#### Task issues
- [ ] #462 `AGENT-04.T01` — Pin CLI/SDK versions and choose integration mode
- [ ] #463 `AGENT-04.T02` — Confine before project discovery
- [ ] #464 `AGENT-04.T03` — Implement structured event decoding
- [ ] #465 `AGENT-04.T04` — Bridge permissions and scoped tool exposure
- [ ] #466 `AGENT-04.T05` — Implement interruption, resume and artifact review
- [ ] #467 `AGENT-04.T06` — Qualify hooks, MCP and coding outcomes

### Discussion (1 comments)

#### Comment 5682153082 — Jordan-Hall — 2026-09-15T14:38:17Z

Source: https://github.com/Jordan-Hall/browser/issues/57#issuecomment-5682153082 | Updated: 2026-09-15T14:38:17Z

<!-- intent-implementation-v1:AGENT-04 -->
###### Implementation proposal — AGENT-04

Implement Claude Code as a confined structured provider behind #55. Choose the headless CLI or SDK sidecar from measured lifecycle/approval requirements; neither path replaces OS containment in #9.

- [ ] **AGENT-04.T01 — Pin CLI/SDK and select the integration path.** Record versions, supported structured modes, permission behavior, auth requirements and schema fixtures. **Verify:** a no-project smoke session negotiates known capabilities and rejects incompatible versions.
- [ ] **AGENT-04.T02 — Contain before configuration discovery.** Launch in a staged project with sanitized config roots/environment, explicit MCP/hook policy, scoped credentials and constrained egress. **Verify:** malicious project/user settings cannot execute before the sandbox exists or read the host home/profile/vault.
- [ ] **AGENT-04.T03 — Normalize structured events.** Parse bounded JSON/stream records with request/session/turn correlation; separate protocol output from diagnostics and partial output from terminal results. **Verify:** malformed, oversized or interleaved records cannot become trusted tools or verified completion.
- [ ] **AGENT-04.T04 — Bridge permissions without widening scope.** Map supported requests to exact runtime proposals. Where interactive bridging is unavailable, preconfigure a narrower supported tool set and fail unavailable actions explicitly. **Verify:** unknown or broader permission requests deny rather than become blanket allow decisions.
- [ ] **AGENT-04.T05 — Interrupt, resume and publish artifacts.** Revoke leases before provider cancellation/termination; collect revision-bound diffs and tests into shared storage; resume only supported compatible sessions. **Verify:** no late output dispatches and provider failure leaves reviewable artifacts and truthful task state.
- [ ] **AGENT-04.T06 — Qualify legitimate and hostile repositories.** Run shared coding tasks plus configuration shadowing, malicious hooks/MCP, auth failures, blocked destinations and cancellation races. **Verify:** independent tests and containment probes determine success.

**Review amendment:** current [headless documentation](https://code.claude.com/docs/en/headless) describes `--bare` for suppressing automatic configuration discovery. Evaluate it against the pinned binary and authentication route; it is not a sandbox and does not remove the need to constrain native file/Bash/network tools. Do not use permission-bypass flags as the unattended-personal-work security model. Preserve explicit inference-location and data-destination disclosure.


---

<a id="issue-58"></a>
## #58 — [P0][AGENT-05] Cursor integration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/58
**Created:** 2026-09-15T12:13:20Z | **Updated:** 2026-09-15T19:30:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #20

#### Objective
Integrate Cursor's Agent Client Protocol path behind `AgentSession`, preserving mode/capability differences and strict workspace confinement.

#### Scope
- ACP transport and lifecycle integration over supported stdio/session surface.
- Capability/mode negotiation and provider-version pinning.
- Normalize progress, permission requests, artifacts, diagnostics and cancellation.
- Translate runtime-approved resources into a staged workspace rather than ambient host access.
- Surface actual model/account/inference location where the provider exposes it.
- Golden protocol fixtures for supported versions and optional capabilities.

#### Security / product rules
- ACP transport does not imply permission to expose all local files/tools.
- Unsupported optional ACP capabilities remain explicitly unsupported.
- Provider account/model identity must be visible for tasks that may transmit private code/context.

#### Acceptance criteria
- [ ] Shared bounded coding fixture runs through ACP without terminal scraping.
- [ ] Session cannot access files outside the approved staged workspace.
- [ ] Permission requests normalize into trusted runtime proposals.
- [ ] Cancellation/termination is reflected in task state and invalidates execution leases.
- [ ] Model/account/inference location is disclosed where determinable.
- [ ] Version/capability drift is caught by startup/golden fixtures.

#### Dependencies
- AGENT-02

**First phase:** P0  
**Maturity target:** P3  
**Owner:** harness-pc-providers

### Discussion (2 comments)

#### Comment 5682158877 — Jordan-Hall — 2026-09-15T14:38:36Z

Source: https://github.com/Jordan-Hall/browser/issues/58#issuecomment-5682158877 | Updated: 2026-09-15T14:38:36Z

<!-- intent-implementation-v1:AGENT-05 -->
###### Implementation proposal — AGENT-05

Implement Cursor through its supported Agent Client Protocol surface behind #55, with the actual client capabilities and confinement disclosed rather than simulated.

- [ ] **AGENT-05.T01 — Verify executable and ACP transport.** Pin a supported `agent acp` build, initialize bounded JSON-RPC over stdio and advertise only implemented client filesystem/terminal capabilities. **Verify:** unsupported version/capability combinations fail before private project context is supplied.
- [ ] **AGENT-05.T02 — Authenticate and create scoped sessions.** Use the supported account flow, staged working directory and explicitly authorized tool servers. **Verify:** inherited config or credentials cannot expose unrelated projects or personal accounts.
- [ ] **AGENT-05.T03 — Translate session updates and permissions.** Normalize progress/errors and map permission choices to current broker proposals. Deny unknown semantics and requests exceeding scope. **Verify:** provider allow-always or allow-once options cannot bypass runtime checks.
- [ ] **AGENT-05.T04 — Publish artifacts and metadata.** Record base-revision diffs, outputs and available model/mode/account information; negotiate optional protocol extensions explicitly. **Verify:** artifacts remain usable through the shared workspace after the provider exits; unknown metadata is labelled unknown.
- [ ] **AGENT-05.T05 — Cancel and recover.** Revoke task/worker epochs, send supported cancellation, then bound process-tree shutdown. Resume only if supported and compatible; otherwise reseed from durable runtime state. **Verify:** buffered late updates cannot dispatch new effects or destroy prior artifacts.
- [ ] **AGENT-05.T06 — Protocol and coding conformance.** Run the common coding fixture plus malformed messages, unknown extensions, permission refusals, auth loss and version changes. **Verify:** independent tests establish correctness and staged-files/egress probes establish containment.

**Reference:** [Cursor ACP documentation](https://cursor.com/docs/cli/acp). ACP is Agent Client Protocol, not the commerce protocol. A working ACP session does not imply every provider-internal tool is mediated; enforce #9 confinement where needed. Never automatically approve permission requests merely to keep an agent run moving.

#### Comment 5686917300 — Jordan-Hall — 2026-09-15T19:30:59Z

Source: https://github.com/Jordan-Hall/browser/issues/58#issuecomment-5686917300 | Updated: 2026-09-15T19:30:59Z

###### Task issues
- [ ] #468 `AGENT-05.T01` — Pin and initialize the Cursor ACP transport
- [ ] #469 `AGENT-05.T02` — Implement authentication and session creation
- [ ] #470 `AGENT-05.T03` — Normalize updates and permission requests
- [ ] #471 `AGENT-05.T04` — Implement artifacts and optional extension support
- [ ] #472 `AGENT-05.T05` — Implement cancellation and session recovery
- [ ] #473 `AGENT-05.T06` — Run ACP and coding conformance


---

<a id="issue-59"></a>
## #59 — [P0][AGENT-06] Grok integration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/59
**Created:** 2026-09-15T12:13:32Z | **Updated:** 2026-09-15T19:32:26Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #20

#### Objective
Integrate xAI/Grok's supported ACP/headless agent surface behind `AgentSession` with verified binaries, supported authentication and explicit capability differences.

#### Scope
- Supported ACP/headless transport and provider-version pinning.
- Verify binary/source/version before execution.
- Supported auth flow without leaking credentials into prompts/project files.
- Normalize progress, permission requests, artifacts, errors, diagnostics and cancellation.
- Stage approved workspace only; enforce SEC-04 confinement/egress.
- Golden fixtures for provider errors, disconnects and capability negotiation.

#### Product rules
- Do not parse interactive terminal paint as the primary supported integration.
- Unsupported provider operations are surfaced rather than approximated silently.
- Provider identity/inference location is visible when known.

#### Acceptance criteria
- [ ] Shared bounded coding fixture runs through the supported structured path.
- [ ] Unsupported capabilities are reported explicitly.
- [ ] Provider process/session is stoppable and cannot dispatch after lease revocation.
- [ ] Only approved workspace/context is available to the provider process.
- [ ] Auth/binary/version failures produce deterministic diagnostics.
- [ ] Protocol drift is covered by golden fixtures.

#### Dependencies
- AGENT-02

**First phase:** P0  
**Maturity target:** P3  
**Owner:** harness-pc-providers

### Discussion (2 comments)

#### Comment 5682164624 — Jordan-Hall — 2026-09-15T14:38:54Z

Source: https://github.com/Jordan-Hall/browser/issues/59#issuecomment-5682164624 | Updated: 2026-09-15T14:38:54Z

<!-- intent-implementation-v1:AGENT-06 -->
###### Implementation proposal — AGENT-06

Integrate the supported Grok Build ACP/headless route behind #55. Preserve actual capability differences and use the common confinement/artifact/verification pipeline.

- [ ] **AGENT-06.T01 — Verify binary/version/interface.** Record executable provenance/hash and supported ACP or structured headless modes; run a no-project capability probe. **Verify:** missing or changed protocol support fails explicitly rather than falling back to terminal-paint scraping.
- [ ] **AGENT-06.T02 — Stage configuration and authentication.** Start inside the approved workspace sandbox with scoped auth, controlled config discovery and named network destinations. **Verify:** hooks/plugins or inherited environment cannot reveal unrelated files, accounts or credentials.
- [ ] **AGENT-06.T03 — Normalize structured events.** Map session/turn progress, artifacts, terminal results and diagnostics through bounded parsers and separate stdout/stderr handling. **Verify:** malformed or incomplete provider output cannot mark a task verified.
- [ ] **AGENT-06.T04 — Permissions and artifacts.** Translate supported permission requests into runtime proposals; confine native internal tools; publish base-bound patches and outputs. **Verify:** unsupported autonomous actions remain disabled and all outputs enter independent review/testing.
- [ ] **AGENT-06.T05 — Stop and recover.** Revoke runtime authority before documented interruption and bounded process-tree termination; negotiate resume where available or honestly reseed from shared state. **Verify:** late provider output cannot regain authority and workspace state survives auth/process failure.
- [ ] **AGENT-06.T06 — Shared qualification.** Run the common coding fixture plus auth, configuration, malformed-event, egress and interruption scenarios. **Verify:** publish supported modes/versions and verified outcomes rather than claiming parity with another provider.

**Reference:** [Grok Build documentation](https://docs.x.ai/build/overview). Resolve supported binary, auth and optional capabilities against the pinned release at implementation time. Local process execution is not evidence that inference stays on-device; disclose known and unknown destinations/execution location. All task acceptance requires code, fixture and containment evidence.

#### Comment 5686942298 — Jordan-Hall — 2026-09-15T19:32:26Z

Source: https://github.com/Jordan-Hall/browser/issues/59#issuecomment-5686942298 | Updated: 2026-09-15T19:32:26Z

###### Task issues
- [ ] #474 `AGENT-06.T01` — Probe official version and structured mode
- [ ] #475 `AGENT-06.T02` — Prepare confined configuration and authentication
- [ ] #476 `AGENT-06.T03` — Implement session and stream normalization
- [ ] #477 `AGENT-06.T04` — Map permissions, tools and artifacts
- [ ] #478 `AGENT-06.T05` — Implement stop, failure and supported resume
- [ ] #479 `AGENT-06.T06` — Qualify provider and shared coding behavior


---

<a id="issue-60"></a>
## #60 — [P3][AGENT-07] Cross-agent delegation and independent verification

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/60
**Created:** 2026-09-15T12:13:41Z | **Updated:** 2026-09-15T19:33:16Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #20

#### Objective
Allow multiple agents/providers to cooperate on one durable task while preserving scoped authority, isolated mutable state and verification independent of provider agreement.

#### Scope
- Delegate specialist jobs with bounded context/tool/output schemas.
- Separate repository worktrees and artifact namespaces for parallel code agents.
- Provider switching mid-task using shared GoalContract, evidence and artifact state.
- Critique/review workers for subjective outputs without treating model consensus as proof.
- Deterministic verifiers: tests, type checks, hashes, connector receipts, account state and exact constraints.
- Staged merge/reconciliation of competing patches/results.
- Concurrency/resource locking for shared accounts/budgets/desktops.

#### Product rules
- Agents cannot delegate additional authority beyond their job grants.
- Changing model/provider must not discard user constraints/evidence/workspace state.
- Two models agreeing is not sufficient verification for externally checkable facts/actions.

#### Acceptance criteria
- [ ] Provider switch preserves task constraints, source evidence and artifacts.
- [ ] Parallel coding agents use separate worktrees and cannot silently overwrite each other.
- [ ] Conflicting outputs are surfaced/reconciled rather than first-writer-wins.
- [ ] Unverified/disagreeing results cannot mark the task Completed.
- [ ] Deterministic checks are preferred and recorded when available.
- [ ] Cancellation propagates to all active delegated jobs and invalidates leases.

#### Dependencies
- AGENT-01
- AGENT-02
- CODE-01

**First phase:** P3  
**Maturity target:** P5  
**Owner:** harness-pc-providers

### Discussion (2 comments)

#### Comment 5682171986 — Jordan-Hall — 2026-09-15T14:39:18Z

Source: https://github.com/Jordan-Hall/browser/issues/60#issuecomment-5682171986 | Updated: 2026-09-15T14:39:18Z

<!-- intent-implementation-v1:AGENT-07 -->
###### Implementation proposal — AGENT-07

Implement bounded specialist cooperation over #54/#55/#72. Shared GoalContract/evidence/artifact state is the handoff boundary; provider sessions and model consensus are not authorities.

- [ ] **AGENT-07.T01 — Specialist contracts.** Define role, explicit inputs/outputs, tools, child budget, deadline and maximum delegation depth/fan-out. **Verify:** grants and budgets can only narrow down the task tree.
- [ ] **AGENT-07.T02 — Isolated mutable work.** Allocate separate worktrees, artifact namespaces and browser contexts; serialize shared carts/accounts/budgets and live desktop ownership. **Verify:** agents cannot silently overwrite each other's code or compete for a single checkout/input stream.
- [ ] **AGENT-07.T03 — Structured results and verifiers.** Require evidence references, candidate artifacts and claimed success predicates; run independent tests/provider-state checks. **Verify:** subjective critique is labelled separately and agreement between agents cannot override failing objective evidence.
- [ ] **AGENT-07.T04 — Reconcile candidates.** Compare outputs against a common base, expose conflicting facts/patches and run integration tests before reviewed merge. **Verify:** first arrival does not automatically win and changing the base invalidates stale apply plans.
- [ ] **AGENT-07.T05 — Provider switching.** Construct a handoff from current goal, remaining DAG, evidence/artifact versions and destination policy; renegotiate capabilities. **Verify:** switching retains constraints and user work, while unsupported private-session continuation is not pretended.
- [ ] **AGENT-07.T06 — Cascading stop/recovery.** Revoke descendant epochs, interrupt workers and clean orphaned staged state without discarding uncertain external effects. **Verify:** no delegated job continues using stale authority after parent cancellation.
- [ ] **AGENT-07.T07 — Evaluate actual benefit.** Compare single-agent and multi-agent runs under equal task/tool/resource budgets; report verified quality, interventions, latency, cost and data exposure. **Verify:** enable delegation policies because of measured benefit, not number of agents.

**Integration test:** two agents produce independent patches, a third checks evidence/tests, the user resolves a conflict, then changes provider and cancels a later run. Artifacts and scope remain stable throughout. Do not auto-deploy agent-generated changes to the harness or policy engine.

#### Comment 5686955822 — Jordan-Hall — 2026-09-15T19:33:16Z

Source: https://github.com/Jordan-Hall/browser/issues/60#issuecomment-5686955822 | Updated: 2026-09-15T19:33:16Z

###### Task issues
- [ ] #480 `AGENT-07.T01` — Define specialist jobs and bounded delegation
- [ ] #481 `AGENT-07.T02` — Allocate isolated mutable workspaces
- [ ] #482 `AGENT-07.T03` — Implement result contracts and independent verifiers
- [ ] #483 `AGENT-07.T04` — Reconcile conflicting outputs and patches
- [ ] #484 `AGENT-07.T05` — Implement provider switching from shared state
- [ ] #485 `AGENT-07.T06` — Implement cascading cancellation and orphan cleanup
- [ ] #486 `AGENT-07.T07` — Evaluate multi-agent benefit against a single agent


---

<a id="issue-239"></a>
## #239 — [TASK][EPIC-AGENT.T01] Ratify task and provider capability semantics

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/239
**Created:** 2026-09-15T15:27:24Z | **Updated:** 2026-09-15T15:27:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #20

### Original description

Parent: #20

Task ID: `EPIC-AGENT.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-240"></a>
## #240 — [TASK][EPIC-AGENT.T02] Integrate all provider workers under containment

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/240
**Created:** 2026-09-15T15:27:30Z | **Updated:** 2026-09-15T15:27:30Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #20

### Original description

Parent: #20

Task ID: `EPIC-AGENT.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-241"></a>
## #241 — [TASK][EPIC-AGENT.T03] Integrate bounded delegation and independent verification

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/241
**Created:** 2026-09-15T15:27:35Z | **Updated:** 2026-09-15T15:27:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #20

### Original description

Parent: #20

Task ID: `EPIC-AGENT.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-242"></a>
## #242 — [TASK][EPIC-AGENT.T04] Qualify interruption, switching and protocol drift

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/242
**Created:** 2026-09-15T15:27:41Z | **Updated:** 2026-09-15T15:27:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #20

### Original description

Parent: #20

Task ID: `EPIC-AGENT.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-439"></a>
## #439 — [TASK][AGENT-01.T01] Define graph nodes, effects and state transitions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/439
**Created:** 2026-09-15T18:28:49Z | **Updated:** 2026-09-15T18:28:49Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #54

### Original description

Parent: #54

Task ID: `AGENT-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-440"></a>
## #440 — [TASK][AGENT-01.T02] Implement durable task submission and planning

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/440
**Created:** 2026-09-15T18:28:54Z | **Updated:** 2026-09-15T18:28:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #54

### Original description

Parent: #54

Task ID: `AGENT-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-441"></a>
## #441 — [TASK][AGENT-01.T03] Build the ready-node scheduler

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/441
**Created:** 2026-09-15T18:29:00Z | **Updated:** 2026-09-15T18:29:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #54

### Original description

Parent: #54

Task ID: `AGENT-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-442"></a>
## #442 — [TASK][AGENT-01.T04] Implement scoped specialist delegation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/442
**Created:** 2026-09-15T18:29:07Z | **Updated:** 2026-09-15T18:29:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #54

### Original description

Parent: #54

Task ID: `AGENT-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-443"></a>
## #443 — [TASK][AGENT-01.T05] Implement conflicting-resource locks and fencing

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/443
**Created:** 2026-09-15T18:29:11Z | **Updated:** 2026-09-15T18:29:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #54

### Original description

Parent: #54

Task ID: `AGENT-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-444"></a>
## #444 — [TASK][AGENT-01.T06] Implement bounded tool execution and context management

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/444
**Created:** 2026-09-15T18:29:16Z | **Updated:** 2026-09-15T18:29:16Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #54

### Original description

Parent: #54

Task ID: `AGENT-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-445"></a>
## #445 — [TASK][AGENT-01.T07] Add independent verification nodes

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/445
**Created:** 2026-09-15T18:29:22Z | **Updated:** 2026-09-15T18:29:22Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #54

### Original description

Parent: #54

Task ID: `AGENT-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-446"></a>
## #446 — [TASK][AGENT-01.T08] Implement checkpoints, stop and recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/446
**Created:** 2026-09-15T18:30:41Z | **Updated:** 2026-09-15T18:30:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #54

### Original description

Parent: #54

Task ID: `AGENT-01.T08`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-447"></a>
## #447 — [TASK][AGENT-01.T09] Expose progress, budgets and revisions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/447
**Created:** 2026-09-15T18:30:47Z | **Updated:** 2026-09-15T18:30:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #54

### Original description

Parent: #54

Task ID: `AGENT-01.T09`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-448"></a>
## #448 — [TASK][AGENT-01.T10] Run graph race, budget and recovery tests

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/448
**Created:** 2026-09-15T18:30:52Z | **Updated:** 2026-09-15T18:30:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #54

### Original description

Parent: #54

Task ID: `AGENT-01.T10`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-449"></a>
## #449 — [TASK][AGENT-02.T01] Define the common lifecycle and optional capabilities

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/449
**Created:** 2026-09-15T18:31:09Z | **Updated:** 2026-09-15T18:31:09Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #55

### Original description

Parent: #55

Task ID: `AGENT-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-450"></a>
## #450 — [TASK][AGENT-02.T02] Implement supervised transport adapters

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/450
**Created:** 2026-09-15T18:31:13Z | **Updated:** 2026-09-15T18:31:13Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #55

### Original description

Parent: #55

Task ID: `AGENT-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-451"></a>
## #451 — [TASK][AGENT-02.T03] Normalize progress, artifacts and termination

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/451
**Created:** 2026-09-15T18:31:19Z | **Updated:** 2026-09-15T18:31:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #55

### Original description

Parent: #55

Task ID: `AGENT-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-452"></a>
## #452 — [TASK][AGENT-02.T04] Implement permission translation and trust classification

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/452
**Created:** 2026-09-15T18:31:23Z | **Updated:** 2026-09-15T18:31:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #55

### Original description

Parent: #55

Task ID: `AGENT-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-453"></a>
## #453 — [TASK][AGENT-02.T05] Build session context and destination policy

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/453
**Created:** 2026-09-15T18:31:29Z | **Updated:** 2026-09-15T18:31:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #55

### Original description

Parent: #55

Task ID: `AGENT-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-454"></a>
## #454 — [TASK][AGENT-02.T06] Implement interruption and resume semantics

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/454
**Created:** 2026-09-15T18:31:33Z | **Updated:** 2026-09-15T18:31:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #55

### Original description

Parent: #55

Task ID: `AGENT-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-455"></a>
## #455 — [TASK][AGENT-02.T07] Create golden protocol and shared workflow fixtures

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/455
**Created:** 2026-09-15T18:31:37Z | **Updated:** 2026-09-15T18:31:37Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #55

### Original description

Parent: #55

Task ID: `AGENT-02.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-456"></a>
## #456 — [TASK][AGENT-03.T01] Pin and probe the supported Codex build

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/456
**Created:** 2026-09-15T18:31:55Z | **Updated:** 2026-09-15T18:31:55Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #56

### Original description

Parent: #56

Task ID: `AGENT-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-457"></a>
## #457 — [TASK][AGENT-03.T02] Implement App Server session lifecycle

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/457
**Created:** 2026-09-15T18:31:59Z | **Updated:** 2026-09-15T18:31:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #56

### Original description

Parent: #56

Task ID: `AGENT-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-458"></a>
## #458 — [TASK][AGENT-03.T03] Translate approvals and tool events

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/458
**Created:** 2026-09-15T18:32:04Z | **Updated:** 2026-09-15T18:32:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #56

### Original description

Parent: #56

Task ID: `AGENT-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-459"></a>
## #459 — [TASK][AGENT-03.T04] Stage context and collect artifacts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/459
**Created:** 2026-09-15T18:32:08Z | **Updated:** 2026-09-15T18:32:08Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #56

### Original description

Parent: #56

Task ID: `AGENT-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-460"></a>
## #460 — [TASK][AGENT-03.T05] Implement interrupt, crash and resume handling

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/460
**Created:** 2026-09-15T18:32:13Z | **Updated:** 2026-09-15T18:32:13Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #56

### Original description

Parent: #56

Task ID: `AGENT-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-461"></a>
## #461 — [TASK][AGENT-03.T06] Qualify the shared coding workflow

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/461
**Created:** 2026-09-15T18:32:19Z | **Updated:** 2026-09-15T18:32:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #56

### Original description

Parent: #56

Task ID: `AGENT-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-462"></a>
## #462 — [TASK][AGENT-04.T01] Pin CLI/SDK versions and choose integration mode

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/462
**Created:** 2026-09-15T18:32:40Z | **Updated:** 2026-09-15T18:32:40Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #57

### Original description

Parent: #57

Task ID: `AGENT-04.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-463"></a>
## #463 — [TASK][AGENT-04.T02] Confine before project discovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/463
**Created:** 2026-09-15T18:32:47Z | **Updated:** 2026-09-15T18:32:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #57

### Original description

Parent: #57

Task ID: `AGENT-04.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-464"></a>
## #464 — [TASK][AGENT-04.T03] Implement structured event decoding

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/464
**Created:** 2026-09-15T18:32:55Z | **Updated:** 2026-09-15T18:32:55Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #57

### Original description

Parent: #57

Task ID: `AGENT-04.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-465"></a>
## #465 — [TASK][AGENT-04.T04] Bridge permissions and scoped tool exposure

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/465
**Created:** 2026-09-15T18:33:04Z | **Updated:** 2026-09-15T18:33:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #57

### Original description

Parent: #57

Task ID: `AGENT-04.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-466"></a>
## #466 — [TASK][AGENT-04.T05] Implement interruption, resume and artifact review

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/466
**Created:** 2026-09-15T18:33:13Z | **Updated:** 2026-09-15T18:33:13Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #57

### Original description

Parent: #57

Task ID: `AGENT-04.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-467"></a>
## #467 — [TASK][AGENT-04.T06] Qualify hooks, MCP and coding outcomes

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/467
**Created:** 2026-09-15T18:33:18Z | **Updated:** 2026-09-15T18:33:18Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #57

### Original description

Parent: #57

Task ID: `AGENT-04.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-468"></a>
## #468 — [TASK][AGENT-05.T01] Pin and initialize the Cursor ACP transport

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/468
**Created:** 2026-09-15T18:33:43Z | **Updated:** 2026-09-15T18:33:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #58

### Original description

Parent: #58

Task ID: `AGENT-05.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-469"></a>
## #469 — [TASK][AGENT-05.T02] Implement authentication and session creation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/469
**Created:** 2026-09-15T18:33:51Z | **Updated:** 2026-09-15T18:33:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #58

### Original description

Parent: #58

Task ID: `AGENT-05.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-470"></a>
## #470 — [TASK][AGENT-05.T03] Normalize updates and permission requests

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/470
**Created:** 2026-09-15T19:30:28Z | **Updated:** 2026-09-15T19:30:28Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #58

### Original description

Parent: #58

Task ID: `AGENT-05.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-471"></a>
## #471 — [TASK][AGENT-05.T04] Implement artifacts and optional extension support

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/471
**Created:** 2026-09-15T19:30:33Z | **Updated:** 2026-09-15T19:30:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #58

### Original description

Parent: #58

Task ID: `AGENT-05.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-472"></a>
## #472 — [TASK][AGENT-05.T05] Implement cancellation and session recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/472
**Created:** 2026-09-15T19:30:45Z | **Updated:** 2026-09-15T19:30:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #58

### Original description

Parent: #58

Task ID: `AGENT-05.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-473"></a>
## #473 — [TASK][AGENT-05.T06] Run ACP and coding conformance

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/473
**Created:** 2026-09-15T19:30:52Z | **Updated:** 2026-09-15T19:30:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #58

### Original description

Parent: #58

Task ID: `AGENT-05.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-474"></a>
## #474 — [TASK][AGENT-06.T01] Probe official version and structured mode

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/474
**Created:** 2026-09-15T19:31:28Z | **Updated:** 2026-09-15T19:31:28Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #59

### Original description

Parent: #59

Task ID: `AGENT-06.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-475"></a>
## #475 — [TASK][AGENT-06.T02] Prepare confined configuration and authentication

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/475
**Created:** 2026-09-15T19:31:38Z | **Updated:** 2026-09-15T19:31:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #59

### Original description

Parent: #59

Task ID: `AGENT-06.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-476"></a>
## #476 — [TASK][AGENT-06.T03] Implement session and stream normalization

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/476
**Created:** 2026-09-15T19:31:52Z | **Updated:** 2026-09-15T19:31:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #59

### Original description

Parent: #59

Task ID: `AGENT-06.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-477"></a>
## #477 — [TASK][AGENT-06.T04] Map permissions, tools and artifacts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/477
**Created:** 2026-09-15T19:31:59Z | **Updated:** 2026-09-15T19:31:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #59

### Original description

Parent: #59

Task ID: `AGENT-06.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-478"></a>
## #478 — [TASK][AGENT-06.T05] Implement stop, failure and supported resume

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/478
**Created:** 2026-09-15T19:32:14Z | **Updated:** 2026-09-15T19:32:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #59

### Original description

Parent: #59

Task ID: `AGENT-06.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-479"></a>
## #479 — [TASK][AGENT-06.T06] Qualify provider and shared coding behavior

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/479
**Created:** 2026-09-15T19:32:19Z | **Updated:** 2026-09-15T19:32:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #59

### Original description

Parent: #59

Task ID: `AGENT-06.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-480"></a>
## #480 — [TASK][AGENT-07.T01] Define specialist jobs and bounded delegation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/480
**Created:** 2026-09-15T19:32:34Z | **Updated:** 2026-09-15T19:32:34Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #60

### Original description

Parent: #60

Task ID: `AGENT-07.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-481"></a>
## #481 — [TASK][AGENT-07.T02] Allocate isolated mutable workspaces

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/481
**Created:** 2026-09-15T19:32:41Z | **Updated:** 2026-09-15T19:32:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #60

### Original description

Parent: #60

Task ID: `AGENT-07.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-482"></a>
## #482 — [TASK][AGENT-07.T03] Implement result contracts and independent verifiers

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/482
**Created:** 2026-09-15T19:32:47Z | **Updated:** 2026-09-15T19:32:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #60

### Original description

Parent: #60

Task ID: `AGENT-07.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-483"></a>
## #483 — [TASK][AGENT-07.T04] Reconcile conflicting outputs and patches

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/483
**Created:** 2026-09-15T19:32:53Z | **Updated:** 2026-09-15T19:32:53Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #60

### Original description

Parent: #60

Task ID: `AGENT-07.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-484"></a>
## #484 — [TASK][AGENT-07.T05] Implement provider switching from shared state

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/484
**Created:** 2026-09-15T19:32:59Z | **Updated:** 2026-09-15T19:32:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #60

### Original description

Parent: #60

Task ID: `AGENT-07.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-485"></a>
## #485 — [TASK][AGENT-07.T06] Implement cascading cancellation and orphan cleanup

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/485
**Created:** 2026-09-15T19:33:04Z | **Updated:** 2026-09-15T19:33:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #60

### Original description

Parent: #60

Task ID: `AGENT-07.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-486"></a>
## #486 — [TASK][AGENT-07.T07] Evaluate multi-agent benefit against a single agent

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/486
**Created:** 2026-09-15T19:33:10Z | **Updated:** 2026-09-15T19:33:10Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #60

### Original description

Parent: #60

Task ID: `AGENT-07.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

