#![cfg(feature = "byte")]

use byte_unit::{Byte, ParseError, Unit, UnitType};
use rust_decimal::prelude::*;

#[test]
fn parse_str() {
    #[allow(unused_mut, clippy::useless_vec)]
    let mut cases = vec![
        (Err(()) as Result<Byte, ()>, "", false),
        (Ok(Byte::from_u64(0)), "0", false),
        (Err(()), "-0", false),
        (Err(()), "b", false),
        (Err(()), "i", false),
        (Err(()), "c", false),
        (Ok(Byte::from_u64(0)), "0b", false),
        (Ok(Byte::from_u64(1)), "1", false),
        (Err(()), "-1", false),
        (Ok(Byte::from_u64(1)), "1b", false),
        (Ok(Byte::from_u64(1)), "1b", true),
        (Err(()), "1i", false),
        (Err(()), "1c", false),
        (Err(()), "1bc", false),
        (Ok(Byte::from_u64(1)), "1 b", true),
        (Ok(Byte::from_u64(1)), "1  b", true),
        (Ok(Byte::from_u64(1)), "1  b", true),
        (Ok(Byte::from_u64(2)), "1.1", false),
        (Ok(Byte::from_u64(1)), "1.1b", false),
        (Ok(Byte::from_u64(2)), "1.1b", true),
        (Ok(Byte::from_u64(0)), "0kb", false),
        (Ok(Byte::from_u64(150)), "1.2kb", false),
        (Ok(Byte::from_u64(1200)), "1.2kb", true),
        (Ok(Byte::from_u64(2048)), "2 kiB", true),
        (Ok(Byte::from_u64(76650000)), "76.65 MB", false),
        (Ok(Byte::from_u64(16000000000000000000)), "16 EB", false),
        (Ok(Byte::from_u64(18446744073709551615)), "18446744073709551615", false),
    ];

    #[cfg(feature = "u128")]
    cases.extend_from_slice(&[
        (Ok(Byte::from_u128(u64::MAX as u128 + 1).unwrap()), "16 EiB", false),
        (Ok(Byte::from_u128(16000000000000000000000000).unwrap()), "16 YB", false),
        (Ok(Byte::from_u128(999000000000000000000000000).unwrap()), "999 YB", false),
        (Err(()), "1000 YB", false),
    ]);

    #[cfg(not(feature = "u128"))]
    cases.extend_from_slice(&[(Err(()), "16 EiB", false)]);

    for (i, case) in cases.iter().enumerate() {
        let result = Byte::parse_str(case.1, case.2);

        match case.0 {
            Ok(unit) => match result {
                Ok(result) => {
                    assert_eq!(unit, result, "{i}");
                },
                Err(error) => {
                    panic!("{i}\n{error}");
                },
            },
            Err(_) => {
                assert!(result.is_err(), "{i}")
            },
        }
    }
}

#[test]
fn parse_str_unit_error_character() {
    let cases = [('r', "2048r"), ('é', "2048é")];

    for (i, (expected, input)) in cases.iter().enumerate() {
        let error = Byte::parse_str(input, true).unwrap_err();

        match error {
            ParseError::Unit(error) => assert_eq!(*expected, error.character, "{i}"),
            error => panic!("{i}\n{error}"),
        }
    }
}

#[test]
fn exact_unit() {
    #[allow(unused_mut, clippy::useless_vec)]
    let mut cases = vec![
        ((0, Unit::B), "0"),
        ((1, Unit::B), "1"),
        ((123456, Unit::B), "123456B"),
        ((1, Unit::TB), "1T"),
        ((1, Unit::TiB), "1Ti"),
        ((10, Unit::TB), "10T"),
        ((10, Unit::TiB), "10Ti"),
        ((1000, Unit::TiB), "1000Ti"),
        ((1, Unit::PiB), "1024Ti"),
    ];

    #[cfg(feature = "u128")]
    cases.extend_from_slice(&[((1, Unit::YiB), "1Yi")]);

    for (i, case) in cases.iter().enumerate() {
        assert_eq!(case.0, Byte::parse_str(case.1, true).unwrap().get_exact_unit(true), "{i}");
    }
}

#[test]
fn recoverable_unit() {
    #[allow(unused_mut, clippy::useless_vec)]
    let mut cases = vec![
        ((0.0, Unit::B), "0"),
        ((1.0, Unit::B), "1"),
        ((123.456, Unit::KB), "123456B"),
        ((1.0, Unit::TB), "1T"),
        ((1.0, Unit::TiB), "1Ti"),
        ((10.0, Unit::TB), "10T"),
        ((10.0, Unit::TiB), "10Ti"),
        ((1000.0, Unit::TiB), "1000Ti"),
        ((1.0, Unit::PiB), "1024Ti"),
    ];

    #[cfg(feature = "u128")]
    cases.extend_from_slice(&[((1.0, Unit::YiB), "1Yi")]);

    for (i, case) in cases.iter().enumerate() {
        assert_eq!(
            (Decimal::from_f64(case.0.0).unwrap(), case.0.1),
            Byte::parse_str(case.1, true).unwrap().get_recoverable_unit(true, 3),
            "{i}"
        );
    }
}

#[test]
fn adjusted_unit() {
    #[allow(unused_mut, clippy::useless_vec)]
    let mut cases = vec![
        ("15 MB", 15000000.0, UnitType::Decimal),
        ("15.000001 MB", 15000001.0, UnitType::Decimal),
        ("14.30511474609375 MiB", 15000000.0, UnitType::Binary),
        ("14.30511474609375 MiB", 15000000.0, UnitType::Both),
    ];

    #[cfg(feature = "u128")]
    cases.extend_from_slice(&[("100 YB", 100000000000000000000000000.0, UnitType::Decimal)]);

    for (i, case) in cases.iter().enumerate() {
        assert_eq!(
            case.0,
            Byte::from_f64(case.1).unwrap().get_appropriate_unit(case.2).to_string(),
            "{i}"
        );
    }
}

#[cfg(feature = "serde")]
#[test]
fn tests() {
    let cases = [
        ("\"1 MB\"", (1.0, Unit::MB)),
        ("\"2 MiB\"", (2.0, Unit::MiB)),
        ("\"5.5 KiB\"", (5.5, Unit::KiB)),
        ("\"1.234 MB\"", (1.234, Unit::MB)),
        ("\"1234.5 KB\"", (1.2345, Unit::MB)),
    ];

    for (i, case) in cases.iter().enumerate() {
        let byte = Byte::from_f64_with_unit(case.1.0, case.1.1).unwrap();

        assert_eq!(case.0, serde_json::to_string(&byte).unwrap(), "{i}");
        assert_eq!(byte, serde_json::from_str::<Byte>(case.0).unwrap(), "{i}");
    }
}

#[test]
fn from_float() {
    let cases = [(0, 0.0), (0, -0.0), (1, 1.0), (2, 1.25), (1, 1e-30), (15000000, 15000000.0)];

    for (expected, input) in cases {
        assert_eq!(Some(Byte::from_u64(expected)), Byte::from_f64(input));
        assert_eq!(Some(Byte::from_u64(expected)), Byte::from_f32(input as f32));
    }

    assert_eq!(Some(Byte::from_u64(2)), Byte::from_f64(1.0 + f64::EPSILON));
    assert_eq!(Some(Byte::from_u64(2)), Byte::from_f32(1.0 + f32::EPSILON));
    assert_eq!(Some(Byte::from_u64(1 << 52)), Byte::from_f64((1u64 << 52) as f64));
    assert_eq!(Some(Byte::from_u64(1 << 23)), Byte::from_f32((1u64 << 23) as f32));

    for input in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN, 1e30, -1.0] {
        assert_eq!(None, Byte::from_f64(input));
        assert_eq!(None, Byte::from_f32(input as f32));
    }
}

#[test]
fn arithmetic_bounds() {
    let one = Byte::from_u64(1);
    let below_max = Byte::MAX.subtract(one).unwrap();

    assert_eq!(Some(Byte::from_u64(3)), one.add(Byte::from_u64(2)));
    assert_eq!(Some(Byte::from_u64(6)), Byte::from_u64(2).multiply(3));
    assert_eq!(Some(Byte::MAX), below_max.add(one));
    assert_eq!(Some(Byte::MAX), Byte::MAX.multiply(1));
    assert_eq!(Some(Byte::MIN), Byte::MAX.multiply(0));
    assert_eq!(None, Byte::MAX.add(one));
    assert_eq!(None, Byte::MAX.multiply(2));
}

#[test]
fn into_usize() {
    assert_eq!(Ok(123usize), usize::try_from(Byte::from_u64(123)));
    let max = Byte::from_u128(usize::MAX as u128).unwrap();
    assert_eq!(Ok(usize::MAX), usize::try_from(max));

    if let Some(too_large) = Byte::from_u128(usize::MAX as u128 + 1) {
        assert!(usize::try_from(too_large).is_err());
    }
}

#[test]
fn parse_fraction_overflow() {
    let input = format!("{}.9", Decimal::MAX);
    assert!(matches!(
        Byte::parse_str(&input, false),
        Err(ParseError::Value(byte_unit::ValueParseError::NumberTooLong))
    ));
}

#[test]
fn adjusted_max() {
    assert_eq!(Byte::MAX, Byte::MAX.get_adjusted_unit(Unit::B).get_byte());
    for unit in [Unit::B, Unit::KB, Unit::KiB] {
        let adjusted = Byte::MAX.get_adjusted_unit(unit);
        assert_eq!(adjusted, adjusted);
        assert_eq!(core::cmp::Ordering::Equal, adjusted.cmp(&adjusted));
    }
}

#[test]
fn adjusted_format_precision() {
    let adjusted = Byte::from_u64_with_unit(1250, Unit::B).unwrap().get_adjusted_unit(Unit::KB);
    assert_eq!("1.3 KB", format!("{adjusted:#.1}"));
    assert_eq!("1 KB", format!("{adjusted:#.0}"));
    assert_eq!("1.25 KB", format!("{adjusted:#.20}"));
    assert_eq!(" 1.3 KB", format!("{adjusted:>#7.1}"));

    let adjusted = Byte::from_u64(1).get_adjusted_unit(Unit::B);
    assert_eq!("1 B", format!("{adjusted:.2}"));
    assert_eq!("1        B", format!("{adjusted:10.2}"));
    assert_eq!("       1 B", format!("{adjusted:>10.2}"));
}

#[cfg(feature = "serde")]
#[test]
fn adjusted_serde() {
    for value in [Byte::from_u64(1250), Byte::MAX] {
        for unit in [Unit::B, Unit::KB, Unit::KiB] {
            let adjusted = value.get_adjusted_unit(unit);
            let serialized = serde_json::to_string(&adjusted).unwrap();
            let recovered: byte_unit::AdjustedByte = serde_json::from_str(&serialized).unwrap();
            if value == Byte::MAX {
                assert_eq!(Byte::MAX, serde_json::from_str::<Byte>(&serialized).unwrap());
            } else {
                assert_eq!(adjusted, recovered);
            }
        }
    }
}

#[test]
fn decimal_bits_to_bytes() {
    let cases = [
        (1, Decimal::ONE),
        (1, Decimal::from(8)),
        (2, Decimal::new(81, 1)),
        (1, Decimal::new(1, 28)),
    ];
    for (expected, input) in cases {
        assert_eq!(Some(Byte::from_u64(expected)), Byte::from_decimal_with_unit(input, Unit::Bit));
    }
    assert_eq!(None, Byte::from_decimal_with_unit(Decimal::NEGATIVE_ONE, Unit::Bit));
}
