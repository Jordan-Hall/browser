use intent_contracts::{
    ApprovalRequirement, BoundedText, CurrencyCode, CurrencyScale, GoalContract, InferenceMode,
    KnownCurrencyScale, Money, SpendAmount, SpendAmountError,
};
use std::error::Error;

#[test]
fn spend_amount_requires_nonnegative_money_and_an_explicit_scale() -> Result<(), Box<dyn Error>> {
    let currency = CurrencyCode::parse("USD")?;
    for amount in [i128::MIN, -1, 0, 1, i128::MAX] {
        for scale in [
            CurrencyScale::Unknown,
            CurrencyScale::Known(KnownCurrencyScale::try_new(2)?),
        ] {
            let money = Money::new(currency, amount, scale);
            let expected = if amount < 0 {
                Err(SpendAmountError::NegativeAmount)
            } else if scale == CurrencyScale::Unknown {
                Err(SpendAmountError::UnknownScale)
            } else {
                Ok(money)
            };
            assert_eq!(
                SpendAmount::try_from(money).map(SpendAmount::money),
                expected
            );
            let wire = serde_json::to_string(&money)?;
            let decoded = serde_json::from_str::<SpendAmount>(&wire);
            assert_eq!(decoded.is_ok(), expected.is_ok());
            assert_eq!(serde_json::from_str::<Money>(&wire)?, money);
            if let Ok(spend) = decoded {
                assert_eq!(serde_json::to_string(&spend)?, wire);
            }
        }
    }
    Ok(())
}

#[test]
fn narrowing_money_never_truncates_wraps_or_rounds() -> Result<(), Box<dyn Error>> {
    let currency = CurrencyCode::parse("USD")?;
    for amount in [
        i128::MIN,
        i128::from(i64::MIN) - 1,
        i128::from(i64::MIN),
        -1,
        0,
        (1_i128 << 53) + 1,
        i128::from(i64::MAX),
        i128::from(i64::MAX) + 1,
        i128::from(u64::MAX),
        i128::from(u64::MAX) + 1,
        i128::MAX,
    ] {
        let money = Money::new(currency, amount, CurrencyScale::Unknown);
        match money.try_minor_units_i64() {
            Ok(narrow) => assert_eq!(i128::from(narrow), amount),
            Err(_) => assert!(amount < i128::from(i64::MIN) || amount > i128::from(i64::MAX)),
        }
        match money.try_minor_units_u64() {
            Ok(narrow) => assert_eq!(i128::from(narrow), amount),
            Err(_) => assert!(amount < 0 || amount > i128::from(u64::MAX)),
        }
    }
    Ok(())
}

#[test]
fn goal_budget_remains_declarative_until_checked() -> Result<(), Box<dyn Error>> {
    let goal = GoalContract::new(
        "018f47f7-5a86-7c00-8000-000000000401".parse()?,
        BoundedText::try_new("test")?,
        InferenceMode::Offline,
        BoundedText::try_new("test")?,
        ApprovalRequirement::Always,
    );
    assert_eq!(goal.checked_budget()?, None);
    let currency = CurrencyCode::parse("USD")?;
    let unknown = Money::new(currency, 100, CurrencyScale::Unknown);
    assert_eq!(
        goal.clone().with_budget(unknown).checked_budget(),
        Err(SpendAmountError::UnknownScale)
    );
    let known = Money::new(
        currency,
        100,
        CurrencyScale::Known(KnownCurrencyScale::try_new(2)?),
    );
    assert_eq!(
        goal.clone()
            .with_budget(known)
            .checked_budget()?
            .map(SpendAmount::money),
        Some(known)
    );
    let negative = Money::new(currency, -1, known.scale());
    assert_eq!(
        goal.with_budget(negative).checked_budget(),
        Err(SpendAmountError::NegativeAmount)
    );
    Ok(())
}
