#![cfg(all(target_os = "linux", target_env = "gnu"))]

use intent_broker::{
    CoreDocumentImportMode, CoreDocumentPersistenceError, CoreWorkspaceActivationRequest,
    DurableCoreDocumentRequest, MAX_WORKSPACE_ACTIVATION_GOALS, activate_persisted_core_workspace,
    persist_core_document_import,
};
use intent_contracts::{
    ArtifactId, BoundedText, Task, UnixTimestampMicros, Workspace, WorkspaceId,
};
use intent_ipc::{CoreRecordKind, WireLimits};
use intent_state::ArtifactScope;
use intent_state::RuntimeOwner;
use serde::Serialize;
use serde_json::Value;
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
            "intent-core-workspace-activation-order-{}-{}",
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
            scope: ArtifactScope::try_new("profile:workspace-activation-order")?,
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
        BoundedText::try_new("validation-order workspace")?,
        now(1)?,
    ))
}

fn current_task(workspace_id: WorkspaceId, n: u64) -> Result<Task> {
    Ok(Task::new(
        id(n)?,
        workspace_id,
        BoundedText::try_new("validation-order task")?,
        now(1)?,
    ))
}

fn activation_request(
    scope: ArtifactScope,
    workspace: ArtifactId,
    goals: Vec<ArtifactId>,
    tasks: Vec<ArtifactId>,
) -> Result<CoreWorkspaceActivationRequest> {
    Ok(CoreWorkspaceActivationRequest {
        privacy_scope: scope,
        workspace_artifact: workspace,
        goal_artifacts: goals,
        task_artifacts: tasks,
        dependencies: Vec::new(),
        activated_at: now(20)?,
    })
}

#[test]
fn cheap_activation_validation_precedes_terminal_task_artifact_reads() -> Result {
    let mut profile = Profile::new()?;
    let workspace_id = id(70)?;
    let workspace = profile.persist(
        CoreRecordKind::Workspace,
        &current_workspace(workspace_id)?,
        2,
    )?;

    let mut terminal: Value = serde_json::to_value(current_task(workspace_id, 71)?)?;
    terminal["state"] = Value::from("completed");
    let terminal =
        profile.persist_bytes(CoreRecordKind::Task, &serde_json::to_vec(&terminal)?, 3)?;

    assert!(matches!(
        activate_persisted_core_workspace(
            &mut profile.owner,
            &profile.artifacts,
            activation_request(
                profile.scope.clone(),
                workspace,
                Vec::new(),
                vec![terminal, terminal],
            )?,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::DuplicateActivationArtifact(id)) if id == terminal
    ));

    assert!(matches!(
        activate_persisted_core_workspace(
            &mut profile.owner,
            &profile.artifacts,
            activation_request(
                profile.scope.clone(),
                workspace,
                vec![terminal; MAX_WORKSPACE_ACTIVATION_GOALS + 1],
                vec![terminal],
            )?,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::InvalidWorkspaceActivation(
            "workspace archive collection budget exceeded"
        ))
    ));

    let active = profile.persist(CoreRecordKind::Task, &current_task(workspace_id, 72)?, 4)?;
    let revision = activate_persisted_core_workspace(
        &mut profile.owner,
        &profile.artifacts,
        activation_request(profile.scope.clone(), workspace, Vec::new(), vec![active])?,
        WireLimits::default(),
    )?;
    assert_eq!(revision.revision, 1);
    assert!(!profile.owner.state().dispatch_status()?.enabled);
    Ok(())
}
