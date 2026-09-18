# Semantic and evidence substrate

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 39

## Contents

- [#17 — EPIC: Semantic and evidence substrate](#issue-17)
- [#41 — [P1][DATA-01] Entities, observations and evidence](#issue-41)
- [#42 — [P1][DATA-02] Hybrid retrieval with source-scoped authorization](#issue-42)
- [#43 — [P2][DATA-03] Dependency graph, invalidation and deletion](#issue-43)
- [#44 — [P1][DATA-04] Goal compiler and coordinated plans](#issue-44)
- [#227 — [TASK][EPIC-DATA.T01] Ratify semantic ownership and source boundaries](#issue-227)
- [#228 — [TASK][EPIC-DATA.T02] Integrate authorized retrieval and provenance](#issue-228)
- [#229 — [TASK][EPIC-DATA.T03] Integrate dependencies and coordinated goal plans](#issue-229)
- [#230 — [TASK][EPIC-DATA.T04] Qualify data evolution and deletion](#issue-230)
- [#344 — [TASK][DATA-01.T01] Define the common ontology and extension rules](#issue-344)
- [#345 — [TASK][DATA-01.T02] Implement scoped identity and entity resolution](#issue-345)
- [#346 — [TASK][DATA-01.T03] Build immutable observation ingestion](#issue-346)
- [#347 — [TASK][DATA-01.T04] Implement evidence spans and claim associations](#issue-347)
- [#348 — [TASK][DATA-01.T05] Separate overlays and inferred projections](#issue-348)
- [#349 — [TASK][DATA-01.T06] Integrate access, freshness and retention metadata](#issue-349)
- [#350 — [TASK][DATA-01.T07] Publish normalization conformance fixtures](#issue-350)
- [#351 — [TASK][DATA-02.T01] Implement authoritative exact retrieval](#issue-351)
- [#352 — [TASK][DATA-02.T02] Build incremental full-text indexing](#issue-352)
- [#353 — [TASK][DATA-02.T03] Add embedding and semantic candidate retrieval](#issue-353)
- [#354 — [TASK][DATA-02.T04] Implement fusion and scoped reranking](#issue-354)
- [#355 — [TASK][DATA-02.T05] Build evidence-preserving context bundles](#issue-355)
- [#356 — [TASK][DATA-02.T06] Implement deletion, cache invalidation and rebuild](#issue-356)
- [#357 — [TASK][DATA-02.T07] Measure quality and privacy independently](#issue-357)
- [#358 — [TASK][DATA-03.T01] Define dependency edges and revision semantics](#issue-358)
- [#359 — [TASK][DATA-03.T02] Record dependencies during computation](#issue-359)
- [#360 — [TASK][DATA-03.T03] Implement durable invalidation traversal](#issue-360)
- [#361 — [TASK][DATA-03.T04] Implement type-specific refresh behavior](#issue-361)
- [#362 — [TASK][DATA-03.T05] Invalidate approvals and transaction preconditions](#issue-362)
- [#363 — [TASK][DATA-03.T06] Propagate deletion through every derivative](#issue-363)
- [#364 — [TASK][DATA-03.T07] Expose invalidation explanations and audit](#issue-364)
- [#365 — [TASK][DATA-03.T08] Qualify graph convergence and recovery](#issue-365)
- [#366 — [TASK][DATA-04.T01] Implement GoalContract editing and normalization](#issue-366)
- [#367 — [TASK][DATA-04.T02] Build capability-aware source planning](#issue-367)
- [#368 — [TASK][DATA-04.T03] Compile views from available typed data](#issue-368)
- [#369 — [TASK][DATA-04.T04] Bind actions to exact capabilities](#issue-369)
- [#370 — [TASK][DATA-04.T05] Compile independent verification requirements](#issue-370)
- [#371 — [TASK][DATA-04.T06] Validate and atomically publish coordinated plans](#issue-371)
- [#372 — [TASK][DATA-04.T07] Implement incremental recompilation and plan diffs](#issue-372)
- [#373 — [TASK][DATA-04.T08] Test compiler correctness and useful diagnostics](#issue-373)

---

<a id="issue-17"></a>
## #17 — EPIC: Semantic and evidence substrate

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/17
**Created:** 2026-09-15T12:07:03Z | **Updated:** 2026-09-15T14:21:50Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1

Own source-backed entities, observations, claims/evidence, retrieval, freshness/dependency invalidation and goal compilation. This layer keeps normalized data useful without erasing provider/account/source semantics.

#### Child issues
- [ ] #41 DATA-01 — Entities, observations and evidence
- [ ] #42 DATA-02 — Hybrid retrieval with source-scoped authorization
- [ ] #43 DATA-03 — Dependency graph, invalidation and deletion
- [ ] #44 DATA-04 — Goal compiler and coordinated plans

#### Cross-cutting gates
Source/user/inferred state stays distinct, access checks precede retrieval, derivatives invalidate correctly, and important displayed facts remain traceable to evidence.

### Discussion (1 comments)

#### Comment 5681844652 — Jordan-Hall — 2026-09-15T14:21:50Z

Source: https://github.com/Jordan-Hall/browser/issues/17#issuecomment-5681844652 | Updated: 2026-09-15T14:21:50Z

<!-- intent-implementation-v1:EPIC-DATA -->
###### Workstream implementation and integration tasks

Integrate #41–#44 as the authoritative source/evidence and goal-compilation substrate, not an undifferentiated vector database.

- [ ] **EPIC-DATA.T01 — Ratify semantic ownership and source boundaries.** Separate entities, account-qualified provider objects, immutable observations, user overlays and derived claims. Define revision/time/access labels. **Proof:** conflicting sources coexist without overwriting provider truth.
- [ ] **EPIC-DATA.T02 — Integrate authorized retrieval and provenance.** Combine relational/FTS/optional vector retrieval with pre-exposure authorization and evidence-preserving context bundles. **Proof:** denied material never leaks through snippets, reranking or summaries.
- [ ] **EPIC-DATA.T03 — Integrate dependencies and goal plans.** Connect observation→claim→view/ranking/article/approval dependencies and compile GoalContract into coordinated query/UI/action/verification plans. **Proof:** an offer change updates the right view and invalidates affected uncommitted authority.
- [ ] **EPIC-DATA.T04 — Qualify evolution and deletion.** Test source revisions, tombstones, schema migrations, extractor upgrades and cache/index cleanup. **Proof:** stale/removed data cannot reappear through derived retrieval; reading artifacts update as reviewable revisions.

**Demonstration:** display one entity as cards, table and synthesized article; correct a user preference without mutating sources; delete a source record and inspect all affected derivatives.

**Boundary:** model-generated conclusions are not original observations, approximate retrieval is not authorization, and deleting eligible source content does not require destroying minimal retained transaction receipts. Task-level tests and migration traces are required for closure.


---

<a id="issue-41"></a>
## #41 — [P1][DATA-01] Entities, observations and evidence

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/41
**Created:** 2026-09-15T12:10:29Z | **Updated:** 2026-09-15T18:17:57Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #17

#### Objective
Create the semantic substrate that lets services act as data/capability providers without collapsing provider-specific truth into one lossy universal schema.

#### Scope
- Small common ontology: Person, Organization, Product, Offer, Article, Claim, Post, Thread, Message, Event, File, Task, ActionReceipt plus provider extensions.
- Stable internal IDs plus provider object/account IDs.
- `Observation` records with source URI/object, source revision, observed/source timestamps, extractor version and content hash.
- `Evidence` spans linking claims/fields to bounded source material.
- Access, sensitivity and freshness labels.
- Separate user overlays and derived/inferred claims from source observations.

#### Data rules
- Product != offer != order; person != account; article != claim.
- Provider originals and identifiers are preserved.
- Inference cannot silently overwrite authoritative source fields.

#### Acceptance criteria
- [ ] Source observations, user overlays and inferred claims are distinct and independently queryable.
- [ ] Each critical source-backed field can expose provider object ID, observation time and freshness.
- [ ] Provider-specific fields can coexist without polluting the common schema.
- [ ] Account/workspace scope is part of entity/observation identity and cache keys.
- [ ] Evidence spans survive normal migrations and are invalidated when the referenced source revision changes incompatibly.

#### Dependencies
- CORE-01
- CORE-02
- SEC-02

**First phase:** P1  
**Maturity target:** P2  
**Owner:** workspace-ui-data

#### Task issues
- [ ] #344 `DATA-01.T01` — Define the common ontology and extension rules
- [ ] #345 `DATA-01.T02` — Implement scoped identity and entity resolution
- [ ] #346 `DATA-01.T03` — Build immutable observation ingestion
- [ ] #347 `DATA-01.T04` — Implement evidence spans and claim associations
- [ ] #348 `DATA-01.T05` — Separate overlays and inferred projections
- [ ] #349 `DATA-01.T06` — Integrate access, freshness and retention metadata
- [ ] #350 `DATA-01.T07` — Publish normalization conformance fixtures

### Discussion (1 comments)

#### Comment 5682003261 — Jordan-Hall — 2026-09-15T14:30:19Z

Source: https://github.com/Jordan-Hall/browser/issues/41#issuecomment-5682003261 | Updated: 2026-09-15T14:30:19Z

<!-- intent-implementation-v1:DATA-01 -->
###### Implementation proposal — DATA-01

Use a small common ontology with provider-qualified extensions, immutable observations and independently stored overlays/derivations. Dependencies #2/#3/#7. Proposed records: Entity, ProviderIdentity, Observation, EvidenceSpan, Claim, UserOverlay, EntityProjection.

- [ ] **DATA-01.T01 — Ontology/extensions.** Distinguish Person/Account, Product/Variant/Offer/Order, Article/Claim and social/file/task types. **Verify:** schema fixtures preserve domain distinctions and extension namespaces.
- [ ] **DATA-01.T02 — Scoped identity/resolution.** Map internal IDs to provider/account/object IDs; represent verified same-as versus probable-match; support reversible correction. **Verify:** similar labels cannot merge accounts or destroy original identities.
- [ ] **DATA-01.T03 — Observation ingestion.** Validate scope/schema/size, retain source revision/time/extractor version and bounded original artifact, then append transactionally. **Verify:** arrival order cannot replace a newer provider revision with an older one.
- [ ] **DATA-01.T04 — Evidence/claims.** Anchor bounded offsets/selectors/provider fields to an exact observation and hash; store support/contradiction and entity/time scope. **Verify:** invalid or shifted spans fail visibly rather than supporting the wrong claim.
- [ ] **DATA-01.T05 — Overlays/inference.** Store annotations/exclusions independently; attach confidence, evidence and transform version to model outputs. **Verify:** display precedence never overwrites source records.
- [ ] **DATA-01.T06 — Access/freshness/retention.** Carry labels at observation/artifact/claim boundaries and emit source changes for invalidation. **Verify:** expired rights or private records cannot survive through more permissive derivatives.
- [ ] **DATA-01.T07 — Normalization tests.** Cover variant mismatches, duplicate names/accounts, edits/deletions, ambiguous clocks, failed extraction, migrations and reversible identity correction. **Verify:** independently known expected objects and evidence remain intact.

**Review boundary:** timestamps and hashes establish provenance/integrity metadata, not truth of a provider's assertion. Unknown values stay unknown. The current source projection, user preference and model conclusion must be independently queryable and explainable.


---

<a id="issue-42"></a>
## #42 — [P1][DATA-02] Hybrid retrieval with source-scoped authorization

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/42
**Created:** 2026-09-15T12:10:38Z | **Updated:** 2026-09-15T18:18:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #17

#### Objective
Build retrieval that combines authoritative relational state, exact full-text search and optional semantic retrieval without leaking private or out-of-scope data into model context.

#### Scope
- Relational queries for authoritative entities/permissions/source metadata.
- Full-text index for exact names, IDs, code, quotations and documents.
- Optional embedding/vector index and reranking for semantic retrieval.
- Task-scoped context builder with access/destination checks before retrieval material is emitted.
- Evidence-preserving summaries and chunk references.
- Account/workspace/profile-aware caches and index partitions.
- Deletion/retention hooks for all searchable derivatives.

#### Security rules
- Authorization occurs before snippets/embeddings/summaries are exposed to a worker/model.
- Embeddings and cached summaries inherit privacy restrictions.
- Work/personal/private-session indexes remain isolated.

#### Acceptance criteria
- [ ] A denied/private record cannot appear through snippets, embedding neighbors, reranker input or cached summaries.
- [ ] Exact-ID/full-text queries prefer authoritative matches over approximate semantic matches.
- [ ] Context bundles preserve evidence links and unresolved constraints.
- [ ] Index updates and deletions are observable and recoverable after crashes.
- [ ] Cloud-context construction applies explicit destination policy before serialization.
- [ ] Retrieval evaluation reports recall, precision, latency and privacy failures separately.

#### Dependencies
- DATA-01
- SEC-03

**First phase:** P1  
**Maturity target:** P3  
**Owner:** workspace-ui-data

#### Task issues
- [ ] #351 `DATA-02.T01` — Implement authoritative exact retrieval
- [ ] #352 `DATA-02.T02` — Build incremental full-text indexing
- [ ] #353 `DATA-02.T03` — Add embedding and semantic candidate retrieval
- [ ] #354 `DATA-02.T04` — Implement fusion and scoped reranking
- [ ] #355 `DATA-02.T05` — Build evidence-preserving context bundles
- [ ] #356 `DATA-02.T06` — Implement deletion, cache invalidation and rebuild
- [ ] #357 `DATA-02.T07` — Measure quality and privacy independently

### Discussion (1 comments)

#### Comment 5682009299 — Jordan-Hall — 2026-09-15T14:30:39Z

Source: https://github.com/Jordan-Hall/browser/issues/42#issuecomment-5682009299 | Updated: 2026-09-15T14:30:39Z

<!-- intent-implementation-v1:DATA-02 -->
###### Implementation proposal — DATA-02

Implement exact relational lookup first, incremental full-text second and optional vectors third. Dependencies #41/#8. Use AuthorizedCorpus, CandidateRef, ChunkRef, ContextBundle and IndexGeneration; indexes are rebuildable derivatives, never permission authorities.

- [ ] **DATA-02.T01 — Exact retrieval.** Resolve scoped typed IDs/provider IDs/paths before approximate search. **Verify:** authoritative exact matches retain source revisions and denied records are not exposed.
- [ ] **DATA-02.T02 — Full-text indexing.** Consume durable source events into policy-partitioned chunks with source/revision/cursor metadata. **Verify:** crash/rebuild produces the same eligible corpus.
- [ ] **DATA-02.T03 — Embeddings/candidates.** Compute locally, version model/tokenizer/chunker and separate incompatible spaces; restrict corpus before search. **Verify:** candidate IDs, scores and snippets cannot disclose denied partitions.
- [ ] **DATA-02.T04 — Fusion/reranking.** Deduplicate by source/chunk and pass only authorized bounded content to the selected local or granted remote reranker. **Verify:** ranking provenance and destination checks survive fusion.
- [ ] **DATA-02.T05 — Context bundles.** Select under byte/token budgets, retain evidence and unresolved constraints, then recheck access/destination at serialization. **Verify:** revocation between retrieval and serialization blocks release.
- [ ] **DATA-02.T06 — Deletion/cache rebuild.** Immediately suppress tombstoned/revoked data, then remove indexes/caches asynchronously; cache keys include account/profile/policy/model/index generation. **Verify:** stale caches cannot resurrect forgotten context.
- [ ] **DATA-02.T07 — Quality/privacy evaluation.** Measure exact-ID, code-search, semantic recall, source support, latency/storage and unauthorized exposure separately. **Verify:** a better recall score cannot hide a privacy regression.

**Decision:** introduce vector storage only once exact retrieval and access boundaries work. An embedding is derived private data when its inputs are private. Filtering a result after sending it to a reranker is too late; enforce scope before every exposure boundary.


---

<a id="issue-43"></a>
## #43 — [P2][DATA-03] Dependency graph, invalidation and deletion

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/43
**Created:** 2026-09-15T12:10:50Z | **Updated:** 2026-09-15T18:19:50Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #17

#### Objective
Track which displayed facts, rankings, synthesized text, approvals and notifications depend on which source observations so change/deletion propagates correctly.

#### Scope
- Dependency edges from observations → claims → derived entities/views/rankings/articles/approvals/alerts.
- Source tombstones, revision history and freshness policies.
- Incremental invalidation/recomputation queue.
- Different update semantics by artifact type: live field refresh, recompute ranking, propose article revision, invalidate approval.
- Deletion propagation to full-text/vector indexes, caches and future context eligibility.
- Audit of why a derivative became stale/invalid.

#### Correctness rules
- Do not silently rewrite text the user is actively reading when a reviewable revision is safer.
- Material source changes invalidate pending approvals that relied on the old version.
- Tombstones/deletions must propagate without erasing immutable action receipts that are legally/operationally required.

#### Acceptance criteria
- [ ] Changing a product price invalidates/recomputes affected comparison/ranking state and stale uncommitted purchase authority.
- [ ] Revoking/deleting a social post removes it from derived feeds and indexes according to policy.
- [ ] Source changes produce explainable dependency traces.
- [ ] Deleted eligible data no longer appears in search, semantic retrieval or new model context.
- [ ] Article/synthesis changes are versioned and reviewable rather than silently overwritten.
- [ ] Invalidation survives crash/restart without leaving permanent mixed revisions.

#### Dependencies
- DATA-01
- CORE-02

**First phase:** P2  
**Maturity target:** P5  
**Owner:** workspace-ui-data

#### Task issues
- [ ] #358 `DATA-03.T01` — Define dependency edges and revision semantics
- [ ] #359 `DATA-03.T02` — Record dependencies during computation
- [ ] #360 `DATA-03.T03` — Implement durable invalidation traversal
- [ ] #361 `DATA-03.T04` — Implement type-specific refresh behavior
- [ ] #362 `DATA-03.T05` — Invalidate approvals and transaction preconditions
- [ ] #363 `DATA-03.T06` — Propagate deletion through every derivative
- [ ] #364 `DATA-03.T07` — Expose invalidation explanations and audit
- [ ] #365 `DATA-03.T08` — Qualify graph convergence and recovery

### Discussion (1 comments)

#### Comment 5682014982 — Jordan-Hall — 2026-09-15T14:30:56Z

Source: https://github.com/Jordan-Hall/browser/issues/43#issuecomment-5682014982 | Updated: 2026-09-15T14:30:56Z

<!-- intent-implementation-v1:DATA-03 -->
###### Implementation proposal — DATA-03

Implement durable dependency invalidation over #41/#3. Every derived record identifies exact input revisions; different output types have different refresh semantics.

- [ ] **DATA-03.T01 — Edge/revision model.** Define supports, derived-from, display-binding and approval-precondition edges plus derivation versions. **Verify:** dependency graphs distinguish evidence links from mere navigation links.
- [ ] **DATA-03.T02 — Capture derivation inputs.** Require transforms/context builders to publish their authorized input manifest with outputs. Keep results unavailable until dependencies commit. **Verify:** a crash cannot publish an untracked derivative.
- [ ] **DATA-03.T03 — Durable traversal.** Queue bounded, idempotent invalidation jobs keyed by cause/input revision; protect against cycles/high fan-out. **Verify:** older events cannot restore a newer invalidated state.
- [ ] **DATA-03.T04 — Type-specific refresh.** Update table cells, rerank offers, propose article revisions and disable stale proposals while preserving overlays/layout/focus. **Verify:** changed data does not silently move controls or rewrite active reading.
- [ ] **DATA-03.T05 — Approval preconditions.** Track material quote/source fields and invalidate affected uncommitted approvals; recheck at dispatch even without events. **Verify:** missed webhooks cannot make a stale quote executable.
- [ ] **DATA-03.T06 — Deletion propagation.** Suppress views, FTS/vectors, caches, notifications and future context; queue offline-device tombstones and retain only justified minimal receipts. **Verify:** deleted sources cannot reappear through derived channels.
- [ ] **DATA-03.T07 — Explain invalidation.** Show source change, affected outputs, recomputation and required new approval with access-checked history. **Verify:** users can trace why a displayed fact/action became stale.
- [ ] **DATA-03.T08 — Convergence tests.** Inject crashes, missing inputs, concurrent deletion/revocation, cycles and fan-out; compare incremental results to a full rebuild. **Verify:** no permanently mixed revision state.

**Acceptance demonstration:** change an offer's price and delete a social post; inspect precisely affected comparisons, rankings, syntheses, indexes and pending actions. Source retention restrictions continue to apply to historical evidence.


---

<a id="issue-44"></a>
## #44 — [P1][DATA-04] Goal compiler and coordinated plans

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/44
**Created:** 2026-09-15T12:10:59Z | **Updated:** 2026-09-15T18:20:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #17

#### Objective
Compile user intent and durable constraints into a coherent data plan, view definition, executable action proposals and verification requirements.

#### Scope
- Define `GoalContract`: original request, clarified constraints, authorized resources/accounts, privacy/inference mode, budgets/deadlines, desired representation, success predicate and approval requirements.
- Query/source planning against installed connector capabilities.
- UI plan against the declarative component catalogue.
- Action bindings that resolve to real capability IDs and exact argument schemas.
- Verification plan for each consequential task/action.
- Compiler diagnostics for missing capability, ambiguous target, stale source or unsatisfied constraint.
- Incremental recompilation preserving the last valid view.

#### Product rules
- A generated button is invalid unless it resolves to an installed capability under current account/resource semantics.
- User edits to constraints/layout compile into updated plans without requiring conversational reconstruction.
- Authorization remains a separate broker decision; compilation never grants authority.

#### Acceptance criteria
- [ ] One goal produces coordinated source queries, view bindings, permitted action proposals and verification predicates.
- [ ] A generated action resolves a real installed operation or is rejected before rendering as actionable.
- [ ] Failed recompilation leaves the previous valid view usable.
- [ ] Missing/ambiguous capabilities produce explicit diagnostics and Original-view fallback where appropriate.
- [ ] Goal constraints remain durable across provider/model switches and conversation compaction.
- [ ] Compilation output is deterministic for the same normalized inputs where model-free planning is sufficient.

#### Dependencies
- CORE-01
- DATA-01
- CONN-01
- UI-01

**First phase:** P1  
**Maturity target:** P3  
**Owner:** workspace-ui-data

#### Task issues
- [ ] #366 `DATA-04.T01` — Implement GoalContract editing and normalization
- [ ] #367 `DATA-04.T02` — Build capability-aware source planning
- [ ] #368 `DATA-04.T03` — Compile views from available typed data
- [ ] #369 `DATA-04.T04` — Bind actions to exact capabilities
- [ ] #370 `DATA-04.T05` — Compile independent verification requirements
- [ ] #371 `DATA-04.T06` — Validate and atomically publish coordinated plans
- [ ] #372 `DATA-04.T07` — Implement incremental recompilation and plan diffs
- [ ] #373 `DATA-04.T08` — Test compiler correctness and useful diagnostics

### Discussion (1 comments)

#### Comment 5682022262 — Jordan-Hall — 2026-09-15T14:31:19Z

Source: https://github.com/Jordan-Hall/browser/issues/44#issuecomment-5682022262 | Updated: 2026-09-15T14:31:19Z

<!-- intent-implementation-v1:DATA-04 -->
###### Implementation proposal — DATA-04

Build a staged goal compiler, not one prompt that invents tools and declares success. Dependencies #2/#41/#45/#49. Proposed outputs: DataPlan, ViewPlan, ActionBinding, VerificationPlan and CompiledWorkspaceRevision.

- [ ] **DATA-04.T01 — Goal editing/normalization.** Preserve original intent plus explicit constraints, accounts/resources, mode, budgets/deadlines, desired view and success predicates. **Verify:** deterministic field edits persist independently from chat.
- [ ] **DATA-04.T02 — Capability-aware data planning.** Resolve installed/authenticated source operations with schema, quota, cost and support limits; preserve gaps/fallbacks. **Verify:** missing APIs never become invented callable tools.
- [ ] **DATA-04.T03 — View compilation.** Select trusted components, bind typed fields/queries and preserve layout locks, accessibility and unknown values. **Verify:** valid data can produce multiple reusable views without new connector logic.
- [ ] **DATA-04.T04 — Exact action binding.** Map controls to provider/account/operation/schema and entity-derived arguments; distinguish unavailable, approval-required and authorized states. **Verify:** generated controls cannot mint grants or bypass transactions.
- [ ] **DATA-04.T05 — Independent verification plans.** Derive checks from user constraints and capability verifiers; separate objective tests/receipts from subjective review. **Verify:** model completion text cannot satisfy an external success predicate.
- [ ] **DATA-04.T06 — Atomic coordinated publication.** Validate references, privacy destinations, budgets, cycles and unsupported operations; publish one revision with dependencies. **Verify:** failed compilation leaves the last approved view usable.
- [ ] **DATA-04.T07 — Incremental recompile/diff.** Recompile affected plans after goal/source/capability changes and present material structural/action differences for review. **Verify:** changed constraints cannot reuse stale approval or overwrite user layout choices.
- [ ] **DATA-04.T08 — Compiler conformance.** Use deterministic golden cases plus repeated model-assisted trials against fixed catalogues; test conflicting constraints, injection strings and stale identities. **Verify:** useful diagnostics identify the unsatisfied contract rather than fabricate success.

**Review boundary:** compilation creates proposals, never authority. The same goal contract must constrain data retrieval, UI controls, execution and verification; a visually convincing interface is insufficient without valid capability bindings.


---

<a id="issue-227"></a>
## #227 — [TASK][EPIC-DATA.T01] Ratify semantic ownership and source boundaries

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/227
**Created:** 2026-09-15T15:26:07Z | **Updated:** 2026-09-15T15:26:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #17

### Original description

Parent: #17

Task ID: `EPIC-DATA.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-228"></a>
## #228 — [TASK][EPIC-DATA.T02] Integrate authorized retrieval and provenance

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/228
**Created:** 2026-09-15T15:26:16Z | **Updated:** 2026-09-15T15:26:16Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #17

### Original description

Parent: #17

Task ID: `EPIC-DATA.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-229"></a>
## #229 — [TASK][EPIC-DATA.T03] Integrate dependencies and coordinated goal plans

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/229
**Created:** 2026-09-15T15:26:21Z | **Updated:** 2026-09-15T15:26:21Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #17

### Original description

Parent: #17

Task ID: `EPIC-DATA.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-230"></a>
## #230 — [TASK][EPIC-DATA.T04] Qualify data evolution and deletion

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/230
**Created:** 2026-09-15T15:26:26Z | **Updated:** 2026-09-15T15:26:26Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #17

### Original description

Parent: #17

Task ID: `EPIC-DATA.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-344"></a>
## #344 — [TASK][DATA-01.T01] Define the common ontology and extension rules

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/344
**Created:** 2026-09-15T18:17:04Z | **Updated:** 2026-09-15T18:17:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #41

### Original description

Parent: #41

Task ID: `DATA-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-345"></a>
## #345 — [TASK][DATA-01.T02] Implement scoped identity and entity resolution

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/345
**Created:** 2026-09-15T18:17:13Z | **Updated:** 2026-09-15T18:17:13Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #41

### Original description

Parent: #41

Task ID: `DATA-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-346"></a>
## #346 — [TASK][DATA-01.T03] Build immutable observation ingestion

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/346
**Created:** 2026-09-15T18:17:19Z | **Updated:** 2026-09-15T18:17:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #41

### Original description

Parent: #41

Task ID: `DATA-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-347"></a>
## #347 — [TASK][DATA-01.T04] Implement evidence spans and claim associations

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/347
**Created:** 2026-09-15T18:17:25Z | **Updated:** 2026-09-15T18:17:25Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #41

### Original description

Parent: #41

Task ID: `DATA-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-348"></a>
## #348 — [TASK][DATA-01.T05] Separate overlays and inferred projections

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/348
**Created:** 2026-09-15T18:17:29Z | **Updated:** 2026-09-15T18:17:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #41

### Original description

Parent: #41

Task ID: `DATA-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-349"></a>
## #349 — [TASK][DATA-01.T06] Integrate access, freshness and retention metadata

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/349
**Created:** 2026-09-15T18:17:39Z | **Updated:** 2026-09-15T18:17:39Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #41

### Original description

Parent: #41

Task ID: `DATA-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-350"></a>
## #350 — [TASK][DATA-01.T07] Publish normalization conformance fixtures

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/350
**Created:** 2026-09-15T18:17:43Z | **Updated:** 2026-09-15T18:17:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #41

### Original description

Parent: #41

Task ID: `DATA-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-351"></a>
## #351 — [TASK][DATA-02.T01] Implement authoritative exact retrieval

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/351
**Created:** 2026-09-15T18:18:04Z | **Updated:** 2026-09-15T18:18:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #42

### Original description

Parent: #42

Task ID: `DATA-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-352"></a>
## #352 — [TASK][DATA-02.T02] Build incremental full-text indexing

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/352
**Created:** 2026-09-15T18:18:08Z | **Updated:** 2026-09-15T18:18:08Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #42

### Original description

Parent: #42

Task ID: `DATA-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-353"></a>
## #353 — [TASK][DATA-02.T03] Add embedding and semantic candidate retrieval

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/353
**Created:** 2026-09-15T18:18:17Z | **Updated:** 2026-09-15T18:18:17Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #42

### Original description

Parent: #42

Task ID: `DATA-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-354"></a>
## #354 — [TASK][DATA-02.T04] Implement fusion and scoped reranking

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/354
**Created:** 2026-09-15T18:18:20Z | **Updated:** 2026-09-15T18:18:20Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #42

### Original description

Parent: #42

Task ID: `DATA-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-355"></a>
## #355 — [TASK][DATA-02.T05] Build evidence-preserving context bundles

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/355
**Created:** 2026-09-15T18:18:25Z | **Updated:** 2026-09-15T18:18:25Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #42

### Original description

Parent: #42

Task ID: `DATA-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-356"></a>
## #356 — [TASK][DATA-02.T06] Implement deletion, cache invalidation and rebuild

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/356
**Created:** 2026-09-15T18:18:31Z | **Updated:** 2026-09-15T18:18:31Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #42

### Original description

Parent: #42

Task ID: `DATA-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-357"></a>
## #357 — [TASK][DATA-02.T07] Measure quality and privacy independently

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/357
**Created:** 2026-09-15T18:18:36Z | **Updated:** 2026-09-15T18:18:36Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #42

### Original description

Parent: #42

Task ID: `DATA-02.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-358"></a>
## #358 — [TASK][DATA-03.T01] Define dependency edges and revision semantics

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/358
**Created:** 2026-09-15T18:18:53Z | **Updated:** 2026-09-15T18:18:53Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #43

### Original description

Parent: #43

Task ID: `DATA-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-359"></a>
## #359 — [TASK][DATA-03.T02] Record dependencies during computation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/359
**Created:** 2026-09-15T18:18:59Z | **Updated:** 2026-09-15T18:18:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #43

### Original description

Parent: #43

Task ID: `DATA-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-360"></a>
## #360 — [TASK][DATA-03.T03] Implement durable invalidation traversal

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/360
**Created:** 2026-09-15T18:19:05Z | **Updated:** 2026-09-15T18:19:05Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #43

### Original description

Parent: #43

Task ID: `DATA-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-361"></a>
## #361 — [TASK][DATA-03.T04] Implement type-specific refresh behavior

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/361
**Created:** 2026-09-15T18:19:09Z | **Updated:** 2026-09-15T18:19:09Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #43

### Original description

Parent: #43

Task ID: `DATA-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-362"></a>
## #362 — [TASK][DATA-03.T05] Invalidate approvals and transaction preconditions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/362
**Created:** 2026-09-15T18:19:13Z | **Updated:** 2026-09-15T18:19:13Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #43

### Original description

Parent: #43

Task ID: `DATA-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-363"></a>
## #363 — [TASK][DATA-03.T06] Propagate deletion through every derivative

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/363
**Created:** 2026-09-15T18:19:18Z | **Updated:** 2026-09-15T18:19:18Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #43

### Original description

Parent: #43

Task ID: `DATA-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-364"></a>
## #364 — [TASK][DATA-03.T07] Expose invalidation explanations and audit

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/364
**Created:** 2026-09-15T18:19:22Z | **Updated:** 2026-09-15T18:19:22Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #43

### Original description

Parent: #43

Task ID: `DATA-03.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-365"></a>
## #365 — [TASK][DATA-03.T08] Qualify graph convergence and recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/365
**Created:** 2026-09-15T18:19:35Z | **Updated:** 2026-09-15T18:19:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #43

### Original description

Parent: #43

Task ID: `DATA-03.T08`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-366"></a>
## #366 — [TASK][DATA-04.T01] Implement GoalContract editing and normalization

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/366
**Created:** 2026-09-15T18:20:03Z | **Updated:** 2026-09-15T18:20:03Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #44

### Original description

Parent: #44

Task ID: `DATA-04.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-367"></a>
## #367 — [TASK][DATA-04.T02] Build capability-aware source planning

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/367
**Created:** 2026-09-15T18:20:12Z | **Updated:** 2026-09-15T18:20:12Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #44

### Original description

Parent: #44

Task ID: `DATA-04.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-368"></a>
## #368 — [TASK][DATA-04.T03] Compile views from available typed data

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/368
**Created:** 2026-09-15T18:20:19Z | **Updated:** 2026-09-15T18:20:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #44

### Original description

Parent: #44

Task ID: `DATA-04.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-369"></a>
## #369 — [TASK][DATA-04.T04] Bind actions to exact capabilities

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/369
**Created:** 2026-09-15T18:20:26Z | **Updated:** 2026-09-15T18:20:26Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #44

### Original description

Parent: #44

Task ID: `DATA-04.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-370"></a>
## #370 — [TASK][DATA-04.T05] Compile independent verification requirements

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/370
**Created:** 2026-09-15T18:20:31Z | **Updated:** 2026-09-15T18:20:31Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #44

### Original description

Parent: #44

Task ID: `DATA-04.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-371"></a>
## #371 — [TASK][DATA-04.T06] Validate and atomically publish coordinated plans

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/371
**Created:** 2026-09-15T18:20:35Z | **Updated:** 2026-09-15T18:20:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #44

### Original description

Parent: #44

Task ID: `DATA-04.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-372"></a>
## #372 — [TASK][DATA-04.T07] Implement incremental recompilation and plan diffs

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/372
**Created:** 2026-09-15T18:20:41Z | **Updated:** 2026-09-15T18:20:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #44

### Original description

Parent: #44

Task ID: `DATA-04.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-373"></a>
## #373 — [TASK][DATA-04.T08] Test compiler correctness and useful diagnostics

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/373
**Created:** 2026-09-15T18:20:45Z | **Updated:** 2026-09-15T18:20:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #44

### Original description

Parent: #44

Task ID: `DATA-04.T08`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

