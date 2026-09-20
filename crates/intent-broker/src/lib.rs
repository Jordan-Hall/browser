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
    use std::collections::BTreeSet;

    if request.goal_artifacts.len() > record_import::MAX_WORKSPACE_ACTIVATION_GOALS
        || request.task_artifacts.is_empty()
        || request.task_artifacts.len() > intent_state::MAX_GRAPH_TASKS
        || request.dependencies.len() > intent_state::MAX_GRAPH_DEPENDENCIES
    {
        return Err(CoreDocumentPersistenceError::InvalidWorkspaceActivation(
            "workspace archive collection budget exceeded",
        ));
    }

    let mut unique = BTreeSet::new();
    for artifact_id in std::iter::once(request.workspace_artifact)
        .chain(request.goal_artifacts.iter().copied())
        .chain(request.task_artifacts.iter().copied())
    {
        if !unique.insert(artifact_id.to_string()) {
            return Err(CoreDocumentPersistenceError::DuplicateActivationArtifact(
                artifact_id,
            ));
        }
    }

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
