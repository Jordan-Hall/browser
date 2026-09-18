use serde::{Deserialize, Deserializer, Serialize, de};
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SchemaVersion {
    major: u16,
    minor: u16,
}

impl SchemaVersion {
    pub const V1: Self = Self { major: 1, minor: 0 };

    pub const fn try_new(major: u16, minor: u16) -> Result<Self, SchemaVersionError> {
        if major == 0 {
            return Err(SchemaVersionError);
        }
        Ok(Self { major, minor })
    }

    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }
}

impl<'de> Deserialize<'de> for SchemaVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RawVersion {
            major: u16,
            minor: u16,
        }

        let raw = RawVersion::deserialize(deserializer)?;
        Self::try_new(raw.major, raw.minor).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchemaVersionError;

impl fmt::Display for SchemaVersionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("schema major version must be greater than zero")
    }
}

impl Error for SchemaVersionError {}

pub(crate) fn deserialize_v1_schema<'de, D>(deserializer: D) -> Result<SchemaVersion, D::Error>
where
    D: Deserializer<'de>,
{
    let version = SchemaVersion::deserialize(deserializer)?;
    if version != SchemaVersion::V1 {
        return Err(de::Error::custom(format_args!(
            "record requires schema version 1.0, got {}.{}",
            version.major(),
            version.minor()
        )));
    }
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::SchemaVersion;

    #[test]
    fn rejects_zero_major_version_from_wire_data() {
        let result = serde_json::from_str::<SchemaVersion>(r#"{"major":0,"minor":1}"#);
        assert!(result.is_err());
    }
}
