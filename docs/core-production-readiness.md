# CORE hardening status and release gates

The integrated published stack now includes retention (#801), the #802 tracker,
authenticated snapshots (#803), and pure recovery/replay/provider policy (#804).
The current local candidate adds real Linux cooperative-worker supervision and
acceptance regressions; see [its exact scope and limits](core-worker-supervision-acceptance.md).
The candidate is unpublished: no new GitHub PR or CI result is claimed.

This is not completion of epic #13 or a production release qualification. All
37 feature/baseline/integration acceptance states remain open. The corrected
[task ledger](core-task-coverage.md) separates published source, partial source,
unpublished source and absent implementation. No issue is closed by this document.

## Compatibility decisions

The repository is a pre-release implementation. `Money.minor_units` now uses a
canonical decimal JSON string, including negative adjustments and the complete
i128 range. Numeric JSON, leading zeroes, `-0`, explicit plus signs, exponent
notation and whitespace are rejected. This is intentionally a wire-format
change, not a transparent migration. Before accepting any previously exported
or persisted numeric-money records, define and test an explicit versioned
migration; do not silently accept two encodings for digest-bearing input.

`ActionProposalDescriptor` no longer accepts a redundant argument hash. Its
constructor derives the hash from the artifact reference, and deserialization
rejects mismatched hashes and unknown proposal fields. This checks consistency
between two claims; verification of the actual immutable bytes and approval,
account, capability, target and source-precondition authorization remain
separate runtime obligations. Existing schema identifiers are not being used
to claim backwards compatibility. Freeze the v1 wire schema only after these
representation decisions and complete record-family fixtures are reviewed.

Envelope unknown fields are rejected. A schema mismatch is rejected before
interpreting a payload as the current Rust type. The byte writer bounds output
while serializing; it cannot constrain arbitrary allocations or execution inside
a caller-provided Serialize implementation. Nested JSON and frame limits remain
in force. Duplicate object keys, including equivalent escaped spellings, are rejected before
typed payload parsing. Collection and node limits are also checked during decoding.

Protocol-offer limits count input entries, including duplicates, rather than
only unique set members. Construction consumes at most the configured maximum
plus one item, and wire sequences reject excess entries during deserialization.
An arbitrary iterator can still block inside next(). ControlCodec advertises
only the implemented v1.0 envelope codec. The local supervisor candidate now
owns that codec after each authenticated live-channel handshake; the codec alone
still is not authority. Accepted production/durable migration integration remains.

## SQLite, publication and collection

Compiled migration SQL for versions 1 through 6 is unchanged. Initialization
rejects populated databases with the default application ID while holding the
writer transaction. The migration ledger and user_version must agree. The
entire pending migration batch, including creation of the ledger, is atomic.
A failure in a later migration rolls back earlier DDL from that batch.

Artifact staging, bounded copying and hashing remain outside the SQLite writer
transaction. Publication acquires BEGIN IMMEDIATE before exposing the final
blob link and retains that transaction through metadata commit. GC takes the
same database's writer lock before its final eligibility check and retains it
through unlink, directory synchronization and metadata cleanup. Reference,
suppression and hold mutations read and write under that same writer boundary.
A second cooperating connection cannot commit a new protected handle in the
gap between eligibility testing and unlink. If a new handle commits first, the
collector defers without invoking unlink.

This trades some writer latency for a verifiable lifecycle invariant. A slow
filesystem barrier stalls other writers; callers must handle the existing
bounded busy timeout and retry policy. Hashing/copying is not inside that lock.
A crashed collection leaves its previously durable queue entry eligible for
retry; absence of the path is not proof of a completed directory barrier. The
retry repeats synchronization before removing metadata. Publication retries
also repeat the target-directory barrier, including deduplication arrivals.

The invariant requires every lifecycle participant to use the SAME database
and a trusted, exclusively assigned artifact root. It does not isolate an
attacker that can replace root ancestors, edit the database directly, or use a
second database against the same root. #803 adds descriptor-relative snapshot
traversal and backup/GC exclusion. Complete trusted-path coverage for live artifact
APIs, lifetime profile ownership, same-user threat modelling and platform
power-loss qualification remain release gates. Non-Unix directory durability
now fails with Unsupported rather than returning false success; this PR does
not qualify Windows support or macOS power-loss behavior.

GC applies live-handle/reference/hold/already-queued exclusions before candidate
LIMIT, bounds expired-hold pruning, and favors less-attempted queued items over
repeated failures. Suppression updates the actual table so affected-row counts
are meaningful, while normal retrieval remains fail-closed through the existing
view. Replacing that view's negative-size sentinel with a typed suppression
projection remains a contract improvement.

## Operations, outbox and inbox

An outcome cannot change the identity of the attempt it reports. Staging an
outbox message records the same identity in the operation projection, journal
and message. Beginning dispatch preserves that staged identity, and recording a
result verifies it. Compensation starts a distinct attempt; this does not yet
persist a separate compensation-origin/reconciliation lineage. No unconditional
NeedsReconciliation-to-Compensated shortcut is added.

Cancelled operations are filtered before the outbox claim limit. Cancellation
is still rechecked before beginning a durable attempt. Public outbox payload
access, generation fencing, source preconditions, abandoned-attempt discovery
and evidence-bound verification remain unresolved: an uncertain external effect
must never be blindly reset to pending and sent again.

One inbox event and all of its consumer effect payloads share the existing
1 MiB admission budget. Individual limits alone allowed excessive aggregate
hash/transaction work. Stored aggregate lengths and counts are checked before
materializing effect payloads, and effect reads use one read snapshot. A duplicate
acknowledgement verifies the existing event bytes and complete stored effect set;
missing or corrupt evidence returns an error rather than success. Recovery or
repair of corrupt records is not implemented by this rejection path.

## Verification and remaining gates

CI checks application and excluded-fuzz lockfiles, formatting, Clippy, all
workspace targets, doctests, excluded fuzz compilation, architecture boundaries
and the existing smoke conformance executable independently. Failed or skipped
checks remain explicit in the artifact; smoke conformance cannot label the
runtime production-ready. Artifacts retain tested source, exact lockfiles,
commit/platform metadata and per-gate outcomes on failure as well as success.
The diagnostic lockfile generation path does not satisfy the committed-lockfile
gate: untracked or changed lockfiles still fail that gate.

Regression coverage includes full-range money through the actual envelope codec,
lazy serializer overflow, pre-payload schema rejection, bounded repeated
capabilities, the actual architecture-check command with a renamed dependency,
foreign database rejection, migration rollback and concurrent initializers,
attempt substitution, cancelled claim starvation, inbox corruption and aggregate
bounds, two-connection publication/hold/reference-versus-GC exclusion, missing-file
barrier retries, UTF-8-safe failure recording and bounded GC selection. These are
unit/integration and deterministic fault-injection tests, NOT power-loss tests.

Outstanding implementation and evidence:

- #135–#136: #803 implements authenticated snapshots; key custody/product and
  activation integration plus storage/process/VM power-loss qualification remain.
- #137–#144: the unpublished supervisor supplies cooperative Linux launch,
  authentication/readiness, admission, limits, revocation, restart and accounting
  with real-process tests. Durable broker integration, hostile containment,
  complete resource isolation and product/platform qualification remain.
- #145–#152: #804 supplies pure classification/replay/provider planning. Trusted
  durable-fact integration, checkpoints, startup activation, read-only external
  reconciliation, credential-isolated replay workers, provider integration,
  recovery UI and independently observed restart scenarios remain unfinished.
- #113 and #211–#214: baseline, integrated acceptance and operational readiness.

Before release, also complete production codec integration, schema fixtures and
authority checks; maintain committed reproducible dependencies; supply pinned
instrumented and resource-bounded fuzz execution, filesystem isolation, complete
restore acceptance, and an external-effect ledger proving uncertain effects are not
silently repeated after a supervisor crash. Release acceptance must identify the
exact tested commit, lockfiles, OS/filesystem and residual limitations. A passing
CI smoke suite is necessary evidence for this PR, not sufficient evidence for
epic closure.

## Individual task references

See [the complete CORE task matrix](core-task-coverage.md) and its
[machine-readable inventory](core-task-coverage.json) for every feature, baseline
and integration task. The earlier #802 hardening was backported to its owning
PRs. The current supervisor candidate has not been published or backported;
its local test evidence must not be used to bypass independent source reviews.
