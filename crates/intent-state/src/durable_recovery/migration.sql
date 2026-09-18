CREATE TABLE recovery_authorities (
    account_id TEXT NOT NULL,
    capability_id TEXT NOT NULL,
    revision INTEGER NOT NULL CHECK(revision > 0),
    enabled INTEGER NOT NULL CHECK(enabled IN (0,1)),
    source_revision TEXT NOT NULL CHECK(length(source_revision) = 64),
    valid_until_micros INTEGER NOT NULL,
    evidence_key_id TEXT NOT NULL CHECK(length(evidence_key_id) = 64),
    effect TEXT NOT NULL CHECK(effect IN ('read_only','local_reversible','external_write','unknown')),
    PRIMARY KEY(account_id, capability_id)
) STRICT;
CREATE TABLE recovery_actions (
    operation_id TEXT PRIMARY KEY REFERENCES durable_operations(operation_id),
    privacy_scope TEXT NOT NULL CHECK(length(privacy_scope) BETWEEN 1 AND 128),
    effect TEXT NOT NULL CHECK(effect IN ('read_only','local_reversible','external_write','unknown')),
    source_revision TEXT NOT NULL CHECK(length(source_revision) = 64),
    destination TEXT NOT NULL CHECK(length(destination) BETWEEN 1 AND 512),
    message_kind TEXT NOT NULL CHECK(length(message_kind) BETWEEN 1 AND 128),
    payload BLOB NOT NULL CHECK(length(payload) <= 1048576),
    deadline_micros INTEGER NOT NULL,
    original_operation_id TEXT REFERENCES recovery_actions(operation_id),
    original_attempt_id TEXT,
    original_receipt TEXT,
    CHECK((original_operation_id IS NULL AND original_attempt_id IS NULL AND original_receipt IS NULL)
       OR (original_operation_id IS NOT NULL AND original_attempt_id IS NOT NULL AND length(original_receipt) = 64))
) STRICT;
CREATE TRIGGER recovery_action_immutable BEFORE UPDATE ON recovery_actions BEGIN
    SELECT RAISE(ABORT, 'recovery action identity is immutable');
END;
CREATE TABLE recovery_approvals (
    approval_id TEXT PRIMARY KEY CHECK(length(approval_id) = 36),
    operation_id TEXT NOT NULL REFERENCES recovery_actions(operation_id),
    runtime_epoch TEXT NOT NULL,
    authority_revision INTEGER NOT NULL CHECK(authority_revision > 0),
    approved_at_micros INTEGER NOT NULL,
    expires_at_micros INTEGER NOT NULL CHECK(expires_at_micros > approved_at_micros)
) STRICT;
CREATE TRIGGER recovery_approval_immutable BEFORE UPDATE ON recovery_approvals BEGIN
    SELECT RAISE(ABORT, 'approval history is immutable');
END;
CREATE TABLE recovery_approval_heads (
    operation_id TEXT PRIMARY KEY REFERENCES recovery_actions(operation_id),
    approval_id TEXT NOT NULL UNIQUE REFERENCES recovery_approvals(approval_id)
) STRICT;
CREATE TABLE recovery_workers (
    worker_id TEXT PRIMARY KEY,
    runtime_epoch TEXT NOT NULL,
    task_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    expires_at_micros INTEGER NOT NULL,
    revoked INTEGER NOT NULL CHECK(revoked IN (0,1))
) STRICT;
CREATE TABLE recovery_worker_capabilities (
    worker_id TEXT NOT NULL REFERENCES recovery_workers(worker_id),
    capability_id TEXT NOT NULL,
    PRIMARY KEY(worker_id, capability_id)
) STRICT;
CREATE TABLE recovery_attempts (
    attempt_id TEXT PRIMARY KEY,
    operation_id TEXT NOT NULL REFERENCES recovery_actions(operation_id),
    outbox_id TEXT NOT NULL UNIQUE REFERENCES outbox_messages(outbox_id),
    runtime_epoch TEXT NOT NULL,
    approval_id TEXT NOT NULL REFERENCES recovery_approvals(approval_id),
    evidence_key_id TEXT NOT NULL CHECK(length(evidence_key_id) = 64),
    worker_id TEXT REFERENCES recovery_workers(worker_id),
    started_at_micros INTEGER,
    UNIQUE(operation_id, attempt_id)
) STRICT;
CREATE TRIGGER recovery_attempt_immutable BEFORE UPDATE OF
    attempt_id, operation_id, outbox_id, runtime_epoch, approval_id, evidence_key_id ON recovery_attempts BEGIN
    SELECT RAISE(ABORT, 'attempt lineage is immutable');
END;
CREATE TRIGGER recovery_attempt_start_immutable BEFORE UPDATE OF worker_id, started_at_micros
ON recovery_attempts WHEN OLD.started_at_micros IS NOT NULL BEGIN
    SELECT RAISE(ABORT, 'started attempt binding is immutable');
END;
CREATE TABLE recovery_claims (
    outbox_id TEXT PRIMARY KEY REFERENCES recovery_attempts(outbox_id),
    worker_id TEXT NOT NULL REFERENCES recovery_workers(worker_id),
    runtime_epoch TEXT NOT NULL,
    token_hash TEXT NOT NULL UNIQUE CHECK(length(token_hash) = 64),
    expires_at_micros INTEGER NOT NULL
) STRICT;
CREATE TABLE recovery_evidence (
    evidence_id TEXT PRIMARY KEY CHECK(length(evidence_id) = 36),
    operation_id TEXT NOT NULL REFERENCES recovery_actions(operation_id),
    attempt_id TEXT NOT NULL REFERENCES recovery_attempts(attempt_id),
    verdict TEXT NOT NULL CHECK(verdict IN ('committed','not_committed','inconclusive')),
    receipt TEXT,
    payload BLOB NOT NULL CHECK(length(payload) <= 8192),
    payload_hash TEXT NOT NULL CHECK(length(payload_hash) = 64),
    key_id TEXT NOT NULL CHECK(length(key_id) = 64),
    authentication_tag BLOB NOT NULL CHECK(length(authentication_tag)=32),
    recorded_at_micros INTEGER NOT NULL
) STRICT;
CREATE INDEX recovery_evidence_attempt ON recovery_evidence(attempt_id, recorded_at_micros);
CREATE TRIGGER recovery_evidence_immutable BEFORE UPDATE ON recovery_evidence BEGIN
    SELECT RAISE(ABORT, 'reconciliation evidence is immutable');
END;
CREATE TRIGGER recovery_evidence_no_delete BEFORE DELETE ON recovery_evidence BEGIN
    SELECT RAISE(ABORT, 'reconciliation evidence cannot be deleted');
END;
CREATE TABLE recovery_plans (
    runtime_epoch TEXT NOT NULL,
    operation_id TEXT NOT NULL REFERENCES durable_operations(operation_id),
    operation_revision INTEGER NOT NULL CHECK(operation_revision >= 0),
    created_at_micros INTEGER NOT NULL,
    payload BLOB NOT NULL CHECK(length(payload) <= 8192),
    payload_hash TEXT NOT NULL CHECK(length(payload_hash) = 64),
    PRIMARY KEY(runtime_epoch, operation_id, operation_revision)
) STRICT;
CREATE TRIGGER recovery_plan_immutable BEFORE UPDATE ON recovery_plans BEGIN
    SELECT RAISE(ABORT, 'recovery plans are immutable');
END;
CREATE TABLE recovery_clock (
    singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
    last_observed_micros INTEGER NOT NULL
) STRICT;
INSERT INTO recovery_clock VALUES(1, 0);

CREATE TABLE recovery_evidence_keys (
    account_id TEXT NOT NULL,
    capability_id TEXT NOT NULL,
    key_id TEXT NOT NULL CHECK(length(key_id)=64),
    revoked INTEGER NOT NULL CHECK(revoked IN (0,1)),
    PRIMARY KEY(account_id,capability_id,key_id)
) STRICT;
CREATE TRIGGER recovery_key_cannot_revive BEFORE UPDATE OF revoked ON recovery_evidence_keys
WHEN OLD.revoked=1 AND NEW.revoked=0 BEGIN SELECT RAISE(ABORT, 'revoked evidence key cannot be revived'); END;
CREATE TABLE recovery_restore_fence (
    singleton INTEGER PRIMARY KEY CHECK(singleton=1),
    required INTEGER NOT NULL CHECK(required IN (0,1))
) STRICT;
INSERT INTO recovery_restore_fence VALUES(1,0);

CREATE TRIGGER recovery_actions_no_delete BEFORE DELETE ON recovery_actions BEGIN SELECT RAISE(ABORT,'durable recovery history cannot be deleted'); END;

CREATE TRIGGER recovery_approvals_no_delete BEFORE DELETE ON recovery_approvals BEGIN SELECT RAISE(ABORT,'durable recovery history cannot be deleted'); END;

CREATE TRIGGER recovery_attempts_no_delete BEFORE DELETE ON recovery_attempts BEGIN SELECT RAISE(ABORT,'durable recovery history cannot be deleted'); END;

CREATE TRIGGER recovery_plans_no_delete BEFORE DELETE ON recovery_plans BEGIN SELECT RAISE(ABORT,'durable recovery history cannot be deleted'); END;

CREATE UNIQUE INDEX recovery_one_compensation ON recovery_actions(original_operation_id) WHERE original_operation_id IS NOT NULL;
