use intent_contracts::{ContentHash, UnixTimestampMicros};
use intent_recovery::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

fn time(value: i64) -> Result<UnixTimestampMicros, Box<dyn Error>> {
    Ok(UnixTimestampMicros::try_new(value)?)
}
fn facts() -> Result<RecoveryFacts, Box<dyn Error>> {
    Ok(RecoveryFacts {
        policy_version: RECOVERY_POLICY_VERSION,
        operation_id: "018f47f7-5a86-7c00-8000-000000000001".parse()?,
        account_id: "018f47f7-5a86-7c00-8000-000000000002".parse()?,
        capability_id: "018f47f7-5a86-7c00-8000-000000000003".parse()?,
        arguments_hash: ContentHash::from_bytes([1; 32]),
        effect: RecoveryEffect::ExternalWrite,
        stage: RecoveryStage::Attempting,
        attempt_id: Some("018f47f7-5a86-7c00-8000-000000000004".parse()?),
        phase: AttemptPhase::Original,
        authority: CurrentAuthority::Current,
        source: SourcePrecondition::Fresh,
        deadline: Some(time(1000)?),
        outcome: None,
        idempotency: None,
    })
}
fn binding(facts: &RecoveryFacts) -> Result<AttemptBinding, Box<dyn Error>> {
    Ok(AttemptBinding {
        operation_id: facts.operation_id,
        account_id: facts.account_id,
        capability_id: facts.capability_id,
        arguments_hash: facts.arguments_hash,
        attempt_id: facts.attempt_id.ok_or("fixture requires attempt")?,
    })
}

#[test]
fn every_stage_effect_authority_and_source_combination_is_non_authorizing() -> TestResult {
    let stages = [
        RecoveryStage::Prepared,
        RecoveryStage::Approved,
        RecoveryStage::DispatchPending,
        RecoveryStage::Attempting,
        RecoveryStage::Accepted,
        RecoveryStage::Verified,
        RecoveryStage::Failed,
        RecoveryStage::NeedsReconciliation,
        RecoveryStage::Cancelled,
        RecoveryStage::Compensating,
        RecoveryStage::Compensated,
    ];
    let effects = [
        RecoveryEffect::ReadOnly,
        RecoveryEffect::LocalReversible,
        RecoveryEffect::ExternalWrite,
        RecoveryEffect::Unknown,
    ];
    let authorities = [
        CurrentAuthority::Current,
        CurrentAuthority::Expired,
        CurrentAuthority::Revoked,
        CurrentAuthority::Unknown,
    ];
    let sources = [
        SourcePrecondition::Fresh,
        SourcePrecondition::Stale,
        SourcePrecondition::Conflict,
        SourcePrecondition::Unknown,
    ];
    for stage in stages {
        for effect in effects {
            for authority in authorities {
                for source in sources {
                    for has_attempt in [false, true] {
                        for expired in [false, true] {
                            let mut f = facts()?;
                            f.stage = stage;
                            f.effect = effect;
                            f.authority = authority;
                            f.source = source;
                            if !has_attempt {
                                f.attempt_id = None;
                            }
                            let d = classify_recovery(&f, time(if expired { 1000 } else { 100 })?);
                            assert!(!d.grants_execution_authority());
                            assert_eq!(d.operation_id, f.operation_id);
                            assert_eq!(d.attempt_id, f.attempt_id);
                            if effect != RecoveryEffect::ReadOnly {
                                assert_ne!(d.disposition, RecoveryDisposition::ReexecuteReadOnly);
                            }
                            assert_ne!(
                                d.disposition,
                                RecoveryDisposition::ProviderIdempotentContinuation
                            );
                            if expired
                                || authority != CurrentAuthority::Current
                                || source != SourcePrecondition::Fresh
                            {
                                assert_ne!(d.disposition, RecoveryDisposition::ReexecuteReadOnly);
                            }
                            if effect == RecoveryEffect::Unknown {
                                assert_eq!(
                                    d.disposition,
                                    RecoveryDisposition::UnsupportedManualRecovery
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[test]
fn lost_acknowledgement_revocation_and_cancellation_never_erase_a_bound_commit() -> TestResult {
    for stage in [
        RecoveryStage::Attempting,
        RecoveryStage::Accepted,
        RecoveryStage::NeedsReconciliation,
        RecoveryStage::Cancelled,
        RecoveryStage::Failed,
        RecoveryStage::Verified,
    ] {
        for authority in [
            CurrentAuthority::Current,
            CurrentAuthority::Revoked,
            CurrentAuthority::Expired,
            CurrentAuthority::Unknown,
        ] {
            let mut f = facts()?;
            f.stage = stage;
            f.authority = authority;
            f.outcome = Some(BoundOutcome {
                binding: binding(&f)?,
                outcome: RecordedOutcome::ExternalCommitted {
                    receipt: ContentHash::from_bytes([2; 32]),
                },
            });
            let d = classify_recovery(&f, time(2000)?);
            assert_eq!(d.disposition, RecoveryDisposition::NoReplayKnownCommit);
            assert_eq!(d.observed, ObservedOutcome::OriginalCommitted);
        }
    }
    let mut f = facts()?;
    f.effect = RecoveryEffect::LocalReversible;
    f.authority = CurrentAuthority::Revoked;
    f.outcome = Some(BoundOutcome {
        binding: binding(&f)?,
        outcome: RecordedOutcome::LocalCommitted {
            revision: 4,
            receipt: ContentHash::from_bytes([3; 32]),
        },
    });
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::NoReplayLocalCommit
    );
    Ok(())
}

#[test]
fn compensation_lineage_cannot_be_confused_with_original_success() -> TestResult {
    let mut f = facts()?;
    f.stage = RecoveryStage::NeedsReconciliation;
    f.phase = AttemptPhase::Compensation {
        original_attempt: "018f47f7-5a86-7c00-8000-000000000005".parse()?,
        original_receipt: ContentHash::from_bytes([5; 32]),
    };
    f.outcome = Some(BoundOutcome {
        binding: binding(&f)?,
        outcome: RecordedOutcome::ExternalCommitted {
            receipt: ContentHash::from_bytes([6; 32]),
        },
    });
    let d = classify_recovery(&f, time(100)?);
    assert_eq!(d.observed, ObservedOutcome::CompensationCommitted);
    assert_eq!(
        d.disposition,
        RecoveryDisposition::NoReplayCompensationConfirmed
    );
    f.phase = AttemptPhase::Compensation {
        original_attempt: f.attempt_id.ok_or("attempt")?,
        original_receipt: ContentHash::from_bytes([5; 32]),
    };
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::BlockedInvalidFacts
    );
    f.phase = AttemptPhase::Original;
    f.stage = RecoveryStage::Compensated;
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::BlockedInvalidFacts
    );
    Ok(())
}

#[test]
fn every_evidence_binding_component_is_checked() -> TestResult {
    let f = facts()?;
    for changed in 0..5 {
        let mut f = f;
        let mut b = binding(&f)?;
        match changed {
            0 => b.operation_id = "018f47f7-5a86-7c00-8000-000000000009".parse()?,
            1 => b.account_id = "018f47f7-5a86-7c00-8000-000000000009".parse()?,
            2 => b.capability_id = "018f47f7-5a86-7c00-8000-000000000009".parse()?,
            3 => b.arguments_hash = ContentHash::from_bytes([9; 32]),
            _ => b.attempt_id = "018f47f7-5a86-7c00-8000-000000000009".parse()?,
        }
        f.outcome = Some(BoundOutcome {
            binding: b,
            outcome: RecordedOutcome::ProvenNotCommitted,
        });
        assert_eq!(
            classify_recovery(&f, time(100)?).disposition,
            RecoveryDisposition::BlockedInvalidFacts
        );
        f.outcome = None;
        f.idempotency = Some(IdempotencyEvidence {
            binding: b,
            guarantee_reference: ContentHash::from_bytes([8; 32]),
            valid_until: time(900)?,
        });
        assert_eq!(
            classify_recovery(&f, time(100)?).disposition,
            RecoveryDisposition::BlockedInvalidFacts
        );
    }
    Ok(())
}

#[test]
fn uncertainty_non_commit_and_idempotency_expiry_have_distinct_paths() -> TestResult {
    let mut f = facts()?;
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::ReadOnlyReconciliation
    );
    f.outcome = Some(BoundOutcome {
        binding: binding(&f)?,
        outcome: RecordedOutcome::ProvenNotCommitted,
    });
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::FreshApprovalAfterProvenNonCommit
    );
    f.outcome = None;
    f.idempotency = Some(IdempotencyEvidence {
        binding: binding(&f)?,
        guarantee_reference: ContentHash::from_bytes([7; 32]),
        valid_until: time(900)?,
    });
    assert_eq!(
        classify_recovery(&f, time(899)?).disposition,
        RecoveryDisposition::ProviderIdempotentContinuation
    );
    assert_eq!(
        classify_recovery(&f, time(900)?).disposition,
        RecoveryDisposition::ReadOnlyReconciliation
    );
    f.authority = CurrentAuthority::Revoked;
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::AwaitFreshAuthority
    );
    f.authority = CurrentAuthority::Current;
    f.source = SourcePrecondition::Stale;
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::RefreshReadOnlySources
    );
    Ok(())
}

#[test]
fn unsupported_and_contradictory_data_never_becomes_ordinary_retry() -> TestResult {
    let mut f = facts()?;
    f.policy_version = 2;
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::UnsupportedManualRecovery
    );
    f.policy_version = 1;
    f.attempt_id = None;
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::BlockedInvalidFacts
    );
    f = facts()?;
    f.stage = RecoveryStage::Verified;
    f.outcome = Some(BoundOutcome {
        binding: binding(&f)?,
        outcome: RecordedOutcome::ProvenNotCommitted,
    });
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::BlockedInvalidFacts
    );
    f = facts()?;
    f.stage = RecoveryStage::Cancelled;
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::ReadOnlyReconciliation
    );
    f.attempt_id = None;
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::TerminalWithoutReplay
    );
    Ok(())
}

#[test]
fn recovery_policy_wire_shape_is_strict_and_cannot_encode_a_permission() -> TestResult {
    let f = facts()?;
    let value = serde_json::to_value(f)?;
    assert_eq!(serde_json::from_value::<RecoveryFacts>(value.clone())?, f);
    let mut unsupported = value;
    unsupported["authorize_dispatch"] = serde_json::json!(true);
    assert!(serde_json::from_value::<RecoveryFacts>(unsupported).is_err());
    let decision = classify_recovery(&f, time(100)?);
    assert_eq!(
        serde_json::from_value::<RecoveryDecision>(serde_json::to_value(decision)?)?,
        decision
    );
    assert!(!decision.grants_execution_authority());
    Ok(())
}

#[test]
fn capability_registration_is_bounded_exact_and_fail_closed() -> TestResult {
    let f = facts()?;
    let p = CapabilityRecoveryPolicy {
        capability_id: f.capability_id,
        policy_version: 1,
        effect: RecoveryEffect::ExternalWrite,
        provider_idempotency_supported: false,
    };
    let registry = RecoveryPolicyRegistry::try_new([p])?;
    assert_eq!(
        registry.classify(&f, time(100)?).disposition,
        RecoveryDisposition::ReadOnlyReconciliation
    );
    let mut changed = f;
    changed.effect = RecoveryEffect::ReadOnly;
    assert_eq!(
        registry.classify(&changed, time(100)?).disposition,
        RecoveryDisposition::BlockedInvalidFacts
    );
    changed = f;
    changed.capability_id = "018f47f7-5a86-7c00-8000-000000000099".parse()?;
    assert_eq!(
        registry.classify(&changed, time(100)?).disposition,
        RecoveryDisposition::UnsupportedManualRecovery
    );
    assert!(matches!(
        RecoveryPolicyRegistry::try_new(std::iter::repeat(p)),
        Err(RecoveryPolicyError::DuplicateCapability)
    ));
    let mut unknown = p;
    unknown.effect = RecoveryEffect::Unknown;
    assert!(matches!(
        RecoveryPolicyRegistry::try_new([unknown]),
        Err(RecoveryPolicyError::Unsupported)
    ));
    let policies: Vec<_> = (0..=MAX_RECOVERY_POLICIES)
        .map(|i| -> Result<_, Box<dyn Error>> {
            Ok(CapabilityRecoveryPolicy {
                capability_id: format!("018f47f7-5a86-7c00-8000-{i:012x}").parse()?,
                ..p
            })
        })
        .collect::<Result<_, _>>()?;
    assert!(matches!(
        RecoveryPolicyRegistry::try_new(policies),
        Err(RecoveryPolicyError::Capacity)
    ));
    Ok(())
}

#[test]
fn local_terminal_without_receipt_requires_local_inspection_not_provider_resend() -> TestResult {
    let mut f = facts()?;
    f.effect = RecoveryEffect::LocalReversible;
    f.stage = RecoveryStage::Verified;
    let d = classify_recovery(&f, time(100)?);
    assert_eq!(d.disposition, RecoveryDisposition::InspectLocalBeforeAfter);
    assert_eq!(
        d.required_evidence,
        RequiredEvidence::LocalBeforeAfterVersions
    );
    f.outcome = Some(BoundOutcome {
        binding: binding(&f)?,
        outcome: RecordedOutcome::ExternalCommitted {
            receipt: ContentHash::from_bytes([1; 32]),
        },
    });
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::BlockedInvalidFacts
    );
    f.outcome = None;
    f.authority = CurrentAuthority::Revoked;
    assert_eq!(
        classify_recovery(&f, time(100)?).disposition,
        RecoveryDisposition::AwaitFreshAuthority
    );
    Ok(())
}
