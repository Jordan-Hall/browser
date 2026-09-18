# Pull-request context register

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

PR titles, descriptions, branch/commit metadata and conversation comments only; not a code review or CI verification.

<a id="issue-785"></a>
## #785 — CORE-01.T01: bootstrap Rust workspace and architecture checks

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/785
**Created:** 2026-09-16T00:15:44Z | **Updated:** 2026-09-17T21:05:42Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implements the workspace/checker portion of **#121 (`CORE-01.T01`)**, under #2, CORE epic #13 and programme #1. Integration and the complete individual task ledger: #802, [CORE task coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**Source updated and verification passed; not an approval or CORE completion claim.** This branch now owns its hardening changes rather than depending on changes left only in #802.

#### Current implementation

- Rust 1.98.1 / edition 2024 / resolver v3 workspace with contracts, IPC, fixture worker and xtask packages.
- Contract dependency checks reject unclassified direct packages, path/workspace dependencies and unreviewed sources. A regression executes the real checker with a forbidden package renamed to an allowed dependency name.
- Committed stage-specific Cargo.lock and locked CI commands.
- Explicit documentation tests; independent gate outcomes and source/lockfile evidence retained even when another gate fails.
- No future runtime packages, schemas or fuzz jobs are imported before their owning task.

#### Stack and evidence

Base: `main`. Next: #786. Updated head: `66a41acb6fb9398ca35ced24d2d0f971ba12165b`.

[Passing CI run 35274507618](https://github.com/Jordan-Hall/browser/actions/runs/35274507618). The same source tree was independently checked locally with the verified Rust 1.98.1 offline toolchain: formatting, Clippy with warnings denied, **5 runtime tests**, documentation-test invocation and the actual architecture checker passed. No doctests exist at this first stage; the ID-separation doctest is introduced in #786.

The branch update is a fast-forward preserving its original history. No PR merge, issue closure or blanket review-thread resolution was performed. This checker covers its declared direct dependency/source policy; it is not a transitive supply-chain audit. Independent review and integrated task acceptance remain required.

### Discussion (3 comments)

#### Comment 5689994478 — chatgpt-codex-connector[bot] — 2026-09-16T00:15:53Z

Source: https://github.com/Jordan-Hall/browser/pull/785#issuecomment-5689994478 | Updated: 2026-09-16T00:18:36Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T00:18:34.338985Z">2026-09-16T00:18:34.338985Z</relative-time> | `fc7b624` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705764537 — Jordan-Hall — 2026-09-16T23:02:13Z

Source: https://github.com/Jordan-Hall/browser/pull/785#issuecomment-5705764537 | Updated: 2026-09-16T23:02:13Z

<!-- intent-core-review:2026-09-16:pr-785 -->
###### CORE review — CORE-01.T01

Reviewed head `fc7b62456f0d5feaa4f4c98d5be1953604328264`, its patch, CI configuration and existing discussion against #121 / #2. This is a review, not an implementation-completion or merge approval.

###### P2 — Architecture validation is not fail-closed
The existing finding at `discussion_r4021391129` remains valid: `is_forbidden_contract_dependency` rejects path dependencies and a finite name-prefix list but accepts unclassified registry dependencies. A GUI/provider dependency with a different name passes. `cargo metadata --no-deps` also does not inspect the resolved transitive graph. Use a reviewed direct-dependency allowlist (including package identity/source, not aliases), and explicitly define whether transitive and build dependencies are covered. Do not describe a name denylist as a complete architecture boundary.

**Acceptance:** tests inject an unlisted GUI dependency, a renamed forbidden package, and a forbidden transitive dependency under the chosen policy; each intended violation must fail the actual checker, not just a helper predicate.

###### P2 — Builds are not reproducible from the pinned compiler alone
The patch has no `Cargo.lock`; dependencies use compatible version ranges, and CI does not pass `--locked`. The later stack still resolves dependencies during CI. Commit the application workspace lockfile and enforce it for build/test/lint/conformance commands. Pin third-party Actions to reviewed full commit SHAs as a separate supply-chain improvement.

###### P2 — The advertised type-safety doctest is outside this CI test command
`cargo test --workspace --all-targets` does not run library documentation tests. T02 adds a `compile_fail` doctest, so add an explicit `cargo test --workspace --doc --locked` gate. Also distinguish `--all-targets` from cross-OS target coverage: this workflow currently runs only Ubuntu.

**Recommended evidence:** clean-checkout CI with a committed lockfile; a deliberately broken compile-fail example causing CI failure; documented Linux/macOS/Windows qualification status. Keep the initial bootstrap scope small, but do not use this green job as proof that later transport/runtime platforms are qualified.

No code, branch, issue state or review-thread resolution was changed by this review.

#### Comment 5706416708 — Jordan-Hall — 2026-09-17T00:11:09Z

Source: https://github.com/Jordan-Hall/browser/pull/785#issuecomment-5706416708 | Updated: 2026-09-17T00:11:09Z

###### Review follow-up — turn the existing findings into executable release gates

Rechecked `fc7b62456f0d5feaa4f4c98d5be1953604328264` against #121 / #2 and the existing review. The lockfile, architecture-policy and doctest findings remain applicable; this is not a fix or approval.

**Recommended implementation order:** commit the resolved application lockfile; make CI and `intent-xtask` use locked resolution; then add a fixture workspace that invokes the actual architecture-check binary and deliberately adds a disallowed dependency. Testing only `is_forbidden_contract_dependency` does not prove Cargo metadata is interpreted correctly. Keep direct, transitive, build and target-specific dependency policies explicit rather than treating them as interchangeable.

**Additional maintenance improvement:** the compiler version is duplicated in `rust-toolchain.toml` and the workflow. Resolve installation from the checked-in toolchain file, or add a drift test. Otherwise a later toolchain update can install one version while rustup selects another.

**Completion evidence:** clean-checkout locked build, explicit doctest run, negative architecture fixture, and an identified platform matrix. `--all-targets` here is not a cross-platform test. Preserve the safe-code lint defaults; do not suppress lint failures to get the stack green.

No repository code or issue state changed in this review.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "main",
  "base_sha": "09197a237a57d53df798221225356e8abfee394a",
  "head_branch": "core-01-t01-bootstrap",
  "head_sha": "66a41acb6fb9398ca35ced24d2d0f971ba12165b",
  "merge_commit_sha": "070a559f06b64cdb760f007c7ee40794de1f9d1e"
}
```

<a id="issue-786"></a>
## #786 — CORE-01.T02: define typed identities and validated value objects

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/786
**Created:** 2026-09-16T00:18:44Z | **Updated:** 2026-09-17T21:06:03Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation for **#122 (`CORE-01.T02`)**, under #2, CORE epic #13 and programme #1. Full individual task/acceptance inventory: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**Hardening is now committed on this owning branch. Task acceptance and compatibility sign-off remain open.**

#### Current implementation

- Non-interchangeable UUID-backed domain IDs, with an executed compile-fail ID-separation doctest.
- UTF-8-byte-bounded text, validated timestamps, byte sizes, canonical currency codes and content hashes, and separately typed account-qualified provider identifiers.
- Full signed i128 monetary amounts round-trip through JSON values using canonical decimal strings. Regressions cover extrema, values beyond JavaScript's exact integer range, invalid spellings and out-of-range input.
- Explicit known/unknown currency scale remains intact; signed monetary values are not execution authorization.
- Incorporates the updated #785 architecture checks, locked build inputs and CI evidence handling.

#### Stack and verification

Base: #785 / `core-01-t01-bootstrap`. Next: #787. Head: `dfcde3730932799c77ff85ae1911197e86c4e95c`.

[Passing CI run 35274516793](https://github.com/Jordan-Hall/browser/actions/runs/35274516793). Local pinned-toolchain verification of this exact source tree also passed formatting, warnings-denied Clippy, **17 runtime tests and 1 doctest**, and the actual architecture checker. Counts are cumulative at this stack stage.

#### Remaining acceptance

The numeric-to-decimal-string money representation is an intentional pre-release wire change, not a backwards-compatible data migration. Existing numeric-money records need an explicitly tested migration/version policy before import. Provider-resource validation and the authority/contract baseline still require review. IDs, content hashes and successful parsing do not grant permission.

Original branch history is preserved without force-pushing. No issue is closed and no PR is merged or approved by this update.

### Discussion (3 comments)

#### Comment 5690040593 — chatgpt-codex-connector[bot] — 2026-09-16T00:18:52Z

Source: https://github.com/Jordan-Hall/browser/pull/786#issuecomment-5690040593 | Updated: 2026-09-16T00:22:56Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T00:22:55.653820Z">2026-09-16T00:22:55.653820Z</relative-time> | `3ce38e3` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705782632 — Jordan-Hall — 2026-09-16T23:04:24Z

Source: https://github.com/Jordan-Hall/browser/pull/786#issuecomment-5705782632 | Updated: 2026-09-16T23:04:24Z

<!-- intent-core-review:2026-09-16:pr-786 -->
###### CORE review — CORE-01.T02

Reviewed head `a9fbf6648214552dd4ab4431ad8d2fde07f2a5ff`, the typed-value patch and existing discussion against #122 / #2; also checked the actual envelope consumer introduced by #788. No implementation changes were made.

###### P2 — Preserve the full monetary range through the real codec
`Money.minor_units` is `i128`, but the downstream control decoder first materializes `serde_json::Value` and the workspace does not enable `arbitrary_precision`. Values outside the `i64::MIN..=u64::MAX` Number range cannot retain the same integer representation through that intermediate form. A direct Money round-trip test is insufficient for the declared contract.

**Change:** choose and version one exact representation: canonical decimal strings for wide integers, or an explicitly configured lossless number path end-to-end. Keep unknown scale non-executable for comparisons/charges until a trusted source resolves it. Do not silently cast money to floating point.

**Regression tests:** `i128::MIN/MAX`, `u64::MAX + 1`, values around 2^53, negative amounts, and unknown scale through `Envelope<Money>` and `Envelope<GoalContract>`; verify both value equality and canonical digest stability. This is a source-level interoperability finding; Rust was not executed locally in this review. Primary reference: https://docs.rs/serde_json/latest/serde_json/struct.Number.html#method.from_i128.

###### P2 — Bounded text does not make a valid identifier
`ProviderId`, `ProviderAccountId` and `ProviderResourceId` currently accept an empty string because their shared constructor only checks maximum byte length. That does not satisfy the issue's validated account-qualified references. Define a non-empty identifier invariant separately from general-purpose `BoundedText`; preserve provider-specific case/Unicode semantics rather than globally trimming or lowercasing IDs. Decide explicitly whether nil UUIDs are valid durable identities.

**Tests:** empty identifiers rejected on construction and deserialization; valid Unicode boundaries; account A/B references with the same resource string remain different; no identifier possession implies authorization.

###### Completion evidence improvement
Run the compile-fail doctest explicitly in CI (see #785), add generated/property coverage rather than calling only example-based tests property tests, and keep wire-domain values separate from narrower SQLite/JavaScript storage representations with checked conversions.

#### Comment 5706419337 — Jordan-Hall — 2026-09-17T00:11:28Z

Source: https://github.com/Jordan-Hall/browser/pull/786#issuecomment-5706419337 | Updated: 2026-09-17T00:11:28Z

###### Review follow-up — separate representable values from executable amounts

Rechecked `a9fbf6648214552dd4ab4431ad8d2fde07f2a5ff` and the later envelope consumer; owning task #122, parent #2. The existing wide-integer/empty-provider-ID findings remain open in the source.

A useful refinement to the proposed fix: **do not ban negative amounts from general `Money`**—refunds and adjustments can be signed. Introduce a separate validated spending/budget amount at the authority boundary, requiring a nonnegative value, a resolved currency scale and an exact account/quote binding. Likewise, a syntactically valid three-letter `CurrencyCode` is not proof that a merchant accepts that currency.

Use one round-trip matrix through construction, JSON, `Envelope<Money>`, persistence conversion and display: zero, negative adjustment, `i128::MIN/MAX`, `u64::MAX + 1`, and values around 2^53. Verify rejected values cannot fall back to floating point or silently change a digest. The current money test named `money_round_trip_preserves_unknown_or_known_scale` exercises only `Known`; add an actual `Unknown` case and its non-executable boundary.

For provider identifiers, reject empty identifiers without globally trimming, lowercasing or Unicode-normalizing opaque provider values. Keep that invariant separate from `BoundedText`, where an empty value can be valid.

These are review requirements and source-derived test gaps, not locally executed Rust test results.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-01-t01-bootstrap",
  "base_sha": "66a41acb6fb9398ca35ced24d2d0f971ba12165b",
  "head_branch": "core-01-t02-typed-values",
  "head_sha": "dfcde3730932799c77ff85ae1911197e86c4e95c",
  "merge_commit_sha": "1221ffa3b9f02bcb8f33cba4eb11da81b8851090"
}
```

<a id="issue-787"></a>
## #787 — CORE-01.T03: specify durable v1 core record schemas

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/787
**Created:** 2026-09-16T00:25:00Z | **Updated:** 2026-09-17T21:06:22Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation for **#123 (`CORE-01.T03`)**, parent #2, CORE epic #13, programme #1. Integration and the individual task ledger: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**This branch now includes the reviewed schema hardening. Complete schema-family and authority acceptance remain open.**

#### Current implementation

Versioned Rust records cover goals, workspaces, tasks, capabilities, observations/evidence, proposals/approvals, operations/receipts, views, memory and artifact references. Unknown external outcomes remain explicit; provider sessions are secondary references.

ActionProposal construction derives its argument hash from the canonical artifact reference instead of accepting inconsistent redundant input. Deserialization rejects mismatched hashes and unsupported proposal fields. Regression tests cover both constructor and wire paths. The updated typed-value/money contract from #786 is included on this branch.

#### Stack and evidence

Base: #786 / `core-01-t02-typed-values`. Next: #788. Head: `43dbf5b2a3bdf873f3d2f5a0de1027da1099b0ee`.

[Passing CI run 35274547485](https://github.com/Jordan-Hall/browser/actions/runs/35274547485). Local Rust 1.98.1 verification of the same tree passed formatting, warnings-denied Clippy, **23 runtime tests and 1 doctest**, and architecture checks. Test totals are cumulative for this stage.

#### Remaining acceptance and compatibility

The ActionProposalDescriptor API no longer takes the redundant arguments_hash field; dependent call sites must use the new constructor contract. Hash equality does not verify underlying blob bytes and is not authorization. Complete record-family golden fixtures/migration coverage, approval expiry/identity, account/capability/target/source-precondition semantics and explicit unknown-field policy for all authority-bearing records remain integration gates. This PR does not establish those properties merely by round-tripping records.

The update preserves original history and the updated predecessor without force-pushing. No merge, approval, issue closure or blanket resolution of review findings is claimed.

### Discussion (3 comments)

#### Comment 5690147422 — chatgpt-codex-connector[bot] — 2026-09-16T00:25:13Z

Source: https://github.com/Jordan-Hall/browser/pull/787#issuecomment-5690147422 | Updated: 2026-09-16T00:28:45Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T00:28:44.812342Z">2026-09-16T00:28:44.812342Z</relative-time> | `70ae02e` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705786011 — Jordan-Hall — 2026-09-16T23:04:49Z

Source: https://github.com/Jordan-Hall/browser/pull/787#issuecomment-5705786011 | Updated: 2026-09-16T23:04:49Z

<!-- intent-core-review:2026-09-16:pr-787 -->
###### CORE review — CORE-01.T03

Reviewed head `7c9d1e441b55216d3f5f0249d5605fd1b8dd0bf9`, `records.rs`, tests and existing review against #123 / #2. Review only; no fix is being claimed.

###### P1 — Canonical bytes and approved argument digest can disagree
The existing finding `discussion_r4021444257` is still present. `ActionProposalDescriptor` independently supplies `canonical_arguments` and `arguments_hash`; `ActionProposal::new` copies both, and derived `Deserialize` accepts a mismatch. Consequently the record cannot guarantee that the artifact to execute is the one whose digest was approved.

**Change:** derive the argument digest from the verified canonical artifact, or use a fallible validated constructor plus equivalent custom deserialization. At dispatch, additionally bind the account, capability, target and exact approved operation; a well-formed DTO is not itself authorization.

**Tests:** deliberately different artifact/proposal hashes through both construction and JSON; each must fail. Verify a matching proposal remains stable through persistence and IPC.

###### P2 — Authority-relevant unknown fields are silently discarded
The v1 records generally derive `Deserialize` without `deny_unknown_fields` or an explicit extension-preservation mechanism. A same-version misspelled or unsupported restriction is accepted and omitted on reserialization. Decide field policy by record family: reject unknown authority-bearing fields, and preserve explicitly extensible presentation metadata without interpreting it as permission. Add negative fixtures for unknown approval/action fields, invalid approval time windows, and oversized collections on non-IPC import paths.

###### Integration gap — One operation truth, not two incompatible enums
The subsequent state layer (#794) introduces `DurableOperationState` with approved/pending/attempting/accepted/verified/compensation states, while this wire `OperationState` has a different shape and success requires a receipt ID. There is no explicit lossless mapping in the reviewed stack. Specify the authoritative state machine and checked projection into public contracts; never collapse accepted-but-unverified into succeeded. Also reconcile artifact media-type bounds (wire: 128 bytes; store: 255 bytes).

###### Coverage before schema freeze
There are only a few example record tests, not fixtures for every listed family and invalid invariant. Add per-family canonical fixtures, validated collection limits, checked state/time/hash invariants and migrations/unsupported-version cases. Keep future API affordances (getters/builders, versioned updates) usable without round-tripping through JSON to modify private fields.

Recommended disposition: changes required before these schemas are treated as the frozen authorization/durability baseline.

#### Comment 5706421555 — Jordan-Hall — 2026-09-17T00:11:45Z

Source: https://github.com/Jordan-Hall/browser/pull/787#issuecomment-5706421555 | Updated: 2026-09-17T00:11:45Z

###### Review follow-up — define the exact authorization binding, not just two equal hashes

Rechecked `7c9d1e441b55216d3f5f0249d5605fd1b8dd0bf9`, particularly `ActionProposalDescriptor`, `ActionProposal::new`, `Approval` and their derived deserialization; owning task #123 / parent #2. The mismatched canonical-artifact and argument-hash finding is still present.

When fixing it, specify **what is hashed and which fields approval authorizes**. Equality between the artifact digest and `arguments_hash` fixes the immediate inconsistency, but the dispatch binding must also include the intended account, capability, target, source preconditions, expiry and canonicalization/schema version. Identical JSON bytes used for another account must not inherit approval.

Prefer separate types for an untrusted proposal DTO, a validated canonical action binding, and a broker-issued dispatch authorization. A public constructor or successfully deserialized `ApprovalState::Approved` must not itself grant authority. Remove redundant digest inputs where the digest can be derived, and apply the same validation on import/deserialization as on construction.

**Regression matrix:** mismatched artifact/hash; same bytes with changed account/capability/target; expired or backwards approval window; duplicate/unknown authority fields; unsupported canonicalization version; a matching valid action. Rejection must occur before outbox staging (#131/#795), with no journal or pending-dispatch residue.

Also add explicit checked conversion between this `OperationState` and the durable state machine from #794. `Accepted` must not project to `Succeeded` without the required verification/receipt.

No fix or production authorization implementation is claimed by this comment.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-01-t02-typed-values",
  "base_sha": "dfcde3730932799c77ff85ae1911197e86c4e95c",
  "head_branch": "core-01-t03-record-schemas",
  "head_sha": "43dbf5b2a3bdf873f3d2f5a0de1027da1099b0ee",
  "merge_commit_sha": "c26c75c288c361bc62aa87e3f373263271c0fb04"
}
```

<a id="issue-788"></a>
## #788 — CORE-01.T04: add bounded wire framing and stable errors

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/788
**Created:** 2026-09-16T00:34:29Z | **Updated:** 2026-09-17T21:06:59Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation for **#124 (`CORE-01.T04`)**, parent #2, CORE epic #13 and programme #1. Integrated coverage and all individual task references: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**The bounded-codec hardening is now on this branch. Full protocol/runtime qualification remains open.**

#### Current implementation

- Explicit envelopes and length-prefixed control/artifact frames with fragmentation, malformed-input and structural-budget checks.
- Control output is bounded while serialization writes, rather than only after allocating the complete encoded payload.
- Envelope schema is validated before payload interpretation, preserving UnsupportedSchema for an incompatible version even when its payload does not match the current type.
- Unknown envelope fields are rejected; JSON decoding applies node, collection and nesting budgets.
- New regression rejects duplicate object keys, including equivalent escaped spellings, at any object depth before typed decoding. Identical key names in distinct objects remain valid.
- Canonical wide-integer money round-trips through the real Envelope codec, using the updated #786 contract.

#### Stack and evidence

Base: #787 / `core-01-t03-record-schemas`. Next: #789. Head: `f9de65f8342b4d19d907983331a299f7c46c6e7b`.

[Passing CI run 35274566964](https://github.com/Jordan-Hall/browser/actions/runs/35274566964). Local pinned Rust 1.98.1 verification passed formatting, warnings-denied Clippy, **44 runtime tests and 1 doctest**, and architecture checks. Counts are cumulative at this stage.

#### Remaining acceptance

The public error enum's stable on-wire representation still needs an explicit contract decision; adding a Rust discriminant does not itself establish numeric JSON encoding. Instrumented fuzzing, authenticated version-negotiated sessions and real worker deadline/cancellation propagation are separate open gates. This codec is not an authenticated transport and the successful parsing of a proposal does not authorize execution.

History and the updated predecessor are preserved without force-pushing. No issue closure, PR merge or blanket review resolution is claimed.

### Discussion (3 comments)

#### Comment 5690232804 — chatgpt-codex-connector[bot] — 2026-09-16T00:34:40Z

Source: https://github.com/Jordan-Hall/browser/pull/788#issuecomment-5690232804 | Updated: 2026-09-16T00:38:12Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T00:38:12.187603Z">2026-09-16T00:38:12.187603Z</relative-time> | `0d64d89` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705788287 — Jordan-Hall — 2026-09-16T23:05:05Z

Source: https://github.com/Jordan-Hall/browser/pull/788#issuecomment-5705788287 | Updated: 2026-09-16T23:05:05Z

<!-- intent-core-review:2026-09-16:pr-788 -->
###### CORE review — CORE-01.T04

Reviewed head `f76de003464148e179fd908aced770956a7d1352`, framing/envelope/error code and prior review against #124 / #2. These are source-level findings; no code was changed or Rust test run locally.

###### Existing findings revalidated — P2
All three earlier findings remain visible in this head:
- `discussion_r4021483294`: schema validation occurs after deserializing `Envelope<T>`, so an incompatible payload can return `InvalidEnvelope` before `UnsupportedSchema`.
- `discussion_r4021483296`: `serde_json::to_vec` builds the entire encoded body before the size check. Use a capped writer that stops emission before allocating beyond the configured output budget.
- `discussion_r4021483299`: derived Serde on `#[repr(u16)] WireErrorCode` serializes variant names, not the documented numeric discriminants. Add an explicit numeric codec and golden tests.

###### Additional P2 — Lossless, unambiguous JSON needs enforcement before Value normalization
`decode_control` first parses into `Value`. Duplicate object keys are collapsed before typed deserialization can reject duplicate authority fields. The same intermediate Number representation conflicts with the full `i128` money contract (see #786). Define duplicate-key rejection and an exact-number path, with schema-envelope preflight before decoding a version-specific payload.

**Tests:** duplicated `account_id`, `schema_version`, cancellation ID and approval fields in both orders; wide money values; unknown-version payloads that are invalid for current T. Require a stable error and no accepted action on ambiguous input.

###### Framing/error-path contract needs tightening
If `push` decodes a valid frame and then encounters a malformed header in the same feed, `Err` discards the already collected frames and exposes no consumed offset. Declare malformed framing fatal and poison/close the channel, or return a structured partial batch plus terminal error. Do not reset and silently resume an ambiguous byte stream. Add valid+invalid+valid batch tests and truncated-EOF tests.

###### Bounds and control-plane qualification
Per-frame bytes and per-feed frame counts are not aggregate connection/process memory budgets. The AST collection/node checks occur after Value allocation. Add checked configuration ceilings, aggregate byte accounting, bounded parse allocation and multi-connection pressure tests. Lane tags on one byte stream do not let stop overtake a partially received artifact frame; prove separate transport/control capacity or bounded chunking/deadlines in #127/#144 rather than inferring priority from the enum alone.

Keep length-before-allocation and complete-frame emission; those are good foundations. The completion gate should include the negative codec cases above, not only successful fragmentation tests.

#### Comment 5706423641 — Jordan-Hall — 2026-09-17T00:12:01Z

Source: https://github.com/Jordan-Hall/browser/pull/788#issuecomment-5706423641 | Updated: 2026-09-17T00:12:01Z

###### Review follow-up — require encode/decode closure under the same limits

Rechecked `f76de003464148e179fd908aced770956a7d1352` against #124 / #2 and the existing review. In addition to the already documented serializer allocation, schema ordering, numeric errors and duplicate-key problems, make this property explicit:

> A supported envelope successfully encoded under a validated `WireLimits` must decode under those same limits without losing information.

`encode_control` currently checks only serialized byte length, whereas `decode_control` additionally applies depth, collection and node limits. A small but deeply nested or over-entry-count payload can therefore be accepted by the encoder and rejected by its matching decoder. The same issue appears with supported-value ranges passing through `serde_json::Value`.

**Improvement:** share a validated wire policy between the version-specific encoder and decoder; enforce output size while writing; reject duplicate authority keys before map normalization; define how structural budgets count the envelope as well as payload. Do not solve this by increasing limits without aggregate memory accounting.

**Tests:** encode/decode at each exact boundary and one over; a valid frame followed by a malformed header in the same feed; truncated EOF; an unsupported-version payload invalid for current `T`; a stalled artifact transfer while cancellation is sent. Declare malformed framing terminal or return an explicit partial-result-plus-terminal-error contract—callers must not guess which prefix was consumed.

This is source review and proposed regression coverage, not a claim that the tests were run or the findings fixed.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-01-t03-record-schemas",
  "base_sha": "43dbf5b2a3bdf873f3d2f5a0de1027da1099b0ee",
  "head_branch": "core-01-t04-wire-framing",
  "head_sha": "f9de65f8342b4d19d907983331a299f7c46c6e7b",
  "merge_commit_sha": "00a93be964d455aad82a29a998440ee303d7122e"
}
```

<a id="issue-789"></a>
## #789 — CORE-01.T05: authenticate locally launched worker channels

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/789
**Created:** 2026-09-16T00:55:06Z | **Updated:** 2026-09-17T21:07:21Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation primitives for **#125 (`CORE-01.T05`)**, parent #2, CORE epic #13 and programme #1. Integrated task ledger: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**Updated source and passing checks do not yet satisfy launched-worker authentication acceptance.**

#### Current implementation

Local transport types model bootstrap tokens, worker roles, instance identities, expected peers and allowed message families. The one-shot authenticator validates its local launch record and has negative cases for invalid bootstrap/role/instance inputs.

This update removes Clone from WorkerLaunchRecord and adds a compile-fail regression preventing implicit duplication of that consumable authorization record. It also incorporates the updated #788 bounded/duplicate-key-rejecting wire codec and preceding contract fixes.

#### Stack and evidence

Base: #788 / `core-01-t04-wire-framing`. Next: #790. Head: `56085134c7daeb6569ee7ccfc71906ca01fb8250`.

[Passing CI run 35274576263](https://github.com/Jordan-Hall/browser/actions/runs/35274576263). Local Rust 1.98.1 checks also passed formatting, warnings-denied Clippy, **51 runtime tests and 2 doctests**, and architecture checks. Counts are cumulative.

#### Blocking integration requirements

A non-cloneable value is not a supervisor-owned atomic one-use registry. Global replay/revocation/expiry protection, live child/epoch binding, a bounded handshake and an actual spawned fixture worker remain required with #137 and #138.

The inherited-socketpair credential test must not be mistaken for verification of the launched child PID. Establish the production transport's process-binding scheme and prove intended-child acceptance plus wrong-child, replay, role-escalation and replaced-epoch rejection. Synthetic/deserialized evidence must not establish production authority. This PR does not claim same-user process isolation or cross-platform authentication qualification.

The original branch history and updated predecessor are preserved without force-pushing; no issue closure, merge, approval or blanket review-thread resolution was performed.

### Discussion (3 comments)

#### Comment 5690387606 — chatgpt-codex-connector[bot] — 2026-09-16T00:55:14Z

Source: https://github.com/Jordan-Hall/browser/pull/789#issuecomment-5690387606 | Updated: 2026-09-16T00:58:21Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T00:58:20.727555Z">2026-09-16T00:58:20.727555Z</relative-time> | `de8d5d8` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705790797 — Jordan-Hall — 2026-09-16T23:05:23Z

Source: https://github.com/Jordan-Hall/browser/pull/789#issuecomment-5705790797 | Updated: 2026-09-16T23:05:23Z

<!-- intent-core-review:2026-09-16:pr-789 -->
###### CORE review — CORE-01.T05

Reviewed head `45c4fef9dbb6e17bf3ec06d2458d0545f679d390`, auth/peer implementations and prior discussion against #125 / #2.

###### P1 — The socketpair credential model does not identify an inherited worker endpoint
The existing finding `discussion_r4021570265` remains applicable. The only live socket test creates both endpoints in one process and expects the creator PID. That is not a launched-child authentication test.

I independently reproduced the Linux OS behavior in this review using Python `socketpair`, an actual subprocess with an inherited endpoint, and `SO_PEERCRED`: the child sent its own PID, but the peer credentials reported the creator's different PID. This is an OS-level reproduction of the design pattern, not execution of the Rust crate.

**Change:** choose a post-spawn authenticated connection in a protected endpoint namespace, or explicitly use inherited-channel possession as the primary bootstrap property and corroborate process identity through a suitable process/per-message mechanism. Do not satisfy the check by substituting the supervisor PID and continue claiming worker-PID verification.

**Acceptance:** real spawned-worker/exec tests, wrong-process endpoint theft, reused launch token, role mismatch, timeout, worker replacement and fresh-epoch tests. Linux/macOS/Windows each need their own evidence.

###### P2 — One-shot needs a registry invariant, not only consuming one object
`WorkerLaunchRecord` and its token are clonable. Two authenticators can be constructed from copies of the same record, so consuming `self` does not establish globally one-use launch authorization. Consume a registry-owned launch entry/nonce atomically, bind it to the channel and epoch, and reject reauthentication after success or revocation. Specify bootstrap expiry and scrub secrets on disposal.

###### Hardening / integration requirements
`PeerCredentialEvidence` is publicly constructible/deserializable and includes `Synthetic`; `AnyLocal` accepts every evidence value. Keep OS-observed evidence opaque at the production authentication boundary and isolate synthetic credentials in fixtures. This is an API-misuse risk, not a claim that an external process can already call a privileged endpoint. Message-family checks also need direction/operation/instance scope: generic lifecycle permission must not mean permission to stop arbitrary peers.

The Windows implementation extracts a client PID but does not yet establish endpoint ACLs, principal/token checks, server identity or a real named-pipe handshake. The small unsafe wrapper is isolated, but Unix-only CI is not Windows qualification. Prefer lifetime-bearing handle interfaces where possible and document each unavoidable FFI safety obligation.

No implementation changes or thread resolutions were made; changes and cross-process evidence are required before authentication is signed off.

#### Comment 5706425677 — Jordan-Hall — 2026-09-17T00:12:17Z

Source: https://github.com/Jordan-Hall/browser/pull/789#issuecomment-5706425677 | Updated: 2026-09-17T00:12:17Z

###### Review follow-up — make authenticated identity a channel-bound result

Rechecked `45c4fef9dbb6e17bf3ec06d2458d0545f679d390` (`auth.rs`, `peer.rs`) and the existing review; owning task #125 / parent #2. The inherited socketpair/PID and clonable one-shot-record findings are not fixed in this head.

The next implementation should expose an **authenticated channel**, not an identity that callers can assemble from a deserializable `PeerCredentialEvidence`. Keep the OS-observed peer evidence private to the transport adapter; atomically consume a supervisor-owned launch entry, bind the result to the connection and fresh worker generation, and explicitly reject synthetic evidence in production entry points.

Split `LifecycleControl` into direction- and instance-scoped operations at dispatch: a worker reporting its own heartbeat/ack is different from a supervisor issuing Stop or Launch. The current family allowlist gives all roles `LifecycleControl`; by itself it cannot establish who may control which process. This is an integration requirement, not evidence of an already exposed remote exploit.

**Required positive/negative pair:** a real launched worker must authenticate and exchange a permitted message, while a different process/role, a second use of the same launch record, an expired bootstrap and a previous generation fail. Include a channel-close-during-handshake case and ensure a failed authentication cannot consume another worker's authorization.

Keep OS credentials as corroboration of the selected launch model, not a substitute for containment. No code was changed or review thread marked fixed.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-01-t04-wire-framing",
  "base_sha": "f9de65f8342b4d19d907983331a299f7c46c6e7b",
  "head_branch": "core-01-t05-worker-auth",
  "head_sha": "56085134c7daeb6569ee7ccfc71906ca01fb8250",
  "merge_commit_sha": "b796861d783b03fb1612081ee0178c1f18835159"
}
```

<a id="issue-790"></a>
## #790 — CORE-01.T06: add protocol negotiation and schema evolution

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/790
**Created:** 2026-09-16T01:05:12Z | **Updated:** 2026-09-17T21:07:38Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation for **#126 (`CORE-01.T06`)**, parent #2, CORE epic #13 and programme #1. Full individual task ledger and integration: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**Negotiation primitives and local codec constraints are implemented; authenticated-session and durable-evolution acceptance remain open.**

#### Current implementation

- Explicit protocol ranges, capabilities, selection results and migration outcomes.
- Protocol offer construction bounds iterator consumption before collecting; an endless repeated capability terminates at the admission limit.
- Wire offer collections are bounded during deserialization, including repeated entries, rather than allocated without limit before validation.
- ControlCodec advertises only the actually implemented v1.0 envelope representation. The generic range algorithm is not itself permission to advertise unsupported wire semantics.
- Incorporates all updated predecessor contracts, strict bounded JSON parsing and launch-record changes.

#### Stack and evidence

Base: #789 / `core-01-t05-worker-auth`. Next: #791. Head: `ae25e5123cae06c9d9ea6c1775b8c46373adb96a`.

[Passing CI run 35274585080](https://github.com/Jordan-Hall/browser/actions/runs/35274585080). Local pinned Rust 1.98.1 verification passed formatting, warnings-denied Clippy, **64 runtime tests and 2 doctests**, and architecture checks. Counts are cumulative for this stage.

#### Remaining acceptance

The selected codec/capabilities must be owned by an authenticated production session and used for every inbound/outbound message. Real handshake-to-codec qualification, explicit support for every advertised version, validated durable migration outputs and equal-version import validation are not established by range-selection tests. Do not advertise arbitrary integer version ranges as implemented protocol support.

Original history and the updated predecessor are preserved without force-pushing. No issue was closed and no PR was merged, approved or blanket review findings resolved.

### Discussion (3 comments)

#### Comment 5690461992 — chatgpt-codex-connector[bot] — 2026-09-16T01:05:21Z

Source: https://github.com/Jordan-Hall/browser/pull/790#issuecomment-5690461992 | Updated: 2026-09-16T01:08:20Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T01:08:19.725690Z">2026-09-16T01:08:19.725690Z</relative-time> | `02de1e7` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705819847 — Jordan-Hall — 2026-09-16T23:08:45Z

Source: https://github.com/Jordan-Hall/browser/pull/790#issuecomment-5705819847 | Updated: 2026-09-16T23:08:45Z

<!-- intent-core-review:2026-09-16:pr-790 -->
###### CORE review — CORE-01.T06

Reviewed head `9134df9db63be066b549de30b178c1efb494f841`, negotiation/migration implementations, the actual envelope codec and existing discussion against #126 / #2. Source-level review; no code changed.

###### P1 integration gap — Negotiated versions cannot be used by the actual codec
`negotiate_protocol` can select versions such as 1.1 or 2.0, while `Envelope` constructors still stamp 1.0 and `decode_control` rejects every version other than `SchemaVersion::V1`. `NegotiatedProtocol::validate_message_version` is a separate helper, not the encode/decode path. The helpers can therefore report a compatible session that cannot exchange its negotiated messages.

**Change:** advertise only implemented codec versions, and make the authenticated session own the negotiated version/capabilities used by the codec. Validate the envelope header against that session before version-specific payload decoding. Capabilities negotiated here describe protocol support, not permission to execute tools.

**Acceptance:** real offer → authenticated session → encode → frame → decode tests for every advertised version, downgrade/no-common-version cases and a payload incompatible with an unsupported version. An offer must not advertise a version simply because the integer fits.

###### P2 — Offer limits are enforced after collection/allocation
`ProtocolOffer::try_new` collects an arbitrary iterator into a BTreeSet before checking its size; deserialization allocates raw Vecs first. A very large or infinite duplicated-capability iterator can perform unbounded work even though the final unique set stays small. Use bounded visitors/iteration with separate input-entry and unique-capability limits, and reject empty capability names. Test repeated duplicates as well as many distinct values.

###### P2 — Migration metadata is not validation of migrated records
The migration runner limits step count but not input/output bytes, and it trusts each callback's claimed target version without checking the output against the target record family/schema. An identity or malformed callback can produce a success-labelled `MigrationOutcome` whose bytes are not actually that version. Add bounded IO, target-family validation and explicit failure diagnostics; register real record migrations separately from fixture transformations.

`ReadOnlyNewerMinor` is also only a classification helper: the durable record deserializers still reject every non-1.0 schema. Either provide an opaque, extension-preserving read-only document path or state that this case is unsupported; never deserialize-and-drop unknown fields then write it back.

**Required evidence:** complete per-family migration fixtures, actual codec negotiation tests, lossless preservation of protected fields, and no automatic overwrite of newer documents. Existing no-finding bot summary is not evidence of these integration properties.

#### Comment 5706433095 — Jordan-Hall — 2026-09-17T00:13:12Z

Source: https://github.com/Jordan-Hall/browser/pull/790#issuecomment-5706433095 | Updated: 2026-09-17T00:13:12Z

###### Review follow-up — negotiate an implemented session, then validate real migrated bytes

Rechecked `9134df9db63be066b549de30b178c1efb494f841` against #126 / #2 and the existing review. The range-selection algorithm is useful, but negotiation, authentication and the V1-only envelope codec are still independent APIs.

Add an integration fixture where both peers offer 1.1: negotiation currently selects 1.1, yet `Envelope::request` constructs 1.0 and the decoder rejects non-1.0. Either advertise only implemented versions or route through a codec registry owned by the authenticated session. Do not allow arbitrary callers to overwrite a version field to bypass this contract.

**Additional migration acceptance detail:** the equal-version fast path returns `source_bytes.to_vec()` without any family validation. A caller supplying invalid bytes and equal source/target versions therefore gets a success-labelled `MigrationOutcome`. Decide explicitly whether this API returns *unvalidated transport bytes* or a *validated migrated document*. For the latter, validate both the no-op and transformed paths, bind the family/version and preserve the original bytes if validation fails.

Bound total migration input/output and diagnostics, not just callback count. Treat migration callbacks as trusted compiled code; step limits alone do not bound their CPU use. Test malformed/no-op output, changed protected fields, output expansion and a newer-minor document that must remain read-only without losing unknown fields.

These are implementation recommendations and source-derived integration cases; no fix or passing integration test is claimed.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-01-t05-worker-auth",
  "base_sha": "56085134c7daeb6569ee7ccfc71906ca01fb8250",
  "head_branch": "core-01-t06-version-negotiation",
  "head_sha": "ae25e5123cae06c9d9ea6c1775b8c46373adb96a",
  "merge_commit_sha": "aa429f00eb779905b3f7a798926446ca7345b514"
}
```

<a id="issue-791"></a>
## #791 — CORE-01.T07: propagate cancellation, deadlines and bounded backpressure

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/791
**Created:** 2026-09-16T08:43:38Z | **Updated:** 2026-09-17T21:07:58Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation primitives for **#127 (`CORE-01.T07`)**, parent #2, CORE epic #13 and programme #1. All task references and integrated acceptance: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**Queue/cancellation hardening is committed here. A qualified real-worker cancellation path is not yet implemented.**

#### Current implementation

Reserved control queues, reliable control handling, bounded best-effort progress, credit accounting and an in-process StreamEndpoint cancellation model are present. QueueLimits fields are now private, so external code cannot construct or mutate an invalid value around its validating constructor. Executed compile-fail doctests cover struct-literal bypass and post-construction mutation. Read-only access does not reopen mutation.

The branch includes the updated negotiated-codec constraints and all preceding contract/wire hardening.

#### Stack and evidence

Base: #790 / `core-01-t06-version-negotiation`. Next: #792. Head: `9cc67277c7e4aa4b21e9ede4bd59f099543d27d2`.

[Passing CI run 35274593054](https://github.com/Jordan-Hall/browser/actions/runs/35274593054). Local Rust 1.98.1 verification passed formatting, warnings-denied Clippy, **73 runtime tests and 4 doctests**, and architecture checks. Counts are cumulative at this stage.

#### Blocking acceptance work

The two endpoints are an in-process model, not two spawned workers. Real transport cancellation under saturation, byte budgets, deadline/epoch checks at dispatch and progress/artifact accounting still need integration.

Cancellation entries need a bounded generation-aware retirement protocol with #137 and #141. Blindly deleting entries while late work can still execute would reactivate revoked authority; retirement must follow fencing and rejection/drain of late output. The current lifetime-registry exhaustion finding remains open.

The update preserves original and predecessor history without force-pushing. No issue closure, PR merge/approval or blanket review resolution was performed.

### Discussion (4 comments)

#### Comment 5694642907 — chatgpt-codex-connector[bot] — 2026-09-16T08:43:48Z

Source: https://github.com/Jordan-Hall/browser/pull/791#issuecomment-5694642907 | Updated: 2026-09-16T08:46:44Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T08:46:43.399009Z">2026-09-16T08:46:43.399009Z</relative-time> | `f13d231` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705822357 — Jordan-Hall — 2026-09-16T23:09:04Z

Source: https://github.com/Jordan-Hall/browser/pull/791#issuecomment-5705822357 | Updated: 2026-09-16T23:09:04Z

<!-- intent-core-review:2026-09-16:pr-791 -->
###### CORE review — CORE-01.T07

Reviewed head `d1371e2064956cf2b4e3441281119701d3f31861`, flow/cancellation/stream code and existing discussion against #127 / #2. No implementation changes made.

###### Existing findings are still present
- **P1 / `discussion_r4024153005`:** public `QueueLimits` fields allow bypassing `try_new`, including reserved capacity 0 and oversized limits, through infallible `PriorityQueue::new`. Make invalid construction impossible or validate at every queue entry point.
- **P1 / `discussion_r4024153018`:** `StreamEndpoint` exposes registration but no safe retirement path for its private cancellation registry. A long-lived endpoint stops accepting new cancellable work after its lifetime registration limit. Add completion-aware retirement without making stale IDs active again.
- **P2 / `discussion_r4024153024`:** `progress_backlog` includes artifact-reference entries from the shared queue. Separate counters or rename the reported metric.

###### P1 qualification gap — This is not yet cancellation across worker boundaries
`two_workers_ack_cancel_while_progress_queues_are_saturated` creates two in-memory values. It spawns no workers, sends no bytes, and exercises no socket backpressure or process exit. The fixture worker itself remains a no-op. Keep this useful unit test, but do not cite it as the issue's required two-worker-boundary test.

`enqueue_progress`, `enqueue_reliable` and `pop_next` do not bind work to an epoch/deadline or reject a cancelled task. The deadline helper is not invoked by this path. Document this queue as a primitive and integrate a broker-owned dispatch guard that revalidates the task/lease at dispatch time. Local acknowledgement must remain distinct from remote termination and rollback.

###### P2 — Count bounds are not byte bounds or guaranteed stop delivery
Generic `T` can contain arbitrarily large allocations. Track admitted bytes as well as item counts. Repeated cancel/ack traffic can fill the reserved queue; define bounded deduplication/coalescing and an observable retry policy instead of assuming the reserved lane cannot saturate. `is_cancelled` returns false for unknown/removed IDs: authorization must require a positively active current-epoch record, not just `!is_cancelled`.

**Tests before sign-off:** actual two-process cancellation under saturated bulk IO; expired deadlines with queued work; revocation racing a dispatch; duplicate cancel storms; more completed requests than registry capacity; malicious limit literals; byte-budget saturation; fresh epochs after restart; an already-dispatched external write stays uncertain rather than becoming cancelled-success.

Recommended improvements should preserve the existing ownership-returning enqueue errors and the separation of stop capacity from progress traffic.

#### Comment 5706205376 — Jordan-Hall — 2026-09-16T23:44:33Z

Source: https://github.com/Jordan-Hall/browser/pull/791#issuecomment-5706205376 | Updated: 2026-09-16T23:44:33Z

<!-- intent-core-review:2026-09-17:pr-791-cancel-result -->
###### Additional review: represent cancellation applied versus acknowledgement queued

Owning task: #127; integration: #141/#212. Source checked at the current stacked tree `fbfcfceb17540e5fe80bec555385e097f3787455`; `stream.rs` is inherited from this PR's head.

**P2 integration/API concern:** `StreamEndpoint::accept_cancel` first mutates `CancellationRegistry` and then enqueues `CancelAck`. If the reserved queue is already full, it returns an enqueue error even though cancellation has already been applied. Keeping the cancellation applied is the safe behavior; the problem is that `Result<(), StreamError<_>>` does not directly distinguish 'cancellation rejected' from 'cancelled, acknowledgement pending'. A caller must not tell the user cancellation failed or restore authority merely because the acknowledgement could not be queued.

Recommended improvement: document the mutation boundary and return an explicit outcome, or retain a bounded/coalesced pending-ack record that can be retried after capacity returns. Do not roll back the cancellation to make the method all-or-nothing. Preserve idempotence and combine this with the existing retirement/epoch recommendations rather than reopening a cancelled ID.

**Concrete regression:** configure one reserved slot, occupy it, register a cancellation ID, then accept cancellation. Assert the record is cancelled despite acknowledgement backpressure; drain/retry and assert one meaningful acknowledgement, no renewed authority and no unbounded ack accumulation. Also distinguish UnknownId from this partial-success outcome in task-centre diagnostics.

This is a source-derived test/review recommendation, not a claim that a Rust regression was run or that the existing findings were fixed. The earlier detailed review remains applicable.

#### Comment 5706445404 — Jordan-Hall — 2026-09-17T00:14:42Z

Source: https://github.com/Jordan-Hall/browser/pull/791#issuecomment-5706445404 | Updated: 2026-09-17T00:14:42Z

###### Review follow-up — cancellation retirement must not reopen stale work

Rechecked `d1371e2064956cf2b4e3441281119701d3f31861` (`flow.rs`, `cancellation.rs`, `stream.rs`), the existing findings and the later acknowledgement-outcome note. Owning task #127; supervisor integration #141.

When adding the missing retirement API, **do not simply expose `remove` and then use `!is_cancelled(id)` as permission**. Removed/unknown IDs currently return false from `is_cancelled`, and registering a removed ID makes it Active again. Require a positively active record bound to the current worker/task generation. Retirement must invalidate buffered work and must not require an unbounded lifetime tombstone map.

A useful combined regression sequence is: fill the progress and reserved queues; apply cancellation; observe acknowledgement enqueue failure while cancellation remains applied; drain/retry acknowledgement; complete/retire; submit late work with the old identity; start a new generation. Late work must be rejected, new work must remain admissible, and control memory must remain bounded after more completed tasks than registry capacity.

Keep the existing ownership-returning enqueue errors. Separate reliable result/artifact publication from disposable progress so a required artifact reference is not lost merely because progress credits run out; document whether the caller persists/retries it.

The current two-worker test uses two in-memory endpoints. Retain it as a unit test, then add actual subprocess/socket saturation tests with deadline and epoch checks at dispatch. No code changes or fixes are claimed.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-01-t06-version-negotiation",
  "base_sha": "ae25e5123cae06c9d9ea6c1775b8c46373adb96a",
  "head_branch": "core-01-t07-cancel-backpressure",
  "head_sha": "9cc67277c7e4aa4b21e9ede4bd59f099543d27d2",
  "merge_commit_sha": "75458d3f87ad0ebe8fdb6262dcf287a414fbeb4d"
}
```

<a id="issue-792"></a>
## #792 — CORE-01.T08: publish contract conformance and fuzz targets

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/792
**Created:** 2026-09-16T08:52:20Z | **Updated:** 2026-09-17T21:08:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation for **#128 (`CORE-01.T08`)**, parent #2, CORE epic #13 and programme #1. Individual feature/baseline/integration coverage: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**This is a contract smoke/conformance and build foundation, not full runtime or fuzz qualification.**

#### Current implementation and hardening

- Conformance executable and golden fixtures exercise core contract/codec smoke cases.
- Fuzz targets cover frame decoding and control envelopes; serde_json is now a declared direct dependency of the excluded fuzz package.
- Both workspace and fuzz Cargo.lock files are committed and checked unchanged; verification commands use --locked.
- CI explicitly executes doctests and compiles the excluded fuzz targets instead of assuming workspace tests cover them.
- Independent checks continue after another gate fails. Verification evidence retains actual failed/skipped outcomes, tested source, platform/commit identity and exact lockfiles, with upload on failure as well as success.
- All preceding contract, parser, offer-bound and queue-limit amendments are present on this branch.

#### Stack and evidence

Base: #791 / `core-01-t07-cancel-backpressure`. Next: #793. Head: `5c26d585a63c0967f2bc24f990dbc3ec56b74360`.

[Passing CI run 35274600376](https://github.com/Jordan-Hall/browser/actions/runs/35274600376). Local Rust 1.98.1 verification also passed formatting, warnings-denied Clippy, **74 runtime tests and 4 doctests**, actual architecture checks, the separately locked fuzz compile check and smoke conformance. Counts are cumulative at this stage.

#### Remaining acceptance

Compiling libFuzzer targets is not running an instrumented fuzz campaign. Pinned instrumented execution with resource/time budgets, minimized failing inputs, complete record-family fixtures and real-process authentication/cancellation conformance remain required. A successful smoke report does not establish production readiness; the outer verification report explicitly preserves that distinction.

The branch and predecessor history are preserved without force-pushing. No merge, approval, issue closure or blanket finding resolution was performed.

### Discussion (3 comments)

#### Comment 5694746429 — chatgpt-codex-connector[bot] — 2026-09-16T08:52:33Z

Source: https://github.com/Jordan-Hall/browser/pull/792#issuecomment-5694746429 | Updated: 2026-09-16T08:55:30Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T08:55:29.711931Z">2026-09-16T08:55:29.711931Z</relative-time> | `70f2ad8` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705825053 — Jordan-Hall — 2026-09-16T23:09:21Z

Source: https://github.com/Jordan-Hall/browser/pull/792#issuecomment-5705825053 | Updated: 2026-09-16T23:09:21Z

<!-- intent-core-review:2026-09-16:pr-792 -->
###### CORE review — CORE-01.T08

Reviewed head `f04d5d5b799823c3d5ad28d8f1412b657d1b0269`, the conformance executable, excluded fuzz package, documentation and existing discussion against #128 / #2. No implementation changes; Rust/fuzz binaries were not executed locally.

###### P2 — Published fuzz target has an undeclared dependency
The existing finding `discussion_r4024222867` remains present: `fuzz_targets/control_envelope.rs` imports `serde_json::Value`, but `fuzz/Cargo.toml` does not declare `serde_json`. A transitive dependency does not supply that crate to this target. The package is excluded from workspace CI, so green workspace checks cannot catch this build failure.

**Change:** declare the dependency, pin a supported fuzz toolchain/tool version, and add a separate fuzz-build/smoke CI job. Keep libFuzzer excluded from production dependencies; exclusion from the production workspace should not mean exclusion from build validation.

###### P1 readiness gap — The report covers smoke checks, not the stated CORE contract gate
The executable checks one protocol-offer string, a synthetic byte-appending migration, synthetic role authentication, one role-family predicate and an approval missing nearly all fields. It does not validate every durable record family, actual spawned-process authentication, negotiated end-to-end codecs, approval hash binding, cancellation/deadlines across processes or storage/recovery. It currently remains green with the concrete defects recorded on #787–#791.

Do not remove the smoke suite; label its coverage accurately and extend it with the real invariants. In particular, malformed-approval coverage must include a structurally complete proposal/approval pair with mismatched bytes/account, not only a truncated JSON object.

###### Evidence and failure-reporting improvements
The report is emitted only after all checks succeed; failure returns early and CI then skips the upload step. Emit a structured failed report with the failing check and bounded diagnostics, keep a non-zero exit status, and upload available evidence on failure. Include tested commit, target OS/architecture, dependency-lock digest, fixture corpus digest and check IDs. The compiler string alone does not identify the tested artifact.

**Acceptance:** intentional regressions in each claimed invariant fail their corresponding gate; both fuzz targets compile; short seeded fuzz runs execute with explicit resource budgets; minimized findings become deterministic regressions; doctests run explicitly; recorded coverage differentiates static/unit/process/platform evidence. Do not treat an all-true smoke report as operational approval for CORE.

#### Comment 5706450338 — Jordan-Hall — 2026-09-17T00:15:19Z

Source: https://github.com/Jordan-Hall/browser/pull/792#issuecomment-5706450338 | Updated: 2026-09-17T00:15:19Z

###### Review follow-up — make coverage and failure evidence machine-checkable

Rechecked `f04d5d5b799823c3d5ad28d8f1412b657d1b0269` against #128 / #2. The existing undeclared `serde_json` fuzz dependency and coverage/failure-report findings remain applicable.

Add a versioned **requirement-to-test manifest** rather than using the conformance executable's all-true result as the CORE completion signal. Each entry should identify task ID, invariant, test/fixture, supported platform, evidence kind (unit/process/platform), and result (`passed`, `failed`, `not_run`, `unsupported`). A missing platform or skipped test must not become a pass through an empty iterator or absent row.

Use mutation tests for the validator itself: deliberately permit a mismatched proposal hash, bypass queue-limit validation, omit one expected case, and force a conformance check to fail. Each mutation should fail the appropriate release gate and still retain a structured report. Keep a non-zero process exit on failure.

The current report returns only after every check succeeds and the upload step has no failure condition. Preserve failure artifacts with bounded, redacted diagnostics and include the tested head/merge SHA, lockfile digest and fixture digest. Do not regenerate an expected golden fixture during the same test that checks it.

Build the excluded fuzz package in its own job and use seeded bounded smoke runs; fuzz-build coverage and runtime fuzzing duration are separate evidence. No Rust/fuzz execution or code fix was performed by this review.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-01-t07-cancel-backpressure",
  "base_sha": "9cc67277c7e4aa4b21e9ede4bd59f099543d27d2",
  "head_branch": "core-01-t08-conformance",
  "head_sha": "5c26d585a63c0967f2bc24f990dbc3ec56b74360",
  "merge_commit_sha": "d99f8e004bef6f22d1993dc90069cff0a9bf573a"
}
```

<a id="issue-793"></a>
## #793 — CORE-02.T01: bootstrap single-writer durable SQLite state

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/793
**Created:** 2026-09-16T08:56:43Z | **Updated:** 2026-09-17T21:09:10Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation for **#129 (`CORE-02.T01`)**, parent #3, CORE epic #13 and programme #1. Complete individual task coverage and integration: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**Database initialization and migration hardening are committed here; profile ownership and production storage qualification remain open.**

#### Current implementation

- A private SQLite connection and explicit WAL/foreign-key/busy-timeout policy, application identity and checksum-backed migration ledger.
- A populated zero-application-ID database is rejected before adoption or journal-mode changes. Initialization validation and adoption share a writer transaction.
- Migration selection, ledger validation and the pending migration batch run under one immediate transaction rather than selecting against a stale pre-lock view.
- user_version must agree with the validated contiguous migration ledger; unsupported, reordered or modified applied migrations fail closed.
- Stage-specific tests reject foreign files and ledger/version corruption without importing future storage schemas prematurely.

#### Stack and evidence

Base: #792 / `core-01-t08-conformance`. Next: #794. Head: `381ea7903f7c055b7939ecbc92e3f14669e05f03`.

[Passing CI run 35274613284](https://github.com/Jordan-Hall/browser/actions/runs/35274613284). Local Rust 1.98.1 checks passed formatting, warnings-denied Clippy, **82 runtime tests and 4 doctests**, architecture checks, separately locked fuzz compilation and smoke conformance. Counts are cumulative. Migration-batch rollback coverage that requires migration 002 is introduced at the next storage stage rather than importing it here.

#### Remaining acceptance

A writer transaction serializes database mutations; it is not lifetime ownership of the profile or protection against a second database sharing an artifact root. Profile-wide owner enforcement, filesystem trust boundaries, authenticated restore validation and platform/process/power-loss qualification remain required. The existing version-001 migration SQL is unchanged; this PR does not silently rewrite already-applied schema history.

Original and updated predecessor histories are preserved without force-pushing. No issue closure, PR merge/approval or blanket review resolution was performed.

### Discussion (3 comments)

#### Comment 5694800333 — chatgpt-codex-connector[bot] — 2026-09-16T08:56:59Z

Source: https://github.com/Jordan-Hall/browser/pull/793#issuecomment-5694800333 | Updated: 2026-09-16T09:00:14Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T09:00:14.001263Z">2026-09-16T09:00:14.001263Z</relative-time> | `ad555f8` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705837422 — Jordan-Hall — 2026-09-16T23:10:50Z

Source: https://github.com/Jordan-Hall/browser/pull/793#issuecomment-5705837422 | Updated: 2026-09-16T23:10:50Z

<!-- intent-core-review:2026-09-16:pr-793 -->
###### CORE review — CORE-02.T01

Reviewed head `c468a457da543806ae2558a081d98d8ee5a2f9cd`, store bootstrap/migrations and existing discussion against #129 / #3. Review only.

###### Existing findings revalidated
- **P1 / `discussion_r4024260259`:** `application_id == 0` unconditionally assigns Intent's ID. An unrelated populated SQLite database can therefore be modified on open. Inspect identity/schema before persistent configuration changes; only explicitly new/empty stores should be adopted. Reject a populated zero-ID database without changing its data, application ID or journal mode.
- **P2 / `discussion_r4024260266`:** migration selection reads the highest version before taking the writer transaction. Two openers can select the same migration, and the second runs stale SQL after the first commits. Acquire ownership/transaction before deciding what is pending, or recheck under lock.
- **P2 / `discussion_r4024260274`:** ledger validation never compares `PRAGMA user_version` with the validated ledger maximum. A corrupted/future user_version can be accepted and reported as current.

###### P1 integration prerequisite — A private connection field is not profile-wide single ownership
`StateStore::open` can be called multiple times for the same path, including from another process. No profile-wide exclusive owner guard or canonical-path identity is established. SQLite serializes individual transactions, but it does not serialize the filesystem work between transactions in artifact ingestion/GC (#797/#801). The latter needs a real single-owner invariant or an explicit cross-writer coordination protocol.

**Change:** bind store ownership to the provisioned profile, hold an OS-backed owner guard for its lifetime, route writes through a bounded service, and define read-only secondary access. Handle aliases, crash release and attempted duplicate startup explicitly. Do not distribute StateStore to untrusted workers.

###### Integrity/durability evidence
`quick_check` is useful but is not full reference, journal/projection, artifact-hash or foreign-key qualification. Add a separate `foreign_key_check` and domain invariant scans; distinguish a fast startup check from a full integrity/backup audit. Verify effective PRAGMAs and tested local filesystems; do not generalize WAL support to network shares or imply bundled SQLite alone makes dependency resolution reproducible.

**Tests:** unrelated zero-ID DB unchanged on refusal; two concurrent initializers; altered user_version in both directions; interrupted migration rollback; duplicate profile opens; foreign-key corruption detection; process crash releasing ownership. Run blocking SQLite/5-second contention waits off the stop/UI control path. No fixes or completion status are claimed here.

#### Comment 5706493426 — Jordan-Hall — 2026-09-17T00:20:49Z

Source: https://github.com/Jordan-Hall/browser/pull/793#issuecomment-5706493426 | Updated: 2026-09-17T00:20:49Z

###### Review follow-up — distinguish creating a store from opening an existing one

Rechecked `c468a457da543806ae2558a081d98d8ee5a2f9cd` and the inherited migration path against #129 / #3. The existing foreign-database adoption, migration-selection race and `user_version` mismatch findings remain open.

**Additional integrity case:** `initialize_store_metadata` runs `INSERT OR IGNORE` with a freshly generated UUID on every open. If the metadata row is missing from an otherwise existing store, opening silently supplies a new store identity instead of distinguishing damage from initial creation. Existing malformed IDs are not validated by `open`; validation happens only when `store_id()` is called.

Separate new-store bootstrap from existing-store validation. An existing store should retain exactly one valid identity, matching profile/root binding and the validated migration ledger; missing/malformed identity should enter explicit recovery, not automatic re-identification. Perform format/identity checks before changing an unrelated file's application ID or journal mode.

**Acceptance additions:** existing database with a deleted metadata row; malformed UUID; populated zero-application-ID foreign database; two actual processes opening the same profile; old schema migration; future/inconsistent ledger version. Rejected opens should leave the inspected database's logical contents and identity unchanged.

A private `Connection` field provides Rust encapsulation, not a profile-wide single-owner guarantee. Define that ownership boundary explicitly before artifact ingestion and GC can run through separate store instances (#797/#801). Preserve WAL/FULL on supported local filesystems and keep busy timeout distinct from ownership and crash recovery.

Source review only; no code or database in the repository was modified.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-01-t08-conformance",
  "base_sha": "5c26d585a63c0967f2bc24f990dbc3ec56b74360",
  "head_branch": "core-02-t01-state-db",
  "head_sha": "381ea7903f7c055b7939ecbc92e3f14669e05f03",
  "merge_commit_sha": "79f4a387b979bbad3a1655d07d841a5645bff040"
}
```

<a id="issue-794"></a>
## #794 — CORE-02.T02: model durable operation and journal records

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/794
**Created:** 2026-09-16T09:09:01Z | **Updated:** 2026-09-17T21:09:30Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation for **#130 (`CORE-02.T02`)**, parent #3, CORE epic #13 and programme #1. Full task/integration ledger: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**Operation/journal hardening is now owned by this branch; evidence-bound reconciliation and compensation lineage remain open.**

#### Current implementation

- Durable operation projections and append-only ordered journal updates share transactional state changes, expected-state checks and bounded reason fields.
- Continuing outcome transitions must retain the active attempt identity. A result for attempt B cannot rewrite the history of attempt A.
- Starting dispatch or compensation requires a distinct attempt identity; rejected changes leave both projection and journal unchanged.
- The branch incorporates the #793 initialization/version fixes and now exercises pending-migration-batch rollback where migration 002 exists.

#### Stack and evidence

Base: #793 / `core-02-t01-state-db`. Next: #795. Head: `cb3856645ba460e1d3102ac4adc58199897ffd15`.

[Passing CI run 35274623965](https://github.com/Jordan-Hall/browser/actions/runs/35274623965). Local Rust 1.98.1 verification also passed formatting, warnings-denied Clippy, **87 runtime tests and 4 doctests**, architecture checks, locked fuzz compilation and smoke conformance. Counts are cumulative at this stage.

#### Remaining acceptance

Attempt continuity does not establish global uniqueness, approval/source-precondition authorization or evidence-bound verification. Original-action and compensation attempt lineage must be stored separately, including reconciliation origin. An unconditional NeedsReconciliation -> Compensated transition would misclassify uncertainty about the original action and must not be used as a shortcut.

The operation and outbox APIs need one accepted transition contract, with exact-attempt reconciliation evidence and immutable verified receipt history. These remain blockers, not completed features hidden behind passing unit tests.

The update preserves original/predecessor history without force-pushing. No issue closure, PR merge/approval or blanket finding resolution was performed.

### Discussion (3 comments)

#### Comment 5694947411 — chatgpt-codex-connector[bot] — 2026-09-16T09:09:13Z

Source: https://github.com/Jordan-Hall/browser/pull/794#issuecomment-5694947411 | Updated: 2026-09-16T09:13:33Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T09:13:32.177129Z">2026-09-16T09:13:32.177129Z</relative-time> | `9bf9888` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705840104 — Jordan-Hall — 2026-09-16T23:11:09Z

Source: https://github.com/Jordan-Hall/browser/pull/794#issuecomment-5705840104 | Updated: 2026-09-16T23:11:09Z

<!-- intent-core-review:2026-09-16:pr-794 -->
###### CORE review — CORE-02.T02

Reviewed head `a01286c2acf45ec7a905eb3ec8de6ee5f3ea555f`, operation transitions/journal persistence and existing discussion against #130 / #3. No code changes.

###### Existing P1 findings remain
`discussion_r4024369593`: `validate_attempt_identity` checks presence, not continuity. `Attempting(A) -> Accepted(B)` is allowed and overwrites the recorded attempt even though B was never dispatched. Require the exact active attempt identity for acceptance, verification and reconciliation; model a new compensation attempt separately.

`discussion_r4024369582`: `Compensating -> NeedsReconciliation` loses the compensation origin and has no valid success path back to `Compensated`. Preserve original versus compensating intent and attempt lineage; do not add an unrestricted transition that lets any uncertain original action become compensated.

**Acceptance:** an exhaustive state/attempt transition matrix, tests rejecting identity substitution, and compensation timeout → read-only reconciliation → compensated with retained original receipt/history.

###### P2 — A schema version is not the source precondition revision
The record persists `source_schema_major/minor`, but the parent proposal calls for the immutable source revision/preconditions on which the action was authorized. An API schema version does not identify a quote/cart/document revision. Add explicitly typed source-precondition references and the evidence/approval binding, without overloading schema version. Verify stale source revisions cannot reuse an approved action.

###### Cross-PR consistency needs one transition owner
Public general transitions can enter dispatch states independently of #795's outbox changes; cancellation can leave a dispatchable-looking outbox behind. Consolidate transition/journal/outbox mutations behind task-specific transactional methods and define a lossless projection to `intent_contracts::OperationState` (#787). `Verified` needs independently recorded verification evidence; a requested enum value alone must not authorize success.

###### Additional improvements
Provide an explicit pre-dispatch abandonment path (Prepared currently cannot transition to Cancelled), checked timestamp ordering, paginated journal reads and a journal/projection consistency verifier. If the journal is called append-only, enforce that property in the trusted repository surface and test deletion/alteration detection. Checksums alone would not authenticate against an attacker owning the database; state that boundary precisely.

Preserve the good atomic snapshot+journal transaction and expected-revision checks. Add two-owner/process race tests, not just two sequential calls with a stale revision. Keep the task open for implementation review until the state-machine and evidence invariants are covered.

#### Comment 5706500421 — Jordan-Hall — 2026-09-17T00:21:43Z

Source: https://github.com/Jordan-Hall/browser/pull/794#issuecomment-5706500421 | Updated: 2026-09-17T00:21:43Z

###### Review follow-up — preserve the outcome of the transaction that just committed

Rechecked `a01286c2acf45ec7a905eb3ec8de6ee5f3ea555f` against #130 / #3 and the existing review. Attempt continuity, compensation-origin recovery and pre-dispatch abandonment still need the documented fixes.

**Additional P2 outcome-contract concern:** `create_operation` and `transition_operation` commit their transaction and then call `load_operation` outside it. If that read fails, the caller receives a normal error although the mutation/journal already committed. With another permitted store instance, the read can instead return a later writer's state rather than the exact revision this call produced. This matters to durable retry logic and must not be interpreted as permission to repeat a side effect.

Prefer constructing/validating the return snapshot while holding the transaction, committing, and returning that snapshot. Alternatively represent committed-but-readback-unavailable explicitly and support outcome lookup by immutable operation/revision identity. Handle ambiguous storage commit errors separately from both clean rollback and known commit.

**Regressions:** injected failure between commit and readback; readback interleaved with a second writer; retry of an identical command; stale expected revision; journal insertion failure before commit. Each case must state whether a revision exists and whether retry is safe, with no duplicate journal event.

Keep the existing atomic snapshot+journal transaction. Extend the same outcome discipline to outbox staging (#795), suppression/hold changes (#801), and recovery (#145–#148), rather than implementing independent retry interpretations.

No local Rust test run or code fix is claimed.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-02-t01-state-db",
  "base_sha": "381ea7903f7c055b7939ecbc92e3f14669e05f03",
  "head_branch": "core-02-t02-operation-journal",
  "head_sha": "cb3856645ba460e1d3102ac4adc58199897ffd15",
  "merge_commit_sha": "b605ec63a5bd05cf8d2140bd6eae00496828dc6c"
}
```

<a id="issue-795"></a>
## #795 — CORE-02.T03: implement transactional outbox dispatch

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/795
**Created:** 2026-09-16T09:23:35Z | **Updated:** 2026-09-17T21:09:56Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation for **#131 (`CORE-02.T03`)**, parent #3, CORE epic #13 and programme #1. Complete task/integration ledger: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**Outbox hardening is committed on this branch; trusted dispatch authorization and restart reconciliation remain unimplemented gates.**

#### Current implementation

- Transactional operation/outbox staging, bounded payloads, leases, attempt-before-send state transitions and recorded dispatch outcomes.
- Attempt identity is consistent across staged operation projection, journal, outbox message, dispatch start and result; mismatched identities fail without rewriting the active attempt.
- Claim selection joins the current operation state and excludes cancelled/non-dispatchable work before LIMIT, preventing an older unusable row from starving live work.
- Dispatch-time state checks are retained for cancellation that occurs after claim. Cancellation-after-claim regression coverage ensures the durable attempt is not started.
- Includes the updated operation, migration and contract stack.

#### Stack and evidence

Base: #794 / `core-02-t02-operation-journal`. Next: #796. Head: `9225ab3ec4c64202b4bcc1b013a6bea61977aa13`.

[Passing CI run 35274632851](https://github.com/Jordan-Hall/browser/actions/runs/35274632851). Local Rust 1.98.1 verification passed formatting, warnings-denied Clippy, **93 runtime tests and 4 doctests**, architecture checks, locked fuzz compilation and smoke conformance. Counts are cumulative at this stage.

#### Blocking acceptance work

Payload/destination access is not yet confined to an opaque, generation-fenced dispatch capability. Approval, exact argument bytes, account/capability/target and source-precondition binding must be enforced by the trusted dispatcher, not assumed from hashes or leases.

A crashed worker's attempting message may already have caused an external effect. Abandoned attempts need durable owner/epoch discovery and read-only reconciliation with an independent external-effect ledger; they must not simply be reset to pending and resent. Original/compensation attempt lineage and complete recovery acceptance remain open with #145, #147 and #148.

Original/predecessor history is preserved without force-pushing. No issue closure, PR merge/approval or blanket review resolution was performed.

### Discussion (3 comments)

#### Comment 5695157964 — chatgpt-codex-connector[bot] — 2026-09-16T09:23:45Z

Source: https://github.com/Jordan-Hall/browser/pull/795#issuecomment-5695157964 | Updated: 2026-09-16T09:27:03Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T09:27:01.266280Z">2026-09-16T09:27:01.266280Z</relative-time> | `6206d15` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705844395 — Jordan-Hall — 2026-09-16T23:11:35Z

Source: https://github.com/Jordan-Hall/browser/pull/795#issuecomment-5705844395 | Updated: 2026-09-16T23:11:35Z

<!-- intent-core-review:2026-09-16:pr-795 -->
###### CORE review — CORE-02.T03

Reviewed head `3c7c848014e9ce9c3b234be72ef1d19ea597d15e`, staging/claim/start/result paths and prior review against #131 / #3. Source review only; no external dispatch or code change.

###### Existing findings revalidated
- **P1 / `discussion_r4024477365`:** claims return `OutboxMessage` with public destination/kind/payload access before `begin_dispatch`. A caller can send and crash without a durable attempt; the expired lease can then be reclaimed. Separate inspection/opaque claim handles from dispatch material, and make the actual network dispatcher require a broker-issued started-attempt token.
- **P1 / `discussion_r4024477372`:** starting dispatch clears lease metadata and makes the row ineligible for claim. A crash leaves `attempting` rows without a public recovery enumeration/transition path. Preserve worker epoch/start/recovery metadata and discover them at startup; classify as uncertain, not safe to resend.
- **P2 / `discussion_r4024477384`:** cancellation of a DispatchPending operation does not retire its outbox, so old cancelled entries can repeatedly consume the claim batch. Terminalize them atomically or filter/retire them during claiming.

###### P1 — Persisted payload integrity is not approval binding
`stage_outbox` reads only the operation's state and revision. It hashes the new payload but does not compare it with the operation's approved arguments or validate destination/kind against the authorized capability/account. Both bytes and destination are independently caller-supplied.

**Change:** stage from an immutable validated action/dispatch descriptor, not unrelated fields. If transport bytes necessarily differ from canonical arguments, record and verify a versioned trusted transformation binding; do not simply equate unrelated hashes. Revalidate expiry/revocation/source preconditions at the actual dispatch boundary.

###### P2 — Lease/result fencing and atomic state invariants
A string lease owner is not an epoch/generation. Reuse of the same owner after lease expiry can make an old claim appear current. Use a distinct claim generation/worker epoch and require it when starting and reporting an attempt. Result recording currently takes only the outbox ID, not the active attempt token.

Staging also records an attempt in the journal while leaving `durable_operations.attempt_identity` unset until begin_dispatch. Define whether this is a planned or active attempt explicitly so snapshot/journal reconstruction agrees. Keep uncertain outcomes distinguishable from terminal rejection rather than relying on the overloaded outbox `failed` state.

**Required tests:** wrong bytes/destination/account rejected; lease A expires and lease B fences A; cancel-with-limit-1 does not starve work; process death before/after send; stale result from replaced worker; recover all unfinished attempts; aggregate claim byte budget; exactly one fixture side effect across crash/restart. Local transactions cannot make a merchant request exactly-once without provider cooperation.

#### Comment 5706529974 — Jordan-Hall — 2026-09-17T00:25:09Z

Source: https://github.com/Jordan-Hall/browser/pull/795#issuecomment-5706529974 | Updated: 2026-09-17T00:25:09Z

###### Review follow-up — make result recording retryable without retrying the external action

Rechecked `3c7c848014e9ce9c3b234be72ef1d19ea597d15e` against #131 / #3 and the existing review. The premature payload exposure, abandoned attempts, cancelled-row starvation and approval-binding findings remain open.

**Additional outcome-contract improvement:** `record_dispatch_result` accepts only an outbox ID plus a result enum and rejects every call after the first terminal transition. If the first result transaction committed but its acknowledgement was lost, redelivering the same result returns `InvalidOutboxTransition`. The consumer cannot directly distinguish an identical already-recorded result from a conflicting result or stale worker response.

Persist a bounded result-observation identity tied to the exact attempt/generation, with provider secondary ID/evidence where available. Return an explicit already-recorded outcome for an identical observation, reject conflicting identity reuse, and retain later legitimate reconciliation evidence as a new observation. Do not make a repeated acknowledgement trigger a repeated purchase/message.

**Combined fixture:** stage approved action → claim A → expire/fence A → claim B → begin B → merchant records one effect → persist acceptance → lose local acknowledgement → redeliver acceptance → restart → verify receipt. Assert one external fixture effect, one acceptance transition, retained evidence and no authority for stale A. Run separate before-send/after-send crash variants; unproven outcomes stay `NeedsReconciliation`.

Also carry the post-commit/readback contract from #794 into `stage_outbox` and claims. State clearly which returned object permits dispatch and which is inspection-only. These are review recommendations, not implemented fixes or an assertion of globally exactly-once delivery.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-02-t02-operation-journal",
  "base_sha": "cb3856645ba460e1d3102ac4adc58199897ffd15",
  "head_branch": "core-02-t03-outbox",
  "head_sha": "9225ab3ec4c64202b4bcc1b013a6bea61977aa13",
  "merge_commit_sha": "cef24c5ee40843a71a1d5344c13a328aa9eefe84"
}
```

<a id="issue-796"></a>
## #796 — CORE-02.T04: implement inbox deduplication and consumer cursors

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/796
**Created:** 2026-09-16T09:30:47Z | **Updated:** 2026-09-17T21:10:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation for **#132 (`CORE-02.T04`)**, parent #3, CORE epic #13 and programme #1. All individual task references and integration status: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**Inbox hardening is committed here; source-precondition and recovery/repair acceptance remain open.**

#### Current implementation

- Event inbox, idempotent consumer effects and monotonic cursors commit atomically; duplicate/conflicting deliveries remain explicit.
- Event payload plus all effect payloads share a checked aggregate admission budget before hashing and writer-lock acquisition, rather than permitting the per-effect maximum to multiply without an aggregate cap.
- Stored-effect reads are bounded and taken from one snapshot.
- Duplicate acknowledgement validates existing event bytes and the complete stored effect set. Missing or corrupted evidence is rejected rather than acknowledged as successful processing.
- Preserves the earlier complete-effect-set verification fix and includes all updated predecessors.

#### Stack and evidence

Base: #795 / `core-02-t03-outbox`. Next: #797. Head: `87e0872da7290dd5a057526912dd3a88e4120816`.

[Passing CI run 35274638664](https://github.com/Jordan-Hall/browser/actions/runs/35274638664). Local Rust 1.98.1 verification passed formatting, warnings-denied Clippy, **100 runtime tests and 4 doctests**, architecture checks, locked fuzz compilation and smoke conformance. Counts are cumulative.

#### Compatibility and remaining acceptance

The aggregate budget is deliberately tighter than the old independently multiplied limits. Imported/restored databases need the same bounded validation policy. Failing closed on corrupted duplicate evidence does not itself provide an operator repair workflow.

Delivery order/cursor sequence is not the external source revision needed for write preconditions. Integration with source-precondition validation, recovery classification and accepted repair behavior remains open; passing deduplication tests does not establish exactly-once external effects.

Original/predecessor history is preserved without force-pushing. No issue closure, PR merge/approval or blanket review resolution was performed.

### Discussion (3 comments)

#### Comment 5695247841 — chatgpt-codex-connector[bot] — 2026-09-16T09:31:03Z

Source: https://github.com/Jordan-Hall/browser/pull/796#issuecomment-5695247841 | Updated: 2026-09-16T09:34:07Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T09:34:06.341143Z">2026-09-16T09:34:06.341143Z</relative-time> | `8a425b3` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705870155 — Jordan-Hall — 2026-09-16T23:13:50Z

Source: https://github.com/Jordan-Hall/browser/pull/796#issuecomment-5705870155 | Updated: 2026-09-16T23:13:50Z

<!-- intent-core-review:2026-09-16:pr-796 -->
###### CORE review — CORE-02.T04

Reviewed head `1fc177f601c62acf6fa4fddffa23b902034663b2`, inbox application/read paths and all returned discussion entries against #132 / #3. Review only.

###### Previously fixed finding acknowledged
`discussion_r4024534582` is addressed in the reviewed source: `consumer_effects` loads the stored aggregate, verifies individual rows, recomputes the sorted set hash and reports `EffectSetHashMismatch` for missing materialized rows. The regression and explicit Fixed reply are present. I am not re-reporting that read-path defect as unfixed.

###### P2 — Duplicate acknowledgement does not establish stored-effect integrity
The duplicate branch in `apply_inbox_event` compares the incoming aggregate with the stored `consumer_events.effects_hash` and returns `Duplicate` without checking the persisted effect rows. After losing an effect row, redelivery can still report success even though the separate read path now detects corruption. Make the semantics explicit: either verify the materialization before acknowledging it as already applied, or return a distinct repair-required state. Repair must rebuild only verified local projections and must never replay external actions.

**Test:** apply → delete/change an effect → redeliver the identical event → fail/repair explicitly; verify a legitimate zero-effect event remains distinguishable from absent/corrupt state.

###### P2 — Aggregate effects can consume 1 GiB per call
The limit is 1,024 effects and each payload may be 1 MiB; there is no total byte budget. This permits approximately 1 GiB of effect payloads in one transaction, besides input/allocation overhead. Enforce total bytes before hashing/materialization, use bounded reads, and keep large values as scoped artifact references. Add aggregate-over-limit tests with individually valid effects.

###### Requirement gap — Delivery order and source revision are still conflated/unspecified
There is only `sequence`, enforced as contiguous after accepting an arbitrary first value. The parent explicitly requires newer out-of-order source revisions to remain processable. Define a runtime delivery cursor separately from provider/entity revision and an explicit stream starting watermark. A source revision must not be rejected merely because unrelated deliveries arrived in a different order. Preserve replay identity independently.

###### Scope and determinism improvements
Define `source` as an account/connector-instance-qualified identity rather than an ambiguous free-text label. Bind consumer projection/schema version to deduplication so intentional projector upgrades use a versioned rebuild, not an unexplained determinism error. Keep the existing atomic effect/dedup/cursor commit, and add failure injection between those writes plus cross-account identical-event-ID tests.

No issue state, code, or existing review resolution was changed.

#### Comment 5706535373 — Jordan-Hall — 2026-09-17T00:25:52Z

Source: https://github.com/Jordan-Hall/browser/pull/796#issuecomment-5706535373 | Updated: 2026-09-17T00:25:52Z

###### Review follow-up — make cursor bootstrap, rebuild and acknowledgement distinct operations

Rechecked `1fc177f601c62acf6fa4fddffa23b902034663b2` against #132 / #3. The aggregate-read fix and its regression are present; the existing duplicate-acknowledgement, aggregate-byte and source-revision gaps remain.

A concrete API refinement is to initialize a consumer with an explicit stream generation, starting delivery watermark and projector version, rather than inferring the baseline from whichever event arrives first. Keep ordinary ordered delivery, importing a source snapshot, and rebuilding a new projection version as separate operations. A legitimate resumed stream may start above zero; a missing early delivery must not silently become a successful bootstrap.

**Fixture sequence:** initialize at watermark 40; deliver 42 first (gap/no effects); deliver 41 then 42; redeliver 41; deliver a newer entity revision attached to the next delivery index; rebuild with a new projector version; repeat an event ID from a different account. Assert cursor/effects/dedup are consistent, account namespaces do not collide, and source revision ordering is independent of delivery order.

For the already-recorded branch, specify whether `Duplicate` asserts only delivery identity or also healthy stored materialization. Couple the latter to the aggregate verification already implemented; a repair path may rebuild verified local projections only, never re-execute external actions. Include legitimate zero-effect and missing-effect cases.

Enforce a total effect-byte budget before hashing and inside any import/read path; individually valid 1 MiB payloads must not combine into a gigabyte transaction. Preserve the atomic local commit and do not describe it as exactly-once external execution.

Review only; no implementation or issue-state changes.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-02-t03-outbox",
  "base_sha": "9225ab3ec4c64202b4bcc1b013a6bea61977aa13",
  "head_branch": "core-02-t04-inbox",
  "head_sha": "87e0872da7290dd5a057526912dd3a88e4120816",
  "merge_commit_sha": "7f9d704fdca4a997fe22f137c8cc348ec52e5cf8"
}
```

<a id="issue-797"></a>
## #797 — CORE-02.T05: implement scoped immutable artifact storage

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/797
**Created:** 2026-09-16T10:11:44Z | **Updated:** 2026-09-17T21:10:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation for **#133 (`CORE-02.T05`)**, parent #3, CORE epic #13 and programme #1. Full individual task ledger and integrated retention coverage: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**Artifact hardening is now on its owning branch. Filesystem isolation and platform durability acceptance remain open.**

#### Current implementation

- Privacy-scoped immutable content-addressed blobs, bounded staging, verification, deduplication and durable references.
- Retry input bytes are compared, reference accounting remains privacy-scoped, and directory publication barriers are repeated on retries/deduplication rather than assuming existence proves a prior sync succeeded.
- Final publication and metadata registration share an immediate writer boundary. Input copying/hashing stays outside that critical section; reference validation/registration uses the same database writer protocol.
- New regression fixes false-success pin registration: invalid reference inserts now report CHECK failures. ON CONFLICT handles only the exact duplicate key, instead of broad INSERT OR IGNORE silently discarding invalid pins. A valid repeated pin remains idempotent.
- Unsupported non-Unix directory durability returns an explicit failure rather than false success.

#### Stack and evidence

Base: #796 / `core-02-t04-inbox`. Next: #801. Head: `e2409171ba94979fe007d69db2ceae7934a2405a`.

[Passing CI run 35274654944](https://github.com/Jordan-Hall/browser/actions/runs/35274654944). Local Rust 1.98.1 verification passed formatting, warnings-denied Clippy, **107 runtime tests and 4 doctests**, architecture checks, locked fuzz compilation and smoke conformance. Counts are cumulative. This stage retains migration-005 storage semantics; retention/view integration and deterministic publication/reference/hold-versus-GC races are exercised in #801, not imported prematurely here.

#### Remaining acceptance and tradeoffs

The lifecycle protocol protects cooperating writers sharing the same database. It does not establish exclusive root/profile ownership, defeat hostile ancestor replacement, or protect against another database using the same root. Filesystem barriers can extend writer-lock duration; busy outcomes remain possible. Consistent backup/restore, descriptor-relative trusted path handling and process/power-loss durability qualification remain required. Windows durability is unsupported, and macOS power-loss behavior is not qualified by Linux tests.

Original/predecessor history is preserved without force-pushing. No issue closure, PR merge/approval or blanket review resolution was performed.

### Discussion (3 comments)

#### Comment 5695784418 — chatgpt-codex-connector[bot] — 2026-09-16T10:11:55Z

Source: https://github.com/Jordan-Hall/browser/pull/797#issuecomment-5695784418 | Updated: 2026-09-16T10:15:33Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T10:15:32.509237Z">2026-09-16T10:15:32.509237Z</relative-time> | `ed6e4bf` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705879009 — Jordan-Hall — 2026-09-16T23:14:30Z

Source: https://github.com/Jordan-Hall/browser/pull/797#issuecomment-5705879009 | Updated: 2026-09-16T23:14:30Z

<!-- intent-core-review:2026-09-16:pr-797 -->
###### CORE review — CORE-02.T05

Reviewed head `07936f3eecbba10d608b520427ab13360cf79eee`, artifact ingestion/reads/reference registration/directory helpers and prior discussion against #133 / #3. Review only.

###### Earlier fixes are present, not re-opened as the same defects
The reviewed code consumes and compares retry bytes, requires a permitted scope for reference counts, and synchronizes newly created directory ancestry. These address `discussion_r4024878090`, `discussion_r4024878101` and `discussion_r4024878109`; the Fixed replies and retry/scope regression tests are present. Preserve them.

###### P1 hardening requirement — Bind filesystem authority, not just string paths
Every call accepts a new `root: &Path`; the database does not bind a canonical provisioned root to its profile identity. Directory checking returns immediately for an existing directory, and `symlink_metadata` followed by `File::open` is a check/open race. If another actor can alter a parent/storage path, an ancestor substitution can redirect reads/writes despite the leaf check. This is conditional on a writable/untrusted storage namespace, not evidence of an existing remote exploit.

**Change:** provision a private profile root with verified ownership/permissions, bind it once to the store, and perform operations relative to an owned directory capability with a no-follow/beneath policy. Keep OS-specific low-level code isolated behind safe interfaces; do not add unsafe Rust where a reviewed safe wrapper suffices. Test ancestor symlinks, leaf swaps, wrong roots and cross-profile reuse.

###### P2 — Non-Unix durability is silently weaker
The non-Unix `sync_directory` returns `Ok(())` without performing a persistence operation. An API that reports durable publication must expose this limitation, reject unsupported durability profiles, or use a tested platform backend. Passing Linux CI cannot qualify Windows/macOS persistence semantics. Test failure/retry after each flush/link/directory-sync boundary, including an already-existing target following an earlier failed sync.

###### P2 — Crash orphans and retention integration need ownership
The publish-before-metadata order safely avoids ordinary metadata pointing to an unwritten blob, but a crash or failed metadata registration after publication can leave an unregistered blob. Drop cleanup handles normal errors, not process death. #801's metadata-driven collector does not by itself enumerate unregistered files/quarantine leftovers. Add bounded orphan reconciliation with a grace period and active-ingestion fencing; never remove another live writer's staged publication.

###### API and performance improvements
A caller-supplied `ArtifactScope` proves matching strings, not an authenticated grant: the supervisor/broker must derive it and prevent untrusted workers from choosing it. `VerifiedArtifact::into_file` intentionally transfers a readable handle, so later suppression cannot retract bytes already handed out; document suppression as denial of new retrieval and define active-reader/export behavior. Add per-profile quotas, incremental cancellation, no unbounded quarantine concurrency, and a consistent media-type contract with #787 (128 versus 255-byte limits).

Qualification remains incomplete until filesystem confinement, crash reconciliation and platform-specific durability evidence exist.

#### Comment 5706481921 — Jordan-Hall — 2026-09-17T00:19:20Z

Source: https://github.com/Jordan-Hall/browser/pull/797#issuecomment-5706481921 | Updated: 2026-09-17T00:19:20Z

###### Additional finding — P1: reference registration can succeed without registering a reference

Reviewed head `07936f3eecbba10d608b520427ab13360cf79eee`; owning task #133, downstream deletion guard #134. This is separate from the three earlier fixed findings and the existing filesystem review.

In `register_artifact_reference` (`artifacts.rs`), `reference_kind` and `reference_id` use `BoundedText`, whose constructor permits empty strings. The table requires both fields to be non-empty, but the insert uses **`INSERT OR IGNORE`** and ignores its affected-row count. An empty kind or ID therefore violates a CHECK constraint, inserts nothing, and still reaches `transaction.commit()?; Ok(())`.

A caller can consequently believe a durable retention/reference pin exists when the reference count is still zero. That undermines reference-based GC once the retention path is repaired/enabled.

**Evidence:** I executed a reduced version of the exact reference-table constraints and insert on SQLite 3.46.1. Empty kind: no SQL error, 0 rows inserted, count remains 0. Empty ID: the same. Valid reference: 1 row. Repeated valid reference: 0 new rows, count remains 1. This is SQL-level reproduction plus source review, not local execution of Rust/rusqlite.

**Fix direction:** validate non-empty reference identifiers; replace broad `OR IGNORE` with targeted `ON CONFLICT(artifact_id, reference_kind, reference_id) DO NOTHING`; and distinguish insertion from an already-existing identical reference. Do not treat arbitrary constraint failures as idempotent success.

**Rust regressions:** empty kind/ID rejected; valid registration increments count; identical retry remains idempotent; wrong scope rejected; a successful retained receipt/evidence reference blocks GC. Preserve the already-fixed retry-byte, reference-count scope and directory-sync behavior.

No implementation change or fix is claimed by this review.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-02-t04-inbox",
  "base_sha": "87e0872da7290dd5a057526912dd3a88e4120816",
  "head_branch": "core-02-t05-artifacts",
  "head_sha": "e2409171ba94979fe007d69db2ceae7934a2405a",
  "merge_commit_sha": "f2b5dda1e44d2f877bd754f5617dac8041a4773f"
}
```

<a id="issue-801"></a>
## #801 — CORE-02.T06: implement reference-safe retention and deletion

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/801
**Created:** 2026-09-16T10:44:26Z | **Updated:** 2026-09-17T21:11:14Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Task and status

Implementation for **#134 (`CORE-02.T06`)**, parent #3, CORE epic #13 and programme #1. Complete individual task/acceptance inventory: #802 and [CORE coverage](https://github.com/Jordan-Hall/browser/blob/core/production-hardening-20260917/docs/core-task-coverage.md).

**Retention fixes and lifecycle regressions are now committed on this branch. CORE and production storage qualification remain incomplete.**

#### Current implementation and resolved defects

- Suppression updates the underlying artifact_handles_all table, not the INSTEAD OF-triggered view. The three previously failing ConcurrentSuppression tests now pass.
- Eligibility and already-queued exclusions are applied before GC candidate LIMIT, preventing held/referenced/live-shared prefixes from starving later eligible blobs.
- Expired-hold pruning is bounded; GC selection favors less-attempted entries.
- Final eligibility, unlink, directory synchronization and metadata cleanup share the same immediate-writer boundary used by #797 publication and reference registration. Hold/suppression validation also runs within that protocol.
- Deterministic two-connection regressions cover competing publication/reference/hold writers and content protected before collection.
- Retry deletion repeats the directory barrier even when the file is already absent. Diagnostics are bounded on UTF-8 boundaries without panicking during error recording.
- Includes #797's new invalid-reference regression so a rejected pin cannot be reported as a successful retention reference.

#### Stack and evidence

Base: #797 / `core-02-t05-artifacts`. Next: integration PR #802. Head: `96e66e0b977b3ad4c716368b0baa430a266cb686`.

[Passing CI run 35274668998](https://github.com/Jordan-Hall/browser/actions/runs/35274668998). Local Rust 1.98.1 verification passed formatting, warnings-denied Clippy, **117 runtime tests and 4 doctests**, architecture checks, locked fuzz compilation and smoke conformance. Counts are cumulative for the complete existing implementation stack. This replaces the historical failing retention result; it does not claim the old failing commit itself became green.

#### Remaining acceptance and operating boundary

Exclusion is for cooperating writers using the same database and trusted root. A second database sharing the root, hostile path/ancestor replacement and missing profile-wide ownership are not covered. Backup coordination (#135), process/VM power-loss qualification (#136), filesystem trust boundaries and platform durability still require implementation/evidence.

Filesystem barriers hold the writer lock and can extend latency; callers must handle busy errors. Non-Unix directory durability fails explicitly rather than pretending to persist deletion. Linux two-connection/fault-injection tests do not qualify Windows/macOS power-loss behavior.

Original/predecessor history is preserved without force-pushing. No issue closure, PR merge/approval or blanket review resolution was performed.

### Discussion (3 comments)

#### Comment 5696195683 — chatgpt-codex-connector[bot] — 2026-09-16T10:44:34Z

Source: https://github.com/Jordan-Hall/browser/pull/801#issuecomment-5696195683 | Updated: 2026-09-16T10:47:22Z

<!-- codex-pull-request-review-summary -->

###### Codex Review Summary

This comment shows the latest Codex review activity on this pull request.

| Review | Status | Commit | Review trigger |
| --- | --- | --- | --- |
| 📝 **Code Review** | ✅ **Completed** <relative-time datetime="2026-09-16T10:47:20.355233Z">2026-09-16T10:47:20.355233Z</relative-time> | `7cb0f86` | PR opened |



<details> <summary>ℹ️ About Codex in GitHub</summary>
<br/>

[Your team has set up Codex to review pull requests in this repo](https://chatgpt.com/codex/cloud/settings/general). Reviews are triggered when you
- Open a pull request for review
- Mark a draft as ready
- Comment "@codex review" or "@codex security review".

Codex reacts with 👀 while any review is running, comments if it has suggestions, and reacts with 👍 once all reviews finish with no findings.

</details>

#### Comment 5705884960 — Jordan-Hall — 2026-09-16T23:14:57Z

Source: https://github.com/Jordan-Hall/browser/pull/801#issuecomment-5705884960 | Updated: 2026-09-16T23:14:57Z

<!-- intent-core-review:2026-09-16:pr-801 -->
###### CORE review — CORE-02.T06

Reviewed head `fbfcfceb17540e5fe80bec555385e097f3787455`, migration 6, retention/GC paths, existing comments and CI run `35086737317` / job `104763204222`, against #134 / #3. No code changes.

###### P1 — First suppression always rolls back (confirmed)
The existing `discussion_r4025136333` is correct and still present. `artifact_handles` is a view with an INSTEAD OF UPDATE trigger. `suppress_artifact` tests the outer statement's affected-row count for exactly 1; it is 0, so the method returns `ConcurrentSuppression` and drops/rolls back the transaction.

The latest CI passes formatting/Clippy but fails all three retention tests with that error. I also independently reproduced the SQL/decision with SQLite 3.46.1: `changes() == 0`, the base-table timestamp becomes set inside the transaction, and the error-path rollback restores NULL. This reproduction is not execution of the Rust crate; the repository CI supplies the Rust failure evidence.

**Fix direction:** use a scoped compare-and-update against the base table under the transaction, or verify the committed logical result without relying on view changes counts. Preserve idempotent AlreadySuppressed behavior and test a genuine concurrent conflict separately. Do not weaken the failing tests.

###### P2 — Recovery sweeps starve later eligible blobs
Existing `discussion_r4025136345` remains valid: LIMIT is applied to sorted suppressed candidates before eligibility checking. A retained prefix can prevent later blobs from ever being examined. Filter eligibility in SQL or use a durable keyset scan with a work budget. Similarly, repeatedly failing oldest GC entries need bounded retry/backoff so they cannot monopolize every batch.

###### P1 latent cross-writer race — Eligibility recheck is after irreversible unlink
Once suppression is repaired, `run_artifact_gc` checks eligibility, commits `deleting`, unlinks the file, then checks eligibility again. #793 does not enforce a profile-wide exclusive owner, so another store can add a hold/live handle between the first check and unlink. Detecting the change afterward cannot restore the deleted file. Hold addition also deletes the queue row; the subsequent failure UPDATE may affect no row.

**Change:** establish true exclusive owner/ingestion coordination, or use a deletion generation/state enforced by all handle/reference/hold registration paths before physical deletion. Revalidate while holding the relevant exclusion. A post-unlink check alone is not protection.

###### Suppression representation and crash semantics
Migration 6 does not filter hidden handles out; it changes their exposed byte size to -1. This turns a legitimate lifecycle state into a generic invalid-record error and makes the PR's “fail-closed view” description misleading. Use explicit lifecycle/tombstone semantics, scoped normal versus retention-only queries, and an intentional suppression error. Keep real corruption distinguishable from user deletion.

**Acceptance:** repaired three existing tests; live-handle/retained-receipt/active-hold cases; 65+ ordered candidates with a retained prefix; crash before/after unlink; hold/ingestion racing GC; deletion failure retry; preserved tombstone identity policy; missing-file reconciliation and correct directory synchronization. CORE-02.T06 is not ready to be marked complete while this head's CI is failing.

#### Comment 5706541001 — Jordan-Hall — 2026-09-17T00:26:36Z

Source: https://github.com/Jordan-Hall/browser/pull/801#issuecomment-5706541001 | Updated: 2026-09-17T00:26:36Z

###### Review follow-up — fix the failing suppression path, then qualify the whole maintenance budget

Rechecked `fbfcfceb17540e5fe80bec555385e097f3787455`, migration 6 and retention helpers. Run `35086737317` / job `104763204222` passes formatting/Clippy but fails all three retention tests with `ConcurrentSuppression`. The existing view-rowcount, prefix starvation and post-unlink race findings remain applicable; this is not ready for completion.

**Additional P2: the batch limit does not bound hold pruning.** Both maintenance entry points call `prune_expired_holds` before inspecting `limit`; that helper deletes every expired hold in a single statement. Even `limit = 0` performs this mutation. With a large hold table, a nominally 64-item maintenance step can hold the writer for an unbounded number of deletions. Give expiry pruning its own bounded, resumable budget, or use the existing expiry predicate for eligibility and prune incrementally. Test zero budget, a large expired set and fair progress behind retained/failed prefixes.

**Additional P2: error reporting can panic on UTF-8.** `record_gc_failure` calls `detail.truncate(2048)` without checking a character boundary. A long non-ASCII I/O error can place byte 2048 inside a character and panic while recording an already-failed deletion. Reuse a UTF-8-safe bounded diagnostic helper; test a constructed `io::Error` containing `"a".repeat(2047) + "é"` and a longer Unicode message. Rust documents this panic condition at https://doc.rust-lang.org/std/string/struct.String.html#method.truncate . This is source/API-contract analysis, not a locally run Rust test.

Also make post-commit enqueue failures distinguishable from rollback: suppression, reference removal and hold release commit before eligibility enqueue. Recovery must rediscover committed-but-unqueued work without telling callers the mutation never happened.

Before enabling GC, include the new false-reference-success regression from #797 (comment `5706481921`): a claimed durable pin must really exist. Do not weaken the three currently failing tests or label these changes fixed without commit and rerun evidence.


---

### PR metadata

```json
{
  "draft": false,
  "merged_at": null,
  "base_branch": "core-02-t05-artifacts",
  "base_sha": "e2409171ba94979fe007d69db2ceae7934a2405a",
  "head_branch": "core-02-t06-retention",
  "head_sha": "96e66e0b977b3ad4c716368b0baa430a266cb686",
  "merge_commit_sha": "9e5df5590606cfa0e2db9fff24dfd2d8e586725b"
}
```

<a id="issue-802"></a>
## #802 — CORE: integrated production-hardening remediation and regression gates

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/802
**Created:** 2026-09-17T16:13:50Z | **Updated:** 2026-09-17T21:16:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

#### Current result — all existing CORE PRs updated, CORE not complete

**All 15 existing CORE PR branches and descriptions have been updated:** #785, #786, #787, #788, #789, #790, #791, #792, #793, #794, #795, #796, #797, #801 and this PR. The hardening code now lives in its owning task PR, not only in the integration branch. Each original task PR has a passing CI run, an exact tested head, its own issue/parent/epic references, predecessor/successor information and explicit remaining acceptance work.

**This does not complete CORE epic #13.** Fourteen feature tasks have implementation PRs with acceptance still open; eighteen feature tasks have no implementation in this stack. Baseline #113 and the four epic integration/readiness tasks also remain open. Referencing a task is not implementing or accepting it. This PR remains draft; no issue is auto-closed.

Programme: #1. CORE epic: #13. Parent requirements: #2, #3, #4 and #5.

#### Source changes in this update

The earlier contract, IPC, migration, operation/outbox/inbox and artifact/retention hardening has been distributed through the fourteen owning branches. Each update preserves the original branch history and its updated predecessor without force-pushing. Early branches contain only their stage's packages, migrations and tests, with appropriate locked dependency inputs.

Two additional correctness fixes are now implemented and tested:

- **#124 / #788:** reject duplicate JSON object keys, including equivalent escaped spellings and nested duplicates, before typed payload interpretation. The decoder retains node, collection and nesting budgets; identical keys in different objects remain legal.
- **#133 / #797:** reject invalid artifact-reference inserts instead of reporting a retention pin that does not exist. A targeted duplicate-key conflict handler replaces broad INSERT OR IGNORE; CHECK failures propagate, while repeating a valid exact pin remains idempotent.

This integration PR now adds the complete task ledger, its CI validation and release-gate documentation over the updated #801 base. Its current diff is seven files, not the earlier 33-file hardening diff: the runtime fixes have moved upstream into their owning PRs. The existing offline review-input export is retained; temporary bundle/import workflows and transport files are absent from the final tree.

#### Exact stack and verification

- Head: `84d2dd0512fb8e6ae02e4b41562676f4b5e8a215`, branch `core/production-hardening-20260917`.
- Updated base: #801, `core-02-t06-retention` at `96e66e0b977b3ad4c716368b0baa430a266cb686`.
- Tested merge after all task branches were updated: `dba74c4e387995b360ed8876f80d32fc46365b29`.
- Source tree: `c11299b9a3427a369158dee1e51d7dc5c89b9994`.
- [Passing final CI run 35275548059](https://github.com/Jordan-Hall/browser/actions/runs/35275548059), [job and logs](https://github.com/Jordan-Hall/browser/actions/runs/35275548059/job/105385179026).

The integrated suite passes **117 unit/integration tests and 4 compile-fail doctests**, with no failed or ignored tests. Formatting, warnings-denied Clippy, tracked/unchanged workspace and fuzz lockfiles, the actual architecture checker, excluded fuzz-target compilation, smoke conformance, the 37-task coverage check and verification-evidence upload pass. Each original task branch was also independently compiled and tested locally with the pinned Rust 1.98.1 toolchain and verified vendored dependencies before publication. Remote verification uses Ubuntu 24.04 and Rust 1.98.1.

Workspace lockfile SHA-256: `9b1431bc16b72ad6ce1b502e7258b0d34a4537ea2e3c27bac4527b03dd0ada5e`.

Fuzz lockfile SHA-256: `530417e779c1a591199d78cc3516fcfff92d396b7aa43e0f2066ffb9f87a3641`.

The fuzz gate is a compile check, not an instrumented fuzz campaign. Linux unit/two-connection/fault-injection tests are not process/VM power-loss or cross-platform qualification. Passing these gates does not establish production readiness.

#### Every individual CORE feature task

The status below is deliberately not a completion checkbox. The first fourteen rows have source implementations, but some still require substantial integration code as well as acceptance evidence.

| Task | Individual issue | Owning PR / verified CI | Remaining acceptance |
|---|---|---|---|
| CORE-01.T01 — workspace and architecture checks | #121 | #785 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274507618) | Independent review and integrated baseline acceptance. |
| CORE-01.T02 — typed identities and values | #122 | #786 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274516793) | Wire representation/provider-resource validation sign-off. |
| CORE-01.T03 — durable record schemas | #123 | #787 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274547485) | Complete record-family fixtures and authority semantics. |
| CORE-01.T04 — bounded framing and errors | #124 | #788 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274566964) | Stable error-wire representation and instrumented parser qualification. |
| CORE-01.T05 — launched-worker authentication | #125 | #789 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274576263) | Actual spawned-process authentication and registry-owned global one-use binding. |
| CORE-01.T06 — version negotiation/schema evolution | #126 | #790 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274585080) | Authenticated codec ownership and validated durable migration integration. |
| CORE-01.T07 — cancellation/deadlines/backpressure | #127 | #791 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274593054) | Real worker cancellation, dispatch deadlines/epochs and bounded generation retirement. |
| CORE-01.T08 — conformance and fuzzing | #128 | #792 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274600376) | Instrumented bounded fuzz execution, complete fixtures and process conformance. |
| CORE-02.T01 — database ownership/migrations | #129 | #793 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274613284) | Profile-wide ownership, filesystem trust boundary and restore validation. |
| CORE-02.T02 — operations/journal | #130 | #794 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274623965) | Evidence-bound transitions and original/compensation lineage. |
| CORE-02.T03 — transactional outbox | #131 | #795 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274632851) | Authority-bound opaque dispatch, generation fencing and abandoned-attempt recovery. |
| CORE-02.T04 — inbox/cursors | #132 | #796 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274638664) | External source-precondition integration and repair/recovery acceptance. |
| CORE-02.T05 — scoped artifact storage | #133 | #797 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274654944) | Exclusive trusted-root ownership, hostile-path protection and platform durability. |
| CORE-02.T06 — retention/deletion | #134 | #801 / [pass](https://github.com/Jordan-Hall/browser/actions/runs/35274668998) | Profile/backup coordination and process/power-loss qualification. |
| CORE-02.T07 — backup, restore and integrity | #135 | No implementation PR | Not implemented in this stack. |
| CORE-02.T08 — storage fault/power-loss qualification | #136 | No implementation PR | Not implemented/qualified in this stack. |
| CORE-03.T01 — worker registry and launch specifications | #137 | No implementation PR | Not implemented in this stack. |
| CORE-03.T02 — process lifecycle and health | #138 | No implementation PR | Not implemented in this stack. |
| CORE-03.T03 — admission and foreground priorities | #139 | No implementation PR | Not implemented in this stack. |
| CORE-03.T04 — platform resource constraints | #140 | No implementation PR | Not implemented in this stack. |
| CORE-03.T05 — lease-first cancellation/revocation | #141 | No implementation PR | Not implemented in this stack. |
| CORE-03.T06 — crash-loop policy/degraded service | #142 | No implementation PR | Not implemented in this stack. |
| CORE-03.T07 — resource observation/cooperative yields | #143 | No implementation PR | Not implemented in this stack. |
| CORE-03.T08 — concurrent supervisor qualification | #144 | No implementation PR | Not implemented/qualified in this stack. |
| CORE-04.T01 — recovery classification | #145 | No implementation PR | Not implemented in this stack. |
| CORE-04.T02 — consistent checkpoints | #146 | No implementation PR | Not implemented in this stack. |
| CORE-04.T03 — startup recovery before dispatch | #147 | No implementation PR | Not implemented in this stack. |
| CORE-04.T04 — uncertain-state reconciliation | #148 | No implementation PR | Not implemented in this stack. |
| CORE-04.T05 — no-production replay contexts | #149 | No implementation PR | Not implemented in this stack. |
| CORE-04.T06 — provider resume/reseeding | #150 | No implementation PR | Not implemented in this stack. |
| CORE-04.T07 — recovery decisions in task centre | #151 | No implementation PR | Not implemented in this stack. |
| CORE-04.T08 — restart/suspend/corrupt-state qualification | #152 | No implementation PR | Not implemented/qualified in this stack. |

#### Every baseline and epic-integration task

| Task | Individual issue | Status |
|---|---|---|
| PROGRAMME.T03 — freeze core contracts and process authority inventory | #113 | Open: required baseline evidence/sign-off not supplied. |
| EPIC-CORE.T01 — runtime ownership and wire contracts | #211 | Open: ownership and integrated contract acceptance missing. |
| EPIC-CORE.T02 — durable dispatch and worker supervision | #212 | Open: dependent implementation and integration missing. |
| EPIC-CORE.T03 — crash and bounded replay end to end | #213 | Open: executable end-to-end qualification missing. |
| EPIC-CORE.T04 — runtime operational readiness | #214 | Open: production operational evidence missing. |

The [committed human-readable matrix](https://github.com/Jordan-Hall/browser/blob/84d2dd0512fb8e6ae02e4b41562676f4b5e8a215/docs/core-task-coverage.md), [machine-readable ledger](https://github.com/Jordan-Hall/browser/blob/84d2dd0512fb8e6ae02e4b41562676f4b5e8a215/docs/core-task-coverage.json) and [coverage checker](https://github.com/Jordan-Hall/browser/blob/84d2dd0512fb8e6ae02e4b41562676f4b5e8a215/scripts/check_core_coverage.py) retain all **37 individual task references**. CI checks inventory completeness and rejects a production-ready flag while any task remains unaccepted. The ledger validator checks recorded evidence/status consistency; it does not independently certify the implementation behind an evidence link.

#### Compatibility, operating boundary and release gate

Money's decimal-string wire representation, proposal descriptor API, unknown-field rejection and aggregate inbox limits are intentional pre-release contract changes. They require explicit compatibility decisions/versioned migration before accepting old persisted data. Hash consistency is neither artifact-byte verification nor authorization.

Artifact publication/reference/hold/GC exclusion applies to cooperating writers sharing the same database and trusted root. It is not profile/root ownership or protection against hostile ancestor replacement/a second database using that root. Filesystem barriers extend writer-lock duration; callers must handle bounded busy errors. Non-Unix directory durability is explicitly unsupported; macOS power-loss behavior is not qualified by these Linux checks.

Before release, complete the missing supervisor/recovery/backup implementations and accepted schema/authority integration, then demonstrate actual-process cancellation/fencing, startup recovery and read-only reconciliation with an independent external-effect ledger proving uncertain effects are not blindly resent. Supply exact commit, lockfiles, OS/filesystem and residual limitations. See [production-readiness details](https://github.com/Jordan-Hall/browser/blob/84d2dd0512fb8e6ae02e4b41562676f4b5e8a215/docs/core-production-readiness.md).

No PR was merged or approved, no issue was closed, and review findings were not blanket-resolved. **All existing PRs updated: yes. All individual tasks referenced: yes. All CORE tasks complete/production-ready: no.**

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

### PR metadata

```json
{
  "draft": true,
  "merged_at": null,
  "base_branch": "core-02-t06-retention",
  "base_sha": "96e66e0b977b3ad4c716368b0baa430a266cb686",
  "head_branch": "core/production-hardening-20260917",
  "head_sha": "84d2dd0512fb8e6ae02e4b41562676f4b5e8a215",
  "merge_commit_sha": "dba74c4e387995b360ed8876f80d32fc46365b29"
}
```

<a id="issue-803"></a>
## #803 — CORE-02.T07: authenticated snapshots and dispatch-disabled restore

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/803
**Created:** 2026-09-18T06:43:48Z | **Updated:** 2026-09-18T06:45:07Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Scope and references

Implementation for #135 (`CORE-02.T07`), parent #3, CORE epic #13, programme #1. Contributes actual process-interruption regressions to #136 and the durable restore/startup dispatch gate to #147; those larger tasks are not declared complete. Stacked above #802 without rewriting any existing branch.

#### Implemented source

- Bounded, authenticated plaintext snapshot export using SQLite's backup API, a complete exact artifact inventory and an independently supplied HMAC-SHA256 key.
- Database plus blob copy under the shared writer exclusion boundary so cooperating publication/reference/hold/GC writers cannot invalidate the snapshot. No temporary pin leases are restored or left immortal after a crash.
- Descriptor-relative Linux GNU file access, no-follow traversal, restricted directories/files, no-replace publication and explicit publication-uncertain outcomes after rename.
- Authenticate manifest before interpreting its database; verify exact schema/ledger/inventory, database bytes and every blob before publishing a fresh restore. Extra/missing/corrupt/duplicate artifacts fail closed.
- Suppressed artifacts, durable references and retention holds are preserved. Restore creates a new runtime epoch with durable dispatch disabled; begin_dispatch enforces the gate inside its writer transaction.
- Real child-process kill tests during copying and after publication, plus deterministic writer exclusion, corruption, wrong-key, interrupted-copy and no-overwrite regressions.

#### Verification status

The authored source has been formatted, compiled and tested locally with the pinned Rust 1.98.1 toolchain and verified offline dependency sources: 128 unit/integration tests and 4 compile-fail doctests pass, along with warnings-denied Clippy, architecture checking, separately locked fuzz compilation and smoke conformance. The exact source patch is being imported on this isolated branch; the temporary transport files/workflow will be removed before final CI qualification. A passing initial bootstrap workflow is not evidence that the new source passed CI.

#### Explicit limits and remaining acceptance

This exports **plaintext**, not encrypted backups. Callers must explicitly consent to writing sensitive data unencrypted and supply the trust key independently; key-store/product integration remains required. Linux GNU is the only implemented snapshot platform. Whole-copy writer exclusion trades concurrency for a simple provable snapshot boundary. The elapsed budget is checked between I/O operations, not a preemptible disk deadline. Process kill tests do not prove VM/power-loss durability.

The new file-backed dispatch gate is deliberately closed by default. Production ownership, startup recovery, fresh authority validation and an activation API are subsequent integration work; this PR must not be merged as though it supplies a working production dispatcher. Existing in-memory fixtures retain an explicitly isolated test-only activation path. No old lease, snapshot or hash becomes current authority.

CORE remains incomplete. No automatic issue closures, approvals or merges are requested by this PR.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

### PR metadata

```json
{
  "draft": true,
  "merged_at": null,
  "base_branch": "core/production-hardening-20260917",
  "base_sha": "84d2dd0512fb8e6ae02e4b41562676f4b5e8a215",
  "head_branch": "core-02-t07-authenticated-snapshots",
  "head_sha": "4fface918d13be0f636bd7b0a3716aedb9b08eb4",
  "merge_commit_sha": "044e727ba55c64f158a63da1de76b0d8cbaf76d6"
}
```

<a id="issue-804"></a>
## #804 — CORE-04: deterministic recovery policy and captured-only replay

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/804
**Created:** 2026-09-18T07:07:18Z | **Updated:** 2026-09-18T07:15:35Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Scope and task references

Adds actual implementation foundations for #145 (`CORE-04.T01`), #149 (`CORE-04.T05`) and #150 (`CORE-04.T06`), under parent #5, CORE epic #13 and programme #1. Stacked above #803 and #802. These are partial task implementations, not an assertion that startup recovery, provider integration or the full CORE epic is complete.

#### Implemented library

- A versioned exhaustive recovery classifier over effect class, durable stage, exact operation/account/capability/argument/attempt binding, original versus compensation phase, authority, source freshness, deadlines and outcome evidence.
- Strong commit evidence survives a lost acknowledgement or expired grant without permitting another write. Contradictions, unsupported policies, missing lineage and mismatched evidence fail closed. Uncertain external writes require read-only reconciliation; provider-idempotent continuation is only a plan requiring an exact current guarantee.
- Bounded per-capability recovery-policy registration rejects duplicates, unknown effects and unregistered capabilities.
- Captured-only replay verifies record identity, scope, ordering, schema, bytes and hashes, with immutable count/byte/time budgets and sticky expiry. Its context has no network, process, database, credential or execution callback capability.
- Read-only provider-resume/reseed planning binds account, provider, durable checkpoint, expiry, authority and source freshness. Bounded repeated failure leads to manual recovery; absent or stale sessions reseed from durable state rather than making provider history authoritative.
- Secret-bearing session and captured-content debug output is redacted.

#### Verification

Locally compiled and tested with pinned Rust 1.98.1 and verified offline dependencies: the complete stack passes 147 runtime tests and 4 compile-fail doctests. The new crate adds 19 tests, including a 2,816-case state/effect/authority/source matrix, individual evidence-binding mutations, compensation and expired-idempotency cases, replay limits/corruption and provider identity/retry cases. Formatting, warnings-denied Clippy, actual architecture checking, locked fuzz-target compilation and smoke conformance pass locally.

The exact tested source patch is being imported on this isolated branch. Temporary import files/workflow will be removed and the resulting source will receive its own read-only CI run; bootstrap success is not that verification.

#### Remaining integration / limits

Inputs must come from trusted durable-fact and provider-evidence verifiers; a hash or deserialized decision is not authorization. The returned decisions are plans, never executable grants, and the live broker must revalidate immediately before acting. The durable original/compensation lineage schema and evidence adapters are not supplied here.

Replay's in-process capability boundary must still be integrated with actual credential-isolated workers; its inert bytes must not be routed into live dispatch. Provider SDK/credential integration and persisted resume-attempt accounting remain required. This pure crate does not claim process supervision, checkpoints, startup activation, recovery UI, VM/power-loss qualification or production readiness.

No automatic issue closure, merge or approval.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

### PR metadata

```json
{
  "draft": true,
  "merged_at": null,
  "base_branch": "core-02-t07-authenticated-snapshots",
  "base_sha": "4fface918d13be0f636bd7b0a3716aedb9b08eb4",
  "head_branch": "core-04-recovery-policy-and-replay",
  "head_sha": "5c6bbb4cc7af28fe1a20d0921409749bc74f2487",
  "merge_commit_sha": "f438769f0d2e7ea4b72467e34ec5d99b0c0be6af"
}
```

<a id="issue-805"></a>
## #805 — CORE-03: Linux worker supervision and real-process acceptance regressions

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/805
**Created:** 2026-09-18T10:42:50Z | **Updated:** 2026-09-18T11:16:17Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Published source and verification

Actual supervisor source is committed and verified above #804, #803 and #802. Programme #1; CORE epic #13; parent requirements #2 #3 #4 #5. This is an implementation PR, not a downloadable-patch handoff or a claim that all CORE acceptance is complete.

Head: `3bfdbc0f679e37368f5d8e04975390bb4d862295`. Source tree: `a353a917f2d6e660e2d4e82b9409c612849c32bb`. Tested merge: `eb7d0c85ab34dc532d0a2d475ac3a0a9c651a93e`. Base: #804 at `5c6bbb4cc7af28fe1a20d0921409749bc74f2487`.

[CI run 35336466160](https://github.com/Jordan-Hall/browser/actions/runs/35336466160), [full job log](https://github.com/Jordan-Hall/browser/actions/runs/35336466160/job/105572335064), [retained evidence](https://github.com/Jordan-Hall/browser/actions/runs/35336466160/artifacts/10542793958).

The actual source job on Ubuntu 24.04 / Rust 1.98.1 passes **177 runtime tests, 5 compile-fail doctests and 15 ledger tests**, including **20 real-child-process scenarios**. Formatting, warnings-denied Clippy, committed/unchanged locks, excluded fuzz compilation, actual architecture checker, contract smoke conformance and failure-aware evidence upload all ran successfully. The same source tree was freshly tested locally. Temporary import payloads and the write-capable import workflow are absent from the final source tree; final CI has read-only repository permissions.

#### Implemented source

- Hash-verified sealed ELF images, private descriptor-relative Unix endpoints, exact post-spawn kernel peer authentication and one-use per-channel bootstrap.
- Separate authenticated control/progress transports, implemented-codec negotiation and readiness gating; bounded frame, queue, byte and request populations.
- Scoped move-only request permits, shared monotonic revocation, late-response rejection, deadline checks and bounded generation retirement.
- Owned-child lifecycle, readiness/heartbeat/progress/OS-exit diagnostics, cancellation escalation, child reaping and bounded restart without implicitly replaying requests.
- Linux pre-exec address-space/CPU/descriptor limits and descriptor hygiene; unsupported unattended-untrusted profiles reject rather than silently downgrade.
- Content-free resource observations, coalesced cooperative yields and priority admission reservations.

The process suite exercises actual accepted work, wrong child/role/bootstrap/generation, replayed hello, replaced executable, inherited descriptor leakage, CPU/memory/file-descriptor bounds, stalled and suspended workers, saturated progress, cancellation, descendant termination, retirement and restart limits. In-process models are not labelled as process evidence.

#### Task and acceptance mapping

#137 #138 #139 #140 #141 #142 #143 #144 have source contributions here. Actual-process authentication/codec/cancellation also advances #125 #126 #127. This does not automatically accept any whole task: durable broker/outbox integration, hard aggregate process-tree containment, provider effects and native UI responsiveness are outside these tests.

The imported ledger and acceptance note retain the historical local-candidate publication wording from their source package. The source is now published in this PR at the exact head above; subsequent integration-ledger updates must replace those stale publication markers without pretending that publication itself is acceptance.

#### Remaining release boundary

This is the Linux approved/cooperative profile, not hostile same-user/cgroup/VM isolation. Per-process kernel limits, admission estimates and cooperative accelerator accounting are distinct. Durable external-dispatch authorization/fencing, original/compensation lineage, startup recovery activation, checkpoints, credential-isolated replay, provider/task-centre integration and actual platform/power-loss qualification remain acceptance gates. A stopped worker does not prove an external effect was rolled back.

All feature references: #121 #122 #123 #124 #125 #126 #127 #128 #129 #130 #131 #132 #133 #134 #135 #136 #137 #138 #139 #140 #141 #142 #143 #144 #145 #146 #147 #148 #149 #150 #151 #152. Baseline/integration: #113 #211 #212 #213 #214.

No merge, approval, issue closure or blanket acceptance is performed. CORE remains unaccepted; this PR stays draft pending the remaining task-specific requirements.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

### PR metadata

```json
{
  "draft": true,
  "merged_at": null,
  "base_branch": "core-04-recovery-policy-and-replay",
  "base_sha": "5c6bbb4cc7af28fe1a20d0921409749bc74f2487",
  "head_branch": "core-03-supervision-acceptance-20260918",
  "head_sha": "3bfdbc0f679e37368f5d8e04975390bb4d862295",
  "merge_commit_sha": "eb7d0c85ab34dc532d0a2d475ac3a0a9c651a93e"
}
```

<a id="issue-806"></a>
## #806 — CORE-04.T02: consistent workspace checkpoints and protected retention pins

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/806
**Created:** 2026-09-18T11:14:08Z | **Updated:** 2026-09-18T11:15:08Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Scope

Actual implementation for #146 (`CORE-04.T02`), parent #5, CORE epic #13 and programme #1. Stacked above #805/#804/#803/#802. Adds migration 008 without modifying migrations 001–007. No automatic issue closure, approval or merge.

#### Implemented source

- Runtime-owned versioned Workspace/GoalContract/Task graph with bounded serialization, graph-revision compare-and-swap, acyclic dependencies, immutable task/workspace association and preserved operation history.
- Atomic immutable checkpoints capturing graph, complete operation projections, journal positions, declared inbox cursors, provider-reference fingerprints, historical worker instances and runtime/store identity in one SQLite writer transaction.
- Descriptor-relative artifact metadata/hash/size/byte verification and exact retention pins coordinated with publication/GC. Missing, corrupt, suppressed or incorrectly scoped dependencies fail visibly.
- SQL-enforced pin ownership, idempotent retry, digest-checked release, immutable historical reads and corruption checks. A checkpoint never grants execution authority or revives old worker epochs.
- Authenticated snapshot/restore integration preserves checkpoints and pins while retaining disabled dispatch.
- 21 new checkpoint regressions, including a real child-process kill before commit, independent writer/CAS races, restore, suppression/GC, corruption and dependency validation.

#### Local verification and remote verification status

The exact authored source tree `1973f7518243d52141ccde5e316be76f4a883762` passed pinned Rust 1.98.1 verification locally: 198 runtime tests, 5 compile-fail doctests, 15 ledger tests, formatting, Clippy with warnings denied, locked fuzz-target compilation, actual architecture checks and existing smoke conformance. The release gate correctly refuses all 37 still-unaccepted task statuses.

This initial commit transports the exact tested source through a checksum/tree-verified import restricted to this isolated branch, without executing imported code under write permissions. The temporary transport will be removed and final read-only source CI inspected before a passing remote result is claimed. Initial baseline/import success is not new-source verification.

#### Acceptance limits

Linux GNU and a trusted root shared by cooperating writers are the implemented boundary; this does not establish profile-wide ownership or hostile-process isolation. Hashing holds the writer boundary and must run away from the UI/control executor; byte budgets are not preemptible disk deadlines. Provider fields are fingerprints, not bearer credentials; free-form domain text may still be sensitive. Backups remain authenticated plaintext.

Independent checkpoint review and the remaining live startup/broker authority, original/compensation lineage, provider integration, native task-centre, instrumented fuzz, process/VM power-loss and platform requirements are not replaced by checkpoint tests. This PR does not declare the CORE epic complete or production-ready.

Individual feature inventory: #121 #122 #123 #124 #125 #126 #127 #128 #129 #130 #131 #132 #133 #134 #135 #136 #137 #138 #139 #140 #141 #142 #143 #144 #145 #146 #147 #148 #149 #150 #151 #152. Baseline/integration: #113 #211 #212 #213 #214.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

### PR metadata

```json
{
  "draft": true,
  "merged_at": null,
  "base_branch": "core-03-supervision-acceptance-20260918",
  "base_sha": "3bfdbc0f679e37368f5d8e04975390bb4d862295",
  "head_branch": "core-04-t02-consistent-checkpoints",
  "head_sha": "85f5e8a3578e7f90f34597f616dc49c9df130166",
  "merge_commit_sha": "05e1c7ab5642c826352369ba696c466bd1a7e68d"
}
```

<a id="issue-807"></a>
## #807 — CORE-04: durable startup recovery, exact-attempt reconciliation and broker gates

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/807
**Created:** 2026-09-18T11:18:10Z | **Updated:** 2026-09-18T12:08:15Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Scope and current implementation pass

This draft is now stacked above the newly published checkpoint PR #806, preserving both branch histories. Its initial duplicate checkpoint contract was removed; no competing migration 008 is introduced.

Implementation targets: #147 (startup recovery), #148 (exact-attempt reconciliation), #130 (attempt lineage), #131 (durable authorization), #141/#212 (worker-generation fencing), and #151 (recovery view/actions). Programme #1; CORE epic #13; requirements #3 #4 #5. Existing checkpoints remain owned by #806/#146, worker supervision by #805, and pure recovery policy by #804.

The branch currently contains the integration contract on the existing implementation stack. Actual source changes and executed evidence are being added in this pass; initial documentation CI is not their verification. A checkpoint or recovery plan is not a grant, an accepted worker message is not a verified provider receipt, and an uncertain external effect must never be silently replayed.

No task auto-closure, PR merge, blanket review resolution or production-complete claim.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

### PR metadata

```json
{
  "draft": true,
  "merged_at": null,
  "base_branch": "core-04-t02-consistent-checkpoints",
  "base_sha": "85f5e8a3578e7f90f34597f616dc49c9df130166",
  "head_branch": "core-04-durable-recovery-integration",
  "head_sha": "420dbced30ad591f333f5c1fc25ac62f84479031",
  "merge_commit_sha": "2afd7f6cb3d833621ddfa9c3380222acf2806044"
}
```

<a id="issue-808"></a>
## #808 — CORE-04.T02: immutable task checkpoints and authenticated restore upgrades

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/808
**Created:** 2026-09-18T11:27:50Z | **Updated:** 2026-09-18T11:28:00Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Scope and references

Implementation for #146 (`CORE-04.T02`), under #5, CORE epic #13 and programme #1. Also contributes executable snapshot/storage-fault coverage to #135 and #136; checkpoint loading does not activate startup recovery #147. Stacked above #805, #804, #803 and #802 without rewriting their branches.

#### Implemented source

- Additive migration 008 with immutable task checkpoints, compare-and-swap graph heads, exact artifact membership and reserved checkpoint-owned retention pins. Migration SQL 001–007 is unchanged.
- Atomic capture of bounded workspace/task/goal graph, verified scoped artifacts, exact operation identity/revision/attempt/outcome and declared consumer cursors under one database writer boundary.
- Stable request-hash-bound retries; a raced or stale graph revision cannot partially change the head or pins.
- Load-time verification of complete operation membership at the journal watermark, current immutable identity, non-regressed cursors, payload/header/request hashes and protected artifact pins/bytes. Historical checkpoints are not execution grants and cannot be constructed by deserializing an unverified StoredCheckpoint.
- Scoped retirement of non-current checkpoints releases dependencies atomically while retaining immutable history. Generic reference APIs cannot fabricate or remove active checkpoint pins.
- Authenticated schema-7 snapshots restore into a freshly validated schema-8 destination only after exact source-schema/inventory validation. Source files remain unchanged; the receipt distinguishes source and restored schema versions; dispatch remains disabled under a fresh epoch.
- Correct published supervisor references in the complete 37-task ledger; no acceptance status is promoted merely because source now exists.

#### Verification

Fresh local Rust 1.98.1 verification passes **195 runtime tests, 6 compile-fail doctests and 15 ledger tests**, formatting, warnings-denied Clippy, locked fuzz compilation, architecture checks and existing smoke conformance. This adds 18 runtime regressions and a checkpoint import compile-fail test over the published #805 source.

Regressions include CAS races, rollback after pin insertion, cycles/duplicates/scope mismatches, aggregate budgets, stable retry, pin-retirement/GC races, corrupt/missing artifacts, rehashed omitted operations, corrupt heads, uncertain outcomes, current backup/restore, authenticated legacy upgrades and actual process death immediately before/after checkpoint commit.

The verified source is being imported on this isolated branch. Temporary import payloads/workflow will be removed before final source CI; initial CI/import success is not new-source verification. The final head and actually executed remote checks will be recorded here.

#### Remaining acceptance boundary

The graph is trusted runtime data, not approval authority. Live graph mutation/startup integration, profile/root ownership, source/authority revalidation, uncertain-effect reconciliation, provider/task-centre integration and actual VM/power-loss/platform qualification remain separate work. File verification is byte-bounded but not a preemptible disk deadline. The shared writer protocol does not protect against a hostile path writer or a second database sharing the artifact root.

No automatic issue closure, approval, merge or production-ready assertion. All CORE task acceptance remains tracked individually.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

### PR metadata

```json
{
  "draft": true,
  "merged_at": null,
  "base_branch": "core-03-supervision-acceptance-20260918",
  "base_sha": "3bfdbc0f679e37368f5d8e04975390bb4d862295",
  "head_branch": "core-04-t02-checkpoints-20260918",
  "head_sha": "039ec91d08722ea9bc8f255f9b4e1202fbd11796",
  "merge_commit_sha": "dfb9e87a6f1948d27835e22bd8350272da03dfb0"
}
```

<a id="issue-809"></a>
## #809 — CORE: join durable authorization to real worker dispatch and crash recovery

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/809
**Created:** 2026-09-18T13:34:53Z | **Updated:** 2026-09-18T13:47:21Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Implementation pass

Connects the actual Linux worker supervisor from #805 to the durable profile-owner, approval, attempt and recovery gates in #807. Stacked above #807 / #806 / #805 / #804 / #803 / #802. Programme #1, CORE epic #13; primary integration tasks #131 #137 #141 #147 #148 #212 #213.

The broker source has been authored and is undergoing local pinned-toolchain verification, including actual worker effects in a separate fixture ledger, lost acknowledgements, stale/expired authority and broker process death. This initial transport marker is not the implementation and its CI does not validate the new source. The exact tested source and final executed CI evidence will be committed and recorded in this PR during this pass.

The broker owns both supervisor and durable state, registers only authenticated Ready generations, commits exact attempts before immediate socket submission, persists transport uncertainty and never upgrades worker acknowledgement to a verified provider receipt. Larger than 1 KiB inline actions require an explicit bulk/artifact transport rather than truncation or implicit fallback.

No source is delivered as a user patch archive. No automatic task closure, PR merge, approval or production-complete assertion. All task-specific acceptance remains evidence-driven; live provider adapters, native recovery UI, isolated replay, resource containment and platform/power-loss qualification are not supplied merely by a passing fixture run.

### Discussion (1 comments)

#### Comment 5730784791 — Jordan-Hall — 2026-09-18T13:36:38Z

Source: https://github.com/Jordan-Hall/browser/pull/809#issuecomment-5730784791 | Updated: 2026-09-18T13:36:38Z

Coordination with #810: the new runtime candidate there targets the trusted conditional-provider adapter boundary, exact current-time recheck after SQLite commit, and atomic WorkerLease admission before external I/O. It uses the existing #807 RuntimeOwner rather than a new database/schema, with a separate provider process/SQLite rollback domain and actual broker-kill regressions that count submissions independently of effects. #809 targets the supervisor socket submission path. Both must preserve one authoritative RuntimeOwner and one attempt identity; neither process acknowledgement nor an independent API path may bypass the other's admission/evidence requirements. Before integration, reconcile the entry points and shared context instead of introducing competing authority stores or silently choosing a different migration. No completion or merge is asserted.


---

### PR metadata

```json
{
  "draft": true,
  "merged_at": null,
  "base_branch": "core-04-durable-recovery-integration",
  "base_sha": "420dbced30ad591f333f5c1fc25ac62f84479031",
  "head_branch": "core-broker-supervisor-dispatch-20260918",
  "head_sha": "9bf2850b8aed80f242f551fd46efdb836cfe1410",
  "merge_commit_sha": "e6ee30edc2c49d5f6ac8ad4986c3eb4e3dcfea85"
}
```

<a id="issue-810"></a>
## #810 — CORE runtime: authenticated broker dispatch and independent crash reconciliation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/810
**Created:** 2026-09-18T13:35:44Z | **Updated:** 2026-09-18T13:35:44Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Scope

Joins the actual supervisor in #805, checkpoints in #806 and durable recovery in #807. Programme #1; CORE epic #13; parent requirements #2 #3 #4 #5. Primary integration #212/#213 and task contributions #125 #126 #127 #130 #131 #137 #138 #141 #145 #147 #148 #151 #152.

The implementation pass supplies a runtime broker with bounded immutable adapter registrations, fresh-clock fencing, exact durable attempt context, final atomic worker admission after the database commit, and exact-attempt read-only reconciliation. A separate provider process owns an independent database and counts submissions separately from effects. Four actual broker-process kill boundaries demonstrate recovery without a second submission.

The authored candidate has passed local formatting, warnings-denied Clippy and the full workspace suite: 250 runtime tests, including 21 broker test entries (20 scenarios plus the subprocess fixture entry point). Source is being published on this isolated branch; this initial contract-only commit and its CI are not verification of that candidate. Final source/CI identity will replace this interim status.

#### Acceptance boundary

An atomic admission that wins before Stop is an in-flight operation, not a promise that external bytes can be undone. Adapter implementations are trusted code and must enforce their own conditional provider I/O and finite deadlines. Missing evidence remains uncertain; neither a lost acknowledgement nor a restored snapshot permits blind retry. This PR does not claim native UI, production-provider/key custody, hostile-process containment, complete fuzz/platform/power-loss qualification or all CORE acceptance.

All individual feature references: #121 #122 #123 #124 #125 #126 #127 #128 #129 #130 #131 #132 #133 #134 #135 #136 #137 #138 #139 #140 #141 #142 #143 #144 #145 #146 #147 #148 #149 #150 #151 #152. Baseline/integration: #113 #211 #212 #213 #214.

No automatic issue closures, merges, approvals or blanket acceptance.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

### PR metadata

```json
{
  "draft": true,
  "merged_at": null,
  "base_branch": "core-04-durable-recovery-integration",
  "base_sha": "420dbced30ad591f333f5c1fc25ac62f84479031",
  "head_branch": "core-runtime-broker-acceptance",
  "head_sha": "8d1d80e2ebaaf323205a75edb418b87164f0ff89",
  "merge_commit_sha": "dcbf8f0b668d8a121201117c94f281af53a07b5e"
}
```

<a id="issue-811"></a>
## #811 — docs: complete issues, epics and task document library for project context

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/pull/811
**Created:** 2026-09-18T19:03:26Z | **Updated:** 2026-09-18T19:04:44Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** none recorded

### Original description

#### Requested deliverable

Export the full repository issue/epic backlog into documents for saving in the project, not only CORE. This is a documentation-only branch off main; existing implementation branches, issue states and acceptance checkboxes are not changed.

The exporter paginates every open/closed issue and issue-conversation comment, separates PRs from issues, preserves original descriptions and comments, and groups the archive by native/declared parent relationships and epic child lists. Native/body-parent disagreements remain visible. A supplementary PR register preserves descriptions, conversation comments and source-head metadata without pretending to validate code or CI.

#### Document library

The restricted export job generates and commits `docs/project-archive/` on this branch:

- `ALL_ISSUES_AND_EPICS.txt` and `.md`: a single complete project-context document.
- `workstreams/`: one document per epic plus programme/ungrouped issues.
- `ISSUE_REGISTER.md`: complete linked issue catalogue.
- `PULL_REQUEST_REGISTER.md`: separate implementation-reference context.
- `snapshot.json`: exact source body/comment strings and metadata.
- `manifest.json`: counts, export time bounds, unique issue coverage, reconciled comment counts and SHA-256 checksums.

This initial PR commit adds the exporter. Generated-document counts and verified commit will be recorded after the job completes and its output is checked.

#### Export boundaries

This is a non-atomic retrieval snapshot. Historical/conflicting status text is preserved, not silently corrected. Attachments remain links; deleted content, edit histories, PR diffs, inline code reviews and CI logs are outside this issue-document export. Source-authored claims and GitHub open/closed states are not implementation acceptance evidence.

The job reads issues/PR metadata and writes generated files only to this isolated documentation branch, with no force push or persistent checkout credentials. It does not execute source issue text. No merge, issue closure, runtime changes or CORE completion claim.

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

### PR metadata

```json
{
  "draft": true,
  "merged_at": null,
  "base_branch": "main",
  "base_sha": "09197a237a57d53df798221225356e8abfee394a",
  "head_branch": "docs/project-issue-archive-20260918",
  "head_sha": "4f305fd41529843b48c221e77ed08be6d88c5fee",
  "merge_commit_sha": "ae8d672715a06d564440c0b40a7a1212441323ad"
}
```

