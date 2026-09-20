use super::RecordValidationError;
use serde::{Deserialize, Deserializer, Serialize, de};
use std::{fmt, marker::PhantomData};

/// Maximum entries in each collection field of a v1 durable record.
pub const MAX_RECORD_COLLECTION_ENTRIES: usize = 16_384;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub(super) struct RecordList<T>(Vec<T>);

impl<T> RecordList<T> {
    pub(super) const fn new() -> Self {
        Self(Vec::new())
    }

    pub(super) fn as_slice(&self) -> &[T] {
        &self.0
    }

    pub(super) fn try_push(&mut self, value: T) -> Result<(), RecordValidationError> {
        if self.0.len() >= MAX_RECORD_COLLECTION_ENTRIES {
            return Err(RecordValidationError::CollectionTooLarge);
        }
        self.0.push(value);
        Ok(())
    }
}

impl<T> Default for RecordList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for RecordList<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ListVisitor<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> de::Visitor<'de> for ListVisitor<T> {
            type Value = RecordList<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "at most {MAX_RECORD_COLLECTION_ENTRIES} record items"
                )
            }

            fn visit_seq<A: de::SeqAccess<'de>>(
                self,
                mut sequence: A,
            ) -> Result<Self::Value, A::Error> {
                let mut items = Vec::new();
                while items.len() < MAX_RECORD_COLLECTION_ENTRIES {
                    let Some(item) = sequence.next_element()? else {
                        return Ok(RecordList(items));
                    };
                    items
                        .try_reserve(1)
                        .map_err(|_| de::Error::custom("record collection allocation failed"))?;
                    items.push(item);
                }
                if sequence.next_element::<de::IgnoredAny>()?.is_some() {
                    return Err(de::Error::custom(RecordValidationError::CollectionTooLarge));
                }
                Ok(RecordList(items))
            }
        }

        deserializer.deserialize_seq(ListVisitor(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn infinite_sequence_without_size_hint_stops_after_the_first_excess_item() {
        let consumed = Cell::new(0_usize);
        let items = std::iter::from_fn(|| {
            consumed.set(consumed.get() + 1);
            Some(1_u8)
        });
        let deserializer = de::value::SeqDeserializer::<_, de::value::Error>::new(items);
        assert!(RecordList::<u8>::deserialize(deserializer).is_err());
        assert_eq!(consumed.get(), MAX_RECORD_COLLECTION_ENTRIES + 1);
    }
}
