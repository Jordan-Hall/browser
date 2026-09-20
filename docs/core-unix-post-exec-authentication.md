# Unix peer authentication after exec

The local transport integration test launches its own test executable, sends a
private bootstrap over stdin and accepts a fresh connection made by the child
after exec. The parent retains the child and connection until authentication
and a bounded acknowledgement finish.

The intended child's PID, effective UID and effective GID must match. A silent
connection from the parent is rejected before reading Hello and leaves the
verifier available for the child. Wrong UID and GID expectations are rejected
on the real child connection. Wrong token, role and instance consume the
credential, and subsequent reuse fails. Each case checks successful child exit;
failure paths kill and reap the child and remove the private test endpoint.

Accepted streams explicitly use blocking mode with a decreasing deadline for
each read and write. This avoids depending on whether an OS inherits the
listener's nonblocking flag.

Linux local runs pass all four parent cases and their four child executions.
Disabling the peer comparison makes the foreign-connection case fail; restoring
the exact source makes all cases pass. Native macOS verification remains pending.
See [the scoped evidence](verification/core-unix-post-exec-authentication-20260920.json).

This qualifies the connected-peer and issued-verifier primitives only. The
fixture exchange and temporary endpoint are not production framing or a native
Supervisor. Full Windows/macOS supervision and CORE-01.T05 acceptance remain open.
