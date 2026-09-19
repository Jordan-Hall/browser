CREATE TABLE provider_recovery_attempts (
    attempt_id TEXT PRIMARY KEY NOT NULL CHECK (length(attempt_id) = 36),
    task_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    provider TEXT NOT NULL CHECK (length(trim(provider)) BETWEEN 1 AND 128),
    checkpoint_hash TEXT NOT NULL CHECK (length(checkpoint_hash) = 64),
    attempt_number INTEGER NOT NULL CHECK (attempt_number BETWEEN 1 AND 4),
    action TEXT NOT NULL CHECK (action IN ('resume_read_only', 'reseed')),
    recorded_at_micros INTEGER NOT NULL,
    outcome TEXT CHECK (outcome IN ('succeeded', 'failed')),
    finished_at_micros INTEGER,
    UNIQUE(task_id, account_id, provider, checkpoint_hash, attempt_number),
    CHECK ((outcome IS NULL) = (finished_at_micros IS NULL)),
    CHECK (finished_at_micros IS NULL OR finished_at_micros >= recorded_at_micros)
) STRICT;

CREATE TRIGGER provider_recovery_attempt_identity_immutable
BEFORE UPDATE OF attempt_id, task_id, account_id, provider, checkpoint_hash,
                 attempt_number, action, recorded_at_micros
ON provider_recovery_attempts
BEGIN
    SELECT RAISE(ABORT, 'provider recovery attempt identity is immutable');
END;

CREATE TRIGGER provider_recovery_outcome_immutable
BEFORE UPDATE OF outcome, finished_at_micros ON provider_recovery_attempts
WHEN OLD.outcome IS NOT NULL
BEGIN
    SELECT RAISE(ABORT, 'provider recovery outcome is immutable');
END;

CREATE TRIGGER provider_recovery_attempt_no_delete
BEFORE DELETE ON provider_recovery_attempts
BEGIN
    SELECT RAISE(ABORT, 'provider recovery attempt history is immutable');
END;
