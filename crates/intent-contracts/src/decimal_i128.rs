use crate::BoundedText;
use serde::{Deserialize, Deserializer, Serializer, de};

pub(crate) fn serialize<S>(value: &i128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_str(value)
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<i128, D::Error>
where
    D: Deserializer<'de>,
{
    let text = BoundedText::<40>::deserialize(deserializer)?;
    let value = text.as_str().parse::<i128>().map_err(de::Error::custom)?;
    if value.to_string() != text.as_str() {
        return Err(de::Error::custom(
            "minor_units must be a canonical decimal integer",
        ));
    }
    Ok(value)
}
