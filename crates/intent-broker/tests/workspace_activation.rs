#![cfg(all(target_os = "linux", target_env = "gnu"))]

use intent_broker::{
    CoreDocumentImportMode, CoreDocumentPersistenceError, CoreWorkspaceActivationRequest,
    DurableCoreDocumentRequest, activate_persisted_core_workspace, persist_core_document_import,
};
use intent_contracts::{
    AccountId, ActionProposalId, ArtifactId, BoundedText, CapabilityId, ContentHash, OperationId,
    SchemaVersion, Task, TaskId, UnixTimestampMicros, Workspace, WorkspaceId,
};
use intent_ipc::{CoreRecordKind, WireLimits};
use intent_state::{
    ArtifactScope, NewDurableOperation, RecoveryError, RuntimeOwner, TaskDependency,
    WorkspaceCheckpointError,
};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{error::Error, fs, os::unix::fs::DirBuilderExt, path::PathBuf, str::FromStr};
use uuid::Uuid;

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

struct Profile {
    root: PathBuf,
    artifacts: PathBuf,
    scope: ArtifactScope,
    owner: RuntimeOwner,
}

impl Profile {
    fn new() -> Result<Self> {
        let root = std::env::temp_dir().join(format!(
            "intent-core-workspace-activation-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        fs::DirBuilder::new().mode(0o700).create(&root)?;
        let artifacts = root.join("artifacts");
        fs::DirBuilder::new().mode(0o700).create(&artifacts)?;
        let owner = RuntimeOwner::open_profile(&root, now(1)?)?;
        Ok(Self {
            root,
            artifacts,
            scope: ArtifactScope::try_new("profile:workspace-activation")?,
            owner,
        })
    }

    fn persist<T: Serialize>(
        &mut self,
        kind: CoreRecordKind,
        value: &T,
        created_at: i64,
    ) -> Result<ArtifactId> {
        let bytes = serde_json::to_vec(value)?;
        self.persist_bytes(kind, &bytes, created_at)
    }

    fn persist_bytes(
        &mut self,
        kind: CoreRecordKind,
        bytes: &[u8],
        created_at: i64,
    ) -> Result<ArtifactId> {
        let artifact_id = ArtifactId::from_uuid(Uuid::new_v4());
        persist_core_document_import(
            self.owner.state_mut(),
            &self.artifacts,
            DurableCoreDocumentRequest {
                artifact_id,
                privacy_scope: self.scope.clone(),
                created_at: now(created_at)?,
                kind,
                mode: CoreDocumentImportMode::Strict,
            },
            bytes,
            WireLimits::default(),
        )?;
        Ok(artifact_id)
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn id<T: FromStr>(n: u64) -> Result<T>
where
    T::Err: Error + 'static,
{
    Ok(format!("018f47f7-5a86-7c00-8000-{n:012x}").parse()?)
}

fn now(value: i64) -> Result<UnixTimestampMicros> {
    Ok(UnixTimestampMicros::try_new(value)?)
}

fn current_workspace(workspace_id: WorkspaceId) -> Result<Workspace> {
    Ok(Workspace::new(
        workspace_id,
        BoundedText::try_new("restored workspace")?,
        now(1)?,
    ))
}

fn current_task(workspace_id: WorkspaceId, n: u64) -> Result<Task> {
    Ok(Task::new(
        id(n)?,
        workspace_id,
        BoundedText::try_new("restored task remains non-authorizing")?,
        now(1)?,
    ))
}

fn request(
    privacy_scope: ArtifactScope,
    workspace_artifact: ArtifactId,
    task_artifacts: Vec<ArtifactId>,
    dependencies: Vec<TaskDependency>,
) -> Result<CoreWorkspaceActivationRequest> {
    Ok(CoreWorkspaceActivationRequest {
        privacy_scope,
        workspace_artifact,
        goal_artifacts: Vec::new(),
        task_artifacts,
        dependencies,
        activated_at: now(20)?,
    })
}

#[test]
fn activation_installs_only_current_records_while_dispatch_stays_fenced() -> Result {
    let mut profile = Profile::new()?;
    let workspace_id = id(1)?;
    let workspace = profile.persist(
        CoreRecordKind::Workspace,
        &current_workspace(workspace_id)?,
        2,
    )?;
    let task = profile.persist(CoreRecordKind::Task, &current_task(workspace_id, 2)?, 3)?;
    let activation = request(profile.scope.clone(), workspace, vec![task], Vec::new())?;

    let revision = activate_persisted_core_workspace(
        &mut profile.owner,
        &profile.artifacts,
        activation.clone(),
        WireLimits::default(),
    )?;
    assert_eq!(revision.workspace_id, workspace_id);
    assert_eq!(revision.revision, 1);
    assert!(!profile.owner.state().dispatch_status()?.enabled);

    profile.owner.activate_after_planning(now(30)?)?;
    assert!(profile.owner.state().dispatch_status()?.enabled);
    assert!(matches!(
        activate_persisted_core_workspace(
            &mut profile.owner,
            &profile.artifacts,
            activation,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::Recovery(
            RecoveryError::Denied("workspace activation requires the startup dispatch fence")
        ))
    ));
    Ok(())
}

#[test]
fn newer_minor_workspace_is_never_reinterpreted_as_writable_activation() -> Result {
    let mut profile = Profile::new()?;
    let workspace_id = id(10)?;
    let mut future: Value = serde_json::to_value(current_workspace(workspace_id)?)?;
    future["schema_version"]["minor"] = Value::from(1_u64);
    future["future_workspace_semantic"] = Value::from("opaque");
    let future_workspace =
        profile.persist_bytes(CoreRecordKind::Workspace, &serde_json::to_vec(&future)?, 2)?;
    let task = profile.persist(CoreRecordKind::Task, &current_task(workspace_id, 11)?, 3)?;
    assert!(matches!(
        activate_persisted_core_workspace(
            &mut profile.owner,
            &profile.artifacts,
            request(profile.scope.clone(), future_workspace, vec![task], Vec::new())?,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::ReadOnlyActivationArtifact(id)) if id == future_workspace
    ));

    let current_workspace = profile.persist(
        CoreRecordKind::Workspace,
        &current_workspace(workspace_id)?,
        4,
    )?;
    let revision = activate_persisted_core_workspace(
        &mut profile.owner,
        &profile.artifacts,
        request(
            profile.scope.clone(),
            current_workspace,
            vec![task],
            Vec::new(),
        )?,
        WireLimits::default(),
    )?;
    assert_eq!(revision.revision, 1, "rejected opaque input wrote no graph");
    Ok(())
}

#[test]
fn activation_rejects_wrong_family_duplicates_and_archived_workspace() -> Result {
    let mut profile = Profile::new()?;
    let workspace_id = id(20)?;
    let task = profile.persist(CoreRecordKind::Task, &current_task(workspace_id, 21)?, 2)?;
    assert!(matches!(
        activate_persisted_core_workspace(
            &mut profile.owner,
            &profile.artifacts,
            request(profile.scope.clone(), task, vec![task], Vec::new())?,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::DuplicateActivationArtifact(id)) if id == task
    ));

    let other_task = profile.persist(CoreRecordKind::Task, &current_task(workspace_id, 22)?, 3)?;
    assert!(matches!(
        activate_persisted_core_workspace(
            &mut profile.owner,
            &profile.artifacts,
            request(profile.scope.clone(), task, vec![other_task], Vec::new())?,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::UnexpectedActivationRecord {
            artifact_id,
            expected: CoreRecordKind::Workspace,
            actual: CoreRecordKind::Task,
        }) if artifact_id == task
    ));

    let mut archived: Value = serde_json::to_value(current_workspace(workspace_id)?)?;
    archived["state"] = Value::from("archived");
    let archived_id = profile.persist_bytes(
        CoreRecordKind::Workspace,
        &serde_json::to_vec(&archived)?,
        4,
    )?;
    assert!(matches!(
        activate_persisted_core_workspace(
            &mut profile.owner,
            &profile.artifacts,
            request(
                profile.scope.clone(),
                archived_id,
                vec![other_task],
                Vec::new()
            )?,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::InvalidWorkspaceActivation(
            "archived workspace cannot be activated"
        ))
    ));
    Ok(())
}

#[test]
fn invalid_dependency_graph_is_atomic_and_corrected_retry_can_activate() -> Result {
    let mut profile = Profile::new()?;
    let workspace_id = id(30)?;
    let workspace = profile.persist(
        CoreRecordKind::Workspace,
        &current_workspace(workspace_id)?,
        2,
    )?;
    let task_a_record = current_task(workspace_id, 31)?;
    let task_b_record = current_task(workspace_id, 32)?;
    let task_a = profile.persist(CoreRecordKind::Task, &task_a_record, 3)?;
    let task_b = profile.persist(CoreRecordKind::Task, &task_b_record, 4)?;
    let cycle = vec![
        TaskDependency {
            prerequisite: task_a_record.task_id(),
            dependent: task_b_record.task_id(),
        },
        TaskDependency {
            prerequisite: task_b_record.task_id(),
            dependent: task_a_record.task_id(),
        },
    ];
    assert!(matches!(
        activate_persisted_core_workspace(
            &mut profile.owner,
            &profile.artifacts,
            request(
                profile.scope.clone(),
                workspace,
                vec![task_a, task_b],
                cycle
            )?,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::WorkspaceGraph(
            WorkspaceCheckpointError::Invalid("cyclic task graph")
        ))
    ));

    let revision = activate_persisted_core_workspace(
        &mut profile.owner,
        &profile.artifacts,
        request(
            profile.scope.clone(),
            workspace,
            vec![task_a, task_b],
            vec![TaskDependency {
                prerequisite: task_a_record.task_id(),
                dependent: task_b_record.task_id(),
            }],
        )?,
        WireLimits::default(),
    )?;
    assert_eq!(
        revision.revision, 1,
        "failed graph left no durable partial state"
    );
    Ok(())
}

#[test]
fn activation_cannot_attach_existing_operation_history_to_imported_task() -> Result {
    let mut profile = Profile::new()?;
    let workspace_id = id(40)?;
    let task_record = current_task(workspace_id, 41)?;
    let task_id: TaskId = task_record.task_id();
    profile
        .owner
        .state_mut()
        .create_operation(NewDurableOperation {
            binding: None,
            operation_id: id::<OperationId>(42)?,
            task_id,
            action_proposal_id: id::<ActionProposalId>(43)?,
            account_id: id::<AccountId>(44)?,
            capability_id: id::<CapabilityId>(45)?,
            arguments_hash: ContentHash::from_bytes(Sha256::digest(b"archived action").into()),
            source_schema: SchemaVersion::V1,
            created_at: now(1)?,
        })?;
    let workspace = profile.persist(
        CoreRecordKind::Workspace,
        &current_workspace(workspace_id)?,
        2,
    )?;
    let task = profile.persist(CoreRecordKind::Task, &task_record, 3)?;

    assert!(matches!(
        activate_persisted_core_workspace(
            &mut profile.owner,
            &profile.artifacts,
            request(profile.scope.clone(), workspace, vec![task], Vec::new())?,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::WorkspaceGraph(
            WorkspaceCheckpointError::Invalid(
                "archive activation cannot attach existing operation history"
            )
        ))
    ));

    let clean_task = profile.persist(CoreRecordKind::Task, &current_task(workspace_id, 46)?, 4)?;
    let revision = activate_persisted_core_workspace(
        &mut profile.owner,
        &profile.artifacts,
        request(
            profile.scope.clone(),
            workspace,
            vec![clean_task],
            Vec::new(),
        )?,
        WireLimits::default(),
    )?;
    assert_eq!(
        revision.revision, 1,
        "operation-history rejection left no durable partial graph"
    );
    Ok(())
}

#[test]
fn terminal_imported_tasks_are_rejected_atomically() -> Result {
    let mut profile = Profile::new()?;
    let workspace_id = id(50)?;
    let workspace = profile.persist(
        CoreRecordKind::Workspace,
        &current_workspace(workspace_id)?,
        2,
    )?;

    for (offset, state) in ["completed", "cancelled", "failed"].into_iter().enumerate() {
        let mut task: Value =
            serde_json::to_value(current_task(workspace_id, 51 + offset as u64)?)?;
        task["state"] = Value::from(state);
        let artifact = profile.persist_bytes(
            CoreRecordKind::Task,
            &serde_json::to_vec(&task)?,
            3 + offset as i64,
        )?;
        assert!(matches!(
            activate_persisted_core_workspace(
                &mut profile.owner,
                &profile.artifacts,
                request(profile.scope.clone(), workspace, vec![artifact], Vec::new(),)?,
                WireLimits::default(),
            ),
            Err(CoreDocumentPersistenceError::WorkspaceGraph(
                WorkspaceCheckpointError::Invalid(
                    "archive activation cannot activate a terminal task"
                )
            ))
        ));
    }

    let active_task = profile.persist(CoreRecordKind::Task, &current_task(workspace_id, 54)?, 6)?;
    let revision = activate_persisted_core_workspace(
        &mut profile.owner,
        &profile.artifacts,
        request(
            profile.scope.clone(),
            workspace,
            vec![active_task],
            Vec::new(),
        )?,
        WireLimits::default(),
    )?;
    assert_eq!(
        revision.revision, 1,
        "terminal-task rejection left no durable partial graph"
    );
    assert!(!profile.owner.state().dispatch_status()?.enabled);
    Ok(())
}
