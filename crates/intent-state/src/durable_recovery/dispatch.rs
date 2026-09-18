use super::*;
use crate::recovery_error::unsigned;
use intent_contracts::{BoundedText, OutboxMessageId, SchemaVersion, WorkerInstanceId};
use intent_recovery::RecoveryEffect;
use rusqlite::{OptionalExtension, params};

impl RuntimeOwner {
    /// Snapshot-only preflight; claim/start still recheck all authority under the writer lock.
    pub fn dispatch_metadata(
        &self,
        outbox: OutboxMessageId,
    ) -> Result<DispatchMetadata, RecoveryError> {
        let tx = self.store.connection.unchecked_transaction()?;
        current_epoch(&tx, self.epoch, true)?;
        let row: Option<(String, String, i64, i64, String, String)> = tx.query_row(
            "SELECT a.operation_id,a.attempt_id,p.deadline_micros,length(p.payload),p.destination,p.message_kind FROM recovery_attempts a JOIN recovery_actions p ON p.operation_id=a.operation_id WHERE a.outbox_id=?1 AND a.runtime_epoch=?2 AND a.started_at_micros IS NULL AND length(CAST(p.destination AS BLOB))<=512 AND length(CAST(p.message_kind AS BLOB))<=128",
            params![outbox.to_string(), self.epoch.to_string()],
            |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?,r.get(5)?)),
        ).optional()?;
        let (operation, attempt, deadline, size, destination, kind) =
            row.ok_or(RecoveryError::Denied("no current unsent managed attempt"))?;
        let op = load(&tx, parse(&operation)?)?;
        let attempt_id = parse(&attempt)?;
        if op.state() != DurableOperationState::DispatchPending
            || op.attempt_identity() != Some(attempt_id)
        {
            return Err(RecoveryError::Denied("operation no longer dispatchable"));
        }
        let result = DispatchMetadata {
            operation_id: op.operation_id(),
            attempt_id,
            task_id: op.task_id(),
            account_id: op.account_id(),
            capability_id: op.capability_id(),
            deadline: UnixTimestampMicros::try_new(deadline)
                .map_err(|_| RecoveryError::Integrity("action deadline"))?,
            payload_bytes: unsigned(size)?,
            routing_json_bytes: (serde_json::to_string(&destination)?.len()
                + serde_json::to_string(&kind)?.len()) as u64,
        };
        tx.commit()?;
        Ok(result)
    }
    pub fn prepare_action(
        &mut self,
        action: RecoverableAction,
    ) -> Result<OperationId, RecoveryError> {
        let new = &action.operation;
        if action.payload.len() > crate::MAX_OUTBOX_PAYLOAD_BYTES || action.payload.is_empty() {
            return Err(RecoveryError::Limit("action payload"));
        }
        if new.source_schema != SchemaVersion::V1
            || action.deadline <= new.created_at
            || action.destination.as_str().is_empty()
            || action.message_kind.as_str().is_empty()
            || [
                new.operation_id.as_uuid(),
                new.task_id.as_uuid(),
                new.action_proposal_id.as_uuid(),
                new.account_id.as_uuid(),
                new.capability_id.as_uuid(),
            ]
            .iter()
            .any(Uuid::is_nil)
        {
            return Err(RecoveryError::Invalid(
                "action identity, deadline or schema",
            ));
        }
        if digest(&action.payload) != new.arguments_hash {
            return Err(RecoveryError::Integrity("approved argument bytes"));
        }
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, true)?;
        observe_clock(&tx, new.created_at)?;
        let active: bool = tx.query_row("SELECT EXISTS(SELECT 1 FROM workspace_graph_tasks t JOIN workspace_graphs g ON g.workspace_id=t.workspace_id WHERE t.task_id=?1 AND t.active=1 AND g.privacy_scope=?2)",params![new.task_id.to_string(),action.scope.as_str()],|r|r.get(0))?;
        if !active {
            return Err(RecoveryError::Denied(
                "action task is not active in this privacy scope",
            ));
        }
        let registered: Option<String> = tx
            .query_row(
                "SELECT effect FROM recovery_authorities WHERE account_id=?1 AND capability_id=?2",
                params![new.account_id.to_string(), new.capability_id.to_string()],
                |r| r.get(0),
            )
            .optional()?;
        if registered.as_deref() != Some(effect_name(action.effect)) {
            return Err(RecoveryError::Denied(
                "action effect differs from registered capability recovery policy",
            ));
        }
        if let Some(origin) = action.compensation {
            let original = load(&tx, origin.operation_id)?;
            let compensation_allowed: bool = tx.query_row("SELECT effect!='read_only' AND original_operation_id IS NULL FROM recovery_actions WHERE operation_id=?1",[origin.operation_id.to_string()],|r|r.get(0))?;
            if !compensation_allowed {
                return Err(RecoveryError::Denied(
                    "only original write operations can be compensated",
                ));
            }

            let original_scope: String = tx.query_row(
                "SELECT privacy_scope FROM recovery_actions WHERE operation_id=?1",
                [origin.operation_id.to_string()],
                |r| r.get(0),
            )?;
            if original.state() != DurableOperationState::Verified
                || original.account_id() != new.account_id
                || original_scope != action.scope.as_str()
                || original.attempt_identity() != Some(origin.attempt_id)
            {
                return Err(RecoveryError::Denied(
                    "compensation origin is not a verified same-scope attempt",
                ));
            }
            let proven: bool = tx.query_row("SELECT EXISTS(SELECT 1 FROM recovery_evidence WHERE operation_id=?1 AND attempt_id=?2 AND verdict='committed' AND receipt=?3)",params![origin.operation_id.to_string(),origin.attempt_id.to_string(),origin.receipt.to_hex()],|r|r.get(0))?;
            if !proven {
                return Err(RecoveryError::Denied("compensation origin receipt"));
            }
            let exists: bool = tx.query_row(
                "SELECT EXISTS(SELECT 1 FROM recovery_actions WHERE original_operation_id=?1)",
                [origin.operation_id.to_string()],
                |r| r.get(0),
            )?;
            if exists {
                return Err(RecoveryError::Conflict(
                    "compensation action already exists; inspect its outcome",
                ));
            }
        }
        tx.execute("INSERT INTO durable_operations(operation_id,task_id,action_proposal_id,account_id,capability_id,arguments_hash,source_schema_major,source_schema_minor,state,revision,created_at_micros,updated_at_micros) VALUES(?1,?2,?3,?4,?5,?6,1,0,'prepared',0,?7,?7)",params![new.operation_id.to_string(),new.task_id.to_string(),new.action_proposal_id.to_string(),new.account_id.to_string(),new.capability_id.to_string(),new.arguments_hash.to_hex(),new.created_at.get()])?;
        tx.execute("INSERT INTO operation_journal(operation_id,revision,to_state,occurred_at_micros) VALUES(?1,0,'prepared',?2)",params![new.operation_id.to_string(),new.created_at.get()])?;
        let effect = match action.effect {
            RecoveryEffect::ReadOnly => "read_only",
            RecoveryEffect::LocalReversible => "local_reversible",
            RecoveryEffect::ExternalWrite => "external_write",
            RecoveryEffect::Unknown => "unknown",
        };
        tx.execute(
            "INSERT INTO recovery_actions VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
            params![
                new.operation_id.to_string(),
                action.scope.as_str(),
                effect,
                action.source_revision.to_hex(),
                action.destination.as_str(),
                action.message_kind.as_str(),
                action.payload,
                action.deadline.get(),
                action.compensation.map(|v| v.operation_id.to_string()),
                action.compensation.map(|v| v.attempt_id.to_string()),
                action.compensation.map(|v| v.receipt.to_hex())
            ],
        )?;
        tx.commit()?;
        Ok(new.operation_id)
    }
    /// Called by a trusted approval UI/broker only after displaying the exact immutable action.
    pub fn approve_action(
        &mut self,
        id: OperationId,
        expected_revision: u64,
        expires: UnixTimestampMicros,
        now: UnixTimestampMicros,
    ) -> Result<u64, RecoveryError> {
        if expires <= now {
            return Err(RecoveryError::Invalid("approval expiry"));
        }
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, true)?;
        observe_clock(&tx, now)?;
        let op = load(&tx, id)?;
        require_revision(&op, expected_revision)?;
        if !managed(&tx, id)? {
            return Err(RecoveryError::Denied("unmanaged operation"));
        }
        match op.state() {
            DurableOperationState::Prepared | DurableOperationState::Approved => {}
            DurableOperationState::DispatchPending => {
                let started: bool = tx.query_row("SELECT EXISTS(SELECT 1 FROM recovery_attempts WHERE attempt_id=?1 AND started_at_micros IS NOT NULL)",[op.attempt_identity().map(|a|a.to_string())],|r|r.get(0))?;
                if started {
                    return Err(RecoveryError::Denied(
                        "possibly committed attempt cannot be reapproved",
                    ));
                }
            }
            DurableOperationState::Failed => {
                let proven: bool = tx.query_row("SELECT EXISTS(SELECT 1 FROM recovery_evidence WHERE operation_id=?1 AND attempt_id=?2 AND verdict='not_committed')",params![id.to_string(),op.attempt_identity().map(|a|a.to_string())],|r|r.get(0))?;
                if !proven {
                    return Err(RecoveryError::Denied(
                        "fresh approval requires proven non-commit",
                    ));
                }
            }
            _ => {
                return Err(RecoveryError::Denied(
                    "uncertain or committed effects cannot be retried",
                ));
            }
        }
        let authority_revision = valid_authority(&tx, &op, now)?;
        let deadline: i64 = tx.query_row(
            "SELECT deadline_micros FROM recovery_actions WHERE operation_id=?1",
            [id.to_string()],
            |r| r.get(0),
        )?;
        if expires.get() > deadline {
            return Err(RecoveryError::Invalid(
                "approval exceeds operation deadline",
            ));
        }
        let approval = Uuid::new_v4();
        tx.execute(
            "INSERT INTO recovery_approvals VALUES(?1,?2,?3,?4,?5,?6)",
            params![
                approval.to_string(),
                id.to_string(),
                self.epoch.to_string(),
                authority_revision,
                now.get(),
                expires.get()
            ],
        )?;
        tx.execute("INSERT INTO recovery_approval_heads VALUES(?1,?2) ON CONFLICT(operation_id) DO UPDATE SET approval_id=excluded.approval_id",params![id.to_string(),approval.to_string()])?;
        tx.execute("UPDATE outbox_messages SET state='failed',lease_owner=NULL,lease_expires_at_micros=NULL,failure_detail='superseded by fresh approval',updated_at_micros=?2 WHERE operation_id=?1 AND state IN ('pending','leased')",params![id.to_string(),now.get()])?;
        tx.execute("DELETE FROM recovery_claims WHERE outbox_id IN (SELECT outbox_id FROM outbox_messages WHERE operation_id=?1)",[id.to_string()])?;
        let rev = transition(
            &tx,
            &op,
            DurableOperationState::Approved,
            None,
            now,
            "current explicit approval",
        )?;
        tx.commit()?;
        Ok(rev)
    }
    pub fn enqueue_action(
        &mut self,
        id: OperationId,
        expected_revision: u64,
        now: UnixTimestampMicros,
    ) -> Result<OutboxMessageId, RecoveryError> {
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, true)?;
        observe_clock(&tx, now)?;
        let op = load(&tx, id)?;
        require_revision(&op, expected_revision)?;
        if op.state() != DurableOperationState::Approved {
            return Err(RecoveryError::Denied("action is not approved"));
        }
        let approval = valid_approval(&tx, &op, self.epoch, now)?;
        let material = load_material(&tx, id)?;
        if digest(&material.payload) != op.arguments_hash() {
            return Err(RecoveryError::Integrity("immutable action bytes"));
        }
        let outbox = OutboxMessageId::from_uuid(Uuid::new_v4());
        let attempt = OperationAttemptId::from_uuid(Uuid::new_v4());
        tx.execute("INSERT INTO outbox_messages(outbox_id,operation_id,attempt_identity,destination,message_kind,payload,payload_hash,state,created_at_micros,updated_at_micros) VALUES(?1,?2,?3,?4,?5,?6,?7,'pending',?8,?8)",params![outbox.to_string(),id.to_string(),attempt.to_string(),material.destination.as_str(),material.message_kind.as_str(),material.payload,op.arguments_hash().to_hex(),now.get()])?;
        tx.execute("INSERT INTO recovery_attempts(attempt_id,operation_id,outbox_id,runtime_epoch,approval_id,evidence_key_id) SELECT ?1,?2,?3,?4,?5,evidence_key_id FROM recovery_authorities WHERE account_id=?6 AND capability_id=?7",params![attempt.to_string(),id.to_string(),outbox.to_string(),self.epoch.to_string(),approval,op.account_id().to_string(),op.capability_id().to_string()])?;
        transition(
            &tx,
            &op,
            DurableOperationState::DispatchPending,
            Some(attempt),
            now,
            "immutable approved action staged",
        )?;
        tx.commit()?;
        Ok(outbox)
    }
    pub fn claim_dispatch(
        &mut self,
        outbox: OutboxMessageId,
        worker: WorkerInstanceId,
        expires: UnixTimestampMicros,
        now: UnixTimestampMicros,
    ) -> Result<DurableDispatchLease, RecoveryError> {
        if expires <= now {
            return Err(RecoveryError::Invalid("dispatch lease expiry"));
        }
        let mut token = [0; 32];
        getrandom::fill(&mut token)
            .map_err(|_| RecoveryError::Denied("system randomness unavailable"))?;
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, true)?;
        observe_clock(&tx, now)?;
        let (id,attempt,approval,epoch):(String,String,String,String)=tx.query_row("SELECT operation_id,attempt_id,approval_id,runtime_epoch FROM recovery_attempts WHERE outbox_id=?1 AND started_at_micros IS NULL",[outbox.to_string()],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?))).optional()?.ok_or(RecoveryError::Missing("unstarted attempt"))?;
        let op = load(&tx, parse(&id)?)?;
        if epoch != self.epoch.to_string()
            || op.state() != DurableOperationState::DispatchPending
            || op.attempt_identity().map(|a| a.to_string()).as_deref() != Some(&attempt)
        {
            return Err(RecoveryError::Denied("superseded attempt"));
        }
        if valid_approval(&tx, &op, self.epoch, now)? != approval {
            return Err(RecoveryError::Denied("attempt approval was replaced"));
        }
        valid_worker(&tx, &op, worker, self.epoch, now)?;
        let prior: Option<i64> = tx
            .query_row(
                "SELECT expires_at_micros FROM recovery_claims WHERE outbox_id=?1",
                [outbox.to_string()],
                |r| r.get(0),
            )
            .optional()?;
        if prior.is_some_and(|expiry| expiry > now.get()) {
            return Err(RecoveryError::Conflict("live dispatch claim"));
        }
        tx.execute("INSERT INTO recovery_claims VALUES(?1,?2,?3,?4,?5) ON CONFLICT(outbox_id) DO UPDATE SET worker_id=excluded.worker_id,runtime_epoch=excluded.runtime_epoch,token_hash=excluded.token_hash,expires_at_micros=excluded.expires_at_micros",params![outbox.to_string(),worker.to_string(),self.epoch.to_string(),digest(&token).to_hex(),expires.get()])?;
        tx.execute("UPDATE outbox_messages SET state='leased',lease_owner=?2,lease_expires_at_micros=?3,updated_at_micros=?4 WHERE outbox_id=?1 AND state IN ('pending','leased')",params![outbox.to_string(),worker.to_string(),expires.get(),now.get()])?;
        tx.commit()?;
        Ok(DurableDispatchLease {
            outbox_id: outbox,
            worker_id: worker,
            epoch: self.epoch,
            token,
        })
    }
    /// Linearization point: revocation/authority checks and persisted attempt start share this transaction.
    /// A revocation after commit cannot unsend the already-admitted effect.
    pub fn begin_authorized_dispatch(
        &mut self,
        lease: DurableDispatchLease,
        now: UnixTimestampMicros,
    ) -> Result<DurableSendAttempt, RecoveryError> {
        if lease.epoch != self.epoch {
            return Err(RecoveryError::Denied("foreign runtime claim"));
        }
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, true)?;
        observe_clock(&tx, now)?;
        let valid:bool=tx.query_row("SELECT EXISTS(SELECT 1 FROM recovery_claims WHERE outbox_id=?1 AND worker_id=?2 AND runtime_epoch=?3 AND token_hash=?4 AND expires_at_micros>?5)",params![lease.outbox_id.to_string(),lease.worker_id.to_string(),self.epoch.to_string(),digest(&lease.token).to_hex(),now.get()],|r|r.get(0))?;
        if !valid {
            return Err(RecoveryError::Denied(
                "expired, revoked or consumed dispatch claim",
            ));
        }
        let (id,attempt,approval):(String,String,String)=tx.query_row("SELECT operation_id,attempt_id,approval_id FROM recovery_attempts WHERE outbox_id=?1 AND runtime_epoch=?2 AND started_at_micros IS NULL",params![lease.outbox_id.to_string(),self.epoch.to_string()],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?)))?;
        let op = load(&tx, parse(&id)?)?;
        let attempt_id = parse(&attempt)?;
        if op.state() != DurableOperationState::DispatchPending
            || op.attempt_identity() != Some(attempt_id)
        {
            return Err(RecoveryError::Denied("operation no longer dispatchable"));
        }
        if valid_approval(&tx, &op, self.epoch, now)? != approval {
            return Err(RecoveryError::Denied("replaced approval"));
        }
        valid_worker(&tx, &op, lease.worker_id, self.epoch, now)?;
        let material = load_material(&tx, op.operation_id())?;
        if digest(&material.payload) != op.arguments_hash() {
            return Err(RecoveryError::Integrity("approved payload changed"));
        }
        let same:bool=tx.query_row("SELECT EXISTS(SELECT 1 FROM outbox_messages WHERE outbox_id=?1 AND payload=?2 AND destination=?3 AND message_kind=?4 AND payload_hash=?5 AND state='leased')",params![lease.outbox_id.to_string(),&material.payload,material.destination.as_str(),material.message_kind.as_str(),op.arguments_hash().to_hex()],|r|r.get(0))?;
        if !same {
            return Err(RecoveryError::Integrity(
                "outbox differs from approved action",
            ));
        }
        tx.execute(
            "UPDATE recovery_attempts SET worker_id=?2,started_at_micros=?3 WHERE outbox_id=?1",
            params![
                lease.outbox_id.to_string(),
                lease.worker_id.to_string(),
                now.get()
            ],
        )?;
        tx.execute(
            "DELETE FROM recovery_claims WHERE outbox_id=?1",
            [lease.outbox_id.to_string()],
        )?;
        tx.execute("UPDATE outbox_messages SET state='attempting',lease_owner=NULL,lease_expires_at_micros=NULL,dispatch_started_at_micros=?2,updated_at_micros=?2 WHERE outbox_id=?1",params![lease.outbox_id.to_string(),now.get()])?;
        transition(
            &tx,
            &op,
            DurableOperationState::Attempting,
            Some(attempt_id),
            now,
            "authorized durable attempt started",
        )?;
        tx.commit()?;
        Ok(DurableSendAttempt {
            outbox_id: lease.outbox_id,
            operation_id: op.operation_id(),
            attempt_id,
            epoch: self.epoch,
            destination: material.destination,
            message_kind: material.message_kind,
            payload: material.payload,
        })
    }
    pub fn record_transport_observation(
        &mut self,
        attempt: DurableSendAttempt,
        result: TransportObservation,
        now: UnixTimestampMicros,
    ) -> Result<u64, RecoveryError> {
        if attempt.epoch != self.epoch {
            return Err(RecoveryError::Denied(
                "late attempt requires evidence reconciliation",
            ));
        }
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, false)?;
        observe_clock(&tx, now)?;
        let op = load(&tx, attempt.operation_id)?;
        if op.state() != DurableOperationState::Attempting
            || op.attempt_identity() != Some(attempt.attempt_id)
        {
            return Err(RecoveryError::Conflict("attempt outcome"));
        }
        let (state, reason) = match result {
            TransportObservation::AcceptedUnverified => (
                DurableOperationState::Accepted,
                "transport acceptance is not a verified effect",
            ),
            TransportObservation::OutcomeUnknown => (
                DurableOperationState::NeedsReconciliation,
                "transport outcome unknown; no automatic resend",
            ),
        };
        tx.execute("UPDATE outbox_messages SET state='completed',completed_at_micros=?2,updated_at_micros=?2 WHERE outbox_id=?1",params![attempt.outbox_id.to_string(),now.get()])?;
        let rev = transition(&tx, &op, state, op.attempt_identity(), now, reason)?;
        tx.commit()?;
        Ok(rev)
    }
}
fn require_revision(op: &DurableOperation, expected: u64) -> Result<(), RecoveryError> {
    if op.revision() != expected {
        return Err(RecoveryError::Conflict("operation revision"));
    }
    Ok(())
}
pub(super) fn parse<T: std::str::FromStr>(value: &str) -> Result<T, RecoveryError> {
    value
        .parse()
        .map_err(|_| RecoveryError::Integrity("stored typed value"))
}
pub(super) fn valid_authority(
    db: &Connection,
    op: &DurableOperation,
    now: UnixTimestampMicros,
) -> Result<i64, RecoveryError> {
    let result:Option<i64>=db.query_row("SELECT a.revision FROM recovery_authorities a JOIN recovery_actions p ON p.operation_id=?1 WHERE a.account_id=?2 AND a.capability_id=?3 AND a.enabled=1 AND a.valid_until_micros>?4 AND a.source_revision=p.source_revision AND p.deadline_micros>?4 AND p.effect!='unknown' AND a.effect=p.effect AND (p.effect='read_only' OR (SELECT required FROM recovery_restore_fence WHERE singleton=1)=0) AND EXISTS(SELECT 1 FROM recovery_evidence_keys k WHERE k.account_id=a.account_id AND k.capability_id=a.capability_id AND k.key_id=a.evidence_key_id AND k.revoked=0) AND EXISTS(SELECT 1 FROM workspace_graph_tasks t JOIN workspace_graphs g ON g.workspace_id=t.workspace_id WHERE t.task_id=?5 AND t.active=1 AND g.privacy_scope=p.privacy_scope)",params![op.operation_id().to_string(),op.account_id().to_string(),op.capability_id().to_string(),now.get(),op.task_id().to_string()],|r|r.get(0)).optional()?;
    result.ok_or(RecoveryError::Denied(
        "authority, deadline, task scope or source precondition changed",
    ))
}
fn valid_approval(
    db: &Connection,
    op: &DurableOperation,
    epoch: Uuid,
    now: UnixTimestampMicros,
) -> Result<String, RecoveryError> {
    let revision = valid_authority(db, op, now)?;
    db.query_row("SELECT p.approval_id FROM recovery_approval_heads h JOIN recovery_approvals p ON p.approval_id=h.approval_id WHERE h.operation_id=?1 AND p.operation_id=?1 AND p.runtime_epoch=?2 AND p.authority_revision=?3 AND p.approved_at_micros<=?4 AND p.expires_at_micros>?4",params![op.operation_id().to_string(),epoch.to_string(),revision,now.get()],|r|r.get(0)).optional()?.ok_or(RecoveryError::Denied("missing, expired or superseded explicit approval"))
}
fn valid_worker(
    db: &Connection,
    op: &DurableOperation,
    worker: WorkerInstanceId,
    epoch: Uuid,
    now: UnixTimestampMicros,
) -> Result<(), RecoveryError> {
    let valid:bool=db.query_row("SELECT EXISTS(SELECT 1 FROM recovery_workers w JOIN recovery_worker_capabilities c ON c.worker_id=w.worker_id WHERE w.worker_id=?1 AND w.runtime_epoch=?2 AND w.task_id=?3 AND w.account_id=?4 AND c.capability_id=?5 AND w.revoked=0 AND w.expires_at_micros>?6)",params![worker.to_string(),epoch.to_string(),op.task_id().to_string(),op.account_id().to_string(),op.capability_id().to_string(),now.get()],|r|r.get(0))?;
    if !valid {
        return Err(RecoveryError::Denied(
            "worker generation, scope, capability or lease",
        ));
    }
    Ok(())
}
struct Material {
    destination: BoundedText<512>,
    message_kind: BoundedText<128>,
    payload: Vec<u8>,
}
fn load_material(db: &Connection, id: OperationId) -> Result<Material, RecoveryError> {
    let size: i64 = db.query_row(
        "SELECT length(payload) FROM recovery_actions WHERE operation_id=?1",
        [id.to_string()],
        |r| r.get(0),
    )?;
    if unsigned(size)? > crate::MAX_OUTBOX_PAYLOAD_BYTES as u64 {
        return Err(RecoveryError::Limit("stored action payload"));
    }
    let (destination, kind, payload): (String, String, Vec<u8>) = db.query_row(
        "SELECT destination,message_kind,payload FROM recovery_actions WHERE operation_id=?1",
        [id.to_string()],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    )?;
    Ok(Material {
        destination: BoundedText::try_new(destination)
            .map_err(|_| RecoveryError::Integrity("destination"))?,
        message_kind: BoundedText::try_new(kind)
            .map_err(|_| RecoveryError::Integrity("message kind"))?,
        payload,
    })
}
