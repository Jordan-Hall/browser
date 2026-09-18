use crate::{
    ObservedOutcome, RECOVERY_POLICY_VERSION, RecoveryDecision, RecoveryDisposition,
    RecoveryEffect, RecoveryFacts, RequiredEvidence, classify_recovery,
};
use intent_contracts::{CapabilityId, UnixTimestampMicros};
use std::{collections::BTreeMap, error::Error, fmt};

pub const MAX_RECOVERY_POLICIES: usize = 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityRecoveryPolicy {
    pub capability_id: CapabilityId,
    pub policy_version: u16,
    pub effect: RecoveryEffect,
    pub provider_idempotency_supported: bool,
}

#[derive(Debug)]
pub struct RecoveryPolicyRegistry {
    policies: BTreeMap<CapabilityId, CapabilityRecoveryPolicy>,
}

impl RecoveryPolicyRegistry {
    pub fn try_new(
        policies: impl IntoIterator<Item = CapabilityRecoveryPolicy>,
    ) -> Result<Self, RecoveryPolicyError> {
        let mut bounded = BTreeMap::new();
        for (index, policy) in policies
            .into_iter()
            .take(MAX_RECOVERY_POLICIES + 1)
            .enumerate()
        {
            if index == MAX_RECOVERY_POLICIES {
                return Err(RecoveryPolicyError::Capacity);
            }
            if policy.policy_version != RECOVERY_POLICY_VERSION
                || policy.effect == RecoveryEffect::Unknown
            {
                return Err(RecoveryPolicyError::Unsupported);
            }
            if policy.provider_idempotency_supported
                && policy.effect != RecoveryEffect::ExternalWrite
            {
                return Err(RecoveryPolicyError::Unsupported);
            }
            if bounded.insert(policy.capability_id, policy).is_some() {
                return Err(RecoveryPolicyError::DuplicateCapability);
            }
        }
        Ok(Self { policies: bounded })
    }

    #[must_use]
    pub fn classify(&self, facts: &RecoveryFacts, now: UnixTimestampMicros) -> RecoveryDecision {
        let Some(policy) = self.policies.get(&facts.capability_id) else {
            return rejected(
                facts,
                RecoveryDisposition::UnsupportedManualRecovery,
                RequiredEvidence::SupportedVersionAndEffect,
            );
        };
        if facts.policy_version != policy.policy_version
            || facts.effect != policy.effect
            || (facts.idempotency.is_some() && !policy.provider_idempotency_supported)
        {
            return rejected(
                facts,
                RecoveryDisposition::BlockedInvalidFacts,
                RequiredEvidence::ConsistentAttemptLineage,
            );
        }
        classify_recovery(facts, now)
    }

    #[must_use]
    pub fn contains(&self, capability: CapabilityId) -> bool {
        self.policies.contains_key(&capability)
    }
}

fn rejected(
    facts: &RecoveryFacts,
    disposition: RecoveryDisposition,
    required_evidence: RequiredEvidence,
) -> RecoveryDecision {
    RecoveryDecision {
        policy_version: RECOVERY_POLICY_VERSION,
        operation_id: facts.operation_id,
        attempt_id: facts.attempt_id,
        phase: facts.phase,
        observed: ObservedOutcome::Conflict,
        disposition,
        required_evidence,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryPolicyError {
    Capacity,
    Unsupported,
    DuplicateCapability,
}
impl fmt::Display for RecoveryPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid capability recovery policy: {self:?}")
    }
}
impl Error for RecoveryPolicyError {}
