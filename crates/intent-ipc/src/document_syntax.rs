//! Syntax-only document validation. Unknown numbers are never coerced to f64.
use crate::{WireError, WireErrorCode, WireLimits};
use intent_contracts::SchemaVersion;
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::value::RawValue;
use std::{collections::HashSet, fmt};

// Borrowed subtree scans are iterative and bounded in depth as well as bytes.
// Do not let a caller-selected larger budget turn repeated scans into unbounded work.
const MAX_SYNTAX_DEPTH: usize = 128;

struct Budget<'de> {
    limits: WireLimits,
    nodes: usize,
    pending: Vec<(&'de RawValue, usize)>,
    header: Option<&'de RawValue>,
    failure: Option<WireErrorCode>,
}

impl<'de> Budget<'de> {
    fn reject<E: de::Error>(&mut self, code: WireErrorCode, detail: &'static str) -> E {
        self.failure = Some(code);
        E::custom(detail)
    }

    fn enqueue<E: de::Error>(&mut self, value: &'de RawValue, depth: usize) -> Result<(), E> {
        if depth > self.limits.max_json_depth.clamp(1, MAX_SYNTAX_DEPTH) {
            return Err(self.reject(WireErrorCode::JsonTooDeep, "JSON depth limit"));
        }
        if self.nodes >= self.limits.max_json_nodes {
            return Err(self.reject(WireErrorCode::JsonNodeLimitExceeded, "JSON node limit"));
        }
        self.pending.try_reserve(1).map_err(|_| {
            self.reject(
                WireErrorCode::AllocationFailed,
                "JSON traversal allocation failed",
            )
        })?;
        self.pending.push((value, depth));
        self.nodes += 1;
        Ok(())
    }
}

struct Container<'a, 'de> {
    budget: &'a mut Budget<'de>,
    depth: usize,
}

impl<'de> Visitor<'de> for Container<'_, 'de> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a bounded JSON container without duplicate keys")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<(), A::Error> {
        let mut count = 0;
        while let Some(value) = sequence.next_element::<&'de RawValue>()? {
            if count >= self.budget.limits.max_collection_entries {
                return Err(self
                    .budget
                    .reject(WireErrorCode::CollectionTooLarge, "JSON array limit"));
            }
            self.budget.enqueue(value, self.depth + 1)?;
            count += 1;
        }
        Ok(())
    }

    fn visit_map<A: MapAccess<'de>>(self, mut object: A) -> Result<(), A::Error> {
        let mut keys = HashSet::new();
        while let Some(key) = object.next_key::<String>()? {
            if keys.contains(&key) {
                return Err(self
                    .budget
                    .reject(WireErrorCode::DuplicateJsonKey, "duplicate JSON object key"));
            }
            if keys.len() >= self.budget.limits.max_collection_entries {
                return Err(self
                    .budget
                    .reject(WireErrorCode::CollectionTooLarge, "JSON object limit"));
            }
            keys.try_reserve(1).map_err(|_| {
                self.budget.reject(
                    WireErrorCode::AllocationFailed,
                    "JSON key allocation failed",
                )
            })?;
            let value = object.next_value::<&'de RawValue>()?;
            self.budget.enqueue(value, self.depth + 1)?;
            if self.depth == 1 && key == "schema_version" {
                self.budget.header = Some(value);
            }
            keys.insert(key);
        }
        Ok(())
    }
}

pub(crate) fn schema_version(input: &[u8], limits: WireLimits) -> Result<SchemaVersion, WireError> {
    if input.len() > limits.max_control_frame_bytes {
        return Err(WireError::new(
            WireErrorCode::FrameTooLarge,
            "record exceeds the control-document byte limit",
        ));
    }
    crate::envelope::preflight_json_structure(input, limits.max_json_depth.min(MAX_SYNTAX_DEPTH))?;
    let text = std::str::from_utf8(input)
        .map_err(|_| WireError::new(WireErrorCode::MalformedJson, "document is not UTF-8"))?;
    let root: &RawValue = serde_json::from_str(text).map_err(|_| {
        WireError::new(WireErrorCode::MalformedJson, "invalid document JSON syntax")
    })?;
    let mut budget = Budget {
        limits,
        nodes: 0,
        pending: Vec::new(),
        header: None,
        failure: None,
    };
    let result: Result<(), serde_json::Error> = (|| {
        budget.enqueue(root, 1)?;
        while let Some((raw, depth)) = budget.pending.pop() {
            let mut decoder = serde_json::Deserializer::from_str(raw.get());
            match raw.get().as_bytes().first() {
                Some(b'{') => decoder.deserialize_map(Container {
                    budget: &mut budget,
                    depth,
                })?,
                Some(b'[') => decoder.deserialize_seq(Container {
                    budget: &mut budget,
                    depth,
                })?,
                Some(b'"') => {
                    // Raw syntax alone does not reject unpaired surrogate escapes.
                    // Decode strings, but deliberately never decode numeric leaves.
                    let _: String = serde_json::from_str(raw.get())?;
                }
                _ => {}
            }
        }
        Ok(())
    })();
    result.map_err(|_| {
        WireError::new(
            budget.failure.unwrap_or(WireErrorCode::MalformedJson),
            "document does not satisfy its JSON syntax budgets",
        )
    })?;
    let header = budget.header.ok_or_else(|| {
        WireError::new(
            WireErrorCode::InvalidRecord,
            "record schema_version is required",
        )
    })?;
    serde_json::from_str(header.get()).map_err(|_| {
        WireError::new(
            WireErrorCode::InvalidRecord,
            "invalid record schema_version",
        )
    })
}
