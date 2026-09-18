# Social, communications and calendars

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 36

## Contents

- [#28 — EPIC: Social, communications and calendars](#issue-28)
- [#85 — [P2][SOC-01] Unified feeds and cross-network normalization](#issue-85)
- [#86 — [P2][SOC-02] User-controlled feed ranking](#issue-86)
- [#87 — [P4][SOC-03] Social posting, replies, cross-posting and messaging](#issue-87)
- [#88 — [P2][SOC-04] Email, calendar, contacts and project actions](#issue-88)
- [#271 — [TASK][EPIC-SOC.T01] Ratify source, identity and audience contracts](#issue-271)
- [#272 — [TASK][EPIC-SOC.T02] Integrate unified read views and transparent ranking](#issue-272)
- [#273 — [TASK][EPIC-SOC.T03] Integrate destination-correct write workflows](#issue-273)
- [#274 — [TASK][EPIC-SOC.T04] Qualify mixed-account semantics and access](#issue-274)
- [#655 — [TASK][SOC-01.T01] Define source-preserving social records](#issue-655)
- [#656 — [TASK][SOC-01.T02] Implement authenticated feed ingestion](#issue-656)
- [#657 — [TASK][SOC-01.T03] Build thread reconstruction and cross-post clustering](#issue-657)
- [#658 — [TASK][SOC-01.T04] Implement verified identity linking and corrections](#issue-658)
- [#659 — [TASK][SOC-01.T05] Propagate deletion, visibility and moderation changes](#issue-659)
- [#660 — [TASK][SOC-01.T06] Build the stable unified feed view](#issue-660)
- [#661 — [TASK][SOC-01.T07] Qualify multi-network privacy and continuity](#issue-661)
- [#662 — [TASK][SOC-02.T01] Specify ranking policy and chronological semantics](#issue-662)
- [#663 — [TASK][SOC-02.T02] Extract bounded relevance features](#issue-663)
- [#664 — [TASK][SOC-02.T03] Implement scoring and constraint evaluation](#issue-664)
- [#665 — [TASK][SOC-02.T04] Build ranking controls and explanations](#issue-665)
- [#666 — [TASK][SOC-02.T05] Integrate scoped preferences and correction](#issue-666)
- [#667 — [TASK][SOC-02.T06] Evaluate goal satisfaction and ranking agency](#issue-667)
- [#668 — [TASK][SOC-03.T01] Define action schemas per network capability](#issue-668)
- [#669 — [TASK][SOC-03.T02] Implement durable draft and destination composition](#issue-669)
- [#670 — [TASK][SOC-03.T03] Stage attachments with scoped uploads](#issue-670)
- [#671 — [TASK][SOC-03.T04] Revalidate reply targets and create approvals](#issue-671)
- [#672 — [TASK][SOC-03.T05] Dispatch cross-post groups with partial results](#issue-672)
- [#673 — [TASK][SOC-03.T06] Implement messaging, deletion and derived-state updates](#issue-673)
- [#674 — [TASK][SOC-03.T07] Qualify account, audience and retry safety](#issue-674)
- [#675 — [TASK][SOC-04.T01] Define domain packs and account-scoped views](#issue-675)
- [#676 — [TASK][SOC-04.T02] Implement recipient and contact resolution](#issue-676)
- [#677 — [TASK][SOC-04.T03] Implement email drafts, replies, forwards and sends](#issue-677)
- [#678 — [TASK][SOC-04.T04] Implement calendar reads and conflict checking](#issue-678)
- [#679 — [TASK][SOC-04.T05] Implement calendar writes and invitation responses](#issue-679)
- [#680 — [TASK][SOC-04.T06] Implement project-specific actions](#issue-680)
- [#681 — [TASK][SOC-04.T07] Qualify cross-service work and privacy](#issue-681)

---

<a id="issue-28"></a>
## #28 — EPIC: Social, communications and calendars

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/28
**Created:** 2026-09-15T12:08:28Z | **Updated:** 2026-09-15T14:25:05Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #1 | **Body-declared parent:** #1
**Native child issues:** #85, #86, #87, #88

### Original description

Programme: #1

Own cross-network feed normalization, transparent user-controlled ranking, destination-correct social writes, and email/calendar/contact/project actions.

#### Child issues
- [ ] #85 SOC-01 — Unified feeds and cross-network normalization
- [ ] #86 SOC-02 — User-controlled feed ranking
- [ ] #87 SOC-03 — Social posting, replies, cross-posting and messaging
- [ ] #88 SOC-04 — Email, calendar, contacts and project actions

#### Cross-cutting gates
Identities are never merged from display-name similarity alone, provider visibility/deletion/block semantics propagate, destination account/audience is explicit, and normalized actions never erase important service-specific meaning.

### Discussion (1 comments)

#### Comment 5681906061 — Jordan-Hall — 2026-09-15T14:25:05Z

Source: https://github.com/Jordan-Hall/browser/issues/28#issuecomment-5681906061 | Updated: 2026-09-15T14:25:05Z

<!-- intent-implementation-v1:EPIC-SOC -->
###### Workstream implementation and integration tasks

Integrate #85–#88 without erasing account, audience, thread or provider-specific action meaning.

- [ ] **EPIC-SOC.T01 — Ratify identity/audience contracts.** Preserve canonical network/account IDs, revisions, visibility, thread links, moderation labels and blocks. **Proof:** similar names or avatars never automatically merge people; public replies and private messages remain distinct capabilities.
- [ ] **EPIC-SOC.T02 — Integrate read views/ranking.** Combine feeds, messages, calendars and project records into persistent source-backed views with chronological and explicit user ranking controls. **Proof:** account restrictions and deletions propagate; ordinary navigation needs no inference.
- [ ] **EPIC-SOC.T03 — Integrate destination-correct writes.** Preview account, recipient/audience, text, attachments and reply/event targets; use separate journalled transactions for each destination. **Proof:** a partial cross-post cannot silently retry destinations that already succeeded.
- [ ] **EPIC-SOC.T04 — Qualify mixed-account access.** Test revocation, stale threads, private-message handling, email recipients, calendar timezones/conflicts and provider outages. **Proof:** no private-cache leakage or audience widening, with truthful unsupported-operation handoff.

**Demonstration:** correlate a topic across two networks, inspect originals, reply from the intended account, draft a related email and calendar action, and preserve each service's semantics.

**Closure evidence:** provider-specific conformance and rights/access records, not merely normalized UI screenshots. Shared representation never gives collaborators the union of connected-account permissions.


---

<a id="issue-85"></a>
## #85 — [P2][SOC-01] Unified feeds and cross-network normalization

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/85
**Created:** 2026-09-15T12:18:39Z | **Updated:** 2026-09-15T20:58:12Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #28 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #28

#### Objective
Create one user-owned social/feed workspace across supported networks while preserving each provider's identity, thread, visibility, moderation and deletion semantics.

#### Scope
- Canonical post/thread/account records with original provider IDs and revisions.
- Multi-account/network feed ingestion through connectors.
- Deduplication/cross-post correlation without destroying original records.
- Explicit verified/uncertain cross-network identity links.
- Thread/parent/reply relationships and attachment metadata.
- Visibility/audience, labels/moderation, blocks/mutes and action availability.
- Tombstone/deletion/visibility-change propagation into derived views/indexes.
- Original/evidence navigation.

#### Data rules
- Display-name/avatar similarity is never sufficient to merge identities.
- Normalized feed is a view, not the source of truth.
- Provider semantics remain attached to each record/action.

#### Acceptance criteria
- [ ] Similar-name adversarial accounts remain distinct unless explicitly verified/linked.
- [ ] Deleted/private/blocked/muted provider records propagate appropriately into derived views.
- [ ] Cross-post deduplication retains access to each original source/network.
- [ ] Thread structure remains correct across refreshes.
- [ ] Account/network/visibility is visible for each item.
- [ ] Feed remains usable as a deterministic workspace without an active model call.

#### Dependencies
- CONN-01
- DATA-01
- UI-02

**First phase:** P2  
**Maturity target:** P5  
**Owner:** connectors-domains

### Discussion (2 comments)

#### Comment 5682354091 — Jordan-Hall — 2026-09-15T14:49:00Z

Source: https://github.com/Jordan-Hall/browser/issues/85#issuecomment-5682354091 | Updated: 2026-09-15T14:49:00Z

<!-- intent-implementation-v1:SOC-01 -->
###### Implementation proposal — SOC-01

Implement a stable unified feed over #45/#41/#50 while retaining each provider's account, visibility, thread and moderation semantics. Normalization creates a view, not a new source of truth.

- [ ] **SOC-01.T01 — Source-preserving records.** Model author and viewer accounts separately, posts, threads, media, visibility, labels, blocks/mutes and supported actions with provider extensions. **Verify:** identities and audiences survive normalization unchanged.
- [ ] **SOC-01.T02 — Authenticated ingestion.** Persist paging/event cursors with observations and deduplicate delivery; bound media loading and egress. **Verify:** private content cannot enter another account's feed/cache.
- [ ] **SOC-01.T03 — Threads/cross-post clusters.** Preserve original reply/repost/parent links and cluster possible copies from identifiers, canonical links and evidence. **Verify:** every original remains inspectable and deleted parents do not corrupt threads.
- [ ] **SOC-01.T04 — Identity links/correction.** Accept verified links or explicit evidence-backed user associations with confidence/provenance and reversible unlinking. **Verify:** matching display names/avatars never automatically merge people.
- [ ] **SOC-01.T05 — Deletion/access/moderation.** Apply known tombstones, visibility changes, blocks/mutes and labels before presentation/retrieval; invalidate derived feeds/indexes. **Verify:** revoked private content cannot remain visible merely to preserve scroll stability.
- [ ] **SOC-01.T06 — Stable feed UI.** Render source/account/audience and Original navigation with supported actions, preserving focus/scroll under normal updates. **Verify:** source-only and model-off views remain useful.
- [ ] **SOC-01.T07 — Multi-network qualification.** Test uncertain identities, duplicate posts, private threads, missing parents, blocks, revocation and outages across multiple accounts. **Verify:** provider-specific fixtures justify support claims before live rollout.

**Connector starting points:** documented client surfaces such as [Bluesky](https://docs.bsky.app/docs/starter-templates/custom-feeds), [Mastodon](https://docs.joinmastodon.org/client/intro/) and feed formats; other providers require actual access review. Federation does not make private content public or guarantee global deletion delivery. Apply local revocation as soon as known and state retention limitations honestly.

#### Comment 5687989167 — Jordan-Hall — 2026-09-15T20:58:12Z

Source: https://github.com/Jordan-Hall/browser/issues/85#issuecomment-5687989167 | Updated: 2026-09-15T20:58:12Z

###### Task issues

- [ ] #655 `SOC-01.T01` — Define source-preserving social records
- [ ] #656 `SOC-01.T02` — Implement authenticated feed ingestion
- [ ] #657 `SOC-01.T03` — Build thread reconstruction and cross-post clustering
- [ ] #658 `SOC-01.T04` — Implement verified identity linking and corrections
- [ ] #659 `SOC-01.T05` — Propagate deletion, visibility and moderation changes
- [ ] #660 `SOC-01.T06` — Build the stable unified feed view
- [ ] #661 `SOC-01.T07` — Qualify multi-network privacy and continuity

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-86"></a>
## #86 — [P2][SOC-02] User-controlled feed ranking

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/86
**Created:** 2026-09-15T12:18:49Z | **Updated:** 2026-09-15T20:58:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #28 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #28

#### Objective
Replace opaque engagement optimization with inspectable user-controlled ranking over the unified feed.

#### Scope
- Chronological mode as a baseline.
- Source/account inclusion and weighting controls.
- Explicit interests, topic limits, recency, chosen diversity/exploration and muted themes.
- Ranking explanations: why this item appeared and which controls affected it.
- Persistent per-workspace/profile ranking preferences.
- Optional learned preferences only through MEM-01/MEM-02 rules.
- Deterministic reranking where possible; model-assisted classification remains evidence/feature input rather than hidden authority.
- A/B/evaluation against stated user goals, not session duration.

#### Product rules
- Personalization changes relevance/presentation, not factual evidence.
- Users can always switch to chronological/source-specific views.
- Commercial relationships cannot silently influence ranking.

#### Acceptance criteria
- [ ] User can inspect/change ranking controls and immediately see the effect.
- [ ] Every ranked item can expose a concise “why shown” explanation.
- [ ] Chronological mode behaves predictably without inference.
- [ ] Ranking preferences do not suppress source/evidence visibility or alter provider access rights.
- [ ] Inferred preferences follow MEM-02 consent/sensitivity rules.
- [ ] Ranking evaluation includes goal satisfaction and corrective interventions, not engagement-only metrics.

#### Dependencies
- SOC-01
- MEM-01

**First phase:** P2  
**Maturity target:** P5  
**Owner:** connectors-domains

### Discussion (2 comments)

#### Comment 5682360384 — Jordan-Hall — 2026-09-15T14:49:18Z

Source: https://github.com/Jordan-Hall/browser/issues/86#issuecomment-5682360384 | Updated: 2026-09-15T14:49:18Z

<!-- intent-implementation-v1:SOC-02 -->
###### Implementation proposal — SOC-02

Implement versioned user-owned ranking over #85/#89. Keep source access/block/mute enforcement ahead of ranking and preserve a deterministic chronological baseline.

- [ ] **SOC-02.T01 — Ranking/time semantics.** Define sources, topic limits, weights, exclusions and stable tie-breaking; distinguish original-post, repost and observation time. **Verify:** the displayed chronological mode follows its declared clock semantics.
- [ ] **SOC-02.T02 — Bounded relevance features.** Derive metadata and optional topic classifications with provenance/model versions and sensitivity checks. **Verify:** private content is never sent to an unapproved classifier/reranker destination.
- [ ] **SOC-02.T03 — Scoring/constraints.** Apply access and moderation filters first, then explicit ranking/diversity rules, recording each factor's actual contribution. **Verify:** a ranking override cannot restore a blocked or inaccessible post.
- [ ] **SOC-02.T04 — Controls/explanations.** Expose chronological/source/interest/exploration choices with immediate preview; generate why-shown text from the executed trace. **Verify:** explanations faithfully reflect scoring, not an independent invented rationale.
- [ ] **SOC-02.T05 — Preferences/correction.** Persist workspace/profile policies through #89, apply optional #90 suggestions only under consent and support locks/reset. **Verify:** inferred preferences cannot overwrite an explicit chosen feed rule.
- [ ] **SOC-02.T06 — Agency evaluation.** Measure relevant coverage, repetition, corrections and user understanding against chronological/source-only baselines. **Verify:** hidden commercial weighting and engagement-only optimization are excluded from default scoring.

**Definition of done:** the user can understand and change why content appears, restore predictable ordering and keep using saved feeds without inference. Diversity/exploration should be understandable controls, not a hidden replacement recommender objective. Personalization changes relevance, not factual evidence or provider access rights.

#### Comment 5687997398 — Jordan-Hall — 2026-09-15T20:58:54Z

Source: https://github.com/Jordan-Hall/browser/issues/86#issuecomment-5687997398 | Updated: 2026-09-15T20:58:54Z

###### Task issues

- [ ] #662 `SOC-02.T01` — Specify ranking policy and chronological semantics
- [ ] #663 `SOC-02.T02` — Extract bounded relevance features
- [ ] #664 `SOC-02.T03` — Implement scoring and constraint evaluation
- [ ] #665 `SOC-02.T04` — Build ranking controls and explanations
- [ ] #666 `SOC-02.T05` — Integrate scoped preferences and correction
- [ ] #667 `SOC-02.T06` — Evaluate goal satisfaction and ranking agency

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-87"></a>
## #87 — [P4][SOC-03] Social posting, replies, cross-posting and messaging

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/87
**Created:** 2026-09-15T12:19:01Z | **Updated:** 2026-09-15T20:59:40Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #28 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #28

#### Objective
Enable supported social interactions from the unified workspace while preserving destination-specific account, audience, thread and message semantics.

#### Scope
- Typed actions for post, reply, repost/share, reaction, delete where supported.
- Cross-post composition with per-destination account/audience/content/attachment preview.
- Private/direct messaging only through explicit provider-specific capabilities.
- Thread/reply target validation immediately before commit.
- Attachment handling, size/type restrictions and upload progress.
- Provider moderation/visibility constraints and action availability.
- Trusted TX-01 approval where policy requires and provider receipt verification.
- CONN-04 support/access matrix per operation/network.

#### Correctness rules
- Generic normalization cannot turn a reply into a private message or change audience semantics.
- Cross-posting is multiple explicit destination actions, not one ambient broadcast permission.
- Account/audience/target changes invalidate stale proposals.

#### Acceptance criteria
- [ ] Preview shows exact destination network/account/audience/text/attachments for every write.
- [ ] Reply targets the intended current thread/post or fails on stale/deleted target.
- [ ] Private message capability cannot be reached through a public-reply mapping.
- [ ] Cross-post writes are independently journaled/verified per destination.
- [ ] Unsupported provider actions are disabled/handed off rather than simulated.
- [ ] Delete/visibility changes propagate back into the unified feed when provider state confirms them.

#### Dependencies
- SOC-01
- TX-01
- CONN-04

**First phase:** P4  
**Maturity target:** P7  
**Owner:** connectors-domains

### Discussion (2 comments)

#### Comment 5682368879 — Jordan-Hall — 2026-09-15T14:49:43Z

Source: https://github.com/Jordan-Hall/browser/issues/87#issuecomment-5682368879 | Updated: 2026-09-15T14:49:43Z

<!-- intent-implementation-v1:SOC-03 -->
###### Implementation proposal — SOC-03

Implement network-specific writes over #85/#78/#48. A cross-post is a group of independent destination transactions, not one atomic broadcast or blanket permission.

- [ ] **SOC-03.T01 — Per-network schemas.** Register exact post/reply/reaction/repost/delete/message operations with account, target, audience and provider extensions. **Verify:** unsupported actions remain unavailable and a public reply cannot map to a private message.
- [ ] **SOC-03.T02 — Durable drafts/destinations.** Preserve original text separately from each destination's formatting, mentions, length and audience adaptations. **Verify:** the user can review every adapted payload before sending; draft state cannot be confused with sent state.
- [ ] **SOC-03.T03 — Scoped attachments.** Validate media/type/size and explicit metadata policy; bind account-specific upload IDs to the intended draft/destination. **Verify:** attachments cannot be reused across unauthorized accounts or silently disclose location metadata.
- [ ] **SOC-03.T04 — Target freshness/approval.** Refresh thread visibility and account state; prepare exact text/audience/attachment proposals with trusted confirmation where required. **Verify:** deleted/stale targets or changed recipients invalidate obsolete approval.
- [ ] **SOC-03.T05 — Partial cross-post outcomes.** Journal and verify each destination independently, retaining success/failure/unknown references. **Verify:** a partial failure never retries already accepted destinations automatically.
- [ ] **SOC-03.T06 — Messaging/deletion/derived state.** Add supported private-message and delete operations with exact participants, fresh policy and receipts. **Verify:** feed/index changes reflect confirmed provider state plus immediate local access restrictions.
- [ ] **SOC-03.T07 — Safety/access tests.** Exercise wrong accounts, mentions, private/public transitions, blocked threads, duplicate events and lost responses. **Verify:** each enabled live write has actual production access and a truthful verification contract.

**Review boundary:** providers differ in text, media, audience and acknowledgement semantics. Show adaptations and partial confirmation limits; do not invent uniform delivered/read guarantees. Current account/audience must remain visible even inside a fully custom user interface.

#### Comment 5688006226 — Jordan-Hall — 2026-09-15T20:59:40Z

Source: https://github.com/Jordan-Hall/browser/issues/87#issuecomment-5688006226 | Updated: 2026-09-15T20:59:40Z

###### Task issues

- [ ] #668 `SOC-03.T01` — Define action schemas per network capability
- [ ] #669 `SOC-03.T02` — Implement durable draft and destination composition
- [ ] #670 `SOC-03.T03` — Stage attachments with scoped uploads
- [ ] #671 `SOC-03.T04` — Revalidate reply targets and create approvals
- [ ] #672 `SOC-03.T05` — Dispatch cross-post groups with partial results
- [ ] #673 `SOC-03.T06` — Implement messaging, deletion and derived-state updates
- [ ] #674 `SOC-03.T07` — Qualify account, audience and retry safety

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-88"></a>
## #88 — [P2][SOC-04] Email, calendar, contacts and project actions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/88
**Created:** 2026-09-15T12:19:12Z | **Updated:** 2026-09-15T21:00:27Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** #28 | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #28

#### Objective
Bring email, calendar, contacts and project systems into the same source-backed workspace/action model so the browser becomes a general work interface, not just a web feed.

#### Scope
- Read-first normalized views for messages/threads, contacts, events/calendars and project tasks/items.
- Draft/reply/forward/send actions with exact account, recipients, thread and attachments.
- Calendar create/update/respond with timezone, attendee and conflict checks.
- Contact/source identity preservation and uncertain identity linking.
- Project task/status/comment actions with provider-specific semantics.
- Trusted approvals for consequential sends/meeting changes where policy requires.
- Provider outcome verification and reconciliation.

#### Correctness rules
- Exact recipient/account/date/timezone matter; display-name inference is insufficient for writes.
- Draft is distinct from sent; scheduled is distinct from accepted.
- Private sources remain account/workspace scoped throughout normalization/retrieval.

#### Acceptance criteria
- [ ] Email send preview/approval identifies exact account, recipients, thread/content and attachments.
- [ ] Calendar writes show timezone, attendees and detected conflicts before commit.
- [ ] External send/update outcome is verified or explicitly unresolved.
- [ ] Private email/calendar/project records cannot leak through cross-account caches/retrieval.
- [ ] Provider-specific task/event semantics remain visible when normalization is incomplete.
- [ ] Read-only workspace remains useful without write permissions.

#### Dependencies
- CONN-01
- DATA-01
- TX-01

**First phase:** P2  
**Maturity target:** P6  
**Owner:** connectors-domains

### Discussion (2 comments)

#### Comment 5682377349 — Jordan-Hall — 2026-09-15T14:50:09Z

Source: https://github.com/Jordan-Hall/browser/issues/88#issuecomment-5682377349 | Updated: 2026-09-15T14:50:09Z

<!-- intent-implementation-v1:SOC-04 -->
###### Implementation proposal — SOC-04

Implement separate email, calendar, contact and project domain packs over #45/#41/#78, reusing the same workspace/authority substrate without flattening service semantics.

- [ ] **SOC-04.T01 — Account-scoped domain views.** Map messages/threads, contacts, events/calendars and project objects with exact provider/account IDs and extensions; build read-only bindings first. **Verify:** useful views require no write permission and private records remain partitioned.
- [ ] **SOC-04.T02 — Recipient/contact resolution.** Resolve exact addresses/provider identities from explicit selection or authorized contacts; retain ambiguity and show final identifiers. **Verify:** similar names or cross-network guesses cannot select a write recipient silently.
- [ ] **SOC-04.T03 — Email operations.** Preserve sender account/thread headers and draft content; validate recipients/attachments and separate save/reply/forward/send. **Verify:** each send has transaction authority and provider references; lost acknowledgement is not blindly resent.
- [ ] **SOC-04.T04 — Calendar normalization/conflicts.** Preserve IANA timezone, all-day dates, recurrences/exceptions and provider semantics; check permitted calendars for availability. **Verify:** unknown availability is not called free and wall-clock/DST cases remain correct.
- [ ] **SOC-04.T05 — Calendar writes/responses.** Bind source version, attendee set, recurrence-edit scope and timezone to create/update/delete/respond proposals. **Verify:** editing one recurrence cannot silently alter the series or invite another attendee set.
- [ ] **SOC-04.T06 — Project actions.** Implement supported status transitions, comments, task creation and assignment using actual workflow rules and exact user/object identities. **Verify:** invalid transitions fail explicitly and resulting provider state is checked.
- [ ] **SOC-04.T07 — Cross-service qualification.** Read a thread, draft a reply, check calendars and propose a project update with separate scopes. **Verify:** no cross-account authority expansion, private-index leakage or duplicate consequential action.

**Review note:** this broad feature is a good candidate for later native sub-issues per service/operation family. These stable tasks keep all scope visible now without creating new issues. Reading one account cannot authorize sending from another; a calendar event being created does not imply every attendee accepted.

#### Comment 5688015220 — Jordan-Hall — 2026-09-15T21:00:27Z

Source: https://github.com/Jordan-Hall/browser/issues/88#issuecomment-5688015220 | Updated: 2026-09-15T21:00:27Z

###### Task issues

- [ ] #675 `SOC-04.T01` — Define domain packs and account-scoped views
- [ ] #676 `SOC-04.T02` — Implement recipient and contact resolution
- [ ] #677 `SOC-04.T03` — Implement email drafts, replies, forwards and sends
- [ ] #678 `SOC-04.T04` — Implement calendar reads and conflict checking
- [ ] #679 `SOC-04.T05` — Implement calendar writes and invitation responses
- [ ] #680 `SOC-04.T06` — Implement project-specific actions
- [ ] #681 `SOC-04.T07` — Qualify cross-service work and privacy

Child task issues are review/planning records only; implementation remains not started.


---

<a id="issue-271"></a>
## #271 — [TASK][EPIC-SOC.T01] Ratify source, identity and audience contracts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/271
**Created:** 2026-09-15T15:30:54Z | **Updated:** 2026-09-15T15:30:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #28

### Original description

Parent: #28

Task ID: `EPIC-SOC.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-272"></a>
## #272 — [TASK][EPIC-SOC.T02] Integrate unified read views and transparent ranking

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/272
**Created:** 2026-09-15T15:30:59Z | **Updated:** 2026-09-15T15:30:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #28

### Original description

Parent: #28

Task ID: `EPIC-SOC.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-273"></a>
## #273 — [TASK][EPIC-SOC.T03] Integrate destination-correct write workflows

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/273
**Created:** 2026-09-15T15:31:04Z | **Updated:** 2026-09-15T15:31:04Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #28

### Original description

Parent: #28

Task ID: `EPIC-SOC.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-274"></a>
## #274 — [TASK][EPIC-SOC.T04] Qualify mixed-account semantics and access

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/274
**Created:** 2026-09-15T15:31:11Z | **Updated:** 2026-09-15T15:31:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #28

### Original description

Parent: #28

Task ID: `EPIC-SOC.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-655"></a>
## #655 — [TASK][SOC-01.T01] Define source-preserving social records

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/655
**Created:** 2026-09-15T19:54:23Z | **Updated:** 2026-09-15T19:54:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #85

### Original description

Parent: #85

Task ID: `SOC-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-656"></a>
## #656 — [TASK][SOC-01.T02] Implement authenticated feed ingestion

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/656
**Created:** 2026-09-15T19:54:27Z | **Updated:** 2026-09-15T19:54:27Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #85

### Original description

Parent: #85

Task ID: `SOC-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-657"></a>
## #657 — [TASK][SOC-01.T03] Build thread reconstruction and cross-post clustering

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/657
**Created:** 2026-09-15T19:54:34Z | **Updated:** 2026-09-15T19:54:34Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #85

### Original description

Parent: #85

Task ID: `SOC-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-658"></a>
## #658 — [TASK][SOC-01.T04] Implement verified identity linking and corrections

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/658
**Created:** 2026-09-15T19:54:40Z | **Updated:** 2026-09-15T19:54:40Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #85

### Original description

Parent: #85

Task ID: `SOC-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-659"></a>
## #659 — [TASK][SOC-01.T05] Propagate deletion, visibility and moderation changes

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/659
**Created:** 2026-09-15T19:54:45Z | **Updated:** 2026-09-15T19:54:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #85

### Original description

Parent: #85

Task ID: `SOC-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-660"></a>
## #660 — [TASK][SOC-01.T06] Build the stable unified feed view

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/660
**Created:** 2026-09-15T19:54:51Z | **Updated:** 2026-09-15T19:54:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #85

### Original description

Parent: #85

Task ID: `SOC-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-661"></a>
## #661 — [TASK][SOC-01.T07] Qualify multi-network privacy and continuity

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/661
**Created:** 2026-09-15T19:54:56Z | **Updated:** 2026-09-15T19:54:56Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #85

### Original description

Parent: #85

Task ID: `SOC-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-662"></a>
## #662 — [TASK][SOC-02.T01] Specify ranking policy and chronological semantics

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/662
**Created:** 2026-09-15T20:58:19Z | **Updated:** 2026-09-15T20:58:19Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #86

### Original description

Parent: #86

Task ID: `SOC-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-663"></a>
## #663 — [TASK][SOC-02.T02] Extract bounded relevance features

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/663
**Created:** 2026-09-15T20:58:24Z | **Updated:** 2026-09-15T20:58:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #86

### Original description

Parent: #86

Task ID: `SOC-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-664"></a>
## #664 — [TASK][SOC-02.T03] Implement scoring and constraint evaluation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/664
**Created:** 2026-09-15T20:58:31Z | **Updated:** 2026-09-15T20:58:31Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #86

### Original description

Parent: #86

Task ID: `SOC-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-665"></a>
## #665 — [TASK][SOC-02.T04] Build ranking controls and explanations

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/665
**Created:** 2026-09-15T20:58:36Z | **Updated:** 2026-09-15T20:58:36Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #86

### Original description

Parent: #86

Task ID: `SOC-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-666"></a>
## #666 — [TASK][SOC-02.T05] Integrate scoped preferences and correction

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/666
**Created:** 2026-09-15T20:58:43Z | **Updated:** 2026-09-15T20:58:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #86

### Original description

Parent: #86

Task ID: `SOC-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-667"></a>
## #667 — [TASK][SOC-02.T06] Evaluate goal satisfaction and ranking agency

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/667
**Created:** 2026-09-15T20:58:48Z | **Updated:** 2026-09-15T20:58:48Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #86

### Original description

Parent: #86

Task ID: `SOC-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-668"></a>
## #668 — [TASK][SOC-03.T01] Define action schemas per network capability

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/668
**Created:** 2026-09-15T20:59:01Z | **Updated:** 2026-09-15T20:59:01Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #87

### Original description

Parent: #87

Task ID: `SOC-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-669"></a>
## #669 — [TASK][SOC-03.T02] Implement durable draft and destination composition

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/669
**Created:** 2026-09-15T20:59:09Z | **Updated:** 2026-09-15T20:59:09Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #87

### Original description

Parent: #87

Task ID: `SOC-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-670"></a>
## #670 — [TASK][SOC-03.T03] Stage attachments with scoped uploads

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/670
**Created:** 2026-09-15T20:59:14Z | **Updated:** 2026-09-15T20:59:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #87

### Original description

Parent: #87

Task ID: `SOC-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-671"></a>
## #671 — [TASK][SOC-03.T04] Revalidate reply targets and create approvals

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/671
**Created:** 2026-09-15T20:59:20Z | **Updated:** 2026-09-15T20:59:20Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #87

### Original description

Parent: #87

Task ID: `SOC-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-672"></a>
## #672 — [TASK][SOC-03.T05] Dispatch cross-post groups with partial results

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/672
**Created:** 2026-09-15T20:59:25Z | **Updated:** 2026-09-15T20:59:25Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #87

### Original description

Parent: #87

Task ID: `SOC-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-673"></a>
## #673 — [TASK][SOC-03.T06] Implement messaging, deletion and derived-state updates

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/673
**Created:** 2026-09-15T20:59:29Z | **Updated:** 2026-09-15T20:59:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #87

### Original description

Parent: #87

Task ID: `SOC-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-674"></a>
## #674 — [TASK][SOC-03.T07] Qualify account, audience and retry safety

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/674
**Created:** 2026-09-15T20:59:34Z | **Updated:** 2026-09-15T20:59:34Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #87

### Original description

Parent: #87

Task ID: `SOC-03.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-675"></a>
## #675 — [TASK][SOC-04.T01] Define domain packs and account-scoped views

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/675
**Created:** 2026-09-15T20:59:47Z | **Updated:** 2026-09-15T20:59:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #88

### Original description

Parent: #88

Task ID: `SOC-04.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-676"></a>
## #676 — [TASK][SOC-04.T02] Implement recipient and contact resolution

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/676
**Created:** 2026-09-15T20:59:53Z | **Updated:** 2026-09-15T20:59:53Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #88

### Original description

Parent: #88

Task ID: `SOC-04.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-677"></a>
## #677 — [TASK][SOC-04.T03] Implement email drafts, replies, forwards and sends

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/677
**Created:** 2026-09-15T20:59:58Z | **Updated:** 2026-09-15T20:59:58Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #88

### Original description

Parent: #88

Task ID: `SOC-04.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-678"></a>
## #678 — [TASK][SOC-04.T04] Implement calendar reads and conflict checking

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/678
**Created:** 2026-09-15T21:00:02Z | **Updated:** 2026-09-15T21:00:02Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #88

### Original description

Parent: #88

Task ID: `SOC-04.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-679"></a>
## #679 — [TASK][SOC-04.T05] Implement calendar writes and invitation responses

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/679
**Created:** 2026-09-15T21:00:09Z | **Updated:** 2026-09-15T21:00:09Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #88

### Original description

Parent: #88

Task ID: `SOC-04.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-680"></a>
## #680 — [TASK][SOC-04.T06] Implement project-specific actions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/680
**Created:** 2026-09-15T21:00:16Z | **Updated:** 2026-09-15T21:00:16Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #88

### Original description

Parent: #88

Task ID: `SOC-04.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-681"></a>
## #681 — [TASK][SOC-04.T07] Qualify cross-service work and privacy

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/681
**Created:** 2026-09-15T21:00:21Z | **Updated:** 2026-09-15T21:00:21Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #88

### Original description

Parent: #88

Task ID: `SOC-04.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

