use intent_contracts::{
    AccountId, CapabilityId, ContentHash, OperationAttemptId, OperationId, UnixTimestampMicros,
};
use serde::{Deserialize, Serialize};

pub const RECOVERY_POLICY_VERSION: u16 = 1;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryEffect {
    ReadOnly,
    LocalReversible,
    ExternalWrite,
    Unknown,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryStage {
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AttemptPhase {
    Original,
    Compensation {
        original_attempt: OperationAttemptId,
        original_receipt: ContentHash,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentAuthority {
    Current,
    Expired,
    Revoked,
    Unknown,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePrecondition {
    Fresh,
    Stale,
    Conflict,
    Unknown,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttemptBinding {
    pub operation_id: OperationId,
    pub account_id: AccountId,
    pub capability_id: CapabilityId,
    pub arguments_hash: ContentHash,
    pub attempt_id: OperationAttemptId,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecordedOutcome {
    Inconclusive,
    ReadCompleted { capture: ContentHash },
    ProvenNotCommitted,
    AcceptedUnverified,
    ExternalCommitted { receipt: ContentHash },
    LocalCommitted { revision: u64, receipt: ContentHash },
    Contradictory,
}

/// Only a trusted evidence verifier may supply these facts to an executor. This
/// value's serializability does not make a provider's assertion authenticated.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoundOutcome {
    pub binding: AttemptBinding,
    pub outcome: RecordedOutcome,
}

/// A provider-specific verifier must establish the guarantee and bind its exact
/// account, payload and validity window. No idempotency secret is retained here.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdempotencyEvidence {
    pub binding: AttemptBinding,
    pub guarantee_reference: ContentHash,
    pub valid_until: UnixTimestampMicros,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryFacts {
    pub policy_version: u16,
    pub operation_id: OperationId,
    pub account_id: AccountId,
    pub capability_id: CapabilityId,
    pub arguments_hash: ContentHash,
    pub effect: RecoveryEffect,
    pub stage: RecoveryStage,
    pub attempt_id: Option<OperationAttemptId>,
    pub phase: AttemptPhase,
    pub authority: CurrentAuthority,
    pub source: SourcePrecondition,
    pub deadline: Option<UnixTimestampMicros>,
    pub outcome: Option<BoundOutcome>,
    pub idempotency: Option<IdempotencyEvidence>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservedOutcome {
    NotStarted,
    Unknown,
    NotCommitted,
    AcceptedUnverified,
    OriginalCommitted,
    CompensationCommitted,
    LocalCommitted,
    ReadCompleted,
    Conflict,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryDisposition {
    UnsupportedManualRecovery,
    RestoreHistoryUncertain,
    BlockedInvalidFacts,
    NoReplayKnownCommit,
    NoReplayCapturedRead,
    NoReplayCompensationConfirmed,
    NoReplayLocalCommit,
    TerminalWithoutReplay,
    DeadlineExpired,
    AwaitFreshAuthority,
    ResolveSourceConflict,
    RefreshReadOnlySources,
    ReexecuteReadOnly,
    InspectLocalBeforeAfter,
    RevalidateBeforeFirstDispatch,
    FreshApprovalAfterProvenNonCommit,
    ReadOnlyReconciliation,
    ProviderIdempotentContinuation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredEvidence {
    None,
    SupportedVersionAndEffect,
    IndependentRollbackDomainHistory,
    ConsistentAttemptLineage,
    CurrentAuthorityAndFreshPreconditions,
    LocalBeforeAfterVersions,
    BoundReadOnlyProviderObservation,
    FreshApproval,
    VerifiedProviderGuaranteeAndCurrentAuthority,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryDecision {
    pub policy_version: u16,
    pub operation_id: OperationId,
    pub attempt_id: Option<OperationAttemptId>,
    pub phase: AttemptPhase,
    pub observed: ObservedOutcome,
    pub disposition: RecoveryDisposition,
    pub required_evidence: RequiredEvidence,
}

impl RecoveryDecision {
    /// This crate has no execution capability. Even an allowed read must pass the
    /// live supervisor/dispatcher checks immediately before it is performed.
    #[must_use]
    pub const fn grants_execution_authority(&self) -> bool {
        false
    }
}

impl RecoveryFacts {
    fn binding(&self) -> Option<AttemptBinding> {
        self.attempt_id.map(|attempt_id| AttemptBinding {
            operation_id: self.operation_id,
            account_id: self.account_id,
            capability_id: self.capability_id,
            arguments_hash: self.arguments_hash,
            attempt_id,
        })
    }

    fn decision(
        &self,
        observed: ObservedOutcome,
        disposition: RecoveryDisposition,
        required_evidence: RequiredEvidence,
    ) -> RecoveryDecision {
        RecoveryDecision {
            policy_version: RECOVERY_POLICY_VERSION,
            operation_id: self.operation_id,
            attempt_id: self.attempt_id,
            phase: self.phase,
            observed,
            disposition,
            required_evidence,
        }
    }
}

/// Classify verified durable facts without querying a provider or mutating state.
/// "Continuation" is a plan requiring a fresh dispatcher check, not a retry API.
#[must_use]
pub fn classify_recovery(facts: &RecoveryFacts, now: UnixTimestampMicros) -> RecoveryDecision {
    use ObservedOutcome as O;
    use RecoveryDisposition as D;
    use RecoveryStage as S;
    use RequiredEvidence as E;

    if facts.policy_version != RECOVERY_POLICY_VERSION || facts.effect == RecoveryEffect::Unknown {
        return facts.decision(
            O::Unknown,
            D::UnsupportedManualRecovery,
            E::SupportedVersionAndEffect,
        );
    }
    let binding = facts.binding();
    let outcome = facts.outcome.map(|value| value.outcome);
    let requires_attempt = matches!(
        facts.stage,
        S::Attempting
            | S::Accepted
            | S::Verified
            | S::NeedsReconciliation
            | S::Compensating
            | S::Compensated
    );
    let is_compensation = matches!(facts.phase, AttemptPhase::Compensation { .. });
    let invalid_phase = match facts.phase {
        AttemptPhase::Original => matches!(facts.stage, S::Compensating | S::Compensated),
        AttemptPhase::Compensation {
            original_attempt, ..
        } => {
            facts
                .attempt_id
                .is_none_or(|active| active == original_attempt)
                || matches!(facts.stage, S::Prepared | S::Approved | S::DispatchPending)
        }
    };
    let invalid_binding = facts
        .outcome
        .is_some_and(|value| Some(value.binding) != binding)
        || facts
            .idempotency
            .is_some_and(|value| Some(value.binding) != binding);
    let invalid_outcome = (facts.effect != RecoveryEffect::ReadOnly
        && matches!(outcome, Some(RecordedOutcome::ReadCompleted { .. })))
        || (facts.idempotency.is_some() && facts.effect != RecoveryEffect::ExternalWrite)
        || (facts.effect == RecoveryEffect::LocalReversible
            && matches!(outcome, Some(RecordedOutcome::ExternalCommitted { .. })))
        || matches!(outcome, Some(RecordedOutcome::Contradictory))
        || (matches!(facts.stage, S::Verified | S::Compensated)
            && matches!(outcome, Some(RecordedOutcome::ProvenNotCommitted)))
        || (facts.effect != RecoveryEffect::LocalReversible
            && matches!(outcome, Some(RecordedOutcome::LocalCommitted { .. })))
        || (facts.effect == RecoveryEffect::ReadOnly
            && (is_compensation
                || matches!(outcome, Some(RecordedOutcome::ExternalCommitted { .. }))));
    if invalid_phase
        || invalid_binding
        || invalid_outcome
        || (requires_attempt && binding.is_none())
        || (matches!(facts.stage, S::Prepared | S::Approved) && binding.is_some())
        || (matches!(facts.stage, S::Prepared | S::Approved | S::DispatchPending)
            && outcome.is_some_and(|value| {
                !matches!(
                    value,
                    RecordedOutcome::Inconclusive | RecordedOutcome::ProvenNotCommitted
                )
            }))
    {
        return facts.decision(
            O::Conflict,
            D::BlockedInvalidFacts,
            E::ConsistentAttemptLineage,
        );
    }

    if matches!(outcome, Some(RecordedOutcome::ReadCompleted { .. })) {
        return facts.decision(O::ReadCompleted, D::NoReplayCapturedRead, E::None);
    }
    // Strong bound commit evidence wins over a lost response, expired grant or
    // cancelled worker. None of those observations undoes an external effect.
    if matches!(outcome, Some(RecordedOutcome::ExternalCommitted { .. })) {
        return if is_compensation {
            facts.decision(
                O::CompensationCommitted,
                D::NoReplayCompensationConfirmed,
                E::None,
            )
        } else {
            facts.decision(O::OriginalCommitted, D::NoReplayKnownCommit, E::None)
        };
    }
    if matches!(outcome, Some(RecordedOutcome::LocalCommitted { .. })) {
        return if is_compensation {
            facts.decision(
                O::CompensationCommitted,
                D::NoReplayCompensationConfirmed,
                E::None,
            )
        } else {
            facts.decision(O::LocalCommitted, D::NoReplayLocalCommit, E::None)
        };
    }

    let never_started = matches!(facts.stage, S::Prepared | S::Approved | S::DispatchPending)
        || (matches!(facts.stage, S::Cancelled | S::Failed) && facts.attempt_id.is_none());
    let observed = match outcome {
        Some(RecordedOutcome::ProvenNotCommitted) => O::NotCommitted,
        Some(RecordedOutcome::AcceptedUnverified) => O::AcceptedUnverified,
        _ if facts.stage == S::Accepted => O::AcceptedUnverified,
        _ if never_started => O::NotStarted,
        _ => O::Unknown,
    };
    if matches!(facts.stage, S::Cancelled | S::Failed) && never_started {
        return facts.decision(observed, D::TerminalWithoutReplay, E::None);
    }
    if facts.deadline.is_some_and(|deadline| now >= deadline) {
        return facts.decision(
            observed,
            D::DeadlineExpired,
            E::CurrentAuthorityAndFreshPreconditions,
        );
    }
    if facts.authority != CurrentAuthority::Current {
        return facts.decision(
            observed,
            D::AwaitFreshAuthority,
            E::CurrentAuthorityAndFreshPreconditions,
        );
    }
    if facts.source == SourcePrecondition::Conflict {
        return facts.decision(
            observed,
            D::ResolveSourceConflict,
            E::CurrentAuthorityAndFreshPreconditions,
        );
    }
    if facts.source != SourcePrecondition::Fresh {
        return facts.decision(
            observed,
            D::RefreshReadOnlySources,
            E::CurrentAuthorityAndFreshPreconditions,
        );
    }

    // Terminal labels alone cannot fabricate receipt evidence. Select a
    // read-only inspection appropriate to the recorded effect domain.
    if matches!(facts.stage, S::Verified | S::Compensated) {
        return match facts.effect {
            RecoveryEffect::LocalReversible => facts.decision(
                observed,
                D::InspectLocalBeforeAfter,
                E::LocalBeforeAfterVersions,
            ),
            RecoveryEffect::ReadOnly => facts.decision(
                observed,
                D::RefreshReadOnlySources,
                E::CurrentAuthorityAndFreshPreconditions,
            ),
            _ => facts.decision(
                observed,
                D::ReadOnlyReconciliation,
                E::BoundReadOnlyProviderObservation,
            ),
        };
    }

    match facts.effect {
        RecoveryEffect::ReadOnly => facts.decision(
            observed,
            D::ReexecuteReadOnly,
            E::CurrentAuthorityAndFreshPreconditions,
        ),
        RecoveryEffect::LocalReversible if !never_started && observed != O::NotCommitted => facts
            .decision(
                observed,
                D::InspectLocalBeforeAfter,
                E::LocalBeforeAfterVersions,
            ),
        RecoveryEffect::LocalReversible | RecoveryEffect::ExternalWrite if never_started => {
            facts.decision(observed, D::RevalidateBeforeFirstDispatch, E::FreshApproval)
        }
        RecoveryEffect::LocalReversible | RecoveryEffect::ExternalWrite
            if observed == O::NotCommitted =>
        {
            facts.decision(
                observed,
                D::FreshApprovalAfterProvenNonCommit,
                E::FreshApproval,
            )
        }
        RecoveryEffect::ExternalWrite
            if !is_compensation
                && facts
                    .idempotency
                    .is_some_and(|value| now < value.valid_until) =>
        {
            facts.decision(
                observed,
                D::ProviderIdempotentContinuation,
                E::VerifiedProviderGuaranteeAndCurrentAuthority,
            )
        }
        RecoveryEffect::ExternalWrite | RecoveryEffect::LocalReversible => facts.decision(
            observed,
            D::ReadOnlyReconciliation,
            E::BoundReadOnlyProviderObservation,
        ),
        RecoveryEffect::Unknown => facts.decision(
            O::Unknown,
            D::UnsupportedManualRecovery,
            E::SupportedVersionAndEffect,
        ),
    }
}
