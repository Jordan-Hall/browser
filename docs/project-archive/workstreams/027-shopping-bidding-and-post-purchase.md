# Shopping, bidding and post-purchase

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 45

## Contents

- [#27 — EPIC: Shopping, bidding and post-purchase](#issue-27)
- [#80 — [P2][SHOP-01] Product, variant and offer graph](#issue-80)
- [#81 — [P2][SHOP-02] Custom shopping UI, comparison and monitors](#issue-81)
- [#82 — [P4][SHOP-03] Checkout and payment integration](#issue-82)
- [#83 — [P4][SHOP-04] Auctions and maximum commitments](#issue-83)
- [#84 — [P4][SHOP-05] Receipts, orders, returns and recurring purchases](#issue-84)
- [#267 — [TASK][EPIC-SHOP.T01] Ratify commerce entity and price semantics](#issue-267)
- [#268 — [TASK][EPIC-SHOP.T02] Integrate comparison and user-controlled monitors](#issue-268)
- [#269 — [TASK][EPIC-SHOP.T03] Integrate purchases, auctions and post-purchase state](#issue-269)
- [#270 — [TASK][EPIC-SHOP.T04] Qualify provider access and commerce integrity](#issue-270)
- [#620 — [TASK][SHOP-01.T01] Define product, variant, seller and offer schemas](#issue-620)
- [#621 — [TASK][SHOP-01.T02] Implement deterministic amount and unit normalization](#issue-621)
- [#622 — [TASK][SHOP-01.T03] Build reversible product and variant matching](#issue-622)
- [#623 — [TASK][SHOP-01.T04] Normalize seller, warranty and returns evidence](#issue-623)
- [#624 — [TASK][SHOP-01.T05] Implement destination-sensitive offers and freshness](#issue-624)
- [#625 — [TASK][SHOP-01.T06] Integrate catalogue ingestion and provenance](#issue-625)
- [#626 — [TASK][SHOP-01.T07] Qualify the offer graph against adversarial fixtures](#issue-626)
- [#627 — [TASK][SHOP-02.T01] Implement editable shopping constraints](#issue-627)
- [#628 — [TASK][SHOP-02.T02] Bind persistent comparison views to offers](#issue-628)
- [#629 — [TASK][SHOP-02.T03] Implement deterministic filtering and ranking controls](#issue-629)
- [#630 — [TASK][SHOP-02.T04] Expose explanations, conflicts and source inspection](#issue-630)
- [#631 — [TASK][SHOP-02.T05] Implement selective refresh and change review](#issue-631)
- [#632 — [TASK][SHOP-02.T06] Add price and availability watches](#issue-632)
- [#633 — [TASK][SHOP-02.T07] Test the second-visit product experience](#issue-633)
- [#634 — [TASK][SHOP-03.T01] Select and qualify a real merchant route](#issue-634)
- [#635 — [TASK][SHOP-03.T02] Implement quote preparation and refresh](#issue-635)
- [#636 — [TASK][SHOP-03.T03] Implement scoped payment handoff](#issue-636)
- [#637 — [TASK][SHOP-03.T04] Build the trusted purchase confirmation](#issue-637)
- [#638 — [TASK][SHOP-03.T05] Commit, verify and reconcile the order](#issue-638)
- [#639 — [TASK][SHOP-03.T06] Implement authentic Original-mode fallback](#issue-639)
- [#640 — [TASK][SHOP-03.T07] Qualify checkout before staged production enablement](#issue-640)
- [#641 — [TASK][SHOP-04.T01] Model auctions and provider bid semantics](#issue-641)
- [#642 — [TASK][SHOP-04.T02] Implement time and freshness validation](#issue-642)
- [#643 — [TASK][SHOP-04.T03] Reserve global maximum exposure](#issue-643)
- [#644 — [TASK][SHOP-04.T04] Prepare and approve exact bid intent](#issue-644)
- [#645 — [TASK][SHOP-04.T05] Submit and reconcile without blind rebidding](#issue-645)
- [#646 — [TASK][SHOP-04.T06] Track outcomes and commitment lifecycle](#issue-646)
- [#647 — [TASK][SHOP-04.T07] Qualify auction concurrency and live access](#issue-647)
- [#648 — [TASK][SHOP-05.T01] Normalize orders, shipments and receipts](#issue-648)
- [#649 — [TASK][SHOP-05.T02] Build status refresh and notifications](#issue-649)
- [#650 — [TASK][SHOP-05.T03] Implement return and cancellation eligibility](#issue-650)
- [#651 — [TASK][SHOP-05.T04] Execute and verify post-purchase actions](#issue-651)
- [#652 — [TASK][SHOP-05.T05] Assemble rights-aware dispute evidence](#issue-652)
- [#653 — [TASK][SHOP-05.T06] Implement expiring recurring purchase rules](#issue-653)
- [#654 — [TASK][SHOP-05.T07] Qualify recurring and compensation recovery](#issue-654)

---

<a id="issue-27"></a>
## #27 — EPIC: Shopping, bidding and post-purchase

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/27
**Created:** 2026-09-15T12:08:21Z | **Updated:** 2026-09-15T14:24:50Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1

Own normalized commerce data, comparison/ranking UI, checkout/payment handoff, auctions, global commitments, receipts, tracking, returns and recurring purchase rules.

#### Child issues
- [ ] #80 SHOP-01 — Product, variant and offer graph
- [ ] #81 SHOP-02 — Custom shopping UI, comparison and monitors
- [ ] #82 SHOP-03 — Checkout and payment integration
- [ ] #83 SHOP-04 — Auctions and maximum commitments
- [ ] #84 SHOP-05 — Receipts, orders, returns and recurring purchases

#### Cross-cutting gates
Product/variant/offer/order are distinct, missing price components remain unknown, affiliate/conflict signals are disclosed, live write support requires provider approval, totals/bids are authority-bound, and ambiguous writes never repeat automatically.

### Discussion (1 comments)

#### Comment 5681901152 — Jordan-Hall — 2026-09-15T14:24:50Z

Source: https://github.com/Jordan-Hall/browser/issues/27#issuecomment-5681901152 | Updated: 2026-09-15T14:24:50Z

<!-- intent-implementation-v1:EPIC-SHOP -->
###### Workstream implementation and integration tasks

Integrate #80–#84 over source-backed data, persistent UI and the shared transaction engine.

- [ ] **EPIC-SHOP.T01 — Ratify entity/price semantics.** Separate product, variant, condition, offer, quote, order and receipt; preserve seller, destination, currency, fees and freshness. **Proof:** missing shipping/tax remains unknown and variant/review mismatches are rejected.
- [ ] **EPIC-SHOP.T02 — Integrate comparison/monitors.** Compile user constraints into stable card/table views, explainable rankings, exclusions and price/availability watches. **Proof:** comparisons reopen without chat and alerts use refreshed source evidence.
- [ ] **EPIC-SHOP.T03 — Integrate purchases/auctions/post-purchase.** Add quote-bound checkout, maximum bids, global reservations, verified orders, returns and narrow recurring rules. **Proof:** concurrent auctions respect the global commitment limit and ambiguous writes do not repeat.
- [ ] **EPIC-SHOP.T04 — Qualify access/integrity.** Test live-enabled routes only after required provider access; run stale quote, refund, outbid, expiry and restart fixtures. **Proof:** each advertised operation has a support/access record and verified or explicitly unresolved outcomes.

**Demonstration:** research an exact item, customize the comparison, monitor its offer, approve a fresh quote or maximum bid, then inspect the receipt and post-purchase state in the same workspace.

**Review boundaries:** provider read access does not imply checkout/bid access; payment credentials stay in approved merchant/PSP paths; affiliate disclosures are separate from ranking; refund/cancellation requests are not completed refunds/cancellations until verified.


---

<a id="issue-80"></a>
## #80 — [P2][SHOP-01] Product, variant and offer graph

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/80
**Created:** 2026-09-15T12:17:37Z | **Updated:** 2026-09-15T19:50:09Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #27

#### Objective
Normalize commerce data accurately enough for cross-source comparison without merging different variants, conditions, sellers or destination-dependent offers.

#### Scope
- Distinct `Product`, `Variant`, `Offer`, seller and marketplace/provider records.
- Entity resolution using identifiers/attributes plus uncertainty state.
- Condition, seller reputation/evidence, stock and observation time.
- Price currency, item price, shipping, tax status, fees and destination dependence.
- Delivery estimates, warranty and returns evidence.
- Offer/source freshness and exact listing link/provider object.
- Conflict/affiliate/commercial relationship metadata.

#### Data rules
- Missing shipping/tax is unknown, never zero.
- Similar-looking variants/conditions cannot be merged without sufficient evidence.
- Review/rating data cannot silently move between incompatible variants.

#### Acceptance criteria
- [ ] Different size/spec/condition variants remain distinct in adversarial fixtures.
- [ ] Every offer preserves seller, source, exact listing, observation time and price components.
- [ ] Unknown price components remain explicitly unknown.
- [ ] Entity-resolution uncertainty is inspectable and can be corrected.
- [ ] Stale offers are visibly marked and refreshable before transaction preparation.
- [ ] Destination-dependent totals recompute for the selected delivery context.

#### Dependencies
- DATA-01
- RES-01

**First phase:** P2  
**Maturity target:** P4  
**Owner:** connectors-domains

### Discussion (2 comments)

#### Comment 5682314551 — Jordan-Hall — 2026-09-15T14:46:58Z

Source: https://github.com/Jordan-Hall/browser/issues/80#issuecomment-5682314551 | Updated: 2026-09-15T14:46:58Z

<!-- intent-implementation-v1:SHOP-01 -->
###### Implementation proposal — SHOP-01

Implement a source-backed commerce graph over #41/#75. Keep Product, Variant, SellerAccount, Offer and observations distinct; a match is a reversible evidence-backed decision, not a destructive merge.

- [ ] **SHOP-01.T01 — Domain schemas.** Separate product families, exact specifications/condition variants, seller accounts and offer instances; preserve region/provider/source/version/time. **Verify:** reviews/orders cannot accidentally attach to a different variant.
- [ ] **SHOP-01.T02 — Exact amounts/units.** Parse decimal prices/currency/quantity/units deterministically and retain originals; model item/shipping/tax/fees as known, unknown or included components. **Verify:** unknown tax/shipping never becomes zero and currencies cannot silently mix.
- [ ] **SHOP-01.T03 — Reversible matching.** Prefer reliable identifiers, then compatible attributes and evidence-backed similarity; store confidence/reasons and allow split/correction. **Verify:** title or embedding similarity alone cannot merge incompatible offers.
- [ ] **SHOP-01.T04 — Seller/warranty/returns evidence.** Attach named-source reputation metrics, coverage/exclusions, deadlines and condition assertions with evidence and timestamps. **Verify:** disagreements remain inspectable and no universal seller-safety guarantee is invented.
- [ ] **SHOP-01.T05 — Destination/freshness.** Bind shipping/tax/delivery estimates to the chosen delivery context and distinguish them from executable quotes. **Verify:** address/context change invalidates dependent totals and critical fields refresh before action preparation.
- [ ] **SHOP-01.T06 — Ingestion/provenance.** Normalize permitted API/protocol/browser observations while retaining exact listing links, original IDs and extraction method; emit dependency changes. **Verify:** weaker visual/inferred data is not relabelled authoritative API data.
- [ ] **SHOP-01.T07 — Adversarial graph qualification.** Test variant collisions, misleading titles, currency ambiguity, stale stock, destination changes and incompatible reviews. **Verify:** measure false merges, unknown handling and evidence completeness separately.

**Core rule:** an offer depends on seller, condition, destination and observation time. Preserve source truth and user overlays separately. The graph supports comparisons and research; actual commitment still requires fresh merchant-specific quote and transaction authority.

#### Comment 5687168966 — Jordan-Hall — 2026-09-15T19:50:09Z

Source: https://github.com/Jordan-Hall/browser/issues/80#issuecomment-5687168966 | Updated: 2026-09-15T19:50:09Z

###### Task issues
- [ ] #620 `SHOP-01.T01` — Define product, variant, seller and offer schemas
- [ ] #621 `SHOP-01.T02` — Implement deterministic amount and unit normalization
- [ ] #622 `SHOP-01.T03` — Build reversible product and variant matching
- [ ] #623 `SHOP-01.T04` — Normalize seller, warranty and returns evidence
- [ ] #624 `SHOP-01.T05` — Implement destination-sensitive offers and freshness
- [ ] #625 `SHOP-01.T06` — Integrate catalogue ingestion and provenance
- [ ] #626 `SHOP-01.T07` — Qualify the offer graph against adversarial fixtures


---

<a id="issue-81"></a>
## #81 — [P2][SHOP-02] Custom shopping UI, comparison and monitors

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/81
**Created:** 2026-09-15T12:17:48Z | **Updated:** 2026-09-15T19:51:01Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #27

#### Objective
Deliver the flagship user-owned shopping experience: exact source-backed offers rendered in a persistent custom comparison UI with explainable ranking and freshness monitoring.

#### Scope
- Goal constraints such as budget, specs, condition, warranty, seller requirements and delivery deadline.
- Card/table comparison views backed by SHOP-01 offers and RES-02 claim/ranking logic.
- User-editable columns, filters, exclusions, pinned products and saved layouts.
- Explain why each result matches/fails constraints and which facts are missing.
- Freshness badges and selective refresh of affected offers/rankings.
- Price/availability monitoring through AUTO-01.
- Affiliate/commercial-conflict disclosure separated from ranking logic.
- Original/evidence navigation per offer/claim.

#### Product rules
- Comparison remains directly usable without an active chat/model call.
- User ranking controls beat engagement/affiliate optimization.
- A missing fact must remain missing, not inferred as favorable.

#### Acceptance criteria
- [ ] User can create/save/reopen a comparison and sort/filter it with inference disabled.
- [ ] Every critical field shows source/freshness or explicit unknown state.
- [ ] Source changes refresh only affected ranking/comparison dependencies.
- [ ] Ranking explanation reflects explicit user constraints and disclosed conflicts.
- [ ] Alerts are based on refreshed source state, not stale cached values.
- [ ] Original listings and evidence are reachable from the comparison.

#### Dependencies
- SHOP-01
- UI-02
- AUTO-01

**First phase:** P2  
**Maturity target:** P5  
**Owner:** connectors-domains

### Discussion (2 comments)

#### Comment 5682319893 — Jordan-Hall — 2026-09-15T14:47:16Z

Source: https://github.com/Jordan-Hall/browser/issues/81#issuecomment-5682319893 | Updated: 2026-09-15T14:47:16Z

<!-- intent-implementation-v1:SHOP-02 -->
###### Implementation proposal — SHOP-02

Build the flagship persistent comparison over #80/#50/#92, with scoring from #76. User exclusions/layouts are overlays, never edits to merchant observations.

- [ ] **SHOP-02.T01 — Editable constraints.** Provide typed budget/spec/condition/delivery/seller/warranty fields alongside interpreted intent. **Verify:** unresolved requirements stay visible and explicit corrections persist in GoalContract.
- [ ] **SHOP-02.T02 — Persistent views.** Compose trusted cards/tables/details with stable offer IDs, selected columns and source-backed price states. **Verify:** reopening without chat preserves filters, pins, selection and layout.
- [ ] **SHOP-02.T03 — Deterministic filters/ranking.** Apply satisfied/violated/unknown hard constraints and actual versioned ranking factors; support reversible exclusions. **Verify:** sorting/filtering needs no model call and unknown values cannot pass a verified-only constraint.
- [ ] **SHOP-02.T04 — Explanations/source inspection.** Expose ranking factors, unmet requirements, missing facts, timestamps and commercial disclosures with exact Evidence/Original navigation. **Verify:** explanations agree with scoring and source records.
- [ ] **SHOP-02.T05 — Selective refresh/history.** Subscribe to offer dependencies, update affected fields/ranks and preserve focus/scroll; identify material changes before actions. **Verify:** stale approvals do not survive changed prices or variants.
- [ ] **SHOP-02.T06 — Price/stock watches.** Create explicit workflows bound to exact offer/product constraints, execution location, freshness and notification limits. **Verify:** alerts require new qualifying observations, not stale cached threshold values.
- [ ] **SHOP-02.T07 — Second-visit benchmark.** Compare ordinary browsing, same-tool/model chat assistance and saved-workspace use on repeated shopping tasks. **Verify:** report exact matches, interventions, source quality and reuse; voluntary source inspection is not a failure metric.

**Dependency refinement:** comparison can start before watches mature; #92 is required for monitoring, not basic tables. Separate mandatory constraints from preferences, keep affiliate incentives out of hidden scoring and allow users to recover excluded offers. The experience must remain useful when inference is unavailable.

#### Comment 5687178917 — Jordan-Hall — 2026-09-15T19:51:01Z

Source: https://github.com/Jordan-Hall/browser/issues/81#issuecomment-5687178917 | Updated: 2026-09-15T19:51:01Z

###### Task issues
- [ ] #627 `SHOP-02.T01` — Implement editable shopping constraints
- [ ] #628 `SHOP-02.T02` — Bind persistent comparison views to offers
- [ ] #629 `SHOP-02.T03` — Implement deterministic filtering and ranking controls
- [ ] #630 `SHOP-02.T04` — Expose explanations, conflicts and source inspection
- [ ] #631 `SHOP-02.T05` — Implement selective refresh and change review
- [ ] #632 `SHOP-02.T06` — Add price and availability watches
- [ ] #633 `SHOP-02.T07` — Test the second-visit product experience


---

<a id="issue-82"></a>
## #82 — [P4][SHOP-03] Checkout and payment integration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/82
**Created:** 2026-09-15T12:18:10Z | **Updated:** 2026-09-15T19:51:46Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #27

#### Objective
Support purchase execution through approved merchant/payment routes with fresh quotes, exact account/delivery confirmation, trusted approval and post-commit verification.

#### Scope
- Quote preparation/refresh from supported commerce connector/protocol.
- Merchant/seller, product/variant, quantity, currency, item price, shipping, tax, fees and destination binding.
- Merchant/PSP/tokenized payment handoff; avoid handling raw card data where possible.
- Trusted purchase confirmation through TX-01.
- Commit via supported API/protocol; supervised Original-view handoff where custom checkout is unavailable.
- Provider order/receipt verification and TX-02 reconciliation.
- Support-matrix integration with CONN-04 production approvals.

#### Safety rules
- Any material total/item/destination/account change invalidates approval.
- Missing tax/shipping means total is not yet fully known.
- Ambiguous commit is never automatically retried.
- Visual checkout automation is not marketed as equivalent to an API-backed transaction.

#### Acceptance criteria
- [ ] Production-enabled routes have documented live provider access/approval.
- [ ] Approval is bound to exact merchant/seller/item/variant/destination/currency/total or configured ceiling.
- [ ] Material quote changes invalidate approval and require refresh/review.
- [ ] Commit produces verified provider order/receipt or explicit NeedsReconciliation.
- [ ] No ambiguous purchase fixture duplicates an order.
- [ ] Unsupported merchant checkout hands off authentically to Original mode with truthful state-transfer limits.

#### Dependencies
- SHOP-01
- TX-01
- TX-02
- CONN-04

**First phase:** P4  
**Maturity target:** P7  
**Owner:** connectors-domains

### Discussion (2 comments)

#### Comment 5682327179 — Jordan-Hall — 2026-09-15T14:47:38Z

Source: https://github.com/Jordan-Hall/browser/issues/82#issuecomment-5682327179 | Updated: 2026-09-15T14:47:38Z

<!-- intent-implementation-v1:SHOP-03 -->
###### Implementation proposal — SHOP-03

Implement real merchant checkout over #80/#78/#79/#48, using scoped opaque PaymentHandle state rather than model-visible card data. Fixture support and production merchant access are separate gates.

- [ ] **SHOP-03.T01 — Qualify a merchant route.** Enumerate catalogue/quote/checkout/payment/order-state operations, pin interfaces and obtain necessary access; separate test/live environments. **Verify:** advertised checkout has actual approved production support.
- [ ] **SHOP-03.T02 — Quote preparation/refresh.** Resolve exact variant, quantity, seller, address and delivery choice; preserve all components, expiry and source state. **Verify:** estimates/unknown fees cannot be presented as a final known total.
- [ ] **SHOP-03.T03 — Scoped payment handoff.** Prefer merchant/PSP-hosted or approved tokenized flows; verify return origin/session and keep card entry outside model context. **Verify:** spoofed callbacks or another account's payment handle reject.
- [ ] **SHOP-03.T04 — Trusted purchase confirmation.** Show account, merchant/seller, item/variant/quantity, destination, currency, total/ceiling, recurring terms and expiry. **Verify:** the approval binds canonical #78 material fields and changed quotes require review.
- [ ] **SHOP-03.T05 — Commit/verify/reconcile.** Dispatch with durable operation identity and reservation, capture provider references and independently verify order state. **Verify:** lost acknowledgements retain uncertainty/liability and never cause blind duplicate purchase.
- [ ] **SHOP-03.T06 — Original fallback.** Open the authentic merchant/account and only transfer permitted state whose transfer is verified; retain workspace return context. **Verify:** protected auth/payment steps hand over to the user and unverified cart state is labelled.
- [ ] **SHOP-03.T07 — Staged qualification.** Exercise quote mutation, hidden fees, callback spoofing, duplicate events, revocation and lost acknowledgements in fixtures before granted low-risk canaries. **Verify:** each live route has current conformance and access evidence.

**Protocol review:** evaluate [UCP](https://ucp.dev/) and relevant merchant protocols as adapters, not a promise of merchant coverage. Hosted/tokenized payment reduces sensitive handling but does not automatically eliminate compliance obligations. Checkout remains fully in scope; unavailable routes must degrade honestly rather than masquerade as API-backed success.

#### Comment 5687187376 — Jordan-Hall — 2026-09-15T19:51:46Z

Source: https://github.com/Jordan-Hall/browser/issues/82#issuecomment-5687187376 | Updated: 2026-09-15T19:51:46Z

###### Task issues
- [ ] #634 `SHOP-03.T01` — Select and qualify a real merchant route
- [ ] #635 `SHOP-03.T02` — Implement quote preparation and refresh
- [ ] #636 `SHOP-03.T03` — Implement scoped payment handoff
- [ ] #637 `SHOP-03.T04` — Build the trusted purchase confirmation
- [ ] #638 `SHOP-03.T05` — Commit, verify and reconcile the order
- [ ] #639 `SHOP-03.T06` — Implement authentic Original-mode fallback
- [ ] #640 `SHOP-03.T07` — Qualify checkout before staged production enablement


---

<a id="issue-83"></a>
## #83 — [P4][SHOP-04] Auctions and maximum commitments

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/83
**Created:** 2026-09-15T12:18:20Z | **Updated:** 2026-09-15T19:53:25Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #27

#### Objective
Support marketplace bidding safely using explicit maximum commitments and a global reservation ledger so simultaneous auctions cannot exceed the user's total budget.

#### Scope
- Auction entity: marketplace/account/item/variant, current price, currency, fees, end time, bid increment and observed time.
- Distinguish current price from user's maximum bid/commitment.
- Bid preparation/refresh/approval/commit through TX-01.
- Reserve worst-case configured commitments across all outstanding auctions through TX-02.
- Handle auction end, outbid, won/lost, cancellation limitations and stale price/time data.
- Provider production-access requirements and test-account/sandbox strategy.
- Notifications for outbid/end/win/unknown outcomes.

#### Safety rules
- A submitted bid may be irrevocable; local cancellation cannot claim to withdraw it.
- Global commitment includes relevant known fees/currency conversions according to policy.
- Timeout after bid submission enters reconciliation, not automatic rebid.

#### Acceptance criteria
- [ ] Fixture tests preserve global budget across simultaneous auctions.
- [ ] Maximum bid is displayed/authorized separately from current price.
- [ ] End time/currency/known fees and source freshness are included before approval.
- [ ] Ambiguous bid submission never causes an automatic duplicate bid.
- [ ] Win/loss/outbid state is verified against provider truth.
- [ ] Live support is enabled only where CONN-04 records required production approval.

#### Dependencies
- SHOP-01
- TX-02
- CONN-04

**First phase:** P4  
**Maturity target:** P7  
**Owner:** connectors-domains

### Discussion (2 comments)

#### Comment 5682332532 — Jordan-Hall — 2026-09-15T14:47:55Z

Source: https://github.com/Jordan-Hall/browser/issues/83#issuecomment-5682332532 | Updated: 2026-09-15T14:47:55Z

<!-- intent-implementation-v1:SHOP-04 -->
###### Implementation proposal — SHOP-04

Implement auction-specific semantics over #80/#79/#48. BidIntent and MaximumCommitment are not the current displayed price; bind every bid to marketplace/account/lot and durable operation identity.

- [ ] **SHOP-04.T01 — Auction model.** Preserve exact lot/variant, increment/reserve/buy-now rules where exposed, currency, fees, provider end time and observations. **Verify:** provider-specific bid semantics are not flattened into a generic purchase.
- [ ] **SHOP-04.T02 — Clock/freshness validation.** Compare provider time with local monotonic deadlines, track skew and refresh before submission. **Verify:** ended, extended, withdrawn or revised lots invalidate obsolete bid plans.
- [ ] **SHOP-04.T03 — Maximum exposure reservations.** Reserve the authorized maximum plus applicable fees across all auctions and purchases through #79; use an explicit timestamped currency policy. **Verify:** simultaneous wins cannot exceed the shared limit unnoticed.
- [ ] **SHOP-04.T04 — Exact approval.** Display lot, account, marketplace, currency, maximum, known fees, end time and other commitments in trusted chrome. **Verify:** approval binds the current lot/limit and cannot be reused after material changes.
- [ ] **SHOP-04.T05 — Submit/reconcile.** Use the approved route, retain bid references and match provider bidding state to the original intent. **Verify:** timeout after possible acceptance does not cause an automatic rebid.
- [ ] **SHOP-04.T06 — Outcome/commitment lifecycle.** Deduplicate outbid/extension/won/lost/withdrawal/settlement events; create follow-on order tasks. **Verify:** reservations release only when provider semantics establish liability has ended, not simply when local execution stops.
- [ ] **SHOP-04.T07 — Concurrency/access qualification.** Test simultaneous wins, skew/extensions, fees, partitions, cancellation and ambiguous delivery in resettable fixtures. **Verify:** live bidding stays disabled until required provider production access is recorded.

**Reference/access consideration:** [eBay buying applications](https://developer.ebay.com/develop/get-started/get-started-on-a-buying-application) is an access route to assess, not proof this product has approval. Do not promise reliable last-second bidding across arbitrary networks. A submitted bid may not be retractable; local cancellation must never imply withdrawal.

#### Comment 5687206277 — Jordan-Hall — 2026-09-15T19:53:25Z

Source: https://github.com/Jordan-Hall/browser/issues/83#issuecomment-5687206277 | Updated: 2026-09-15T19:53:25Z

###### Task issues
- [ ] #641 `SHOP-04.T01` — Model auctions and provider bid semantics
- [ ] #642 `SHOP-04.T02` — Implement time and freshness validation
- [ ] #643 `SHOP-04.T03` — Reserve global maximum exposure
- [ ] #644 `SHOP-04.T04` — Prepare and approve exact bid intent
- [ ] #645 `SHOP-04.T05` — Submit and reconcile without blind rebidding
- [ ] #646 `SHOP-04.T06` — Track outcomes and commitment lifecycle
- [ ] #647 `SHOP-04.T07` — Qualify auction concurrency and live access


---

<a id="issue-84"></a>
## #84 — [P4][SHOP-05] Receipts, orders, returns and recurring purchases

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/84
**Created:** 2026-09-15T12:18:29Z | **Updated:** 2026-09-15T19:54:13Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #27

#### Objective
Extend commerce beyond checkout into provider-truth order tracking, receipts, returns/refunds/disputes and explicitly bounded recurring purchase workflows.

#### Scope
- Normalize order, shipment/delivery, receipt/invoice and return/refund/dispute state without losing provider IDs.
- Refresh order status from provider truth and preserve observation times.
- Receipt/invoice artifact storage and export.
- Return/refund/cancellation preparation as new TX actions with current policy/eligibility checks.
- Evidence bundle for disputes (order, listing/source snapshots, communications, receipts where permitted).
- Recurring purchase rules with product/seller/price/quantity/frequency/expiry ceilings.
- Renewal/recurring action notifications, reservations and revocation.

#### Correctness rules
- Compensation is explicit and may fail; UI never promises refund/cancellation before provider verification.
- Recurring authority is narrow, expiring and inspectable.
- Order status comes from provider observations rather than local inference.

#### Acceptance criteria
- [ ] Order/delivery/refund state reflects provider truth with freshness metadata.
- [ ] Returns/refunds/cancellations create new proposals and receipts.
- [ ] Recurring rule cannot exceed configured product/amount/quantity/frequency/time boundaries.
- [ ] Revocation prevents new recurring commits while preserving existing order history.
- [ ] Dispute evidence links source/order/receipt records with access controls intact.
- [ ] Ambiguous compensation remains NeedsReconciliation instead of being called successful.

#### Dependencies
- SHOP-03
- TX-02
- AUTO-01

**First phase:** P4  
**Maturity target:** P6  
**Owner:** connectors-domains

### Discussion (3 comments)

#### Comment 5682339103 — Jordan-Hall — 2026-09-15T14:48:14Z

Source: https://github.com/Jordan-Hall/browser/issues/84#issuecomment-5682339103 | Updated: 2026-09-15T14:48:14Z

<!-- intent-implementation-v1:SHOP-05 -->
###### Implementation proposal — SHOP-05

Implement order lifecycle and expiring recurring rules over #82/#79/#92. Keep payment, fulfillment, shipment, return, refund and dispute dimensions separate so partial outcomes remain representable.

- [ ] **SHOP-05.T01 — Orders/receipts.** Ingest exact line items, variants/sellers, totals, provider IDs and observation times; store invoices as labelled artifacts. **Verify:** partial shipments and partial refunds cannot be collapsed into a misleading single status.
- [ ] **SHOP-05.T02 — Refresh/notifications.** Use authenticated events or bounded polling with deduplication/revisions and stale-state indicators. **Verify:** delayed events cannot regress newer order state or repeatedly notify the same change.
- [ ] **SHOP-05.T03 — Eligibility/proposals.** Retrieve current return/cancellation rules, deadlines, fees and shipping requirements; prepare a new compensation proposal linked to the original receipt. **Verify:** eligibility is not inferred from generic policy text alone when provider confirmation is needed.
- [ ] **SHOP-05.T04 — Execute/verify remedies.** Use transaction policy, receipts and reconciliation, preserving partial acceptance and physical steps the user must complete. **Verify:** requesting a refund/cancellation is not reported as completed until independently confirmed.
- [ ] **SHOP-05.T05 — Dispute evidence.** Assemble reviewed permitted listing snapshots, receipts, communications and timeline with source/time/hash provenance. **Verify:** unrelated private records and restricted source bodies are redacted before sharing.
- [ ] **SHOP-05.T06 — Recurring purchase rules.** Specify product/variant, sellers, quantity/amount, frequency, deadline, substitutions and expiry; refresh/reserve/revalidate each run. **Verify:** recurring consent cannot authorize arbitrary substitutes or unlimited purchases.
- [ ] **SHOP-05.T07 — Recovery qualification.** Test duplicate/missed triggers, partial refunds, late events, revoked grants and ambiguous compensation across restart/export. **Verify:** historical orders remain available while revoked recurrence cannot create new commits.

**Review boundary:** recurring authority is narrow, inspectable and expiring, not a portable blanket approval. Preserve original and compensating receipts and unknown exposure. Physical return shipping, provider rejection and refund timing must remain visible rather than being hidden behind a generic successful workflow badge.

#### Comment 5682743719 — Nakagawa-master — 2026-09-15T15:12:02Z

Source: https://github.com/Jordan-Hall/browser/issues/84#issuecomment-5682743719 | Updated: 2026-09-15T15:12:02Z

One race worth making explicit in T06/T07 is that **“revalidated for this run” can still become stale before the provider-side commit**.

I’d separate the durable recurring rule from the one-occurrence execution authority:

```text
recurring rule / consent
→ derive one occurrence
→ refresh current rule + provider facts
→ prepare candidate action
→ immediately-before-side-effect revalidate exact rule version/scope
→ commit once
→ receipt binds what actually authorized that commit
```

A prepared proposal/reservation should not itself become portable authority. If the rule is revoked, expires, or is materially edited after preparation but before the side effect, the old prepared action should fail closed and be re-derived under the new state. Prior completed orders remain historical facts.

Useful receipt fields would include at least `rule_id` + immutable rule version/digest, occurrence/window identity, exact product/variant/seller, bounded price/quantity/frequency terms, observed provider state/quote identity where available, `checked_at`, and the committed provider result.

A few regression cases would make the boundary concrete:

- rule revoked after refresh but before commit → no new order;
- price/quantity ceiling narrowed after a candidate was prepared → old candidate cannot inherit the new authority;
- product/seller/variant changes behind an otherwise similar listing → revalidation required;
- exact expiry boundary races with dispatch → authority must be current at commit, not merely when the run started;
- retry/restart finds a prepared action from an older rule version → reconcile/re-derive, never blind replay.

If a provider “reservation” itself creates a real hold, charge, inventory lock, or other consequential effect, that reservation is already an execution boundary and should receive the same current-authority check rather than being treated as harmless preparation.

This seems complementary to the existing `refresh/reserve/revalidate each run` requirement: it closes the TOCTOU window *inside* a run and makes revocation-before-commit directly testable.

#### Comment 5687215186 — Jordan-Hall — 2026-09-15T19:54:13Z

Source: https://github.com/Jordan-Hall/browser/issues/84#issuecomment-5687215186 | Updated: 2026-09-15T19:54:13Z

###### Task issues
- [ ] #648 `SHOP-05.T01` — Normalize orders, shipments and receipts
- [ ] #649 `SHOP-05.T02` — Build status refresh and notifications
- [ ] #650 `SHOP-05.T03` — Implement return and cancellation eligibility
- [ ] #651 `SHOP-05.T04` — Execute and verify post-purchase actions
- [ ] #652 `SHOP-05.T05` — Assemble rights-aware dispute evidence
- [ ] #653 `SHOP-05.T06` — Implement expiring recurring purchase rules
- [ ] #654 `SHOP-05.T07` — Qualify recurring and compensation recovery


---

<a id="issue-267"></a>
## #267 — [TASK][EPIC-SHOP.T01] Ratify commerce entity and price semantics

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/267
**Created:** 2026-09-15T15:30:23Z | **Updated:** 2026-09-15T15:30:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #27

### Original description

Parent: #27

Task ID: `EPIC-SHOP.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-268"></a>
## #268 — [TASK][EPIC-SHOP.T02] Integrate comparison and user-controlled monitors

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/268
**Created:** 2026-09-15T15:30:29Z | **Updated:** 2026-09-15T15:30:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #27

### Original description

Parent: #27

Task ID: `EPIC-SHOP.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-269"></a>
## #269 — [TASK][EPIC-SHOP.T03] Integrate purchases, auctions and post-purchase state

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/269
**Created:** 2026-09-15T15:30:39Z | **Updated:** 2026-09-15T15:30:39Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #27

### Original description

Parent: #27

Task ID: `EPIC-SHOP.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-270"></a>
## #270 — [TASK][EPIC-SHOP.T04] Qualify provider access and commerce integrity

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/270
**Created:** 2026-09-15T15:30:45Z | **Updated:** 2026-09-15T15:30:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #27

### Original description

Parent: #27

Task ID: `EPIC-SHOP.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-620"></a>
## #620 — [TASK][SHOP-01.T01] Define product, variant, seller and offer schemas

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/620
**Created:** 2026-09-15T19:49:25Z | **Updated:** 2026-09-15T19:49:25Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #80

### Original description

Parent: #80

Task ID: `SHOP-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-621"></a>
## #621 — [TASK][SHOP-01.T02] Implement deterministic amount and unit normalization

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/621
**Created:** 2026-09-15T19:49:30Z | **Updated:** 2026-09-15T19:49:30Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #80

### Original description

Parent: #80

Task ID: `SHOP-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-622"></a>
## #622 — [TASK][SHOP-01.T03] Build reversible product and variant matching

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/622
**Created:** 2026-09-15T19:49:38Z | **Updated:** 2026-09-15T19:49:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #80

### Original description

Parent: #80

Task ID: `SHOP-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-623"></a>
## #623 — [TASK][SHOP-01.T04] Normalize seller, warranty and returns evidence

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/623
**Created:** 2026-09-15T19:49:44Z | **Updated:** 2026-09-15T19:49:44Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #80

### Original description

Parent: #80

Task ID: `SHOP-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-624"></a>
## #624 — [TASK][SHOP-01.T05] Implement destination-sensitive offers and freshness

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/624
**Created:** 2026-09-15T19:49:53Z | **Updated:** 2026-09-15T19:49:53Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #80

### Original description

Parent: #80

Task ID: `SHOP-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-625"></a>
## #625 — [TASK][SHOP-01.T06] Integrate catalogue ingestion and provenance

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/625
**Created:** 2026-09-15T19:49:58Z | **Updated:** 2026-09-15T19:49:58Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #80

### Original description

Parent: #80

Task ID: `SHOP-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-626"></a>
## #626 — [TASK][SHOP-01.T07] Qualify the offer graph against adversarial fixtures

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/626
**Created:** 2026-09-15T19:50:03Z | **Updated:** 2026-09-15T19:50:03Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #80

### Original description

Parent: #80

Task ID: `SHOP-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-627"></a>
## #627 — [TASK][SHOP-02.T01] Implement editable shopping constraints

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/627
**Created:** 2026-09-15T19:50:14Z | **Updated:** 2026-09-15T19:50:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #81

### Original description

Parent: #81

Task ID: `SHOP-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-628"></a>
## #628 — [TASK][SHOP-02.T02] Bind persistent comparison views to offers

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/628
**Created:** 2026-09-15T19:50:18Z | **Updated:** 2026-09-15T19:50:18Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #81

### Original description

Parent: #81

Task ID: `SHOP-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-629"></a>
## #629 — [TASK][SHOP-02.T03] Implement deterministic filtering and ranking controls

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/629
**Created:** 2026-09-15T19:50:25Z | **Updated:** 2026-09-15T19:50:25Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #81

### Original description

Parent: #81

Task ID: `SHOP-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-630"></a>
## #630 — [TASK][SHOP-02.T04] Expose explanations, conflicts and source inspection

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/630
**Created:** 2026-09-15T19:50:30Z | **Updated:** 2026-09-15T19:50:30Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #81

### Original description

Parent: #81

Task ID: `SHOP-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-631"></a>
## #631 — [TASK][SHOP-02.T05] Implement selective refresh and change review

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/631
**Created:** 2026-09-15T19:50:36Z | **Updated:** 2026-09-15T19:50:36Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #81

### Original description

Parent: #81

Task ID: `SHOP-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-632"></a>
## #632 — [TASK][SHOP-02.T06] Add price and availability watches

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/632
**Created:** 2026-09-15T19:50:43Z | **Updated:** 2026-09-15T19:50:43Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #81

### Original description

Parent: #81

Task ID: `SHOP-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-633"></a>
## #633 — [TASK][SHOP-02.T07] Test the second-visit product experience

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/633
**Created:** 2026-09-15T19:50:53Z | **Updated:** 2026-09-15T19:50:53Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #81

### Original description

Parent: #81

Task ID: `SHOP-02.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-634"></a>
## #634 — [TASK][SHOP-03.T01] Select and qualify a real merchant route

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/634
**Created:** 2026-09-15T19:51:07Z | **Updated:** 2026-09-15T19:51:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #82

### Original description

Parent: #82

Task ID: `SHOP-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-635"></a>
## #635 — [TASK][SHOP-03.T02] Implement quote preparation and refresh

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/635
**Created:** 2026-09-15T19:51:12Z | **Updated:** 2026-09-15T19:51:12Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #82

### Original description

Parent: #82

Task ID: `SHOP-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-636"></a>
## #636 — [TASK][SHOP-03.T03] Implement scoped payment handoff

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/636
**Created:** 2026-09-15T19:51:17Z | **Updated:** 2026-09-15T19:51:17Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #82

### Original description

Parent: #82

Task ID: `SHOP-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-637"></a>
## #637 — [TASK][SHOP-03.T04] Build the trusted purchase confirmation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/637
**Created:** 2026-09-15T19:51:22Z | **Updated:** 2026-09-15T19:51:22Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #82

### Original description

Parent: #82

Task ID: `SHOP-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-638"></a>
## #638 — [TASK][SHOP-03.T05] Commit, verify and reconcile the order

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/638
**Created:** 2026-09-15T19:51:29Z | **Updated:** 2026-09-15T19:51:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #82

### Original description

Parent: #82

Task ID: `SHOP-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-639"></a>
## #639 — [TASK][SHOP-03.T06] Implement authentic Original-mode fallback

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/639
**Created:** 2026-09-15T19:51:35Z | **Updated:** 2026-09-15T19:51:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #82

### Original description

Parent: #82

Task ID: `SHOP-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-640"></a>
## #640 — [TASK][SHOP-03.T07] Qualify checkout before staged production enablement

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/640
**Created:** 2026-09-15T19:51:40Z | **Updated:** 2026-09-15T19:51:40Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #82

### Original description

Parent: #82

Task ID: `SHOP-03.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-641"></a>
## #641 — [TASK][SHOP-04.T01] Model auctions and provider bid semantics

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/641
**Created:** 2026-09-15T19:51:51Z | **Updated:** 2026-09-15T19:51:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #83

### Original description

Parent: #83

Task ID: `SHOP-04.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-642"></a>
## #642 — [TASK][SHOP-04.T02] Implement time and freshness validation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/642
**Created:** 2026-09-15T19:51:57Z | **Updated:** 2026-09-15T19:51:57Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #83

### Original description

Parent: #83

Task ID: `SHOP-04.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-643"></a>
## #643 — [TASK][SHOP-04.T03] Reserve global maximum exposure

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/643
**Created:** 2026-09-15T19:52:55Z | **Updated:** 2026-09-15T19:52:55Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #83

### Original description

Parent: #83

Task ID: `SHOP-04.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-644"></a>
## #644 — [TASK][SHOP-04.T04] Prepare and approve exact bid intent

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/644
**Created:** 2026-09-15T19:53:00Z | **Updated:** 2026-09-15T19:53:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #83

### Original description

Parent: #83

Task ID: `SHOP-04.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-645"></a>
## #645 — [TASK][SHOP-04.T05] Submit and reconcile without blind rebidding

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/645
**Created:** 2026-09-15T19:53:07Z | **Updated:** 2026-09-15T19:53:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #83

### Original description

Parent: #83

Task ID: `SHOP-04.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-646"></a>
## #646 — [TASK][SHOP-04.T06] Track outcomes and commitment lifecycle

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/646
**Created:** 2026-09-15T19:53:12Z | **Updated:** 2026-09-15T19:53:12Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #83

### Original description

Parent: #83

Task ID: `SHOP-04.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-647"></a>
## #647 — [TASK][SHOP-04.T07] Qualify auction concurrency and live access

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/647
**Created:** 2026-09-15T19:53:18Z | **Updated:** 2026-09-15T19:53:18Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #83

### Original description

Parent: #83

Task ID: `SHOP-04.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-648"></a>
## #648 — [TASK][SHOP-05.T01] Normalize orders, shipments and receipts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/648
**Created:** 2026-09-15T19:53:32Z | **Updated:** 2026-09-15T19:53:32Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #84

### Original description

Parent: #84

Task ID: `SHOP-05.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-649"></a>
## #649 — [TASK][SHOP-05.T02] Build status refresh and notifications

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/649
**Created:** 2026-09-15T19:53:39Z | **Updated:** 2026-09-15T19:53:39Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #84

### Original description

Parent: #84

Task ID: `SHOP-05.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-650"></a>
## #650 — [TASK][SHOP-05.T03] Implement return and cancellation eligibility

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/650
**Created:** 2026-09-15T19:53:45Z | **Updated:** 2026-09-15T19:53:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #84

### Original description

Parent: #84

Task ID: `SHOP-05.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-651"></a>
## #651 — [TASK][SHOP-05.T04] Execute and verify post-purchase actions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/651
**Created:** 2026-09-15T19:53:51Z | **Updated:** 2026-09-15T19:53:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #84

### Original description

Parent: #84

Task ID: `SHOP-05.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-652"></a>
## #652 — [TASK][SHOP-05.T05] Assemble rights-aware dispute evidence

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/652
**Created:** 2026-09-15T19:53:57Z | **Updated:** 2026-09-15T19:53:57Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #84

### Original description

Parent: #84

Task ID: `SHOP-05.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-653"></a>
## #653 — [TASK][SHOP-05.T06] Implement expiring recurring purchase rules

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/653
**Created:** 2026-09-15T19:54:02Z | **Updated:** 2026-09-15T19:54:02Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #84

### Original description

Parent: #84

Task ID: `SHOP-05.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-654"></a>
## #654 — [TASK][SHOP-05.T07] Qualify recurring and compensation recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/654
**Created:** 2026-09-15T19:54:08Z | **Updated:** 2026-09-15T19:54:08Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #84

### Original description

Parent: #84

Task ID: `SHOP-05.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

