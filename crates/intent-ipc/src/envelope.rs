use crate::{Frame, FrameLane, WireError, WireErrorCode, WireLimits};
use intent_contracts::{CancellationId, RequestId, SchemaVersion, TraceId, UnixTimestampMicros};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EnvelopeKind {
    Request { request_id: RequestId },
    Response { request_id: RequestId },
    Event,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Envelope<T> {
    schema_version: SchemaVersion,
    trace_id: TraceId,
    #[serde(default)]
    cancellation_id: Option<CancellationId>,
    #[serde(default)]
    deadline: Option<UnixTimestampMicros>,
    message: EnvelopeKind,
    payload: T,
}

impl<T> Envelope<T> {
    #[must_use]
    pub const fn request(trace_id: TraceId, request_id: RequestId, payload: T) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            trace_id,
            cancellation_id: None,
            deadline: None,
            message: EnvelopeKind::Request { request_id },
            payload,
        }
    }

    #[must_use]
    pub const fn response(trace_id: TraceId, request_id: RequestId, payload: T) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            trace_id,
            cancellation_id: None,
            deadline: None,
            message: EnvelopeKind::Response { request_id },
            payload,
        }
    }

    #[must_use]
    pub const fn event(trace_id: TraceId, payload: T) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            trace_id,
            cancellation_id: None,
            deadline: None,
            message: EnvelopeKind::Event,
            payload,
        }
    }

    #[must_use]
    pub const fn with_cancellation_id(mut self, cancellation_id: CancellationId) -> Self {
        self.cancellation_id = Some(cancellation_id);
        self
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: UnixTimestampMicros) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub const fn schema_version(&self) -> SchemaVersion {
        self.schema_version
    }

    #[must_use]
    pub const fn trace_id(&self) -> TraceId {
        self.trace_id
    }

    #[must_use]
    pub const fn cancellation_id(&self) -> Option<CancellationId> {
        self.cancellation_id
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<UnixTimestampMicros> {
        self.deadline
    }

    #[must_use]
    pub const fn message(&self) -> EnvelopeKind {
        self.message
    }

    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    #[must_use]
    pub fn into_payload(self) -> T {
        self.payload
    }
}

pub fn encode_control<T: Serialize>(
    envelope: &Envelope<T>,
    limits: WireLimits,
) -> Result<Frame, WireError> {
    let payload = serde_json::to_vec(envelope).map_err(|error| {
        WireError::new(
            WireErrorCode::InvalidEnvelope,
            format!("failed to serialize control envelope: {error}"),
        )
    })?;

    if payload.len() > limits.max_control_frame_bytes {
        return Err(WireError::new(
            WireErrorCode::FrameTooLarge,
            format!(
                "control envelope is {} bytes but limit is {}",
                payload.len(),
                limits.max_control_frame_bytes
            ),
        ));
    }

    Ok(Frame::new(FrameLane::Control, payload))
}

pub fn decode_control<T: DeserializeOwned>(
    frame: &Frame,
    limits: WireLimits,
) -> Result<Envelope<T>, WireError> {
    if frame.lane() != FrameLane::Control {
        return Err(WireError::new(
            WireErrorCode::WrongLane,
            "typed envelopes can only be decoded from the control lane",
        ));
    }

    if frame.payload().len() > limits.max_control_frame_bytes {
        return Err(WireError::new(
            WireErrorCode::FrameTooLarge,
            "received control frame exceeds configured limit",
        ));
    }

    preflight_json_structure(frame.payload(), limits.max_json_depth)?;
    let value: Value = serde_json::from_slice(frame.payload()).map_err(|error| {
        WireError::new(
            WireErrorCode::MalformedJson,
            format!("malformed control JSON: {error}"),
        )
    })?;
    validate_json_value(&value, limits)?;

    let envelope: Envelope<T> = serde_json::from_value(value).map_err(|error| {
        WireError::new(
            WireErrorCode::InvalidEnvelope,
            format!("invalid control envelope: {error}"),
        )
    })?;

    if envelope.schema_version != SchemaVersion::V1 {
        return Err(WireError::new(
            WireErrorCode::UnsupportedSchema,
            format!(
                "unsupported envelope schema {}.{}",
                envelope.schema_version.major(),
                envelope.schema_version.minor()
            ),
        ));
    }

    Ok(envelope)
}

fn preflight_json_structure(input: &[u8], max_depth: usize) -> Result<(), WireError> {
    let max_depth = max_depth.max(1);
    let mut depth = 0_usize;
    let mut in_string = false;
    let mut escaped = false;

    for byte in input {
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }
            match byte {
                b'\\' => escaped = true,
                b'"' => in_string = false,
                _ => {}
            }
            continue;
        }

        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => {
                depth = depth.checked_add(1).ok_or_else(|| {
                    WireError::new(WireErrorCode::JsonTooDeep, "JSON depth overflow")
                })?;
                if depth > max_depth {
                    return Err(WireError::new(
                        WireErrorCode::JsonTooDeep,
                        format!("JSON depth exceeds configured limit {max_depth}"),
                    ));
                }
            }
            b'}' | b']' => {
                depth = depth.checked_sub(1).ok_or_else(|| {
                    WireError::new(
                        WireErrorCode::MalformedJson,
                        "JSON contains an unmatched closing delimiter",
                    )
                })?;
            }
            _ => {}
        }
    }

    if in_string || escaped || depth != 0 {
        return Err(WireError::new(
            WireErrorCode::MalformedJson,
            "JSON ended with an unterminated string or container",
        ));
    }
    Ok(())
}

fn validate_json_value(value: &Value, limits: WireLimits) -> Result<(), WireError> {
    let mut nodes = 0_usize;
    validate_json_node(value, 1, &mut nodes, limits)
}

fn validate_json_node(
    value: &Value,
    depth: usize,
    nodes: &mut usize,
    limits: WireLimits,
) -> Result<(), WireError> {
    *nodes = nodes.checked_add(1).ok_or_else(|| {
        WireError::new(
            WireErrorCode::JsonNodeLimitExceeded,
            "JSON node count overflow",
        )
    })?;
    if *nodes > limits.max_json_nodes {
        return Err(WireError::new(
            WireErrorCode::JsonNodeLimitExceeded,
            format!("JSON node count exceeds limit {}", limits.max_json_nodes),
        ));
    }

    if depth > limits.max_json_depth.max(1) {
        return Err(WireError::new(
            WireErrorCode::JsonTooDeep,
            format!(
                "JSON depth exceeds configured limit {}",
                limits.max_json_depth
            ),
        ));
    }

    match value {
        Value::Array(values) => {
            validate_collection_len(values.len(), limits)?;
            for child in values {
                validate_json_node(child, depth + 1, nodes, limits)?;
            }
        }
        Value::Object(values) => {
            validate_collection_len(values.len(), limits)?;
            for child in values.values() {
                validate_json_node(child, depth + 1, nodes, limits)?;
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
    Ok(())
}

fn validate_collection_len(len: usize, limits: WireLimits) -> Result<(), WireError> {
    if len > limits.max_collection_entries {
        return Err(WireError::new(
            WireErrorCode::CollectionTooLarge,
            format!(
                "JSON collection contains {len} entries but limit is {}",
                limits.max_collection_entries
            ),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Envelope, EnvelopeKind, decode_control, encode_control};
    use crate::{Frame, FrameLane, WireErrorCode, WireLimits};
    use intent_contracts::{CancellationId, RequestId, TraceId, UnixTimestampMicros};
    use serde::{Deserialize, Serialize};
    use std::error::Error;
    use std::str::FromStr;

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    struct TestPayload {
        value: String,
    }

    #[test]
    fn request_round_trip_preserves_correlation_deadline_and_cancellation()
    -> Result<(), Box<dyn Error>> {
        let limits = WireLimits::for_tests();
        let trace = TraceId::from_str("018f47f7-5a86-7c00-8000-000000000401")?;
        let request = RequestId::from_str("018f47f7-5a86-7c00-8000-000000000402")?;
        let cancellation = CancellationId::from_str("018f47f7-5a86-7c00-8000-000000000403")?;
        let deadline = UnixTimestampMicros::try_new(1_700_000_000_000_000)?;
        let envelope = Envelope::request(
            trace,
            request,
            TestPayload {
                value: "bounded".to_owned(),
            },
        )
        .with_cancellation_id(cancellation)
        .with_deadline(deadline);

        let frame = encode_control(&envelope, limits)?;
        let decoded: Envelope<TestPayload> = decode_control(&frame, limits)?;

        assert_eq!(decoded.trace_id(), trace);
        assert_eq!(decoded.cancellation_id(), Some(cancellation));
        assert_eq!(decoded.deadline(), Some(deadline));
        assert_eq!(
            decoded.message(),
            EnvelopeKind::Request {
                request_id: request
            }
        );
        assert_eq!(decoded.payload(), envelope.payload());
        Ok(())
    }

    #[test]
    fn deeply_nested_json_is_rejected_before_typed_deserialization() -> Result<(), Box<dyn Error>> {
        let limits = WireLimits::for_tests();
        let mut payload = "[".repeat(limits.max_json_depth + 1);
        payload.push('0');
        payload.push_str(&"]".repeat(limits.max_json_depth + 1));
        let frame = Frame::new(FrameLane::Control, payload.into_bytes());

        let Err(error) = decode_control::<TestPayload>(&frame, limits) else {
            return Err("deep JSON unexpectedly decoded".into());
        };
        assert_eq!(error.code(), WireErrorCode::JsonTooDeep);
        Ok(())
    }

    #[test]
    fn oversized_collection_is_rejected_with_stable_error() -> Result<(), Box<dyn Error>> {
        let limits = WireLimits::for_tests();
        let array = vec!["0"; limits.max_collection_entries + 1].join(",");
        let frame = Frame::new(FrameLane::Control, format!("[{array}]").into_bytes());

        let Err(error) = decode_control::<TestPayload>(&frame, limits) else {
            return Err("oversized collection unexpectedly decoded".into());
        };
        assert_eq!(error.code(), WireErrorCode::CollectionTooLarge);
        Ok(())
    }

    #[test]
    fn malformed_json_is_rejected_with_stable_error() -> Result<(), Box<dyn Error>> {
        let limits = WireLimits::for_tests();
        let frame = Frame::new(FrameLane::Control, b"{\"value\":1".to_vec());
        let Err(error) = decode_control::<TestPayload>(&frame, limits) else {
            return Err("malformed JSON unexpectedly decoded".into());
        };
        assert_eq!(error.code(), WireErrorCode::MalformedJson);
        Ok(())
    }

    #[test]
    fn braces_inside_strings_do_not_count_toward_depth() -> Result<(), Box<dyn Error>> {
        let limits = WireLimits::for_tests();
        let trace = TraceId::from_str("018f47f7-5a86-7c00-8000-000000000411")?;
        let request = RequestId::from_str("018f47f7-5a86-7c00-8000-000000000412")?;
        let value = r#"{{[["quoted"]]}}"#.to_owned();
        let envelope = Envelope::request(
            trace,
            request,
            TestPayload {
                value: value.clone(),
            },
        );
        let frame = encode_control(&envelope, limits)?;
        let decoded: Envelope<TestPayload> = decode_control(&frame, limits)?;

        assert_eq!(decoded.payload().value, value);
        Ok(())
    }

    #[test]
    fn artifact_lane_cannot_be_typed_as_control_envelope() -> Result<(), Box<dyn Error>> {
        let frame = Frame::new(FrameLane::Artifact, b"{}".to_vec());
        let Err(error) = decode_control::<TestPayload>(&frame, WireLimits::for_tests()) else {
            return Err("artifact frame unexpectedly decoded as control envelope".into());
        };
        assert_eq!(error.code(), WireErrorCode::WrongLane);
        Ok(())
    }
}
