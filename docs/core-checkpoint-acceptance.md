# Consistent workspace checkpoints — CORE-04.T02

Programme #1; CORE epic #13; parent #5; individual task #146. Storage and integration dependencies: #129, #130, #133, #134, #135, #136, #145, #147, #151, #152, #211, #212, #213, #214. This is source implementation and scoped verification, not acceptance of the entire CORE epic.

## Authoritative state and publication boundary

`save_workspace_graph` persists the existing `Workspace`, `GoalContract` and `Task` domain records inside a bounded versioned graph container. Graph revisions belong to this repository; expected-revision compare-and-swap rejects racing writers. A task's workspace association is permanent even after graph retirement. Removing a task with durable operation history is rejected rather than hiding its history.

`checkpoint_workspace` reads that committed graph revision, every operation belonging to its active task nodes, each declared consumer cursor, provider reference fingerprints and historical worker instances under one immediate SQLite writer transaction. It verifies the operation projection against the journal tail and contiguous revision inventory. The database transaction also covers artifact validation, exact checkpoint pins and snapshot publication. Publication/reference/hold/GC writers use this same database boundary. No external service is contacted.

The graph must be acyclic, have distinct correctly scoped task/goal identities and valid edges, and resolve all declared local dependencies. Reference hashes and sizes must equal actual metadata and file bytes. File verification uses descriptor-relative no-follow reads under a private Linux/GNU root. An artifact hash or an `ArtifactScope` argument is not a grant: callers must already be trusted and authorized for that scope.

Checkpoints are immutable. Retrying the same ID and request returns the original snapshot; changing its workspace/revision/time conflicts. Serialization is byte-bounded while writing; stored bytes must match their digest and canonical typed encoding. Unknown fields, unsupported schemas, missing pins, registry mismatches and corrupt projection/journal state are rejected. Source text is redacted from graph/checkpoint Debug output.

## Retention, restore and privacy

Checkpoint pins integrate with existing artifact references, so all existing GC selection and final-eligibility checks protect the referenced blob. SQL guards prevent callers from dropping a pin while the checkpoint still exists or registering arbitrary immortal pins in the reserved checkpoint namespace. `release_checkpoint` compares the expected digest and removes exactly that checkpoint plus its pins atomically. Other live references and holds remain effective.

Suppression still removes normal access immediately. A checkpoint does not bypass suppression: loading it fails with an explicit suppressed dependency while its retained bytes remain protected. The existing authenticated plaintext backup/restore path preserves graph, checkpoints and pins. Restoring a backup does not enable dispatch, revive a historical runtime epoch or imply rollback of any external effect.

Provider fields contain fingerprints referencing a separate trusted session store, not bearer credentials. There are no credential or OS-handle fields in the checkpoint schema. Free-form user/domain text is still sensitive and can itself contain secrets; this feature is not a content-redaction or encrypted-storage system.

## Admission limits and tradeoffs

Graphs admit at most 256 tasks, 2,048 dependency edges, 64 goals/cursors/provider references, 256 historical worker instances, and 512 declared artifact references. Graph serialization is capped at 1 MiB; the complete checkpoint at 4 MiB. Each snapshot admits at most 512 operation projections, 4,096 journal revisions per operation, and 256 MiB of distinct referenced blob bytes. Exceeding a limit is explicit failure, never truncation. Callers must handle these limits and SQLite busy errors.

Artifact hashing holds the database writer boundary. This favors a straightforward consistent snapshot over concurrent write throughput; it is not a preemptible disk deadline and must run away from the UI/control executor. The root must be exclusively assigned to the same trusted profile/database. This PR does not establish profile ownership or hostile same-user containment, and it does not qualify other operating systems/filesystems.

## Executed regression coverage

The checkpoint test module exercises consistent graph/operation/cursor capture and reopen, graph CAS races, permanent task ownership, cycles/duplicates/foreign nodes, preservation of operation history, idempotent checkpoint retries, pin protection during suppression/GC, explicit digest-checked release, corrupt/missing blobs, wrong hashes, missing cursors, transaction rollback, competing-writer exclusion, graph/journal corruption, immutable historical snapshots, reserved-reference guards, and Debug redaction.

An actual child process is killed after inserting checkpoint/pins but before committing. Reopen observes neither partial checkpoint nor orphan pins; retry succeeds. A complete authenticated snapshot export/restore preserves checkpoint dependencies while retaining the disabled-dispatch barrier. These tests are subprocess and local filesystem evidence, not abrupt VM/power-loss qualification.

## Remaining task acceptance

Independent review of the committed implementation and its declared limits remains required. Live broker/authority revalidation, durable startup recovery activation, complete original/compensation lineage, provider-session resolution, the native task-centre UI and platform/power-loss qualification are separate unfinished gates. Checkpoint reads return historical facts; `grants_execution_authority()` is always false. No completion flag or review record is fabricated.

SQLite's writer and foreign-key action semantics used here are documented at https://sqlite.org/lang_transaction.html and https://sqlite.org/foreignkeys.html. In particular, the parent row is removed before its cascade actions, which allows explicit checkpoint release to remove pins while direct pin deletion remains guarded.
