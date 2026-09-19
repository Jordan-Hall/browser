use crate::StateStore;
use intent_contracts::{
    BoundedText, ContentHash, OperationAttemptId, OperationId, OutboxMessageId, UnixTimestampMicros,
};
use rusqlite::{OptionalExtension, Transaction, TransactionBehavior, params};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

pub const MAX_OUTBOX_PAYLOAD_BYTES: usize = 1024 * 1024;
pub const MAX_OUTBOX_CLAIM_BATCH: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutboxState {
    Pending,
    Leased,
    Attempting,
    Completed,
    Failed,
}

impl OutboxState {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Leased => "leased",
            Self::Attempting => "attempting",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }

    fn parse(value: &str) -> Result<Self, OutboxError> {
        match value {
            "pending" => Ok(Self::Pending),
            "leased" => Ok(Self::Leased),
            "attempting" => Ok(Self::Attempting),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            other => Err(OutboxError::InvalidStoredRecord(format!(
                "unknown outbox state {other}"
            ))),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewOutboxMessage {
    pub outbox_id: OutboxMessageId,
    pub operation_id: OperationId,
    pub attempt_identity: OperationAttemptId,
    pub destination: BoundedText<512>,
    pub message_kind: BoundedText<128>,
    pub payload: Vec<u8>,
    pub created_at: UnixTimestampMicros,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutboxMessage {
    outbox_id: OutboxMessageId,
    operation_id: OperationId,
    attempt_identity: OperationAttemptId,
    destination: BoundedText<512>,
    message_kind: BoundedText<128>,
    payload: Vec<u8>,
    payload_hash: ContentHash,
    state: OutboxState,
    lease_owner: Option<BoundedText<128>>,
    lease_expires_at: Option<UnixTimestampMicros>,
    dispatch_started_at: Option<UnixTimestampMicros>,
    created_at: UnixTimestampMicros,
    updated_at: UnixTimestampMicros,
}

impl OutboxMessage {
    #[must_use]
    pub const fn outbox_id(&self) -> OutboxMessageId {
        self.outbox_id
    }

    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    #[must_use]
    pub const fn attempt_identity(&self) -> OperationAttemptId {
        self.attempt_identity
    }

    #[must_use]
    pub fn destination(&self) -> &BoundedText<512> {
        &self.destination
    }

    #[must_use]
    pub fn message_kind(&self) -> &BoundedText<128> {
        &self.message_kind
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    #[must_use]
    pub const fn payload_hash(&self) -> ContentHash {
        self.payload_hash
    }

    #[must_use]
    pub const fn state(&self) -> OutboxState {
        self.state
    }

    #[must_use]
    pub fn lease_owner(&self) -> Option<&BoundedText<128>> {
        self.lease_owner.as_ref()
    }

    #[must_use]
    pub const fn lease_expires_at(&self) -> Option<UnixTimestampMicros> {
        self.lease_expires_at
    }

    #[must_use]
    pub const fn dispatch_started_at(&self) -> Option<UnixTimestampMicros> {
        self.dispatch_started_at
    }

    #[must_use]
    pub const fn created_at(&self) -> UnixTimestampMicros {
        self.created_at
    }

    #[must_use]
    pub const fn updated_at(&self) -> UnixTimestampMicros {
        self.updated_at
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutboxClaim {
    outbox_id: OutboxMessageId,
    operation_id: OperationId,
    attempt_identity: OperationAttemptId,
    lease_expires_at: UnixTimestampMicros,
}

impl OutboxClaim {
    #[must_use]
    pub const fn outbox_id(&self) -> OutboxMessageId {
        self.outbox_id
    }

    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    #[must_use]
    pub const fn attempt_identity(&self) -> OperationAttemptId {
        self.attempt_identity
    }

    #[must_use]
    pub const fn lease_expires_at(&self) -> UnixTimestampMicros {
        self.lease_expires_at
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatchAttempt {
    outbox_id: OutboxMessageId,
    operation_id: OperationId,
    attempt_identity: OperationAttemptId,
    destination: BoundedText<512>,
    message_kind: BoundedText<128>,
    payload: Vec<u8>,
    payload_hash: ContentHash,
}

impl DispatchAttempt {
    #[must_use]
    pub const fn outbox_id(&self) -> OutboxMessageId {
        self.outbox_id
    }

    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    #[must_use]
    pub const fn attempt_identity(&self) -> OperationAttemptId {
        self.attempt_identity
    }

    #[must_use]
    pub fn destination(&self) -> &BoundedText<512> {
        &self.destination
    }

    #[must_use]
    pub fn message_kind(&self) -> &BoundedText<128> {
        &self.message_kind
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    #[must_use]
    pub const fn payload_hash(&self) -> ContentHash {
        self.payload_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DispatchResult {
    Accepted,
    Ambiguous(BoundedText<2048>),
    Rejected(BoundedText<2048>),
}
