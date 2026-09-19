use super::*;
use crate::test_support::{Profile, TestResult, id, time};
use intent_recovery::{
    MAX_PROVIDER_RECOVERY_ATTEMPTS, RECOVERY_POLICY_VERSION, SessionResumeCapability,
};
use std::sync::{Arc, Barrier};

fn request() -> TestResult<ProviderRecoveryRequest> {
    let scope = ProviderRecoveryScope {
        task_id: id(1)?,
        account_id: id(2)?,
        provider: BoundedText::try_new("fixture-provider")?,
        checkpoint: ContentHash::from_bytes([3; 32]),
    };
    Ok(ProviderRecoveryRequest {
        session: Some(ProviderSessionEvidence {
            account_id: scope.account_id,
            provider: scope.provider.clone(),
            session_reference: BoundedText::try_new("secret-reference-never-persisted")?,
            checkpoint_hash: scope.checkpoint,
            valid_until: time(1000)?,
            capability: SessionResumeCapability::ReadOnlyVersion1,
        }),
        scope,
        policy_version: RECOVERY_POLICY_VERSION,
        effect: RecoveryEffect::ReadOnly,
        authority: CurrentAuthority::Current,
        source: SourcePrecondition::Fresh,
    })
}

fn record(
    store: &mut StateStore,
    request: &ProviderRecoveryRequest,
) -> TestResult<ProviderRecoveryAttempt> {
    match store.record_provider_recovery_attempt(request, time(10)?)? {
        ProviderRecoveryAccounting::Recorded(attempt) => Ok(attempt),
        other => Err(format!("unexpected accounting result: {other:?}").into()),
    }
}

#[test]
fn successful_resume_and_reseed_are_durable_without_enabling_dispatch() -> TestResult {
    let profile = Profile::new()?;
    let mut store = StateStore::open(profile.database())?;
    let mut request = request()?;
    let before = store.dispatch_status()?;
    assert!(!before.enabled);
    let resume = record(&mut store, &request)?;
    assert_eq!(resume.action, ProviderRecoveryAction::ResumeReadOnlySession);
    assert_eq!(resume.number, 1);
    assert_eq!(resume.state, ProviderRecoveryAttemptState::Pending);
    let finished = store.finish_provider_recovery_attempt(
        &request.scope,
        resume.id,
        ProviderRecoveryOutcome::Succeeded,
        time(20)?,
    )?;
    assert_eq!(
        finished.state,
        ProviderRecoveryAttemptState::Finished {
            outcome: ProviderRecoveryOutcome::Succeeded,
            recorded_at: time(20)?,
        }
    );
    assert_eq!(store.dispatch_status()?, before);
    drop(store);
    let mut store = StateStore::open(profile.database())?;
    assert_eq!(
        store.provider_recovery_attempts(&request.scope)?,
        vec![finished]
    );
    request.session = None;
    let reseed = record(&mut store, &request)?;
    assert_eq!(
        reseed.action,
        ProviderRecoveryAction::ReseedFromDurableCheckpoint
    );
    assert_eq!(reseed.number, 2);
    store.finish_provider_recovery_attempt(
        &request.scope,
        reseed.id,
        ProviderRecoveryOutcome::Succeeded,
        time(30)?,
    )?;
    assert_eq!(store.dispatch_status()?, before);
    let sql: String = store.connection.query_row(
        "SELECT sql FROM sqlite_schema WHERE name = 'provider_recovery_attempts'",
        [],
        |row| row.get(0),
    )?;
    assert!(!sql.contains("session_reference"));
    Ok(())
}

#[test]
fn failed_and_unfinished_resume_reseed_and_budget_survives_every_reopen() -> TestResult {
    for finish in [false, true] {
        let profile = Profile::new()?;
        let request = request()?;
        for index in 1..=MAX_PROVIDER_RECOVERY_ATTEMPTS {
            let mut store = StateStore::open(profile.database())?;
            let attempt = record(&mut store, &request)?;
            assert_eq!(attempt.number, index);
            assert_eq!(
                attempt.action,
                if index == 1 {
                    ProviderRecoveryAction::ResumeReadOnlySession
                } else {
                    ProviderRecoveryAction::ReseedFromDurableCheckpoint
                }
            );
            if finish {
                store.finish_provider_recovery_attempt(
                    &request.scope,
                    attempt.id,
                    ProviderRecoveryOutcome::Failed,
                    time(20)?,
                )?;
            }
        }
        let mut store = StateStore::open(profile.database())?;
        for _ in 0..3 {
            assert_eq!(
                store.record_provider_recovery_attempt(&request, time(30)?)?,
                ProviderRecoveryAccounting::Required(ProviderRecoveryPlan::ManualRecoveryRequired)
            );
        }
        assert_eq!(
            store.provider_recovery_attempts(&request.scope)?.len(),
            usize::from(MAX_PROVIDER_RECOVERY_ATTEMPTS)
        );
    }
    Ok(())
}

#[test]
fn retry_budget_and_outcomes_are_isolated_by_every_scope_component() -> TestResult {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let base = request()?;
    let first = record(&mut store, &base)?;
    for _ in 1..MAX_PROVIDER_RECOVERY_ATTEMPTS {
        record(&mut store, &base)?;
    }
    for field in 0..4 {
        let mut other = base.clone();
        match field {
            0 => other.scope.task_id = id(20)?,
            1 => other.scope.account_id = id(21)?,
            2 => other.scope.provider = BoundedText::try_new("other-provider")?,
            _ => other.scope.checkpoint = ContentHash::from_bytes([4; 32]),
        }
        assert!(store.provider_recovery_attempts(&other.scope)?.is_empty());
        assert!(matches!(
            store.finish_provider_recovery_attempt(
                &other.scope,
                first.id,
                ProviderRecoveryOutcome::Succeeded,
                time(20)?,
            ),
            Err(ProviderRecoveryError::AttemptNotFound)
        ));
        let attempt = record(&mut store, &other)?;
        assert_eq!(attempt.number, 1);
        assert_eq!(store.provider_recovery_attempts(&other.scope)?.len(), 1);
    }
    assert_eq!(
        store.provider_recovery_attempts(&base.scope)?[0].state,
        ProviderRecoveryAttemptState::Pending
    );
    Ok(())
}

#[test]
fn outcomes_are_idempotent_and_conflicts_or_backward_time_leave_history_unchanged() -> TestResult {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let request = request()?;
    let attempt = record(&mut store, &request)?;
    assert!(matches!(
        store.finish_provider_recovery_attempt(
            &request.scope,
            attempt.id,
            ProviderRecoveryOutcome::Succeeded,
            time(9)?,
        ),
        Err(ProviderRecoveryError::OutcomePredatesAttempt)
    ));
    assert_eq!(
        store.provider_recovery_attempts(&request.scope)?,
        vec![attempt.clone()]
    );
    let finished = store.finish_provider_recovery_attempt(
        &request.scope,
        attempt.id,
        ProviderRecoveryOutcome::Succeeded,
        time(20)?,
    )?;
    assert_eq!(
        store.finish_provider_recovery_attempt(
            &request.scope,
            attempt.id,
            ProviderRecoveryOutcome::Succeeded,
            time(50)?,
        )?,
        finished
    );
    assert!(matches!(
        store.finish_provider_recovery_attempt(
            &request.scope,
            attempt.id,
            ProviderRecoveryOutcome::Failed,
            time(60)?,
        ),
        Err(ProviderRecoveryError::ConflictingOutcome)
    ));
    assert_eq!(
        store.provider_recovery_attempts(&request.scope)?,
        vec![finished]
    );
    assert_eq!(
        record(&mut store, &request)?.action,
        ProviderRecoveryAction::ResumeReadOnlySession
    );
    Ok(())
}

#[test]
fn blocked_plans_consume_no_budget_and_cannot_turn_live_writes_into_resume() -> TestResult {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let base = request()?;
    for case in 0..6 {
        let mut request = base.clone();
        let expected = match case {
            0 => {
                request.effect = RecoveryEffect::ExternalWrite;
                ProviderRecoveryPlan::ReadOnlyReconciliationRequired
            }
            1 => {
                request.effect = RecoveryEffect::LocalReversible;
                ProviderRecoveryPlan::ReadOnlyReconciliationRequired
            }
            2 => {
                request.effect = RecoveryEffect::Unknown;
                ProviderRecoveryPlan::Unsupported
            }
            3 => {
                request.policy_version = 999;
                ProviderRecoveryPlan::Unsupported
            }
            4 => {
                request.authority = CurrentAuthority::Revoked;
                ProviderRecoveryPlan::FreshAuthenticationRequired
            }
            _ => {
                request.source = SourcePrecondition::Stale;
                ProviderRecoveryPlan::SourceRefreshRequired
            }
        };
        assert_eq!(
            store.record_provider_recovery_attempt(&request, time(10)?)?,
            ProviderRecoveryAccounting::Required(expected)
        );
        assert!(store.provider_recovery_attempts(&base.scope)?.is_empty());
    }
    assert_eq!(record(&mut store, &base)?.number, 1);
    Ok(())
}

#[test]
fn failed_transactions_do_not_consume_budget_or_finish_attempts() -> TestResult {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let request = request()?;
    store.connection.execute_batch(
        "CREATE TEMP TRIGGER reject_attempt AFTER INSERT ON provider_recovery_attempts
         BEGIN SELECT RAISE(FAIL, 'injected failure'); END;",
    )?;
    assert!(
        store
            .record_provider_recovery_attempt(&request, time(10)?)
            .is_err()
    );
    assert!(store.provider_recovery_attempts(&request.scope)?.is_empty());
    store
        .connection
        .execute_batch("DROP TRIGGER reject_attempt")?;
    let attempt = record(&mut store, &request)?;
    assert_eq!(attempt.number, 1);
    store.connection.execute_batch(
        "CREATE TEMP TRIGGER reject_outcome AFTER UPDATE ON provider_recovery_attempts
         BEGIN SELECT RAISE(FAIL, 'injected failure'); END;",
    )?;
    assert!(
        store
            .finish_provider_recovery_attempt(
                &request.scope,
                attempt.id,
                ProviderRecoveryOutcome::Succeeded,
                time(20)?,
            )
            .is_err()
    );
    assert_eq!(
        store.provider_recovery_attempts(&request.scope)?,
        vec![attempt.clone()]
    );
    store
        .connection
        .execute_batch("DROP TRIGGER reject_outcome")?;
    store.finish_provider_recovery_attempt(
        &request.scope,
        attempt.id,
        ProviderRecoveryOutcome::Succeeded,
        time(20)?,
    )?;
    Ok(())
}

#[test]
fn concurrent_connections_share_one_durable_budget() -> TestResult {
    let profile = Profile::new()?;
    drop(StateStore::open(profile.database())?);
    let barrier = Arc::new(Barrier::new(8));
    let mut threads = Vec::new();
    for _ in 0..8 {
        let path = profile.database();
        let barrier = barrier.clone();
        let request = request()?;
        let now = time(10)?;
        threads.push(std::thread::spawn(move || -> Result<_, String> {
            let store = StateStore::open(path).map_err(|e| e.to_string());
            barrier.wait();
            store?
                .record_provider_recovery_attempt(&request, now)
                .map_err(|e| e.to_string())
        }));
    }
    let mut recorded = 0;
    let mut blocked = 0;
    for thread in threads {
        match thread
            .join()
            .map_err(|_| "provider accounting thread panicked")??
        {
            ProviderRecoveryAccounting::Recorded(_) => recorded += 1,
            ProviderRecoveryAccounting::Required(ProviderRecoveryPlan::ManualRecoveryRequired) => {
                blocked += 1
            }
            other => return Err(format!("unexpected concurrent result: {other:?}").into()),
        }
    }
    assert_eq!(recorded, MAX_PROVIDER_RECOVERY_ATTEMPTS);
    assert_eq!(blocked, 8 - MAX_PROVIDER_RECOVERY_ATTEMPTS);
    let store = StateStore::open(profile.database())?;
    let attempts = store.provider_recovery_attempts(&request()?.scope)?;
    assert_eq!(
        attempts.iter().map(|a| a.number).collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    Ok(())
}

#[test]
fn migration_011_preserves_all_prior_migration_checksums() -> TestResult {
    let mut connection = Connection::open_in_memory()?;
    crate::migrations::apply_migrations_through(&mut connection, 10)?;
    let history = |connection: &Connection| -> Result<Vec<(i64, String, String)>, rusqlite::Error> {
        connection.prepare("SELECT version, name, checksum FROM schema_migrations WHERE version <= 10 ORDER BY version")?
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?.collect()
    };
    let before = history(&connection)?;
    crate::migrations::apply_migrations(&mut connection)?;
    assert_eq!(history(&connection)?, before);
    assert_eq!(
        connection.query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))?,
        11
    );
    crate::migrations::validate_applied_migrations(&connection)?;
    Ok(())
}

#[test]
fn sql_cannot_erase_attempts_change_scope_or_replace_outcomes() -> TestResult {
    let mut store = StateStore::open_in_memory_for_tests()?;
    let request = request()?;
    let attempt = record(&mut store, &request)?;
    for sql in [
        "DELETE FROM provider_recovery_attempts",
        "UPDATE provider_recovery_attempts SET account_id = 'other'",
        "UPDATE provider_recovery_attempts SET attempt_number = 0",
    ] {
        assert!(store.connection.execute(sql, []).is_err());
    }
    let finished = store.finish_provider_recovery_attempt(
        &request.scope,
        attempt.id,
        ProviderRecoveryOutcome::Succeeded,
        time(20)?,
    )?;
    assert!(
        store
            .connection
            .execute(
                "UPDATE provider_recovery_attempts SET outcome = 'failed'",
                []
            )
            .is_err()
    );
    assert!(
        store
            .connection
            .execute(
                "UPDATE provider_recovery_attempts SET outcome = NULL, finished_at_micros = NULL",
                []
            )
            .is_err()
    );
    assert_eq!(
        store.provider_recovery_attempts(&request.scope)?,
        vec![finished]
    );
    Ok(())
}

struct ChildGuard(std::process::Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn provider_attempt_crash_child() -> TestResult {
    let Some(database) = std::env::var_os("INTENT_PROVIDER_ATTEMPT_CRASH_DATABASE") else {
        return Ok(());
    };
    let ready = std::env::var_os("INTENT_PROVIDER_ATTEMPT_CRASH_READY")
        .ok_or("missing child readiness path")?;
    let mut store = StateStore::open(database)?;
    record(&mut store, &request()?)?;
    std::fs::write(ready, b"attempt committed")?;
    loop {
        std::thread::park();
    }
}

#[test]
fn killed_process_leaves_pending_attempt_counted_and_forces_reseed() -> TestResult {
    let profile = Profile::new()?;
    let ready = profile.database().with_extension("ready");
    let mut child = ChildGuard(
        std::process::Command::new(std::env::current_exe()?)
            .args([
                "--exact",
                "provider_recovery::tests::provider_attempt_crash_child",
                "--nocapture",
            ])
            .env("INTENT_PROVIDER_ATTEMPT_CRASH_DATABASE", profile.database())
            .env("INTENT_PROVIDER_ATTEMPT_CRASH_READY", &ready)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .spawn()?,
    );
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    while !ready.exists() {
        if child.0.try_wait()?.is_some() {
            return Err("provider accounting child exited before committing".into());
        }
        if std::time::Instant::now() >= deadline {
            return Err("provider accounting child readiness timed out".into());
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    child.0.kill()?;
    child.0.wait()?;
    let mut store = StateStore::open(profile.database())?;
    let request = request()?;
    let attempts = store.provider_recovery_attempts(&request.scope)?;
    assert_eq!(attempts.len(), 1);
    assert_eq!(attempts[0].state, ProviderRecoveryAttemptState::Pending);
    assert_eq!(
        attempts[0].action,
        ProviderRecoveryAction::ResumeReadOnlySession
    );
    let next = record(&mut store, &request)?;
    assert_eq!(next.number, 2);
    assert_eq!(
        next.action,
        ProviderRecoveryAction::ReseedFromDurableCheckpoint
    );
    assert!(!store.dispatch_status()?.enabled);
    Ok(())
}
