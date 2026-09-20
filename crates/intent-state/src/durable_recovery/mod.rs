//! Trusted runtime owner. These methods are not worker-facing deserialization APIs.
mod dispatch;
mod evidence;
mod planner;
mod projection;
mod types;
use crate::operations::load_operation_from_connection;
use crate::recovery_error::{digest, sql_u64};
use crate::snapshot_fs::Directory;
use crate::{DurableOperation, DurableOperationState, RecoveryError, StateStore};
pub use evidence::*;
use intent_contracts::{OperationAttemptId, OperationId, UnixTimestampMicros};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use std::{fs::File, path::Path};
pub use types::*;
use uuid::Uuid;

/// Lifetime ownership of one private profile directory and database inode.
/// Advisory locks coordinate trusted owners; this is not a hostile-user sandbox.
/// No clone/deserialization or API for constructing an enabled runtime epoch exists.
#[derive(Debug)]
pub struct RuntimeOwner {
    store: StateStore,
    epoch: Uuid,
    _database_lock: File,
    _directory_lock: Directory,
}
impl RuntimeOwner {
    pub fn open_profile(
        root: impl AsRef<Path>,
        now: UnixTimestampMicros,
    ) -> Result<Self, RecoveryError> {
        let directory = Directory::open_private(root.as_ref())?;
        let database = directory.lock_profile()?;
        let mut store = StateStore::open(directory.sqlite_path())?;
        let epoch = Uuid::new_v4();
        {
            let tx = store
                .connection
                .transaction_with_behavior(TransactionBehavior::Immediate)?;
            // Commit the fence before planning. A planning failure cannot reopen dispatch.
            tx.execute("UPDATE runtime_control SET epoch=?1, dispatch_enabled=0, reason='startup recovery required' WHERE singleton=1", [epoch.to_string()])?;
            tx.execute("DELETE FROM recovery_claims", [])?;
            // Never use old approvals after restart/restore, even when their wall clock has not expired.
            tx.execute("DELETE FROM recovery_approval_heads", [])?;
            if now.get() < 0 {
                return Err(RecoveryError::Invalid("negative runtime clock"));
            }
            tx.commit()?;
        }
        store.runtime_epoch = Some(epoch);
        Ok(Self {
            store,
            epoch,
            _database_lock: database,
            _directory_lock: directory,
        })
    }
    pub fn epoch(&self) -> Uuid {
        self.epoch
    }
    /// Trusted application state access. Low-level operation/outbox mutators cannot bypass managed action guards.
    pub fn state(&self) -> &StateStore {
        &self.store
    }
    pub fn state_mut(&mut self) -> &mut StateStore {
        &mut self.store
    }

    /// Require this owner to still be in its startup fence before installing durable archive
    /// state. This check grants no authority and deliberately refuses a running incarnation.
    pub fn require_workspace_activation_fence(&self) -> Result<(), RecoveryError> {
        current_epoch(&self.store.connection, self.epoch, false)?;
        let enabled: bool = self.store.connection.query_row(
            "SELECT dispatch_enabled FROM runtime_control WHERE singleton=1",
            [],
            |row| row.get(0),
        )?;
        if enabled {
            return Err(RecoveryError::Denied(
                "workspace activation requires the startup dispatch fence",
            ));
        }
        Ok(())
    }

    pub fn update_authority(
        &mut self,
        input: AuthorityUpdate,
        now: UnixTimestampMicros,
    ) -> Result<u64, RecoveryError> {
        if input.enabled
            && (input.valid_until <= now
                || input.effect == intent_recovery::RecoveryEffect::Unknown)
        {
            return Err(RecoveryError::Invalid("expired authority"));
        }
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, false)?;
        observe_clock(&tx, now)?;
        let previous: Option<i64> = tx.query_row("SELECT revision FROM recovery_authorities WHERE account_id=?1 AND capability_id=?2",
            params![input.account_id.to_string(), input.capability_id.to_string()], |r| r.get(0)).optional()?;
        if previous.unwrap_or(0) != sql_u64(input.expected_revision)? {
            return Err(RecoveryError::Conflict("authority revision"));
        }
        let next = sql_u64(input.expected_revision)?
            .checked_add(1)
            .ok_or(RecoveryError::Limit("authority revision"))?;
        tx.execute("INSERT INTO recovery_authorities VALUES(?1,?2,?3,?4,?5,?6,?7,?8) ON CONFLICT(account_id,capability_id) DO UPDATE SET revision=excluded.revision,enabled=excluded.enabled,source_revision=excluded.source_revision,valid_until_micros=excluded.valid_until_micros,evidence_key_id=excluded.evidence_key_id,effect=excluded.effect",
            params![input.account_id.to_string(), input.capability_id.to_string(), next, input.enabled, input.source_revision.to_hex(), input.valid_until.get(), input.evidence_key_id.to_hex(),effect_name(input.effect)])?;
        tx.execute("INSERT INTO recovery_evidence_keys VALUES(?1,?2,?3,0) ON CONFLICT(account_id,capability_id,key_id) DO NOTHING", params![input.account_id.to_string(),input.capability_id.to_string(),input.evidence_key_id.to_hex()])?;
        tx.commit()?;
        Ok(next as u64)
    }
    pub fn register_worker(
        &mut self,
        input: WorkerRegistration,
        now: UnixTimestampMicros,
    ) -> Result<(), RecoveryError> {
        if input.capabilities.is_empty() || input.capabilities.len() > 64 || input.expires_at <= now
        {
            return Err(RecoveryError::Invalid("worker capabilities or expiry"));
        }
        let unique: std::collections::BTreeSet<_> = input.capabilities.iter().collect();
        if unique.len() != input.capabilities.len() {
            return Err(RecoveryError::Invalid("duplicate worker capability"));
        }
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, true)?;
        observe_clock(&tx, now)?;
        let count: i64 = tx.query_row(
            "SELECT count(*) FROM recovery_workers WHERE runtime_epoch=?1 AND revoked=0",
            [self.epoch.to_string()],
            |r| r.get(0),
        )?;
        if count >= 1024 {
            return Err(RecoveryError::Limit("active worker registry"));
        }
        tx.execute(
            "INSERT INTO recovery_workers VALUES(?1,?2,?3,?4,?5,0)",
            params![
                input.worker_id.to_string(),
                self.epoch.to_string(),
                input.task_id.to_string(),
                input.account_id.to_string(),
                input.expires_at.get()
            ],
        )?;
        for cap in input.capabilities {
            tx.execute(
                "INSERT INTO recovery_worker_capabilities VALUES(?1,?2)",
                params![input.worker_id.to_string(), cap.to_string()],
            )?;
        }
        tx.commit()?;
        Ok(())
    }
    /// Durable cancellation linearization point. Revoke the worker, retire its unstarted claims,
    /// and journal cancellation intent for its active started attempts in one writer transaction
    /// before notification. Started attempts remain uncertain until transport settlement or restart
    /// recovery moves them to reconciliation.
    pub fn revoke_worker(
        &mut self,
        worker: intent_contracts::WorkerInstanceId,
        now: UnixTimestampMicros,
    ) -> Result<(), RecoveryError> {
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, false)?;
        observe_clock(&tx, now)?;
        let revoked: Option<bool> = tx
            .query_row(
                "SELECT revoked FROM recovery_workers WHERE worker_id=?1 AND runtime_epoch=?2",
                params![worker.to_string(), self.epoch.to_string()],
                |row| row.get(0),
            )
            .optional()?;
        let Some(already_revoked) = revoked else {
            return Err(RecoveryError::Missing("current worker"));
        };
        if already_revoked {
            tx.commit()?;
            return Ok(());
        }
        tx.execute(
            "UPDATE recovery_workers SET revoked=1 WHERE worker_id=?1 AND runtime_epoch=?2",
            params![worker.to_string(), self.epoch.to_string()],
        )?;

        let claimed = {
            let mut statement = tx.prepare(
                "SELECT a.operation_id,a.attempt_id,a.outbox_id \
                 FROM recovery_claims c JOIN recovery_attempts a ON a.outbox_id=c.outbox_id \
                 WHERE c.worker_id=?1 AND c.runtime_epoch=?2 AND a.runtime_epoch=?2 \
                   AND a.started_at_micros IS NULL ORDER BY a.operation_id",
            )?;
            statement
                .query_map(params![worker.to_string(), self.epoch.to_string()], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                })?
                .collect::<Result<Vec<_>, _>>()?
        };
        for (operation, attempt, outbox) in claimed {
            let operation_id = dispatch::parse(&operation)?;
            let attempt_id = dispatch::parse(&attempt)?;
            let op = load(&tx, operation_id)?;
            if op.state() != DurableOperationState::DispatchPending
                || op.attempt_identity() != Some(attempt_id)
            {
                return Err(RecoveryError::Integrity("claimed attempt state changed"));
            }
            if tx.execute(
                "UPDATE outbox_messages SET state='failed',lease_owner=NULL,lease_expires_at_micros=NULL, \
                 failure_detail='cancelled by worker revocation before dispatch',updated_at_micros=?2 \
                 WHERE outbox_id=?1 AND state IN ('pending','leased')",
                params![outbox, now.get()],
            )? != 1
            {
                return Err(RecoveryError::Integrity("claimed outbox state changed"));
            }
            tx.execute(
                "DELETE FROM recovery_approval_heads WHERE operation_id=?1",
                [operation],
            )?;
            tx.execute("DELETE FROM recovery_claims WHERE outbox_id=?1", [outbox])?;
        }

        let started = {
            let mut statement = tx.prepare(
                "SELECT a.operation_id,a.attempt_id,a.outbox_id \
                 FROM recovery_attempts a JOIN durable_operations o \
                   ON o.operation_id=a.operation_id AND o.attempt_identity=a.attempt_id \
                 WHERE a.worker_id=?1 AND a.runtime_epoch=?2 \
                   AND a.started_at_micros IS NOT NULL AND o.state='attempting' \
                 ORDER BY a.operation_id",
            )?;
            statement
                .query_map(params![worker.to_string(), self.epoch.to_string()], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                })?
                .collect::<Result<Vec<_>, _>>()?
        };
        for (operation, attempt, outbox) in started {
            let operation_id = dispatch::parse(&operation)?;
            let attempt_id = dispatch::parse(&attempt)?;
            let op = load(&tx, operation_id)?;
            let attempting_outbox: bool = tx.query_row(
                "SELECT EXISTS(SELECT 1 FROM outbox_messages WHERE outbox_id=?1 \
                 AND operation_id=?2 AND attempt_identity=?3 AND state='attempting')",
                params![outbox, operation, attempt],
                |row| row.get(0),
            )?;
            if op.state() != DurableOperationState::Attempting
                || op.attempt_identity() != Some(attempt_id)
                || !attempting_outbox
            {
                return Err(RecoveryError::Integrity("started attempt state changed"));
            }
            transition(
                &tx,
                &op,
                DurableOperationState::Attempting,
                Some(attempt_id),
                now,
                "worker cancellation requested after dispatch start; outcome retained for reconciliation",
            )?;
        }

        tx.execute(
            "DELETE FROM recovery_claims WHERE worker_id=?1 AND runtime_epoch=?2",
            params![worker.to_string(), self.epoch.to_string()],
        )?;
        tx.commit()?;
        Ok(())
    }
}
impl Drop for RuntimeOwner {
    fn drop(&mut self) {
        // A failed shutdown write does not allow a new owner to inherit this epoch: open_profile always fences first.
        let _ = self.store.connection.execute("UPDATE runtime_control SET dispatch_enabled=0,reason='runtime owner stopped' WHERE singleton=1 AND epoch=?1",[self.epoch.to_string()]);
    }
}
pub(super) fn current_epoch(
    db: &Connection,
    epoch: Uuid,
    enabled: bool,
) -> Result<(), RecoveryError> {
    let valid: bool = db.query_row(
        "SELECT epoch=?1 AND (?2=0 OR dispatch_enabled=1) FROM runtime_control WHERE singleton=1",
        params![epoch.to_string(), enabled],
        |r| r.get(0),
    )?;
    if !valid {
        return Err(RecoveryError::Denied("stale or disabled runtime epoch"));
    }
    Ok(())
}
pub(super) fn observe_clock(
    db: &Connection,
    now: UnixTimestampMicros,
) -> Result<(), RecoveryError> {
    let last: i64 = db.query_row(
        "SELECT last_observed_micros FROM recovery_clock WHERE singleton=1",
        [],
        |r| r.get(0),
    )?;
    if now.get() < last {
        return Err(RecoveryError::Denied(
            "wall clock moved backwards; recovery review required",
        ));
    }
    db.execute(
        "UPDATE recovery_clock SET last_observed_micros=?1 WHERE singleton=1",
        [now.get()],
    )?;
    Ok(())
}
pub(super) fn load(db: &Connection, id: OperationId) -> Result<DurableOperation, RecoveryError> {
    load_operation_from_connection(db, id)?.ok_or(RecoveryError::Missing("operation"))
}
pub(super) fn transition(
    db: &Connection,
    current: &DurableOperation,
    next: DurableOperationState,
    attempt: Option<OperationAttemptId>,
    now: UnixTimestampMicros,
    reason: &'static str,
) -> Result<u64, RecoveryError> {
    if now < current.updated_at() {
        return Err(RecoveryError::Invalid("operation clock regression"));
    }
    let rev = current
        .revision()
        .checked_add(1)
        .ok_or(RecoveryError::Limit("operation revision"))?;
    if db.execute("UPDATE durable_operations SET state=?1,state_detail=?2,attempt_identity=?3,revision=?4,updated_at_micros=?5 WHERE operation_id=?6 AND revision=?7",
        params![next.as_str(),reason,attempt.map(|x|x.to_string()),sql_u64(rev)?,now.get(),current.operation_id().to_string(),sql_u64(current.revision())?])? != 1 {
        return Err(RecoveryError::Conflict("operation revision"));
    }
    db.execute("INSERT INTO operation_journal(operation_id,revision,from_state,to_state,state_detail,attempt_identity,occurred_at_micros) VALUES(?1,?2,?3,?4,?5,?6,?7)",
        params![current.operation_id().to_string(),sql_u64(rev)?,current.state().as_str(),next.as_str(),reason,attempt.map(|x|x.to_string()),now.get()])?;
    Ok(rev)
}
pub(super) fn managed(db: &Connection, id: OperationId) -> rusqlite::Result<bool> {
    db.query_row(
        "SELECT EXISTS(SELECT 1 FROM recovery_actions WHERE operation_id=?1)",
        [id.to_string()],
        |r| r.get(0),
    )
}

#[cfg(test)]
mod tests;

pub(super) fn effect_name(effect: intent_recovery::RecoveryEffect) -> &'static str {
    match effect {
        intent_recovery::RecoveryEffect::ReadOnly => "read_only",
        intent_recovery::RecoveryEffect::LocalReversible => "local_reversible",
        intent_recovery::RecoveryEffect::ExternalWrite => "external_write",
        intent_recovery::RecoveryEffect::Unknown => "unknown",
    }
}
