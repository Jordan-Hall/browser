use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

pub const MAX_CURRENCY_SCALE: u8 = 18;
pub const MIN_UNIX_TIMESTAMP_MICROS: i64 = -62_135_596_800_000_000;
pub const MAX_UNIX_TIMESTAMP_MICROS: i64 = 253_402_300_799_999_999;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct BoundedText<const MAX_BYTES: usize>(String);

impl<const MAX_BYTES: usize> BoundedText<MAX_BYTES> {
    pub fn try_new(value: impl Into<String>) -> Result<Self, BoundedTextError> {
        let value = value.into();
        let actual_bytes = value.len();
        if actual_bytes > MAX_BYTES {
            return Err(BoundedTextError {
                actual_bytes,
                max_bytes: MAX_BYTES,
            });
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl<const MAX_BYTES: usize> fmt::Display for BoundedText<MAX_BYTES> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl<'de, const MAX_BYTES: usize> Deserialize<'de> for BoundedText<MAX_BYTES> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::try_new(value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundedTextError {
    actual_bytes: usize,
    max_bytes: usize,
}

impl fmt::Display for BoundedTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "text is {} bytes but maximum is {} bytes",
            self.actual_bytes, self.max_bytes
        )
    }
}

impl Error for BoundedTextError {}

macro_rules! bounded_text_newtype {
    ($name:ident, $max_bytes:expr) => {
        #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
        #[serde(transparent)]
        pub struct $name(BoundedText<$max_bytes>);

        impl $name {
            pub fn try_new(value: impl Into<String>) -> Result<Self, BoundedTextError> {
                BoundedText::try_new(value).map(Self)
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = BoundedTextError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::try_new(value)
            }
        }
    };
}

bounded_text_newtype!(ProviderId, 128);
bounded_text_newtype!(ProviderAccountId, 256);
bounded_text_newtype!(ProviderResourceId, 512);

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AccountQualifiedResourceId {
    provider: ProviderId,
    account: ProviderAccountId,
    resource: ProviderResourceId,
}

impl AccountQualifiedResourceId {
    #[must_use]
    pub const fn new(
        provider: ProviderId,
        account: ProviderAccountId,
        resource: ProviderResourceId,
    ) -> Self {
        Self {
            provider,
            account,
            resource,
        }
    }

    #[must_use]
    pub const fn provider(&self) -> &ProviderId {
        &self.provider
    }

    #[must_use]
    pub const fn account(&self) -> &ProviderAccountId {
        &self.account
    }

    #[must_use]
    pub const fn resource(&self) -> &ProviderResourceId {
        &self.resource
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CurrencyCode([u8; 3]);

impl CurrencyCode {
    pub fn parse(value: &str) -> Result<Self, CurrencyCodeError> {
        let bytes = value.as_bytes();
        if bytes.len() != 3 || !bytes.iter().all(u8::is_ascii_uppercase) {
            return Err(CurrencyCodeError);
        }
        Ok(Self([bytes[0], bytes[1], bytes[2]]))
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}{}{}",
            char::from(self.0[0]),
            char::from(self.0[1]),
            char::from(self.0[2])
        )
    }
}

impl FromStr for CurrencyCode {
    type Err = CurrencyCodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl Serialize for CurrencyCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for CurrencyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CurrencyCodeError;

impl fmt::Display for CurrencyCodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("currency code must contain exactly three uppercase ASCII letters")
    }
}

impl Error for CurrencyCodeError {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct KnownCurrencyScale(u8);

impl KnownCurrencyScale {
    pub const fn try_new(value: u8) -> Result<Self, CurrencyScaleError> {
        if value > MAX_CURRENCY_SCALE {
            return Err(CurrencyScaleError(value));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl<'de> Deserialize<'de> for KnownCurrencyScale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Self::try_new(value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyScale {
    Unknown,
    Known(KnownCurrencyScale),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CurrencyScaleError(u8);

impl fmt::Display for CurrencyScaleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "currency scale {} exceeds supported maximum {}",
            self.0, MAX_CURRENCY_SCALE
        )
    }
}

impl Error for CurrencyScaleError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Money {
    currency: CurrencyCode,
    minor_units: i128,
    scale: CurrencyScale,
}

impl Money {
    #[must_use]
    pub const fn new(currency: CurrencyCode, minor_units: i128, scale: CurrencyScale) -> Self {
        Self {
            currency,
            minor_units,
            scale,
        }
    }

    #[must_use]
    pub const fn currency(self) -> CurrencyCode {
        self.currency
    }

    #[must_use]
    pub const fn minor_units(self) -> i128 {
        self.minor_units
    }

    #[must_use]
    pub const fn scale(self) -> CurrencyScale {
        self.scale
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct UnixTimestampMicros(i64);

impl UnixTimestampMicros {
    pub const fn try_new(value: i64) -> Result<Self, TimestampError> {
        if value < MIN_UNIX_TIMESTAMP_MICROS || value > MAX_UNIX_TIMESTAMP_MICROS {
            return Err(TimestampError(value));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for UnixTimestampMicros {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;
        Self::try_new(value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampError(i64);

impl fmt::Display for TimestampError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Unix timestamp in microseconds is out of supported range: {}",
            self.0
        )
    }
}

impl Error for TimestampError {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ByteSize(u64);

impl ByteSize {
    #[must_use]
    pub const fn from_bytes(bytes: u64) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_bytes(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContentHash([u8; 32]);

impl ContentHash {
    pub fn from_hex(value: &str) -> Result<Self, ContentHashError> {
        if value.len() != 64 {
            return Err(ContentHashError);
        }

        let mut bytes = [0_u8; 32];
        for (index, pair) in value.as_bytes().as_chunks::<2>().0.iter().enumerate() {
            let high = decode_hex_nibble(pair[0]).ok_or(ContentHashError)?;
            let low = decode_hex_nibble(pair[1]).ok_or(ContentHashError)?;
            bytes[index] = (high << 4) | low;
        }
        Ok(Self(bytes))
    }

    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn into_bytes(self) -> [u8; 32] {
        self.0
    }

    #[must_use]
    pub fn to_hex(self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(64);
        for byte in self.0 {
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        output
    }
}

impl fmt::Display for ContentHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}

impl Serialize for ContentHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for ContentHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::from_hex(&value).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContentHashError;

impl fmt::Display for ContentHashError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("content hash must contain exactly 64 hexadecimal characters")
    }
}

impl Error for ContentHashError {}

const fn decode_hex_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoundedText, ByteSize, ContentHash, CurrencyCode, CurrencyScale, KnownCurrencyScale, Money,
        UnixTimestampMicros,
    };
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn bounded_text_rejects_oversized_deserialized_value() {
        let result = serde_json::from_str::<BoundedText<4>>("\"hello\"");
        assert!(result.is_err());
    }

    #[test]
    fn currency_code_rejects_noncanonical_input() {
        assert!(CurrencyCode::from_str("usd").is_err());
        assert!(CurrencyCode::from_str("USDT").is_err());
    }

    #[test]
    fn currency_scale_rejects_unbounded_precision() {
        assert!(KnownCurrencyScale::try_new(19).is_err());
    }

    #[test]
    fn money_round_trip_preserves_unknown_or_known_scale() -> Result<(), Box<dyn Error>> {
        let currency = CurrencyCode::from_str("USD")?;
        let scale = KnownCurrencyScale::try_new(2)?;
        let money = Money::new(currency, 12_345, CurrencyScale::Known(scale));
        let encoded = serde_json::to_string(&money)?;
        let decoded: Money = serde_json::from_str(&encoded)?;
        assert_eq!(decoded, money);
        Ok(())
    }

    #[test]
    fn timestamp_rejects_values_outside_supported_calendar_range() {
        let result = serde_json::from_str::<UnixTimestampMicros>("253402300800000000");
        assert!(result.is_err());
    }

    #[test]
    fn byte_size_rejects_negative_serialized_values() {
        let result = serde_json::from_str::<ByteSize>("-1");
        assert!(result.is_err());
    }

    #[test]
    fn content_hash_round_trip_is_canonical_lower_hex() -> Result<(), Box<dyn Error>> {
        let source = "0123456789abcdef0123456789ABCDEF0123456789abcdef0123456789ABCDEF";
        let hash = ContentHash::from_hex(source)?;
        let encoded = serde_json::to_string(&hash)?;
        assert_eq!(encoded, format!("\"{}\"", source.to_ascii_lowercase()));
        Ok(())
    }
}
