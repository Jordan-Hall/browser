use crate::{WireError, WireErrorCode, WireLimits};
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};
use std::fmt;

struct Budget {
    limits: WireLimits,
    nodes: usize,
    failure: Option<WireErrorCode>,
}

impl Budget {
    fn reject<E: de::Error>(&mut self, code: WireErrorCode, detail: &'static str) -> E {
        self.failure = Some(code);
        E::custom(detail)
    }
}

struct Node<'a> {
    budget: &'a mut Budget,
    depth: usize,
}

impl<'de> DeserializeSeed<'de> for Node<'_> {
    type Value = Value;

    fn deserialize<D: de::Deserializer<'de>>(self, decoder: D) -> Result<Value, D::Error> {
        if self.depth > self.budget.limits.max_json_depth.max(1) {
            return Err(self
                .budget
                .reject(WireErrorCode::JsonTooDeep, "JSON depth limit"));
        }
        if self.budget.nodes >= self.budget.limits.max_json_nodes {
            return Err(self
                .budget
                .reject(WireErrorCode::JsonNodeLimitExceeded, "JSON node limit"));
        }
        self.budget.nodes += 1;
        decoder.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for Node<'_> {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("bounded JSON without duplicate object keys")
    }

    fn visit_bool<E: de::Error>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Value, E> {
        Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }

    fn visit_unit<E: de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Value, A::Error> {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(Node {
            budget: self.budget,
            depth: self.depth + 1,
        })? {
            if values.len() >= self.budget.limits.max_collection_entries {
                return Err(self
                    .budget
                    .reject(WireErrorCode::CollectionTooLarge, "JSON array limit"));
            }
            values.try_reserve(1).map_err(|_| {
                self.budget.reject(
                    WireErrorCode::AllocationFailed,
                    "JSON array allocation failed",
                )
            })?;
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut object: A) -> Result<Value, A::Error> {
        let mut values = Map::new();
        while let Some(key) = object.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(self
                    .budget
                    .reject(WireErrorCode::DuplicateJsonKey, "duplicate JSON object key"));
            }
            if values.len() >= self.budget.limits.max_collection_entries {
                return Err(self
                    .budget
                    .reject(WireErrorCode::CollectionTooLarge, "JSON object limit"));
            }
            let value = object.next_value_seed(Node {
                budget: self.budget,
                depth: self.depth + 1,
            })?;
            values.insert(key, value);
        }
        Ok(Value::Object(values))
    }
}

pub(crate) fn decode(input: &[u8], limits: WireLimits) -> Result<Value, WireError> {
    let mut budget = Budget {
        limits,
        nodes: 0,
        failure: None,
    };
    let mut decoder = serde_json::Deserializer::from_slice(input);
    let result = Node {
        budget: &mut budget,
        depth: 1,
    }
    .deserialize(&mut decoder);
    let value = result.map_err(|error| {
        WireError::new(
            budget.failure.unwrap_or(WireErrorCode::MalformedJson),
            error.to_string(),
        )
    })?;
    decoder
        .end()
        .map_err(|error| WireError::new(WireErrorCode::MalformedJson, error.to_string()))?;
    Ok(value)
}
