# Record collection and time invariants

Schema 1.0 contract types enforce intrinsic validity during construction and
deserialization. These checks apply to direct typed Serde imports and to the
canonical IPC, document import and migration paths. They do not authorize actions
or validate referenced records against live state.

## Collection bounds

`MAX_RECORD_COLLECTION_ENTRIES` is 16,384. Each field below independently uses this
ceiling, matching the existing default IPC collection budget.

| Record | Collection fields |
| --- | --- |
| GoalContract | clarified_constraints, authorized_accounts |
| Task | required_capabilities, result_artifacts |
| Evidence | source_observations |
| Receipt | evidence_ids |
| ViewDefinition | bindings, action_capabilities |

A private `RecordList` stores these fields and serializes as an ordinary JSON
array. Its streaming deserializer retains at most the limit and rejects the first
excess item. It does not allocate from an untrusted sequence size hint. Empty and
absent default collections remain valid; repeated values count toward the limit.
This bound introduces no uniqueness rule.

The two GoalContract collection builders now return `Result` and reject growth
beyond the same ceiling. Existing accessors still return immutable slices. Other
record constructors initialize empty lists.

IPC byte, depth, node and collection budgets remain independent. A caller can use
stricter limits, and a record below its collection ceiling can still exceed the
transport's byte budget. Increasing transport budgets cannot increase the schema
collection limit. Direct typed deserialization does not replace document admission
budgets.

## Time and state meaning

The following relationships must hold within each record:

- Workspace and Task: `updated_at >= created_at`.
- MemoryRecord: an optional `expires_at >= created_at`.
- Approval in the Approved state: an optional `expires_at >= approved_at`.

Equal times, valid dates before the Unix epoch and absent optional expiry remain
representable. Validation does not consult the current clock. Expiry effectiveness
at dispatch time remains a runtime check.

Workspace, Task and MemoryRecord constructors remain infallible because they
establish these relationships by construction. `Approval::new` now returns
`Result` and uses the same interval check as Approval deserialization. The public
ApprovalState enum is an input description; an invalid interval cannot become a
checked Approval record through either path. Imported Approved records still do
not establish user consent.

Deterministic evidence keeps the JSON representation `{"kind":"deterministic"}`
and the public Rust unit variant. Its private wire decoder rejects unknown origin
fields instead of silently ignoring them. ApprovalState's existing tagged format
and parser remain unchanged, including Pending without a details object.

This unit adds no cross-record timestamp comparisons, state-transition policy,
required terminal details, evidence-existence checks or provider authority. The
checked Operation execution representation retains its existing attempt, evidence
and compensation invariants.

## Compatibility and verification

All existing full and minimal canonical fixtures retain their JSON shape. Inputs
with contradictory intervals or unknown deterministic-origin members now fail
closed. Lists above the ceiling also fail through direct typed imports and through
IPC callers that deliberately enlarge transport budgets. The schema remains 1.0
during the pre-freeze contract work; this is a narrowing of previously accepted
invalid input.

Regression tests exercise every collection at its exact ceiling and one item over,
both growth builders, a lazy unbounded sequence, temporal construction/import
parity, every ApprovalState variant and the canonical import/migration/envelope
paths. These checks contribute to CORE-01.T03 acceptance and do not establish
production readiness.
