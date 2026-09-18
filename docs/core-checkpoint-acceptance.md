# Consistent checkpoint acceptance

Programme #1; CORE epic #13; requirement #5; primary task #146 (CORE-04.T02). This PR is stacked above #805, #804 and #803 and contributes to #136, #147, #152 and #213 without declaring those broader tasks complete.

## Implementation contract

A checkpoint contains one workspace graph revision, typed tasks and goals, dependency edges, the exact operation projections and consumer cursor positions observed in the writer transaction, a worker epoch, non-secret provider registry references and explicit artifact references. It is an immutable historical snapshot, not authority to dispatch or a rollback of external effects.

Publication compares the previous checkpoint head and graph revision. Graph validation, operation/cursor capture, artifact verification, retention roots and head publication share one SQLite immediate transaction. Immutable checkpoint roots cannot be removed through the generic artifact-reference API. Only superseded checkpoints can be retired; retirement releases their roots transactionally.

Linux/GNU artifact verification uses descriptor-relative no-follow traversal from a private root, exact size/hash checks and an aggregate artifact budget. The checkpoint payload is limited during serialization to 1 MiB; tasks, edges, provider references, cursors and operations have explicit count limits. Unknown schema versions, invalid graphs, missing dependencies, scope mismatches and suppressed content are not silently accepted.

## Executable acceptance scenarios

The source change includes tests for stable snapshot capture without mutating live state; exact-request idempotency; stale/reused identities; competing publishers; retention-root protection; suppressed content; missing/corrupt/wrong-scope artifacts; graph cycles and duplicates; missing cursors; payload/count budgets; symlink rejection; transactional rollback on a failed pin; authenticated snapshot round-trip; and abrupt child-process exit after checkpoint commit.

The new source must receive its own successful CI run after publication. An initial documentation/bootstrap run is not evidence that these scenarios ran.

## Remaining integration and operating limits

A checkpoint does not reopen the dispatch gate, establish current grants, restore a provider's private session internals or replace current operation history. Startup planning must reconcile current durable outcomes rather than restoring old operation projections over newer facts. The task-centre, provider adapters, runtime owner and end-to-end recovery coordinator remain separate integrations. Private graph text can contain sensitive user data; no explicit credential or live OS-handle field is accepted, and diagnostic output omits graph content.

The writer is held during bounded artifact verification; storage stalls are not preemptible disk deadlines. Process-kill tests are not VM/physical-power-loss or cross-platform qualification. The source, checks actually run and remaining acceptance are recorded in the PR rather than inferred from task references.
