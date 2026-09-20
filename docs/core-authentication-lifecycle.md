# Authentication lifecycle process cases

These Linux fixture cases exercise existing supervisor behavior after the
startup-deadline change. They add executable evidence for CORE-01.T05; they do
not change the production supervisor or accept the whole task.

| Case | Causal setup | Required observation |
|---|---|---|
| Revoked startup | Parent cancels while the actual child is gated before control Hello, progress Hello or Ready, then releases the valid message. | No lease, no return to Ready, original cancellation retained and child reaped. |
| Handshake disconnect | Child receives control Welcome, waits for the parent's gate, closes that socket and stays alive without opening progress. | ControlClosed stops the startup, no lease is issued and the child is reaped. |
| Foreign peer then owner | Intended child forwards its real bootstrap to another process, waits for that process to fail, then connects itself. | A rejected peer is counted, the intended PID becomes Ready with the same generation, and scoped work succeeds. |
| Foreign lifecycle request | An authenticated worker sends a correctly framed Cancel naming an independent Ready generation. | Sender fails with ProtocolViolation; the other lease remains live and completes scoped work. |
| Foreign heartbeat | An authenticated worker sends a valid first Heartbeat sequence naming an independent generation. | Sender fails with ProtocolViolation; the other generation still completes scoped work. |

The gate publishes its PID atomically and marks a step completed only after
writing the full gated message or closing the socket. The parent waits for that
marker without polling, then observes the supervisor's decision. On-time
controls still exercise all three message gates.

The foreign-peer case checks more than rejection: a valid intended child must
still consume its own credentials afterward. Its marker records the foreign
PID, and the parent compares it with the supervisor's actual owned process.
The lifecycle case proves directional rejection of worker-originated Cancel.
The heartbeat case separately exercises generation binding on a permitted
worker message. Both require the surviving generation to remain usable and
complete scoped work.

After its single foreign message, the sender stays quiet until cleanup. This
prevents a later heartbeat with a repeated sequence from concealing a missing
generation check. Review identified that false-positive path in the first
heartbeat fixture before qualification.

The fixture subprocesses run under the same bounded supervisor cleanup used by
the existing tests. Terminal cases retire their generations. Dropping the
supervisor also kills and reaps owned children on a test error.

The final Linux run passed six startup tests, 24 supervision tests and 185 state
tests and doctests after merging main `db8fd87`. Removing the peer prefilter
prevented the intended owner from becoming Ready. Removing only the heartbeat
generation predicate produced HeartbeatTimeout instead of ProtocolViolation.
Both cases passed when production was restored. An independent source and
evidence review verified the recorded blobs, log hashes and failure causes.
See [the verification report](verification/core-authentication-lifecycle-20260920.json).

Public low-level authentication APIs, secret disposal and Windows/macOS
supervisor authentication still need implementation or qualification. Hostile
same-user containment remains SEC-04 work. None of these cases treats a portable
job with Linux-only tests disabled as authentication evidence for that platform.
