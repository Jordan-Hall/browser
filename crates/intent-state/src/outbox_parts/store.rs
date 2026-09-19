impl StateStore {
    pub(crate) fn stage_outbox(
        &mut self,
        new: NewOutboxMessage,
        expected_operation_revision: u64,
    ) -> Result<OutboxMessage, OutboxError> {
        validate_payload(&new.payload)?;
        let payload_hash = hash_payload(&new.payload);
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let managed: bool = transaction.query_row(
            "SELECT EXISTS(SELECT 1 FROM recovery_actions WHERE operation_id=?1)",
            [new.operation_id.to_string()],
            |row| row.get(0),
        )?;
        if managed {
            return Err(OutboxError::RecoveryRequired);
        }
        let (operation_state, actual_revision): (String, i64) = transaction
            .query_row(
                "SELECT state, revision FROM durable_operations WHERE operation_id = ?1",
                [new.operation_id.to_string()],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?
            .ok_or(OutboxError::OperationNotFound(new.operation_id))?;
        let actual_revision = nonnegative_u64(actual_revision, "operation revision")?;
        if actual_revision != expected_operation_revision {
            return Err(OutboxError::StaleOperationRevision {
                operation_id: new.operation_id,
                expected: expected_operation_revision,
                actual: actual_revision,
            });
        }
        if operation_state != "approved" {
            return Err(OutboxError::OperationNotDispatchable {
                operation_id: new.operation_id,
                state: operation_state,
            });
        }
        let next_revision = expected_operation_revision
            .checked_add(1)
            .ok_or(OutboxError::CounterOverflow("operation revision"))?;

        transaction.execute(
            "UPDATE durable_operations SET state = 'dispatch_pending', revision = ?1, updated_at_micros = ?2, attempt_identity = ?5 WHERE operation_id = ?3 AND revision = ?4",
            params![
                to_sql_i64(next_revision, "operation revision")?,
                new.created_at.get(),
                new.operation_id.to_string(),
                to_sql_i64(expected_operation_revision, "expected operation revision")?,
                new.attempt_identity.to_string(),
            ],
        )?;
        append_operation_journal(
            &transaction,
            OperationJournalWrite {
                operation_id: new.operation_id,
                revision: next_revision,
                from_state: "approved",
                to_state: "dispatch_pending",
                attempt_identity: Some(new.attempt_identity),
                detail: None,
                occurred_at: new.created_at,
            },
        )?;
        transaction.execute(
            r#"
            INSERT INTO outbox_messages(
                outbox_id, operation_id, attempt_identity, destination, message_kind,
                payload, payload_hash, state, created_at_micros, updated_at_micros
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'pending', ?8, ?8)
            "#,
            params![
                new.outbox_id.to_string(),
                new.operation_id.to_string(),
                new.attempt_identity.to_string(),
                new.destination.as_str(),
                new.message_kind.as_str(),
                new.payload,
                payload_hash.to_hex(),
                new.created_at.get(),
            ],
        )?;
        transaction.commit()?;
        self.load_outbox(new.outbox_id)?.ok_or_else(|| {
            OutboxError::InvalidStoredRecord(
                "outbox disappeared after successful staging transaction".to_owned(),
            )
        })
    }

    pub fn claim_outbox(
        &mut self,
        lease_owner: BoundedText<128>,
        now: UnixTimestampMicros,
        lease_expires_at: UnixTimestampMicros,
        limit: usize,
    ) -> Result<Vec<OutboxClaim>, OutboxError> {
        if lease_expires_at.get() <= now.get() {
            return Err(OutboxError::InvalidLeaseWindow);
        }
        let limit = limit.min(MAX_OUTBOX_CLAIM_BATCH);
        if limit == 0 {
            return Ok(Vec::new());
        }
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut claimed = Vec::new();
        let mut raw_ids = Vec::new();
        {
            let mut statement = transaction.prepare(
                r#"
                SELECT outbox.outbox_id, outbox.operation_id, outbox.attempt_identity
                FROM outbox_messages AS outbox
                JOIN durable_operations AS operation ON operation.operation_id = outbox.operation_id
                WHERE operation.state = 'dispatch_pending'
                  AND NOT EXISTS (SELECT 1 FROM recovery_actions a WHERE a.operation_id=operation.operation_id)
                  AND (outbox.state = 'pending'
                    OR (outbox.state = 'leased' AND outbox.lease_expires_at_micros <= ?1))
                ORDER BY outbox.created_at_micros ASC, outbox.outbox_id ASC
                LIMIT ?2
                "#,
            )?;
            let rows = statement.query_map(
                params![now.get(), to_sql_i64(limit as u64, "claim limit")?],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                },
            )?;
            for row in rows {
                let (outbox_id, operation_id, attempt_identity) = row?;
                claimed.push(OutboxClaim {
                    outbox_id: parse_id(&outbox_id, "outbox id")?,
                    operation_id: parse_id(&operation_id, "operation id")?,
                    attempt_identity: parse_id(&attempt_identity, "attempt identity")?,
                    lease_expires_at,
                });
                raw_ids.push(outbox_id);
            }
        }
        for id in &raw_ids {
            transaction.execute(
                r#"
                UPDATE outbox_messages
                SET state = 'leased', lease_owner = ?1, lease_expires_at_micros = ?2,
                    updated_at_micros = ?3
                WHERE outbox_id = ?4
                "#,
                params![lease_owner.as_str(), lease_expires_at.get(), now.get(), id],
            )?;
        }
        transaction.commit()?;
        Ok(claimed)
    }

    pub fn begin_dispatch(
        &mut self,
        outbox_id: OutboxMessageId,
        lease_owner: &BoundedText<128>,
        now: UnixTimestampMicros,
    ) -> Result<DispatchAttempt, OutboxError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        if transaction.path() != Some("") {
            return Err(OutboxError::RecoveryRequired);
        }
        crate::runtime_gate::require_dispatch(&transaction, self.runtime_epoch)?;
        let raw = load_outbox_from_connection(&transaction, outbox_id)?
            .ok_or(OutboxError::OutboxNotFound(outbox_id))?;
        if raw.state != OutboxState::Leased
            || raw.lease_owner.as_ref() != Some(lease_owner)
            || raw
                .lease_expires_at
                .is_none_or(|expires| expires.get() <= now.get())
        {
            return Err(OutboxError::LeaseNotHeld(outbox_id));
        }
        let (operation_state, revision, active_attempt): (String, i64, Option<String>) = transaction.query_row(
            "SELECT state, revision, attempt_identity FROM durable_operations WHERE operation_id = ?1",
            [raw.operation_id.to_string()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;
        let expected_attempt = raw.attempt_identity.to_string();
        if operation_state != "dispatch_pending"
            || active_attempt.as_deref() != Some(expected_attempt.as_str())
        {
            return Err(OutboxError::OperationNotDispatchable {
                operation_id: raw.operation_id,
                state: operation_state,
            });
        }
        let revision = nonnegative_u64(revision, "operation revision")?;
        let next_revision = revision
            .checked_add(1)
            .ok_or(OutboxError::CounterOverflow("operation revision"))?;

        transaction.execute(
            r#"
            UPDATE outbox_messages
            SET state = 'attempting', lease_owner = NULL, lease_expires_at_micros = NULL,
                dispatch_started_at_micros = ?1, updated_at_micros = ?1
            WHERE outbox_id = ?2
            "#,
            params![now.get(), outbox_id.to_string()],
        )?;
        transaction.execute(
            r#"
            UPDATE durable_operations
            SET state = 'attempting', attempt_identity = ?1, revision = ?2,
                updated_at_micros = ?3
            WHERE operation_id = ?4 AND revision = ?5
            "#,
            params![
                raw.attempt_identity.to_string(),
                to_sql_i64(next_revision, "operation revision")?,
                now.get(),
                raw.operation_id.to_string(),
                to_sql_i64(revision, "expected operation revision")?,
            ],
        )?;
        append_operation_journal(
            &transaction,
            OperationJournalWrite {
                operation_id: raw.operation_id,
                revision: next_revision,
                from_state: "dispatch_pending",
                to_state: "attempting",
                attempt_identity: Some(raw.attempt_identity),
                detail: None,
                occurred_at: now,
            },
        )?;
        transaction.commit()?;

        Ok(DispatchAttempt {
            outbox_id: raw.outbox_id,
            operation_id: raw.operation_id,
            attempt_identity: raw.attempt_identity,
            destination: raw.destination,
            message_kind: raw.message_kind,
            payload: raw.payload,
            payload_hash: raw.payload_hash,
        })
    }

    pub fn record_dispatch_result(
        &mut self,
        outbox_id: OutboxMessageId,
        result: DispatchResult,
        occurred_at: UnixTimestampMicros,
    ) -> Result<(), OutboxError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let raw = load_outbox_from_connection(&transaction, outbox_id)?
            .ok_or(OutboxError::OutboxNotFound(outbox_id))?;
        let managed: bool = transaction.query_row(
            "SELECT EXISTS(SELECT 1 FROM recovery_actions WHERE operation_id=?1)",
            [raw.operation_id.to_string()],
            |row| row.get(0),
        )?;
        if managed {
            return Err(OutboxError::RecoveryRequired);
        }
        if raw.state != OutboxState::Attempting {
            return Err(OutboxError::InvalidOutboxTransition {
                outbox_id,
                state: raw.state,
            });
        }
        let (operation_state, revision, active_attempt): (String, i64, Option<String>) = transaction.query_row(
            "SELECT state, revision, attempt_identity FROM durable_operations WHERE operation_id = ?1",
            [raw.operation_id.to_string()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;
        let expected_attempt = raw.attempt_identity.to_string();
        if operation_state != "attempting"
            || active_attempt.as_deref() != Some(expected_attempt.as_str())
        {
            return Err(OutboxError::OperationNotDispatchable {
                operation_id: raw.operation_id,
                state: operation_state,
            });
        }
        let revision = nonnegative_u64(revision, "operation revision")?;
        let next_revision = revision
            .checked_add(1)
            .ok_or(OutboxError::CounterOverflow("operation revision"))?;
        let (outbox_state, operation_state, detail) = match &result {
            DispatchResult::Accepted => ("completed", "accepted", None),
            DispatchResult::Ambiguous(detail) => {
                ("failed", "needs_reconciliation", Some(detail.as_str()))
            }
            DispatchResult::Rejected(detail) => ("failed", "failed", Some(detail.as_str())),
        };
        transaction.execute(
            r#"
            UPDATE outbox_messages
            SET state = ?1, completed_at_micros = ?2, failure_detail = ?3,
                updated_at_micros = ?2
            WHERE outbox_id = ?4
            "#,
            params![
                outbox_state,
                occurred_at.get(),
                detail,
                outbox_id.to_string()
            ],
        )?;
        transaction.execute(
            r#"
            UPDATE durable_operations
            SET state = ?1, state_detail = ?2, revision = ?3, updated_at_micros = ?4
            WHERE operation_id = ?5 AND revision = ?6
            "#,
            params![
                operation_state,
                detail,
                to_sql_i64(next_revision, "operation revision")?,
                occurred_at.get(),
                raw.operation_id.to_string(),
                to_sql_i64(revision, "expected operation revision")?,
            ],
        )?;
        append_operation_journal(
            &transaction,
            OperationJournalWrite {
                operation_id: raw.operation_id,
                revision: next_revision,
                from_state: "attempting",
                to_state: operation_state,
                attempt_identity: Some(raw.attempt_identity),
                detail,
                occurred_at,
            },
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub(crate) fn load_outbox(
        &self,
        outbox_id: OutboxMessageId,
    ) -> Result<Option<OutboxMessage>, OutboxError> {
        load_outbox_from_connection(&self.connection, outbox_id)
    }
}
