use crate::BrokerError;
use intent_contracts::{
    BoundedText, ContentHash, RequestId, UnixTimestampMicros, WorkerInstanceId,
};
use intent_recovery::AttemptBinding;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
use intent_state::{DispatchMetadata, DurableSendAttempt};
use intent_supervisor::WorkerScope;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use uuid::Uuid;

/// Larger payloads require an explicit artifact/bulk-transport integration, not implicit truncation.
pub const MAX_INLINE_ACTION_BYTES: usize = 1024;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireInvocation {
    schema_version: u16,
    runtime_epoch: Uuid,
    worker: WorkerInstanceId,
    request: RequestId,
    scope: WorkerScope,
    binding: AttemptBinding,
    deadline: UnixTimestampMicros,
    destination: BoundedText<512>,
    message_kind: BoundedText<128>,
    payload_hex: BoundedText<2048>,
}

/// Validated transport data, not an approval, a grant or proof of external completion.
/// It can only be decoded after the worker validates the authenticated Execute envelope.
pub struct WorkerInvocation {
    wire: WireInvocation,
    payload: Vec<u8>,
}
impl WorkerInvocation {
    pub fn decode(
        input: &str,
        worker: WorkerInstanceId,
        request: RequestId,
        scope: WorkerScope,
        capability: intent_contracts::CapabilityId,
        now: UnixTimestampMicros,
    ) -> Result<Self, BrokerError> {
        if input.len() > 4096 {
            return Err(BrokerError::Invalid("invocation byte budget"));
        }
        let wire: WireInvocation =
            serde_json::from_str(input).map_err(|_| BrokerError::Invalid("invocation schema"))?;
        if wire.schema_version != 1
            || wire.worker != worker
            || wire.request != request
            || wire.scope != scope
            || wire.binding.account_id != scope.account_id
            || wire.binding.capability_id != capability
            || wire.deadline <= now
        {
            return Err(BrokerError::Invalid("invocation identity or deadline"));
        }
        let text = wire.payload_hex.as_str().as_bytes();
        if !text.len().is_multiple_of(2)
            || text.iter().any(|v| !matches!(v, b'0'..=b'9' | b'a'..=b'f'))
        {
            return Err(BrokerError::Invalid("noncanonical payload bytes"));
        }
        let digit = |v: u8| if v <= b'9' { v - b'0' } else { v - b'a' + 10 };
        let payload: Vec<u8> = text
            .as_chunks::<2>()
            .0
            .iter()
            .map(|v| digit(v[0]) * 16 + digit(v[1]))
            .collect();
        if ContentHash::from_bytes(Sha256::digest(&payload).into()) != wire.binding.arguments_hash {
            return Err(BrokerError::Invalid("invocation payload hash"));
        }
        Ok(Self { wire, payload })
    }
    pub fn binding(&self) -> AttemptBinding {
        self.wire.binding
    }
    pub fn runtime_epoch(&self) -> Uuid {
        self.wire.runtime_epoch
    }
    pub fn destination(&self) -> &str {
        self.wire.destination.as_str()
    }
    pub fn message_kind(&self) -> &str {
        self.wire.message_kind.as_str()
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}
impl fmt::Debug for WorkerInvocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorkerInvocation")
            .field("operation_id", &self.wire.binding.operation_id)
            .field("attempt_id", &self.wire.binding.attempt_id)
            .field("payload_bytes", &self.payload.len())
            .finish_non_exhaustive()
    }
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(crate) fn encode(
    attempt: &DurableSendAttempt,
    metadata: DispatchMetadata,
    epoch: Uuid,
    worker: WorkerInstanceId,
    request: RequestId,
) -> Result<BoundedText<4096>, BrokerError> {
    if attempt.operation_id() != metadata.operation_id
        || attempt.attempt_id() != metadata.attempt_id
        || attempt.payload().len() > MAX_INLINE_ACTION_BYTES
    {
        return Err(BrokerError::Invalid(
            "attempt binding or inline payload budget",
        ));
    }
    let mut hex = String::with_capacity(attempt.payload().len() * 2);
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    for b in attempt.payload() {
        hex.push(char::from(DIGITS[usize::from(b >> 4)]));
        hex.push(char::from(DIGITS[usize::from(b & 15)]));
    }
    let wire = WireInvocation {
        schema_version: 1,
        runtime_epoch: epoch,
        worker,
        request,
        scope: WorkerScope {
            task_id: metadata.task_id,
            account_id: metadata.account_id,
        },
        binding: AttemptBinding {
            operation_id: metadata.operation_id,
            attempt_id: metadata.attempt_id,
            account_id: metadata.account_id,
            capability_id: metadata.capability_id,
            arguments_hash: ContentHash::from_bytes(Sha256::digest(attempt.payload()).into()),
        },
        deadline: metadata.deadline,
        destination: BoundedText::try_new(attempt.destination())
            .map_err(|_| BrokerError::Invalid("destination budget"))?,
        message_kind: BoundedText::try_new(attempt.message_kind())
            .map_err(|_| BrokerError::Invalid("message kind budget"))?,
        payload_hex: BoundedText::try_new(hex)
            .map_err(|_| BrokerError::Invalid("payload budget"))?,
    };
    let text =
        serde_json::to_string(&wire).map_err(|_| BrokerError::Invalid("invocation encoding"))?;
    BoundedText::try_new(text).map_err(|_| BrokerError::Invalid("encoded invocation budget"))
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(crate) fn check_shape(payload_bytes: u64, routing_json_bytes: u64) -> Result<(), BrokerError> {
    // Fixed v1 identity/header fields fit within 900 bytes, including maximum timestamp spelling.
    if payload_bytes > MAX_INLINE_ACTION_BYTES as u64
        || payload_bytes
            .checked_mul(2)
            .and_then(|v| v.checked_add(routing_json_bytes))
            .and_then(|v| v.checked_add(900))
            .is_none_or(|v| v > 4096)
    {
        return Err(BrokerError::Invalid("inline invocation encoding budget"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use intent_contracts::{AccountId, CapabilityId, OperationAttemptId, OperationId, TaskId};
    type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
    fn sample() -> Result<WireInvocation> {
        let account = AccountId::from_uuid(Uuid::new_v4());
        Ok(WireInvocation {
            schema_version: 1,
            runtime_epoch: Uuid::new_v4(),
            worker: WorkerInstanceId::from_uuid(Uuid::new_v4()),
            request: RequestId::from_uuid(Uuid::new_v4()),
            scope: WorkerScope {
                task_id: TaskId::from_uuid(Uuid::new_v4()),
                account_id: account,
            },
            binding: AttemptBinding {
                operation_id: OperationId::from_uuid(Uuid::new_v4()),
                attempt_id: OperationAttemptId::from_uuid(Uuid::new_v4()),
                account_id: account,
                capability_id: CapabilityId::from_uuid(Uuid::new_v4()),
                arguments_hash: ContentHash::from_bytes(Sha256::digest([0, 255]).into()),
            },
            deadline: UnixTimestampMicros::try_new(100)?,
            destination: BoundedText::try_new("secret://endpoint")?,
            message_kind: BoundedText::try_new("append")?,
            payload_hex: BoundedText::try_new("00ff")?,
        })
    }
    fn decode(
        text: &str,
        wire: &WireInvocation,
    ) -> std::result::Result<WorkerInvocation, BrokerError> {
        WorkerInvocation::decode(
            text,
            wire.worker,
            wire.request,
            wire.scope,
            wire.binding.capability_id,
            UnixTimestampMicros::try_new(0).map_err(|_| BrokerError::Clock)?,
        )
    }
    #[test]
    fn invocation_roundtrip_is_exact_and_debug_is_redacted() -> Result {
        let wire = sample()?;
        let value = decode(&serde_json::to_string(&wire)?, &wire)?;
        assert_eq!(value.payload(), [0, 255]);
        assert_eq!(value.binding(), wire.binding);
        let debug = format!("{value:?}");
        assert!(!debug.contains("secret://endpoint"));
        assert!(!debug.contains("00ff"));
        Ok(())
    }
    #[test]
    fn nested_identity_duplicate_unknown_field_and_payload_corruption_fail_closed() -> Result {
        let wire = sample()?;
        let text = serde_json::to_string(&wire)?;
        let duplicate = text.replacen(
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1",
            1,
        );
        assert!(decode(&duplicate, &wire).is_err());
        for field in ["worker", "request"] {
            let mut value = serde_json::to_value(&wire)?;
            value[field] = serde_json::json!(Uuid::new_v4());
            assert!(decode(&serde_json::to_string(&value)?, &wire).is_err());
        }
        for field in ["account_id", "task_id"] {
            let mut value = serde_json::to_value(&wire)?;
            value["scope"][field] = serde_json::json!(Uuid::new_v4());
            assert!(decode(&serde_json::to_string(&value)?, &wire).is_err());
        }
        for field in ["account_id", "capability_id"] {
            let mut value = serde_json::to_value(&wire)?;
            value["binding"][field] = serde_json::json!(Uuid::new_v4());
            assert!(decode(&serde_json::to_string(&value)?, &wire).is_err());
        }
        for invalid in ["00FF", "0", "00fe", "0z"] {
            let mut value = serde_json::to_value(&wire)?;
            value["payload_hex"] = serde_json::json!(invalid);
            assert!(decode(&serde_json::to_string(&value)?, &wire).is_err());
        }
        let mut unknown = serde_json::to_value(&wire)?;
        unknown["approved"] = serde_json::json!(true);
        assert!(decode(&serde_json::to_string(&unknown)?, &wire).is_err());
        assert!(
            WorkerInvocation::decode(
                &text,
                wire.worker,
                wire.request,
                wire.scope,
                wire.binding.capability_id,
                wire.deadline
            )
            .is_err()
        );
        Ok(())
    }
    #[test]
    fn encoded_budget_preflight_covers_maximum_payload_and_escaped_routing() -> Result {
        let mut wire = sample()?;
        wire.payload_hex = BoundedText::try_new("00".repeat(MAX_INLINE_ACTION_BYTES))?;
        let routing = serde_json::to_string(&wire.destination)?.len()
            + serde_json::to_string(&wire.message_kind)?.len();
        check_shape(MAX_INLINE_ACTION_BYTES as u64, routing as u64)?;
        assert!(serde_json::to_string(&wire)?.len() <= 4096);
        assert!(check_shape(1025, 0).is_err());
        assert!(check_shape(1024, 3072).is_err());
        assert!(check_shape(u64::MAX, u64::MAX).is_err());
        Ok(())
    }
}
