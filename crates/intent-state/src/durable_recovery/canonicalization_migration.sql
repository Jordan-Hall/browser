ALTER TABLE recovery_actions
ADD COLUMN canonicalization_version INTEGER NOT NULL DEFAULT 1
CHECK(canonicalization_version = 1);
