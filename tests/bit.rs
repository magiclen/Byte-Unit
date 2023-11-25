#![cfg(feature = "bit")]

use byte_unit::{Bit, Unit, UnitType};
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
            (Decimal::from_f64(case.0 .0).unwrap(), case.0 .1),
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
        let bit = Bit::from_f64_with_unit(case.1 .0, case.1 .1).unwrap();

        assert_eq!(case.0, serde_json::to_string(&bit).unwrap(), "{i}");
        assert_eq!(bit, serde_json::from_str(case.0).unwrap(), "{i}");
    }
}
