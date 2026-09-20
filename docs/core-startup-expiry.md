# Worker startup admission deadline

The Linux supervisor gives each launched generation one monotonic startup
deadline. It starts immediately before generating the two bootstrap tokens.
Process spawn and delivery of the bootstrap packet consume this interval;
executable loading and sealing precede it. A blocked launch call itself is not
bounded by this change. Its next admission decision rejects an expired startup.

Both channel authentications and Ready must be admitted strictly before that
same deadline. Neither a successful control handshake nor a progress handshake
extends it. A queued message has no trusted arrival timestamp; the supervisor
uses the time of admission, not the worker's claimed send time.

The lane checks fresh time before taking a one-use authenticator and before
publishing authenticated state. The Ready transition checks fresh time again.
Expiry records HandshakeTimeout, revokes the generation's lease, discards unused
authenticators, and uses the existing cancellation, TERM/KILL and reap path.
Credential disposal here does not establish memory erasure.

Startup expiry applies even when a shared read budget prevents service. An
expired silent startup is also stopped. Workers already admitted as Ready keep
the existing budget-sensitive heartbeat and progress policy, lane byte limits
and fair read cursor. Startup timeout cannot revoke an established session.

## Regression evidence

The real fixture can pause before the control Hello, progress Hello or Ready.
Its parent observes a marker written by the launched process, releases it, and
waits for the complete gated message to be written before polling again. This
prevents early timeout cleanup from concealing late message admission.

Before the fix, a worker that had received both Welcome messages could send
Ready after expiry and reach Ready with no failure. The same fixture released
on time passed. The initial corrected source rejects that late Ready and still
admits the on-time control.

The expanded tests cover late first Hello, an on-time control Welcome followed
by late progress Hello, and all three on-time controls. Each negative requires
HandshakeTimeout, permanent lease refusal and bounded child reaping. Private
boundary tests cover exact expiry, a stale outer poll timestamp, a silent
startup with zero read budget, and leaving an expired lane credential unconsumed
until its owner disposes of the stopped generation.

The first expanded fixture run exposed a marker race: the parent could read an
empty PID file between creation and writing. The fixture now publishes that
marker by rename after writing. The original failure log is retained with the
verification report.

Review also found a clock mismatch in the first patch. Comparing fresh health
ages with heartbeat/progress values stamped by the outer poll clock could time
out a just-serviced Ready worker. Two private regressions reproduced that false
timeout. Health observations and age calculations now retain the same existing
poll clock; only startup admission, passive startup expiry and stop escalation
use fresh time. Ready initializes its health times at admission, and subsequent
same-poll observations cannot move them backwards.

## Remaining task scope

This change does not accept CORE-01.T05. Public transport authentication APIs,
secret erasure, additional revoked/disconnected/foreign lifecycle cases and
Windows/macOS launch qualification remain separate work. Linux PID/UID/GID and
private-endpoint checks do not provide hostile same-user containment.
