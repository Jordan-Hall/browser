CREATE TABLE workspace_graphs (
    workspace_id TEXT PRIMARY KEY NOT NULL,
    privacy_scope TEXT NOT NULL CHECK (length(privacy_scope) BETWEEN 1 AND 128),
    revision INTEGER NOT NULL CHECK (revision > 0),
    graph BLOB NOT NULL CHECK (length(graph) BETWEEN 1 AND 1048576),
    graph_hash TEXT NOT NULL CHECK (length(graph_hash) = 64),
    updated_at_micros INTEGER NOT NULL
) STRICT;

CREATE TRIGGER workspace_graph_identity BEFORE UPDATE OF workspace_id, privacy_scope
ON workspace_graphs BEGIN SELECT RAISE(ABORT, 'workspace graph identity is immutable'); END;

CREATE TABLE workspace_graph_tasks (
    task_id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspace_graphs(workspace_id) ON DELETE RESTRICT,
    active INTEGER NOT NULL CHECK (active IN (0, 1))
) STRICT;
CREATE INDEX workspace_graph_tasks_workspace ON workspace_graph_tasks(workspace_id, active);
CREATE TRIGGER workspace_graph_task_identity BEFORE UPDATE OF task_id, workspace_id
ON workspace_graph_tasks BEGIN SELECT RAISE(ABORT, 'task workspace identity is immutable'); END;

CREATE TABLE workspace_checkpoints (
    checkpoint_id TEXT PRIMARY KEY NOT NULL CHECK (length(checkpoint_id) = 36),
    workspace_id TEXT NOT NULL REFERENCES workspace_graphs(workspace_id) ON DELETE RESTRICT,
    privacy_scope TEXT NOT NULL,
    graph_revision INTEGER NOT NULL CHECK (graph_revision > 0),
    created_at_micros INTEGER NOT NULL,
    payload BLOB NOT NULL CHECK (length(payload) BETWEEN 1 AND 4194304),
    payload_hash TEXT NOT NULL CHECK (length(payload_hash) = 64)
) STRICT;
CREATE INDEX workspace_checkpoint_history ON workspace_checkpoints(workspace_id, created_at_micros, checkpoint_id);
CREATE TRIGGER workspace_checkpoint_immutable BEFORE UPDATE ON workspace_checkpoints
BEGIN SELECT RAISE(ABORT, 'checkpoint snapshots are immutable'); END;

CREATE TABLE workspace_checkpoint_pins (
    checkpoint_id TEXT NOT NULL REFERENCES workspace_checkpoints(checkpoint_id) ON DELETE CASCADE,
    artifact_id TEXT NOT NULL REFERENCES artifact_handles_all(artifact_id) ON DELETE RESTRICT,
    PRIMARY KEY(checkpoint_id, artifact_id)
) STRICT, WITHOUT ROWID;
CREATE TRIGGER workspace_checkpoint_pin_immutable BEFORE UPDATE ON workspace_checkpoint_pins
BEGIN SELECT RAISE(ABORT, 'checkpoint pins are immutable'); END;
CREATE TRIGGER workspace_checkpoint_pin_delete_guard BEFORE DELETE ON workspace_checkpoint_pins
WHEN EXISTS (SELECT 1 FROM workspace_checkpoints WHERE checkpoint_id = OLD.checkpoint_id)
BEGIN SELECT RAISE(ABORT, 'release the checkpoint before removing its pins'); END;
CREATE TRIGGER workspace_checkpoint_pin_reference AFTER INSERT ON workspace_checkpoint_pins
BEGIN
    INSERT INTO artifact_references(artifact_id, reference_kind, reference_id, created_at_micros)
    SELECT NEW.artifact_id, 'core_checkpoint', NEW.checkpoint_id, created_at_micros
    FROM workspace_checkpoints WHERE checkpoint_id = NEW.checkpoint_id;
END;
CREATE TRIGGER workspace_checkpoint_pin_release AFTER DELETE ON workspace_checkpoint_pins
BEGIN
    DELETE FROM artifact_references WHERE artifact_id = OLD.artifact_id
    AND reference_kind = 'core_checkpoint' AND reference_id = OLD.checkpoint_id;
END;
CREATE TRIGGER workspace_checkpoint_reference_delete_guard BEFORE DELETE ON artifact_references
WHEN OLD.reference_kind = 'core_checkpoint' AND EXISTS (
    SELECT 1 FROM workspace_checkpoints WHERE checkpoint_id = OLD.reference_id
)
BEGIN SELECT RAISE(ABORT, 'release the checkpoint before removing its references'); END;
CREATE TRIGGER workspace_checkpoint_reference_update_guard BEFORE UPDATE ON artifact_references
WHEN OLD.reference_kind = 'core_checkpoint' OR NEW.reference_kind = 'core_checkpoint'
BEGIN SELECT RAISE(ABORT, 'checkpoint references are immutable'); END;

CREATE TRIGGER workspace_checkpoint_reference_insert_guard BEFORE INSERT ON artifact_references
WHEN NEW.reference_kind = 'core_checkpoint' AND NOT EXISTS (
    SELECT 1 FROM workspace_checkpoint_pins WHERE checkpoint_id = NEW.reference_id AND artifact_id = NEW.artifact_id
)
BEGIN SELECT RAISE(ABORT, 'checkpoint references require registered pins'); END;
