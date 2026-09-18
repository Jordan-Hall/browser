use crate::ids::{
    AccountId, ActionProposalId, ApprovalId, ArtifactId, CapabilityId, ConnectorId, EvidenceId,
    GoalContractId, MemoryRecordId, ObservationId, OperationId, ReceiptId, TaskId,
    ViewDefinitionId, WorkspaceId,
};
use crate::values::{
    AccountQualifiedResourceId, BoundedText, ByteSize, ContentHash, Money, ProviderId,
    UnixTimestampMicros,
};
use crate::version::{SchemaVersion, deserialize_v1_schema};
use serde::{Deserialize, Serialize};

mod proposal_validation;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactReference {
    #[serde(deserialize_with = "deserialize_v1_schema")]
    schema_version: SchemaVersion,
    id: ArtifactId,
    content_hash: ContentHash,
    byte_size: ByteSize,
    media_type: BoundedText<128>,
}

impl ArtifactReference {
    #[must_use]
    pub const fn new(
        id: ArtifactId,
        content_hash: ContentHash,
        byte_size: ByteSize,
        media_type: BoundedText<128>,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            id,
            content_hash,
            byte_size,
            media_type,
        }
    }

    #[must_use]
    pub const fn artifact_id(&self) -> ArtifactId {
        self.id
    }

    #[must_use]
    pub const fn content_hash(&self) -> ContentHash {
        self.content_hash
    }

    #[must_use]
    pub const fn byte_size(&self) -> ByteSize {
        self.byte_size
    }

    #[must_use]
    pub fn media_type(&self) -> &str {
        self.media_type.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoalConstraint {
    key: BoundedText<128>,
    value: BoundedText<4096>,
}

impl GoalConstraint {
    #[must_use]
    pub const fn new(key: BoundedText<128>, value: BoundedText<4096>) -> Self {
        Self { key, value }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InferenceMode {
    Offline,
    LocalInferenceOnlineConnectors,
    Hybrid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRequirement {
    None,
    RiskBased,
    Always,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoalContract {
    #[serde(deserialize_with = "deserialize_v1_schema")]
    schema_version: SchemaVersion,
    id: GoalContractId,
    original_request: BoundedText<16384>,
    #[serde(default)]
    clarified_constraints: Vec<GoalConstraint>,
    #[serde(default)]
    authorized_accounts: Vec<AccountId>,
    inference_mode: InferenceMode,
    #[serde(default)]
    budget: Option<Money>,
    #[serde(default)]
    deadline: Option<UnixTimestampMicros>,
    #[serde(default)]
    desired_representation: Option<BoundedText<4096>>,
    success_predicate: BoundedText<4096>,
    approval_requirement: ApprovalRequirement,
}

impl GoalContract {
    #[must_use]
    pub const fn goal_contract_id(&self) -> GoalContractId {
        self.id
    }

    #[must_use]
    pub fn new(
        id: GoalContractId,
        original_request: BoundedText<16384>,
        inference_mode: InferenceMode,
        success_predicate: BoundedText<4096>,
        approval_requirement: ApprovalRequirement,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            id,
            original_request,
            clarified_constraints: Vec::new(),
            authorized_accounts: Vec::new(),
            inference_mode,
            budget: None,
            deadline: None,
            desired_representation: None,
            success_predicate,
            approval_requirement,
        }
    }

    #[must_use]
    pub fn with_constraint(mut self, constraint: GoalConstraint) -> Self {
        self.clarified_constraints.push(constraint);
        self
    }

    #[must_use]
    pub fn with_authorized_account(mut self, account: AccountId) -> Self {
        self.authorized_accounts.push(account);
        self
    }

    #[must_use]
    pub const fn with_budget(mut self, budget: Money) -> Self {
        self.budget = Some(budget);
        self
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: UnixTimestampMicros) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub fn with_desired_representation(mut self, representation: BoundedText<4096>) -> Self {
        self.desired_representation = Some(representation);
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceState {
    Active,
    Archived,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Workspace {
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

impl Workspace {
    #[must_use]
    pub const fn workspace_id(&self) -> WorkspaceId {
        self.id
    }

    #[must_use]
    pub const fn goal_contract_id(&self) -> Option<GoalContractId> {
        self.goal_contract_id
    }

    #[must_use]
    pub const fn new(
        id: WorkspaceId,
        title: BoundedText<512>,
        created_at: UnixTimestampMicros,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            id,
            goal_contract_id: None,
            title,
            state: WorkspaceState::Active,
            revision: 0,
            active_view: None,
            created_at,
            updated_at: created_at,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskState {
    Draft,
    Scoped,
    Planned,
    Running,
    AwaitingApproval,
    Verifying,
    Completed,
    Suspended,
    Cancelled,
    Failed,
    Blocked,
    NeedsReconciliation,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
    #[serde(deserialize_with = "deserialize_v1_schema")]
    schema_version: SchemaVersion,
    id: TaskId,
    workspace_id: WorkspaceId,
    #[serde(default)]
    goal_contract_id: Option<GoalContractId>,
    state: TaskState,
    success_predicate: BoundedText<4096>,
    #[serde(default)]
    required_capabilities: Vec<CapabilityId>,
    #[serde(default)]
    result_artifacts: Vec<ArtifactReference>,
    #[serde(default)]
    failure_detail: Option<BoundedText<4096>>,
    created_at: UnixTimestampMicros,
    updated_at: UnixTimestampMicros,
}

impl Task {
    #[must_use]
    pub const fn task_id(&self) -> TaskId {
        self.id
    }

    #[must_use]
    pub const fn workspace_id(&self) -> WorkspaceId {
        self.workspace_id
    }

    #[must_use]
    pub const fn goal_contract_id(&self) -> Option<GoalContractId> {
        self.goal_contract_id
    }

    #[must_use]
    pub fn result_artifacts(&self) -> &[ArtifactReference] {
        &self.result_artifacts
    }

    #[must_use]
    pub fn required_capabilities(&self) -> &[CapabilityId] {
        &self.required_capabilities
    }

    #[must_use]
    pub const fn new(
        id: TaskId,
        workspace_id: WorkspaceId,
        success_predicate: BoundedText<4096>,
        created_at: UnixTimestampMicros,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            id,
            workspace_id,
            goal_contract_id: None,
            state: TaskState::Draft,
            success_predicate,
            required_capabilities: Vec::new(),
            result_artifacts: Vec::new(),
            failure_detail: None,
            created_at,
            updated_at: created_at,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityEffectClass {
    ReadOnly,
    LocalReversible,
    ExternalCompensatable,
    IrreversibleOrUncertain,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilitySupportLevel {
    Prototype,
    Sandbox,
    Supervised,
    Production,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityDescriptor {
    pub name: BoundedText<256>,
    pub effect_class: CapabilityEffectClass,
    pub resource_scope: BoundedText<1024>,
    pub input_schema_hash: ContentHash,
    pub output_schema_hash: ContentHash,
    pub support_level: CapabilitySupportLevel,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capability {
    #[serde(deserialize_with = "deserialize_v1_schema")]
    schema_version: SchemaVersion,
    id: CapabilityId,
    connector_id: ConnectorId,
    account_id: AccountId,
    name: BoundedText<256>,
    effect_class: CapabilityEffectClass,
    resource_scope: BoundedText<1024>,
    input_schema_hash: ContentHash,
    output_schema_hash: ContentHash,
    support_level: CapabilitySupportLevel,
}

impl Capability {
    #[must_use]
    pub fn new(
        id: CapabilityId,
        connector_id: ConnectorId,
        account_id: AccountId,
        descriptor: CapabilityDescriptor,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            id,
            connector_id,
            account_id,
            name: descriptor.name,
            effect_class: descriptor.effect_class,
            resource_scope: descriptor.resource_scope,
            input_schema_hash: descriptor.input_schema_hash,
            output_schema_hash: descriptor.output_schema_hash,
            support_level: descriptor.support_level,
        }
    }

    #[must_use]
    pub const fn account_id(&self) -> AccountId {
        self.account_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationKind {
    Source,
    UserOverlay,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    #[serde(deserialize_with = "deserialize_v1_schema")]
    schema_version: SchemaVersion,
    id: ObservationId,
    account_id: AccountId,
    source: AccountQualifiedResourceId,
    kind: ObservationKind,
    observed_at: UnixTimestampMicros,
    #[serde(default)]
    source_revision: Option<BoundedText<256>>,
    payload: ArtifactReference,
}

impl Observation {
    #[must_use]
    pub const fn new(
        id: ObservationId,
        account_id: AccountId,
        source: AccountQualifiedResourceId,
        kind: ObservationKind,
        observed_at: UnixTimestampMicros,
        payload: ArtifactReference,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            id,
            account_id,
            source,
            kind,
            observed_at,
            source_revision: None,
            payload,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceRelation {
    Supports,
    Contradicts,
    Mixed,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EvidenceOrigin {
    Deterministic,
    ModelDerived {
        provider: ProviderId,
        model: BoundedText<256>,
        generated_at: UnixTimestampMicros,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    #[serde(deserialize_with = "deserialize_v1_schema")]
    schema_version: SchemaVersion,
    id: EvidenceId,
    workspace_id: WorkspaceId,
    claim: BoundedText<8192>,
    #[serde(default)]
    source_observations: Vec<ObservationId>,
    relation: EvidenceRelation,
    origin: EvidenceOrigin,
}

impl Evidence {
    #[must_use]
    pub fn new(
        id: EvidenceId,
        workspace_id: WorkspaceId,
        claim: BoundedText<8192>,
        relation: EvidenceRelation,
        origin: EvidenceOrigin,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            id,
            workspace_id,
            claim,
            source_observations: Vec::new(),
            relation,
            origin,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionProposalDescriptor {
    pub canonical_arguments: ArtifactReference,
    pub effect_class: CapabilityEffectClass,
    pub approval_requirement: ApprovalRequirement,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ActionProposal {
    #[serde(deserialize_with = "deserialize_v1_schema")]
    schema_version: SchemaVersion,
    id: ActionProposalId,
    task_id: TaskId,
    capability_id: CapabilityId,
    account_id: AccountId,
    #[serde(default)]
    target_resource: Option<AccountQualifiedResourceId>,
    canonical_arguments: ArtifactReference,
    arguments_hash: ContentHash,
    effect_class: CapabilityEffectClass,
    #[serde(default)]
    expires_at: Option<UnixTimestampMicros>,
    approval_requirement: ApprovalRequirement,
}

impl ActionProposal {
    #[must_use]
    pub fn new(
        id: ActionProposalId,
        task_id: TaskId,
        capability_id: CapabilityId,
        account_id: AccountId,
        descriptor: ActionProposalDescriptor,
    ) -> Self {
        let arguments_hash = descriptor.canonical_arguments.content_hash();
        Self {
            schema_version: SchemaVersion::V1,
            id,
            task_id,
            capability_id,
            account_id,
            target_resource: None,
            canonical_arguments: descriptor.canonical_arguments,
            arguments_hash,
            effect_class: descriptor.effect_class,
            expires_at: None,
            approval_requirement: descriptor.approval_requirement,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    content = "details",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ApprovalState {
    Pending,
    Approved {
        approved_at: UnixTimestampMicros,
        #[serde(default)]
        expires_at: Option<UnixTimestampMicros>,
    },
    Rejected {
        rejected_at: UnixTimestampMicros,
    },
    Revoked {
        revoked_at: UnixTimestampMicros,
    },
    Expired {
        expired_at: UnixTimestampMicros,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Approval {
    #[serde(deserialize_with = "deserialize_v1_schema")]
    schema_version: SchemaVersion,
    id: ApprovalId,
    action_proposal_id: ActionProposalId,
    exact_arguments_hash: ContentHash,
    state: ApprovalState,
}

impl Approval {
    #[must_use]
    pub const fn new(
        id: ApprovalId,
        action_proposal_id: ActionProposalId,
        exact_arguments_hash: ContentHash,
        state: ApprovalState,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            id,
            action_proposal_id,
            exact_arguments_hash,
            state,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    content = "details",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum OperationState {
    Prepared,
    Dispatching,
    NeedsReconciliation {
        reason: BoundedText<2048>,
        #[serde(default)]
        last_checked_at: Option<UnixTimestampMicros>,
    },
    Succeeded {
        receipt_id: ReceiptId,
    },
    Failed {
        reason: BoundedText<2048>,
    },
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Operation {
    #[serde(deserialize_with = "deserialize_v1_schema")]
    schema_version: SchemaVersion,
    id: OperationId,
    action_proposal_id: ActionProposalId,
    account_id: AccountId,
    idempotency_key: BoundedText<256>,
    state: OperationState,
    #[serde(default)]
    last_reconciled_at: Option<UnixTimestampMicros>,
}

impl Operation {
    #[must_use]
    pub const fn new(
        id: OperationId,
        action_proposal_id: ActionProposalId,
        account_id: AccountId,
        idempotency_key: BoundedText<256>,
        state: OperationState,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            id,
            action_proposal_id,
            account_id,
            idempotency_key,
            state,
            last_reconciled_at: None,
        }
    }

    #[must_use]
    pub const fn state(&self) -> &OperationState {
        &self.state
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptOutcome {
    Succeeded,
    Compensated,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    #[serde(deserialize_with = "deserialize_v1_schema")]
    schema_version: SchemaVersion,
    id: ReceiptId,
    operation_id: OperationId,
    account_id: AccountId,
    provider_receipt_id: BoundedText<512>,
    outcome: ReceiptOutcome,
    verified_at: UnixTimestampMicros,
    #[serde(default)]
    evidence_ids: Vec<EvidenceId>,
}

impl Receipt {
    #[must_use]
    pub fn new(
        id: ReceiptId,
        operation_id: OperationId,
        account_id: AccountId,
        provider_receipt_id: BoundedText<512>,
        outcome: ReceiptOutcome,
        verified_at: UnixTimestampMicros,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            id,
            operation_id,
            account_id,
            provider_receipt_id,
            outcome,
            verified_at,
            evidence_ids: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ViewBinding {
    key: BoundedText<128>,
    artifact: ArtifactReference,
}

impl ViewBinding {
    #[must_use]
    pub const fn new(key: BoundedText<128>, artifact: ArtifactReference) -> Self {
        Self { key, artifact }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ViewDefinition {
    #[serde(deserialize_with = "deserialize_v1_schema")]
    schema_version: SchemaVersion,
    id: ViewDefinitionId,
    workspace_id: WorkspaceId,
    revision: u64,
    layout: ArtifactReference,
    #[serde(default)]
    bindings: Vec<ViewBinding>,
    #[serde(default)]
    action_capabilities: Vec<CapabilityId>,
}

impl ViewDefinition {
    #[must_use]
    pub fn new(
        id: ViewDefinitionId,
        workspace_id: WorkspaceId,
        revision: u64,
        layout: ArtifactReference,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            id,
            workspace_id,
            revision,
            layout,
            bindings: Vec::new(),
            action_capabilities: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "scope",
    content = "id",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum MemoryScope {
    Global,
    Workspace(WorkspaceId),
    Account(AccountId),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryKind {
    ExplicitPreference,
    UserCorrection,
    ApprovedPattern,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryRecord {
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

impl MemoryRecord {
    #[must_use]
    pub const fn new(
        id: MemoryRecordId,
        scope: MemoryScope,
        kind: MemoryKind,
        value: BoundedText<16384>,
        created_at: UnixTimestampMicros,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            id,
            scope,
            kind,
            value,
            provenance_evidence: None,
            created_at,
            expires_at: None,
        }
    }
}

macro_rules! impl_record_identity {
    ($record:ty, $id:ty) => {
        impl $record {
            #[must_use]
            pub const fn schema_version(&self) -> SchemaVersion {
                self.schema_version
            }

            #[must_use]
            pub const fn id(&self) -> $id {
                self.id
            }
        }
    };
}

impl_record_identity!(ArtifactReference, ArtifactId);
impl_record_identity!(GoalContract, GoalContractId);
impl_record_identity!(Workspace, WorkspaceId);
impl_record_identity!(Task, TaskId);
impl_record_identity!(Capability, CapabilityId);
impl_record_identity!(Observation, ObservationId);
impl_record_identity!(Evidence, EvidenceId);
impl_record_identity!(ActionProposal, ActionProposalId);
impl_record_identity!(Approval, ApprovalId);
impl_record_identity!(Operation, OperationId);
impl_record_identity!(Receipt, ReceiptId);
impl_record_identity!(ViewDefinition, ViewDefinitionId);
impl_record_identity!(MemoryRecord, MemoryRecordId);

#[cfg(test)]
mod tests {
    use super::{Capability, Operation, OperationState, Workspace};
    use crate::ids::{AccountId, CapabilityId};
    use std::error::Error;

    const ACCOUNT_A: &str = "018f47f7-5a86-7c00-8000-0000000000a1";
    const ACCOUNT_B: &str = "018f47f7-5a86-7c00-8000-0000000000b2";

    #[test]
    fn workspace_fixture_accepts_missing_optional_fields_and_round_trips()
    -> Result<(), Box<dyn Error>> {
        let fixture = r#"{
            "schema_version":{"major":1,"minor":0},
            "id":"018f47f7-5a86-7c00-8000-000000000101",
            "title":"Research workspace",
            "state":"active",
            "revision":1,
            "created_at":0,
            "updated_at":0
        }"#;
        let workspace: Workspace = serde_json::from_str(fixture)?;
        let encoded = serde_json::to_string(&workspace)?;
        let decoded: Workspace = serde_json::from_str(&encoded)?;
        assert_eq!(decoded, workspace);
        Ok(())
    }

    #[test]
    fn unsupported_record_schema_is_rejected() {
        let fixture = r#"{
            "schema_version":{"major":2,"minor":0},
            "id":"018f47f7-5a86-7c00-8000-000000000102",
            "title":"Future workspace",
            "state":"active",
            "revision":0,
            "created_at":0,
            "updated_at":0
        }"#;
        assert!(serde_json::from_str::<Workspace>(fixture).is_err());
    }

    #[test]
    fn unresolved_external_outcome_is_preserved() -> Result<(), Box<dyn Error>> {
        let fixture = format!(
            r#"{{
                "schema_version":{{"major":1,"minor":0}},
                "id":"018f47f7-5a86-7c00-8000-000000000201",
                "action_proposal_id":"018f47f7-5a86-7c00-8000-000000000202",
                "account_id":"{ACCOUNT_A}",
                "idempotency_key":"merchant-order-42",
                "state":{{"state":"needs_reconciliation","details":{{"reason":"timeout_after_dispatch"}}}}
            }}"#
        );
        let operation: Operation = serde_json::from_str(&fixture)?;
        assert!(matches!(
            operation.state(),
            OperationState::NeedsReconciliation { .. }
        ));
        Ok(())
    }

    #[test]
    fn capability_account_scope_remains_distinct() -> Result<(), Box<dyn Error>> {
        fn fixture(id: &str, account: &str) -> String {
            format!(
                r#"{{
                    "schema_version":{{"major":1,"minor":0}},
                    "id":"{id}",
                    "connector_id":"018f47f7-5a86-7c00-8000-000000000301",
                    "account_id":"{account}",
                    "name":"orders.read",
                    "effect_class":"read_only",
                    "resource_scope":"orders/*",
                    "input_schema_hash":"0000000000000000000000000000000000000000000000000000000000000000",
                    "output_schema_hash":"1111111111111111111111111111111111111111111111111111111111111111",
                    "support_level":"production"
                }}"#
            )
        }

        let first: Capability =
            serde_json::from_str(&fixture("018f47f7-5a86-7c00-8000-000000000302", ACCOUNT_A))?;
        let second: Capability =
            serde_json::from_str(&fixture("018f47f7-5a86-7c00-8000-000000000303", ACCOUNT_B))?;

        assert_ne!(first.account_id(), second.account_id());
        assert_eq!(
            first.account_id(),
            serde_json::from_str::<AccountId>(&format!("\"{ACCOUNT_A}\""))?
        );
        assert_ne!(first.id(), CapabilityId::from_uuid(second.id().as_uuid()));
        Ok(())
    }
}
