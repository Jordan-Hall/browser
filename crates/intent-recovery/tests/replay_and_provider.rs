use intent_contracts::{
    AccountId, BoundedText, ContentHash, SchemaVersion, TaskId, UnixTimestampMicros,
};
use intent_recovery::*;
use sha2::{Digest, Sha256};
use std::{error::Error, time::Duration};
type TestResult = Result<(), Box<dyn Error>>;

fn task() -> Result<TaskId, Box<dyn Error>> {
    Ok("018f47f7-5a86-7c00-8000-000000000001".parse()?)
}
fn account() -> Result<AccountId, Box<dyn Error>> {
    Ok("018f47f7-5a86-7c00-8000-000000000002".parse()?)
}
fn time(value: i64) -> Result<UnixTimestampMicros, Box<dyn Error>> {
    Ok(UnixTimestampMicros::try_new(value)?)
}
fn record(sequence: u64, data: &[u8]) -> Result<CapturedRecord, Box<dyn Error>> {
    Ok(CapturedRecord {
        schema: SchemaVersion::V1,
        task_id: task()?,
        account_id: account()?,
        sequence,
        kind: CapturedKind::Observation,
        hash: ContentHash::from_bytes(Sha256::digest(data).into()),
        bytes: data.to_vec(),
    })
}
fn limits(entries: usize, bytes: usize) -> Result<ReplayLimits, Box<dyn Error>> {
    Ok(ReplayLimits::try_new(
        entries,
        bytes,
        bytes.min(MAX_REPLAY_ENTRY_BYTES),
        Duration::from_secs(10),
    )?)
}

#[test]
fn captures_replay_deterministically_with_exact_budget_release() -> TestResult {
    let entries = vec![record(8, b"first")?, record(9, b"second")?];
    let mut first =
        NoProductionReplay::from_captures(task()?, account()?, entries.clone(), limits(2, 11)?)?;
    let mut second =
        NoProductionReplay::from_captures(task()?, account()?, entries, limits(2, 11)?)?;
    assert_eq!(first.remaining_bytes(), 11);
    for sequence in [8, 9] {
        let output = first.step()?.ok_or("missing capture")?;
        assert_eq!(output.sequence(), sequence);
        assert_eq!(Some(output), second.step()?);
    }
    assert_eq!(first.remaining_entries(), 0);
    assert_eq!(first.remaining_bytes(), 0);
    assert_eq!(first.step()?, None);
    assert_eq!(first.step()?, None);
    Ok(())
}

#[test]
fn capture_scope_sequence_schema_and_hash_are_verified() -> TestResult {
    for changed in 0..4 {
        let mut capture = record(1, b"bytes")?;
        let expected = match changed {
            0 => {
                capture.task_id = "018f47f7-5a86-7c00-8000-000000000099".parse()?;
                ReplayError::ScopeMismatch
            }
            1 => {
                capture.account_id = "018f47f7-5a86-7c00-8000-000000000099".parse()?;
                ReplayError::ScopeMismatch
            }
            2 => {
                capture.schema = SchemaVersion::try_new(2, 0)?;
                ReplayError::UnsupportedSchema
            }
            _ => {
                capture.bytes[0] ^= 1;
                ReplayError::CorruptCapture
            }
        };
        assert_eq!(
            NoProductionReplay::from_captures(task()?, account()?, [capture], limits(4, 64)?).err(),
            Some(expected)
        );
    }
    for second in [1, 0] {
        assert_eq!(
            NoProductionReplay::from_captures(
                task()?,
                account()?,
                [record(1, b"a")?, record(second, b"b")?],
                limits(4, 64)?
            )
            .err(),
            Some(ReplayError::UnorderedSequence)
        );
    }
    Ok(())
}

#[test]
fn invalid_and_aggregate_replay_budgets_fail_before_admitting_data() -> TestResult {
    assert!(ReplayLimits::try_new(0, 1, 1, Duration::from_secs(1)).is_err());
    assert!(ReplayLimits::try_new(MAX_REPLAY_ENTRIES + 1, 1, 1, Duration::from_secs(1)).is_err());
    assert!(ReplayLimits::try_new(1, usize::MAX, 1, Duration::from_secs(1)).is_err());
    assert!(ReplayLimits::try_new(1, 1, 2, Duration::from_secs(1)).is_err());
    assert!(ReplayLimits::try_new(1, 1, 1, Duration::ZERO).is_err());
    let a = record(1, b"123")?;
    let b = record(2, b"456")?;
    assert_eq!(
        NoProductionReplay::from_captures(
            task()?,
            account()?,
            [a.clone(), b.clone()],
            limits(2, 5)?
        )
        .err(),
        Some(ReplayError::ByteLimit)
    );
    assert_eq!(
        NoProductionReplay::from_captures(task()?, account()?, [a, b], limits(1, 6)?).err(),
        Some(ReplayError::EntryLimit)
    );
    assert_eq!(
        NoProductionReplay::from_captures(
            task()?,
            account()?,
            [record(1, b"oversized")?],
            limits(1, 3)?
        )
        .err(),
        Some(ReplayError::ByteLimit)
    );
    Ok(())
}

#[test]
fn an_infinite_capture_iterator_is_consumed_only_to_the_entry_budget() -> TestResult {
    let capture = record(0, b"one")?;
    let mut pulls = 0u64;
    let captures = std::iter::from_fn(|| {
        pulls += 1;
        let mut c = capture.clone();
        c.sequence = pulls;
        Some(c)
    });
    assert_eq!(
        NoProductionReplay::from_captures(task()?, account()?, captures, limits(2, 32)?).err(),
        Some(ReplayError::EntryLimit)
    );
    assert_eq!(pulls, 3);
    Ok(())
}

#[test]
fn captured_action_text_is_not_interpreted_or_logged_as_a_command() -> TestResult {
    let bytes =
        br#"{"action":"delete","destination":"production","credential":"sensitive-fixture"}"#;
    let capture = record(1, bytes)?;
    assert!(!format!("{capture:?}").contains("sensitive-fixture"));
    let mut context =
        NoProductionReplay::from_captures(task()?, account()?, [capture], limits(1, 512)?)?;
    assert!(!format!("{context:?}").contains("sensitive-fixture"));
    let replayed = context.step()?.ok_or("capture")?;
    assert_eq!(replayed.captured_bytes(), bytes);
    assert!(!format!("{replayed:?}").contains("sensitive-fixture"));
    assert_eq!(context.step()?, None);
    Ok(())
}

fn provider_facts() -> Result<ProviderRecoveryFacts, Box<dyn Error>> {
    let provider = BoundedText::try_new("fixture-provider")?;
    let hash = ContentHash::from_bytes([1; 32]);
    Ok(ProviderRecoveryFacts {
        policy_version: 1,
        account_id: account()?,
        provider: provider.clone(),
        durable_checkpoint: hash,
        effect: RecoveryEffect::ReadOnly,
        authority: CurrentAuthority::Current,
        source: SourcePrecondition::Fresh,
        previous_resume_failed: false,
        attempts_in_incarnation: 0,
        session: Some(ProviderSessionEvidence {
            account_id: account()?,
            provider,
            session_reference: BoundedText::try_new("sensitive-session-reference")?,
            checkpoint_hash: hash,
            valid_until: time(1000)?,
            capability: SessionResumeCapability::ReadOnlyVersion1,
        }),
    })
}

#[test]
fn provider_resume_is_bound_to_current_account_provider_and_checkpoint() -> TestResult {
    let f = provider_facts()?;
    assert_eq!(
        plan_provider_recovery(&f, time(999)?),
        ProviderRecoveryPlan::ResumeReadOnlySession
    );
    assert_eq!(
        plan_provider_recovery(&f, time(1000)?),
        ProviderRecoveryPlan::ReseedFromDurableCheckpoint
    );
    for changed in 0..5 {
        let mut f = f.clone();
        let s = f.session.as_mut().ok_or("session")?;
        match changed {
            0 => s.account_id = "018f47f7-5a86-7c00-8000-000000000099".parse()?,
            1 => s.provider = BoundedText::try_new("other-provider")?,
            2 => s.checkpoint_hash = ContentHash::from_bytes([2; 32]),
            3 => s.session_reference = BoundedText::try_new("")?,
            _ => s.capability = SessionResumeCapability::Unsupported,
        }
        assert_eq!(
            plan_provider_recovery(&f, time(100)?),
            ProviderRecoveryPlan::ReseedFromDurableCheckpoint
        );
    }
    assert!(!format!("{f:?}").contains("sensitive-session-reference"));
    Ok(())
}

#[test]
fn provider_resume_never_replays_writes_or_bypasses_expired_authority() -> TestResult {
    let mut f = provider_facts()?;
    for effect in [
        RecoveryEffect::ExternalWrite,
        RecoveryEffect::LocalReversible,
    ] {
        f.effect = effect;
        assert_eq!(
            plan_provider_recovery(&f, time(100)?),
            ProviderRecoveryPlan::ReadOnlyReconciliationRequired
        );
    }
    f.effect = RecoveryEffect::ReadOnly;
    for authority in [
        CurrentAuthority::Expired,
        CurrentAuthority::Revoked,
        CurrentAuthority::Unknown,
    ] {
        f.authority = authority;
        assert_eq!(
            plan_provider_recovery(&f, time(100)?),
            ProviderRecoveryPlan::FreshAuthenticationRequired
        );
    }
    f.authority = CurrentAuthority::Current;
    f.previous_resume_failed = true;
    assert_eq!(
        plan_provider_recovery(&f, time(100)?),
        ProviderRecoveryPlan::ReseedFromDurableCheckpoint
    );
    f.source = SourcePrecondition::Stale;
    assert_eq!(
        plan_provider_recovery(&f, time(100)?),
        ProviderRecoveryPlan::SourceRefreshRequired
    );
    f.policy_version = 2;
    assert_eq!(
        plan_provider_recovery(&f, time(100)?),
        ProviderRecoveryPlan::Unsupported
    );
    Ok(())
}

#[test]
fn provider_policy_roundtrips_but_rejects_unrecognized_authority_fields() -> TestResult {
    let f = provider_facts()?;
    let mut value = serde_json::to_value(&f)?;
    assert_eq!(
        serde_json::from_value::<ProviderRecoveryFacts>(value.clone())?,
        f
    );
    value["restored_tool_authority"] = serde_json::json!(true);
    assert!(serde_json::from_value::<ProviderRecoveryFacts>(value).is_err());
    Ok(())
}

#[test]
fn repeated_provider_recovery_attempts_stop_at_the_incarnation_budget() -> TestResult {
    let mut f = provider_facts()?;
    f.attempts_in_incarnation = MAX_PROVIDER_RECOVERY_ATTEMPTS - 1;
    assert_eq!(
        plan_provider_recovery(&f, time(100)?),
        ProviderRecoveryPlan::ResumeReadOnlySession
    );
    f.attempts_in_incarnation = MAX_PROVIDER_RECOVERY_ATTEMPTS;
    assert_eq!(
        plan_provider_recovery(&f, time(100)?),
        ProviderRecoveryPlan::ManualRecoveryRequired
    );
    f.previous_resume_failed = true;
    assert_eq!(
        plan_provider_recovery(&f, time(100)?),
        ProviderRecoveryPlan::ManualRecoveryRequired
    );
    Ok(())
}
