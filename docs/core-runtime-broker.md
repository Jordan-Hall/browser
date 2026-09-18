# Runtime broker integration

Programme #1; CORE epic #13; parent requirements #2, #3, #4 and #5. This implementation is stacked above durable recovery #807, checkpoints #806 and actual-process supervision #805.

The broker joins the authoritative state owner and an authenticated supervisor lease. It commits exact approved attempt identity before exposing bytes, then resamples time and performs the final atomic worker admission before invoking a trusted conditional-effect adapter. Revocation before that admission prevents the adapter call. Revocation after admission preserves an in-flight uncertain effect; it cannot claim external rollback.

The acceptance suite uses real authenticated worker processes and a separate provider process with its own SQLite database. It measures actual submissions as well as committed effects, so idempotent fixture behavior cannot hide a blind resend. Four broker-process kill boundaries cover before start, after durable start, after independent provider acceptance and after transport observation. Read-only signed evidence, not a worker acknowledgement or missing provider row, determines verified completion.

The current implementation pass adds the broker, clock fencing, immutable adapter contracts, bounded registries and these executable regressions. The initial contract-only commit is not source verification. Final source and exact CI evidence will be recorded on the PR before any completion claim.

The Linux cooperating-profile boundary remains explicit. Trusted adapters must enforce source preconditions atomically in their provider and bound I/O; arbitrary adapter code is part of the trusted computing base. Production provider/key custody, native recovery UI, credential-isolated replay, hostile-process and aggregate resource containment, instrumented fuzzing, platform/power-loss qualification and independent acceptance are not established by this contract.

No automatic issue closure, approval or merge. No snapshot, plan, process result or content hash becomes execution authority.
