# Action authorization bindings

`ActionBinding` records the task, internal account, capability, provider-qualified target, exact argument artifact, source revision, byte interpretation, effect policy, approval requirement and proposal expiry. New proposals require an explicit `ActionContext`. Approval construction copies the proposal binding; imports reject an argument digest that contradicts the embedded binding. These records describe an action and grant no permission.

`exact_bytes_v1` preserves the approved artifact bytes. It does not name a JSON canonicalization algorithm or permit a provider transformation. Unsupported discriminants fail decoding. Provider account identifiers remain opaque and distinct from internal account IDs.

`StateStore::stage_record_bound_outbox` requires the proposal, approval and durable operation to have the same complete binding. It checks payload size/hash and expiry before the existing atomic staging transaction. Its return value remains non-executable. Identical bytes cannot be rebound to another provider, provider account, resource, source revision or policy through this record path.

`RuntimeOwner` remains the execution authority owner. Preparation derives the source revision from the binding, checks its effect and expiry, and persists it with the operation in one transaction. Approval, enqueue, claim and attempt start retain the existing live authority/source checks. Imported `Approved` data cannot replace runtime consent. Provider-target execution remains refused until the trusted provider-account mapping exists.

Migration 12 adds an immutable, versioned binding record and a creation-time requirement marker. Previous migrations and checksums are unchanged. Legacy rows are not backfilled: they remain readable, but cannot receive a binding later or obtain a new runtime approval or dispatch. A newly bound operation with a missing, malformed or contradictory binding fails loading. Recovery planning cannot label a mismatched bound source as fresh.

The contract format is a pre-freeze schema-1.0 addition. Legacy proposal/approval JSON without binding fields round-trips explicitly unbound. Older strict schema-1.0 readers reject the new fields. This is fail-closed behavior, not a claim of transparent backward compatibility. Older database readers reject schema 12.

Regression coverage includes complete proposal/approval round trips, unsupported canonicalization, contradictory imported approval digests, same-payload target rebinding, missing/changed source and policy bindings, immutable storage, corrupt identities/formats, legacy migration without guessed facts, and source changes after approval. Rejection tests inspect the outbox, attempts and operation journal, as appropriate, for residue.

This increment does not accept CORE-01.T03. Provider-target execution, trusted provider transformations and the separate wire-state distinction between accepted, verified and compensation outcomes remain unfinished.
