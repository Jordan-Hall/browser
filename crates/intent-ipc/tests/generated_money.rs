use intent_contracts::{
    ApprovalRequirement, BoundedText, CurrencyCode, CurrencyScale, GoalContract, InferenceMode,
    KnownCurrencyScale, Money, SpendAmountError, TraceId,
};
use intent_ipc::{Envelope, WireLimits, decode_control, encode_control};
use std::error::Error;

#[test]
fn generated_money_roundtrips_through_envelopes_goals_and_checked_storage()
-> Result<(), Box<dyn Error>> {
    let mut state = 0x1234_5678_9abc_def0_1357_2468_ace0_bdf1_u128;
    for iteration in 0..4096 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let amount = i128::from_le_bytes(state.to_le_bytes());
        let scale = if iteration % 2 == 0 {
            CurrencyScale::Unknown
        } else {
            CurrencyScale::Known(KnownCurrencyScale::try_new((iteration % 19) as u8)?)
        };
        assert_money_roundtrip(
            amount,
            scale,
            i64::try_from(amount).ok(),
            u64::try_from(amount).ok(),
        )?;
    }
    Ok(())
}

#[test]
fn money_boundaries_roundtrip_through_envelopes_goals_and_checked_storage()
-> Result<(), Box<dyn Error>> {
    let cases = [
        (i128::MIN, None, None),
        (i128::from(i64::MIN) - 1, None, None),
        (i128::from(i64::MIN), Some(i64::MIN), None),
        (-1, Some(-1), None),
        (0, Some(0), Some(0)),
        (
            9_007_199_254_740_993,
            Some(9_007_199_254_740_993),
            Some(9_007_199_254_740_993),
        ),
        (
            i128::from(i64::MAX),
            Some(i64::MAX),
            Some(9_223_372_036_854_775_807),
        ),
        (
            i128::from(i64::MAX) + 1,
            None,
            Some(9_223_372_036_854_775_808),
        ),
        (i128::from(u64::MAX), None, Some(u64::MAX)),
        (i128::from(u64::MAX) + 1, None, None),
        (i128::MAX, None, None),
    ];
    for scale in [
        CurrencyScale::Unknown,
        CurrencyScale::Known(KnownCurrencyScale::try_new(0)?),
        CurrencyScale::Known(KnownCurrencyScale::try_new(2)?),
        CurrencyScale::Known(KnownCurrencyScale::try_new(18)?),
    ] {
        for (amount, signed, unsigned) in cases {
            assert_money_roundtrip(amount, scale, signed, unsigned)?;
        }
    }
    Ok(())
}

fn assert_money_roundtrip(
    amount: i128,
    scale: CurrencyScale,
    signed: Option<i64>,
    unsigned: Option<u64>,
) -> Result<(), Box<dyn Error>> {
    let trace: TraceId = "018f47f7-5a86-7c00-8000-000000000401".parse()?;
    let money = Money::new(CurrencyCode::parse("USD")?, amount, scale);
    let direct = serde_json::to_vec(&money)?;
    let decoded: Money = serde_json::from_slice(&direct)?;
    assert_eq!(decoded, money);
    let envelope = Envelope::event(trace, decoded);
    let frame = encode_control(&envelope, WireLimits::default())?;
    let recovered = decode_control::<Money>(&frame, WireLimits::default())?;
    assert_eq!(recovered, envelope);
    assert_eq!(
        encode_control(&recovered, WireLimits::default())?.payload(),
        frame.payload()
    );

    let goal = GoalContract::new(
        "018f47f7-5a86-7c00-8000-000000000402".parse()?,
        BoundedText::try_new("generated money")?,
        InferenceMode::Offline,
        BoundedText::try_new("preserve declared budget")?,
        ApprovalRequirement::Always,
    )
    .with_budget(recovered.into_payload());
    let envelope = Envelope::event(trace, goal);
    let frame = encode_control(&envelope, WireLimits::default())?;
    let recovered = decode_control::<GoalContract>(&frame, WireLimits::default())?;
    assert_eq!(recovered, envelope);
    let goal = recovered.into_payload();
    let encoded_goal = serde_json::to_value(&goal)?;
    assert_eq!(encoded_goal["budget"]["minor_units"], amount.to_string());
    let budget: Money = serde_json::from_value(encoded_goal["budget"].clone())?;
    assert_eq!(budget, money);
    assert_eq!(budget.try_minor_units_i64().ok(), signed);
    assert_eq!(budget.try_minor_units_u64().ok(), unsigned);
    match (amount < 0, scale) {
        (true, _) => assert_eq!(goal.checked_budget(), Err(SpendAmountError::NegativeAmount)),
        (false, CurrencyScale::Unknown) => {
            assert_eq!(goal.checked_budget(), Err(SpendAmountError::UnknownScale));
        }
        (false, CurrencyScale::Known(_)) => {
            assert_eq!(
                goal.checked_budget()?.map(|spend| spend.money()),
                Some(money)
            );
        }
    }
    Ok(())
}
