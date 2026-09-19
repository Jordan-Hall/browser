use intent_broker::{
    CoreDocumentImportMode, CoreDocumentPersistenceError, DurableCoreDocumentRequest,
    list_persisted_core_document_imports, persist_core_document_import,
    select_persisted_core_document_import,
};
use intent_contracts::{ArtifactId, BoundedText, UnixTimestampMicros};
use intent_ipc::{CoreRecordKind, WireLimits};
use intent_state::{ArtifactError, ArtifactScope, NewArtifact, StateStore};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Cursor;
use std::path::{Path, PathBuf};
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
            "intent-core-archive-hardening-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        fs::create_dir_all(&root)?;
        Ok(Self {
            artifacts: root.join("artifacts"),
            store: StateStore::open(root.join("state.sqlite3"))?,
            scope: ArtifactScope::try_new("profile:archive-hardening")?,
            root,
        })
    }

    fn current_request(
        &self,
        created_at: i64,
    ) -> Result<DurableCoreDocumentRequest, Box<dyn Error>> {
        Ok(DurableCoreDocumentRequest {
            artifact_id: ArtifactId::from_uuid(Uuid::new_v4()),
            privacy_scope: self.scope.clone(),
            created_at: UnixTimestampMicros::try_new(created_at)?,
            kind: CoreRecordKind::GoalContract,
            mode: CoreDocumentImportMode::Strict,
        })
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn current_goal() -> Vec<u8> {
    format!(
        r#"{{"schema_version":{{"major":1,"minor":0}},"id":"{GOAL_ID}","original_request":"archive","inference_mode":"offline","budget":{{"currency":"GBP","minor_units":"1","scale":{{"known":2}}}},"success_predicate":"preserve","approval_requirement":"always"}}"#
    )
    .into_bytes()
}

fn find_blob(root: &Path, hash: &str) -> Result<PathBuf, Box<dyn Error>> {
    for scope_dir in fs::read_dir(root.join("blobs"))? {
        let candidate = scope_dir?.path().join(hash);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err("stored blob not found".into())
}

#[test]
fn unsupported_newer_family_cannot_hide_a_supported_archive_entry() -> Result<(), Box<dyn Error>> {
    let mut fixture = Fixture::new()?;
    let request = fixture.current_request(10)?;
    let valid_id = request.artifact_id;
    persist_core_document_import(
        &mut fixture.store,
        &fixture.artifacts,
        request,
        &current_goal(),
        WireLimits::default(),
    )?;

    let future_id = ArtifactId::from_uuid(Uuid::new_v4());
    fixture.store.store_artifact(
        &fixture.artifacts,
        NewArtifact {
            artifact_id: future_id,
            privacy_scope: fixture.scope.clone(),
            media_type: BoundedText::try_new(
                "application/vnd.intent.core-record+json;family=intent.future_family",
            )?,
            created_at: UnixTimestampMicros::try_new(20)?,
        },
        &mut Cursor::new(b"unsupported-family"),
    )?;

    let listed = list_persisted_core_document_imports(&fixture.store, &fixture.scope, 1)?;
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].artifact().artifact_id(), valid_id);
    assert_eq!(listed[0].kind(), CoreRecordKind::GoalContract);
    assert!(!listed[0].is_read_only());
    Ok(())
}

#[test]
fn selection_verification_stops_at_recorded_size_plus_one() -> Result<(), Box<dyn Error>> {
    let mut fixture = Fixture::new()?;
    let request = fixture.current_request(10)?;
    let artifact_id = request.artifact_id;
    let imported = persist_core_document_import(
        &mut fixture.store,
        &fixture.artifacts,
        request,
        &current_goal(),
        WireLimits::default(),
    )?;
    let metadata = imported.artifact();
    let blob = find_blob(&fixture.artifacts, &metadata.content_hash().to_hex())?;
    OpenOptions::new()
        .write(true)
        .open(blob)?
        .set_len(metadata.byte_size().saturating_add(16 * 1024 * 1024))?;

    assert!(matches!(
        select_persisted_core_document_import(
            &fixture.store,
            &fixture.artifacts,
            &fixture.scope,
            artifact_id,
            WireLimits::default(),
        ),
        Err(CoreDocumentPersistenceError::Artifact(ArtifactError::InvalidStoredRecord(detail)))
            if detail.contains("changed size during bounded verification")
    ));
    Ok(())
}
