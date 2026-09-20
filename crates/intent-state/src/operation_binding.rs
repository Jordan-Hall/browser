use crate::{NewDurableOperation, StateError};
use intent_contracts::{AccountId, ActionBinding, CapabilityId, ContentHash, OperationId, TaskId};
use rusqlite::{Connection, OptionalExtension, params};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(
    tag = "schema",
    content = "binding",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum StoredActionBinding {
    V1(ActionBinding),
}

pub(crate) fn insert_binding(
    db: &Connection,
    operation: &NewDurableOperation,
) -> Result<(), StateError> {
    let Some(binding) = operation.binding.as_ref() else {
        return Ok(());
    };
    validate_identity(
        binding,
        operation.task_id,
        operation.account_id,
        operation.capability_id,
        operation.arguments_hash,
    )?;
    let json = serde_json::to_string(&StoredActionBinding::V1(binding.clone()))
        .map_err(|error| StateError::InvalidStoredOperation(error.to_string()))?;
    db.execute(
        "INSERT INTO durable_operation_bindings(operation_id,binding_json) VALUES(?1,?2)",
        params![operation.operation_id.to_string(), json],
    )?;
    Ok(())
}

pub(crate) fn load_binding(
    db: &Connection,
    operation_id: OperationId,
    required: bool,
) -> Result<Option<ActionBinding>, StateError> {
    let json: Option<String> = db
        .query_row(
            "SELECT binding_json FROM durable_operation_bindings WHERE operation_id=?1",
            [operation_id.to_string()],
            |row| row.get(0),
        )
        .optional()?;
    if json.is_some() != required {
        return Err(StateError::InvalidStoredOperation(
            "missing or retrofitted action binding".into(),
        ));
    }
    json.map(|json| {
        serde_json::from_str::<StoredActionBinding>(&json)
            .map(|stored| match stored {
                StoredActionBinding::V1(binding) => binding,
            })
            .map_err(|error| {
                StateError::InvalidStoredOperation(format!("invalid action binding: {error}"))
            })
    })
    .transpose()
}

pub(crate) fn validate_identity(
    binding: &ActionBinding,
    task: TaskId,
    account: AccountId,
    capability: CapabilityId,
    arguments: ContentHash,
) -> Result<(), StateError> {
    match binding.context.canonicalization {
        intent_contracts::CanonicalizationVersion::ExactBytesV1 => {}
    }
    if binding.task_id != task
        || binding.account_id != account
        || binding.capability_id != capability
        || binding.canonical_arguments.content_hash() != arguments
    {
        return Err(StateError::InvalidStoredOperation(
            "action binding identity mismatch".into(),
        ));
    }
    Ok(())
}
