use intent_contracts::{
    AccountId, AccountQualifiedResourceId, BoundedText, ByteSize, ContentHash, CurrencyCode,
    KnownCurrencyScale, MAX_CURRENCY_SCALE, MAX_UNIX_TIMESTAMP_MICROS, MIN_UNIX_TIMESTAMP_MICROS,
    ProfileId, ProviderAccountId, ProviderId, ProviderResourceId, TaskId, UnixTimestampMicros,
    WorkspaceId,
};
use intent_ipc::{Envelope, WireLimits, decode_control, encode_control};
use serde::{Serialize, de::DeserializeOwned};
use std::error::Error;
use std::fmt::Debug;

fn roundtrip<T>(value: T) -> Result<T, Box<dyn Error>>
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let direct = serde_json::to_vec(&value)?;
    let decoded: T = serde_json::from_slice(&direct)?;
    assert_eq!(decoded, value);
    assert_eq!(serde_json::to_vec(&decoded)?, direct);
    let envelope = Envelope::event("018f47f7-5a86-7c00-8000-000000000501".parse()?, value);
    let frame = encode_control(&envelope, WireLimits::default())?;
    let decoded = decode_control::<T>(&frame, WireLimits::default())?;
    assert_eq!(decoded, envelope);
    assert_eq!(encode_control(&decoded, WireLimits::default())?, frame);
    Ok(serde_json::from_slice(&direct)?)
}

fn reject<T: DeserializeOwned>(json: &str) -> Result<(), Box<dyn Error>> {
    assert!(serde_json::from_str::<T>(json).is_err(), "accepted {json}");
    let value: serde_json::Value = serde_json::from_str(json)?;
    let envelope = Envelope::event("018f47f7-5a86-7c00-8000-000000000501".parse()?, value);
    let frame = encode_control(&envelope, WireLimits::default())?;
    assert!(
        decode_control::<T>(&frame, WireLimits::default()).is_err(),
        "accepted {json}"
    );
    Ok(())
}

struct Samples(u64);

impl Samples {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

#[test]
fn generated_calendar_values_and_size_narrowing_preserve_exact_integers()
-> Result<(), Box<dyn Error>> {
    let mut samples = Samples(0x8e63_a924_67d1_f503);
    let span = u64::try_from(MAX_UNIX_TIMESTAMP_MICROS - MIN_UNIX_TIMESTAMP_MICROS)? + 1;
    for _ in 0..1024 {
        let bits = samples.next();
        let size = roundtrip(ByteSize::from_bytes(bits))?;
        assert_eq!(size.try_bytes_i64(), i64::try_from(bits));
        assert_eq!(size.try_bytes_usize(), usize::try_from(bits));
        let timestamp = MIN_UNIX_TIMESTAMP_MICROS + i64::try_from(bits % span)?;
        assert_eq!(
            roundtrip(UnixTimestampMicros::try_new(timestamp)?)?.get(),
            timestamp
        );
        let candidate = i64::from_le_bytes(samples.next().to_le_bytes());
        let valid = (MIN_UNIX_TIMESTAMP_MICROS..=MAX_UNIX_TIMESTAMP_MICROS).contains(&candidate);
        assert_eq!(UnixTimestampMicros::try_new(candidate).is_ok(), valid);
        if !valid {
            reject::<UnixTimestampMicros>(&candidate.to_string())?;
        }
        let negative_size = -i128::from(samples.next()) - 1;
        reject::<ByteSize>(&negative_size.to_string())?;
    }
    for value in [MIN_UNIX_TIMESTAMP_MICROS, -1, 0, MAX_UNIX_TIMESTAMP_MICROS] {
        roundtrip(UnixTimestampMicros::try_new(value)?)?;
    }
    for value in [
        i64::MIN,
        MIN_UNIX_TIMESTAMP_MICROS - 1,
        MAX_UNIX_TIMESTAMP_MICROS + 1,
        i64::MAX,
    ] {
        assert!(UnixTimestampMicros::try_new(value).is_err());
        reject::<UnixTimestampMicros>(&value.to_string())?;
    }
    for bytes in [
        0,
        1,
        (1 << 53) + 1,
        i64::MAX as u64,
        i64::MAX as u64 + 1,
        u64::MAX,
    ] {
        let size = roundtrip(ByteSize::from_bytes(bytes))?;
        assert_eq!(size.try_bytes_i64(), i64::try_from(bytes));
        assert_eq!(size.try_bytes_usize(), usize::try_from(bytes));
    }
    for json in ["null", "true", "1.5", "\"1\"", "[]", "18446744073709551616"] {
        reject::<ByteSize>(json)?;
        reject::<UnixTimestampMicros>(json)?;
    }
    Ok(())
}

#[test]
fn generated_currency_and_text_validation_matches_constructor_and_wire()
-> Result<(), Box<dyn Error>> {
    let mut samples = Samples(0xb92e_3076_a458_1fc3);
    for _ in 0..1024 {
        let bytes = samples.next().to_le_bytes();
        let code: String = bytes[..3]
            .iter()
            .map(|byte| char::from(b'A' + byte % 26))
            .collect();
        assert_eq!(roundtrip(CurrencyCode::parse(&code)?)?.to_string(), code);
        for malformed in [
            code.to_ascii_lowercase(),
            format!("{code} "),
            format!(" {code}"),
            code[..2].to_owned(),
        ] {
            assert!(CurrencyCode::parse(&malformed).is_err());
            reject::<CurrencyCode>(&serde_json::to_string(&malformed)?)?;
        }
        let mut text = String::new();
        for _ in 0..samples.next() % 100 {
            let scalar = u32::try_from(samples.next() % 0x11_0000)?;
            if let Some(character) = char::from_u32(scalar) {
                text.push(character);
            }
        }
        let valid = text.len() <= 128;
        assert_eq!(BoundedText::<128>::try_new(&text).is_ok(), valid);
        if valid {
            assert_eq!(
                roundtrip(BoundedText::<128>::try_new(&text)?)?.as_str(),
                text
            );
        } else {
            reject::<BoundedText<128>>(&serde_json::to_string(&text)?)?;
        }
    }
    for scale in 0..=u8::MAX {
        if scale <= MAX_CURRENCY_SCALE {
            assert_eq!(roundtrip(KnownCurrencyScale::try_new(scale)?)?.get(), scale);
        } else {
            assert!(KnownCurrencyScale::try_new(scale).is_err());
            reject::<KnownCurrencyScale>(&scale.to_string())?;
        }
    }
    for json in ["-1", "256", "1.5", "null", "\"2\""] {
        reject::<KnownCurrencyScale>(json)?;
    }
    Ok(())
}

#[test]
fn generated_identifiers_hashes_and_account_scopes_survive_the_actual_codec()
-> Result<(), Box<dyn Error>> {
    let mut samples = Samples(0xc41f_75a9_d268_30be);
    for _ in 0..1024 {
        let mut bytes = [0; 32];
        for chunk in bytes.as_chunks_mut::<8>().0 {
            chunk.copy_from_slice(&samples.next().to_le_bytes());
        }
        let hash = roundtrip(ContentHash::from_bytes(bytes))?;
        assert_eq!(hash.into_bytes(), bytes);
        assert_eq!(ContentHash::from_hex(&hash.to_hex().to_uppercase())?, hash);
        let hex = hash.to_hex();
        let uuid = format!(
            "{}-{}-{}-{}-{}",
            &hex[..8],
            &hex[8..12],
            &hex[12..16],
            &hex[16..20],
            &hex[20..32]
        );
        let profile = roundtrip(uuid.parse::<ProfileId>()?)?;
        assert_eq!(
            roundtrip(uuid.parse::<WorkspaceId>()?)?.as_uuid(),
            profile.as_uuid()
        );
        assert_eq!(
            roundtrip(uuid.parse::<TaskId>()?)?.as_uuid(),
            profile.as_uuid()
        );
        assert_eq!(
            roundtrip(uuid.parse::<AccountId>()?)?.as_uuid(),
            profile.as_uuid()
        );
        let resource = ProviderResourceId::try_new(format!(" {hex}/é/e\u{0301} "))?;
        let first = AccountQualifiedResourceId::new(
            ProviderId::try_new("Service")?,
            ProviderAccountId::try_new(&uuid)?,
            resource.clone(),
        );
        let second = AccountQualifiedResourceId::new(
            ProviderId::try_new("Service")?,
            ProviderAccountId::try_new(format!("{uuid}/other"))?,
            resource,
        );
        assert_ne!(roundtrip(first)?, roundtrip(second)?);
        reject::<ContentHash>(&serde_json::to_string(&format!("g{}", &hex[1..]))?)?;
        reject::<ProfileId>(&serde_json::to_string(&format!("{uuid}x"))?)?;
    }
    let nil: ProfileId = "00000000-0000-0000-0000-000000000000".parse()?;
    assert_eq!(roundtrip(nil)?, nil);
    Ok(())
}
