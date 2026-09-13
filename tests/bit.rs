#![cfg(feature = "bit")]

use byte_unit::{Bit, ParseError, Unit, UnitType};
use rust_decimal::prelude::*;

#[test]
fn parse_str() {
    #[allow(unused_mut, clippy::useless_vec)]
    let mut cases = vec![
        (Err(()) as Result<Bit, ()>, ""),
        (Ok(Bit::from_u64(0)), "0"),
        (Err(()), "-0"),
        (Err(()), "b"),
        (Err(()), "i"),
        (Err(()), "c"),
        (Ok(Bit::from_u64(0)), "0b"),
        (Ok(Bit::from_u64(1)), "1"),
        (Err(()), "-1"),
        (Ok(Bit::from_u64(1)), "1b"),
        (Ok(Bit::from_u64(8)), "1B"),
        (Err(()), "1i"),
        (Err(()), "1c"),
        (Err(()), "1bc"),
        (Ok(Bit::from_u64(1)), "1 b"),
        (Ok(Bit::from_u64(1)), "1  b"),
        (Ok(Bit::from_u64(1)), "1  b"),
        (Ok(Bit::from_u64(2)), "1.1"),
        (Ok(Bit::from_u64(2)), "1.1b"),
        (Ok(Bit::from_u64(0)), "0kb"),
        (Ok(Bit::from_u64(1200)), "1.2kb"),
        (Ok(Bit::from_u64(9600)), "1.2kB"),
        (Ok(Bit::from_u64(2048)), "2 kib"),
        (Ok(Bit::from_u64(76650000)), "76.65 Mb"),
        (Ok(Bit::from_u64(16000000000000000000)), "16 Eb"),
        (Ok(Bit::from_u64(18446744073709551615)), "18446744073709551615"),
    ];

    #[cfg(feature = "u128")]
    cases.extend_from_slice(&[
        (Ok(Bit::from_u128(u64::MAX as u128 + 1).unwrap()), "16 Eib"),
        (Ok(Bit::from_u128(16000000000000000000000000).unwrap()), "16 Yb"),
        (Ok(Bit::from_u128(999000000000000000000000000).unwrap()), "999 Yb"),
        (Err(()), "1000 Yb"),
    ]);

    #[cfg(not(feature = "u128"))]
    cases.extend_from_slice(&[(Err(()), "16 Eib")]);

    for (i, case) in cases.iter().enumerate() {
        let result = Bit::parse_str(case.1);

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
        let error = Bit::parse_str(input).unwrap_err();

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
        ((0, Unit::Bit), "0"),
        ((1, Unit::Bit), "1"),
        ((123456, Unit::Bit), "123456b"),
        ((1, Unit::Tbit), "1T"),
        ((1, Unit::Tibit), "1Ti"),
        ((10, Unit::Tbit), "10T"),
        ((10, Unit::Tibit), "10Ti"),
        ((125, Unit::TiB), "1000Ti"),
        ((1, Unit::Pibit), "1024Ti"),
    ];

    #[cfg(feature = "u128")]
    cases.extend_from_slice(&[((1, Unit::Yibit), "1Yi")]);

    for (i, case) in cases.iter().enumerate() {
        assert_eq!(case.0, Bit::parse_str(case.1).unwrap().get_exact_unit(true), "{i}");
    }
}

#[test]
fn recoverable_unit() {
    #[allow(unused_mut, clippy::useless_vec)]
    let mut cases = vec![
        ((0.0, Unit::Bit), "0"),
        ((1.0, Unit::Bit), "1"),
        ((15.432, Unit::KB), "123456b"),
        ((1.0, Unit::Tbit), "1T"),
        ((1.0, Unit::Tibit), "1Ti"),
        ((1.25, Unit::TB), "10T"),
        ((1.25, Unit::TiB), "10Ti"),
        ((125.0, Unit::TiB), "1000Ti"),
        ((1.0, Unit::Pibit), "1024Ti"),
    ];

    #[cfg(feature = "u128")]
    cases.extend_from_slice(&[((1.0, Unit::Yibit), "1Yi")]);

    for (i, case) in cases.iter().enumerate() {
        assert_eq!(
            (Decimal::from_f64(case.0.0).unwrap(), case.0.1),
            Bit::parse_str(case.1).unwrap().get_recoverable_unit(true, 3),
            "{i}"
        );
    }
}

#[test]
fn adjusted_unit() {
    #[allow(unused_mut, clippy::useless_vec)]
    let mut cases = vec![
        ("15 Mb", 15000000.0, UnitType::Decimal),
        ("15.000001 Mb", 15000001.0, UnitType::Decimal),
        ("14.30511474609375 Mib", 15000000.0, UnitType::Binary),
        ("14.30511474609375 Mib", 15000000.0, UnitType::Both),
    ];

    #[cfg(feature = "u128")]
    cases.extend_from_slice(&[("100 Yb", 100000000000000000000000000.0, UnitType::Decimal)]);

    for (i, case) in cases.iter().enumerate() {
        assert_eq!(
            case.0,
            Bit::from_f64(case.1).unwrap().get_appropriate_unit(case.2).to_string(),
            "{i}"
        );
    }
}

#[cfg(feature = "serde")]
#[test]
fn tests() {
    let cases = [
        ("\"1 Mb\"", (1.0, Unit::Mbit)),
        ("\"2 Mib\"", (2.0, Unit::Mibit)),
        ("\"5.5 Kib\"", (5.5, Unit::Kibit)),
        ("\"1.234 Mb\"", (1.234, Unit::Mbit)),
        ("\"1234.5 Kb\"", (1.2345, Unit::Mbit)),
    ];

    for (i, case) in cases.iter().enumerate() {
        let bit = Bit::from_f64_with_unit(case.1.0, case.1.1).unwrap();

        assert_eq!(case.0, serde_json::to_string(&bit).unwrap(), "{i}");
        assert_eq!(bit, serde_json::from_str::<Bit>(case.0).unwrap(), "{i}");
    }
}

#[test]
fn from_float() {
    let cases = [(0, 0.0), (0, -0.0), (1, 1.0), (2, 1.25), (1, 1e-30), (15000000, 15000000.0)];

    for (expected, input) in cases {
        assert_eq!(Some(Bit::from_u64(expected)), Bit::from_f64(input));
        assert_eq!(Some(Bit::from_u64(expected)), Bit::from_f32(input as f32));
    }

    assert_eq!(Some(Bit::from_u64(2)), Bit::from_f64(1.0 + f64::EPSILON));
    assert_eq!(Some(Bit::from_u64(2)), Bit::from_f32(1.0 + f32::EPSILON));
    assert_eq!(Some(Bit::from_u64(1 << 52)), Bit::from_f64((1u64 << 52) as f64));
    assert_eq!(Some(Bit::from_u64(1 << 23)), Bit::from_f32((1u64 << 23) as f32));

    for input in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN, 1e30, -1.0] {
        assert_eq!(None, Bit::from_f64(input));
        assert_eq!(None, Bit::from_f32(input as f32));
    }
}

#[test]
fn arithmetic_bounds() {
    let one = Bit::from_u64(1);
    let below_max = Bit::MAX.subtract(one).unwrap();

    assert_eq!(Some(Bit::from_u64(3)), one.add(Bit::from_u64(2)));
    assert_eq!(Some(Bit::from_u64(6)), Bit::from_u64(2).multiply(3));
    assert_eq!(Some(Bit::MAX), below_max.add(one));
    assert_eq!(Some(Bit::MAX), Bit::MAX.multiply(1));
    assert_eq!(Some(Bit::MIN), Bit::MAX.multiply(0));
    assert_eq!(None, Bit::MAX.add(one));
    assert_eq!(None, Bit::MAX.multiply(2));
}

#[test]
fn into_usize() {
    assert_eq!(Ok(123usize), usize::try_from(Bit::from_u64(123)));
    let max = Bit::from_u128(usize::MAX as u128).unwrap();
    assert_eq!(Ok(usize::MAX), usize::try_from(max));

    if let Some(too_large) = Bit::from_u128(usize::MAX as u128 + 1) {
        assert!(usize::try_from(too_large).is_err());
    }
}

#[test]
fn parse_fraction_overflow() {
    let input = format!("{}.9", Decimal::MAX);
    assert!(matches!(
        Bit::parse_str(&input),
        Err(ParseError::Value(byte_unit::ValueParseError::NumberTooLong))
    ));
}

#[test]
fn adjusted_max() {
    assert_eq!(Bit::MAX, Bit::MAX.get_adjusted_unit(Unit::Bit).get_bit());
    for unit in [Unit::Bit, Unit::KB, Unit::KiB] {
        let adjusted = Bit::MAX.get_adjusted_unit(unit);
        assert_eq!(adjusted, adjusted);
        assert_eq!(core::cmp::Ordering::Equal, adjusted.cmp(&adjusted));
    }
}

#[test]
fn adjusted_format_precision() {
    let adjusted = Bit::from_u64_with_unit(1250, Unit::B).unwrap().get_adjusted_unit(Unit::KB);
    assert_eq!("1.3 KB", format!("{adjusted:#.1}"));
    assert_eq!("1 KB", format!("{adjusted:#.0}"));
    assert_eq!("1.25 KB", format!("{adjusted:#.20}"));
    assert_eq!(" 1.3 KB", format!("{adjusted:>#7.1}"));

    let adjusted = Bit::from_u64(1).get_adjusted_unit(Unit::Bit);
    assert_eq!("1 b", format!("{adjusted:.2}"));
    assert_eq!("1        b", format!("{adjusted:10.2}"));
    assert_eq!("       1 b", format!("{adjusted:>10.2}"));
}

#[cfg(feature = "serde")]
#[test]
fn adjusted_serde() {
    for value in [Bit::from_u64(1250), Bit::MAX] {
        for unit in [Unit::Bit, Unit::KB, Unit::KiB] {
            let adjusted = value.get_adjusted_unit(unit);
            let serialized = serde_json::to_string(&adjusted).unwrap();
            let recovered: byte_unit::AdjustedBit = serde_json::from_str(&serialized).unwrap();
            if value == Bit::MAX {
                assert_eq!(Bit::MAX, serde_json::from_str::<Bit>(&serialized).unwrap());
            } else {
                assert_eq!(adjusted, recovered);
            }
        }
    }
}
