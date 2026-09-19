use intent_broker::{
    CoreDocumentArchiveSelection, CoreDocumentImportMode, CoreDocumentPersistenceError,
    DurableCoreDocumentRequest, MAX_CORE_DOCUMENT_ARCHIVE_PAGE, PersistedCoreDocumentImport,
    list_persisted_core_document_imports, persist_core_document_import,
    select_persisted_core_document_import,
};
use intent_contracts::{ArtifactId, BoundedText, UnixTimestampMicros};
use intent_ipc::{CoreRecordKind, WireErrorCode, WireLimits};
use intent_state::{ArtifactError, ArtifactScope, NewArtifact, StateStore};
use std::error::Error;
use std::fs;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use uuid::Uuid;

const GOAL_ID: &str = "018f47f7-5a86-7c00-8000-000000000001";

struct Fixture {
    root: PathBuf,
    artifacts: PathBuf,
    store: StateStore,
    scope: ArtifactScope,
}

impl Fixture {
    fn new() -> Result<Self, Box<dyn Error>> {
        let root = std::env::temp_dir().join(format!(
            "intent-core-import-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        fs::create_dir_all(&root)?;
        let artifacts = root.join("artifacts");
        let store = StateStore::open(root.join("state.sqlite3"))?;
        Ok(Self {
            root,
            artifacts,
            store,
            scope: ArtifactScope::try_new("profile:test")?,
        })
    }

    fn request(
        &self,
        mode: CoreDocumentImportMode,
    ) -> Result<DurableCoreDocumentRequest, Box<dyn Error>> {
        Ok(DurableCoreDocumentRequest {
            artifact_id: ArtifactId::from_uuid(Uuid::new_v4()),
            privacy_scope: self.scope.clone(),
            created_at: UnixTimestampMicros::try_new(1)?,
            kind: CoreRecordKind::GoalContract,
            mode,
        })
    }

    fn stored_bytes(&self, artifact_id: ArtifactId) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut file = self
            .store
            .open_verified_artifact(&self.artifacts, artifact_id, &self.scope)?
            .into_file();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn current_goal(minor_units: &str) -> Vec<u8> {
    format!(
        r#"{{"schema_version":{{"major":1,"minor":0}},"id":"{GOAL_ID}","original_request":"import","inference_mode":"offline","budget":{{"currency":"GBP","minor_units":"{minor_units}","scale":{{"known":2}}}},"success_predicate":"preserve","approval_requirement":"always"}}"#
    )
    .into_bytes()
}

fn legacy_goal(minor_units: &str) -> Vec<u8> {
    format!(
        r#"{{"schema_version":{{"major":1,"minor":0}},"id":"{GOAL_ID}","original_request":"import","inference_mode":"offline","budget":{{"currency":"GBP","minor_units":{minor_units},"scale":{{"known":2}}}},"success_predicate":"preserve","approval_requirement":"always"}}"#
    )
    .into_bytes()
}

fn future_goal() -> Vec<u8> {
    format!(
        " \n{{\"schema_version\":{{\"major\":1,\"minor\":1}},\"id\":\"{GOAL_ID}\",\"future_numeric\":1e400}}\t\n"
    )
    .into_bytes()
}

#[test]
fn current_import_is_canonicalized_before_durable_storage() -> Result<(), Box<dyn Error>> {
    let mut fixture = Fixture::new()?;
    let request = fixture.request(CoreDocumentImportMode::Strict)?;
    let artifact_id = request.artifact_id;
    let imported = persist_core_document_import(
        &mut fixture.store,
        &fixture.artifacts,
        request,
        &current_goal("123"),
        WireLimits::default(),
    )?;

    assert!(!imported.is_read_only());
    assert!(!imported.used_legacy_numeric_money_adapter());
    assert!(imported.record().is_some());
    assert_eq!(imported.artifact().artifact_id(), artifact_id);
    let stored = fixture.stored_bytes(artifact_id)?;
    let expected = b"\"minor_units\":\"123\"";
    assert!(
        stored
            .windows(expected.len())
            .any(|window| window == expected)
    );
    Ok(())
}

#[test]
fn newer_minor_import_persists_exact_bytes_and_stays_read_only() -> Result<(), Box<dyn Error>> {
    let mut fixture = Fixture::new()?;
    let request = fixture.request(CoreDocumentImportMode::Strict)?;
    let artifact_id = request.artifact_id;
    let source = future_goal();
    let imported = persist_core_document_import(
        &mut fixture.store,
        &fixture.artifacts,
        request,
        &source,
        WireLimits::default(),
    )?;

    assert!(matches!(
        &imported,
        PersistedCoreDocumentImport::ReadOnlyNewerMinor { .. }
    ));
    assert!(imported.is_read_only());
    assert!(imported.record().is_none());
    assert_eq!(fixture.stored_bytes(artifact_id)?, source);
    Ok(())
}

#[test]
fn legacy_numeric_money_requires_explicit_mode_and_persists_current_bytes()
-> Result<(), Box<dyn Error>> {
    let mut fixture = Fixture::new()?;
    let source = legacy_goal(&i128::MIN.to_string());
    let strict = fixture.request(CoreDocumentImportMode::Strict)?;
    let strict_id = strict.artifact_id;
    let result = persist_core_document_import(
        &mut fixture.store,
        &fixture.artifacts,
        strict,
        &source,
        WireLimits::default(),
    );
    assert!(matches!(
        result,
        Err(CoreDocumentPersistenceError::Wire(error)) if error.code() == WireErrorCode::InvalidRecord
    ));
    assert!(matches!(
        fixture.store.artifact_metadata(strict_id, &fixture.scope),
        Err(ArtifactError::ArtifactNotFound(id)) if id == strict_id
    ));

    let legacy = fixture.request(CoreDocumentImportMode::LegacyNumericMoneyGoalV1)?;
    let legacy_id = legacy.artifact_id;
    let imported = persist_core_document_import(
        &mut fixture.store,
        &fixture.artifacts,
        legacy,
        &source,
        WireLimits::default(),
    )?;
    assert!(imported.used_legacy_numeric_money_adapter());
    assert!(!imported.is_read_only());
    let stored = fixture.stored_bytes(legacy_id)?;
    let expected = format!("\"minor_units\":\"{}\"", i128::MIN);
    assert!(
        stored
            .windows(expected.len())
            .any(|window| window == expected.as_bytes())
    );
    Ok(())
}

#[test]
fn failed_validation_creates_no_durable_artifact_handle() -> Result<(), Box<dyn Error>> {
    let mut fixture = Fixture::new()?;
    let request = fixture.request(CoreDocumentImportMode::Strict)?;
    let artifact_id = request.artifact_id;
    let malformed = b"{\"schema_version\":{\"major\":1,\"minor\":0},\"budget\":";
    assert!(
        persist_core_document_import(
            &mut fixture.store,
            &fixture.artifacts,
            request,
            malformed,
            WireLimits::default(),
        )
        .is_err()
    );
    assert!(matches!(
        fixture.store.artifact_metadata(artifact_id, &fixture.scope),
        Err(ArtifactError::ArtifactNotFound(id)) if id == artifact_id
    ));
    assert!(!fixture.artifacts.exists());
    Ok(())
}

#[test]
fn legacy_mode_cannot_be_applied_to_another_record_family() -> Result<(), Box<dyn Error>> {
    let mut fixture = Fixture::new()?;
    let mut request = fixture.request(CoreDocumentImportMode::LegacyNumericMoneyGoalV1)?;
    request.kind = CoreRecordKind::Workspace;
    let artifact_id = request.artifact_id;
    assert!(matches!(
        persist_core_document_import(
            &mut fixture.store,
            &fixture.artifacts,
            request,
            &legacy_goal("1"),
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::LegacyModeRequiresGoalContract)
    ));
    assert!(matches!(
        fixture.store.artifact_metadata(artifact_id, &fixture.scope),
        Err(ArtifactError::ArtifactNotFound(id)) if id == artifact_id
    ));
    Ok(())
}

#[test]
fn reusing_an_import_artifact_id_with_different_bytes_fails_closed() -> Result<(), Box<dyn Error>> {
    let mut fixture = Fixture::new()?;
    let first = fixture.request(CoreDocumentImportMode::Strict)?;
    let artifact_id = first.artifact_id;
    let created_at = first.created_at;
    persist_core_document_import(
        &mut fixture.store,
        &fixture.artifacts,
        first,
        &current_goal("1"),
        WireLimits::default(),
    )?;
    let second = DurableCoreDocumentRequest {
        artifact_id,
        privacy_scope: fixture.scope.clone(),
        created_at,
        kind: CoreRecordKind::GoalContract,
        mode: CoreDocumentImportMode::Strict,
    };
    assert!(matches!(
        persist_core_document_import(
            &mut fixture.store,
            &fixture.artifacts,
            second,
            &current_goal("2"),
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::Artifact(ArtifactError::HandleConflict(id))) if id == artifact_id
    ));
    let stored = fixture.stored_bytes(artifact_id)?;
    let expected = b"\"minor_units\":\"1\"";
    assert!(
        stored
            .windows(expected.len())
            .any(|window| window == expected)
    );
    Ok(())
}

#[test]
fn archive_listing_and_selection_are_scoped_bounded_and_revalidated() -> Result<(), Box<dyn Error>>
{
    let mut fixture = Fixture::new()?;
    let mut current = fixture.request(CoreDocumentImportMode::Strict)?;
    current.created_at = UnixTimestampMicros::try_new(10)?;
    let current_id = current.artifact_id;
    persist_core_document_import(
        &mut fixture.store,
        &fixture.artifacts,
        current,
        &current_goal("7"),
        WireLimits::default(),
    )?;

    let mut future = fixture.request(CoreDocumentImportMode::Strict)?;
    future.created_at = UnixTimestampMicros::try_new(20)?;
    let future_id = future.artifact_id;
    persist_core_document_import(
        &mut fixture.store,
        &fixture.artifacts,
        future,
        &future_goal(),
        WireLimits::default(),
    )?;

    let other_scope = ArtifactScope::try_new("profile:other")?;
    let mut other = fixture.request(CoreDocumentImportMode::Strict)?;
    other.privacy_scope = other_scope.clone();
    other.created_at = UnixTimestampMicros::try_new(30)?;
    let other_id = other.artifact_id;
    persist_core_document_import(
        &mut fixture.store,
        &fixture.artifacts,
        other,
        &current_goal("9"),
        WireLimits::default(),
    )?;

    let ordinary_id = ArtifactId::from_uuid(Uuid::new_v4());
    fixture.store.store_artifact(
        &fixture.artifacts,
        NewArtifact {
            artifact_id: ordinary_id,
            privacy_scope: fixture.scope.clone(),
            media_type: BoundedText::try_new("application/octet-stream")?,
            created_at: UnixTimestampMicros::try_new(40)?,
        },
        &mut Cursor::new(b"not-a-core-document"),
    )?;

    let listed = list_persisted_core_document_imports(&fixture.store, &fixture.scope, 8)?;
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].artifact().artifact_id(), future_id);
    assert!(listed[0].is_read_only());
    assert_eq!(listed[1].artifact().artifact_id(), current_id);
    assert!(!listed[1].is_read_only());
    assert!(
        listed
            .iter()
            .all(|entry| entry.kind() == CoreRecordKind::GoalContract)
    );

    let selected = select_persisted_core_document_import(
        &fixture.store,
        &fixture.artifacts,
        &fixture.scope,
        current_id,
        WireLimits::default(),
    )?;
    assert!(matches!(
        selected,
        CoreDocumentArchiveSelection::Current { .. }
    ));
    assert_eq!(selected.kind(), CoreRecordKind::GoalContract);
    assert!(selected.record().is_some());
    assert!(!selected.is_read_only());

    let selected = select_persisted_core_document_import(
        &fixture.store,
        &fixture.artifacts,
        &fixture.scope,
        future_id,
        WireLimits::default(),
    )?;
    assert!(matches!(
        selected,
        CoreDocumentArchiveSelection::ReadOnlyNewerMinor { .. }
    ));
    assert!(selected.is_read_only());
    assert!(selected.record().is_none());

    assert!(matches!(
        select_persisted_core_document_import(
            &fixture.store,
            &fixture.artifacts,
            &fixture.scope,
            ordinary_id,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::NotCoreDocumentArtifact(id)) if id == ordinary_id
    ));
    assert!(matches!(
        select_persisted_core_document_import(
            &fixture.store,
            &fixture.artifacts,
            &fixture.scope,
            other_id,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::Artifact(ArtifactError::AccessDenied(id))) if id == other_id
    ));
    assert!(matches!(
        list_persisted_core_document_imports(&fixture.store, &fixture.scope, 0),
        Err(CoreDocumentPersistenceError::InvalidArchivePage)
    ));
    assert!(matches!(
        list_persisted_core_document_imports(
            &fixture.store,
            &fixture.scope,
            MAX_CORE_DOCUMENT_ARCHIVE_PAGE + 1,
        ),
        Err(CoreDocumentPersistenceError::InvalidArchivePage)
    ));
    Ok(())
}

#[test]
fn archive_selection_rejects_family_mode_metadata_that_disagrees_with_bytes()
-> Result<(), Box<dyn Error>> {
    let mut fixture = Fixture::new()?;
    let forged_current = ArtifactId::from_uuid(Uuid::new_v4());
    fixture.store.store_artifact(
        &fixture.artifacts,
        NewArtifact {
            artifact_id: forged_current,
            privacy_scope: fixture.scope.clone(),
            media_type: BoundedText::try_new(
                "application/vnd.intent.core-record+json;family=intent.goal_contract",
            )?,
            created_at: UnixTimestampMicros::try_new(50)?,
        },
        &mut Cursor::new(future_goal()),
    )?;
    assert!(matches!(
        select_persisted_core_document_import(
            &fixture.store,
            &fixture.artifacts,
            &fixture.scope,
            forged_current,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::InvalidArchiveMetadata(id)) if id == forged_current
    ));

    let forged_read_only = ArtifactId::from_uuid(Uuid::new_v4());
    fixture.store.store_artifact(
        &fixture.artifacts,
        NewArtifact {
            artifact_id: forged_read_only,
            privacy_scope: fixture.scope.clone(),
            media_type: BoundedText::try_new(
                "application/vnd.intent.core-record.readonly+json;family=intent.goal_contract",
            )?,
            created_at: UnixTimestampMicros::try_new(51)?,
        },
        &mut Cursor::new(current_goal("11")),
    )?;
    assert!(matches!(
        select_persisted_core_document_import(
            &fixture.store,
            &fixture.artifacts,
            &fixture.scope,
            forged_read_only,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::InvalidArchiveMetadata(id)) if id == forged_read_only
    ));
    Ok(())
}
