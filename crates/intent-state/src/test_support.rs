use crate::*;
use intent_contracts::*;
use std::{error::Error, fs, path::PathBuf, str::FromStr};
use uuid::Uuid;

pub type TestResult<T = ()> = Result<T, Box<dyn Error>>;

pub struct Profile(PathBuf);
impl Profile {
    pub fn new() -> TestResult<Self> {
        let root = std::env::temp_dir().join(format!("intent-hardening-{}", Uuid::new_v4()));
        fs::create_dir_all(&root)?;
        Ok(Self(root))
    }
    pub fn database(&self) -> PathBuf {
        self.0.join("state.sqlite3")
    }
}
impl Drop for Profile {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

pub fn id<T>(number: u64) -> TestResult<T>
where
    T: FromStr,
    T::Err: Error + 'static,
{
    Ok(format!("018f47f7-5a86-7c00-8000-{number:012x}").parse()?)
}

pub fn time(value: i64) -> TestResult<UnixTimestampMicros> {
    Ok(UnixTimestampMicros::try_new(value)?)
}

pub fn approved(store: &mut StateStore, number: u64) -> TestResult<DurableOperation> {
    let operation = store.create_operation(NewDurableOperation {
        operation_id: id(number)?,
        task_id: id(number + 1000)?,
        action_proposal_id: id(number + 2000)?,
        account_id: id(number + 3000)?,
        capability_id: id(number + 4000)?,
        arguments_hash: ContentHash::from_bytes([42; 32]),
        source_schema: SchemaVersion::V1,
        created_at: time(1)?,
    })?;
    advance(
        store,
        operation.operation_id(),
        DurableOperationState::Approved,
        None,
    )
}

pub fn advance(
    store: &mut StateStore,
    operation_id: OperationId,
    next: DurableOperationState,
    attempt: Option<OperationAttemptId>,
) -> TestResult<DurableOperation> {
    let current = store
        .load_operation(operation_id)?
        .ok_or("missing operation")?;
    Ok(store.transition_operation(
        operation_id,
        OperationTransition {
            expected_revision: current.revision(),
            next_state: next,
            state_detail: None,
            attempt_identity: attempt,
            occurred_at: time(100 + i64::try_from(current.revision())?)?,
        },
    )?)
}
