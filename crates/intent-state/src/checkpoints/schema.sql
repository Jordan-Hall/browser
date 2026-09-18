CREATE TABLE task_checkpoints (
    checkpoint_id TEXT PRIMARY KEY NOT NULL CHECK (length(checkpoint_id) = 36),
    task_id TEXT NOT NULL CHECK (length(task_id) = 36),
    workspace_id TEXT NOT NULL CHECK (length(workspace_id) = 36),
    privacy_scope TEXT NOT NULL CHECK (length(privacy_scope) BETWEEN 1 AND 128),
    graph_revision INTEGER NOT NULL CHECK (graph_revision >= 0),
    predecessor TEXT REFERENCES task_checkpoints(checkpoint_id) ON DELETE RESTRICT,
    request_hash TEXT NOT NULL CHECK (length(request_hash) = 64),
    payload_hash TEXT NOT NULL CHECK (length(payload_hash) = 64),
    payload BLOB NOT NULL CHECK (length(payload) BETWEEN 1 AND 1048576),
    journal_watermark INTEGER NOT NULL CHECK (journal_watermark >= 0),
    created_at_micros INTEGER NOT NULL,
    retired INTEGER NOT NULL DEFAULT 0 CHECK (retired IN (0, 1)),
    UNIQUE(task_id, graph_revision)
) STRICT;

CREATE TABLE task_checkpoint_heads (
    task_id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    privacy_scope TEXT NOT NULL,
    checkpoint_id TEXT NOT NULL UNIQUE REFERENCES task_checkpoints(checkpoint_id) ON DELETE RESTRICT,
    graph_revision INTEGER NOT NULL CHECK (graph_revision >= 0)
) STRICT;

CREATE TABLE checkpoint_artifacts (
    checkpoint_id TEXT NOT NULL REFERENCES task_checkpoints(checkpoint_id) ON DELETE RESTRICT,
    artifact_id TEXT NOT NULL REFERENCES artifact_handles_all(artifact_id) ON DELETE RESTRICT,
    PRIMARY KEY(checkpoint_id, artifact_id)
) STRICT, WITHOUT ROWID;

CREATE INDEX checkpoint_artifact_lookup ON checkpoint_artifacts(artifact_id);

CREATE TRIGGER task_checkpoints_immutable
BEFORE UPDATE OF checkpoint_id, task_id, workspace_id, privacy_scope, graph_revision,
    predecessor, request_hash, payload_hash, payload, journal_watermark, created_at_micros
ON task_checkpoints
BEGIN
    SELECT RAISE(ABORT, 'checkpoint content and identity are immutable');
END;

CREATE TRIGGER task_checkpoints_no_delete
BEFORE DELETE ON task_checkpoints
BEGIN
    SELECT RAISE(ABORT, 'checkpoint history must retain a retirement tombstone');
END;

CREATE TRIGGER checkpoint_pin_create
AFTER INSERT ON checkpoint_artifacts
BEGIN
    SELECT CASE WHEN NOT EXISTS (
        SELECT 1 FROM task_checkpoints c JOIN artifact_handles_all h
        ON h.artifact_id = NEW.artifact_id AND h.privacy_scope = c.privacy_scope
        WHERE c.checkpoint_id = NEW.checkpoint_id AND c.retired = 0
          AND h.suppressed_at_micros IS NULL
    ) THEN RAISE(ABORT, 'checkpoint artifact is unavailable or outside scope') END;
    INSERT INTO artifact_references(artifact_id, reference_kind, reference_id, created_at_micros)
    SELECT NEW.artifact_id, 'runtime_checkpoint', NEW.checkpoint_id, created_at_micros
    FROM task_checkpoints WHERE checkpoint_id = NEW.checkpoint_id;
END;

CREATE TRIGGER checkpoint_pin_protect
BEFORE DELETE ON artifact_references
WHEN OLD.reference_kind = 'runtime_checkpoint' AND EXISTS (
    SELECT 1 FROM checkpoint_artifacts p JOIN task_checkpoints c USING(checkpoint_id)
    WHERE p.checkpoint_id = OLD.reference_id AND p.artifact_id = OLD.artifact_id AND c.retired = 0
)
BEGIN
    SELECT RAISE(ABORT, 'active checkpoint owns this artifact reference');
END;

CREATE TRIGGER checkpoint_membership_protect
BEFORE DELETE ON checkpoint_artifacts
WHEN EXISTS (SELECT 1 FROM task_checkpoints WHERE checkpoint_id = OLD.checkpoint_id AND retired = 0)
BEGIN
    SELECT RAISE(ABORT, 'retire a checkpoint before releasing its dependencies');
END;

CREATE TRIGGER checkpoint_membership_no_update
BEFORE UPDATE ON checkpoint_artifacts
BEGIN
    SELECT RAISE(ABORT, 'checkpoint dependency identity is immutable');
END;

CREATE TRIGGER checkpoint_retirement_guard
BEFORE UPDATE OF retired ON task_checkpoints
WHEN NEW.retired != 1 OR OLD.retired != 0 OR EXISTS (
    SELECT 1 FROM task_checkpoint_heads WHERE checkpoint_id = OLD.checkpoint_id
)
BEGIN
    SELECT RAISE(ABORT, 'current or already retired checkpoint cannot be retired');
END;

CREATE TRIGGER checkpoint_retire_dependencies
AFTER UPDATE OF retired ON task_checkpoints
WHEN NEW.retired = 1
BEGIN
    DELETE FROM checkpoint_artifacts WHERE checkpoint_id = NEW.checkpoint_id;
    DELETE FROM artifact_references WHERE reference_kind = 'runtime_checkpoint' AND reference_id = NEW.checkpoint_id;
END;
