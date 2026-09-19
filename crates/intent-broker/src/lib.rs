#![forbid(unsafe_code)]
//! Runtime-owned bridge between durable authorization and authenticated cooperative workers.
//! Worker acknowledgement is transport evidence, never a verified external receipt.
mod error;
mod invocation;
mod record_import;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
mod runtime;
pub use error::*;
pub use invocation::*;
pub use record_import::*;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub use runtime::*;

#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub fn activate_persisted_core_workspace(
    owner: &mut intent_state::RuntimeOwner,
    artifact_root: &std::path::Path,
    request: record_import::CoreWorkspaceActivationRequest,
    limits: intent_ipc::WireLimits,
) -> Result<intent_state::GraphRevision, record_import::CoreDocumentPersistenceError> {
    use intent_contracts::TaskState;
    use intent_ipc::CoreRecord;
    use record_import::{CoreDocumentArchiveSelection, CoreDocumentPersistenceError};

    for artifact_id in request.task_artifacts.iter().copied() {
        if let CoreDocumentArchiveSelection::Current {
            record: CoreRecord::Task(task),
            ..
        } = record_import::select_persisted_core_document_import(
            owner.state(),
            artifact_root,
            &request.privacy_scope,
            artifact_id,
            limits,
        )? && matches!(
            task.state(),
            TaskState::Completed | TaskState::Cancelled | TaskState::Failed
        ) {
            return Err(CoreDocumentPersistenceError::WorkspaceGraph(
                intent_state::WorkspaceCheckpointError::Invalid(
                    "archive activation cannot activate a terminal task",
                ),
            ));
        }
    }

    record_import::activate_persisted_core_workspace(owner, artifact_root, request, limits)
}
