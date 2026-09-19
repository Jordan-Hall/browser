use intent_contracts::{
    AccountQualifiedResourceId, BoundedText, ProviderAccountId, ProviderId, ProviderResourceId,
};
use serde_json::json;
use std::error::Error;

#[test]
fn empty_provider_identifiers_are_rejected_at_every_entry_point() {
    assert!(ProviderId::try_new("").is_err());
    assert!(ProviderAccountId::try_new("").is_err());
    assert!(ProviderResourceId::try_new("").is_err());
    assert!("".parse::<ProviderId>().is_err());
    assert!("".parse::<ProviderAccountId>().is_err());
    assert!("".parse::<ProviderResourceId>().is_err());
    assert!(serde_json::from_str::<ProviderId>("\"\"").is_err());
    assert!(serde_json::from_value::<ProviderId>(json!("")).is_err());
    assert!(serde_json::from_value::<ProviderAccountId>(json!("")).is_err());
    assert!(serde_json::from_value::<ProviderResourceId>(json!("")).is_err());
    for field in ["provider", "account", "resource"] {
        let mut value = json!({"provider": "service", "account": "owner", "resource": "item"});
        value[field] = json!("");
        assert!(serde_json::from_value::<AccountQualifiedResourceId>(value).is_err());
    }
    assert!(BoundedText::<128>::try_new("").is_ok());
}

#[test]
fn opaque_identifiers_preserve_case_whitespace_and_unicode() -> Result<(), Box<dyn Error>> {
    for value in [
        "Account",
        "account",
        " account ",
        " ",
        "\u{00e9}",
        "e\u{0301}",
        "\u{1f680}",
    ] {
        let provider = ProviderId::try_new(value)?;
        let account = ProviderAccountId::try_new(value)?;
        let resource = ProviderResourceId::try_new(value)?;
        assert_eq!(provider.as_str(), value);
        assert_eq!(account.as_str(), value);
        assert_eq!(resource.as_str(), value);
        assert_eq!(serde_json::to_value(&provider)?, json!(value));
        assert_eq!(
            serde_json::from_value::<ProviderId>(json!(value))?,
            provider
        );
        assert_eq!(
            serde_json::from_value::<ProviderAccountId>(json!(value))?,
            account
        );
        assert_eq!(
            serde_json::from_value::<ProviderResourceId>(json!(value))?,
            resource
        );
    }
    Ok(())
}

#[test]
fn provider_identifier_limits_count_utf8_bytes() -> Result<(), Box<dyn Error>> {
    for length in 0..=513 {
        let value = "x".repeat(length);
        let wire = serde_json::to_string(&value)?;
        assert_eq!(
            ProviderId::try_new(&value).is_ok(),
            (1..=128).contains(&length)
        );
        assert_eq!(
            ProviderAccountId::try_new(&value).is_ok(),
            (1..=256).contains(&length)
        );
        assert_eq!(
            ProviderResourceId::try_new(&value).is_ok(),
            (1..=512).contains(&length)
        );
        assert_eq!(
            serde_json::from_str::<ProviderId>(&wire).is_ok(),
            (1..=128).contains(&length)
        );
        assert_eq!(
            serde_json::from_str::<ProviderAccountId>(&wire).is_ok(),
            (1..=256).contains(&length)
        );
        assert_eq!(
            serde_json::from_str::<ProviderResourceId>(&wire).is_ok(),
            (1..=512).contains(&length)
        );
    }
    assert!(ProviderId::try_new("\u{00e9}".repeat(64)).is_ok());
    assert!(ProviderId::try_new("\u{00e9}".repeat(65)).is_err());
    Ok(())
}

#[test]
fn resource_identity_includes_both_provider_and_account() -> Result<(), Box<dyn Error>> {
    let original = AccountQualifiedResourceId::new(
        ProviderId::try_new("service")?,
        ProviderAccountId::try_new("first")?,
        ProviderResourceId::try_new("same-resource")?,
    );
    for (provider, account) in [("service", "second"), ("other", "first")] {
        let other = AccountQualifiedResourceId::new(
            ProviderId::try_new(provider)?,
            ProviderAccountId::try_new(account)?,
            ProviderResourceId::try_new("same-resource")?,
        );
        assert_ne!(original, other);
    }
    Ok(())
}
