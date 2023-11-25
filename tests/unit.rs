use byte_unit::Unit;

#[test]
fn parse_str() {
    #[allow(unused_mut, clippy::useless_vec)]
    let mut cases = vec![
        (Ok(Unit::Bit) as Result<Unit, ()>, "", true, false),
        (Ok(Unit::B), "", true, true),
        (Ok(Unit::Bit), "b", false, false),
        (Ok(Unit::B), "b", true, false),
        (Err(()), "i", true, false),
        (Err(()), "c", true, false),
        (Err(()), "bc", true, false),
        (Err(()), "bi", true, false),
        (Ok(Unit::Bit), "bit", true, false),
        (Err(()), "bic", true, false),
        (Ok(Unit::Bit), "bits", true, false),
        (Err(()), "bitc", true, false),
        (Ok(Unit::Kbit), "K", false, false),
        (Ok(Unit::KB), "K", false, true),
        (Err(()), "Kc", false, true),
        (Ok(Unit::Kibit), "Ki", false, false),
        (Ok(Unit::KiB), "Ki", false, true),
        (Err(()), "Kic", false, true),
        (Ok(Unit::Kbit), "Kb", false, false),
        (Ok(Unit::Kbit), "kb", false, false),
        (Ok(Unit::KB), "Kb", true, false),
        (Err(()), "Kbc", true, false),
        (Ok(Unit::Kbit), "Kbit", true, false),
        (Ok(Unit::Kibit), "Kib", false, false),
        (Ok(Unit::KiB), "Kib", true, false),
        (Err(()), "Kibc", true, false),
        (Ok(Unit::Kibit), "Kibit", true, false),
        (Ok(Unit::Kibit), "Kibits", true, false),
        (Ok(Unit::KB), "KB", true, false),
        (Ok(Unit::KiB), "KiB", true, false),
        // M
        (Ok(Unit::Mbit), "Mb", false, false),
        (Ok(Unit::MB), "Mb", true, false),
        (Ok(Unit::Mibit), "Mib", false, false),
        (Ok(Unit::MiB), "Mib", true, false),
        (Ok(Unit::MB), "MB", true, false),
        (Ok(Unit::MiB), "MiB", true, false),
        // G
        (Ok(Unit::Gbit), "Gb", false, false),
        (Ok(Unit::GB), "Gb", true, false),
        (Ok(Unit::Gibit), "Gib", false, false),
        (Ok(Unit::GiB), "Gib", true, false),
        (Ok(Unit::GB), "GB", true, false),
        (Ok(Unit::GiB), "GiB", true, false),
        // T
        (Ok(Unit::Tbit), "Tb", false, false),
        (Ok(Unit::TB), "Tb", true, false),
        (Ok(Unit::Tibit), "Tib", false, false),
        (Ok(Unit::TiB), "Tib", true, false),
        (Ok(Unit::TB), "TB", true, false),
        (Ok(Unit::TiB), "TiB", true, false),
        // P
        (Ok(Unit::Pbit), "Pb", false, false),
        (Ok(Unit::PB), "Pb", true, false),
        (Ok(Unit::Pibit), "Pib", false, false),
        (Ok(Unit::PiB), "Pib", true, false),
        (Ok(Unit::PB), "PB", true, false),
        (Ok(Unit::PiB), "PiB", true, false),
        // E
        (Ok(Unit::Ebit), "Eb", false, false),
        (Ok(Unit::EB), "Eb", true, false),
        (Ok(Unit::Eibit), "Eib", false, false),
        (Ok(Unit::EiB), "Eib", true, false),
        (Ok(Unit::EB), "EB", true, false),
        (Ok(Unit::EiB), "EiB", true, false),
    ];

    #[cfg(feature = "u128")]
    cases.extend_from_slice(&[
        // Z
        (Ok(Unit::Zbit), "Zb", false, false),
        (Ok(Unit::ZB), "Zb", true, false),
        (Ok(Unit::Zibit), "Zib", false, false),
        (Ok(Unit::ZiB), "Zib", true, false),
        (Ok(Unit::ZB), "ZB", true, false),
        (Ok(Unit::ZiB), "ZiB", true, false),
        // Y
        (Ok(Unit::Ybit), "Yb", false, false),
        (Ok(Unit::YB), "Yb", true, false),
        (Ok(Unit::Yibit), "Yib", false, false),
        (Ok(Unit::YiB), "Yib", true, false),
        (Ok(Unit::YB), "YB", true, false),
        (Ok(Unit::YiB), "YiB", true, false),
    ]);

    for (i, case) in cases.iter().enumerate() {
        let result = Unit::parse_str(case.1, case.2, case.3);

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

#[cfg(feature = "serde")]
#[test]
fn tests() {
    let cases = [
        ("\"Mb\"", Unit::Mbit),
        ("\"Mib\"", Unit::Mibit),
        ("\"MB\"", Unit::MB),
        ("\"MiB\"", Unit::MiB),
    ];

    for (i, case) in cases.iter().enumerate() {
        assert_eq!(case.0, serde_json::to_string(&case.1).unwrap(), "{i}");
        assert_eq!(case.1, serde_json::from_str(case.0).unwrap(), "{i}");
    }
}
