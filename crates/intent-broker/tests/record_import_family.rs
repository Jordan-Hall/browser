use intent_broker::{
    CoreDocumentImportMode, CoreDocumentPersistenceError, DurableCoreDocumentRequest,
    persist_core_document_import,
};
use intent_contracts::{ArtifactId, UnixTimestampMicros};
use intent_ipc::{CoreRecordKind, WireLimits};
use intent_state::{ArtifactError, ArtifactScope, StateStore};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

struct Fixture {
    root: PathBuf,
    artifacts: PathBuf,
    store: StateStore,
    scope: ArtifactScope,
}

impl Fixture {
    fn new() -> Result<Self, Box<dyn Error>> {
        let root = std::env::temp_dir().join(format!(
            "intent-core-import-family-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        fs::create_dir_all(&root)?;
        Ok(Self {
            artifacts: root.join("artifacts"),
            store: StateStore::open(root.join("state.sqlite3"))?,
            scope: ArtifactScope::try_new("profile:test")?,
            root,
        })
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn opaque_artifact_id_cannot_be_rebound_to_another_record_family() -> Result<(), Box<dyn Error>> {
    let mut fixture = Fixture::new()?;
    let artifact_id = ArtifactId::from_uuid(Uuid::new_v4());
    let created_at = UnixTimestampMicros::try_new(1)?;
    let source = br#"{"schema_version":{"major":1,"minor":1},"future_numeric":1e400}"#;
    let first = DurableCoreDocumentRequest {
        artifact_id,
        privacy_scope: fixture.scope.clone(),
        created_at,
        kind: CoreRecordKind::GoalContract,
        mode: CoreDocumentImportMode::Strict,
    };
    persist_core_document_import(
        &mut fixture.store,
        &fixture.artifacts,
        first,
        source,
        WireLimits::default(),
    )?;

    let rebound = DurableCoreDocumentRequest {
        artifact_id,
        privacy_scope: fixture.scope.clone(),
        created_at,
        kind: CoreRecordKind::Workspace,
        mode: CoreDocumentImportMode::Strict,
    };
    assert!(matches!(
        persist_core_document_import(
            &mut fixture.store,
            &fixture.artifacts,
            rebound,
            source,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::Artifact(ArtifactError::HandleConflict(id))) if id == artifact_id
    ));
    assert!(
        fixture
            .store
            .artifact_metadata(artifact_id, &fixture.scope)?
            .media_type()
            .as_str()
            .ends_with("family=intent.goal_contract")
    );
    Ok(())
}
