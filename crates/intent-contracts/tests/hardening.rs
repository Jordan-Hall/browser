use intent_contracts::{
    ActionProposal, ActionProposalDescriptor, ApprovalRequirement, ArtifactReference, BoundedText,
    ByteSize, CapabilityEffectClass, ContentHash, CurrencyCode, CurrencyScale, KnownCurrencyScale,
    Money,
};
use serde_json::{Value, json};
use std::error::Error;

#[test]
fn every_money_boundary_survives_json_value_roundtrips() -> Result<(), Box<dyn Error>> {
    let scales = [
        CurrencyScale::Unknown,
        CurrencyScale::Known(KnownCurrencyScale::try_new(2)?),
    ];
    for scale in scales {
        for amount in [
            i128::MIN,
            -1,
            0,
            1,
            (1_i128 << 53) - 1,
            1_i128 << 53,
            (1_i128 << 53) + 1,
            i128::from(u64::MAX) + 1,
            i128::MAX,
        ] {
            let money = Money::new(CurrencyCode::parse("USD")?, amount, scale);
            let value = serde_json::to_value(money)?;
            assert_eq!(value["minor_units"], Value::String(amount.to_string()));
            let bytes = serde_json::to_vec(&value)?;
            let intermediate: Value = serde_json::from_slice(&bytes)?;
            let decoded: Money = serde_json::from_value(intermediate)?;
            assert_eq!(decoded, money);
            assert_eq!(serde_json::to_vec(&decoded)?, serde_json::to_vec(&money)?);
        }
    }
    Ok(())
}

#[test]
fn money_rejects_ambiguous_spellings_and_numeric_json() -> Result<(), Box<dyn Error>> {
    let money = Money::new(CurrencyCode::parse("USD")?, 0, CurrencyScale::Unknown);
    for invalid in [
        "",
        "+1",
        "01",
        "-0",
        " 1",
        "1 ",
        "1.0",
        "1e3",
        "170141183460469231731687303715884105728",
        "-170141183460469231731687303715884105729",
    ] {
        let mut value = serde_json::to_value(money)?;
        value["minor_units"] = json!(invalid);
        assert!(serde_json::from_value::<Money>(value).is_err());
    }
    for invalid in [json!(1), json!(null), json!(true), json!([])] {
        let mut value = serde_json::to_value(money)?;
        value["minor_units"] = invalid;
        assert!(serde_json::from_value::<Money>(value).is_err());
    }
    Ok(())
}

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
                BoundedText::try_new("application/json")?,
            ),
            effect_class: CapabilityEffectClass::IrreversibleOrUncertain,
            approval_requirement: ApprovalRequirement::Always,
        },
    ))
}

#[test]
fn proposal_construction_and_wire_enforce_the_same_hash_binding() -> Result<(), Box<dyn Error>> {
    let proposal = proposal()?;
    let value = serde_json::to_value(&proposal)?;
    assert_eq!(
        value["arguments_hash"],
        value["canonical_arguments"]["content_hash"]
    );
    assert_eq!(
        serde_json::from_value::<ActionProposal>(value.clone())?,
        proposal
    );
    let mut mismatched = value.clone();
    mismatched["arguments_hash"] = json!("00".repeat(32));
    assert!(serde_json::from_value::<ActionProposal>(mismatched).is_err());
    let mut unknown = value;
    unknown["destination_override"] = json!("unsupported restriction");
    assert!(serde_json::from_value::<ActionProposal>(unknown).is_err());
    Ok(())
}

#[test]
fn borrowed_and_escaped_text_obey_utf8_byte_limits() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        serde_json::from_str::<BoundedText<2>>(r#""é""#)?.as_str(),
        "é"
    );
    assert_eq!(
        serde_json::from_str::<BoundedText<2>>(r#""\u00e9""#)?.as_str(),
        "é"
    );
    assert!(serde_json::from_str::<BoundedText<1>>(r#""é""#).is_err());
    assert!(serde_json::from_str::<BoundedText<1>>(r#""\u00e9""#).is_err());
    assert_eq!(
        serde_json::from_str::<BoundedText<0>>(r#""""#)?.as_str(),
        ""
    );
    Ok(())
}
