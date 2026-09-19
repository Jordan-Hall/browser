//! Durable accounting for read-only provider recovery, independent of dispatch authority.

use crate::StateStore;
use intent_contracts::{AccountId, BoundedText, ContentHash, TaskId, UnixTimestampMicros};
use intent_recovery::{
    CurrentAuthority, ProviderRecoveryFacts, ProviderRecoveryPlan, ProviderSessionEvidence,
    RecoveryEffect, SourcePrecondition, plan_provider_recovery,
};
use rusqlite::{Connection, TransactionBehavior, params};
use std::{error::Error, fmt};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRecoveryScope {
    pub task_id: TaskId,
    pub account_id: AccountId,
    pub provider: BoundedText<128>,
    pub checkpoint: ContentHash,
}

/// Facts supplied by trusted runtime verifiers. Retry history comes only from the store.
#[derive(Clone, Debug)]
pub struct ProviderRecoveryRequest {
    pub scope: ProviderRecoveryScope,
    pub policy_version: u16,
    pub effect: RecoveryEffect,
    pub authority: CurrentAuthority,
    pub source: SourcePrecondition,
    pub session: Option<ProviderSessionEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderRecoveryAttemptId(Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderRecoveryAction {
    ResumeReadOnlySession,
    ReseedFromDurableCheckpoint,
}

impl ProviderRecoveryAction {
    fn as_str(self) -> &'static str {
        match self {
            Self::ResumeReadOnlySession => "resume_read_only",
            Self::ReseedFromDurableCheckpoint => "reseed",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderRecoveryOutcome {
    Succeeded,
    Failed,
}

impl ProviderRecoveryOutcome {
    fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderRecoveryAttemptState {
    Pending,
    Finished {
        outcome: ProviderRecoveryOutcome,
        recorded_at: UnixTimestampMicros,
    },
}

/// A historical accounting record. It conveys no authority to contact a provider,
/// disclose checkpoint data, resume tools, or execute external effects.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRecoveryAttempt {
    pub id: ProviderRecoveryAttemptId,
    pub scope: ProviderRecoveryScope,
    pub number: u16,
    pub action: ProviderRecoveryAction,
    pub recorded_at: UnixTimestampMicros,
    pub state: ProviderRecoveryAttemptState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderRecoveryAccounting {
    /// The attempt was committed before returning. A retry consumes another attempt.
    Recorded(ProviderRecoveryAttempt),
    /// The planner requires another step. No attempt was consumed.
    Required(ProviderRecoveryPlan),
}

impl StateStore {
    /// Atomically plan and account for one read-only provider recovery attempt.
    ///
    /// The budget covers the entire task/account/provider/checkpoint history, including
    /// pending attempts left by crashes. Reopening the store never resets it. An unfinished
    /// or failed resume selects reseeding on subsequent eligible attempts. No session
    /// reference or credential is persisted, and dispatch state is never changed.
    ///
    /// Callers must bind these facts to the durable task/workspace checkpoint and separately
    /// validate current authority, source access, data destinations, adapter compatibility
    /// and worker epochs immediately before any provider work. A supplied hash is not proof
    /// that the checkpoint or session belongs to this task.
    pub fn record_provider_recovery_attempt(
        &mut self,
        request: &ProviderRecoveryRequest,
        now: UnixTimestampMicros,
    ) -> Result<ProviderRecoveryAccounting, ProviderRecoveryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let scope = &request.scope;
        let (count, resume_failed): (u16, bool) = transaction.query_row(
            "SELECT count(*), COALESCE(MAX(action = 'resume_read_only' AND (outcome IS NULL OR outcome = 'failed')), 0)
             FROM provider_recovery_attempts
             WHERE task_id = ?1 AND account_id = ?2 AND provider = ?3 AND checkpoint_hash = ?4",
            params![scope.task_id.to_string(), scope.account_id.to_string(), scope.provider.as_str(), scope.checkpoint.to_hex()],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;
        let plan = plan_provider_recovery(
            &ProviderRecoveryFacts {
                policy_version: request.policy_version,
                account_id: scope.account_id,
                provider: scope.provider.clone(),
                durable_checkpoint: scope.checkpoint,
                effect: request.effect,
                authority: request.authority,
                source: request.source,
                session: request.session.clone(),
                previous_resume_failed: resume_failed,
                attempts_in_incarnation: count,
            },
            now,
        );
        let action = match plan {
            ProviderRecoveryPlan::ResumeReadOnlySession => {
                ProviderRecoveryAction::ResumeReadOnlySession
            }
            ProviderRecoveryPlan::ReseedFromDurableCheckpoint => {
                ProviderRecoveryAction::ReseedFromDurableCheckpoint
            }
            required => return Ok(ProviderRecoveryAccounting::Required(required)),
        };
        let number = count.checked_add(1).ok_or(ProviderRecoveryError::Corrupt)?;
        let id = ProviderRecoveryAttemptId(Uuid::new_v4());
        transaction.execute(
            "INSERT INTO provider_recovery_attempts
             (attempt_id, task_id, account_id, provider, checkpoint_hash, attempt_number, action, recorded_at_micros)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![id.0.to_string(), scope.task_id.to_string(), scope.account_id.to_string(), scope.provider.as_str(), scope.checkpoint.to_hex(), number, action.as_str(), now.get()],
        )?;
        transaction.commit()?;
        Ok(ProviderRecoveryAccounting::Recorded(
            ProviderRecoveryAttempt {
                id,
                scope: scope.clone(),
                number,
                action,
                recorded_at: now,
                state: ProviderRecoveryAttemptState::Pending,
            },
        ))
    }

    /// Load bounded attempt history in order. Pending records remain visible after crashes.
    pub fn provider_recovery_attempts(
        &self,
        scope: &ProviderRecoveryScope,
    ) -> Result<Vec<ProviderRecoveryAttempt>, ProviderRecoveryError> {
        read_attempts(&self.connection, scope)
    }

    /// Record the first terminal outcome. Repeating that outcome returns its original
    /// timestamp; conflicting outcomes fail without changing the record. Scope must match.
    pub fn finish_provider_recovery_attempt(
        &mut self,
        scope: &ProviderRecoveryScope,
        id: ProviderRecoveryAttemptId,
        outcome: ProviderRecoveryOutcome,
        now: UnixTimestampMicros,
    ) -> Result<ProviderRecoveryAttempt, ProviderRecoveryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut attempt = read_attempts(&transaction, scope)?
            .into_iter()
            .find(|attempt| attempt.id == id)
            .ok_or(ProviderRecoveryError::AttemptNotFound)?;
        match attempt.state {
            ProviderRecoveryAttemptState::Finished {
                outcome: existing, ..
            } => {
                if outcome != existing {
                    return Err(ProviderRecoveryError::ConflictingOutcome);
                }
            }
            ProviderRecoveryAttemptState::Pending => {
                if now < attempt.recorded_at {
                    return Err(ProviderRecoveryError::OutcomePredatesAttempt);
                }
                transaction.execute(
                    "UPDATE provider_recovery_attempts SET outcome = ?1, finished_at_micros = ?2
                     WHERE attempt_id = ?3 AND outcome IS NULL",
                    params![outcome.as_str(), now.get(), id.0.to_string()],
                )?;
                attempt.state = ProviderRecoveryAttemptState::Finished {
                    outcome,
                    recorded_at: now,
                };
            }
        }
        transaction.commit()?;
        Ok(attempt)
    }
}

fn read_attempts(
    connection: &Connection,
    scope: &ProviderRecoveryScope,
) -> Result<Vec<ProviderRecoveryAttempt>, ProviderRecoveryError> {
    let mut statement = connection.prepare(
        "SELECT attempt_id, attempt_number, action, recorded_at_micros, outcome, finished_at_micros
         FROM provider_recovery_attempts
         WHERE task_id = ?1 AND account_id = ?2 AND provider = ?3 AND checkpoint_hash = ?4
         ORDER BY attempt_number",
    )?;
    let mut rows = statement.query(params![
        scope.task_id.to_string(),
        scope.account_id.to_string(),
        scope.provider.as_str(),
        scope.checkpoint.to_hex(),
    ])?;
    let mut attempts = Vec::new();
    while let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        let number: u16 = row.get(1)?;
        let action: String = row.get(2)?;
        let recorded_at = timestamp(row.get(3)?)?;
        let outcome: Option<String> = row.get(4)?;
        let finished_at: Option<i64> = row.get(5)?;
        let state = match (outcome.as_deref(), finished_at) {
            (None, None) => ProviderRecoveryAttemptState::Pending,
            (Some(value), Some(at)) => ProviderRecoveryAttemptState::Finished {
                outcome: match value {
                    "succeeded" => ProviderRecoveryOutcome::Succeeded,
                    "failed" => ProviderRecoveryOutcome::Failed,
                    _ => return Err(ProviderRecoveryError::Corrupt),
                },
                recorded_at: timestamp(at)?,
            },
            _ => return Err(ProviderRecoveryError::Corrupt),
        };
        attempts.push(ProviderRecoveryAttempt {
            id: ProviderRecoveryAttemptId(
                Uuid::parse_str(&id).map_err(|_| ProviderRecoveryError::Corrupt)?,
            ),
            scope: scope.clone(),
            number,
            action: match action.as_str() {
                "resume_read_only" => ProviderRecoveryAction::ResumeReadOnlySession,
                "reseed" => ProviderRecoveryAction::ReseedFromDurableCheckpoint,
                _ => return Err(ProviderRecoveryError::Corrupt),
            },
            recorded_at,
            state,
        });
    }
    Ok(attempts)
}

fn timestamp(value: i64) -> Result<UnixTimestampMicros, ProviderRecoveryError> {
    UnixTimestampMicros::try_new(value).map_err(|_| ProviderRecoveryError::Corrupt)
}

#[derive(Debug)]
pub enum ProviderRecoveryError {
    Sqlite(rusqlite::Error),
    AttemptNotFound,
    ConflictingOutcome,
    OutcomePredatesAttempt,
    Corrupt,
}

impl fmt::Display for ProviderRecoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => write!(f, "provider recovery accounting: {error}"),
            Self::AttemptNotFound => f.write_str("provider recovery attempt not found in scope"),
            Self::ConflictingOutcome => f.write_str("provider recovery outcome is immutable"),
            Self::OutcomePredatesAttempt => {
                f.write_str("provider recovery outcome predates attempt")
            }
            Self::Corrupt => f.write_str("invalid provider recovery accounting record"),
        }
    }
}

impl Error for ProviderRecoveryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Sqlite(error) => Some(error),
            _ => None,
        }
    }
}

impl From<rusqlite::Error> for ProviderRecoveryError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

#[cfg(test)]
mod tests;
