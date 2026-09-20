use crate::{CoreRecord, CoreRecordKind, WireError, WireErrorCode, WireLimits, decode_core_record};
use intent_contracts::SchemaVersion;
use serde::Deserialize;
use serde_json::value::RawValue;

/// Result of an explicit conversion from the pre-freeze numeric-money GoalContract shape.
///
/// The legacy bytes are never auto-detected as current schema. Callers must opt into this
/// conversion path, and the returned bytes are re-encoded by the current validated codec.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyNumericMoneyImport {
    record: CoreRecord,
    canonical_bytes: Vec<u8>,
}

impl LegacyNumericMoneyImport {
    #[must_use]
    pub const fn source_version(&self) -> SchemaVersion {
        SchemaVersion::V1
    }

    #[must_use]
    pub const fn target_version(&self) -> SchemaVersion {
        SchemaVersion::V1
    }

    #[must_use]
    pub fn record(&self) -> &CoreRecord {
        &self.record
    }

    #[must_use]
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }

    #[must_use]
    pub fn into_parts(self) -> (CoreRecord, Vec<u8>) {
        (self.record, self.canonical_bytes)
    }
}

#[derive(Deserialize)]
struct LegacyGoalProbe<'a> {
    #[serde(borrow)]
    budget: Option<LegacyMoneyProbe<'a>>,
}

#[derive(Deserialize)]
struct LegacyMoneyProbe<'a> {
    #[serde(borrow)]
    minor_units: &'a RawValue,
}

/// Convert the explicitly declared pre-freeze GoalContract numeric-money shape into the
/// current canonical decimal-string representation.
///
/// Historical records used schema 1.0 before the wire shape was frozen, so this is a
/// format-specific compatibility adapter rather than an ordinary `MigrationRegistry` step.
/// It accepts only a GoalContract containing a canonical JSON integer `budget.minor_units`;
/// current decimal strings, fractions, exponents, non-canonical integers and other record
/// families are rejected. The full input is syntax-budgeted before conversion and the
/// converted document is decoded and re-encoded by the current GoalContract codec.
pub fn migrate_legacy_numeric_money_goal_v1(
    input: &[u8],
    limits: WireLimits,
) -> Result<LegacyNumericMoneyImport, WireError> {
    let version = crate::document_syntax::schema_version(input, limits)?;
    if version != SchemaVersion::V1 {
        return Err(WireError::new(
            WireErrorCode::UnsupportedSchema,
            "legacy numeric-money adapter accepts schema 1.0 only",
        ));
    }

    let text = std::str::from_utf8(input)
        .map_err(|_| WireError::new(WireErrorCode::MalformedJson, "document is not UTF-8"))?;
    let probe: LegacyGoalProbe<'_> = serde_json::from_str(text).map_err(|_| {
        WireError::new(
            WireErrorCode::InvalidRecord,
            "legacy numeric-money GoalContract is malformed",
        )
    })?;
    let money = probe.budget.ok_or_else(|| {
        WireError::new(
            WireErrorCode::InvalidRecord,
            "legacy numeric-money GoalContract must contain a budget",
        )
    })?;
    let raw = money.minor_units.get();
    let value = parse_legacy_integer(raw)?;

    let input_start = text.as_ptr() as usize;
    let raw_start = raw.as_ptr() as usize;
    let start = raw_start.checked_sub(input_start).ok_or_else(|| {
        WireError::new(
            WireErrorCode::InvalidRecord,
            "legacy numeric-money field is not borrowed from the input",
        )
    })?;
    let end = start.checked_add(raw.len()).ok_or_else(|| {
        WireError::new(
            WireErrorCode::InvalidRecord,
            "legacy numeric-money field range overflowed",
        )
    })?;
    if end > input.len() {
        return Err(WireError::new(
            WireErrorCode::InvalidRecord,
            "legacy numeric-money field is outside the input",
        ));
    }

    let canonical_integer = value.to_string();
    let replacement_len = canonical_integer.len().checked_add(2).ok_or_else(|| {
        WireError::new(
            WireErrorCode::AllocationFailed,
            "legacy numeric-money replacement size overflowed",
        )
    })?;
    let converted_len = input
        .len()
        .checked_sub(raw.len())
        .and_then(|len| len.checked_add(replacement_len))
        .ok_or_else(|| {
            WireError::new(
                WireErrorCode::AllocationFailed,
                "legacy numeric-money document size overflowed",
            )
        })?;
    if converted_len > limits.max_control_frame_bytes {
        return Err(WireError::new(
            WireErrorCode::FrameTooLarge,
            "converted legacy record exceeds the control-document byte limit",
        ));
    }

    let mut converted = Vec::new();
    converted.try_reserve_exact(converted_len).map_err(|_| {
        WireError::new(
            WireErrorCode::AllocationFailed,
            "legacy numeric-money conversion allocation failed",
        )
    })?;
    converted.extend_from_slice(&input[..start]);
    converted.push(b'"');
    converted.extend_from_slice(canonical_integer.as_bytes());
    converted.push(b'"');
    converted.extend_from_slice(&input[end..]);

    let record = decode_core_record(CoreRecordKind::GoalContract, &converted, limits)?;
    let canonical_bytes = record.encode(limits)?;
    Ok(LegacyNumericMoneyImport {
        record,
        canonical_bytes,
    })
}

fn parse_legacy_integer(raw: &str) -> Result<i128, WireError> {
    let digits = raw.strip_prefix('-').unwrap_or(raw);
    if digits.is_empty()
        || !digits.bytes().all(|byte| byte.is_ascii_digit())
        || (digits.len() > 1 && digits.starts_with('0'))
    {
        return Err(WireError::new(
            WireErrorCode::InvalidRecord,
            "legacy minor_units must be a canonical JSON integer",
        ));
    }
    let value = raw.parse::<i128>().map_err(|_| {
        WireError::new(
            WireErrorCode::InvalidRecord,
            "legacy minor_units is outside the i128 domain",
        )
    })?;
    if value.to_string() != raw {
        return Err(WireError::new(
            WireErrorCode::InvalidRecord,
            "legacy minor_units must be a canonical JSON integer",
        ));
    }
    Ok(value)
}
