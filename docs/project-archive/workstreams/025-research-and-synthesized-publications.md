# Research and synthesized publications

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 29

## Contents

- [#25 — EPIC: Research and synthesized publications](#issue-25)
- [#75 — [P1][RES-01] Deep retrieval and exact-link discovery](#issue-75)
- [#76 — [P2][RES-02] Claim extraction, contradiction and ranking](#issue-76)
- [#77 — [P2][RES-03] Living articles and research artifacts](#issue-77)
- [#259 — [TASK][EPIC-RES.T01] Ratify research evidence and completion criteria](#issue-259)
- [#260 — [TASK][EPIC-RES.T02] Integrate discovery, claims and explainable ranking](#issue-260)
- [#261 — [TASK][EPIC-RES.T03] Integrate living publications and source updates](#issue-261)
- [#262 — [TASK][EPIC-RES.T04] Qualify factual support and second-visit usefulness](#issue-262)
- [#585 — [TASK][RES-01.T01] Compile constraints into a research plan](#issue-585)
- [#586 — [TASK][RES-01.T02] Implement permitted source discovery](#issue-586)
- [#587 — [TASK][RES-01.T03] Resolve canonical resources and exact links](#issue-587)
- [#588 — [TASK][RES-01.T04] Inspect and extract bounded source material](#issue-588)
- [#589 — [TASK][RES-01.T05] Cluster duplicated and derivative sources](#issue-589)
- [#590 — [TASK][RES-01.T06] Persist research progress and useful partial results](#issue-590)
- [#591 — [TASK][RES-01.T07] Evaluate exact-link and constraint performance](#issue-591)
- [#592 — [TASK][RES-02.T01] Define the claim and evidence-relation model](#issue-592)
- [#593 — [TASK][RES-02.T02] Implement constrained claim extraction](#issue-593)
- [#594 — [TASK][RES-02.T03] Align entities, variants and time scopes](#issue-594)
- [#595 — [TASK][RES-02.T04] Build contradiction and independence analysis](#issue-595)
- [#596 — [TASK][RES-02.T05] Implement explicit constraint filtering and ranking](#issue-596)
- [#597 — [TASK][RES-02.T06] Generate explanations and recompute on source change](#issue-597)
- [#598 — [TASK][RES-02.T07] Qualify claim support and ranking calibration](#issue-598)
- [#599 — [TASK][RES-03.T01] Define publication and revision records](#issue-599)
- [#600 — [TASK][RES-03.T02] Compile a source-backed outline and draft](#issue-600)
- [#601 — [TASK][RES-03.T03] Implement the reader and evidence controls](#issue-601)
- [#602 — [TASK][RES-03.T04] Implement stable annotations and revision diffs](#issue-602)
- [#603 — [TASK][RES-03.T05] Drive reviewable updates from source invalidation](#issue-603)
- [#604 — [TASK][RES-03.T06] Implement rights-aware export and sharing](#issue-604)
- [#605 — [TASK][RES-03.T07] Test publication continuity and factual regressions](#issue-605)

---

<a id="issue-25"></a>
## #25 — EPIC: Research and synthesized publications

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/25
**Created:** 2026-09-15T12:08:09Z | **Updated:** 2026-09-15T14:24:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #1 | **Body-declared parent:** #1
**Native child issues:** #75, #76, #77

### Original description

Programme: #1

Own deep research planning/retrieval, evidence/contradiction analysis and source-backed synthesized publications that become durable workspace artifacts rather than transient chat answers.

#### Child issues
- [ ] #75 RES-01 — Deep retrieval and exact-link discovery
- [ ] #76 RES-02 — Claim extraction, contradiction and ranking
- [ ] #77 RES-03 — Living articles and research artifacts

#### Cross-cutting gates
Canonical links and exact sources, source-independence checks, claim/evidence links, freshness and contradiction retention, attribution/caching rules, and explicit uncertainty/gaps.

### Discussion (1 comments)

#### Comment 5681891460 — Jordan-Hall — 2026-09-15T14:24:19Z

Source: https://github.com/Jordan-Hall/browser/issues/25#issuecomment-5681891460 | Updated: 2026-09-15T14:24:19Z

<!-- intent-implementation-v1:EPIC-RES -->
###### Workstream implementation and integration tasks

Integrate #75–#77 so research produces durable source-backed applications and publications, not only chat prose.

- [ ] **EPIC-RES.T01 — Ratify evidence/completion criteria.** Define exact-link validity, source independence, claim/evidence support, freshness, gaps and attribution requirements. **Proof:** a bounded research goal has explicit constraints and independently assessable completion criteria.
- [ ] **EPIC-RES.T02 — Integrate discovery/claims/ranking.** Connect source planning, permitted retrieval, entity resolution, evidence spans, contradictions and explainable ranking. **Proof:** snippets and syndicated copies cannot masquerade as independent primary evidence.
- [ ] **EPIC-RES.T03 — Integrate living publications/updates.** Render research as versioned articles/dossiers with annotations, provenance inspection and reviewable source-driven revisions. **Proof:** updates do not silently rewrite a passage being read or destroy annotations.
- [ ] **EPIC-RES.T04 — Qualify support and second-visit usefulness.** Run held-out research tasks and reopen results without chat/inference. **Proof:** important claims retain evidence or explicit uncertainty; saved views remain directly usable.

**Demonstration:** compare competing primary sources, retain a credible disagreement, publish a synthesized article, update one source and inspect the revision/evidence changes tomorrow.

**Review boundary:** a retrieval failure is an evidence gap, not permission to fabricate contents; ranking cannot be silently driven by affiliate incentives; access to an article does not imply unlimited redistribution rights. Close against research-quality, continuity and source-policy evidence.


---

<a id="issue-75"></a>
## #75 — [P1][RES-01] Deep retrieval and exact-link discovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/75
**Created:** 2026-09-15T12:16:25Z | **Updated:** 2026-09-15T19:45:53Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #25 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #25

#### Objective
Build a research retrieval pipeline that optimizes for useful exact sources and primary evidence rather than a traditional ranked page of links.

#### Scope
- Query decomposition from GoalContract constraints.
- Source planning across installed connectors, public web, domain APIs and user-authorized sources.
- Canonical URL/object resolution and exact-link validation.
- Retrieval logs including query/source/time/result disposition.
- Primary-source preference and independent-source grouping.
- Record inaccessible/failed sources as explicit evidence gaps.
- Product/entity identifier extraction for downstream entity resolution.
- Cache only within source-specific access/retention rules.

#### Research rules
- Search snippets are discovery hints, not sufficient evidence for material facts.
- Ten syndicated copies of one story count as one underlying information source where detected.
- Retrieval breadth does not override privacy/account permissions.

#### Acceptance criteria
- [ ] Material results include accessible canonical source links/object IDs where available.
- [ ] Important facts are grounded in underlying evidence, not snippets alone.
- [ ] Primary sources are preferred/explained when available.
- [ ] Inaccessible sources remain explicit gaps rather than hallucinated contents.
- [ ] Duplicate/syndicated source groups are tracked.
- [ ] Research fixture scores exact-link validity, constraint coverage, source quality and missing-source honesty.

#### Dependencies
- CONN-01
- DATA-01

**First phase:** P1  
**Maturity target:** P2  
**Owner:** connectors-domains

### Discussion (2 comments)

#### Comment 5682278484 — Jordan-Hall — 2026-09-15T14:45:04Z

Source: https://github.com/Jordan-Hall/browser/issues/75#issuecomment-5682278484 | Updated: 2026-09-15T14:45:04Z

<!-- intent-implementation-v1:RES-01 -->
###### Implementation proposal — RES-01

Implement bounded, checkpointed research discovery over #45/#41. Store ResearchPlan, QueryIntent, RetrievalAttempt, CanonicalResource, SourceCluster and explicit ResearchGap records.

- [ ] **RES-01.T01 — Research plan.** Translate GoalContract into answerable questions, mandatory facts, exclusions, source classes and budgets; preserve ambiguous constraints for review. **Verify:** every retrieval step serves an explicit requirement rather than open-ended browsing.
- [ ] **RES-01.T02 — Permitted discovery.** Fan out bounded search/API/feed/local queries after account/destination checks; deduplicate requests and enforce quotas. **Verify:** search terms cannot leak private context to an unauthorized service and snippets remain discovery hints.
- [ ] **RES-01.T03 — Exact-link resolution.** Follow permitted redirects with egress checks, preserve original/final URLs and meaningful variant parameters, validate content/object identity and detect login walls/soft-404s. **Verify:** a reachable but unrelated page does not count as an exact result.
- [ ] **RES-01.T04 — Source inspection.** Retrieve bounded text/structured records through isolated connector/browser/parser workers and preserve versions/hashes/evidence spans. **Verify:** important facts come from inspected material, not invented inaccessible contents.
- [ ] **RES-01.T05 — Source independence.** Cluster canonical/exact/near-duplicate material and explicit derivation links, retaining clustering evidence and corrections. **Verify:** syndicated copies cannot count as independent corroboration and originals remain accessible.
- [ ] **RES-01.T06 — Durable partial results.** Persist query disposition, evidence, gaps and budget use; publish source-backed entities incrementally and revalidate resumed work. **Verify:** restart retains useful results without repeating obsolete or unauthorized requests.
- [ ] **RES-01.T07 — Retrieval evaluation.** Use held-out exact records, inaccessible sources, duplicate reporting, hostile pages and stale offers. **Verify:** compare exact-link validity, constraint coverage, independent evidence and cost per useful result against baseline search.

**Critical detail:** canonicalization must not blindly strip query strings containing meaningful variants or access state. Signed access URLs require careful private handling, not automatic publication. Purchase-critical facts require underlying evidence; failures stay visible as gaps. Better links than ordinary search remain a hypothesis to demonstrate.

#### Comment 5687117151 — Jordan-Hall — 2026-09-15T19:45:52Z

Source: https://github.com/Jordan-Hall/browser/issues/75#issuecomment-5687117151 | Updated: 2026-09-15T19:45:52Z

###### Task issues
- [ ] #585 `RES-01.T01` — Compile constraints into a research plan
- [ ] #586 `RES-01.T02` — Implement permitted source discovery
- [ ] #587 `RES-01.T03` — Resolve canonical resources and exact links
- [ ] #588 `RES-01.T04` — Inspect and extract bounded source material
- [ ] #589 `RES-01.T05` — Cluster duplicated and derivative sources
- [ ] #590 `RES-01.T06` — Persist research progress and useful partial results
- [ ] #591 `RES-01.T07` — Evaluate exact-link and constraint performance


---

<a id="issue-76"></a>
## #76 — [P2][RES-02] Claim extraction, contradiction and ranking

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/76
**Created:** 2026-09-15T12:16:36Z | **Updated:** 2026-09-15T19:46:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #25 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #25

#### Objective
Transform retrieved sources into evidence-backed claims, contradictions and explainable rankings without flattening uncertainty or duplicated reporting.

#### Scope
- Claim extraction with bounded evidence spans and source/revision/time references.
- Distinguish fact, source assertion, interpretation and inference.
- Resolve entity/variant/time scope before merging claims.
- Source-independence and syndication/derivation detection.
- Contradiction clusters with evidence from credible competing sources.
- Confidence/uncertainty representation based on evidence quality and gaps.
- Ranking against explicit user constraints with explainable factors and commercial-conflict flags.
- Recompute/invalidate through DATA-03 when source state changes.

#### Research rules
- Model agreement does not replace source evidence.
- Contradictory credible evidence remains visible rather than being averaged away.
- Ranking should optimize explicit user goals, not affiliate/engagement incentives.

#### Acceptance criteria
- [ ] Every material claim can enumerate supporting/contradicting evidence spans.
- [ ] Duplicated/syndicated coverage is not counted as independent corroboration.
- [ ] Variant/time mismatches are detected before claims are merged.
- [ ] Ranking explanations expose constraints, missing facts and commercial conflicts.
- [ ] Source changes invalidate/recompute affected claims/rankings.
- [ ] Evaluation measures evidence support, contradiction retention and ranking constraint satisfaction separately.

#### Dependencies
- RES-01
- DATA-02
- DATA-03

**First phase:** P2  
**Maturity target:** P5  
**Owner:** connectors-domains

### Discussion (2 comments)

#### Comment 5682284520 — Jordan-Hall — 2026-09-15T14:45:22Z

Source: https://github.com/Jordan-Hall/browser/issues/76#issuecomment-5682284520 | Updated: 2026-09-15T14:45:22Z

<!-- intent-implementation-v1:RES-02 -->
###### Implementation proposal — RES-02

Build claim/evidence analysis and deterministic user-directed ranking over #75/#42/#43. Confidence is an evidence assessment unless probability calibration has actually been measured.

- [ ] **RES-02.T01 — Claim relations.** Represent subject/predicate/object, units, qualifiers and time separately from prose, with supports/contradicts/qualifies/unresolved evidence relations. **Verify:** each claim is tied to exact eligible source spans.
- [ ] **RES-02.T02 — Constrained extraction.** Generate schema-validated candidates through permitted inference, validate offsets and normalize units deterministically; record extractor/model versions. **Verify:** fabricated or mismatched spans cannot become accepted evidence.
- [ ] **RES-02.T03 — Entity/variant/time alignment.** Resolve source-backed identities and compare condition, variant, unit and date before merging. **Verify:** different variants or prices at different times are not falsely treated as one contradictory fact.
- [ ] **RES-02.T04 — Contradiction/independence analysis.** Group comparable incompatible assertions, retain independent-source metadata and explicit corrections/supersession. **Verify:** credible alternatives remain visible rather than averaged away or outvoted by syndicated copies.
- [ ] **RES-02.T05 — Constraints and ranking.** Evaluate hard requirements as satisfied/violated/unknown, then rank with versioned user factors and deterministic tie-breakers. **Verify:** missing facts are not interpreted favorably and commercial metadata cannot secretly influence scoring.
- [ ] **RES-02.T06 — Faithful explanations/updates.** Generate why-ranked details from actual scoring/evidence traces, showing gaps, contradictions and freshness; recompute affected dependencies only. **Verify:** explanations match the implemented ranking rather than post-hoc model rationalization.
- [ ] **RES-02.T07 — Quality/calibration tests.** Use human-reviewed entailment, identity, contradiction and constraint fixtures. **Verify:** report support accuracy, omissions, false merges and explanation fidelity separately; do not invent calibrated confidence.

**Definition of done:** important claims have supporting or contradicting evidence, actual ranking factors are inspectable and source revisions invalidate affected outputs. Multiple models agreeing is not independent factual verification. User personalization changes relevance, not the source evidence.

#### Comment 5687127956 — Jordan-Hall — 2026-09-15T19:46:45Z

Source: https://github.com/Jordan-Hall/browser/issues/76#issuecomment-5687127956 | Updated: 2026-09-15T19:46:45Z

###### Task issues
- [ ] #592 `RES-02.T01` — Define the claim and evidence-relation model
- [ ] #593 `RES-02.T02` — Implement constrained claim extraction
- [ ] #594 `RES-02.T03` — Align entities, variants and time scopes
- [ ] #595 `RES-02.T04` — Build contradiction and independence analysis
- [ ] #596 `RES-02.T05` — Implement explicit constraint filtering and ranking
- [ ] #597 `RES-02.T06` — Generate explanations and recompute on source change
- [ ] #598 `RES-02.T07` — Qualify claim support and ranking calibration


---

<a id="issue-77"></a>
## #77 — [P2][RES-03] Living articles and research artifacts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/77
**Created:** 2026-09-15T12:16:59Z | **Updated:** 2026-09-15T19:47:34Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #25 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #25

#### Objective
Turn research into durable, evidence-backed synthesized publications that can evolve as sources change without masquerading as original publisher pages.

#### Scope
- Article/research artifact schema with claim/evidence graph, source list, publication/observation dates and unresolved gaps.
- User-selectable depth/format and section organization.
- Inline provenance/contradiction/uncertainty controls.
- Revision history, diff, annotations, pin/lock and reviewable updates.
- Export/share with source-aware quotation/caching constraints.
- Change notifications driven by source dependency invalidation.
- Original-source navigation from claims/passages.

#### Product rules
- Generated publication is clearly labeled synthesis.
- Material factual/purchase-critical claims require evidence or explicit uncertainty.
- New source data proposes/creates a revision; it does not silently rewrite text being read.

#### Acceptance criteria
- [ ] Every material claim links to evidence or is explicitly marked unresolved/inferred.
- [ ] Contradictions remain visible in the synthesized artifact.
- [ ] Source updates create reviewable revisions with meaningful diffs.
- [ ] User annotations survive regenerated revisions where anchors remain valid or are surfaced for repair.
- [ ] Export respects source access/quotation/redaction policy.
- [ ] User can open originals from the relevant claim/evidence context.

#### Dependencies
- RES-02
- UI-02

**First phase:** P2  
**Maturity target:** P5  
**Owner:** connectors-domains

### Discussion (2 comments)

#### Comment 5682290193 — Jordan-Hall — 2026-09-15T14:45:40Z

Source: https://github.com/Jordan-Hall/browser/issues/77#issuecomment-5682290193 | Updated: 2026-09-15T14:45:40Z

<!-- intent-implementation-v1:RES-03 -->
###### Implementation proposal — RES-03

Implement durable publications over #76/#50 using immutable PublicationRevision records, stable section/claim anchors and separate annotations. The current approved revision and pending updates are distinct.

- [ ] **RES-03.T01 — Publication records.** Store goal, outline, sources/revisions, claims, gaps and display preferences with stable identity. **Verify:** model changes do not replace the article's identity or user annotations.
- [ ] **RES-03.T02 — Evidence-backed draft.** Compile an outline from the goal/claim graph; generate only from eligible evidence and bind factual passages to claim IDs. **Verify:** conflicts, interpretation and unsupported hypotheses are explicit rather than hidden in polished prose.
- [ ] **RES-03.T03 — Reader/evidence controls.** Render adjustable depth/typography, tables, source badges, contradictions and Original links with stable focus/reading position. **Verify:** keyboard/screen-reader access and saved reading work without active generation.
- [ ] **RES-03.T04 — Annotation/diff handling.** Anchor notes to claims/sections with text-context fallbacks; show added/changed/removed/unsupported claims. **Verify:** ambiguous relocated notes require repair rather than silently attaching to the wrong passage.
- [ ] **RES-03.T05 — Reviewable updates.** Subscribe to evidence invalidation, mark impacted passages stale and prepare revision proposals under the user's update policy. **Verify:** ordinary updates do not replace a paragraph being read; mandatory access revocation still removes restricted content.
- [ ] **RES-03.T06 — Rights-aware export.** Export selected revisions/notes with citations, synthesis labels and redaction; enforce quotation/caching/sharing policies. **Verify:** inaccessible source bodies and hidden indexes do not leak in exported files.
- [ ] **RES-03.T07 — Continuity/factual regression.** Test corrections, deletions, conflict, interruption, provider changes, export and annotation repair, plus sampled human fidelity/readability review. **Verify:** generated changes retain source support and user-owned state.

**Product boundary:** label the page as a synthesis, not an original publisher article; do not imitate branding to imply authorship. Regenerate sections from explicit dependency sets rather than rewriting the entire publication on every interaction. Closure requires revision, source-policy, accessibility and evidence-fidelity tests.

#### Comment 5687138427 — Jordan-Hall — 2026-09-15T19:47:34Z

Source: https://github.com/Jordan-Hall/browser/issues/77#issuecomment-5687138427 | Updated: 2026-09-15T19:47:34Z

###### Task issues
- [ ] #599 `RES-03.T01` — Define publication and revision records
- [ ] #600 `RES-03.T02` — Compile a source-backed outline and draft
- [ ] #601 `RES-03.T03` — Implement the reader and evidence controls
- [ ] #602 `RES-03.T04` — Implement stable annotations and revision diffs
- [ ] #603 `RES-03.T05` — Drive reviewable updates from source invalidation
- [ ] #604 `RES-03.T06` — Implement rights-aware export and sharing
- [ ] #605 `RES-03.T07` — Test publication continuity and factual regressions


---

<a id="issue-259"></a>
## #259 — [TASK][EPIC-RES.T01] Ratify research evidence and completion criteria

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/259
**Created:** 2026-09-15T15:29:24Z | **Updated:** 2026-09-15T15:29:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #25

### Original description

Parent: #25

Task ID: `EPIC-RES.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-260"></a>
## #260 — [TASK][EPIC-RES.T02] Integrate discovery, claims and explainable ranking

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/260
**Created:** 2026-09-15T15:29:31Z | **Updated:** 2026-09-15T15:29:31Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #25

### Original description

Parent: #25

Task ID: `EPIC-RES.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-261"></a>
## #261 — [TASK][EPIC-RES.T03] Integrate living publications and source updates

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/261
**Created:** 2026-09-15T15:29:38Z | **Updated:** 2026-09-15T15:29:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #25

### Original description

Parent: #25

Task ID: `EPIC-RES.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-262"></a>
## #262 — [TASK][EPIC-RES.T04] Qualify factual support and second-visit usefulness

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/262
**Created:** 2026-09-15T15:29:48Z | **Updated:** 2026-09-15T15:29:48Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #25

### Original description

Parent: #25

Task ID: `EPIC-RES.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-585"></a>
## #585 — [TASK][RES-01.T01] Compile constraints into a research plan

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/585
**Created:** 2026-09-15T19:45:14Z | **Updated:** 2026-09-15T19:45:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #75

### Original description

Parent: #75

Task ID: `RES-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-586"></a>
## #586 — [TASK][RES-01.T02] Implement permitted source discovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/586
**Created:** 2026-09-15T19:45:21Z | **Updated:** 2026-09-15T19:45:21Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #75

### Original description

Parent: #75

Task ID: `RES-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-587"></a>
## #587 — [TASK][RES-01.T03] Resolve canonical resources and exact links

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/587
**Created:** 2026-09-15T19:45:27Z | **Updated:** 2026-09-15T19:45:27Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #75

### Original description

Parent: #75

Task ID: `RES-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-588"></a>
## #588 — [TASK][RES-01.T04] Inspect and extract bounded source material

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/588
**Created:** 2026-09-15T19:45:31Z | **Updated:** 2026-09-15T19:45:31Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #75

### Original description

Parent: #75

Task ID: `RES-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-589"></a>
## #589 — [TASK][RES-01.T05] Cluster duplicated and derivative sources

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/589
**Created:** 2026-09-15T19:45:36Z | **Updated:** 2026-09-15T19:45:36Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #75

### Original description

Parent: #75

Task ID: `RES-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-590"></a>
## #590 — [TASK][RES-01.T06] Persist research progress and useful partial results

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/590
**Created:** 2026-09-15T19:45:41Z | **Updated:** 2026-09-15T19:45:41Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #75

### Original description

Parent: #75

Task ID: `RES-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-591"></a>
## #591 — [TASK][RES-01.T07] Evaluate exact-link and constraint performance

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/591
**Created:** 2026-09-15T19:45:46Z | **Updated:** 2026-09-15T19:45:46Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #75

### Original description

Parent: #75

Task ID: `RES-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-592"></a>
## #592 — [TASK][RES-02.T01] Define the claim and evidence-relation model

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/592
**Created:** 2026-09-15T19:46:00Z | **Updated:** 2026-09-15T19:46:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #76

### Original description

Parent: #76

Task ID: `RES-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-593"></a>
## #593 — [TASK][RES-02.T02] Implement constrained claim extraction

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/593
**Created:** 2026-09-15T19:46:05Z | **Updated:** 2026-09-15T19:46:05Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #76

### Original description

Parent: #76

Task ID: `RES-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-594"></a>
## #594 — [TASK][RES-02.T03] Align entities, variants and time scopes

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/594
**Created:** 2026-09-15T19:46:10Z | **Updated:** 2026-09-15T19:46:10Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #76

### Original description

Parent: #76

Task ID: `RES-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-595"></a>
## #595 — [TASK][RES-02.T04] Build contradiction and independence analysis

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/595
**Created:** 2026-09-15T19:46:15Z | **Updated:** 2026-09-15T19:46:15Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #76

### Original description

Parent: #76

Task ID: `RES-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-596"></a>
## #596 — [TASK][RES-02.T05] Implement explicit constraint filtering and ranking

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/596
**Created:** 2026-09-15T19:46:22Z | **Updated:** 2026-09-15T19:46:22Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #76

### Original description

Parent: #76

Task ID: `RES-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-597"></a>
## #597 — [TASK][RES-02.T06] Generate explanations and recompute on source change

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/597
**Created:** 2026-09-15T19:46:33Z | **Updated:** 2026-09-15T19:46:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #76

### Original description

Parent: #76

Task ID: `RES-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-598"></a>
## #598 — [TASK][RES-02.T07] Qualify claim support and ranking calibration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/598
**Created:** 2026-09-15T19:46:38Z | **Updated:** 2026-09-15T19:46:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #76

### Original description

Parent: #76

Task ID: `RES-02.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-599"></a>
## #599 — [TASK][RES-03.T01] Define publication and revision records

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/599
**Created:** 2026-09-15T19:46:51Z | **Updated:** 2026-09-15T19:46:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #77

### Original description

Parent: #77

Task ID: `RES-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-600"></a>
## #600 — [TASK][RES-03.T02] Compile a source-backed outline and draft

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/600
**Created:** 2026-09-15T19:46:57Z | **Updated:** 2026-09-15T19:46:57Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #77

### Original description

Parent: #77

Task ID: `RES-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-601"></a>
## #601 — [TASK][RES-03.T03] Implement the reader and evidence controls

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/601
**Created:** 2026-09-15T19:47:03Z | **Updated:** 2026-09-15T19:47:03Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #77

### Original description

Parent: #77

Task ID: `RES-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-602"></a>
## #602 — [TASK][RES-03.T04] Implement stable annotations and revision diffs

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/602
**Created:** 2026-09-15T19:47:09Z | **Updated:** 2026-09-15T19:47:09Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #77

### Original description

Parent: #77

Task ID: `RES-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-603"></a>
## #603 — [TASK][RES-03.T05] Drive reviewable updates from source invalidation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/603
**Created:** 2026-09-15T19:47:14Z | **Updated:** 2026-09-15T19:47:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #77

### Original description

Parent: #77

Task ID: `RES-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-604"></a>
## #604 — [TASK][RES-03.T06] Implement rights-aware export and sharing

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/604
**Created:** 2026-09-15T19:47:22Z | **Updated:** 2026-09-15T19:47:22Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #77

### Original description

Parent: #77

Task ID: `RES-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-605"></a>
## #605 — [TASK][RES-03.T07] Test publication continuity and factual regressions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/605
**Created:** 2026-09-15T19:47:27Z | **Updated:** 2026-09-15T19:47:27Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #77

### Original description

Parent: #77

Task ID: `RES-03.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

