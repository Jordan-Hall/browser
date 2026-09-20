use intent_contracts::{
    ActionProposal, Approval, ApprovalState, BoundedText, Evidence, EvidenceOrigin, GoalConstraint,
    GoalContract, MAX_RECORD_COLLECTION_ENTRIES, MemoryRecord, Receipt, RecordValidationError,
    Task, UnixTimestampMicros, ViewDefinition, Workspace,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

fn fixture(family: &str) -> Result<Value, Box<dyn Error>> {
    let fixtures: Vec<Value> =
        serde_json::from_str(include_str!("../../../fixtures/core/records-v1.json"))?;
    fixtures
        .into_iter()
        .find(|fixture| fixture["family"] == family)
        .map(|fixture| fixture["full"].clone())
        .ok_or_else(|| format!("missing fixture {family}").into())
}

fn rejects_reversed_time<T: DeserializeOwned>(family: &str, later: &str) -> TestResult {
    let mut value = fixture(family)?;
    value["created_at"] = json!(100);
    value[later] = json!(99);
    assert!(serde_json::from_value::<T>(value.clone()).is_err());
    value[later] = json!(100);
    serde_json::from_value::<T>(value.clone())?;
    value["created_at"] = json!(-100);
    value[later] = json!(-99);
    serde_json::from_value::<T>(value.clone())?;
    if later == "expires_at" {
        value.as_object_mut().ok_or("record object")?.remove(later);
        serde_json::from_value::<T>(value)?;
    }
    Ok(())
}

#[test]
fn workspace_update_cannot_precede_creation() -> TestResult {
    rejects_reversed_time::<Workspace>("intent.workspace", "updated_at")
}

#[test]
fn task_update_cannot_precede_creation() -> TestResult {
    rejects_reversed_time::<Task>("intent.task", "updated_at")
}

#[test]
fn memory_expiry_cannot_precede_creation() -> TestResult {
    rejects_reversed_time::<MemoryRecord>("intent.memory_record", "expires_at")
}

#[test]
fn approval_expiry_cannot_precede_approval() -> TestResult {
    let mut value = fixture("intent.approval")?;
    value["state"] = json!({
        "state": "approved",
        "details": {"approved_at": 100, "expires_at": 99}
    });
    assert!(serde_json::from_value::<Approval>(value.clone()).is_err());
    value["state"]["details"]["expires_at"] = json!(100);
    serde_json::from_value::<Approval>(value)?;
    Ok(())
}

#[test]
fn deterministic_evidence_rejects_unknown_origin_fields() -> TestResult {
    let mut value = fixture("intent.evidence")?;
    value["origin"] = json!({"kind": "deterministic", "authority_override": true});
    assert!(serde_json::from_value::<Evidence>(value.clone()).is_err());
    value["origin"] = json!({"kind": "deterministic"});
    assert_eq!(
        serde_json::to_value(EvidenceOrigin::Deterministic)?,
        value["origin"]
    );
    serde_json::from_value::<Evidence>(value)?;
    Ok(())
}

fn collection_boundary<T: DeserializeOwned + Serialize>(family: &str, field: &str) -> TestResult {
    let mut value = fixture(family)?;
    let item = value[field][0].clone();
    assert!(!item.is_null(), "missing fixture element {family}.{field}");
    value[field] = json!(vec![item.clone(); MAX_RECORD_COLLECTION_ENTRIES]);
    let record: T = serde_json::from_value(value.clone())?;
    assert_eq!(serde_json::to_value(record)?, value);
    value[field].as_array_mut().ok_or("collection")?.push(item);
    assert!(
        serde_json::from_value::<T>(value).is_err(),
        "accepted excess element in {family}.{field}"
    );
    Ok(())
}

#[test]
fn every_record_collection_accepts_the_boundary_and_rejects_the_next_item() -> TestResult {
    collection_boundary::<GoalContract>("intent.goal_contract", "clarified_constraints")?;
    collection_boundary::<GoalContract>("intent.goal_contract", "authorized_accounts")?;
    collection_boundary::<Task>("intent.task", "required_capabilities")?;
    collection_boundary::<Task>("intent.task", "result_artifacts")?;
    collection_boundary::<Evidence>("intent.evidence", "source_observations")?;
    collection_boundary::<Receipt>("intent.receipt", "evidence_ids")?;
    collection_boundary::<ViewDefinition>("intent.view_definition", "bindings")?;
    collection_boundary::<ViewDefinition>("intent.view_definition", "action_capabilities")?;
    Ok(())
}

#[test]
fn collection_builders_cannot_bypass_record_limits() -> TestResult {
    let original: GoalContract = serde_json::from_value(fixture("intent.goal_contract")?)?;
    let mut goal = original.clone();
    let account = goal.authorized_accounts()[0];
    for _ in goal.authorized_accounts().len()..MAX_RECORD_COLLECTION_ENTRIES {
        goal = goal.with_authorized_account(account)?;
    }
    assert_eq!(
        goal.authorized_accounts().len(),
        MAX_RECORD_COLLECTION_ENTRIES
    );
    assert_eq!(
        goal.with_authorized_account(account),
        Err(RecordValidationError::CollectionTooLarge)
    );
    let mut goal = original;
    let constraint =
        GoalConstraint::new(BoundedText::try_new("region")?, BoundedText::try_new("GB")?);
    for _ in goal.constraints().len()..MAX_RECORD_COLLECTION_ENTRIES {
        goal = goal.with_constraint(constraint.clone())?;
    }
    assert_eq!(goal.constraints().len(), MAX_RECORD_COLLECTION_ENTRIES);
    assert_eq!(
        goal.with_constraint(constraint),
        Err(RecordValidationError::CollectionTooLarge)
    );
    Ok(())
}

#[test]
fn approval_constructor_and_import_agree_on_interval_validity() -> TestResult {
    let proposal: ActionProposal = serde_json::from_value(fixture("intent.action_proposal")?)?;
    let id = "018f47f7-5a86-7c00-8000-000000000099".parse()?;
    let approved_at = UnixTimestampMicros::try_new(-100)?;
    for expiry in [None, Some(-100), Some(-99), Some(-101)] {
        let state = ApprovalState::Approved {
            approved_at,
            expires_at: expiry.map(UnixTimestampMicros::try_new).transpose()?,
        };
        let constructed = Approval::new(id, &proposal, state.clone());
        let mut imported = fixture("intent.approval")?;
        imported["state"] = serde_json::to_value(state)?;
        let imported = serde_json::from_value::<Approval>(imported);
        assert_eq!(constructed.is_ok(), expiry != Some(-101));
        assert_eq!(constructed.is_ok(), imported.is_ok());
        if let Ok(approval) = constructed {
            assert_eq!(
                serde_json::from_value::<Approval>(serde_json::to_value(&approval)?)?,
                approval
            );
        }
    }
    Ok(())
}

#[test]
fn every_approval_state_rejects_unknown_details() -> TestResult {
    let original = fixture("intent.approval")?;
    for (state, timestamp) in [
        ("pending", None),
        ("approved", Some("approved_at")),
        ("rejected", Some("rejected_at")),
        ("revoked", Some("revoked_at")),
        ("expired", Some("expired_at")),
    ] {
        let mut value = original.clone();
        value["state"] = json!({"state": state});
        if let Some(timestamp) = timestamp {
            value["state"]["details"] = json!({timestamp: 100});
        }
        serde_json::from_value::<Approval>(value.clone())?;
        if timestamp.is_none() {
            value["state"]["details"] = json!({});
        }
        value["state"]["details"]["authority_override"] = json!(true);
        assert!(
            serde_json::from_value::<Approval>(value).is_err(),
            "unknown detail accepted in {state}"
        );
    }
    Ok(())
}
