use crate::{OutboxError, StateError, StateStore};
use rusqlite::Connection;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispatchStatus {
    pub epoch: Uuid,
    pub enabled: bool,
    pub reason: String,
}

impl StateStore {
    pub fn dispatch_status(&self) -> Result<DispatchStatus, StateError> {
        let (epoch, enabled, reason): (String, bool, String) = self.connection.query_row(
            "SELECT epoch, dispatch_enabled, reason FROM runtime_control WHERE singleton = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;
        Ok(DispatchStatus {
            epoch: Uuid::parse_str(&epoch).map_err(|_| StateError::InvalidStoreId(epoch))?,
            enabled,
            reason,
        })
    }
}

pub(crate) fn require_dispatch(
    connection: &Connection,
    runtime_epoch: Option<Uuid>,
) -> Result<(), OutboxError> {
    let (epoch, enabled): (String, bool) = connection.query_row(
        "SELECT epoch, dispatch_enabled FROM runtime_control WHERE singleton = 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    if enabled && runtime_epoch.is_some_and(|expected| expected.to_string() == epoch) {
        Ok(())
    } else {
        Err(OutboxError::RecoveryRequired)
    }
}
