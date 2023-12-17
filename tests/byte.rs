#![cfg(feature = "byte")]

use byte_unit::{Byte, Unit, UnitType};
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
            (Decimal::from_f64(case.0 .0).unwrap(), case.0 .1),
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
        let byte = Byte::from_f64_with_unit(case.1 .0, case.1 .1).unwrap();

        assert_eq!(case.0, serde_json::to_string(&byte).unwrap(), "{i}");
        assert_eq!(byte, serde_json::from_str::<Byte>(case.0).unwrap(), "{i}");
    }
}
