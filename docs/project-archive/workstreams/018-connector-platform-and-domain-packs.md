# Connector platform and domain packs

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 37

## Contents

- [#18 — EPIC: Connector platform and domain packs](#issue-18)
- [#45 — [P1][CONN-01] Connector SDK, registry and manifests](#issue-45)
- [#46 — [P1][CONN-02] Identity/auth lifecycle and account isolation](#issue-46)
- [#47 — [P2][CONN-03] Protocol adapters and compatibility extraction](#issue-47)
- [#48 — [P0][CONN-04] Connector health, permissions and partner access](#issue-48)
- [#231 — [TASK][EPIC-CONN.T01] Ratify connector and domain-pack contracts](#issue-231)
- [#232 — [TASK][EPIC-CONN.T02] Integrate auth, protocol and account isolation](#issue-232)
- [#233 — [TASK][EPIC-CONN.T03] Validate drift, outage and genuine handoff](#issue-233)
- [#234 — [TASK][EPIC-CONN.T04] Publish access and maintenance readiness](#issue-234)
- [#374 — [TASK][CONN-01.T01] Define manifests and domain schema registration](#issue-374)
- [#375 — [TASK][CONN-01.T02] Implement connector worker lifecycle](#issue-375)
- [#376 — [TASK][CONN-01.T03] Implement read/query and pagination contracts](#issue-376)
- [#377 — [TASK][CONN-01.T04] Implement typed action and verifier registration](#issue-377)
- [#378 — [TASK][CONN-01.T05] Implement events, freshness and rate-limit semantics](#issue-378)
- [#379 — [TASK][CONN-01.T06] Publish structured errors and compatibility diagnostics](#issue-379)
- [#380 — [TASK][CONN-01.T07] Build the reference connector and conformance kit](#issue-380)
- [#381 — [TASK][CONN-02.T01] Model accounts and connection ownership](#issue-381)
- [#382 — [TASK][CONN-02.T02] Implement supported authentication flows](#issue-382)
- [#383 — [TASK][CONN-02.T03] Broker credential storage and use](#issue-383)
- [#384 — [TASK][CONN-02.T04] Implement refresh concurrency and revocation](#issue-384)
- [#385 — [TASK][CONN-02.T05] Enforce account-scoped caches and indices](#issue-385)
- [#386 — [TASK][CONN-02.T06] Implement logout, connection removal and reauthentication](#issue-386)
- [#387 — [TASK][CONN-02.T07] Run multi-account auth conformance](#issue-387)
- [#388 — [TASK][CONN-03.T01] Implement REST and GraphQL adapter primitives](#issue-388)
- [#389 — [TASK][CONN-03.T02] Implement MCP discovery and tool/resource mapping](#issue-389)
- [#390 — [TASK][CONN-03.T03] Implement browser-resident WebMCP integration](#issue-390)
- [#391 — [TASK][CONN-03.T04] Implement local files and feed adapters](#issue-391)
- [#392 — [TASK][CONN-03.T05] Implement semantic browser compatibility extraction](#issue-392)
- [#393 — [TASK][CONN-03.T06] Add bounded visual fallback and provider UI isolation](#issue-393)
- [#394 — [TASK][CONN-03.T07] Build per-protocol drift and disconnection tests](#issue-394)
- [#395 — [TASK][CONN-04.T01] Create operation-level access and support inventory](#issue-395)
- [#396 — [TASK][CONN-04.T02] Begin partner and rights review work](#issue-396)
- [#397 — [TASK][CONN-04.T03] Integrate health and support status with the runtime](#issue-397)
- [#398 — [TASK][CONN-04.T04] Implement schema and API drift canaries](#issue-398)
- [#399 — [TASK][CONN-04.T05] Implement quota, retry-after and backoff policy](#issue-399)
- [#400 — [TASK][CONN-04.T06] Create safe production canary and sunset operations](#issue-400)
- [#401 — [TASK][CONN-04.T07] Gate advertised support and ownership](#issue-401)

---

<a id="issue-18"></a>
## #18 — EPIC: Connector platform and domain packs

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/18
**Created:** 2026-09-15T12:07:10Z | **Updated:** 2026-09-15T14:22:10Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1

Own connector contracts, account/auth isolation, protocol adapters, compatibility extraction and production-access health. Services are treated as typed data/action providers, not as the user's mandatory presentation layer.

#### Child issues
- [ ] #45 CONN-01 — Connector SDK, registry and manifests
- [ ] #46 CONN-02 — Identity/auth lifecycle and account isolation
- [ ] #47 CONN-03 — Protocol adapters and compatibility extraction
- [ ] #48 CONN-04 — Connector health, permissions and partner access

#### Cross-cutting gates
Exact read/write schemas, account isolation, truthful support levels, API/MCP/WebMCP/browser-fallback separation, schema drift/rate-limit handling and real production-access verification.

### Discussion (1 comments)

#### Comment 5681850603 — Jordan-Hall — 2026-09-15T14:22:10Z

Source: https://github.com/Jordan-Hall/browser/issues/18#issuecomment-5681850603 | Updated: 2026-09-15T14:22:10Z

<!-- intent-implementation-v1:EPIC-CONN -->
###### Workstream implementation and integration tasks

Implement #45–#48 as typed data/action connectors, preserving provider authority and account semantics.

- [ ] **EPIC-CONN.T01 — Ratify connector/domain contracts.** Define resource/action schemas, pagination, freshness, events, error/effect classes, idempotency and verification metadata. **Proof:** installing a reference connector registers capabilities without granting them.
- [ ] **EPIC-CONN.T02 — Integrate auth/protocol/account isolation.** Connect vault-backed authentication to official APIs, MCP, browser-resident WebMCP and explicit compatibility extraction. **Proof:** two accounts cannot share caches, tokens or write authority.
- [ ] **EPIC-CONN.T03 — Validate drift/outage/handoff.** Simulate rate limits, auth expiry, schema changes and missing capabilities; retain structured diagnostics and authentic Original-mode handoff. **Proof:** failures cannot silently mislabel fields or fabricate supported actions.
- [ ] **EPIC-CONN.T04 — Publish access/maintenance readiness.** Record operation-level sandbox/live approvals, terms, quotas, canaries and a maintenance owner. **Proof:** each advertised production write has actual access and conformance evidence.

**Integration demonstration:** ingest equivalent objects from two providers into one view, revoke one account, change a schema and show precise capability degradation without losing the workspace.

**Dependency correction:** partner-access research begins in P0 from draft contracts; it need not wait for the full connector SDK. API, MCP/WebMCP, DOM and visual paths must retain distinct reliability/support classifications. No protocol-discovery result is a grant.


---

<a id="issue-45"></a>
## #45 — [P1][CONN-01] Connector SDK, registry and manifests

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/45
**Created:** 2026-09-15T12:11:17Z | **Updated:** 2026-09-15T18:21:50Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #18

#### Objective
Define the typed connector contract that turns services into discoverable data/action providers without assuming one universal domain model.

#### Scope
- Connector manifest with provider/version, resource schemas, read/write actions, auth requirements and support level.
- Pagination/cursor semantics, freshness/TTL policy, subscriptions/webhooks/polling, rate limits and retry guidance.
- Error taxonomy, idempotency/reconciliation metadata, effect class, timeout and verification method per action.
- Capability registration into the policy broker.
- Provider/domain extensions alongside common resource types.
- Resettable connector contract-test harness.

#### Design rules
- Read and write capabilities are declared separately.
- Tool discovery never grants trust or authority.
- Operations expose provider/account semantics that cannot be safely normalized away.

#### Acceptance criteria
- [ ] Every operation declares exact input/output schema, scope, effect class, limits and verification strategy.
- [ ] Connector installation registers capabilities without granting them.
- [ ] Pagination, rate-limit and structured error behavior pass resettable contract tests.
- [ ] Unsupported or experimental operations expose truthful support status.
- [ ] Schema/version negotiation fails visibly rather than silently coercing incompatible data.
- [ ] Reference connector can read normalized resources into DATA-01 without losing provider IDs.

#### Dependencies
- CORE-01
- SEC-02

**First phase:** P1  
**Maturity target:** P2  
**Owner:** connectors-domains

#### Task issues
- [ ] #374 `CONN-01.T01` — Define manifests and domain schema registration
- [ ] #375 `CONN-01.T02` — Implement connector worker lifecycle
- [ ] #376 `CONN-01.T03` — Implement read/query and pagination contracts
- [ ] #377 `CONN-01.T04` — Implement typed action and verifier registration
- [ ] #378 `CONN-01.T05` — Implement events, freshness and rate-limit semantics
- [ ] #379 `CONN-01.T06` — Publish structured errors and compatibility diagnostics
- [ ] #380 `CONN-01.T07` — Build the reference connector and conformance kit

### Discussion (1 comments)

#### Comment 5682031477 — Jordan-Hall — 2026-09-15T14:31:48Z

Source: https://github.com/Jordan-Hall/browser/issues/45#issuecomment-5682031477 | Updated: 2026-09-15T14:31:48Z

<!-- intent-implementation-v1:CONN-01 -->
###### Implementation proposal — CONN-01

Create the connector SDK/host/registry over #2/#7. Connector manifests declare resources and operations; installation never grants authority. Preserve provider-specific domain extensions.

- [ ] **CONN-01.T01 — Manifests/schema registration.** Define provider/package/version, resource types, reads/writes, auth routes, destinations and support classes. **Verify:** namespaced extensions validate and conflicting descriptors fail.
- [ ] **CONN-01.T02 — Worker lifecycle.** Launch under sandbox profiles, negotiate versions, register capabilities and issue scoped request/context handles. **Verify:** crashes/resource abuse cannot escape the connector boundary.
- [ ] **CONN-01.T03 — Queries/pagination.** Implement typed get/list/search, bounded pages and opaque cursors bound to account/query/version. **Verify:** cursors cannot cross accounts; partial failure differs from exhausted results.
- [ ] **CONN-01.T04 — Actions/verifiers.** Register exact input/output/effect/idempotency/precondition/timeout/verification semantics with policy and transactions. **Verify:** absent capabilities return Unsupported, not fabricated success.
- [ ] **CONN-01.T05 — Events/freshness/quotas.** Emit deduplicated durable source events; specify revisions, TTLs, webhook/poll cursors and retry-after metadata. **Verify:** delayed/duplicate events preserve correct state and scheduler admission respects quotas.
- [ ] **CONN-01.T06 — Structured errors.** Standardize denied, unauthenticated, unavailable, rate-limited, stale, conflict, malformed and ambiguous outcomes with safe recovery guidance. **Verify:** errors retain semantics without leaking credentials.
- [ ] **CONN-01.T07 — Reference/conformance connector.** Implement a resettable local service with paging, auth expiry, events and a reversible test action. **Verify:** other domain teams can run the contract suite without production credentials.

**Interfaces:** ConnectorManifest, ResourceSchema, ActionDescriptor, PaginationContract, FreshnessPolicy, SubscriptionSpec, StructuredError and VerificationDescriptor. Source caching/quotation/redistribution policies must travel with derived records. Tool discovery or protocol compatibility is never authorization.


---

<a id="issue-46"></a>
## #46 — [P1][CONN-02] Identity/auth lifecycle and account isolation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/46
**Created:** 2026-09-15T12:11:28Z | **Updated:** 2026-09-15T18:22:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #18

#### Objective
Broker connector identity/authentication so multiple accounts can coexist without leaking data, credentials or write authority across them.

#### Scope
- OAuth/OIDC/device-code and provider-specific auth adapters where required.
- Account identity records and explicit account selector in every read/write context.
- Token refresh/rotation/revocation through the vault; never place raw secrets in model context.
- Audience/downstream restrictions for delegated credentials.
- Re-auth/expiry UX and capability suspension when auth becomes invalid.
- Per-account cache/index/source scope and connection health.
- Logout/account removal with credential purge and derivative-data policy hooks.

#### Security rules
- No token passthrough between unrelated providers/audiences.
- Connector identity and user-visible account identity must agree before consequential writes.
- Same provider with multiple accounts must remain logically and cryptographically separate where possible.

#### Acceptance criteria
- [ ] Two accounts on the same connector never reuse private cache data or write authority.
- [ ] Expired/revoked tokens stop new operations and surface actionable re-auth state.
- [ ] Raw credentials remain absent from prompts, traces and general worker environments.
- [ ] Write previews show the exact provider/account identity.
- [ ] Account removal revokes credentials and follows configured local-data deletion/retention rules.
- [ ] Cross-account confusion/adversarial fixtures fail closed.

#### Dependencies
- CONN-01
- SEC-03

**First phase:** P1  
**Maturity target:** P4  
**Owner:** connectors-domains

#### Task issues
- [ ] #381 `CONN-02.T01` — Model accounts and connection ownership
- [ ] #382 `CONN-02.T02` — Implement supported authentication flows
- [ ] #383 `CONN-02.T03` — Broker credential storage and use
- [ ] #384 `CONN-02.T04` — Implement refresh concurrency and revocation
- [ ] #385 `CONN-02.T05` — Enforce account-scoped caches and indices
- [ ] #386 `CONN-02.T06` — Implement logout, connection removal and reauthentication
- [ ] #387 `CONN-02.T07` — Run multi-account auth conformance

### Discussion (1 comments)

#### Comment 5682038066 — Jordan-Hall — 2026-09-15T14:32:09Z

Source: https://github.com/Jordan-Hall/browser/issues/46#issuecomment-5682038066 | Updated: 2026-09-15T14:32:09Z

<!-- intent-implementation-v1:CONN-02 -->
###### Implementation proposal — CONN-02

Implement connection/account/auth state separately from normalized content. Dependencies #45/#8. Token plaintext must never become an entity field or model context.

- [ ] **CONN-02.T01 — Account ownership.** Store stable provider/account IDs, display labels, profile bindings, auth state and vault handles. **Verify:** two identically named accounts remain distinct.
- [ ] **CONN-02.T02 — Authentication flows.** Implement required OAuth/OIDC/device-code adapters with validated redirect/state/nonce and PKCE where appropriate. **Verify:** callback replay, wrong origin and wrong profile fail.
- [ ] **CONN-02.T03 — Credential use.** Resolve vault handles through exact account/audience/scope/destination checks or brokered HTTP. **Verify:** tokens cannot be passed through to another provider or unrelated endpoint.
- [ ] **CONN-02.T04 — Refresh/revocation.** Serialize refresh using per-account leases, persist rotation atomically and suspend invalid grants. **Verify:** concurrent refresh and interrupted rotation do not lose credentials or restore revoked access.
- [ ] **CONN-02.T05 — Account-scoped caches.** Partition by provider/account/profile/revision and revalidate rights; show exact identity on writes. **Verify:** shared normalized IDs cannot join private data across accounts accidentally.
- [ ] **CONN-02.T06 — Logout/removal/reauth.** Revoke where supported, remove local credentials, suspend dependent workflows and apply retention; preserve unresolved receipts. **Verify:** reauth never blindly resumes an obsolete write.
- [ ] **CONN-02.T07 — Auth conformance.** Test fake providers plus granted low-risk live checks for expiry, scope rotation, callback swaps, handoff and refresh failure. **Verify:** every failure has a deterministic recovery path.

**Proposed types:** ConnectionId, ProviderAccount, AuthSession, TokenMetadata, CredentialHandle, RefreshLease and RevocationEvent. Prefer maintained standards implementations; keep provider quirks in explicit adapters. Do not assume that a valid browser login automatically supplies a connector token or equivalent action authority.


---

<a id="issue-47"></a>
## #47 — [P2][CONN-03] Protocol adapters and compatibility extraction

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/47
**Created:** 2026-09-15T12:11:39Z | **Updated:** 2026-09-15T18:23:25Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #18

#### Objective
Support multiple connector transports while preserving one internal capability/security model and a truthful degradation path when structured APIs are unavailable.

#### Scope
- Official REST/GraphQL/provider SDK adapters.
- MCP client support with explicit tool/resource/schema mapping.
- WebMCP browser-resident adapter where pages expose structured tools.
- Feed/file/local-source adapters and domain-specific protocols.
- Authenticated DOM/accessibility extraction through WEB-03 as compatibility path.
- Bounded visual extraction only as a last-resort compatibility mode.
- Optional provider-supplied UI treated as untrusted content.

#### Design rules
- Protocol/tool discovery never grants permission.
- MCP/WebMCP descriptions remain untrusted input.
- Provider-specific semantics and support limitations remain visible through the normalized interface.
- DOM/visual automation is not represented as equivalent reliability to an official API.

#### Acceptance criteria
- [ ] Same logical resource/action can be represented through supported protocol adapters without bypassing the capability broker.
- [ ] Unsupported or lossy semantics fail visibly and can route to Original view.
- [ ] Malicious MCP/WebMCP metadata cannot widen authority or override task/system policy.
- [ ] DOM fallback records origin/account/source revision and verification limits.
- [ ] Visual fallback is explicitly marked lower-confidence/compatibility mode.
- [ ] Protocol-specific conformance fixtures cover disconnects, malformed schemas and version drift.

#### Dependencies
- CONN-01
- WEB-03

**First phase:** P2  
**Maturity target:** P5  
**Owner:** connectors-domains

#### Task issues
- [ ] #388 `CONN-03.T01` — Implement REST and GraphQL adapter primitives
- [ ] #389 `CONN-03.T02` — Implement MCP discovery and tool/resource mapping
- [ ] #390 `CONN-03.T03` — Implement browser-resident WebMCP integration
- [ ] #391 `CONN-03.T04` — Implement local files and feed adapters
- [ ] #392 `CONN-03.T05` — Implement semantic browser compatibility extraction
- [ ] #393 `CONN-03.T06` — Add bounded visual fallback and provider UI isolation
- [ ] #394 `CONN-03.T07` — Build per-protocol drift and disconnection tests

### Discussion (1 comments)

#### Comment 5682043317 — Jordan-Hall — 2026-09-15T14:32:26Z

Source: https://github.com/Jordan-Hall/browser/issues/47#issuecomment-5682043317 | Updated: 2026-09-15T14:32:26Z

<!-- intent-implementation-v1:CONN-03 -->
###### Implementation proposal — CONN-03

Use protocol-specific adapters behind one internal connector/security contract, preserving assurance level and provider semantics. Dependencies #45/#35.

- [ ] **CONN-03.T01 — REST/GraphQL primitives.** Implement brokered requests, typed decoding, paging/conditional requests and structured provider errors. **Verify:** credentials and redirects remain bound to the connection/destination.
- [ ] **CONN-03.T02 — MCP mapping.** Negotiate a pinned version, map reviewed tools/resources to internal capabilities and preserve descriptor revisions. **Verify:** changed or malicious metadata cannot widen grants or override task policy.
- [ ] **CONN-03.T03 — WebMCP.** Discover tools only inside authorized page contexts; bind to origin/navigation/account and invalidate on changes. **Verify:** navigating away makes old page-tool handles unusable.
- [ ] **CONN-03.T04 — File/feed adapters.** Add scoped local sources and supported RSS/Atom/JSON feeds with canonical IDs, bounded media, freshness and retention. **Verify:** traversal and unauthorized remote-media loading are denied.
- [ ] **CONN-03.T05 — Semantic compatibility extraction.** Use the browser broker and tested extractors for API-less reads; preserve origin/account/revision and missing fields. **Verify:** a model cannot turn extraction into arbitrary privileged scripting.
- [ ] **CONN-03.T06 — Visual/provider-UI isolation.** Permit bounded fresh screenshots only when needed; label visual/inferred outputs and sandbox optional provider UI. **Verify:** provider content cannot access trusted approval callbacks.
- [ ] **CONN-03.T07 — Protocol regressions.** Record startup, version drift, malformed schemas, paging, disconnect and auth-expiry scenarios. **Verify:** fallback retains source identity and explicitly reports changed assurance.

**Reference checks:** [MCP security guidance](https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices) and [WebMCP](https://developer.chrome.com/docs/ai/webmcp). WebMCP is browser/page-context tooling, not a universal headless API. MCP, Agent Client Protocol and commerce protocols have different roles. Discovery never grants permission, and DOM/visual fallback is not automatically equivalent to an official API.


---

<a id="issue-48"></a>
## #48 — [P0][CONN-04] Connector health, permissions and partner access

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/48
**Created:** 2026-09-15T12:11:50Z | **Updated:** 2026-09-15T18:24:20Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #18

#### Objective
Treat real provider access, schema drift, quotas, legal restrictions and operational ownership as part of feature readiness—not external paperwork hidden behind an integration logo.

#### Scope
- Capability support matrix: prototype / sandbox / supervised / production / unavailable.
- Track provider application approvals, scopes, commercial terms, ToS, caching/redistribution and automation restrictions.
- Schema/API-version monitoring and drift canaries.
- Rate-limit/quota telemetry, retry-after behavior and backoff policy.
- Production test accounts and low-risk canary operations.
- Named owner and escalation path per connector.
- Sunset/deprecation workflow and truthful Original-view fallback.

#### Product rules
- Do not market an operation as supported until the production route and required permissions are actually available.
- Read support does not imply write/checkout/bid support.
- Brittle extraction paths have separate reliability/support status from official APIs.

#### Acceptance criteria
- [ ] Every advertised connector capability has a live-access record, conformance result and owner.
- [ ] Rate limits and schema/API changes create visible degraded state rather than data corruption.
- [ ] Provider revocation/deprecation can disable capability dispatch centrally.
- [ ] Cache/redistribution restrictions are enforceable by the connector/data layer.
- [ ] Canary failures automatically prevent unsafe promotion where configured.
- [ ] Product support matrix distinguishes API, MCP/WebMCP, DOM and visual-fallback paths.

#### Dependencies
- CONN-01

**First phase:** P0  
**Maturity target:** P7  
**Owner:** connectors-domains

#### Task issues
- [ ] #395 `CONN-04.T01` — Create operation-level access and support inventory
- [ ] #396 `CONN-04.T02` — Begin partner and rights review work
- [ ] #397 `CONN-04.T03` — Integrate health and support status with the runtime
- [ ] #398 `CONN-04.T04` — Implement schema and API drift canaries
- [ ] #399 `CONN-04.T05` — Implement quota, retry-after and backoff policy
- [ ] #400 `CONN-04.T06` — Create safe production canary and sunset operations
- [ ] #401 `CONN-04.T07` — Gate advertised support and ownership

### Discussion (1 comments)

#### Comment 5682048942 — Jordan-Hall — 2026-09-15T14:32:45Z

Source: https://github.com/Jordan-Hall/browser/issues/48#issuecomment-5682048942 | Updated: 2026-09-15T14:32:45Z

<!-- intent-implementation-v1:CONN-04 -->
###### Implementation proposal — CONN-04

Maintain an operation-level CapabilitySupportRecord: provider/operation/route, scopes, approval/terms evidence, version, conformance/canary status, owner and sunset plan.

- [ ] **CONN-04.T01 — Access/support inventory.** Enumerate reads, writes, checkout and bidding separately, with actual routes and required approvals. **Verify:** every advertised capability has an evidence-backed support classification.
- [ ] **CONN-04.T02 — Partner/rights work.** Prepare access applications and review allowed use, caching, redistribution and native-provider embedding. **Verify:** unresolved rights remain explicit rather than assumed granted.
- [ ] **CONN-04.T03 — Runtime health.** Publish availability/auth/quota/conformance state through #45 and trusted UI. **Verify:** transient outages differ from revoked rights and unsupported operations.
- [ ] **CONN-04.T04 — Drift canaries.** Compare low-risk live probes against pinned schema and semantic expectations; quarantine incompatible mappings. **Verify:** surprising fields cannot silently corrupt normalized values.
- [ ] **CONN-04.T05 — Quota/backoff.** Track provider/account budgets, honor retry-after, preserve cursors and spread retries. **Verify:** outages do not cause retry storms or stale-data claims of freshness.
- [ ] **CONN-04.T06 — Canary/sunset operations.** Use granted test accounts and bounded reversible checks; implement emergency disablement and authentic export/handoff. **Verify:** deprecation stops unsafe dispatch without destroying user work.
- [ ] **CONN-04.T07 — Promotion gate.** Require live access, conformance, rights/privacy review and named maintenance ownership before production support. **Verify:** capability logos cannot bypass operation-specific gates.

**Dependency correction:** start T01/T02 in P0; they do not wait for a finished SDK. #45 is an integration dependency for runtime health, not a reason to delay partner applications. API/MCP/WebMCP/DOM/visual routes retain distinct support and reliability levels. No date or provider approval is assumed by this plan.


---

<a id="issue-231"></a>
## #231 — [TASK][EPIC-CONN.T01] Ratify connector and domain-pack contracts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/231
**Created:** 2026-09-15T15:26:33Z | **Updated:** 2026-09-15T15:26:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #18

### Original description

Parent: #18

Task ID: `EPIC-CONN.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-232"></a>
## #232 — [TASK][EPIC-CONN.T02] Integrate auth, protocol and account isolation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/232
**Created:** 2026-09-15T15:26:40Z | **Updated:** 2026-09-15T15:26:40Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #18

### Original description

Parent: #18

Task ID: `EPIC-CONN.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-233"></a>
## #233 — [TASK][EPIC-CONN.T03] Validate drift, outage and genuine handoff

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/233
**Created:** 2026-09-15T15:26:45Z | **Updated:** 2026-09-15T15:26:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #18

### Original description

Parent: #18

Task ID: `EPIC-CONN.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-234"></a>
## #234 — [TASK][EPIC-CONN.T04] Publish access and maintenance readiness

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/234
**Created:** 2026-09-15T15:26:50Z | **Updated:** 2026-09-15T15:26:50Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #18

### Original description

Parent: #18

Task ID: `EPIC-CONN.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-374"></a>
## #374 — [TASK][CONN-01.T01] Define manifests and domain schema registration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/374
**Created:** 2026-09-15T18:21:07Z | **Updated:** 2026-09-15T18:21:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #45

### Original description

Parent: #45

Task ID: `CONN-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-375"></a>
## #375 — [TASK][CONN-01.T02] Implement connector worker lifecycle

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/375
**Created:** 2026-09-15T18:21:15Z | **Updated:** 2026-09-15T18:21:15Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #45

### Original description

Parent: #45

Task ID: `CONN-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-376"></a>
## #376 — [TASK][CONN-01.T03] Implement read/query and pagination contracts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/376
**Created:** 2026-09-15T18:21:20Z | **Updated:** 2026-09-15T18:21:20Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #45

### Original description

Parent: #45

Task ID: `CONN-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-377"></a>
## #377 — [TASK][CONN-01.T04] Implement typed action and verifier registration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/377
**Created:** 2026-09-15T18:21:24Z | **Updated:** 2026-09-15T18:21:24Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #45

### Original description

Parent: #45

Task ID: `CONN-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-378"></a>
## #378 — [TASK][CONN-01.T05] Implement events, freshness and rate-limit semantics

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/378
**Created:** 2026-09-15T18:21:29Z | **Updated:** 2026-09-15T18:21:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #45

### Original description

Parent: #45

Task ID: `CONN-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-379"></a>
## #379 — [TASK][CONN-01.T06] Publish structured errors and compatibility diagnostics

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/379
**Created:** 2026-09-15T18:21:35Z | **Updated:** 2026-09-15T18:21:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #45

### Original description

Parent: #45

Task ID: `CONN-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-380"></a>
## #380 — [TASK][CONN-01.T07] Build the reference connector and conformance kit

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/380
**Created:** 2026-09-15T18:21:40Z | **Updated:** 2026-09-15T18:21:40Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #45

### Original description

Parent: #45

Task ID: `CONN-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-381"></a>
## #381 — [TASK][CONN-02.T01] Model accounts and connection ownership

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/381
**Created:** 2026-09-15T18:21:55Z | **Updated:** 2026-09-15T18:21:55Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #46

### Original description

Parent: #46

Task ID: `CONN-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-382"></a>
## #382 — [TASK][CONN-02.T02] Implement supported authentication flows

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/382
**Created:** 2026-09-15T18:21:59Z | **Updated:** 2026-09-15T18:21:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #46

### Original description

Parent: #46

Task ID: `CONN-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-383"></a>
## #383 — [TASK][CONN-02.T03] Broker credential storage and use

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/383
**Created:** 2026-09-15T18:22:05Z | **Updated:** 2026-09-15T18:22:05Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #46

### Original description

Parent: #46

Task ID: `CONN-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-384"></a>
## #384 — [TASK][CONN-02.T04] Implement refresh concurrency and revocation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/384
**Created:** 2026-09-15T18:22:10Z | **Updated:** 2026-09-15T18:22:10Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #46

### Original description

Parent: #46

Task ID: `CONN-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-385"></a>
## #385 — [TASK][CONN-02.T05] Enforce account-scoped caches and indices

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/385
**Created:** 2026-09-15T18:22:16Z | **Updated:** 2026-09-15T18:22:16Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #46

### Original description

Parent: #46

Task ID: `CONN-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-386"></a>
## #386 — [TASK][CONN-02.T06] Implement logout, connection removal and reauthentication

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/386
**Created:** 2026-09-15T18:22:23Z | **Updated:** 2026-09-15T18:22:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #46

### Original description

Parent: #46

Task ID: `CONN-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-387"></a>
## #387 — [TASK][CONN-02.T07] Run multi-account auth conformance

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/387
**Created:** 2026-09-15T18:22:27Z | **Updated:** 2026-09-15T18:22:27Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #46

### Original description

Parent: #46

Task ID: `CONN-02.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-388"></a>
## #388 — [TASK][CONN-03.T01] Implement REST and GraphQL adapter primitives

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/388
**Created:** 2026-09-15T18:22:44Z | **Updated:** 2026-09-15T18:22:44Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #47

### Original description

Parent: #47

Task ID: `CONN-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-389"></a>
## #389 — [TASK][CONN-03.T02] Implement MCP discovery and tool/resource mapping

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/389
**Created:** 2026-09-15T18:22:49Z | **Updated:** 2026-09-15T18:22:49Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #47

### Original description

Parent: #47

Task ID: `CONN-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-390"></a>
## #390 — [TASK][CONN-03.T03] Implement browser-resident WebMCP integration

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/390
**Created:** 2026-09-15T18:22:54Z | **Updated:** 2026-09-15T18:22:54Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #47

### Original description

Parent: #47

Task ID: `CONN-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-391"></a>
## #391 — [TASK][CONN-03.T04] Implement local files and feed adapters

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/391
**Created:** 2026-09-15T18:22:58Z | **Updated:** 2026-09-15T18:22:58Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #47

### Original description

Parent: #47

Task ID: `CONN-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-392"></a>
## #392 — [TASK][CONN-03.T05] Implement semantic browser compatibility extraction

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/392
**Created:** 2026-09-15T18:23:03Z | **Updated:** 2026-09-15T18:23:03Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #47

### Original description

Parent: #47

Task ID: `CONN-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-393"></a>
## #393 — [TASK][CONN-03.T06] Add bounded visual fallback and provider UI isolation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/393
**Created:** 2026-09-15T18:23:06Z | **Updated:** 2026-09-15T18:23:06Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #47

### Original description

Parent: #47

Task ID: `CONN-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-394"></a>
## #394 — [TASK][CONN-03.T07] Build per-protocol drift and disconnection tests

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/394
**Created:** 2026-09-15T18:23:11Z | **Updated:** 2026-09-15T18:23:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #47

### Original description

Parent: #47

Task ID: `CONN-03.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-395"></a>
## #395 — [TASK][CONN-04.T01] Create operation-level access and support inventory

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/395
**Created:** 2026-09-15T18:23:31Z | **Updated:** 2026-09-15T18:23:31Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #48

### Original description

Parent: #48

Task ID: `CONN-04.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-396"></a>
## #396 — [TASK][CONN-04.T02] Begin partner and rights review work

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/396
**Created:** 2026-09-15T18:23:37Z | **Updated:** 2026-09-15T18:23:37Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #48

### Original description

Parent: #48

Task ID: `CONN-04.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-397"></a>
## #397 — [TASK][CONN-04.T03] Integrate health and support status with the runtime

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/397
**Created:** 2026-09-15T18:23:42Z | **Updated:** 2026-09-15T18:23:42Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #48

### Original description

Parent: #48

Task ID: `CONN-04.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-398"></a>
## #398 — [TASK][CONN-04.T04] Implement schema and API drift canaries

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/398
**Created:** 2026-09-15T18:23:50Z | **Updated:** 2026-09-15T18:23:50Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #48

### Original description

Parent: #48

Task ID: `CONN-04.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-399"></a>
## #399 — [TASK][CONN-04.T05] Implement quota, retry-after and backoff policy

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/399
**Created:** 2026-09-15T18:23:56Z | **Updated:** 2026-09-15T18:23:56Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #48

### Original description

Parent: #48

Task ID: `CONN-04.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-400"></a>
## #400 — [TASK][CONN-04.T06] Create safe production canary and sunset operations

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/400
**Created:** 2026-09-15T18:24:02Z | **Updated:** 2026-09-15T18:24:02Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #48

### Original description

Parent: #48

Task ID: `CONN-04.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-401"></a>
## #401 — [TASK][CONN-04.T07] Gate advertised support and ownership

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/401
**Created:** 2026-09-15T18:24:08Z | **Updated:** 2026-09-15T18:24:08Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #48

### Original description

Parent: #48

Task ID: `CONN-04.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

