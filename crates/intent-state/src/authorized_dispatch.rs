use crate::{
    DurableOperationState, NewOutboxMessage, OutboxError, OutboxMessage, OutboxState, StateError,
    StateStore,
};
use intent_contracts::{
    ActionProposal, Approval, ApprovalState, BoundedText, ContentHash, OperationAttemptId,
    OperationId, OutboxMessageId, UnixTimestampMicros,
};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AuthorizedDispatchError {
    State(StateError),
    Outbox(OutboxError),
    OperationNotFound(OperationId),
    BindingMismatch(&'static str),
    ApprovalNotActive,
    ApprovalNotYetEffective,
    ApprovalExpired,
    ProposalExpired,
}

impl fmt::Display for AuthorizedDispatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::State(error) => write!(
                formatter,
                "state error while binding dispatch records: {error}"
            ),
            Self::Outbox(error) => write!(formatter, "outbox error after record binding: {error}"),
            Self::OperationNotFound(operation_id) => {
                write!(formatter, "durable operation {operation_id} does not exist")
            }
            Self::BindingMismatch(field) => {
                write!(formatter, "dispatch records do not match {field}")
            }
            Self::ApprovalNotActive => formatter.write_str("approval record is not approved"),
            Self::ApprovalNotYetEffective => {
                formatter.write_str("approval record timestamp is later than the staging time")
            }
            Self::ApprovalExpired => formatter.write_str("approval record has expired"),
            Self::ProposalExpired => formatter.write_str("action proposal has expired"),
        }
    }
}

impl Error for AuthorizedDispatchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::State(error) => Some(error),
            Self::Outbox(error) => Some(error),
            _ => None,
        }
    }
}

impl From<StateError> for AuthorizedDispatchError {
    fn from(value: StateError) -> Self {
        Self::State(value)
    }
}

impl From<OutboxError> for AuthorizedDispatchError {
    fn from(value: OutboxError) -> Self {
        Self::Outbox(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StagedOutbox {
    outbox_id: OutboxMessageId,
    operation_id: OperationId,
    attempt_identity: OperationAttemptId,
    created_at: UnixTimestampMicros,
}

impl StagedOutbox {
    #[must_use]
    pub const fn outbox_id(self) -> OutboxMessageId {
        self.outbox_id
    }

    #[must_use]
    pub const fn operation_id(self) -> OperationId {
        self.operation_id
    }

    #[must_use]
    pub const fn attempt_identity(self) -> OperationAttemptId {
        self.attempt_identity
    }

    #[must_use]
    pub const fn created_at(self) -> UnixTimestampMicros {
        self.created_at
    }
}

#[must_use]
fn hash_payload(payload: &[u8]) -> ContentHash {
    ContentHash::from_bytes(Sha256::digest(payload).into())
}

struct ExpectedStoredOutbox<'a> {
    outbox_id: OutboxMessageId,
    operation_id: OperationId,
    attempt_identity: OperationAttemptId,
    destination: &'a BoundedText<512>,
    message_kind: &'a BoundedText<128>,
    payload_hash: ContentHash,
    staged_at: UnixTimestampMicros,
}

fn verify_stored_outbox_projection(
    stored: &OutboxMessage,
    expected: ExpectedStoredOutbox<'_>,
) -> Result<(), AuthorizedDispatchError> {
    let exact_projection = stored.outbox_id() == expected.outbox_id
        && stored.operation_id() == expected.operation_id
        && stored.attempt_identity() == expected.attempt_identity
        && stored.destination() == expected.destination
        && stored.message_kind() == expected.message_kind
        && stored.payload_hash() == expected.payload_hash
        && hash_payload(stored.payload()) == expected.payload_hash
        && stored.state() == OutboxState::Pending
        && stored.lease_owner().is_none()
        && stored.lease_expires_at().is_none()
        && stored.dispatch_started_at().is_none()
        && stored.created_at() == expected.staged_at
        && stored.updated_at() == expected.staged_at;
    if exact_projection {
        Ok(())
    } else {
        Err(AuthorizedDispatchError::BindingMismatch(
            "persisted outbox projection",
        ))
    }
}

impl StateStore {
    /// Stage an outbox message only when persisted records and exact payload bytes agree.
    ///
    /// This is a record-binding check, not a grant of executable authority or user consent.
    /// Callers must separately enforce the runtime-owner, authority, policy, and provider gates
    /// before any external effect is attempted. The validation time must also be the
    /// persisted staging time and cannot precede the operation's latest transition. The return
    /// value intentionally contains no executable destination, message kind, payload, or hash.
    pub fn stage_record_bound_outbox(
        &mut self,
        proposal: &ActionProposal,
        approval: &Approval,
        new: NewOutboxMessage,
        expected_operation_revision: u64,
        now: UnixTimestampMicros,
    ) -> Result<StagedOutbox, AuthorizedDispatchError> {
        if new.payload.len() > crate::MAX_OUTBOX_PAYLOAD_BYTES {
            return Err(OutboxError::PayloadTooLarge(new.payload.len()).into());
        }
        let operation = self
            .load_operation(new.operation_id)?
            .ok_or(AuthorizedDispatchError::OperationNotFound(new.operation_id))?;

        if new.created_at != now || now < operation.updated_at() {
            return Err(AuthorizedDispatchError::BindingMismatch("staging time"));
        }
        if operation.state() != DurableOperationState::Approved {
            return Err(AuthorizedDispatchError::BindingMismatch("operation state"));
        }
        if operation.action_proposal_id() != proposal.action_proposal_id() {
            return Err(AuthorizedDispatchError::BindingMismatch("proposal id"));
        }
        if operation.task_id() != proposal.task_id() {
            return Err(AuthorizedDispatchError::BindingMismatch("task id"));
        }
        if operation.account_id() != proposal.account_id() {
            return Err(AuthorizedDispatchError::BindingMismatch("account id"));
        }
        if operation.capability_id() != proposal.capability_id() {
            return Err(AuthorizedDispatchError::BindingMismatch("capability id"));
        }
        if operation.arguments_hash() != proposal.arguments_hash() {
            return Err(AuthorizedDispatchError::BindingMismatch("argument hash"));
        }
        if proposal.canonical_arguments().content_hash() != proposal.arguments_hash() {
            return Err(AuthorizedDispatchError::BindingMismatch(
                "canonical argument hash",
            ));
        }
        let payload_size = u64::try_from(new.payload.len())
            .map_err(|_| AuthorizedDispatchError::BindingMismatch("payload byte size"))?;
        if proposal.canonical_arguments().byte_size().as_bytes() != payload_size {
            return Err(AuthorizedDispatchError::BindingMismatch(
                "payload byte size",
            ));
        }
        let payload_hash = hash_payload(&new.payload);
        if payload_hash != proposal.arguments_hash() {
            return Err(AuthorizedDispatchError::BindingMismatch("payload hash"));
        }
        if approval.action_proposal_id() != proposal.action_proposal_id() {
            return Err(AuthorizedDispatchError::BindingMismatch(
                "approval proposal id",
            ));
        }
        if approval.exact_arguments_hash() != proposal.arguments_hash() {
            return Err(AuthorizedDispatchError::BindingMismatch(
                "approval argument hash",
            ));
        }
        if proposal
            .expires_at()
            .is_some_and(|expires_at| expires_at.get() <= now.get())
        {
            return Err(AuthorizedDispatchError::ProposalExpired);
        }
        match approval.state() {
            ApprovalState::Approved {
                approved_at,
                expires_at,
            } => {
                if approved_at.get() > now.get() {
                    return Err(AuthorizedDispatchError::ApprovalNotYetEffective);
                }
                if expires_at.is_some_and(|expires_at| expires_at.get() <= now.get()) {
                    return Err(AuthorizedDispatchError::ApprovalExpired);
                }
            }
            _ => return Err(AuthorizedDispatchError::ApprovalNotActive),
        }

        let binding = proposal
            .binding()
            .ok_or(AuthorizedDispatchError::BindingMismatch("unbound proposal"))?;
        if operation.binding() != Some(&binding) || approval.exact_binding() != Some(&binding) {
            return Err(AuthorizedDispatchError::BindingMismatch("action binding"));
        }
        let staged = StagedOutbox {
            outbox_id: new.outbox_id,
            operation_id: new.operation_id,
            attempt_identity: new.attempt_identity,
            created_at: new.created_at,
        };
        let expected_destination = new.destination.clone();
        let expected_message_kind = new.message_kind.clone();
        let stored = self
            .stage_outbox(new, expected_operation_revision)
            .map_err(AuthorizedDispatchError::from)?;
        verify_stored_outbox_projection(
            &stored,
            ExpectedStoredOutbox {
                outbox_id: staged.outbox_id,
                operation_id: staged.operation_id,
                attempt_identity: staged.attempt_identity,
                destination: &expected_destination,
                message_kind: &expected_message_kind,
                payload_hash,
                staged_at: staged.created_at,
            },
        )?;
        Ok(staged)
    }
}

#[cfg(test)]
mod tests {
    mod rejection_regressions;
    use super::{AuthorizedDispatchError, hash_payload};
    use crate::{
        DurableOperationState, NewDurableOperation, NewOutboxMessage, OperationTransition,
        OutboxState, StateStore,
    };
    use intent_contracts::{
        AccountId, ActionProposal, ActionProposalDescriptor, ActionProposalId, Approval,
        ApprovalId, ApprovalRequirement, ApprovalState, ArtifactReference, BoundedText, ByteSize,
        CapabilityEffectClass, CapabilityId, ContentHash, OperationAttemptId, OperationId,
        OutboxMessageId, SchemaVersion, TaskId, UnixTimestampMicros,
    };
    use serde_json::json;
    use std::error::Error;
    use std::str::FromStr;

    const ARGUMENTS: &[u8] = br#"{"cart":"stable"}"#;

    fn task_id() -> Result<TaskId, Box<dyn Error>> {
        Ok(TaskId::from_str("018f47f7-5a86-7c00-8000-000000000a01")?)
    }

    fn proposal_id() -> Result<ActionProposalId, Box<dyn Error>> {
        Ok(ActionProposalId::from_str(
            "018f47f7-5a86-7c00-8000-000000000a02",
        )?)
    }

    fn account_id() -> Result<AccountId, Box<dyn Error>> {
        Ok(AccountId::from_str("018f47f7-5a86-7c00-8000-000000000a03")?)
    }

    fn capability_id() -> Result<CapabilityId, Box<dyn Error>> {
        Ok(CapabilityId::from_str(
            "018f47f7-5a86-7c00-8000-000000000a04",
        )?)
    }

    fn operation_id() -> Result<OperationId, Box<dyn Error>> {
        Ok(OperationId::from_str(
            "018f47f7-5a86-7c00-8000-000000000a05",
        )?)
    }

    fn outbox_id() -> Result<OutboxMessageId, Box<dyn Error>> {
        Ok(OutboxMessageId::from_str(
            "018f47f7-5a86-7c00-8000-000000000a06",
        )?)
    }

    fn attempt_id() -> Result<OperationAttemptId, Box<dyn Error>> {
        Ok(OperationAttemptId::from_str(
            "018f47f7-5a86-7c00-8000-000000000a07",
        )?)
    }

    fn arguments_hash() -> ContentHash {
        hash_payload(ARGUMENTS)
    }

    fn proposal() -> Result<ActionProposal, Box<dyn Error>> {
        Ok(ActionProposal::new(
            proposal_id()?,
            task_id()?,
            capability_id()?,
            account_id()?,
            ActionProposalDescriptor {
                context: intent_contracts::ActionContext {
                    source_revision: ContentHash::from_bytes([7; 32]),
                    canonicalization: intent_contracts::CanonicalizationVersion::ExactBytesV1,
                },
                target_resource: None,
                expires_at: None,
                canonical_arguments: ArtifactReference::new(
                    "018f47f7-5a86-7c00-8000-000000000a08".parse()?,
                    arguments_hash(),
                    ByteSize::from_bytes(u64::try_from(ARGUMENTS.len())?),
                    BoundedText::try_new("application/json")?,
                ),
                effect_class: CapabilityEffectClass::IrreversibleOrUncertain,
                approval_requirement: ApprovalRequirement::Always,
            },
        ))
    }

    fn approval(state: ApprovalState) -> Result<Approval, Box<dyn Error>> {
        Ok(Approval::new(
            ApprovalId::from_str("018f47f7-5a86-7c00-8000-000000000a09")?,
            &proposal()?,
            state,
        )?)
    }

    fn new_operation() -> Result<NewDurableOperation, Box<dyn Error>> {
        Ok(NewDurableOperation {
            binding: proposal()?.binding(),
            operation_id: operation_id()?,
            task_id: task_id()?,
            action_proposal_id: proposal_id()?,
            account_id: account_id()?,
            capability_id: capability_id()?,
            arguments_hash: arguments_hash(),
            source_schema: SchemaVersion::V1,
            created_at: UnixTimestampMicros::try_new(100)?,
        })
    }

    fn approved_operation(store: &mut StateStore) -> Result<(), Box<dyn Error>> {
        let operation = store.create_operation(new_operation()?)?;
        store.transition_operation(
            operation.operation_id(),
            OperationTransition {
                expected_revision: operation.revision(),
                next_state: DurableOperationState::Approved,
                state_detail: None,
                attempt_identity: None,
                occurred_at: UnixTimestampMicros::try_new(110)?,
            },
        )?;
        Ok(())
    }

    fn message() -> Result<NewOutboxMessage, Box<dyn Error>> {
        Ok(NewOutboxMessage {
            outbox_id: outbox_id()?,
            operation_id: operation_id()?,
            attempt_identity: attempt_id()?,
            destination: BoundedText::try_new("connector://checkout")?,
            message_kind: BoundedText::try_new("commit")?,
            payload: ARGUMENTS.to_vec(),
            created_at: UnixTimestampMicros::try_new(120)?,
        })
    }

    fn approved_record() -> Result<Approval, Box<dyn Error>> {
        approval(ApprovalState::Approved {
            approved_at: UnixTimestampMicros::try_new(111)?,
            expires_at: Some(UnixTimestampMicros::try_new(200)?),
        })
    }

    fn assert_unstaged(store: &StateStore) -> Result<(), Box<dyn Error>> {
        assert!(store.load_outbox(outbox_id()?)?.is_none());
        let operation = store
            .load_operation(operation_id()?)?
            .ok_or("operation missing")?;
        assert_eq!(operation.state(), DurableOperationState::Approved);
        assert_eq!(operation.revision(), 1);
        assert_eq!(store.operation_journal(operation_id()?)?.len(), 2);
        Ok(())
    }

    #[test]
    fn exact_record_binding_stages_atomically() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        let staged = store.stage_record_bound_outbox(
            &proposal()?,
            &approved_record()?,
            message()?,
            1,
            UnixTimestampMicros::try_new(120)?,
        )?;
        assert_eq!(staged.outbox_id(), outbox_id()?);
        assert_eq!(staged.operation_id(), operation_id()?);
        assert_eq!(staged.attempt_identity(), attempt_id()?);
        assert_eq!(staged.created_at(), UnixTimestampMicros::try_new(120)?);
        assert_eq!(
            store
                .load_outbox(outbox_id()?)?
                .ok_or("staged outbox missing")?
                .state(),
            OutboxState::Pending
        );
        assert_eq!(
            store
                .load_operation(operation_id()?)?
                .ok_or("operation missing")?
                .state(),
            DurableOperationState::DispatchPending
        );
        assert_eq!(store.operation_journal(operation_id()?)?.len(), 3);
        Ok(())
    }

    #[test]
    fn rebound_payload_is_rejected_without_dispatch_residue() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;

        let mut same_size = message()?;
        same_size.payload = br#"{"cart":"stablE"}"#.to_vec();
        let Err(error) = store.stage_record_bound_outbox(
            &proposal()?,
            &approved_record()?,
            same_size,
            1,
            UnixTimestampMicros::try_new(120)?,
        ) else {
            return Err("same-size rebound payload unexpectedly staged".into());
        };
        assert!(matches!(
            error,
            AuthorizedDispatchError::BindingMismatch("payload hash")
        ));
        assert_unstaged(&store)?;

        let mut wrong_size = message()?;
        wrong_size.payload = br#"{"cart":"changed"}"#.to_vec();
        let Err(error) = store.stage_record_bound_outbox(
            &proposal()?,
            &approved_record()?,
            wrong_size,
            1,
            UnixTimestampMicros::try_new(120)?,
        ) else {
            return Err("wrong-size rebound payload unexpectedly staged".into());
        };
        assert!(matches!(
            error,
            AuthorizedDispatchError::BindingMismatch("payload byte size")
        ));
        assert_unstaged(&store)?;
        Ok(())
    }

    #[test]
    fn rebound_account_is_rejected_without_dispatch_residue() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        let mut value = serde_json::to_value(proposal()?)?;
        value["account_id"] = json!("018f47f7-5a86-7c00-8000-000000000aff");
        let rebound: ActionProposal = serde_json::from_value(value)?;
        let Err(error) = store.stage_record_bound_outbox(
            &rebound,
            &approved_record()?,
            message()?,
            1,
            UnixTimestampMicros::try_new(120)?,
        ) else {
            return Err("rebound account unexpectedly staged".into());
        };
        assert!(matches!(
            error,
            AuthorizedDispatchError::BindingMismatch("account id")
        ));
        assert_unstaged(&store)?;
        Ok(())
    }

    #[test]
    fn rebound_target_fails_closed_when_durable_binding_is_untargeted() -> Result<(), Box<dyn Error>>
    {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        let mut value = serde_json::to_value(proposal()?)?;
        value["target_resource"] = json!({
            "provider": "shop",
            "account": "account-a",
            "resource": "cart/17"
        });
        let targeted: ActionProposal = serde_json::from_value(value)?;
        let Err(error) = store.stage_record_bound_outbox(
            &targeted,
            &approved_record()?,
            message()?,
            1,
            UnixTimestampMicros::try_new(120)?,
        ) else {
            return Err(
                "targeted proposal unexpectedly staged without durable target binding".into(),
            );
        };
        assert!(matches!(
            error,
            AuthorizedDispatchError::BindingMismatch("action binding")
        ));
        assert_unstaged(&store)?;
        Ok(())
    }

    #[test]
    fn expired_records_leave_no_dispatch_residue() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        approved_operation(&mut store)?;
        let expired_approval = approval(ApprovalState::Approved {
            approved_at: UnixTimestampMicros::try_new(111)?,
            expires_at: Some(UnixTimestampMicros::try_new(120)?),
        })?;
        let Err(error) = store.stage_record_bound_outbox(
            &proposal()?,
            &expired_approval,
            message()?,
            1,
            UnixTimestampMicros::try_new(120)?,
        ) else {
            return Err("expired approval record unexpectedly staged".into());
        };
        assert!(matches!(error, AuthorizedDispatchError::ApprovalExpired));
        assert_unstaged(&store)?;

        let mut value = serde_json::to_value(proposal()?)?;
        value["expires_at"] = json!(119);
        let expired_proposal: ActionProposal = serde_json::from_value(value)?;
        let Err(error) = store.stage_record_bound_outbox(
            &expired_proposal,
            &approved_record()?,
            message()?,
            1,
            UnixTimestampMicros::try_new(120)?,
        ) else {
            return Err("expired proposal unexpectedly staged".into());
        };
        assert!(matches!(error, AuthorizedDispatchError::ProposalExpired));
        assert_unstaged(&store)?;
        Ok(())
    }
}
