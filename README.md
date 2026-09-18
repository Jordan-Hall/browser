# Intent Browser

A local-first browser workspace built around durable tasks, source data and explicit permissions.

The workspace, not the conversation, is the main product object. It should keep its files, task history and source references when a conversation ends, a worker crashes or the user changes model providers. Navigation, filtering and other ordinary interactions should work without inference.

Rust owns the runtime, state and authorization boundaries. Browser engines, connectors, model runtimes and external agents run behind typed interfaces. They can propose work; they do not get to approve their own actions.

## Current state

This is an early implementation, not a released browser. CORE work is being developed in stacked pull requests; the default branch does not yet contain the runnable runtime. The current implementations cover contracts, bounded IPC, durable state, artifacts, snapshots, worker supervision, checkpoints and recovery. Integration and release qualification are still in progress.

Start with the [implementation programme](https://github.com/Jordan-Hall/browser/issues/1) and [CORE epic](https://github.com/Jordan-Hall/browser/issues/13). The [open pull requests](https://github.com/Jordan-Hall/browser/pulls) show what is actually proposed. A passing build is not a claim that the browser is ready for daily use.

## Project documents

The [document guide](docs/README.md) explains where to find the roadmap, requirements and task discussions.

| Document | Use it for |
| --- | --- |
| [Issue register](docs/project-archive/ISSUE_REGISTER.md) | Finding an issue by number, title or workstream. |
| [Workstream library](docs/project-archive/README.md) | Reading an epic with its requirements, tasks and comments. |
| [All issues and epics](docs/project-archive/ALL_ISSUES_AND_EPICS.md) | Loading the complete backlog into a project or searching it locally. |
| [Pull request register](docs/project-archive/PULL_REQUEST_REGISTER.md) | Looking up implementation references captured with the archive. |

The archive is a dated copy of GitHub, not a second live tracker. Keep issue numbers and stable task IDs when discussing changes. Check the live issue and PR before relying on a status quoted in an older comment.

## Runtime layout

The CORE implementation branches use the following crates:

| Crate | Responsibility |
| --- | --- |
| `intent-contracts` | Typed identities, values and versioned domain records. |
| `intent-ipc`, `intent-local-transport` | Bounded messages, protocol negotiation and local peer authentication. |
| `intent-state` | SQLite state, operations, journals, inbox/outbox, artifacts, checkpoints and snapshots. |
| `intent-supervisor` | Worker launch, readiness, resource admission, cancellation and restart policy. |
| `intent-recovery`, `intent-broker` | Recovery decisions and the boundary between approved durable work and worker dispatch. |
| `intent-fixture-worker`, `intent-conformance`, `intent-xtask` | Process fixtures, contract checks and dependency rules. |

The implemented process and snapshot paths currently target Linux. Cooperative workers, per-process limits and admission estimates are not substitutes for a hostile-process sandbox or aggregate resource containment. Other platforms need their own implementation and test evidence.

## Working on CORE

Check out the branch of the task PR you are changing. Each PR is based on the preceding implementation branch, so check its base before rebasing or opening a follow-up. Do not copy a later migration or API into an earlier stage just to make its build pass.

On a CORE branch with `Cargo.toml`, use the toolchain pinned in `rust-toolchain.toml` and keep both the workspace and fuzz lockfiles committed:

```sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-targets --no-fail-fast
cargo test --locked --workspace --doc --no-fail-fast
cargo check --locked --manifest-path fuzz/Cargo.toml --bins
cargo run --locked -p intent-xtask -- arch-check
cargo run --locked -p intent-conformance --quiet
python3 scripts/check_core_coverage.py
```

The fuzz command above only compiles the targets. It does not run a fuzz campaign. The coverage check validates the task ledger; it does not accept the tasks it lists.

For a release assessment, also run:

```sh
python3 scripts/check_core_coverage.py --require-accepted
```

That check is expected to fail while required acceptance evidence is missing. Fix the implementation or supply the required evidence rather than weakening the check.

## Before opening a pull request

Reference the individual task and its parent requirement. Include the behavior changed, a regression test, the commands actually run and any compatibility or migration impact. Keep changes to the trusted runtime small enough to review.

For external writes, persist the attempt before dispatch and retain uncertainty when the result is unknown. A timeout is not proof that nothing happened. Restoring a local snapshot does not roll back an external service, and a worker acknowledgement is not a verified provider receipt.

Use fixture accounts and independent effect ledgers for destructive tests. Do not put production credentials, personal browsing data or live provider captures in tests or committed documents.
