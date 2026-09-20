use super::*;
use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordValidationError {
    CollectionTooLarge,
    TimeOrder {
        earlier: &'static str,
        later: &'static str,
    },
}

impl fmt::Display for RecordValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CollectionTooLarge => write!(
                formatter,
                "record collection exceeds {MAX_RECORD_COLLECTION_ENTRIES} entries"
            ),
            Self::TimeOrder { earlier, later } => {
                write!(formatter, "{later} precedes {earlier}")
            }
        }
    }
}

impl Error for RecordValidationError {}

fn require_not_before(
    earlier: (&'static str, UnixTimestampMicros),
    later: (&'static str, UnixTimestampMicros),
) -> Result<(), RecordValidationError> {
    if later.1 < earlier.1 {
        return Err(RecordValidationError::TimeOrder {
            earlier: earlier.0,
            later: later.0,
        });
    }
    Ok(())
}

impl ApprovalState {
    pub(super) fn validate(&self) -> Result<(), RecordValidationError> {
        if let Self::Approved {
            approved_at,
            expires_at: Some(expires_at),
        } = self
        {
            require_not_before(("approved_at", *approved_at), ("expires_at", *expires_at))?;
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Workspace {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct WireWorkspace {
            #[serde(deserialize_with = "deserialize_v1_schema")]
            schema_version: SchemaVersion,
            id: WorkspaceId,
            #[serde(default)]
            goal_contract_id: Option<GoalContractId>,
            title: BoundedText<512>,
            state: WorkspaceState,
            revision: u64,
            #[serde(default)]
            active_view: Option<ViewDefinitionId>,
            created_at: UnixTimestampMicros,
            updated_at: UnixTimestampMicros,
        }
        let wire = WireWorkspace::deserialize(deserializer)?;
        require_not_before(
            ("created_at", wire.created_at),
            ("updated_at", wire.updated_at),
        )
        .map_err(serde::de::Error::custom)?;
        Ok(Self {
            schema_version: wire.schema_version,
            id: wire.id,
            goal_contract_id: wire.goal_contract_id,
            title: wire.title,
            state: wire.state,
            revision: wire.revision,
            active_view: wire.active_view,
            created_at: wire.created_at,
            updated_at: wire.updated_at,
        })
    }
}

impl<'de> Deserialize<'de> for Task {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct WireTask {
            #[serde(deserialize_with = "deserialize_v1_schema")]
            schema_version: SchemaVersion,
            id: TaskId,
            workspace_id: WorkspaceId,
            #[serde(default)]
            goal_contract_id: Option<GoalContractId>,
            state: TaskState,
            success_predicate: BoundedText<4096>,
            #[serde(default)]
            required_capabilities: RecordList<CapabilityId>,
            #[serde(default)]
            result_artifacts: RecordList<ArtifactReference>,
            #[serde(default)]
            failure_detail: Option<BoundedText<4096>>,
            created_at: UnixTimestampMicros,
            updated_at: UnixTimestampMicros,
        }
        let wire = WireTask::deserialize(deserializer)?;
        require_not_before(
            ("created_at", wire.created_at),
            ("updated_at", wire.updated_at),
        )
        .map_err(serde::de::Error::custom)?;
        Ok(Self {
            schema_version: wire.schema_version,
            id: wire.id,
            workspace_id: wire.workspace_id,
            goal_contract_id: wire.goal_contract_id,
            state: wire.state,
            success_predicate: wire.success_predicate,
            required_capabilities: wire.required_capabilities,
            result_artifacts: wire.result_artifacts,
            failure_detail: wire.failure_detail,
            created_at: wire.created_at,
            updated_at: wire.updated_at,
        })
    }
}

impl<'de> Deserialize<'de> for MemoryRecord {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct WireMemory {
            #[serde(deserialize_with = "deserialize_v1_schema")]
            schema_version: SchemaVersion,
            id: MemoryRecordId,
            scope: MemoryScope,
            kind: MemoryKind,
            value: BoundedText<16384>,
            #[serde(default)]
            provenance_evidence: Option<EvidenceId>,
            created_at: UnixTimestampMicros,
            #[serde(default)]
            expires_at: Option<UnixTimestampMicros>,
        }
        let wire = WireMemory::deserialize(deserializer)?;
        if let Some(expires_at) = wire.expires_at {
            require_not_before(("created_at", wire.created_at), ("expires_at", expires_at))
                .map_err(serde::de::Error::custom)?;
        }
        Ok(Self {
            schema_version: wire.schema_version,
            id: wire.id,
            scope: wire.scope,
            kind: wire.kind,
            value: wire.value,
            provenance_evidence: wire.provenance_evidence,
            created_at: wire.created_at,
            expires_at: wire.expires_at,
        })
    }
}

impl<'de> Deserialize<'de> for EvidenceOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
        enum WireOrigin {
            Deterministic {},
            ModelDerived {
                provider: ProviderId,
                model: BoundedText<256>,
                generated_at: UnixTimestampMicros,
            },
        }
        Ok(match WireOrigin::deserialize(deserializer)? {
            WireOrigin::Deterministic {} => Self::Deterministic,
            WireOrigin::ModelDerived {
                provider,
                model,
                generated_at,
            } => Self::ModelDerived {
                provider,
                model,
                generated_at,
            },
        })
    }
}
