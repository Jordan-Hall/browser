# Linux worker-supervision candidate: source and acceptance evidence

Programme #1; CORE epic #13; requirements #2 and #4. Source baseline: PR #804,
`5c6bbb4cc7af28fe1a20d0921409749bc74f2487`, above snapshot PR #803 and tracking
PR #802. The downloaded, successful PR #804 CI source tree was compared with
that head and contains no source difference.

**Publication: PR #805. Acceptance: partial. Production ready: no.**
The source is published at `3bfdbc0f679e37368f5d8e04975390bb4d862295`.
[CI run 35336466160](https://github.com/Jordan-Hall/browser/actions/runs/35336466160)
executed 177 runtime tests, 5 doctests and 15 ledger tests, including twenty
real-process scenarios. Formatting, Clippy, locked dependency checks, actual
architecture checking, excluded fuzz-target compilation and smoke conformance
also ran successfully. These are scoped verification results, not whole-CORE
acceptance. Temporary import files/workflows are absent from the final tree.

## Implemented process boundary

`intent-supervisor` owns each real child, immutable configuration, fresh worker
instance, independent control/progress sockets, one-use authenticators, selected
v1 codec, lease, bounded request state, resource admission and restart budget.
The fixture worker is a real executable using the same transport/client module,
not an in-memory endpoint simulation.

The approved ELF is copied through a bounded streaming hash into a sealed memfd.
The sealed image is executed through its retained descriptor; replacing the
original path afterward cannot substitute a different executable. Each launch
receives a new instance UUID and two distinct bootstrap secrets. Bootstrap is
bounded and delivered over stdin, not arguments or the environment. The private
0700 socket namespace is opened descriptor-relatively without following path
symlinks. Accepted post-spawn Unix connections must have the kernel PID, UID and
GID of the actual owned child. A descendant given the genuine token is rejected
because it has a different kernel peer PID. Identity cannot be supplied as a
caller-deserialized credential object at this live authorization boundary.

Both channels must complete their one-use handshake before readiness. Every
subsequent frame uses the selected, implemented v1 codec and existing strict
bounded JSON/frame parsing. Separate progress transport and per-poll count/byte
budgets prevent a progress flood from consuming the control reader's allowance.
Output, queued work, pending requests and retained observations are independently
bounded. Transport, control messages and encoded work redact content in Debug.

The child environment is cleared. Non-stdio descriptors, including descriptors
left inheritable by an embedding process, are marked close-on-exec by the child
setup hook. stdout/stderr go to the null device so fixture logging cannot block
the control path. Linux address-space, CPU-time, descriptor and core-file hard
limits, NO_NEW_PRIVS and parent-death signalling are installed before exec.
Unsupported close_range/sealing/limit operations fail launch rather than being
ignored. The small unsafe pre-exec hook performs syscall-only setup, not Rust
allocation, locking, environment manipulation or logging.

## Lifecycle, requests and resource accounting

Readiness, missed heartbeat, stalled correlated work, deadline expiry, protocol
violation, channel loss and OS exit remain distinct. Heartbeats alone cannot
complete a request. Replies must match the exact generation, request and admitted
sequence. Work observations are not provider receipts or proof of external
completion. Expired queued work is not sent, and late responses after revocation
do not discharge outstanding requests.

Lease clones share one atomic admission/revocation cell. Scope, role, capability
and deadline checks precede issuance of a move-only request permit. Revocation
permanently prevents later admissions and happens before worker notification.
Cancellation never needs work-queue capacity. A partially written frame is not
spliced into a cancel frame: the separate grace/TERM/KILL path remains available.
Unresponsive, paused or protocol-failing children cannot postpone the configured
signal escalation merely by failing to cooperate with the wire protocol.

The supervisor keeps terminal entries until explicit retirement. Retirement is
permitted only after reaping and returns unresolved requests plus observations;
stale leases stay revoked after the entry disappears. Restart preserves the
original lifetime and bounded backoff/count budget, requires a new handshake,
and refuses outstanding requests/observations. It does not replay their work.

Worker-count, declared memory and CPU admission have separate interactive/speech
reservations. Work queues separately bound class counts and serialized bytes,
round-robin foreground/speech, allow an aged background slot, and skip blocked
workers instead of letting one head entry block another worker. An empty speech
queue does not accidentally promote unaged background work over interactive work.
Metrics sample at most one bounded /proc record per poll and become unavailable
when stale. Yield requests are coalesced; their acknowledgement records only a
cooperative protocol acknowledgement, not real GPU memory release.

## Task-specific evidence and remaining acceptance

The executable scenarios are in
`crates/intent-fixture-worker/tests/supervision.rs`; parser, lease, admission and
restart regressions also live next to their implementation. Every test starts
its own bounded fixtures and owns cleanup. The delivered verification manifest
records the exact local source tree, commands, test totals and logs.

| Tasks | Implemented and exercised here | Still required for task acceptance |
|---|---|---|
| #125 CORE-01.T05; #137 CORE-03.T01 | Real post-spawn PID binding; wrong child/token/role/generation and replay rejection; sealed executable replacement regression; immutable scoped launch config. | Independent threat-model/ownership review and production launch integration; hostile-process containment and other platforms are not supplied. |
| #126 CORE-01.T06 | Handshake-owned implemented v1 codec on both actual channels. | Durable migration output validation and complete record-family/version fixtures; no fictional support for additional versions. |
| #127 CORE-01.T07; #138 CORE-03.T02 | Actual readiness, scoped request/response, deadlines, distinct heartbeat/progress/OS failures; bounded transport and terminal retirement. | Accepted production broker integration and all lifecycle/suspend/descendant conditions. |
| #139 CORE-03.T03 | Count/byte queue bounds; reservations; foreground/speech fairness; aged background service and blocked-worker skipping. | Aggregate enforced resource controls, genuine native-shell/speech load and service-level latency evidence. |
| #140 CORE-03.T04 | Actual AS allocation rejection, file-descriptor exhaustion, CPU-hog kernel termination, NO_NEW_PRIVS and inherited-descriptor rejection. | cgroup/container/VM containment, descendants that escape process groups, aggregate CPU/RSS/PID constraints, GPU enforcement and platform support. |
| #141 CORE-03.T05 | Shared revocation order; two saturated workers cancelled while an independent worker remains ready; late response rejection; ignored cancel escalation; SIGSTOP/SIGCONT lease invalidation. | The actual external sender/outbox must share a durable fencing/authorization boundary. Already-admitted uncertain effects need exact-attempt reconciliation. |
| #142 CORE-03.T06 | Fresh epochs, permanent old-lease revocation, bounded restart/backoff/circuit and original lifetime; no pending-work replay. | Durable checkpoint and restart-budget persistence plus visible product degraded mode. |
| #143 CORE-03.T07 | Content-free bounded/stale-aware process samples; coalesced yield signalling. | Accelerator observations, real unload/KV eviction integration and foreground/speech measurements under realistic model load. |
| #144 CORE-03.T08 | Real process churn beyond registry lifetime capacity, concurrent admission/revoke, saturation, malformed replies, paused worker, cooperative descendants and cleanup. | Hostile descendant escape, real supervisor restart around external dispatch, whole-system suspend/resume, VM/power-loss and other platforms. |
| #113; #211; #212; #213; #214 | Traceability to this source and explicit operating limits. | Complete accepted ownership inventory, durable dispatch/recovery integration, independent external-effect ledger and operational readiness. |

## Qualification limits that remain release blockers

This is an **approved/cooperative local-worker** boundary, not a sandbox.
`ExecutionBoundary::UnattendedUntrusted` is explicitly rejected before launch.
Same-user hostile processes, root/capability separation, network/filesystem
isolation, loader/shared-library tampering and adversarial descriptor passing
are not made safe by a bootstrap token or SO_PEERCRED. The kernel and dynamic
loader/libraries remain trusted. Supplementary groups and privileged host
credentials are not stripped into a separate sandbox identity. Do not load
arbitrary plugins/models as executable code under the cooperative profile.

Linux rlimits are per-process controls: virtual address space is not aggregate
RSS; CPU seconds are not a scheduler quota; descendants can fork and change
process groups. Admission numbers are estimates, not kernel-wide enforcement.
Supervisor image buffers, shared memory, GPU resources and escaped descendants
are not fully accounted by these reservations. The process group cleanup test
uses a cooperative child; it is not evidence of hostile tree confinement.

Worker-request admission is distinct from external-effect authorization. A
request admitted before revocation may be in flight. The sender does not hold an
atomic lock over a real provider call, and the library is not wired to the
SQLite operation/outbox broker, grant/source-precondition validators or recovery
activation. A channel already held by an authorized process can be delegated by
that process. Do not route observations or local request permits directly into
production writes. Original/compensation lineage and uncertain-effect handling
still require the durable recovery integration tracked elsewhere.

Polling budgets bound configured work, not kernel scheduling or hard real-time
latency. The single two-flood cancellation measurement in the evidence is a
local regression observation, not a percentile SLO or native UI benchmark.
Image copying and process spawn belong on a launch executor. Blocking storage
and backup must remain away from control polling. Application code must not
externally reap the owned children. Drop is final cleanup and may wait for an
uninterruptible child; it is not the latency-qualified stop API. Regular polling
must continue through revocation and reaping.

Linux 5.11-or-newer close_range-CLOEXEC semantics and memfd sealing are required;
this run only qualifies the recorded Linux container/kernel/toolchain. No
Windows/macOS implementation or actual machine power-loss qualification is
claimed. Fuzz targets are compile-checked, not an instrumented campaign.

The corrected 37-task ledger incorporates source contributions from existing
#803/#804 and distinguishes this unpublished supervisor. It accepts no task.
Four feature tasks still have no source implementation even in this candidate:
#146 consistent checkpoints, #148 uncertain-state reconciliation, #151 recovery
task-centre UI, and #152 restart/suspend/corrupt-state qualification. Other rows
contain incomplete implementation/integration/qualification, not only paperwork.
The published repository still lacks this candidate's eight CORE-03 task
contributions until a source PR is actually opened and accepted.

## Reproduction

```sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-targets --no-fail-fast
cargo test --locked --workspace --doc --no-fail-fast
cargo test --locked -p intent-fixture-worker --test supervision -- --test-threads=4
cargo check --locked --manifest-path fuzz/Cargo.toml --bins
cargo run --locked -p intent-xtask -- arch-check
cargo run --locked -p intent-conformance --quiet
python3 -m unittest discover -s scripts/tests -v
python3 scripts/check_core_coverage.py
python3 scripts/check_core_coverage.py --require-accepted
```

The final command **must currently fail**: a consistent inventory is not a
passing release gate. The archive records this as expected rejection, not
successful production acceptance. No ignored tests are substituted for missing
platforms or missing implementations.

## Primary API references

- Rust CommandExt/pre_exec safety contract: https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html
- Linux post-connection peer credentials: https://man7.org/linux/man-pages/man7/unix.7.html
- Sealed anonymous executable image: https://man7.org/linux/man-pages/man2/memfd_create.2.html
- File seal semantics: https://man7.org/linux/man-pages/man2/F_GET_SEALS.2const.html
- Deferred descriptor closure: https://man7.org/linux/man-pages/man2/close_range.2.html
- Non-reaping child-exit observation: https://man7.org/linux/man-pages/man2/waitpid.2.html
