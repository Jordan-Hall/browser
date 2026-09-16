use crate::{StateError, StateStore};
use intent_contracts::{
    AccountId, ActionProposalId, BoundedText, CapabilityId, ContentHash, OperationAttemptId,
    OperationId, SchemaVersion, TaskId, UnixTimestampMicros,
};
use rusqlite::{OptionalExtension, Transaction, TransactionBehavior, params};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DurableOperationState {
    Prepared,
    Approved,
    DispatchPending,
    Attempting,
    Accepted,
    Verified,
    Failed,
    NeedsReconciliation,
    Cancelled,
    Compensating,
    Compensated,
}

impl DurableOperationState {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prepared => "prepared",
            Self::Approved => "approved",
            Self::DispatchPending => "dispatch_pending",
            Self::Attempting => "attempting",
            Self::Accepted => "accepted",
            Self::Verified => "verified",
            Self::Failed => "failed",
            Self::NeedsReconciliation => "needs_reconciliation",
            Self::Cancelled => "cancelled",
            Self::Compensating => "compensating",
            Self::Compensated => "compensated",
        }
    }

    fn parse(value: &str) -> Result<Self, StateError> {
        match value {
            "prepared" => Ok(Self::Prepared),
            "approved" => Ok(Self::Approved),
            "dispatch_pending" => Ok(Self::DispatchPending),
            "attempting" => Ok(Self::Attempting),
            "accepted" => Ok(Self::Accepted),
            "verified" => Ok(Self::Verified),
            "failed" => Ok(Self::Failed),
            "needs_reconciliation" => Ok(Self::NeedsReconciliation),
            "cancelled" => Ok(Self::Cancelled),
            "compensating" => Ok(Self::Compensating),
            "compensated" => Ok(Self::Compensated),
            other => Err(StateError::InvalidStoredOperation(format!(
                "unknown operation state {other}"
            ))),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurableOperation {
    operation_id: OperationId,
    task_id: TaskId,
    action_proposal_id: ActionProposalId,
    account_id: AccountId,
    capability_id: CapabilityId,
    arguments_hash: ContentHash,
    source_schema: SchemaVersion,
    state: DurableOperationState,
    state_detail: Option<BoundedText<2048>>,
    attempt_identity: Option<OperationAttemptId>,
    revision: u64,
    created_at: UnixTimestampMicros,
    updated_at: UnixTimestampMicros,
}

impl DurableOperation {
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    #[must_use]
    pub const fn task_id(&self) -> TaskId {
        self.task_id
    }

    #[must_use]
    pub const fn action_proposal_id(&self) -> ActionProposalId {
        self.action_proposal_id
    }

    #[must_use]
    pub const fn account_id(&self) -> AccountId {
        self.account_id
    }

    #[must_use]
    pub const fn capability_id(&self) -> CapabilityId {
        self.capability_id
    }

    #[must_use]
    pub const fn arguments_hash(&self) -> ContentHash {
        self.arguments_hash
    }

    #[must_use]
    pub const fn source_schema(&self) -> SchemaVersion {
        self.source_schema
    }

    #[must_use]
    pub const fn state(&self) -> DurableOperationState {
        self.state
    }

    #[must_use]
    pub fn state_detail(&self) -> Option<&BoundedText<2048>> {
        self.state_detail.as_ref()
    }

    #[must_use]
    pub const fn attempt_identity(&self) -> Option<OperationAttemptId> {
        self.attempt_identity
    }

    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
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
pub struct NewDurableOperation {
    pub operation_id: OperationId,
    pub task_id: TaskId,
    pub action_proposal_id: ActionProposalId,
    pub account_id: AccountId,
    pub capability_id: CapabilityId,
    pub arguments_hash: ContentHash,
    pub source_schema: SchemaVersion,
    pub created_at: UnixTimestampMicros,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationTransition {
    pub expected_revision: u64,
    pub next_state: DurableOperationState,
    pub state_detail: Option<BoundedText<2048>>,
    pub attempt_identity: Option<OperationAttemptId>,
    pub occurred_at: UnixTimestampMicros,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationJournalEntry {
    sequence: u64,
    operation_id: OperationId,
    revision: u64,
    from_state: Option<DurableOperationState>,
    to_state: DurableOperationState,
    state_detail: Option<BoundedText<2048>>,
    attempt_identity: Option<OperationAttemptId>,
    occurred_at: UnixTimestampMicros,
}

impl OperationJournalEntry {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    #[must_use]
    pub const fn from_state(&self) -> Option<DurableOperationState> {
        self.from_state
    }

    #[must_use]
    pub const fn to_state(&self) -> DurableOperationState {
        self.to_state
    }

    #[must_use]
    pub fn state_detail(&self) -> Option<&BoundedText<2048>> {
        self.state_detail.as_ref()
    }

    #[must_use]
    pub const fn attempt_identity(&self) -> Option<OperationAttemptId> {
        self.attempt_identity
    }

    #[must_use]
    pub const fn occurred_at(&self) -> UnixTimestampMicros {
        self.occurred_at
    }
}

impl StateStore {
    pub fn create_operation(
        &mut self,
        new: NewDurableOperation,
    ) -> Result<DurableOperation, StateError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;

        transaction.execute(
            r#"
            INSERT INTO durable_operations(
                operation_id, task_id, action_proposal_id, account_id, capability_id,
                arguments_hash, source_schema_major, source_schema_minor, state,
                state_detail, attempt_identity, revision, created_at_micros, updated_at_micros
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'prepared', NULL, NULL, 0, ?9, ?9)
            "#,
            params![
                new.operation_id.to_string(),
                new.task_id.to_string(),
                new.action_proposal_id.to_string(),
                new.account_id.to_string(),
                new.capability_id.to_string(),
                new.arguments_hash.to_hex(),
                i64::from(new.source_schema.major()),
                i64::from(new.source_schema.minor()),
                new.created_at.get(),
            ],
        )?;

        append_journal(
            &transaction,
            new.operation_id,
            0,
            None,
            DurableOperationState::Prepared,
            None,
            None,
            new.created_at,
        )?;
        transaction.commit()?;
        self.load_operation(new.operation_id)?.ok_or_else(|| {
            StateError::InvalidStoredOperation(
                "operation disappeared after successful creation transaction".to_owned(),
            )
        })
    }

    pub fn transition_operation(
        &mut self,
        operation_id: OperationId,
        transition: OperationTransition,
    ) -> Result<DurableOperation, StateError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let current = load_operation_from_connection(&transaction, operation_id)?
            .ok_or(StateError::OperationNotFound(operation_id))?;

        if current.revision != transition.expected_revision {
            return Err(StateError::StaleOperationRevision {
                operation_id,
                expected: transition.expected_revision,
                actual: current.revision,
            });
        }
        validate_transition(current.state, transition.next_state)?;
        validate_attempt_identity(transition.next_state, transition.attempt_identity)?;

        let next_revision = current
            .revision
            .checked_add(1)
            .ok_or_else(|| StateError::InvalidStoredOperation("operation revision overflow".to_owned()))?;
        let changed = transaction.execute(
            r#"
            UPDATE durable_operations
            SET state = ?1, state_detail = ?2, attempt_identity = ?3,
                revision = ?4, updated_at_micros = ?5
            WHERE operation_id = ?6 AND revision = ?7
            "#,
            params![
                transition.next_state.as_str(),
                transition.state_detail.as_ref().map(BoundedText::as_str),
                transition.attempt_identity.map(|id| id.to_string()),
                u64_to_i64(next_revision, "operation revision")?,
                transition.occurred_at.get(),
                operation_id.to_string(),
                u64_to_i64(transition.expected_revision, "expected operation revision")?,
            ],
        )?;
        if changed != 1 {
            return Err(StateError::StaleOperationRevision {
                operation_id,
                expected: transition.expected_revision,
                actual: current.revision,
            });
        }

        append_journal(
            &transaction,
            operation_id,
            next_revision,
            Some(current.state),
            transition.next_state,
            transition.state_detail.as_ref(),
            transition.attempt_identity,
            transition.occurred_at,
        )?;
        transaction.commit()?;
        self.load_operation(operation_id)?.ok_or_else(|| {
            StateError::InvalidStoredOperation(
                "operation disappeared after successful transition transaction".to_owned(),
            )
        })
    }

    pub fn load_operation(
        &self,
        operation_id: OperationId,
    ) -> Result<Option<DurableOperation>, StateError> {
        load_operation_from_connection(&self.connection, operation_id)
    }

    pub fn operation_journal(
        &self,
        operation_id: OperationId,
    ) -> Result<Vec<OperationJournalEntry>, StateError> {
        let mut statement = self.connection.prepare(
            r#"
            SELECT sequence, revision, from_state, to_state, state_detail,
                   attempt_identity, occurred_at_micros
            FROM operation_journal
            WHERE operation_id = ?1
            ORDER BY revision ASC
            "#,
        )?;
        let rows = statement.query_map([operation_id.to_string()], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, i64>(6)?,
            ))
        })?;

        let mut entries = Vec::new();
        for row in rows {
            let (sequence, revision, from_state, to_state, detail, attempt, occurred_at) = row?;
            entries.push(OperationJournalEntry {
                sequence: i64_to_u64(sequence, "journal sequence")?,
                operation_id,
                revision: i64_to_u64(revision, "journal revision")?,
                from_state: from_state
                    .as_deref()
                    .map(DurableOperationState::parse)
                    .transpose()?,
                to_state: DurableOperationState::parse(&to_state)?,
                state_detail: parse_detail(detail)?,
                attempt_identity: parse_optional_id(attempt, "operation attempt id")?,
                occurred_at: UnixTimestampMicros::try_new(occurred_at).map_err(|error| {
                    StateError::InvalidStoredOperation(format!(
                        "invalid journal timestamp: {error}"
                    ))
                })?,
            });
        }
        Ok(entries)
    }
}

fn append_journal(
    transaction: &Transaction<'_>,
    operation_id: OperationId,
    revision: u64,
    from_state: Option<DurableOperationState>,
    to_state: DurableOperationState,
    state_detail: Option<&BoundedText<2048>>,
    attempt_identity: Option<OperationAttemptId>,
    occurred_at: UnixTimestampMicros,
) -> Result<(), StateError> {
    transaction.execute(
        r#"
        INSERT INTO operation_journal(
            operation_id, revision, from_state, to_state, state_detail,
            attempt_identity, occurred_at_micros
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
        params![
            operation_id.to_string(),
            u64_to_i64(revision, "journal revision")?,
            from_state.map(DurableOperationState::as_str),
            to_state.as_str(),
            state_detail.map(BoundedText::as_str),
            attempt_identity.map(|id| id.to_string()),
            occurred_at.get(),
        ],
    )?;
    Ok(())
}

fn load_operation_from_connection(
    connection: &rusqlite::Connection,
    operation_id: OperationId,
) -> Result<Option<DurableOperation>, StateError> {
    let raw = connection
        .query_row(
            r#"
            SELECT task_id, action_proposal_id, account_id, capability_id, arguments_hash,
                   source_schema_major, source_schema_minor, state, state_detail,
                   attempt_identity, revision, created_at_micros, updated_at_micros
            FROM durable_operations
            WHERE operation_id = ?1
            "#,
            [operation_id.to_string()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, Option<String>>(8)?,
                    row.get::<_, Option<String>>(9)?,
                    row.get::<_, i64>(10)?,
                    row.get::<_, i64>(11)?,
                    row.get::<_, i64>(12)?,
                ))
            },
        )
        .optional()?;

    let Some((
        task_id,
        action_proposal_id,
        account_id,
        capability_id,
        arguments_hash,
        schema_major,
        schema_minor,
        state,
        state_detail,
        attempt_identity,
        revision,
        created_at,
        updated_at,
    )) = raw
    else {
        return Ok(None);
    };

    Ok(Some(DurableOperation {
        operation_id,
        task_id: parse_id(&task_id, "task id")?,
        action_proposal_id: parse_id(&action_proposal_id, "action proposal id")?,
        account_id: parse_id(&account_id, "account id")?,
        capability_id: parse_id(&capability_id, "capability id")?,
        arguments_hash: ContentHash::from_hex(&arguments_hash).map_err(|error| {
            StateError::InvalidStoredOperation(format!("invalid arguments hash: {error}"))
        })?,
        source_schema: SchemaVersion::try_new(
            u16::try_from(schema_major).map_err(|_| {
                StateError::InvalidStoredOperation("schema major does not fit u16".to_owned())
            })?,
            u16::try_from(schema_minor).map_err(|_| {
                StateError::InvalidStoredOperation("schema minor does not fit u16".to_owned())
            })?,
        )
        .map_err(|error| {
            StateError::InvalidStoredOperation(format!("invalid source schema: {error}"))
        })?,
        state: DurableOperationState::parse(&state)?,
        state_detail: parse_detail(state_detail)?,
        attempt_identity: parse_optional_id(attempt_identity, "operation attempt id")?,
        revision: i64_to_u64(revision, "operation revision")?,
        created_at: UnixTimestampMicros::try_new(created_at).map_err(|error| {
            StateError::InvalidStoredOperation(format!("invalid created timestamp: {error}"))
        })?,
        updated_at: UnixTimestampMicros::try_new(updated_at).map_err(|error| {
            StateError::InvalidStoredOperation(format!("invalid updated timestamp: {error}"))
        })?,
    }))
}

fn validate_transition(
    from: DurableOperationState,
    to: DurableOperationState,
) -> Result<(), StateError> {
    let allowed = matches!(
        (from, to),
        (DurableOperationState::Prepared, DurableOperationState::Approved)
            | (
                DurableOperationState::Approved,
                DurableOperationState::DispatchPending
            )
            | (
                DurableOperationState::DispatchPending,
                DurableOperationState::Attempting
            )
            | (
                DurableOperationState::Attempting,
                DurableOperationState::Accepted
            )
            | (
                DurableOperationState::Attempting,
                DurableOperationState::NeedsReconciliation
            )
            | (
                DurableOperationState::Accepted,
                DurableOperationState::Verified
            )
            | (
                DurableOperationState::Accepted,
                DurableOperationState::NeedsReconciliation
            )
            | (
                DurableOperationState::NeedsReconciliation,
                DurableOperationState::Accepted
            )
            | (
                DurableOperationState::NeedsReconciliation,
                DurableOperationState::Verified
            )
            | (
                DurableOperationState::NeedsReconciliation,
                DurableOperationState::Failed
            )
            | (DurableOperationState::Approved, DurableOperationState::Cancelled)
            | (
                DurableOperationState::DispatchPending,
                DurableOperationState::Cancelled
            )
            | (DurableOperationState::Attempting, DurableOperationState::Failed)
            | (DurableOperationState::Accepted, DurableOperationState::Failed)
            | (DurableOperationState::Verified, DurableOperationState::Compensating)
            | (
                DurableOperationState::Compensating,
                DurableOperationState::Compensated
            )
            | (
                DurableOperationState::Compensating,
                DurableOperationState::NeedsReconciliation
            )
    );
    if allowed {
        Ok(())
    } else {
        Err(StateError::InvalidOperationTransition { from, to })
    }
}

fn validate_attempt_identity(
    state: DurableOperationState,
    attempt_identity: Option<OperationAttemptId>,
) -> Result<(), StateError> {
    let required = matches!(
        state,
        DurableOperationState::Attempting
            | DurableOperationState::Accepted
            | DurableOperationState::Verified
            | DurableOperationState::NeedsReconciliation
            | DurableOperationState::Failed
            | DurableOperationState::Compensating
            | DurableOperationState::Compensated
    );
    if required && attempt_identity.is_none() {
        return Err(StateError::MissingOperationAttemptIdentity { state });
    }
    Ok(())
}

fn parse_detail(value: Option<String>) -> Result<Option<BoundedText<2048>>, StateError> {
    value
        .map(|value| {
            BoundedText::try_new(value).map_err(|error| {
                StateError::InvalidStoredOperation(format!("invalid state detail: {error}"))
            })
        })
        .transpose()
}

fn parse_id<T>(value: &str, label: &str) -> Result<T, StateError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    T::from_str(value).map_err(|error| {
        StateError::InvalidStoredOperation(format!("invalid {label}: {error}"))
    })
}

fn parse_optional_id<T>(value: Option<String>, label: &str) -> Result<Option<T>, StateError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    value.map(|value| parse_id(&value, label)).transpose()
}

fn i64_to_u64(value: i64, label: &str) -> Result<u64, StateError> {
    u64::try_from(value)
        .map_err(|_| StateError::InvalidStoredOperation(format!("negative {label}")))
}

fn u64_to_i64(value: u64, label: &str) -> Result<i64, StateError> {
    i64::try_from(value)
        .map_err(|_| StateError::InvalidStoredOperation(format!("{label} exceeds SQLite INTEGER")))
}

#[cfg(test)]
mod tests {
    use super::{
        DurableOperationState, NewDurableOperation, OperationTransition,
    };
    use crate::{StateError, StateStore};
    use intent_contracts::{
        AccountId, ActionProposalId, CapabilityId, ContentHash, OperationAttemptId, OperationId,
        SchemaVersion, TaskId, UnixTimestampMicros,
    };
    use std::error::Error;
    use std::str::FromStr;

    fn operation() -> Result<NewDurableOperation, Box<dyn Error>> {
        Ok(NewDurableOperation {
            operation_id: OperationId::from_str("018f47f7-5a86-7c00-8000-000000000821")?,
            task_id: TaskId::from_str("018f47f7-5a86-7c00-8000-000000000822")?,
            action_proposal_id: ActionProposalId::from_str(
                "018f47f7-5a86-7c00-8000-000000000823",
            )?,
            account_id: AccountId::from_str("018f47f7-5a86-7c00-8000-000000000824")?,
            capability_id: CapabilityId::from_str("018f47f7-5a86-7c00-8000-000000000825")?,
            arguments_hash: ContentHash::from_bytes([0x42; 32]),
            source_schema: SchemaVersion::V1,
            created_at: UnixTimestampMicros::try_new(100)?,
        })
    }

    fn attempt() -> Result<OperationAttemptId, Box<dyn Error>> {
        Ok(OperationAttemptId::from_str(
            "018f47f7-5a86-7c00-8000-000000000826",
        )?)
    }

    #[test]
    fn create_and_transition_append_same_revision_journal() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        let created = store.create_operation(operation()?)?;
        assert_eq!(created.state(), DurableOperationState::Prepared);
        assert_eq!(created.revision(), 0);

        let approved = store.transition_operation(
            created.operation_id(),
            OperationTransition {
                expected_revision: 0,
                next_state: DurableOperationState::Approved,
                state_detail: None,
                attempt_identity: None,
                occurred_at: UnixTimestampMicros::try_new(110)?,
            },
        )?;
        assert_eq!(approved.revision(), 1);
        let journal = store.operation_journal(created.operation_id())?;
        assert_eq!(journal.len(), 2);
        assert_eq!(journal[0].revision(), 0);
        assert_eq!(journal[1].revision(), 1);
        assert_eq!(journal[1].from_state(), Some(DurableOperationState::Prepared));
        assert_eq!(journal[1].to_state(), DurableOperationState::Approved);
        Ok(())
    }

    #[test]
    fn stale_revision_is_rejected_without_extra_journal_entry() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        let created = store.create_operation(operation()?)?;
        store.transition_operation(
            created.operation_id(),
            OperationTransition {
                expected_revision: 0,
                next_state: DurableOperationState::Approved,
                state_detail: None,
                attempt_identity: None,
                occurred_at: UnixTimestampMicros::try_new(110)?,
            },
        )?;
        let Err(error) = store.transition_operation(
            created.operation_id(),
            OperationTransition {
                expected_revision: 0,
                next_state: DurableOperationState::Approved,
                state_detail: None,
                attempt_identity: None,
                occurred_at: UnixTimestampMicros::try_new(111)?,
            },
        ) else {
            return Err("stale transition unexpectedly succeeded".into());
        };
        assert!(matches!(error, StateError::StaleOperationRevision { .. }));
        assert_eq!(store.operation_journal(created.operation_id())?.len(), 2);
        Ok(())
    }

    #[test]
    fn dispatch_states_require_durable_attempt_identity() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        let created = store.create_operation(operation()?)?;
        let approved = store.transition_operation(
            created.operation_id(),
            OperationTransition {
                expected_revision: 0,
                next_state: DurableOperationState::Approved,
                state_detail: None,
                attempt_identity: None,
                occurred_at: UnixTimestampMicros::try_new(110)?,
            },
        )?;
        let pending = store.transition_operation(
            created.operation_id(),
            OperationTransition {
                expected_revision: approved.revision(),
                next_state: DurableOperationState::DispatchPending,
                state_detail: None,
                attempt_identity: None,
                occurred_at: UnixTimestampMicros::try_new(120)?,
            },
        )?;
        let Err(error) = store.transition_operation(
            created.operation_id(),
            OperationTransition {
                expected_revision: pending.revision(),
                next_state: DurableOperationState::Attempting,
                state_detail: None,
                attempt_identity: None,
                occurred_at: UnixTimestampMicros::try_new(130)?,
            },
        ) else {
            return Err("attempt without identity unexpectedly succeeded".into());
        };
        assert!(matches!(
            error,
            StateError::MissingOperationAttemptIdentity {
                state: DurableOperationState::Attempting
            }
        ));

        let attempting = store.transition_operation(
            created.operation_id(),
            OperationTransition {
                expected_revision: pending.revision(),
                next_state: DurableOperationState::Attempting,
                state_detail: None,
                attempt_identity: Some(attempt()?),
                occurred_at: UnixTimestampMicros::try_new(131)?,
            },
        )?;
        assert_eq!(attempting.state(), DurableOperationState::Attempting);
        assert_eq!(attempting.attempt_identity(), Some(attempt()?));
        Ok(())
    }
}
