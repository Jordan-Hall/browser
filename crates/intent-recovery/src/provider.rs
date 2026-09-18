use crate::{CurrentAuthority, RecoveryEffect, SourcePrecondition};
use intent_contracts::{AccountId, BoundedText, ContentHash, UnixTimestampMicros};
use serde::{Deserialize, Serialize};

pub const MAX_PROVIDER_RECOVERY_ATTEMPTS: u16 = 4;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionResumeCapability {
    Unsupported,
    ReadOnlyVersion1,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderSessionEvidence {
    pub account_id: AccountId,
    pub provider: BoundedText<128>,
    pub session_reference: BoundedText<256>,
    pub checkpoint_hash: ContentHash,
    pub valid_until: UnixTimestampMicros,
    pub capability: SessionResumeCapability,
}

impl std::fmt::Debug for ProviderSessionEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderSessionEvidence")
            .field("capability", &self.capability)
            .field("session_reference", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderRecoveryFacts {
    pub policy_version: u16,
    pub account_id: AccountId,
    pub provider: BoundedText<128>,
    pub durable_checkpoint: ContentHash,
    pub effect: RecoveryEffect,
    pub authority: CurrentAuthority,
    pub source: SourcePrecondition,
    pub session: Option<ProviderSessionEvidence>,
    pub previous_resume_failed: bool,
    pub attempts_in_incarnation: u16,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderRecoveryPlan {
    Unsupported,
    ManualRecoveryRequired,
    ReadOnlyReconciliationRequired,
    FreshAuthenticationRequired,
    SourceRefreshRequired,
    ResumeReadOnlySession,
    ReseedFromDurableCheckpoint,
}

/// A provider conversation is a disposable execution aid, never authoritative
/// task state. No plan carries credentials or permission to execute a tool.
#[must_use]
pub fn plan_provider_recovery(
    facts: &ProviderRecoveryFacts,
    now: UnixTimestampMicros,
) -> ProviderRecoveryPlan {
    use ProviderRecoveryPlan as P;
    if facts.policy_version != crate::RECOVERY_POLICY_VERSION
        || facts.provider.as_str().trim().is_empty()
        || facts.effect == RecoveryEffect::Unknown
    {
        return P::Unsupported;
    }
    if facts.effect != RecoveryEffect::ReadOnly {
        return P::ReadOnlyReconciliationRequired;
    }
    if facts.attempts_in_incarnation >= MAX_PROVIDER_RECOVERY_ATTEMPTS {
        return P::ManualRecoveryRequired;
    }
    if facts.authority != CurrentAuthority::Current {
        return P::FreshAuthenticationRequired;
    }
    if facts.source != SourcePrecondition::Fresh {
        return P::SourceRefreshRequired;
    }
    if facts.previous_resume_failed {
        return P::ReseedFromDurableCheckpoint;
    }
    match &facts.session {
        Some(session)
            if session.account_id == facts.account_id
                && session.provider == facts.provider
                && !session.session_reference.as_str().trim().is_empty()
                && session.checkpoint_hash == facts.durable_checkpoint
                && now < session.valid_until
                && session.capability == SessionResumeCapability::ReadOnlyVersion1 =>
        {
            P::ResumeReadOnlySession
        }
        _ => P::ReseedFromDurableCheckpoint,
    }
}
