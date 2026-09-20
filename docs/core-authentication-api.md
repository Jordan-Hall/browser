# Issued worker verifiers

Worker authentication now issues a random wire token together with a move-only
verifier. The supervisor creates one pair for each lane before spawn, binds each
verifier to the actual child's process afterward, and then delivers bootstrap.
There is no public constructor that turns a worker token back into a verifier.

The public API accepts a live Unix connection or Windows named-pipe handle.
Transport code observes the connected process itself. Caller-created peer
evidence, Synthetic peers, AnyLocal policy and the old launch-record constructor
are removed. ExpectedPeer remains public launcher policy; it is not an observed
credential. The Windows client can separately verify its expected server PID
without receiving a reusable observed-peer value.

The non-consuming peer check runs before the supervisor reads Hello. A foreign
process is disconnected without using the intended child's credential. A
matching peer's Hello consumes the verifier on success or schema, token, role or
generation failure. A subsequent authentication attempt returns AlreadyConsumed.
The connected authentication method repeats peer observation before consuming
the Hello, even if a caller already performed the preflight check.

The supervisor still owns its private socket, decoded frame, identity, channel
and generation in Lane. The API does not prove the origin of a caller-supplied
decoded Hello by Rust types alone, and a returned identity cannot be injected
into Supervisor or converted into a WorkerLease. The existing absolute deadline,
both-lane readiness and revocation checks remain the authority boundary.

WorkerHello and BootstrapToken retain their existing v1 wire representation.
The Rust API is intentionally contracted, with all workspace callers migrated.
The synthetic authentication conformance entry is removed. Portable role policy
checks and actual-worker authentication evidence remain separate.

The Windows fixture delivers random bootstrap on child stdin, rejects a foreign
process while preserving the intended verifier, then admits the real child.
Separate cases reject wrong tokens, roles and generations, and consumed verifiers
reject replay. The Unix fixture additionally leaves a foreign connector silent,
requiring rejection before Hello and subsequent owner admission and scoped work.

The Windows fixture disconnects the rejected client before reconnecting the pipe,
following the [Windows named-pipe connection contract](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-connectnamedpipe).
Its default security descriptor is still fixture configuration, not qualified
production endpoint ACL or principal enforcement.

Secret erasure, full Windows/macOS supervisor authentication and independent
whole-task acceptance remain open. Hostile same-user containment belongs to
SEC-04. This API contraction addresses misleading public authority construction;
it does not claim a newly demonstrated remote bypass of the existing supervisor.

Local verification passed strict workspace Clippy, 106 passing Linux test
results, the native Windows real-pipe cases, seven compile-fail API examples
on each platform, architecture checks and 18 portable conformance checks.
Removing only the pre-Hello peer filter made the silent foreign-peer case fail
with HandshakeTimeout; restoring the exact runtime made it pass.
See [the scoped verification report](verification/core-authentication-api-20260920.json).
Published exact-source CI remains a separate gate.
