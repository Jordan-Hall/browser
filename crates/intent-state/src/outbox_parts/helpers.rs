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
                   dispatch_started_at_micros, completed_at_micros, failure_detail,
                   created_at_micros, updated_at_micros
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
                    row.get::<_, Option<i64>>(10)?,
                    row.get::<_, Option<String>>(11)?,
                    row.get::<_, i64>(12)?,
                    row.get::<_, i64>(13)?,
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
        completed_at,
        failure_detail,
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
        return Err(OutboxError::PayloadHashMismatch { outbox_id });
    }
    let state = OutboxState::parse(&state)?;
    let completed_at = optional_timestamp(completed_at, "completion")?;
    if matches!(
        state,
        OutboxState::Pending | OutboxState::Leased | OutboxState::Attempting
    ) && (completed_at.is_some() || failure_detail.is_some())
    {
        return Err(OutboxError::InvalidStoredRecord(
            "non-terminal outbox record contains terminal metadata".to_owned(),
        ));
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
        state,
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
            Self::PayloadHashMismatch { outbox_id } => {
                write!(
                    formatter,
                    "outbox payload {outbox_id} failed integrity validation"
                )
            }
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
