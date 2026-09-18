# Branch consolidation — 18 September 2026

The default branch is `main`; no `master` branch existed. The owner requested consolidation of all implementation branches, including incomplete work.

## Checkpoint conflict resolution

PR #806 and PR #808 contain different, useful checkpoint APIs, not duplicate implementations. Both remain compiled and tested: existing task checkpoints keep their root API and `checkpoints` module; workspace checkpoints are exposed under `intent_state::workspace_checkpoints`, with `WorkspaceCheckpointRequest`, `WorkspaceCheckpointId` and `WorkspaceCheckpointError` aliases. Neither implementation or its regression suite was discarded.

Canonical migrations 001–008 are unchanged, including the task-checkpoint migration already on main. Workspace checkpoints are appended as migration 009 and durable recovery as migration 010. Task and workspace pins use separate reference kinds. Databases created on the formerly unmerged workspace-checkpoint branch have a divergent migration-008 checksum and are intentionally rejected, not silently reinterpreted; retain those original databases until an explicit export/import migration is designed.

Record validation, broker dispatch, durable recovery, supervision and existing documentation are retained. Implementation evidence from both stacks is preserved; no task is promoted to accepted. Obsolete checksum-transport payloads/workflows are removed from the working tree only after their source branches are incorporated into ancestry. They remain recoverable from Git history.

## Preserved branch tips

Every tip below was verified as an ancestor of this candidate before publication. Branch deletion is a separate, expected-SHA-guarded step after the candidate has passed CI and reached main.

| Branch | Original tip |
| --- | --- |
| `core-01-record-validation-20260918` | `317d7dafe2002effe66cf4bdc48ab99d91cceb22` |
| `core-01-t01-bootstrap` | `57a4c8aede0dcb0412eec5679802de0de8e3a17a` |
| `core-01-t02-typed-values` | `e6bdd8204ca8d167e7b30e1e0e6d40a655e70000` |
| `core-01-t03-record-schemas` | `79a3dfb815e75eac268b4621ce80add720497f33` |
| `core-01-t04-wire-framing` | `c8197e8afa9398fed6d9e854ed2eac49a9056d57` |
| `core-01-t05-worker-auth` | `9a2b811051aacb07213c2e405ee48346b5be44fe` |
| `core-01-t06-version-negotiation` | `5341a563ebf67d20ddd56505230d2c192ae0f46b` |
| `core-01-t07-cancel-backpressure` | `9cc67277c7e4aa4b21e9ede4bd59f099543d27d2` |
| `core-01-t08-conformance` | `5c26d585a63c0967f2bc24f990dbc3ec56b74360` |
| `core-02-t01-state-db` | `381ea7903f7c055b7939ecbc92e3f14669e05f03` |
| `core-02-t02-operation-journal` | `cb3856645ba460e1d3102ac4adc58199897ffd15` |
| `core-02-t03-outbox` | `9225ab3ec4c64202b4bcc1b013a6bea61977aa13` |
| `core-02-t04-inbox` | `87e0872da7290dd5a057526912dd3a88e4120816` |
| `core-02-t05-artifacts` | `e2409171ba94979fe007d69db2ceae7934a2405a` |
| `core-02-t06-retention` | `96e66e0b977b3ad4c716368b0baa430a266cb686` |
| `core-02-t07-authenticated-snapshots` | `4fface918d13be0f636bd7b0a3716aedb9b08eb4` |
| `core-03-supervision-acceptance-20260918` | `716474380d801166f0911c5c8009793da26be0d5` |
| `core-03-supervisor-runtime` | `4240339a7619f3a9cd7aee4cab3fd01ca74f3b8a` |
| `core-04-durable-recovery-integration` | `5cdcd5ef71940cd3bcc129cbd02ac68c20aa7ce7` |
| `core-04-recovery-policy-and-replay` | `e69ec1da33c27059f344857810bf946572fe01c3` |
| `core-04-t02-checkpoints-20260918` | `039ec91d08722ea9bc8f255f9b4e1202fbd11796` |
| `core-04-t02-consistent-checkpoints` | `845e808324b4c6f189e0ad880c5b164e43215376` |
| `core-broker-supervisor-dispatch-20260918` | `9bf2850b8aed80f242f551fd46efdb836cfe1410` |
| `core-runtime-broker-acceptance` | `8d1d80e2ebaaf323205a75edb418b87164f0ff89` |
| `core/production-hardening-20260917` | `84d2dd0512fb8e6ae02e4b41562676f4b5e8a215` |
| `docs/project-issue-archive-20260918` | `e560d398306588b085dd533da28c79dddc007829` |
| `main` | `93bc1291457f3a3bc8250be76d5bda4d3e1f589b` |
| `maintenance/consolidate-20260918` | `c06c4e80750b768c337d482bee56a019c56d2c91` |
