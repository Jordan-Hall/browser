use crate::artifacts::artifact_blob_path;
use crate::{ArtifactError, ArtifactMetadata, ArtifactScope, StateStore};
use intent_contracts::{ArtifactId, BoundedText, ContentHash, UnixTimestampMicros};
use rusqlite::params;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

pub const MAX_ARTIFACT_METADATA_PAGE: usize = 256;

/// Bounded metadata used for archive discovery. Full artifact metadata and blob integrity are
/// reloaded by artifact ID when the caller selects an entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactCatalogEntry {
    artifact_id: ArtifactId,
    media_type: BoundedText<255>,
    created_at: UnixTimestampMicros,
}

impl ArtifactCatalogEntry {
    #[must_use]
    pub const fn artifact_id(&self) -> ArtifactId {
        self.artifact_id
    }

    #[must_use]
    pub const fn media_type(&self) -> &BoundedText<255> {
        &self.media_type
    }

    #[must_use]
    pub const fn created_at(&self) -> UnixTimestampMicros {
        self.created_at
    }
}

impl StateStore {
    /// Lists live artifact handles in one privacy scope whose media type begins with an exact
    /// prefix. Results are deterministic and bounded; suppressed handles are never returned.
    pub fn list_artifact_metadata_by_media_prefix(
        &self,
        permitted_scope: &ArtifactScope,
        media_type_prefix: &str,
        limit: usize,
    ) -> Result<Vec<ArtifactCatalogEntry>, ArtifactError> {
        validate_catalog_query(media_type_prefix, limit, "media-type prefix")?;
        let sql_limit = sql_limit(limit)?;
        let mut statement = self.connection.prepare(
            r#"
            SELECT artifact_id, media_type, created_at_micros
              FROM artifact_handles
             WHERE privacy_scope = ?1
               AND suppressed_at_micros IS NULL
               AND substr(media_type, 1, length(?2)) = ?2
             ORDER BY created_at_micros DESC, artifact_id ASC
             LIMIT ?3
            "#,
        )?;
        let rows = statement.query_map(
            params![permitted_scope.as_str(), media_type_prefix, sql_limit],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            },
        )?;
        let mut entries = Vec::new();
        for row in rows {
            let (artifact_id, media_type, created_at) = row?;
            entries.push(catalog_entry(artifact_id, media_type, created_at)?);
        }
        Ok(entries)
    }

    /// Lists live artifact handles in one privacy scope whose media type exactly matches the
    /// requested value. Filtering occurs in SQL before LIMIT, so unsupported suffixes cannot
    /// crowd supported records out of a bounded product page.
    pub fn list_artifact_metadata_by_media_type(
        &self,
        permitted_scope: &ArtifactScope,
        media_type: &str,
        limit: usize,
    ) -> Result<Vec<ArtifactCatalogEntry>, ArtifactError> {
        validate_catalog_query(media_type, limit, "media type")?;
        let sql_limit = sql_limit(limit)?;
        let mut statement = self.connection.prepare(
            r#"
            SELECT artifact_id, media_type, created_at_micros
              FROM artifact_handles
             WHERE privacy_scope = ?1
               AND suppressed_at_micros IS NULL
               AND media_type = ?2
             ORDER BY created_at_micros DESC, artifact_id ASC
             LIMIT ?3
            "#,
        )?;
        let rows = statement.query_map(
            params![permitted_scope.as_str(), media_type, sql_limit],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            },
        )?;
        let mut entries = Vec::new();
        for row in rows {
            let (artifact_id, media_type, created_at) = row?;
            entries.push(catalog_entry(artifact_id, media_type, created_at)?);
        }
        Ok(entries)
    }

    /// Reads, verifies and captures an artifact in one bounded pass. The durable byte size must
    /// fit within `maximum`; the filesystem read is then capped at that recorded size plus one so
    /// an appended/corrupted blob cannot force verification to consume the global artifact limit.
    /// The returned bytes are the exact bytes that were hashed, eliminating a second-read TOCTOU.
    pub fn read_verified_artifact_bytes_bounded(
        &self,
        root: &Path,
        artifact_id: ArtifactId,
        permitted_scope: &ArtifactScope,
        maximum: u64,
    ) -> Result<(ArtifactMetadata, Vec<u8>), ArtifactError> {
        let metadata = self.artifact_metadata(artifact_id, permitted_scope)?;
        if metadata.byte_size() > maximum {
            return Err(ArtifactError::ArtifactTooLarge {
                size: metadata.byte_size(),
                maximum,
            });
        }
        let expected_size = usize::try_from(metadata.byte_size())
            .map_err(|_| ArtifactError::SizeOverflow)?;
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(expected_size).map_err(|error| {
            ArtifactError::InvalidInput(format!(
                "failed to reserve bounded artifact verification buffer: {error}"
            ))
        })?;

        let path = artifact_blob_path(root, &metadata);
        let file_type = fs::symlink_metadata(&path)
            .map_err(|error| map_missing_blob(error, artifact_id))?
            .file_type();
        if !file_type.is_file() || file_type.is_symlink() {
            return Err(ArtifactError::InvalidBlobType(artifact_id));
        }
        let mut file = File::open(&path).map_err(|error| map_missing_blob(error, artifact_id))?;
        file.by_ref()
            .take(metadata.byte_size().saturating_add(1))
            .read_to_end(&mut bytes)?;

        let observed_size = u64::try_from(bytes.len()).map_err(|_| ArtifactError::SizeOverflow)?;
        if observed_size != metadata.byte_size() {
            return Err(ArtifactError::InvalidStoredRecord(format!(
                "artifact {artifact_id} changed size during bounded verification: expected {}, observed bounded read {}",
                metadata.byte_size(),
                observed_size
            )));
        }
        let actual_hash = hash_bytes(&bytes);
        if actual_hash != metadata.content_hash() {
            return Err(ArtifactError::BlobIntegrityMismatch {
                artifact_id,
                expected_hash: metadata.content_hash(),
                actual_hash,
                expected_size: metadata.byte_size(),
                actual_size: observed_size,
            });
        }
        Ok((metadata, bytes))
    }
}

fn validate_catalog_query(value: &str, limit: usize, label: &str) -> Result<(), ArtifactError> {
    if value.is_empty() || value.len() > 255 {
        return Err(ArtifactError::InvalidInput(format!(
            "artifact {label} must contain 1..=255 bytes"
        )));
    }
    if limit == 0 || limit > MAX_ARTIFACT_METADATA_PAGE {
        return Err(ArtifactError::InvalidInput(format!(
            "artifact metadata page must contain 1..={MAX_ARTIFACT_METADATA_PAGE} entries"
        )));
    }
    Ok(())
}

fn sql_limit(limit: usize) -> Result<i64, ArtifactError> {
    i64::try_from(limit)
        .map_err(|_| ArtifactError::InvalidInput("artifact metadata page limit is too large".to_owned()))
}

fn catalog_entry(
    artifact_id: String,
    media_type: String,
    created_at: i64,
) -> Result<ArtifactCatalogEntry, ArtifactError> {
    let artifact_id = artifact_id.parse::<ArtifactId>().map_err(|_| {
        ArtifactError::InvalidStoredRecord("invalid artifact id in catalog".to_owned())
    })?;
    let media_type = BoundedText::try_new(media_type).map_err(|error| {
        ArtifactError::InvalidStoredRecord(format!(
            "invalid artifact media type in catalog: {error}"
        ))
    })?;
    let created_at = UnixTimestampMicros::try_new(created_at).map_err(|error| {
        ArtifactError::InvalidStoredRecord(format!(
            "invalid artifact creation time in catalog: {error}"
        ))
    })?;
    Ok(ArtifactCatalogEntry {
        artifact_id,
        media_type,
        created_at,
    })
}

fn map_missing_blob(error: io::Error, artifact_id: ArtifactId) -> ArtifactError {
    if error.kind() == io::ErrorKind::NotFound {
        ArtifactError::MissingBlob(artifact_id)
    } else {
        ArtifactError::Io(error)
    }
}

fn hash_bytes(bytes: &[u8]) -> ContentHash {
    let digest = Sha256::digest(bytes);
    let mut hash = [0_u8; 32];
    hash.copy_from_slice(&digest);
    ContentHash::from_bytes(hash)
}

#[cfg(test)]
mod tests {
    use super::MAX_ARTIFACT_METADATA_PAGE;
    use crate::{ArtifactError, ArtifactScope, NewArtifact, StateStore};
    use intent_contracts::{ArtifactId, BoundedText, UnixTimestampMicros};
    use std::error::Error;
    use std::fs;
    use std::io::Cursor;
    use uuid::Uuid;

    #[test]
    fn metadata_listing_is_scoped_ordered_bounded_and_skips_suppressed()
    -> Result<(), Box<dyn Error>> {
        let root = std::env::temp_dir().join(format!(
            "intent-artifact-catalog-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        fs::create_dir_all(&root)?;
        let result = (|| -> Result<(), Box<dyn Error>> {
            let mut store = StateStore::open_in_memory_for_tests()?;
            let scope = ArtifactScope::try_new("profile:archive/private")?;
            let other = ArtifactScope::try_new("profile:other/private")?;
            let media = "application/vnd.intent.core-record+json;family=intent.workspace";
            let mut ids = Vec::new();
            for (target_scope, created, media_type) in [
                (&scope, 100, media),
                (&scope, 300, media),
                (&scope, 200, "application/octet-stream"),
                (&other, 400, media),
                (&scope, 250, media),
            ] {
                let id = ArtifactId::from_uuid(Uuid::new_v4());
                ids.push(id);
                store.store_artifact(
                    &root,
                    NewArtifact {
                        artifact_id: id,
                        privacy_scope: target_scope.clone(),
                        media_type: BoundedText::try_new(media_type)?,
                        created_at: UnixTimestampMicros::try_new(created)?,
                    },
                    &mut Cursor::new(format!("artifact-{created}").into_bytes()),
                )?;
            }
            store.suppress_artifact(ids[4], &scope, UnixTimestampMicros::try_new(500)?)?;
            let found = store.list_artifact_metadata_by_media_prefix(
                &scope,
                "application/vnd.intent.core-record+json;family=",
                8,
            )?;
            assert_eq!(found.len(), 2);
            assert_eq!(found[0].artifact_id(), ids[1]);
            assert_eq!(found[1].artifact_id(), ids[0]);
            let exact = store.list_artifact_metadata_by_media_type(&scope, media, 8)?;
            assert_eq!(exact.len(), 2);
            assert_eq!(exact[0].artifact_id(), ids[1]);
            assert_eq!(exact[1].artifact_id(), ids[0]);
            assert!(matches!(
                store.list_artifact_metadata_by_media_prefix(&scope, media, 0),
                Err(ArtifactError::InvalidInput(_))
            ));
            assert!(matches!(
                store.list_artifact_metadata_by_media_prefix(
                    &scope,
                    media,
                    MAX_ARTIFACT_METADATA_PAGE + 1,
                ),
                Err(ArtifactError::InvalidInput(_))
            ));
            Ok(())
        })();
        let _ = fs::remove_dir_all(&root);
        result
    }
}
