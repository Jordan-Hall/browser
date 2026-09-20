use crate::{
    BoundedText, CapabilityId, ContentHash, EvidenceId, OperationAttemptId, OperationId, TaskId,
    UnixTimestampMicros,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStage {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionAttempt {
    pub id: OperationAttemptId,
    pub started_at: Option<UnixTimestampMicros>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case", deny_unknown_fields)]
pub enum ExecutionPhase {
    Original {},
    Compensation {
        original_operation_id: OperationId,
        original_attempt_id: OperationAttemptId,
        original_receipt: ContentHash,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum ExecutionOutcome {
    ReadCompleted {
        capture: ContentHash,
    },
    LocalCommitted {
        before_revision: ContentHash,
        after_revision: ContentHash,
        revision: u64,
        receipt: ContentHash,
    },
    ExternalCommitted {
        receipt: ContentHash,
    },
    ProvenNotCommitted {
        observation: ContentHash,
    },
    Inconclusive {},
}

impl ExecutionOutcome {
    #[must_use]
    pub const fn committed_reference(self) -> Option<ContentHash> {
        match self {
            Self::ReadCompleted { capture } => Some(capture),
            Self::LocalCommitted { receipt, .. } | Self::ExternalCommitted { receipt } => {
                Some(receipt)
            }
            Self::ProvenNotCommitted { .. } | Self::Inconclusive {} => None,
        }
    }

    #[must_use]
    const fn is_compensating_write(self) -> bool {
        matches!(Self::LocalCommitted { .. } | Self::ExternalCommitted { .. })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionEvidence {
    pub evidence_id: EvidenceId,
    pub attempt_id: OperationAttemptId,
    pub payload_hash: ContentHash,
    pub observed_at: UnixTimestampMicros,
    pub recorded_at: UnixTimestampMicros,
    pub outcome: ExecutionOutcome,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerifiedCompensation {
    pub operation_id: OperationId,
    pub attempt: ExecutionAttempt,
    pub evidence: ExecutionEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionObservationData {
    pub task_id: TaskId,
    pub capability_id: CapabilityId,
    pub arguments_hash: ContentHash,
    pub revision: u64,
    pub stage: ExecutionStage,
    pub attempt: Option<ExecutionAttempt>,
    pub phase: ExecutionPhase,
    pub evidence: Option<ExecutionEvidence>,
    pub compensation: Option<VerifiedCompensation>,
    pub detail: Option<BoundedText<2048>>,
}

/// A checked description of recorded execution, never a dispatch capability.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ExecutionObservationData")]
pub struct ExecutionObservation {
    #[serde(flatten)]
    data: ExecutionObservationData,
}

impl ExecutionObservation {
    #[must_use]
    pub fn data(&self) -> &ExecutionObservationData {
        &self.data
    }

    #[must_use]
    pub fn last_reconciled_at(&self) -> Option<UnixTimestampMicros> {
        self.data.evidence.as_ref().map(|e| e.recorded_at).max(
            self.data
                .compensation
                .as_ref()
                .map(|c| c.evidence.recorded_at),
        )
    }

    pub(crate) fn validate_operation(&self, id: OperationId) -> Result<(), &'static str> {
        if let ExecutionPhase::Compensation {
            original_operation_id,
            original_attempt_id,
            ..
        } = self.data.phase
            && (original_operation_id == id
                || self
                    .data
                    .attempt
                    .is_some_and(|a| a.id == original_attempt_id))
        {
            return Err("compensation must preserve a distinct original operation and attempt");
        }
        if self
            .data
            .compensation
            .as_ref()
            .is_some_and(|c| c.operation_id == id)
        {
            return Err("operation cannot compensate itself");
        }
        Ok(())
    }
}

fn validate_evidence(
    evidence: &ExecutionEvidence,
    attempt: ExecutionAttempt,
) -> Result<(), &'static str> {
    let started = attempt
        .started_at
        .ok_or("evidence requires a started attempt")?;
    if evidence.attempt_id != attempt.id
        || evidence.observed_at < started
        || evidence.recorded_at < evidence.observed_at
    {
        return Err("evidence attempt or observation time does not match");
    }
    if let ExecutionOutcome::LocalCommitted {
        before_revision,
        after_revision,
        revision,
        ..
    } = evidence.outcome
        && (before_revision == after_revision || revision == 0)
    {
        return Err("local commit requires distinct revisions and a nonzero revision");
    }
    Ok(())
}

impl TryFrom<ExecutionObservationData> for ExecutionObservation {
    type Error = &'static str;

    fn try_from(data: ExecutionObservationData) -> Result<Self, Self::Error> {
        use ExecutionStage as S;
        match data.stage {
            S::Prepared | S::Approved if data.attempt.is_some() => {
                return Err("unsent approval state cannot own an active attempt");
            }
            S::DispatchPending if !data.attempt.is_some_and(|a| a.started_at.is_none()) => {
                return Err("pending dispatch requires an unstarted attempt");
            }
            S::Cancelled if data.attempt.is_some_and(|a| a.started_at.is_some()) => {
                return Err("local cancellation cannot erase a started attempt");
            }
            S::Attempting
            | S::Accepted
            | S::Verified
            | S::NeedsReconciliation
            | S::Compensating
            | S::Compensated
                if !data.attempt.is_some_and(|a| a.started_at.is_some()) =>
            {
                return Err("sent state requires a started attempt");
            }
            _ => {}
        }
        if let Some(evidence) = &data.evidence {
            validate_evidence(evidence, data.attempt.ok_or("evidence without an attempt")?)?;
        }
        let outcome = data.evidence.as_ref().map(|e| e.outcome);
        let committed = outcome.is_some_and(|o| o.committed_reference().is_some());
        let compensable = outcome.is_some_and(ExecutionOutcome::is_compensating_write);
        let valid = match data.stage {
            S::Verified => committed,
            S::Compensated => compensable,
            S::Failed => {
                outcome.is_none_or(|o| matches!(o, ExecutionOutcome::ProvenNotCommitted { .. }))
            }
            S::NeedsReconciliation => {
                outcome.is_none_or(|o| matches!(o, ExecutionOutcome::Inconclusive {}))
            }
            S::Prepared
            | S::Approved
            | S::DispatchPending
            | S::Attempting
            | S::Accepted
            | S::Cancelled
            | S::Compensating => outcome.is_none(),
        };
        if !valid {
            return Err("execution stage contradicts its recorded evidence");
        }
        if (data.stage == S::Compensated) != data.compensation.is_some() {
            return Err("compensated state requires separate verified compensation");
        }
        if data.stage == S::Compensated && !matches!(data.phase, ExecutionPhase::Original {}) {
            return Err("compensated state must describe an original operation");
        }
        if let Some(compensation) = &data.compensation {
            validate_evidence(&compensation.evidence, compensation.attempt)?;
            let original_recorded_at = data
                .evidence
                .as_ref()
                .ok_or("compensation requires original committed evidence")?
                .recorded_at;
            if compensation
                .attempt
                .started_at
                .is_none_or(|started| started < original_recorded_at)
            {
                return Err("compensation cannot predate original committed evidence");
            }
            if !compensation.evidence.outcome.is_compensating_write()
                || data
                    .attempt
                    .is_some_and(|a| a.id == compensation.attempt.id)
                || data
                    .evidence
                    .as_ref()
                    .is_some_and(|e| e.evidence_id == compensation.evidence.evidence_id)
            {
                return Err("compensation requires its own committed write attempt and evidence");
            }
        }
        Ok(Self { data })
    }
}
