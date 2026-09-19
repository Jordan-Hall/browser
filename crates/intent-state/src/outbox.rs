use crate::StateStore;
use intent_contracts::{
    BoundedText, ContentHash, OperationAttemptId, OperationId, OutboxMessageId, UnixTimestampMicros,
};
use rusqlite::{OptionalExtension, Transaction, TransactionBehavior, params};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

pub const MAX_OUTBOX_PAYLOAD_BYTES: usize = 1024 * 1024;
pub const MAX_OUTBOX_CLAIM_BATCH: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutboxState {
    Pending,
    Leased,
    Attempting,
    Completed,
    Failed,
}

impl OutboxState {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Leased => "leased",
            Self::Attempting => "attempting",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }

    fn parse(value: &str) -> Result<Self, OutboxError> {
        match value {
            "pending" => Ok(Self::Pending),
            "leased" => Ok(Self::Leased),
            "attempting" => Ok(Self::Attempting),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            other => Err(OutboxError::InvalidStoredRecord(format!(
                "unknown outbox state {other}"
            ))),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewOutboxMessage {
    pub outbox_id: OutboxMessageId,
    pub operation_id: OperationId,
    pub attempt_identity: OperationAttemptId,
    pub destination: BoundedText<512>,
    pub message_kind: BoundedText<128>,
    pub payload: Vec<u8>,
    pub created_at: UnixTimestampMicros,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutboxMessage {
    outbox_id: OutboxMessageId,
    operation_id: OperationId,
    attempt_identity: OperationAttemptId,
    destination: BoundedText<512>,
    message_kind: BoundedText<128>,
    payload: Vec<u8>,
    payload_hash: ContentHash,
    state: OutboxState,
    lease_owner: Option<BoundedText<128>>,
    lease_expires_at: Option<UnixTimestampMicros>,
    dispatch_started_at: Option<UnixTimestampMicros>,
    created_at: UnixTimestampMicros,
    updated_at: UnixTimestampMicros,
}

impl OutboxMessage {
    #[must_use]
    pub const fn outbox_id(&self) -> OutboxMessageId {
        self.outbox_id
    }

    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    #[must_use]
    pub const fn attempt_identity(&self) -> OperationAttemptId {
        self.attempt_identity
    }

    #[must_use]
    pub fn destination(&self) -> &BoundedText<512> {
        &self.destination
    }

    #[must_use]
    pub fn message_kind(&self) -> &BoundedText<128> {
        &self.message_kind
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    #[must_use]
    pub const fn payload_hash(&self) -> ContentHash {
        self.payload_hash
    }

    #[must_use]
    pub const fn state(&self) -> OutboxState {
        self.state
    }

    #[must_use]
    pub fn lease_owner(&self) -> Option<&BoundedText<128>> {
        self.lease_owner.as_ref()
    }

    #[must_use]
    pub const fn lease_expires_at(&self) -> Option<UnixTimestampMicros> {
        self.lease_expires_at
    }

    #[must_use]
    pub const fn dispatch_started_at(&self) -> Option<UnixTimestampMicros> {
        self.dispatch_started_at
    }

    #[must_use]
    pub const fn created_at(&self) -> UnixTimestampMicros {
        self.created_at
    }

    #[must_use]
    pub const fn updated_at(&self) -> UnixTimestampMicros {
        self.updated_at
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutboxClaim {
    outbox_id: OutboxMessageId,
    operation_id: OperationId,
    attempt_identity: OperationAttemptId,
    lease_expires_at: UnixTimestampMicros,
}

impl OutboxClaim {
    #[must_use]
    pub const fn outbox_id(&self) -> OutboxMessageId {
        self.outbox_id
    }

    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    #[must_use]
    pub const fn attempt_identity(&self) -> OperationAttemptId {
        self.attempt_identity
    }

    #[must_use]
    pub const fn lease_expires_at(&self) -> UnixTimestampMicros {
        self.lease_expires_at
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatchAttempt {
    outbox_id: OutboxMessageId,
    operation_id: OperationId,
    attempt_identity: OperationAttemptId,
    destination: BoundedText<512>,
    message_kind: BoundedText<128>,
    payload: Vec<u8>,
    payload_hash: ContentHash,
}

impl DispatchAttempt {
    #[must_use]
    pub const fn outbox_id(&self) -> OutboxMessageId {
        self.outbox_id
    }

    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    #[must_use]
    pub const fn attempt_identity(&self) -> OperationAttemptId {
        self.attempt_identity
    }

    #[must_use]
    pub fn destination(&self) -> &BoundedText<512> {
        &self.destination
    }

    #[must_use]
    pub fn message_kind(&self) -> &BoundedText<128> {
        &self.message_kind
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    #[must_use]
    pub const fn payload_hash(&self) -> ContentHash {
        self.payload_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DispatchResult {
    Accepted,
    Ambiguous(BoundedText<2048>),
    Rejected(BoundedText<2048>),
}

impl StateStore {
    pub(crate) fn stage_outbox(
        &mut self,
        new: NewOutboxMessage,
        expected_operation_revision: u64,
    ) -> Result<OutboxMessage, OutboxError> {
        validate_payload(&new.payload)?;
        let payload_hash = hash_payload(&new.payload);
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let managed: bool = transaction.query_row(
            "SELECT EXISTS(SELECT 1 FROM recovery_actions WHERE operation_id=?1)",
            [new.operation_id.to_string()],
            |row| row.get(0),
        )?;
        if managed {
            return Err(OutboxError::RecoveryRequired);
        }
        let (operation_state, actual_revision): (String, i64) = transaction
            .query_row(
                "SELECT state, revision FROM durable_operations WHERE operation_id = ?1",
                [new.operation_id.to_string()],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?
            .ok_or(OutboxError::OperationNotFound(new.operation_id))?;
        let actual_revision = nonnegative_u64(actual_revision, "operation revision")?;
        if actual_revision != expected_operation_revision {
            return Err(OutboxError::StaleOperationRevision {
                operation_id: new.operation_id,
                expected: expected_operation_revision,
                actual: actual_revision,
            });
        }
        if operation_state != "approved" {
            return Err(OutboxError::OperationNotDispatchable {
                operation_id: new.operation_id,
                state: operation_state,
            });
        }
        let next_revision = expected_operation_revision
            .checked_add(1)
            .ok_or(OutboxError::CounterOverflow("operation revision"))?;

        transaction.execute(
            "UPDATE durable_operations SET state = 'dispatch_pending', revision = ?1, updated_at_micros = ?2, attempt_identity = ?5 WHERE operation_id = ?3 AND revision = ?4",
            params![
                to_sql_i64(next_revision, "operation revision")?,
                new.created_at.get(),
                new.operation_id.to_string(),
                to_sql_i64(expected_operation_revision, "expected operation revision")?,
                new.attempt_identity.to_string(),
            ],
        )?;
        append_operation_journal(
            &transaction,
            OperationJournalWrite {
                operation_id: new.operation_id,
                revision: next_revision,
                from_state: "approved",
                to_state: "dispatch_pending",
                attempt_identity: Some(new.attempt_identity),
                detail: None,
                occurred_at: new.created_at,
            },
        )?;
        transaction.execute(
            r#"
            INSERT INTO outbox_messages(
                outbox_id, operation_id, attempt_identity, destination, message_kind,
                payload, payload_hash, state, created_at_micros, updated_at_micros
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'pending', ?8, ?8)
            "#,
            params![
                new.outbox_id.to_string(),
                new.operation_id.to_string(),
                new.attempt_identity.to_string(),
                new.destination.as_str(),
                new.message_kind.as_str(),
                new.payload,
                payload_hash.to_hex(),
                new.created_at.get(),
            ],
        )?;
        transaction.commit()?;
        self.load_outbox(new.outbox_id)?.ok_or_else(|| {
            OutboxError::InvalidStoredRecord(
                "outbox disappeared after successful staging transaction".to_owned(),
            )
        })
    }

    pub fn claim_outbox(
        &mut self,
        lease_owner: BoundedText<128>,
        now: UnixTimestampMicros,
        lease_expires_at: UnixTimestampMicros,
        limit: usize,
    ) -> Result<Vec<OutboxClaim>, OutboxError> {
        if lease_expires_at.get() <= now.get() {
            return Err(OutboxError::InvalidLeaseWindow);
        }
        let limit = limit.min(MAX_OUTBOX_CLAIM_BATCH);
        if limit == 0 {
            return Ok(Vec::new());
        }
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut ids = Vec::new();
        {
            let mut statement = transaction.prepare(
                r#"
                SELECT outbox.outbox_id
                FROM outbox_messages AS outbox
                JOIN durable_operations AS operation ON operation.operation_id = outbox.operation_id
                WHERE operation.state = 'dispatch_pending'
                  AND NOT EXISTS (SELECT 1 FROM recovery_actions a WHERE a.operation_id=operation.operation_id)
                  AND (outbox.state = 'pending'
                    OR (outbox.state = 'leased' AND outbox.lease_expires_at_micros <= ?1))
                ORDER BY outbox.created_at_micros ASC, outbox.outbox_id ASC
                LIMIT ?2
                "#,
            )?;
            let rows = statement.query_map(
                params![now.get(), to_sql_i64(limit as u64, "claim limit")?],
                |row| row.get::<_, String>(0),
            )?;
            for row in rows {
                ids.push(row?);
            }
        }
        for id in &ids {
            transaction.execute(
                r#"
                UPDATE outbox_messages
                SET state = 'leased', lease_owner = ?1, lease_expires_at_micros = ?2,
                    updated_at_micros = ?3
                WHERE outbox_id = ?4
                "#,
                params![lease_owner.as_str(), lease_expires_at.get(), now.get(), id],
            )?;
        }
        transaction.commit()?;

        let mut claimed = Vec::with_capacity(ids.len());
        for id in ids {
            let id = parse_id::<OutboxMessageId>(&id, "outbox id")?;
            let message = self.load_outbox(id)?.ok_or_else(|| {
                OutboxError::InvalidStoredRecord("claimed outbox row disappeared".to_owned())
            })?;
            let lease_expires_at = message.lease_expires_at.ok_or_else(|| {
                OutboxError::InvalidStoredRecord(
                    "claimed outbox row is missing its lease expiry".to_owned(),
                )
            })?;
            claimed.push(OutboxClaim {
                outbox_id: message.outbox_id,
                operation_id: message.operation_id,
                attempt_identity: message.attempt_identity,
                lease_expires_at,
            });
        }
        Ok(claimed)
    }

    pub fn begin_dispatch(
        &mut self,
        outbox_id: OutboxMessageId,
        lease_owner: &BoundedText<128>,
        now: UnixTimestampMicros,
    ) -> Result<DispatchAttempt, OutboxError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        if transaction.path() != Some("") {
            return Err(OutboxError::RecoveryRequired);
        }
        crate::runtime_gate::require_dispatch(&transaction, self.runtime_epoch)?;
        let raw = load_outbox_from_connection(&transaction, outbox_id)?
            .ok_or(OutboxError::OutboxNotFound(outbox_id))?;
        if raw.state != OutboxState::Leased
            || raw.lease_owner.as_ref() != Some(lease_owner)
            || raw
                .lease_expires_at
                .is_none_or(|expires| expires.get() <= now.get())
        {
            return Err(OutboxError::LeaseNotHeld(outbox_id));
        }
        let (operation_state, revision, active_attempt): (String, i64, Option<String>) = transaction.query_row(
            "SELECT state, revision, attempt_identity FROM durable_operations WHERE operation_id = ?1",
            [raw.operation_id.to_string()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;
        let expected_attempt = raw.attempt_identity.to_string();
        if operation_state != "dispatch_pending"
            || active_attempt.as_deref() != Some(expected_attempt.as_str())
        {
            return Err(OutboxError::OperationNotDispatchable {
                operation_id: raw.operation_id,
                state: operation_state,
            });
        }
        let revision = nonnegative_u64(revision, "operation revision")?;
        let next_revision = revision
            .checked_add(1)
            .ok_or(OutboxError::CounterOverflow("operation revision"))?;

        transaction.execute(
            r#"
            UPDATE outbox_messages
            SET state = 'attempting', lease_owner = NULL, lease_expires_at_micros = NULL,
                dispatch_started_at_micros = ?1, updated_at_micros = ?1
            WHERE outbox_id = ?2
            "#,
            params![now.get(), outbox_id.to_string()],
        )?;
        transaction.execute(
            r#"
            UPDATE durable_operations
            SET state = 'attempting', attempt_identity = ?1, revision = ?2,
                updated_at_micros = ?3
            WHERE operation_id = ?4 AND revision = ?5
            "#,
            params![
                raw.attempt_identity.to_string(),
                to_sql_i64(next_revision, "operation revision")?,
                now.get(),
                raw.operation_id.to_string(),
                to_sql_i64(revision, "expected operation revision")?,
            ],
        )?;
        append_operation_journal(
            &transaction,
            OperationJournalWrite {
                operation_id: raw.operation_id,
                revision: next_revision,
                from_state: "dispatch_pending",
                to_state: "attempting",
                attempt_identity: Some(raw.attempt_identity),
                detail: None,
                occurred_at: now,
            },
        )?;
        transaction.commit()?;

        Ok(DispatchAttempt {
            outbox_id: raw.outbox_id,
            operation_id: raw.operation_id,
            attempt_identity: raw.attempt_identity,
            destination: raw.destination,
            message_kind: raw.message_kind,
            payload: raw.payload,
            payload_hash: raw.payload_hash,
        })
    }

    pub fn record_dispatch_result(
        &mut self,
        outbox_id: OutboxMessageId,
        result: DispatchResult,
        occurred_at: UnixTimestampMicros,
    ) -> Result<(), OutboxError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let raw = load_outbox_from_connection(&transaction, outbox_id)?
            .ok_or(OutboxError::OutboxNotFound(outbox_id))?;
        let managed: bool = transaction.query_row(
            "SELECT EXISTS(SELECT 1 FROM recovery_actions WHERE operation_id=?1)",
            [raw.operation_id.to_string()],
            |row| row.get(0),
        )?;
        if managed {
            return Err(OutboxError::RecoveryRequired);
        }
        if raw.state != OutboxState::Attempting {
            return Err(OutboxError::InvalidOutboxTransition {
                outbox_id,
                state: raw.state,
            });
        }
        let (operation_state, revision, active_attempt): (String, i64, Option<String>) = transaction.query_row(
            "SELECT state, revision, attempt_identity FROM durable_operations WHERE operation_id = ?1",
            [raw.operation_id.to_string()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;
        let expected_attempt = raw.attempt_identity.to_string();
        if operation_state != "attempting"
            || active_attempt.as_deref() != Some(expected_attempt.as_str())
        {
            return Err(OutboxError::OperationNotDispatchable {
                operation_id: raw.operation_id,
                state: operation_state,
            });
        }
        let revision = nonnegative_u64(revision, "operation revision")?;
        let next_revision = revision
            .checked_add(1)
            .ok_or(OutboxError::CounterOverflow("operation revision"))?;
        let (outbox_state, operation_state, detail) = match &result {
            DispatchResult::Accepted => ("completed", "accepted", None),
            DispatchResult::Ambiguous(detail) => {
                ("failed", "needs_reconciliation", Some(detail.as_str()))
            }
            DispatchResult::Rejected(detail) => ("failed", "failed", Some(detail.as_str())),
        };
        transaction.execute(
            r#"
            UPDATE outbox_messages
            SET state = ?1, completed_at_micros = ?2, failure_detail = ?3,
                updated_at_micros = ?2
            WHERE outbox_id = ?4
            "#,
            params![
                outbox_state,
                occurred_at.get(),
                detail,
                outbox_id.to_string()
            ],
        )?;
        transaction.execute(
            r#"
            UPDATE durable_operations
            SET state = ?1, state_detail = ?2, revision = ?3, updated_at_micros = ?4
            WHERE operation_id = ?5 AND revision = ?6
            "#,
            params![
                operation_state,
                detail,
                to_sql_i64(next_revision, "operation revision")?,
                occurred_at.get(),
                raw.operation_id.to_string(),
                to_sql_i64(revision, "expected operation revision")?,
            ],
        )?;
        append_operation_journal(
            &transaction,
            OperationJournalWrite {
                operation_id: raw.operation_id,
                revision: next_revision,
                from_state: "attempting",
                to_state: operation_state,
                attempt_identity: Some(raw.attempt_identity),
                detail,
                occurred_at,
            },
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub(crate) fn load_outbox(
        &self,
        outbox_id: OutboxMessageId,
    ) -> Result<Option<OutboxMessage>, OutboxError> {
        load_outbox_from_connection(&self.connection, outbox_id)
    }
}

struct OperationJournalWrite<'a> {
    operation_id: OperationId,
    revision: u64,
    from_state: &'a str,
    to_state: &'a str,
    attempt_identity: Option<OperationAttemptId>,
    detail: Option<&'a str>,
    occurred_at: UnixTimestampMicros,
}

fn append_operation_journal(
    transaction: &Transaction<'_>,
    entry: OperationJournalWrite<'_>,
) -> Result<(), OutboxError> {
    transaction.execute(
        r#"
        INSERT INTO operation_journal(
            operation_id, revision, from_state, to_state, state_detail,
            attempt_identity, occurred_at_micros
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
        params![
            entry.operation_id.to_string(),
            to_sql_i64(entry.revision, "journal revision")?,
            entry.from_state,
            entry.to_state,
            entry.detail,
            entry.attempt_identity.map(|id| id.to_string()),
            entry.occurred_at.get(),
        ],
    )?;
    Ok(())
}

fn load_outbox_from_connection(
    connection: &rusqlite::Connection,
    outbox_id: OutboxMessageId,
) -> Result<Option<OutboxMessage>, OutboxError> {
    let managed: bool = connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM recovery_attempts WHERE outbox_id=?1)",
        [outbox_id.to_string()],
        |row| row.get(0),
    )?;
    if managed {
        return Err(OutboxError::RecoveryRequired);
    }
    let raw = connection
        .query_row(
            r#"
            SELECT operation_id, attempt_identity, destination, message_kind, payload,
                   payload_hash, state, lease_owner, lease_expires_at_micros,
                   dispatch_started_at_micros, created_at_micros, updated_at_micros
            FROM outbox_messages WHERE outbox_id = ?1
            "#,
            [outbox_id.to_string()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, Vec<u8>>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, Option<String>>(7)?,
                    row.get::<_, Option<i64>>(8)?,
                    row.get::<_, Option<i64>>(9)?,
                    row.get::<_, i64>(10)?,
                    row.get::<_, i64>(11)?,
                ))
            },
        )
        .optional()?;
    let Some((
        operation_id,
        attempt_identity,
        destination,
        message_kind,
        payload,
        payload_hash,
        state,
        lease_owner,
        lease_expires_at,
        dispatch_started_at,
        created_at,
        updated_at,
    )) = raw
    else {
        return Ok(None);
    };
    let stored_hash = ContentHash::from_hex(&payload_hash).map_err(|error| {
        OutboxError::InvalidStoredRecord(format!("invalid payload hash: {error}"))
    })?;
    let actual_hash = hash_payload(&payload);
    if stored_hash != actual_hash {
        return Err(OutboxError::PayloadHashMismatch {
            outbox_id,
            expected: stored_hash,
            actual: actual_hash,
        });
    }

    Ok(Some(OutboxMessage {
        outbox_id,
        operation_id: parse_id(&operation_id, "operation id")?,
        attempt_identity: parse_id(&attempt_identity, "attempt identity")?,
        destination: BoundedText::try_new(destination).map_err(|error| {
            OutboxError::InvalidStoredRecord(format!("invalid destination: {error}"))
        })?,
        message_kind: BoundedText::try_new(message_kind).map_err(|error| {
            OutboxError::InvalidStoredRecord(format!("invalid message kind: {error}"))
        })?,
        payload,
        payload_hash: stored_hash,
        state: OutboxState::parse(&state)?,
        lease_owner: lease_owner
            .map(BoundedText::try_new)
            .transpose()
            .map_err(|error| {
                OutboxError::InvalidStoredRecord(format!("invalid lease owner: {error}"))
            })?,
        lease_expires_at: optional_timestamp(lease_expires_at, "lease expiry")?,
        dispatch_started_at: optional_timestamp(dispatch_started_at, "dispatch start")?,
        created_at: timestamp(created_at, "created at")?,
        updated_at: timestamp(updated_at, "updated at")?,
    }))
}

fn validate_payload(payload: &[u8]) -> Result<(), OutboxError> {
    if payload.len() > MAX_OUTBOX_PAYLOAD_BYTES {
        return Err(OutboxError::PayloadTooLarge(payload.len()));
    }
    Ok(())
}

#[must_use]
fn hash_payload(payload: &[u8]) -> ContentHash {
    let digest = Sha256::digest(payload);
    let mut bytes = [0_u8; 32];
    bytes.copy_from_slice(&digest);
    ContentHash::from_bytes(bytes)
}

fn parse_id<T>(value: &str, label: &str) -> Result<T, OutboxError>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    T::from_str(value)
        .map_err(|error| OutboxError::InvalidStoredRecord(format!("invalid {label}: {error}")))
}

fn timestamp(value: i64, label: &str) -> Result<UnixTimestampMicros, OutboxError> {
    UnixTimestampMicros::try_new(value)
        .map_err(|error| OutboxError::InvalidStoredRecord(format!("invalid {label}: {error}")))
}

fn optional_timestamp(
    value: Option<i64>,
    label: &str,
) -> Result<Option<UnixTimestampMicros>, OutboxError> {
    value.map(|value| timestamp(value, label)).transpose()
}

fn nonnegative_u64(value: i64, label: &'static str) -> Result<u64, OutboxError> {
    u64::try_from(value).map_err(|_| OutboxError::InvalidStoredRecord(format!("negative {label}")))
}

fn to_sql_i64(value: u64, label: &'static str) -> Result<i64, OutboxError> {
    i64::try_from(value).map_err(|_| OutboxError::CounterOverflow(label))
}

#[derive(Debug)]
pub enum OutboxError {
    RecoveryRequired,
    Sqlite(rusqlite::Error),
    OperationNotFound(OperationId),
    OutboxNotFound(OutboxMessageId),
    StaleOperationRevision {
        operation_id: OperationId,
        expected: u64,
        actual: u64,
    },
    OperationNotDispatchable {
        operation_id: OperationId,
        state: String,
    },
    PayloadTooLarge(usize),
    PayloadHashMismatch {
        outbox_id: OutboxMessageId,
        expected: ContentHash,
        actual: ContentHash,
    },
    InvalidLeaseWindow,
    LeaseNotHeld(OutboxMessageId),
    InvalidOutboxTransition {
        outbox_id: OutboxMessageId,
        state: OutboxState,
    },
    CounterOverflow(&'static str),
    InvalidStoredRecord(String),
}

impl fmt::Display for OutboxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RecoveryRequired => formatter.write_str(
                "dispatch blocked: current runtime recovery and authority revalidation required",
            ),
            Self::Sqlite(error) => write!(formatter, "SQLite outbox error: {error}"),
            Self::OperationNotFound(id) => write!(formatter, "operation {id} does not exist"),
            Self::OutboxNotFound(id) => write!(formatter, "outbox message {id} does not exist"),
            Self::StaleOperationRevision {
                operation_id,
                expected,
                actual,
            } => write!(
                formatter,
                "operation {operation_id} revision conflict: expected {expected}, actual {actual}"
            ),
            Self::OperationNotDispatchable {
                operation_id,
                state,
            } => write!(
                formatter,
                "operation {operation_id} cannot dispatch from state {state}"
            ),
            Self::PayloadTooLarge(size) => write!(
                formatter,
                "outbox payload is {size} bytes; maximum is {MAX_OUTBOX_PAYLOAD_BYTES}"
            ),
            Self::PayloadHashMismatch {
                outbox_id,
                expected,
                actual,
            } => write!(
                formatter,
                "outbox payload {outbox_id} hash mismatch: expected {expected}, actual {actual}"
            ),
            Self::InvalidLeaseWindow => formatter.write_str("outbox lease must expire after now"),
            Self::LeaseNotHeld(id) => write!(formatter, "outbox lease is not held for {id}"),
            Self::InvalidOutboxTransition { outbox_id, state } => write!(
                formatter,
                "outbox {outbox_id} cannot transition from {}",
                state.as_str()
            ),
            Self::CounterOverflow(label) => write!(formatter, "{label} exceeds storage range"),
            Self::InvalidStoredRecord(detail) => {
                write!(formatter, "invalid stored outbox record: {detail}")
            }
        }
    }
}

impl Error for OutboxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Sqlite(error) => Some(error),
            _ => None,
        }
    }
}

impl From<rusqlite::Error> for OutboxError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{DispatchResult, NewOutboxMessage, OutboxError, OutboxState};
    use crate::{DurableOperationState, NewDurableOperation, OperationTransition, StateStore};
    use intent_contracts::{
        AccountId, ActionProposalId, BoundedText, CapabilityId, ContentHash, OperationAttemptId,
        OperationId, OutboxMessageId, SchemaVersion, TaskId, UnixTimestampMicros,
    };
    use std::error::Error;
    use std::str::FromStr;

    fn operation_id() -> Result<OperationId, Box<dyn Error>> {
        Ok(OperationId::from_str(
            "018f47f7-5a86-7c00-8000-000000000831",
        )?)
    }

    fn attempt_id() -> Result<OperationAttemptId, Box<dyn Error>> {
        Ok(OperationAttemptId::from_str(
            "018f47f7-5a86-7c00-8000-000000000832",
        )?)
    }

    fn outbox_id() -> Result<OutboxMessageId, Box<dyn Error>> {
        Ok(OutboxMessageId::from_str(
            "018f47f7-5a86-7c00-8000-000000000833",
        )?)
    }

    fn approved_operation(store: &mut StateStore) -> Result<(), Box<dyn Error>> {
        let operation = store.create_operation(NewDurableOperation {
            operation_id: operation_id()?,
            task_id: TaskId::from_str("018f47f7-5a86-7c00-8000-000000000834")?,
            action_proposal_id: ActionProposalId::from_str("018f47f7-5a86-7c00-8000-000000000835")?,
            account_id: AccountId::from_str("018f47f7-5a86-7c00-8000-000000000836")?,
            capability_id: CapabilityId::from_str("018f47f7-5a86-7c00-8000-000000000837")?,
            arguments_hash: ContentHash::from_bytes([0x83; 32]),
            source_schema: SchemaVersion::V1,
            created_at: UnixTimestampMicros::try_new(100)?,
        })?;
        store.transition_operation(
            operation.operation_id(),
            OperationTransition {
                expected_revision: operation.revision(),
                next_state: DurableOperationState::Approved,
                state_detail: None,
                attempt_identity: None,
                occurred_at: UnixTimestampMicros::try_new(110)?,
            },
        )?;
        Ok(())
    }

    fn message() -> Result<NewOutboxMessage, Box<dyn Error>> {
        Ok(NewOutboxMessage {
            outbox_id: outbox_id()?,
            operation_id: operation_id()?,
            attempt_identity: attempt_id()?,
            destination: BoundedText::try_new("connector://checkout")?,
            message_kind: BoundedText::try_new("commit")?,
            payload: br#"{"cart":"stable"}"#.to_vec(),
            created_at: UnixTimestampMicros::try_new(120)?,
        })
    }

    #[test]
    fn staging_atomically_marks_operation_pending_and_writes_outbox() -> Result<(), Box<dyn Error>>
    {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        let staged = store.stage_outbox(message()?, 1)?;
        assert_eq!(staged.state(), OutboxState::Pending);
        assert_eq!(
            store
                .load_operation(operation_id()?)?
                .ok_or("operation missing")?
                .state(),
            DurableOperationState::DispatchPending
        );
        assert_eq!(store.operation_journal(operation_id()?)?.len(), 3);
        Ok(())
    }

    #[test]
    fn only_lease_holder_can_begin_external_attempt() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        store.stage_outbox(message()?, 1)?;
        let owner = BoundedText::try_new("dispatcher-a")?;
        let claimed = store.claim_outbox(
            owner.clone(),
            UnixTimestampMicros::try_new(130)?,
            UnixTimestampMicros::try_new(230)?,
            1,
        )?;
        assert_eq!(claimed.len(), 1);
        assert_eq!(claimed[0].outbox_id(), outbox_id()?);
        assert_eq!(claimed[0].operation_id(), operation_id()?);
        assert_eq!(claimed[0].attempt_identity(), attempt_id()?);
        assert_eq!(
            claimed[0].lease_expires_at(),
            UnixTimestampMicros::try_new(230)?
        );
        let wrong_owner = BoundedText::try_new("dispatcher-b")?;
        let Err(error) = store.begin_dispatch(
            outbox_id()?,
            &wrong_owner,
            UnixTimestampMicros::try_new(140)?,
        ) else {
            return Err("wrong lease owner unexpectedly began dispatch".into());
        };
        assert!(matches!(error, OutboxError::LeaseNotHeld(_)));
        let attempt =
            store.begin_dispatch(outbox_id()?, &owner, UnixTimestampMicros::try_new(140)?)?;
        assert_eq!(attempt.attempt_identity(), attempt_id()?);
        assert_eq!(
            store
                .load_operation(operation_id()?)?
                .ok_or("operation missing")?
                .state(),
            DurableOperationState::Attempting
        );
        Ok(())
    }

    #[test]
    fn ambiguous_dispatch_becomes_reconciliation_not_success() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        store.stage_outbox(message()?, 1)?;
        let owner = BoundedText::try_new("dispatcher")?;
        store.claim_outbox(
            owner.clone(),
            UnixTimestampMicros::try_new(130)?,
            UnixTimestampMicros::try_new(230)?,
            1,
        )?;
        store.begin_dispatch(outbox_id()?, &owner, UnixTimestampMicros::try_new(140)?)?;
        store.record_dispatch_result(
            outbox_id()?,
            DispatchResult::Ambiguous(BoundedText::try_new("connection reset after write")?),
            UnixTimestampMicros::try_new(150)?,
        )?;
        assert_eq!(
            store
                .load_operation(operation_id()?)?
                .ok_or("operation missing")?
                .state(),
            DurableOperationState::NeedsReconciliation
        );
        Ok(())
    }
}
