use intent_contracts::{
    ApprovalRequirement, BoundedText, CurrencyCode, CurrencyScale, GoalContract, InferenceMode,
    KnownCurrencyScale, Money, TraceId,
};
use intent_ipc::{Envelope, WireLimits, decode_control, encode_control};
use std::error::Error;

#[test]
fn generated_money_roundtrips_through_envelopes_goals_and_checked_storage()
-> Result<(), Box<dyn Error>> {
    let trace: TraceId = "018f47f7-5a86-7c00-8000-000000000401".parse()?;
    let currency = CurrencyCode::parse("USD")?;
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
        let money = Money::new(currency, amount, scale);
        let direct = serde_json::to_vec(&money)?;
        let decoded: Money = serde_json::from_slice(&direct)?;
        assert_eq!(decoded, money);
        let envelope = Envelope::event(trace, money);
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
        .with_budget(money);
        let envelope = Envelope::event(trace, goal.clone());
        let frame = encode_control(&envelope, WireLimits::default())?;
        assert_eq!(
            decode_control::<GoalContract>(&frame, WireLimits::default())?,
            envelope
        );
        assert_eq!(
            goal.checked_budget().is_ok(),
            amount >= 0 && scale != CurrencyScale::Unknown
        );
        assert_eq!(
            decoded.try_minor_units_i64().is_ok(),
            (i128::from(i64::MIN)..=i128::from(i64::MAX)).contains(&amount)
        );
        assert_eq!(
            decoded.try_minor_units_u64().is_ok(),
            (0..=i128::from(u64::MAX)).contains(&amount)
        );
    }
    Ok(())
}
