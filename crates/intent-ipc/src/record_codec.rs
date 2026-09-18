use crate::{WireError, WireErrorCode, WireLimits};
use intent_contracts::{
    ActionProposal, Approval, ArtifactReference, BoundedTextError, Capability, Evidence,
    GoalContract, MemoryRecord, MigrationFailure, Observation, Operation, Receipt, RecordFamily,
    SchemaValidatorFn, SchemaVersion, Task, ViewDefinition, Workspace,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

macro_rules! core_records {
    ($($record:ident => $family:literal),+ $(,)?) => {
        /// Closed catalogue of implemented v1 record codecs. A family is a schema
        /// identity, not a capability or a grant.
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
        pub enum CoreRecordKind {
            $(#[serde(rename = $family)] $record),+
        }

        /// Typed imported data. It cannot be deserialized without selecting a
        /// record codec, and it carries no permission to execute an action.
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum CoreRecord {
            $($record(Box<$record>)),+
        }

        impl CoreRecordKind {
            pub const ALL: &'static [Self] = &[$(Self::$record),+];

            #[must_use]
            pub const fn family_name(self) -> &'static str {
                match self { $(Self::$record => $family),+ }
            }

            pub fn record_family(self) -> Result<RecordFamily, BoundedTextError> {
                RecordFamily::try_new(self.family_name())
            }

            /// The validator uses the default control-document budgets and the
            /// exact v1 type, including nested schema/unknown-field checks.
            #[must_use]
            pub const fn validator(self) -> SchemaValidatorFn {
                match self { $(Self::$record => validate::<$record>),+ }
            }
        }

        impl CoreRecord {
            #[must_use]
            pub const fn kind(&self) -> CoreRecordKind {
                match self { $(Self::$record(_) => CoreRecordKind::$record),+ }
            }

            pub fn encode(&self, limits: WireLimits) -> Result<Vec<u8>, WireError> {
                match self {
                    $(Self::$record(record) => crate::bounded_json::encode(
                        record.as_ref(), limits.max_control_frame_bytes,
                    )),+
                }
            }
        }

        /// Decode a complete record, rejecting its version before interpreting
        /// fields under the current schema. Duplicates and trailing JSON fail.
        pub fn decode_core_record(
            kind: CoreRecordKind,
            input: &[u8],
            limits: WireLimits,
        ) -> Result<CoreRecord, WireError> {
            match kind {
                $(CoreRecordKind::$record => decode_typed::<$record>(input, limits)
                    .map(Box::new).map(CoreRecord::$record)),+
            }
        }
    };
}

core_records! {
    GoalContract => "intent.goal_contract",
    Workspace => "intent.workspace",
    Task => "intent.task",
    Capability => "intent.capability",
    Observation => "intent.observation",
    Evidence => "intent.evidence",
    ActionProposal => "intent.action_proposal",
    Approval => "intent.approval",
    Operation => "intent.operation",
    Receipt => "intent.receipt",
    ViewDefinition => "intent.view_definition",
    MemoryRecord => "intent.memory_record",
    ArtifactReference => "intent.artifact_reference",
}

fn decode_typed<T: DeserializeOwned>(input: &[u8], limits: WireLimits) -> Result<T, WireError> {
    if input.len() > limits.max_control_frame_bytes {
        return Err(WireError::new(
            WireErrorCode::FrameTooLarge,
            "record exceeds the control-document byte limit",
        ));
    }
    crate::envelope::preflight_json_structure(input, limits.max_json_depth)?;
    let value = crate::strict_json::decode(input, limits)?;
    let header = value.get("schema_version").ok_or_else(|| {
        WireError::new(
            WireErrorCode::InvalidRecord,
            "record schema_version is required",
        )
    })?;
    let version: SchemaVersion = serde_json::from_value(header.clone()).map_err(|_| {
        WireError::new(
            WireErrorCode::InvalidRecord,
            "invalid record schema_version",
        )
    })?;
    if version != SchemaVersion::V1 {
        return Err(WireError::new(
            WireErrorCode::UnsupportedSchema,
            "record codec implements schema 1.0 only",
        ));
    }
    serde_json::from_value(value).map_err(|_| {
        WireError::new(
            WireErrorCode::InvalidRecord,
            "record does not satisfy its v1 schema",
        )
    })
}

fn validate<T: DeserializeOwned>(input: &[u8]) -> Result<(), MigrationFailure> {
    decode_typed::<T>(input, WireLimits::default())
        .map(|_| ())
        .map_err(|error| MigrationFailure::new(error.code().wire_name()))
}
