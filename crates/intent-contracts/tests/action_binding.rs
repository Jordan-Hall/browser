use intent_contracts::{
    ActionContext, ActionProposal, ActionProposalDescriptor, Approval, ApprovalRequirement,
    ApprovalState, ArtifactReference, BoundedText, ByteSize, CanonicalizationVersion,
    CapabilityEffectClass, ContentHash, UnixTimestampMicros,
};
use std::error::Error;

fn proposal() -> Result<ActionProposal, Box<dyn Error>> {
    let id = "018f47f7-5a86-7c00-8000-000000000401";
    Ok(ActionProposal::new(
        id.parse()?,
        id.parse()?,
        id.parse()?,
        id.parse()?,
        ActionProposalDescriptor {
            canonical_arguments: ArtifactReference::new(
                id.parse()?,
                ContentHash::from_bytes([1; 32]),
                ByteSize::from_bytes(2),
                BoundedText::try_new("application/octet-stream")?,
            ),
            context: ActionContext {
                source_revision: ContentHash::from_bytes([2; 32]),
                canonicalization: CanonicalizationVersion::ExactBytesV1,
            },
            target_resource: None,
            expires_at: Some(UnixTimestampMicros::try_new(500)?),
            effect_class: CapabilityEffectClass::IrreversibleOrUncertain,
            approval_requirement: ApprovalRequirement::Always,
        },
    ))
}

#[test]
fn proposal_and_approval_preserve_the_complete_material_binding() -> Result<(), Box<dyn Error>> {
    let proposal = proposal()?;
    let approval = Approval::new(
        "018f47f7-5a86-7c00-8000-000000000402".parse()?,
        &proposal,
        ApprovalState::Approved {
            approved_at: UnixTimestampMicros::try_new(100)?,
            expires_at: None,
        },
    );
    let decoded: ActionProposal = serde_json::from_slice(&serde_json::to_vec(&proposal)?)?;
    let decoded_approval: Approval = serde_json::from_slice(&serde_json::to_vec(&approval)?)?;
    assert_eq!(decoded, proposal);
    assert_eq!(decoded_approval.exact_binding(), decoded.binding().as_ref());
    assert_eq!(
        decoded_approval.exact_arguments_hash(),
        proposal.arguments_hash()
    );
    Ok(())
}

#[test]
fn unsupported_canonicalization_and_unknown_context_fields_are_rejected()
-> Result<(), Box<dyn Error>> {
    let value = serde_json::to_value(proposal()?)?;
    for version in [
        serde_json::json!("exact_bytes_v2"),
        serde_json::json!("json_v1"),
        serde_json::json!(null),
    ] {
        let mut changed = value.clone();
        changed["context"]["canonicalization"] = version;
        assert!(serde_json::from_value::<ActionProposal>(changed).is_err());
    }
    let mut changed = value;
    changed["context"]["ignore_source_revision"] = serde_json::json!(true);
    assert!(serde_json::from_value::<ActionProposal>(changed).is_err());
    Ok(())
}

#[test]
fn legacy_proposals_remain_explicitly_unbound_after_roundtrip() -> Result<(), Box<dyn Error>> {
    let mut legacy = serde_json::to_value(proposal()?)?;
    legacy
        .as_object_mut()
        .ok_or("proposal object")?
        .remove("context");
    let decoded: ActionProposal = serde_json::from_value(legacy.clone())?;
    assert!(decoded.binding().is_none());
    assert_eq!(serde_json::to_value(decoded)?, legacy);
    Ok(())
}

#[test]
fn approval_import_rejects_a_digest_inconsistent_with_its_binding() -> Result<(), Box<dyn Error>> {
    let proposal = proposal()?;
    let approval = Approval::new(
        "018f47f7-5a86-7c00-8000-000000000402".parse()?,
        &proposal,
        ApprovalState::Pending,
    );
    let mut wire = serde_json::to_value(approval)?;
    wire["exact_arguments_hash"] = serde_json::to_value(ContentHash::from_bytes([3; 32]))?;
    assert!(serde_json::from_value::<Approval>(wire).is_err());
    Ok(())
}
