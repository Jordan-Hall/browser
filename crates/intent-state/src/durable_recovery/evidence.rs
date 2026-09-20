use super::dispatch::parse;
use super::*;
use crate::recovery_error::bounded_json;
use hmac::{Hmac, Mac};
use intent_contracts::{AccountId, CapabilityId, ContentHash};
use intent_recovery::AttemptBinding;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::fmt;

const EVIDENCE_DOMAIN: &[u8] = b"intent.read-only-reconciliation.v1\0";
/// The signer must be a trusted, read-only evidence adapter. A successful MAC verifies
/// that adapter's attestation, not an arbitrary external provider's settlement system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReconciliationVerdict {
    ReadCompleted {
        capture: ContentHash,
    },
    LocalCommitted {
        before_revision: ContentHash,
        after_revision: ContentHash,
        revision: u64,
        receipt: ContentHash,
    },
    Committed {
        receipt: ContentHash,
    },
    AuthoritativeNonCommit {
        observation: ContentHash,
    },
    Inconclusive,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadOnlyAttestation {
    pub schema_version: u16,
    pub evidence_id: Uuid,
    pub binding: AttemptBinding,
    pub verdict: ReconciliationVerdict,
    pub observed_at: UnixTimestampMicros,
    pub valid_until: UnixTimestampMicros,
}
/// A profile-independent trust key supplied by the operator's evidence adapter.
/// Key material is never stored in the profile, restored from a backup or printed.
pub struct EvidenceVerifier {
    key: [u8; 32],
    key_id: ContentHash,
}
impl EvidenceVerifier {
    pub fn new(key: [u8; 32]) -> Result<Self, RecoveryError> {
        if key.iter().all(|v| *v == 0) {
            return Err(RecoveryError::Invalid("empty evidence trust key"));
        }
        Ok(Self {
            key_id: digest(&key),
            key,
        })
    }
    pub fn key_id(&self) -> ContentHash {
        self.key_id
    }
    pub fn verify(
        &self,
        bytes: &[u8],
        tag: &[u8; 32],
        now: UnixTimestampMicros,
    ) -> Result<VerifiedReadOnlyEvidence, RecoveryError> {
        if bytes.len() > 8192 {
            return Err(RecoveryError::Limit("evidence envelope"));
        }
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key)
            .map_err(|_| RecoveryError::Invalid("evidence key"))?;
        mac.update(EVIDENCE_DOMAIN);
        mac.update(bytes);
        mac.verify_slice(tag)
            .map_err(|_| RecoveryError::Denied("evidence authentication"))?;
        let value: ReadOnlyAttestation = serde_json::from_slice(bytes)?;
        if value.schema_version != 1
            || value.evidence_id.is_nil()
            || value.observed_at > now
            || value.valid_until <= now
            || value.valid_until <= value.observed_at
        {
            return Err(RecoveryError::Invalid(
                "evidence version, identity or observation validity",
            ));
        }
        let canonical = bounded_json(&value, 8192)?;
        if canonical != bytes {
            return Err(RecoveryError::Invalid(
                "evidence must use canonical encoding",
            ));
        }
        Ok(VerifiedReadOnlyEvidence {
            value,
            tag: *tag,
            canonical,
            key_id: self.key_id,
        })
    }
}
impl fmt::Debug for EvidenceVerifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EvidenceVerifier")
            .field("key_id", &self.key_id)
            .finish_non_exhaustive()
    }
}
/// Cannot be forged by deserializing a provider response or copied into two outcomes.
pub struct VerifiedReadOnlyEvidence {
    tag: [u8; 32],
    value: ReadOnlyAttestation,
    canonical: Vec<u8>,
    key_id: ContentHash,
}
impl fmt::Debug for VerifiedReadOnlyEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerifiedReadOnlyEvidence")
            .field("evidence_id", &self.value.evidence_id)
            .finish_non_exhaustive()
    }
}
impl VerifiedReadOnlyEvidence {
    pub fn binding(&self) -> intent_recovery::AttemptBinding {
        self.value.binding
    }
}

fn ignore_post_decision_inconclusive(previous_decisive: bool, verdict: &str) -> bool {
    previous_decisive && verdict == "inconclusive"
}

impl RuntimeOwner {
    pub fn revoke_evidence_key(
        &mut self,
        account: AccountId,
        capability: CapabilityId,
        key: ContentHash,
        now: UnixTimestampMicros,
    ) -> Result<(), RecoveryError> {
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, false)?;
        observe_clock(&tx, now)?;
        tx.execute("UPDATE recovery_evidence_keys SET revoked=1 WHERE account_id=?1 AND capability_id=?2 AND key_id=?3",params![account.to_string(),capability.to_string(),key.to_hex()])?;
        tx.commit()?;
        Ok(())
    }
    /// Applies authenticated read-only observations to their exact durable attempt.
    /// Acceptance/timeout of a transport is intentionally not accepted as such evidence.
    pub fn reconcile(
        &mut self,
        evidence: VerifiedReadOnlyEvidence,
        expected_revision: u64,
        now: UnixTimestampMicros,
    ) -> Result<u64, RecoveryError> {
        let value = &evidence.value;
        let binding = value.binding;
        if value.valid_until <= now || value.observed_at > now {
            return Err(RecoveryError::Denied("evidence expired while pending"));
        }
        let tx = self
            .store
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        current_epoch(&tx, self.epoch, false)?;
        observe_clock(&tx, now)?;
        let op = load(&tx, binding.operation_id)?;
        let existing: Option<String> = tx
            .query_row(
                "SELECT payload_hash FROM recovery_evidence WHERE evidence_id=?1",
                [value.evidence_id.to_string()],
                |r| r.get(0),
            )
            .optional()?;
        if let Some(hash) = existing {
            if hash != digest(&evidence.canonical).to_hex() {
                return Err(RecoveryError::Conflict(
                    "evidence identity reused for different facts",
                ));
            }
            tx.commit()?;
            return Ok(op.revision());
        }
        if op.revision() != expected_revision {
            return Err(RecoveryError::Conflict("reconciliation operation revision"));
        }
        if op.account_id() != binding.account_id
            || op.capability_id() != binding.capability_id
            || op.arguments_hash() != binding.arguments_hash
            || op.attempt_identity() != Some(binding.attempt_id)
        {
            return Err(RecoveryError::Denied(
                "evidence does not bind the active operation/account/capability/arguments/attempt",
            ));
        }
        let started:Option<i64>=tx.query_row("SELECT started_at_micros FROM recovery_attempts WHERE attempt_id=?1 AND operation_id=?2 AND evidence_key_id=?3",params![binding.attempt_id.to_string(),binding.operation_id.to_string(),evidence.key_id.to_hex()],|r|r.get(0)).optional()?.flatten();
        if !started.is_some_and(|at| at <= value.observed_at.get()) {
            return Err(RecoveryError::Denied(
                "evidence predates or mismatches durable attempt start",
            ));
        }
        let valid_key:bool=tx.query_row("SELECT EXISTS(SELECT 1 FROM recovery_evidence_keys WHERE account_id=?1 AND capability_id=?2 AND key_id=?3 AND revoked=0)",params![binding.account_id.to_string(),binding.capability_id.to_string(),evidence.key_id.to_hex()],|r|r.get(0))?;
        if !valid_key {
            return Err(RecoveryError::Denied("evidence trust key was revoked"));
        }
        let previous:Option<(String,Option<String>,Vec<u8>)>=tx.query_row("SELECT verdict,receipt,payload FROM recovery_evidence WHERE attempt_id=?1 AND verdict!='inconclusive' ORDER BY recorded_at_micros,evidence_id LIMIT 1",[binding.attempt_id.to_string()],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?))).optional()?;
        let (effect, source): (String, String) = tx.query_row(
            "SELECT effect,source_revision FROM recovery_actions WHERE operation_id=?1",
            [op.operation_id().to_string()],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )?;
        match value.verdict {
            ReconciliationVerdict::Committed { .. } if effect != "external_write" => {
                return Err(RecoveryError::Denied(
                    "external receipt for a different effect domain",
                ));
            }
            ReconciliationVerdict::ReadCompleted { .. } if effect != "read_only" => {
                return Err(RecoveryError::Denied("read capture cannot prove a write"));
            }
            ReconciliationVerdict::LocalCommitted {
                before_revision,
                after_revision,
                revision,
                ..
            } if (effect != "local_reversible"
                || before_revision.to_hex() != source
                || before_revision == after_revision
                || revision == 0) =>
            {
                return Err(RecoveryError::Denied(
                    "local before/after evidence domain or source binding",
                ));
            }
            _ => {}
        }
        let (verdict, receipt, next) = match value.verdict {
            ReconciliationVerdict::ReadCompleted { capture } => (
                "committed",
                Some(capture.to_hex()),
                DurableOperationState::Verified,
            ),
            ReconciliationVerdict::LocalCommitted { receipt, .. } => (
                "committed",
                Some(receipt.to_hex()),
                DurableOperationState::Verified,
            ),
            ReconciliationVerdict::Committed { receipt } => (
                "committed",
                Some(receipt.to_hex()),
                DurableOperationState::Verified,
            ),
            ReconciliationVerdict::AuthoritativeNonCommit { observation } => (
                "not_committed",
                Some(observation.to_hex()),
                DurableOperationState::Failed,
            ),
            ReconciliationVerdict::Inconclusive => (
                "inconclusive",
                None,
                DurableOperationState::NeedsReconciliation,
            ),
        };
        if ignore_post_decision_inconclusive(previous.is_some(), verdict) {
            tx.commit()?;
            return Ok(op.revision());
        }
        if let Some((old, old_receipt, old_payload)) = &previous {
            let old_value: ReadOnlyAttestation = serde_json::from_slice(old_payload)?;
            if old != verdict || old_receipt != &receipt || old_value.verdict != value.verdict {
                return Err(RecoveryError::Conflict(
                    "contradictory final evidence; manual investigation required",
                ));
            }
            return Err(RecoveryError::Denied(
                "attempt already has decisive outcome evidence",
            ));
        }
        if !matches!(
            op.state(),
            DurableOperationState::Attempting
                | DurableOperationState::Accepted
                | DurableOperationState::NeedsReconciliation
                | DurableOperationState::Verified
                | DurableOperationState::Compensated
                | DurableOperationState::Failed
        ) {
            return Err(RecoveryError::Denied(
                "operation cannot accept outcome evidence in this stage",
            ));
        }
        tx.execute(
            "INSERT INTO recovery_evidence VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            params![
                value.evidence_id.to_string(),
                binding.operation_id.to_string(),
                binding.attempt_id.to_string(),
                verdict,
                receipt,
                &evidence.canonical,
                digest(&evidence.canonical).to_hex(),
                evidence.key_id.to_hex(),
                evidence.tag.as_slice(),
                now.get()
            ],
        )?;
        let rev = if previous.is_some() {
            op.revision()
        } else {
            tx.execute("UPDATE outbox_messages SET state='completed',completed_at_micros=?2,updated_at_micros=?2 WHERE operation_id=?1 AND attempt_identity=?3",params![binding.operation_id.to_string(),now.get(),binding.attempt_id.to_string()])?;
            transition(
                &tx,
                &op,
                next,
                op.attempt_identity(),
                now,
                "exact-attempt authenticated read-only reconciliation",
            )?
        };
        if verdict == "committed" {
            let origin:Option<(String,String,String)>=tx.query_row("SELECT original_operation_id,original_attempt_id,original_receipt FROM recovery_actions WHERE operation_id=?1 AND original_operation_id IS NOT NULL",[op.operation_id().to_string()],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?))).optional()?;
            if let Some((original, attempt, receipt)) = origin {
                if matches!(value.verdict, ReconciliationVerdict::ReadCompleted { .. }) {
                    return Err(RecoveryError::Denied(
                        "compensation requires committed write evidence",
                    ));
                }
                let original = load(&tx, parse(&original)?)?;
                if !matches!(
                    original.state(),
                    DurableOperationState::Verified | DurableOperationState::Compensated
                ) || original.attempt_identity() != Some(parse(&attempt)?)
                {
                    return Err(RecoveryError::Integrity(
                        "compensation origin state or attempt changed",
                    ));
                }
                let committed:bool=tx.query_row("SELECT EXISTS(SELECT 1 FROM recovery_evidence WHERE operation_id=?1 AND attempt_id=?2 AND verdict='committed' AND receipt=?3)",params![original.operation_id().to_string(),attempt,receipt],|r|r.get(0))?;
                if !committed {
                    return Err(RecoveryError::Integrity(
                        "original receipt missing during compensation",
                    ));
                }
                if original.state() != DurableOperationState::Compensated {
                    transition(
                        &tx,
                        &original,
                        DurableOperationState::Compensated,
                        original.attempt_identity(),
                        now,
                        "separate compensation attempt has verified commit evidence",
                    )?;
                }
            }
        }
        tx.commit()?;
        Ok(rev)
    }
}

#[cfg(test)]
mod tests {
    use super::ignore_post_decision_inconclusive;

    #[test]
    fn post_decision_inconclusive_is_a_non_persistent_noop() {
        assert!(ignore_post_decision_inconclusive(true, "inconclusive"));
        assert!(!ignore_post_decision_inconclusive(false, "inconclusive"));
        assert!(!ignore_post_decision_inconclusive(true, "committed"));
        assert!(!ignore_post_decision_inconclusive(true, "not_committed"));
    }
}
