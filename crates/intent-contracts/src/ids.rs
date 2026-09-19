use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

macro_rules! typed_uuid {
    ($($name:ident),+ $(,)?) => {
        $(
            #[derive(
                Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
            )]
            #[serde(transparent)]
            /// A UUID value, including nil. Identity alone never grants authority.
            pub struct $name(Uuid);

            impl $name {
                #[must_use]
                pub const fn from_uuid(value: Uuid) -> Self {
                    Self(value)
                }

                #[must_use]
                pub const fn as_uuid(self) -> Uuid {
                    self.0
                }
            }

            impl fmt::Display for $name {
                fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.0.fmt(formatter)
                }
            }

            impl FromStr for $name {
                type Err = uuid::Error;

                fn from_str(value: &str) -> Result<Self, Self::Err> {
                    Uuid::parse_str(value).map(Self)
                }
            }
        )+
    };
}

typed_uuid!(
    GoalContractId,
    WorkspaceId,
    TaskId,
    CapabilityId,
    ObservationId,
    EvidenceId,
    ActionProposalId,
    ApprovalId,
    OperationId,
    OperationAttemptId,
    OutboxMessageId,
    ReceiptId,
    ViewDefinitionId,
    MemoryRecordId,
    ArtifactId,
    AccountId,
    ConnectorId,
    RequestId,
    CancellationId,
    TraceId,
    WorkerInstanceId,
    CheckpointId,
);

#[cfg(test)]
mod tests {
    use super::{TaskId, WorkspaceId};
    use std::error::Error;
    use std::str::FromStr;

    #[test]
    fn nil_uuid_remains_a_lossless_identity_value() -> Result<(), Box<dyn Error>> {
        let nil = "00000000-0000-0000-0000-000000000000";
        let task = TaskId::from_str(nil)?;
        assert_eq!(task.to_string(), nil);
        assert_eq!(
            serde_json::from_str::<TaskId>(&serde_json::to_string(&task)?)?,
            task
        );
        Ok(())
    }

    #[test]
    fn typed_id_string_round_trip_is_lossless() -> Result<(), Box<dyn Error>> {
        let raw = "018f47f7-5a86-7c00-8000-000000000001";
        let task = TaskId::from_str(raw)?;

        assert_eq!(task.to_string(), raw);
        Ok(())
    }

    #[test]
    fn different_id_types_preserve_same_uuid_without_becoming_equal_types()
    -> Result<(), Box<dyn Error>> {
        let raw = "018f47f7-5a86-7c00-8000-000000000002";
        let task = TaskId::from_str(raw)?;
        let workspace = WorkspaceId::from_uuid(task.as_uuid());

        assert_eq!(workspace.to_string(), task.to_string());
        Ok(())
    }
}
