use super::*;

impl<'de> Deserialize<'de> for ActionProposal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct WireProposal {
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

        let wire = WireProposal::deserialize(deserializer)?;
        if wire.arguments_hash != wire.canonical_arguments.content_hash() {
            return Err(serde::de::Error::custom(
                "proposal arguments_hash does not match canonical_arguments.content_hash",
            ));
        }
        Ok(Self {
            schema_version: wire.schema_version,
            id: wire.id,
            task_id: wire.task_id,
            capability_id: wire.capability_id,
            account_id: wire.account_id,
            target_resource: wire.target_resource,
            canonical_arguments: wire.canonical_arguments,
            arguments_hash: wire.arguments_hash,
            effect_class: wire.effect_class,
            expires_at: wire.expires_at,
            approval_requirement: wire.approval_requirement,
        })
    }
}
