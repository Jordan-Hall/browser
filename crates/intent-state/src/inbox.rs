use crate::StateStore;
use intent_contracts::{BoundedText, ContentHash, UnixTimestampMicros};
use rusqlite::{OptionalExtension, TransactionBehavior, params};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;

pub const MAX_INBOX_PAYLOAD_BYTES: usize = 1024 * 1024;
pub const MAX_CONSUMER_EFFECTS: usize = 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InboxEvent {
    pub source: BoundedText<128>,
    pub event_id: BoundedText<256>,
    pub stream: BoundedText<128>,
    pub sequence: u64,
    pub payload: Vec<u8>,
    pub received_at: UnixTimestampMicros,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerEffect {
    pub key: BoundedText<256>,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InboxApplyResult {
    Applied { sequence: u64, effect_count: usize },
    Duplicate { sequence: u64 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredConsumerEffect {
    key: BoundedText<256>,
    payload: Vec<u8>,
    payload_hash: ContentHash,
}

impl StoredConsumerEffect {
    #[must_use]
    pub fn key(&self) -> &BoundedText<256> {
        &self.key
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    #[must_use]
    pub const fn payload_hash(&self) -> ContentHash {
        self.payload_hash
    }
}

impl StateStore {
    pub fn apply_inbox_event(
        &mut self,
        consumer: &BoundedText<128>,
        event: InboxEvent,
        mut effects: Vec<ConsumerEffect>,
        processed_at: UnixTimestampMicros,
    ) -> Result<InboxApplyResult, InboxError> {
        validate_payload(&event.payload, "inbox payload")?;
        if effects.len() > MAX_CONSUMER_EFFECTS {
            return Err(InboxError::TooManyEffects(effects.len()));
        }
        let mut aggregate_payload_bytes = event.payload.len();
        for effect in &effects {
            validate_payload(&effect.payload, "consumer effect")?;
            aggregate_payload_bytes = aggregate_payload_bytes
                .checked_add(effect.payload.len())
                .ok_or(InboxError::CounterOverflow("aggregate inbox payload bytes"))?;
            if aggregate_payload_bytes > MAX_INBOX_PAYLOAD_BYTES {
                return Err(InboxError::PayloadTooLarge {
                    label: "aggregate inbox and consumer-effect payloads",
                    size: aggregate_payload_bytes,
                });
            }
        }
        effects.sort_by(|left, right| left.key.cmp(&right.key));
        for pair in effects.windows(2) {
            if pair[0].key == pair[1].key {
                return Err(InboxError::DuplicateEffectKey(
                    pair[0].key.as_str().to_owned(),
                ));
            }
        }

        let payload_hash = hash_bytes(&event.payload);
        let effects_hash = hash_effects(&effects);
        let sequence = to_sql_i64(event.sequence, "inbox sequence")?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;

        let existing_event: Option<(String, i64, String)> = transaction
            .query_row(
                r#"
                SELECT stream, sequence, payload_hash
                FROM inbox_events
                WHERE source = ?1 AND event_id = ?2
                "#,
                params![event.source.as_str(), event.event_id.as_str()],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .optional()?;
        match existing_event {
            Some((stream, stored_sequence, stored_hash)) => {
                if stream != event.stream.as_str()
                    || stored_sequence != sequence
                    || stored_hash != payload_hash.to_hex()
                {
                    return Err(InboxError::EventIdentityCollision {
                        source: event.source.as_str().to_owned(),
                        event_id: event.event_id.as_str().to_owned(),
                    });
                }
                validate_stored_event(&transaction, &event.source, &event.event_id, payload_hash)?;
            }
            None => {
                let stream_collision: Option<String> = transaction
                    .query_row(
                        r#"
                        SELECT event_id FROM inbox_events
                        WHERE source = ?1 AND stream = ?2 AND sequence = ?3
                        "#,
                        params![event.source.as_str(), event.stream.as_str(), sequence],
                        |row| row.get(0),
                    )
                    .optional()?;
                if let Some(existing_event_id) = stream_collision {
                    return Err(InboxError::SequenceCollision {
                        source: event.source.as_str().to_owned(),
                        stream: event.stream.as_str().to_owned(),
                        sequence: event.sequence,
                        existing_event_id,
                    });
                }
                transaction.execute(
                    r#"
                    INSERT INTO inbox_events(
                        source, event_id, stream, sequence, payload, payload_hash, received_at_micros
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                    "#,
                    params![
                        event.source.as_str(),
                        event.event_id.as_str(),
                        event.stream.as_str(),
                        sequence,
                        &event.payload,
                        payload_hash.to_hex(),
                        event.received_at.get(),
                    ],
                )?;
            }
        }

        let already_processed: Option<(i64, String)> = transaction
            .query_row(
                r#"
                SELECT sequence, effects_hash FROM consumer_events
                WHERE consumer = ?1 AND source = ?2 AND event_id = ?3
                "#,
                params![
                    consumer.as_str(),
                    event.source.as_str(),
                    event.event_id.as_str()
                ],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        if let Some((stored_sequence, stored_effects_hash)) = already_processed {
            if stored_sequence != sequence || stored_effects_hash != effects_hash.to_hex() {
                return Err(InboxError::DuplicateDeliveryChanged {
                    consumer: consumer.as_str().to_owned(),
                    source: event.source.as_str().to_owned(),
                    event_id: event.event_id.as_str().to_owned(),
                });
            }
            load_consumer_effects(&transaction, consumer, &event.source, &event.event_id)?;
            transaction.commit()?;
            return Ok(InboxApplyResult::Duplicate {
                sequence: event.sequence,
            });
        }

        let cursor: Option<i64> = transaction
            .query_row(
                r#"
                SELECT last_sequence FROM consumer_cursors
                WHERE consumer = ?1 AND source = ?2 AND stream = ?3
                "#,
                params![
                    consumer.as_str(),
                    event.source.as_str(),
                    event.stream.as_str()
                ],
                |row| row.get(0),
            )
            .optional()?;
        if let Some(last_sequence) = cursor {
            let last_sequence = nonnegative_u64(last_sequence, "consumer cursor")?;
            if event.sequence <= last_sequence {
                return Err(InboxError::StaleSequence {
                    sequence: event.sequence,
                    cursor: last_sequence,
                });
            }
            let expected = last_sequence
                .checked_add(1)
                .ok_or(InboxError::CounterOverflow("consumer cursor"))?;
            if event.sequence != expected {
                return Err(InboxError::SequenceGap {
                    expected,
                    received: event.sequence,
                });
            }
        }

        transaction.execute(
            r#"
            INSERT INTO consumer_events(
                consumer, source, event_id, stream, sequence, effects_hash, processed_at_micros
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                consumer.as_str(),
                event.source.as_str(),
                event.event_id.as_str(),
                event.stream.as_str(),
                sequence,
                effects_hash.to_hex(),
                processed_at.get(),
            ],
        )?;
        for effect in &effects {
            let effect_hash = hash_bytes(&effect.payload);
            transaction.execute(
                r#"
                INSERT INTO consumer_effects(
                    consumer, source, event_id, effect_key, payload, payload_hash
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
                params![
                    consumer.as_str(),
                    event.source.as_str(),
                    event.event_id.as_str(),
                    effect.key.as_str(),
                    &effect.payload,
                    effect_hash.to_hex(),
                ],
            )?;
        }
        transaction.execute(
            r#"
            INSERT INTO consumer_cursors(consumer, source, stream, last_sequence, updated_at_micros)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(consumer, source, stream) DO UPDATE SET
                last_sequence = excluded.last_sequence,
                updated_at_micros = excluded.updated_at_micros
            "#,
            params![
                consumer.as_str(),
                event.source.as_str(),
                event.stream.as_str(),
                sequence,
                processed_at.get(),
            ],
        )?;
        transaction.commit()?;

        Ok(InboxApplyResult::Applied {
            sequence: event.sequence,
            effect_count: effects.len(),
        })
    }

    pub fn consumer_cursor(
        &self,
        consumer: &BoundedText<128>,
        source: &BoundedText<128>,
        stream: &BoundedText<128>,
    ) -> Result<Option<u64>, InboxError> {
        self.connection
            .query_row(
                r#"
                SELECT last_sequence FROM consumer_cursors
                WHERE consumer = ?1 AND source = ?2 AND stream = ?3
                "#,
                params![consumer.as_str(), source.as_str(), stream.as_str()],
                |row| row.get::<_, i64>(0),
            )
            .optional()?
            .map(|value| nonnegative_u64(value, "consumer cursor"))
            .transpose()
    }

    pub fn consumer_effects(
        &self,
        consumer: &BoundedText<128>,
        source: &BoundedText<128>,
        event_id: &BoundedText<256>,
    ) -> Result<Vec<StoredConsumerEffect>, InboxError> {
        let transaction = self.connection.unchecked_transaction()?;
        let effects = load_consumer_effects(&transaction, consumer, source, event_id)?;
        transaction.commit()?;
        Ok(effects)
    }
}

fn load_consumer_effects(
    connection: &rusqlite::Connection,
    consumer: &BoundedText<128>,
    source: &BoundedText<128>,
    event_id: &BoundedText<256>,
) -> Result<Vec<StoredConsumerEffect>, InboxError> {
    let expected_effects_hash: Option<String> = connection
        .query_row(
            r#"
                SELECT effects_hash
                FROM consumer_events
                WHERE consumer = ?1 AND source = ?2 AND event_id = ?3
                "#,
            params![consumer.as_str(), source.as_str(), event_id.as_str()],
            |row| row.get(0),
        )
        .optional()?;
    let Some(expected_effects_hash) = expected_effects_hash else {
        return Ok(Vec::new());
    };
    let expected_effects_hash = ContentHash::from_hex(&expected_effects_hash).map_err(|error| {
        InboxError::InvalidStoredRecord(format!("invalid aggregate effects hash: {error}"))
    })?;

    validate_stored_effect_budget(connection, consumer, source, event_id)?;
    let mut statement = connection.prepare(
        r#"
            SELECT effect_key, payload, payload_hash
            FROM consumer_effects
            WHERE consumer = ?1 AND source = ?2 AND event_id = ?3
            ORDER BY effect_key ASC
            "#,
    )?;
    let rows = statement.query_map(
        params![consumer.as_str(), source.as_str(), event_id.as_str()],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Vec<u8>>(1)?,
                row.get::<_, String>(2)?,
            ))
        },
    )?;
    let mut effects = Vec::new();
    for row in rows {
        let (key, payload, payload_hash) = row?;
        let stored_hash = ContentHash::from_hex(&payload_hash).map_err(|error| {
            InboxError::InvalidStoredRecord(format!("invalid effect hash: {error}"))
        })?;
        let actual_hash = hash_bytes(&payload);
        if stored_hash != actual_hash {
            return Err(InboxError::EffectHashMismatch {
                key: key.clone(),
                expected: stored_hash,
                actual: actual_hash,
            });
        }
        effects.push(StoredConsumerEffect {
            key: BoundedText::try_new(key).map_err(|error| {
                InboxError::InvalidStoredRecord(format!("invalid effect key: {error}"))
            })?,
            payload,
            payload_hash: stored_hash,
        });
    }

    let actual_effects_hash = hash_stored_effects(&effects);
    if actual_effects_hash != expected_effects_hash {
        return Err(InboxError::EffectSetHashMismatch {
            expected: expected_effects_hash,
            actual: actual_effects_hash,
        });
    }
    Ok(effects)
}

fn validate_stored_effect_budget(
    connection: &rusqlite::Connection,
    consumer: &BoundedText<128>,
    source: &BoundedText<128>,
    event_id: &BoundedText<256>,
) -> Result<(), InboxError> {
    let (count, bytes): (i64, i64) = connection.query_row(
        "SELECT count(*), coalesce(sum(length(payload)), 0) FROM consumer_effects WHERE consumer = ?1 AND source = ?2 AND event_id = ?3",
        params![consumer.as_str(), source.as_str(), event_id.as_str()],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    let count =
        usize::try_from(count).map_err(|_| InboxError::CounterOverflow("stored effect count"))?;
    if count > MAX_CONSUMER_EFFECTS {
        return Err(InboxError::TooManyEffects(count));
    }
    let event_bytes: i64 = connection.query_row(
        "SELECT length(payload) FROM inbox_events WHERE source = ?1 AND event_id = ?2",
        params![source.as_str(), event_id.as_str()],
        |row| row.get(0),
    )?;
    let total = bytes
        .checked_add(event_bytes)
        .and_then(|total| usize::try_from(total).ok())
        .ok_or(InboxError::CounterOverflow(
            "stored aggregate inbox payload bytes",
        ))?;
    if total > MAX_INBOX_PAYLOAD_BYTES {
        return Err(InboxError::PayloadTooLarge {
            label: "stored inbox and consumer-effect payloads",
            size: total,
        });
    }
    Ok(())
}

fn validate_stored_event(
    connection: &rusqlite::Connection,
    source: &BoundedText<128>,
    event_id: &BoundedText<256>,
    expected_hash: ContentHash,
) -> Result<(), InboxError> {
    let size: i64 = connection.query_row(
        "SELECT length(payload) FROM inbox_events WHERE source = ?1 AND event_id = ?2",
        params![source.as_str(), event_id.as_str()],
        |row| row.get(0),
    )?;
    let size =
        usize::try_from(size).map_err(|_| InboxError::CounterOverflow("stored event bytes"))?;
    if size > MAX_INBOX_PAYLOAD_BYTES {
        return Err(InboxError::PayloadTooLarge {
            label: "stored inbox event",
            size,
        });
    }
    let payload: Vec<u8> = connection.query_row(
        "SELECT payload FROM inbox_events WHERE source = ?1 AND event_id = ?2",
        params![source.as_str(), event_id.as_str()],
        |row| row.get(0),
    )?;
    if hash_bytes(&payload) != expected_hash {
        return Err(InboxError::InvalidStoredRecord(
            "stored inbox payload does not match its digest".to_owned(),
        ));
    }
    Ok(())
}

fn validate_payload(payload: &[u8], label: &'static str) -> Result<(), InboxError> {
    if payload.len() > MAX_INBOX_PAYLOAD_BYTES {
        return Err(InboxError::PayloadTooLarge {
            label,
            size: payload.len(),
        });
    }
    Ok(())
}

#[must_use]
fn hash_bytes(payload: &[u8]) -> ContentHash {
    let digest = Sha256::digest(payload);
    let mut bytes = [0_u8; 32];
    bytes.copy_from_slice(&digest);
    ContentHash::from_bytes(bytes)
}

#[must_use]
fn hash_effects(effects: &[ConsumerEffect]) -> ContentHash {
    let mut hasher = Sha256::new();
    for effect in effects {
        hasher.update((effect.key.as_str().len() as u64).to_be_bytes());
        hasher.update(effect.key.as_str().as_bytes());
        hasher.update(hash_bytes(&effect.payload).into_bytes());
    }
    finalize_hash(hasher)
}

#[must_use]
fn hash_stored_effects(effects: &[StoredConsumerEffect]) -> ContentHash {
    let mut hasher = Sha256::new();
    for effect in effects {
        hasher.update((effect.key.as_str().len() as u64).to_be_bytes());
        hasher.update(effect.key.as_str().as_bytes());
        hasher.update(hash_bytes(&effect.payload).into_bytes());
    }
    finalize_hash(hasher)
}

#[must_use]
fn finalize_hash(hasher: Sha256) -> ContentHash {
    let digest = hasher.finalize();
    let mut bytes = [0_u8; 32];
    bytes.copy_from_slice(&digest);
    ContentHash::from_bytes(bytes)
}

fn nonnegative_u64(value: i64, label: &'static str) -> Result<u64, InboxError> {
    u64::try_from(value).map_err(|_| InboxError::InvalidStoredRecord(format!("negative {label}")))
}

fn to_sql_i64(value: u64, label: &'static str) -> Result<i64, InboxError> {
    i64::try_from(value).map_err(|_| InboxError::CounterOverflow(label))
}

#[derive(Debug)]
pub enum InboxError {
    Sqlite(rusqlite::Error),
    PayloadTooLarge {
        label: &'static str,
        size: usize,
    },
    TooManyEffects(usize),
    DuplicateEffectKey(String),
    EventIdentityCollision {
        source: String,
        event_id: String,
    },
    SequenceCollision {
        source: String,
        stream: String,
        sequence: u64,
        existing_event_id: String,
    },
    DuplicateDeliveryChanged {
        consumer: String,
        source: String,
        event_id: String,
    },
    StaleSequence {
        sequence: u64,
        cursor: u64,
    },
    SequenceGap {
        expected: u64,
        received: u64,
    },
    EffectHashMismatch {
        key: String,
        expected: ContentHash,
        actual: ContentHash,
    },
    EffectSetHashMismatch {
        expected: ContentHash,
        actual: ContentHash,
    },
    CounterOverflow(&'static str),
    InvalidStoredRecord(String),
}

impl fmt::Display for InboxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => write!(formatter, "SQLite inbox error: {error}"),
            Self::PayloadTooLarge { label, size } => write!(
                formatter,
                "{label} is {size} bytes; maximum is {MAX_INBOX_PAYLOAD_BYTES}"
            ),
            Self::TooManyEffects(count) => write!(
                formatter,
                "consumer produced {count} effects; maximum is {MAX_CONSUMER_EFFECTS}"
            ),
            Self::DuplicateEffectKey(key) => {
                write!(formatter, "duplicate consumer effect key {key}")
            }
            Self::EventIdentityCollision { source, event_id } => write!(
                formatter,
                "inbox event identity collision for {source}/{event_id}"
            ),
            Self::SequenceCollision {
                source,
                stream,
                sequence,
                existing_event_id,
            } => write!(
                formatter,
                "inbox sequence collision for {source}/{stream}/{sequence}; existing event {existing_event_id}"
            ),
            Self::DuplicateDeliveryChanged {
                consumer,
                source,
                event_id,
            } => write!(
                formatter,
                "duplicate delivery changed deterministic effects for {consumer}/{source}/{event_id}"
            ),
            Self::StaleSequence { sequence, cursor } => write!(
                formatter,
                "inbox sequence {sequence} is behind consumer cursor {cursor}"
            ),
            Self::SequenceGap { expected, received } => write!(
                formatter,
                "consumer sequence gap: expected {expected}, received {received}"
            ),
            Self::EffectHashMismatch {
                key,
                expected,
                actual,
            } => write!(
                formatter,
                "consumer effect {key} hash mismatch: expected {expected}, actual {actual}"
            ),
            Self::EffectSetHashMismatch { expected, actual } => write!(
                formatter,
                "consumer effect set hash mismatch: expected {expected}, actual {actual}"
            ),
            Self::CounterOverflow(label) => write!(formatter, "{label} exceeds storage range"),
            Self::InvalidStoredRecord(detail) => {
                write!(formatter, "invalid stored inbox record: {detail}")
            }
        }
    }
}

impl Error for InboxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Sqlite(error) => Some(error),
            _ => None,
        }
    }
}

impl From<rusqlite::Error> for InboxError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{ConsumerEffect, InboxApplyResult, InboxError, InboxEvent};
    use crate::StateStore;
    use intent_contracts::{BoundedText, UnixTimestampMicros};
    use rusqlite::params;
    use std::error::Error;

    fn event(sequence: u64, event_id: &str) -> Result<InboxEvent, Box<dyn Error>> {
        Ok(InboxEvent {
            source: BoundedText::try_new("connector")?,
            event_id: BoundedText::try_new(event_id)?,
            stream: BoundedText::try_new("orders")?,
            sequence,
            payload: format!("event-{sequence}").into_bytes(),
            received_at: UnixTimestampMicros::try_new(100 + i64::try_from(sequence)?)?,
        })
    }

    fn effects(value: &str) -> Result<Vec<ConsumerEffect>, Box<dyn Error>> {
        Ok(vec![ConsumerEffect {
            key: BoundedText::try_new("projection")?,
            payload: value.as_bytes().to_vec(),
        }])
    }

    #[test]
    fn duplicate_delivery_does_not_apply_effect_twice() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        let consumer = BoundedText::try_new("workspace-projector")?;
        let first = store.apply_inbox_event(
            &consumer,
            event(10, "evt-10")?,
            effects("v1")?,
            UnixTimestampMicros::try_new(200)?,
        )?;
        assert!(matches!(
            first,
            InboxApplyResult::Applied {
                effect_count: 1,
                ..
            }
        ));
        let duplicate = store.apply_inbox_event(
            &consumer,
            event(10, "evt-10")?,
            effects("v1")?,
            UnixTimestampMicros::try_new(201)?,
        )?;
        assert_eq!(duplicate, InboxApplyResult::Duplicate { sequence: 10 });
        let source = BoundedText::try_new("connector")?;
        let event_id = BoundedText::try_new("evt-10")?;
        assert_eq!(
            store.consumer_effects(&consumer, &source, &event_id)?.len(),
            1
        );
        Ok(())
    }

    #[test]
    fn missing_materialized_effect_is_detected_by_aggregate_hash() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        let consumer = BoundedText::try_new("workspace-projector")?;
        store.apply_inbox_event(
            &consumer,
            event(10, "evt-10")?,
            effects("v1")?,
            UnixTimestampMicros::try_new(200)?,
        )?;
        store.connection.execute(
            "DELETE FROM consumer_effects WHERE consumer = ?1 AND source = ?2 AND event_id = ?3",
            params![consumer.as_str(), "connector", "evt-10"],
        )?;
        let source = BoundedText::try_new("connector")?;
        let event_id = BoundedText::try_new("evt-10")?;
        let Err(error) = store.consumer_effects(&consumer, &source, &event_id) else {
            return Err("truncated materialized effects unexpectedly validated".into());
        };
        assert!(matches!(error, InboxError::EffectSetHashMismatch { .. }));
        Ok(())
    }

    #[test]
    fn sequence_gap_is_explicit_and_cursor_does_not_advance() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        let consumer = BoundedText::try_new("projector")?;
        store.apply_inbox_event(
            &consumer,
            event(20, "evt-20")?,
            effects("v20")?,
            UnixTimestampMicros::try_new(220)?,
        )?;
        let Err(error) = store.apply_inbox_event(
            &consumer,
            event(22, "evt-22")?,
            effects("v22")?,
            UnixTimestampMicros::try_new(222)?,
        ) else {
            return Err("sequence gap unexpectedly applied".into());
        };
        assert!(matches!(
            error,
            InboxError::SequenceGap {
                expected: 21,
                received: 22
            }
        ));
        let source = BoundedText::try_new("connector")?;
        let stream = BoundedText::try_new("orders")?;
        assert_eq!(
            store.consumer_cursor(&consumer, &source, &stream)?,
            Some(20)
        );
        Ok(())
    }

    #[test]
    fn changed_duplicate_effects_fail_closed() -> Result<(), Box<dyn Error>> {
        let mut store = StateStore::open_in_memory_for_tests()?;
        let consumer = BoundedText::try_new("projector")?;
        store.apply_inbox_event(
            &consumer,
            event(1, "evt-1")?,
            effects("original")?,
            UnixTimestampMicros::try_new(200)?,
        )?;
        let Err(error) = store.apply_inbox_event(
            &consumer,
            event(1, "evt-1")?,
            effects("changed")?,
            UnixTimestampMicros::try_new(201)?,
        ) else {
            return Err("changed duplicate unexpectedly accepted".into());
        };
        assert!(matches!(error, InboxError::DuplicateDeliveryChanged { .. }));
        Ok(())
    }
}
