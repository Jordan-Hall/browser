# Durable recovery integration

Programme #1; CORE epic #13; requirements #3, #4 and #5. This branch builds on the existing checkpoint implementation in #806, rather than introducing a competing checkpoint schema or migration 008.

Implementation targets: startup recovery before dispatch (#147), exact-attempt read-only reconciliation and original/compensation lineage (#148/#130), durable authorization and worker-generation fencing (#131/#141/#212), and recovery view/actions (#151). Checkpoints remain owned by #806/#146, and pure recovery policy remains owned by #804/#145.

Acceptance must demonstrate persisted attempt identity, current source/approval checks, startup fencing, explicit unknown outcomes, idempotent evidence application, stale-action rejection, and recovery without blindly repeating a possibly accepted external effect. A historical checkpoint, process acknowledgement, document or smoke-test result is not execution authority or a provider receipt.

This draft contains the integration contract first. Source and executed regression evidence are added in this implementation pass. It is not a production-readiness declaration, and no task is auto-closed or PR merged by this work.
