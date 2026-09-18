use intent_contracts::{BoundedText, BoundedTextError, SchemaVersion};
use serde::{Deserialize, Deserializer, Serialize, de};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

pub const MAX_PROTOCOL_RANGES: usize = 16;
pub const MAX_PROTOCOL_CAPABILITIES: usize = 64;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ProtocolCapability(BoundedText<64>);

impl ProtocolCapability {
    pub fn try_new(value: impl Into<String>) -> Result<Self, BoundedTextError> {
        BoundedText::try_new(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct ProtocolRange {
    major: u16,
    min_minor: u16,
    max_minor: u16,
}

impl ProtocolRange {
    pub const fn try_new(
        major: u16,
        min_minor: u16,
        max_minor: u16,
    ) -> Result<Self, ProtocolOfferError> {
        if major == 0 {
            return Err(ProtocolOfferError::ZeroMajor);
        }
        if min_minor > max_minor {
            return Err(ProtocolOfferError::InvalidRange {
                min_minor,
                max_minor,
            });
        }
        Ok(Self {
            major,
            min_minor,
            max_minor,
        })
    }

    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    #[must_use]
    pub const fn min_minor(self) -> u16 {
        self.min_minor
    }

    #[must_use]
    pub const fn max_minor(self) -> u16 {
        self.max_minor
    }
}

impl<'de> Deserialize<'de> for ProtocolRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawRange {
            major: u16,
            min_minor: u16,
            max_minor: u16,
        }

        let raw = RawRange::deserialize(deserializer)?;
        Self::try_new(raw.major, raw.min_minor, raw.max_minor).map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProtocolOffer {
    ranges: Vec<ProtocolRange>,
    capabilities: BTreeSet<ProtocolCapability>,
}

impl ProtocolOffer {
    pub fn try_new(
        ranges: Vec<ProtocolRange>,
        capabilities: impl IntoIterator<Item = ProtocolCapability>,
    ) -> Result<Self, ProtocolOfferError> {
        if ranges.is_empty() {
            return Err(ProtocolOfferError::NoRanges);
        }
        if ranges.len() > MAX_PROTOCOL_RANGES {
            return Err(ProtocolOfferError::TooManyRanges(ranges.len()));
        }
        let mut bounded_capabilities = BTreeSet::new();
        for (index, capability) in capabilities
            .into_iter()
            .take(MAX_PROTOCOL_CAPABILITIES + 1)
            .enumerate()
        {
            if index == MAX_PROTOCOL_CAPABILITIES {
                return Err(ProtocolOfferError::TooManyCapabilities(index + 1));
            }
            bounded_capabilities.insert(capability);
        }
        let capabilities = bounded_capabilities;
        Ok(Self {
            ranges,
            capabilities,
        })
    }

    #[must_use]
    pub fn ranges(&self) -> &[ProtocolRange] {
        &self.ranges
    }

    #[must_use]
    pub const fn capabilities(&self) -> &BTreeSet<ProtocolCapability> {
        &self.capabilities
    }
}

impl<'de> Deserialize<'de> for ProtocolOffer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RawOffer {
            #[serde(deserialize_with = "deserialize_ranges")]
            ranges: Vec<ProtocolRange>,
            #[serde(default, deserialize_with = "deserialize_capabilities")]
            capabilities: Vec<ProtocolCapability>,
        }

        let raw = RawOffer::deserialize(deserializer)?;
        Self::try_new(raw.ranges, raw.capabilities).map_err(de::Error::custom)
    }
}

fn deserialize_ranges<'de, D>(deserializer: D) -> Result<Vec<ProtocolRange>, D::Error>
where
    D: Deserializer<'de>,
{
    crate::bounded_sequence::deserialize::<D, ProtocolRange, MAX_PROTOCOL_RANGES>(deserializer)
}

fn deserialize_capabilities<'de, D>(deserializer: D) -> Result<Vec<ProtocolCapability>, D::Error>
where
    D: Deserializer<'de>,
{
    crate::bounded_sequence::deserialize::<D, ProtocolCapability, MAX_PROTOCOL_CAPABILITIES>(
        deserializer,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NegotiatedProtocol {
    version: SchemaVersion,
    capabilities: BTreeSet<ProtocolCapability>,
}

impl NegotiatedProtocol {
    #[must_use]
    pub const fn version(&self) -> SchemaVersion {
        self.version
    }

    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<ProtocolCapability> {
        &self.capabilities
    }

    #[must_use]
    pub fn supports(&self, capability: &ProtocolCapability) -> bool {
        self.capabilities.contains(capability)
    }

    pub fn validate_message_version(
        &self,
        message_version: SchemaVersion,
    ) -> Result<(), NegotiationError> {
        if message_version != self.version {
            return Err(NegotiationError::VersionMismatch {
                negotiated: self.version,
                received: message_version,
            });
        }
        Ok(())
    }
}

pub fn negotiate_protocol(
    local: &ProtocolOffer,
    remote: &ProtocolOffer,
) -> Result<NegotiatedProtocol, NegotiationError> {
    let mut selected: Option<SchemaVersion> = None;
    for local_range in &local.ranges {
        for remote_range in &remote.ranges {
            if local_range.major != remote_range.major {
                continue;
            }
            let lower = local_range.min_minor.max(remote_range.min_minor);
            let upper = local_range.max_minor.min(remote_range.max_minor);
            if lower > upper {
                continue;
            }
            let candidate = SchemaVersion::try_new(local_range.major, upper)
                .map_err(|_| NegotiationError::InvalidOffer)?;
            if selected.is_none_or(|current| candidate > current) {
                selected = Some(candidate);
            }
        }
    }

    let version = selected.ok_or(NegotiationError::NoCompatibleVersion)?;
    let capabilities = local
        .capabilities
        .intersection(&remote.capabilities)
        .cloned()
        .collect();

    Ok(NegotiatedProtocol {
        version,
        capabilities,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolChangeClass {
    AdditiveOptional,
    AuthorityAffecting,
}

pub fn validate_version_change(
    previous: SchemaVersion,
    next: SchemaVersion,
    class: ProtocolChangeClass,
) -> Result<(), VersionChangeError> {
    if next <= previous {
        return Err(VersionChangeError::NonForward { previous, next });
    }
    if class == ProtocolChangeClass::AuthorityAffecting && next.major() == previous.major() {
        return Err(VersionChangeError::AuthorityChangeRequiresMajor { previous, next });
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolOfferError {
    ZeroMajor,
    InvalidRange { min_minor: u16, max_minor: u16 },
    NoRanges,
    TooManyRanges(usize),
    TooManyCapabilities(usize),
}

impl fmt::Display for ProtocolOfferError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroMajor => formatter.write_str("protocol major version must be non-zero"),
            Self::InvalidRange {
                min_minor,
                max_minor,
            } => write!(
                formatter,
                "protocol minor range is invalid: {min_minor}..={max_minor}"
            ),
            Self::NoRanges => formatter.write_str("protocol offer must contain a version range"),
            Self::TooManyRanges(count) => write!(
                formatter,
                "protocol offer has {count} ranges; maximum is {MAX_PROTOCOL_RANGES}"
            ),
            Self::TooManyCapabilities(count) => write!(
                formatter,
                "protocol offer has {count} capabilities; maximum is {MAX_PROTOCOL_CAPABILITIES}"
            ),
        }
    }
}

impl Error for ProtocolOfferError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NegotiationError {
    InvalidOffer,
    NoCompatibleVersion,
    VersionMismatch {
        negotiated: SchemaVersion,
        received: SchemaVersion,
    },
}

impl fmt::Display for NegotiationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOffer => {
                formatter.write_str("protocol offer contained an invalid version")
            }
            Self::NoCompatibleVersion => {
                formatter.write_str("local and remote peers have no compatible protocol version")
            }
            Self::VersionMismatch {
                negotiated,
                received,
            } => write!(
                formatter,
                "message version {}.{} does not match negotiated version {}.{}",
                received.major(),
                received.minor(),
                negotiated.major(),
                negotiated.minor()
            ),
        }
    }
}

impl Error for NegotiationError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VersionChangeError {
    NonForward {
        previous: SchemaVersion,
        next: SchemaVersion,
    },
    AuthorityChangeRequiresMajor {
        previous: SchemaVersion,
        next: SchemaVersion,
    },
}

impl fmt::Display for VersionChangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonForward { previous, next } => write!(
                formatter,
                "protocol change must move forward: {}.{} -> {}.{}",
                previous.major(),
                previous.minor(),
                next.major(),
                next.minor()
            ),
            Self::AuthorityChangeRequiresMajor { previous, next } => write!(
                formatter,
                "authority-affecting change requires a major version bump: {}.{} -> {}.{}",
                previous.major(),
                previous.minor(),
                next.major(),
                next.minor()
            ),
        }
    }
}

impl Error for VersionChangeError {}

#[cfg(test)]
mod tests {
    use super::{
        MAX_PROTOCOL_RANGES, NegotiationError, ProtocolCapability, ProtocolChangeClass,
        ProtocolOffer, ProtocolRange, negotiate_protocol, validate_version_change,
    };
    use intent_contracts::SchemaVersion;
    use std::error::Error;

    fn capability(name: &str) -> Result<ProtocolCapability, Box<dyn Error>> {
        Ok(ProtocolCapability::try_new(name)?)
    }

    #[test]
    fn negotiation_selects_highest_common_version_and_capability_intersection()
    -> Result<(), Box<dyn Error>> {
        let local = ProtocolOffer::try_new(
            vec![
                ProtocolRange::try_new(1, 0, 3)?,
                ProtocolRange::try_new(2, 0, 1)?,
            ],
            [capability("artifact_lane")?, capability("cancellation")?],
        )?;
        let remote = ProtocolOffer::try_new(
            vec![
                ProtocolRange::try_new(1, 1, 2)?,
                ProtocolRange::try_new(2, 0, 0)?,
            ],
            [
                capability("cancellation")?,
                capability("remote_diagnostics")?,
            ],
        )?;

        let negotiated = negotiate_protocol(&local, &remote)?;
        assert_eq!(negotiated.version(), SchemaVersion::try_new(2, 0)?);
        assert!(negotiated.supports(&capability("cancellation")?));
        assert!(!negotiated.supports(&capability("artifact_lane")?));
        Ok(())
    }

    #[test]
    fn offer_round_trip_preserves_ranges_and_capabilities() -> Result<(), Box<dyn Error>> {
        let offer = ProtocolOffer::try_new(
            vec![ProtocolRange::try_new(1, 0, 2)?],
            [capability("artifact_lane")?, capability("cancellation")?],
        )?;
        let bytes = serde_json::to_vec(&offer)?;
        let decoded: ProtocolOffer = serde_json::from_slice(&bytes)?;
        assert_eq!(decoded, offer);
        Ok(())
    }

    #[test]
    fn negotiated_session_rejects_unselected_message_version() -> Result<(), Box<dyn Error>> {
        let local = ProtocolOffer::try_new(
            vec![ProtocolRange::try_new(1, 0, 3)?],
            std::iter::empty::<ProtocolCapability>(),
        )?;
        let remote = ProtocolOffer::try_new(
            vec![ProtocolRange::try_new(1, 1, 2)?],
            std::iter::empty::<ProtocolCapability>(),
        )?;
        let negotiated = negotiate_protocol(&local, &remote)?;
        assert_eq!(negotiated.version(), SchemaVersion::try_new(1, 2)?);
        assert_eq!(
            negotiated.validate_message_version(SchemaVersion::try_new(1, 1)?),
            Err(NegotiationError::VersionMismatch {
                negotiated: SchemaVersion::try_new(1, 2)?,
                received: SchemaVersion::try_new(1, 1)?,
            })
        );
        Ok(())
    }

    #[test]
    fn unsupported_major_fails_early() -> Result<(), Box<dyn Error>> {
        let local = ProtocolOffer::try_new(
            vec![ProtocolRange::try_new(1, 0, 3)?],
            std::iter::empty::<ProtocolCapability>(),
        )?;
        let remote = ProtocolOffer::try_new(
            vec![ProtocolRange::try_new(2, 0, 1)?],
            std::iter::empty::<ProtocolCapability>(),
        )?;
        assert_eq!(
            negotiate_protocol(&local, &remote),
            Err(NegotiationError::NoCompatibleVersion)
        );
        Ok(())
    }

    #[test]
    fn deserialization_enforces_offer_bounds() {
        let json = format!(
            "{{\"ranges\":[{}],\"capabilities\":[]}}",
            (0..=MAX_PROTOCOL_RANGES)
                .map(|_| "{\"major\":1,\"min_minor\":0,\"max_minor\":0}")
                .collect::<Vec<_>>()
                .join(",")
        );
        let result = serde_json::from_str::<ProtocolOffer>(&json);
        assert!(result.is_err());
    }

    #[test]
    fn authority_change_requires_major_bump() -> Result<(), Box<dyn Error>> {
        let v1 = SchemaVersion::try_new(1, 0)?;
        let v1_1 = SchemaVersion::try_new(1, 1)?;
        let v2 = SchemaVersion::try_new(2, 0)?;
        assert!(
            validate_version_change(v1, v1_1, ProtocolChangeClass::AuthorityAffecting).is_err()
        );
        validate_version_change(v1, v2, ProtocolChangeClass::AuthorityAffecting)?;
        validate_version_change(v1, v1_1, ProtocolChangeClass::AdditiveOptional)?;
        Ok(())
    }
}
