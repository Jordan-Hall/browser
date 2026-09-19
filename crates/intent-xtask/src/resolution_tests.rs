use super::{Metadata, validate_contract_dependencies};
use serde_json::{Value, json};
use std::error::Error;

fn metadata() -> Value {
    json!({
        "workspace_members": ["contracts"],
        "packages": [
            {
                "id": "contracts",
                "name": "intent-contracts",
                "source": null,
                "dependencies": [{
                    "name": "serde",
                    "source": "registry+https://github.com/rust-lang/crates.io-index",
                    "path": null
                }]
            },
            {
                "id": "serde-registry",
                "name": "serde",
                "source": "registry+https://github.com/rust-lang/crates.io-index",
                "dependencies": []
            }
        ],
        "resolve": {"nodes": [{"id": "contracts", "dependencies": ["serde-registry"]}]}
    })
}

fn valid(value: Value) -> Result<bool, Box<dyn Error>> {
    let metadata: Metadata = serde_json::from_value(value)?;
    Ok(validate_contract_dependencies(&metadata).is_ok())
}

#[test]
fn accepts_reviewed_declared_and_resolved_dependency() -> Result<(), Box<dyn Error>> {
    assert!(valid(metadata())?);
    Ok(())
}

#[test]
fn rejects_incomplete_resolved_graph() -> Result<(), Box<dyn Error>> {
    let mut no_graph = metadata();
    no_graph["resolve"] = Value::Null;
    assert!(!valid(no_graph)?);
    let mut no_node = metadata();
    no_node["resolve"]["nodes"] = json!([]);
    assert!(!valid(no_node)?);
    let mut no_package = metadata();
    no_package["resolve"]["nodes"][0]["dependencies"] = json!(["missing"]);
    assert!(!valid(no_package)?);
    Ok(())
}

#[test]
fn rejects_resolved_package_or_source_substitution() -> Result<(), Box<dyn Error>> {
    for source in [
        Value::Null,
        json!("git+https://example.invalid/serde"),
        json!("registry+https://example.invalid/index"),
    ] {
        let mut value = metadata();
        value["packages"][1]["source"] = source;
        assert!(!valid(value)?);
    }
    let mut renamed = metadata();
    renamed["packages"][1]["name"] = json!("provider-sdk");
    assert!(!valid(renamed)?);
    Ok(())
}

#[test]
fn contracts_must_be_a_workspace_member() -> Result<(), Box<dyn Error>> {
    let mut value = metadata();
    value["workspace_members"] = json!(["some-other-package"]);
    assert!(!valid(value)?);
    Ok(())
}
