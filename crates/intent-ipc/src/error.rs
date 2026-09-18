use intent_contracts::BoundedText;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::error::Error;
use std::fmt;

const MAX_ERROR_DETAIL_BYTES: usize = 512;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum WireErrorCode {
    InvalidLane = 1,
    FrameTooLarge = 2,
    MalformedJson = 3,
    JsonTooDeep = 4,
    CollectionTooLarge = 5,
    JsonNodeLimitExceeded = 6,
    UnsupportedSchema = 7,
    WrongLane = 8,
    InvalidEnvelope = 9,
    AllocationFailed = 10,
    DuplicateJsonKey = 11,
    InvalidRecord = 12,
}

impl WireErrorCode {
    /// Stable v1 JSON spelling. Numeric discriminants are local diagnostics,
    /// not an alternative encoding accepted by the JSON protocol.
    #[must_use]
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::InvalidLane => "InvalidLane",
            Self::FrameTooLarge => "FrameTooLarge",
            Self::MalformedJson => "MalformedJson",
            Self::JsonTooDeep => "JsonTooDeep",
            Self::CollectionTooLarge => "CollectionTooLarge",
            Self::JsonNodeLimitExceeded => "JsonNodeLimitExceeded",
            Self::UnsupportedSchema => "UnsupportedSchema",
            Self::WrongLane => "WrongLane",
            Self::InvalidEnvelope => "InvalidEnvelope",
            Self::AllocationFailed => "AllocationFailed",
            Self::DuplicateJsonKey => "DuplicateJsonKey",
            Self::InvalidRecord => "InvalidRecord",
        }
    }
}

impl Serialize for WireErrorCode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.wire_name())
    }
}

impl<'de> Deserialize<'de> for WireErrorCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let name = BoundedText::<64>::deserialize(deserializer)?;
        match name.as_str() {
            "InvalidLane" => Ok(Self::InvalidLane),
            "FrameTooLarge" => Ok(Self::FrameTooLarge),
            "MalformedJson" => Ok(Self::MalformedJson),
            "JsonTooDeep" => Ok(Self::JsonTooDeep),
            "CollectionTooLarge" => Ok(Self::CollectionTooLarge),
            "JsonNodeLimitExceeded" => Ok(Self::JsonNodeLimitExceeded),
            "UnsupportedSchema" => Ok(Self::UnsupportedSchema),
            "WrongLane" => Ok(Self::WrongLane),
            "InvalidEnvelope" => Ok(Self::InvalidEnvelope),
            "AllocationFailed" => Ok(Self::AllocationFailed),
            "DuplicateJsonKey" => Ok(Self::DuplicateJsonKey),
            "InvalidRecord" => Ok(Self::InvalidRecord),
            _ => Err(de::Error::custom("unsupported v1 wire error code")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WireError {
    code: WireErrorCode,
    detail: Box<str>,
}

impl WireError {
    #[must_use]
    pub fn new(code: WireErrorCode, detail: impl AsRef<str>) -> Self {
        Self {
            code,
            detail: truncate_utf8(detail.as_ref(), MAX_ERROR_DETAIL_BYTES).into(),
        }
    }

    #[must_use]
    pub const fn code(&self) -> WireErrorCode {
        self.code
    }

    #[must_use]
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for WireError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "wire error {}: {}",
            self.code as u16, self.detail
        )
    }
}

impl Error for WireError {}

fn truncate_utf8(value: &str, max_bytes: usize) -> &str {
    if value.len() <= max_bytes {
        return value;
    }

    let mut end = max_bytes;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    &value[..end]
}

#[cfg(test)]
mod tests {
    use super::{WireError, WireErrorCode};

    #[test]
    fn error_detail_is_bounded_on_utf8_boundary() {
        let detail = "🦀".repeat(200);
        let error = WireError::new(WireErrorCode::MalformedJson, &detail);

        assert!(error.detail().len() <= 512);
        assert!(error.detail().is_char_boundary(error.detail().len()));
    }

    #[test]
    fn stable_error_codes_are_explicit() {
        assert_eq!(WireErrorCode::InvalidLane as u16, 1);
        assert_eq!(WireErrorCode::InvalidEnvelope as u16, 9);
        assert_eq!(WireErrorCode::AllocationFailed as u16, 10);
    }
}
