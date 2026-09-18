# Deterministic recovery and captured-only replay

Related tasks: #145 (`CORE-04.T01`), #149 (`CORE-04.T05`), #150 (`CORE-04.T06`).
Parent #5, epic #13, programme #1. This is an implemented library boundary, not
completion of the supervisor, durable startup recovery or production qualification.

## What the code does

`intent-recovery` is a safe Rust crate with no provider, network, process-launch,
mutable database or credential-store dependency. It computes versioned recovery
plans from verified facts, holds a bounded capability-policy registry, and replays
only explicitly supplied captured observations/evidence. The provider-session
policy decides between a read-only continuation plan, reseeding from a durable
checkpoint, reauthentication, source refresh or manual intervention.

The classifier binds evidence to operation, account, capability, argument hash
and exact attempt. Original and compensation phases remain separate; compensation
facts identify the original attempt and receipt. The state/effect matrix is an
exhaustive match. Unknown versions/effects, missing mandatory attempts, wrong
bindings and contradictory evidence are blocked rather than guessed.

A known local commit with failed acknowledgement is not a rollback. A verified
external commit survives worker cancellation, expired authority and a missing
local response. A compensation receipt is not the original action's success.
Without stronger bound evidence, a possibly sent external write requires
read-only reconciliation, not a reset to pending. Proven non-commit still needs
fresh approval for a new write. A provider-idempotent continuation plan additionally
requires an exact, unexpired, provider-specific guarantee and current authority
and preconditions; it is not a generic assumption about idempotency keys.

## Trust boundary: plans are not capabilities

All `RecoveryDecision` values are descriptive plans. They do not contain an
execution handle and `grants_execution_authority()` is always false. Deserializing
facts, a receipt hash or an idempotency reference does not verify the underlying
evidence. A trusted evidence verifier must construct the inputs; a production
dispatcher must revalidate current authority, epoch, deadline and preconditions
immediately before acting. No unchecked `Decision -> dispatch()` adapter is
provided. The registry rejects missing or inconsistent capability policies.

The raw classifier is public for pure policy testing; production integration must
use the registry and its authoritative capability inventory. The registry itself
is not an authorization service. Durable outcome and compensation-lineage storage,
startup-barrier integration and provider-specific observation verification remain
separate implementation requirements (#130, #147 and #148).

## Captured-only replay

`NoProductionReplay` receives owned captures with an expected task/account scope.
It verifies supported schema, ordered unique sequence, content hashes, count,
per-entry and aggregate byte budgets before admitting each record. Hard ceilings
are 4,096 entries, 1 MiB per entry, 16 MiB aggregate and 60 seconds. Expiry releases
the remaining buffers and is sticky. Iteration is bounded to the entry limit plus
one; arbitrary time/allocation inside a caller-supplied iterator is outside that
admission guarantee. Time is checked between operations, not by interrupting the
caller or the operating system.

A step returns `ReplayedRecord`, explicitly labelled captured data, not a live
observation or sendable action. There is no executor callback, live credential or
network endpoint to accidentally invoke. Action-looking text remains inert bytes.
Debug output omits captured content and provider session references. Callers must
not feed replay output into a production-effect executor; wiring that isolation
through real worker/provider contexts is still a separate acceptance gate.

## Provider session policy

Provider sessions are secondary references. Read-only resume requires the same
account, provider and immutable checkpoint hash, explicit resume support, an
unexpired session and current authority/fresh source facts. Missing, expired,
mismatched or rejected sessions reseed from the authoritative durable checkpoint,
not from a model-generated reconstruction. Writes never resume through this path.
Four attempts per incarnation exhaust the automatic recovery budget; the caller
must persist/update that counter and must not reset it after each failed call.

This library does not load credentials, invoke provider SDKs, decide that a new
approval exists or start a new session itself. Product/provider adapter integration
and user-visible recovery action wiring are not claimed by these policy tests.

## Verification

Tests enumerate the stage/effect/authority/source/attempt/deadline cross-product,
verify known-commit precedence and compensation identity, mutate every binding
component, check idempotency expiry, reject unsupported wire fields and incomplete
capability inventories, and exercise replay limits/corruption/scope/expiry and
provider-account/checkpoint/retry mismatches. Compilation and tests are required
on the whole integrated workspace, not only this crate.
