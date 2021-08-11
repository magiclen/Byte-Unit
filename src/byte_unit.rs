extern crate utf8_width;

use core::convert::TryFrom;
use core::str::{from_utf8_unchecked, Bytes, FromStr};

#[cfg(feature = "serde")]
use alloc::string::String;

use alloc::fmt::{self, Display, Formatter};

use crate::UnitIncorrectError;

#[cfg(feature = "serde")]
use crate::serde::ser::{Serialize, Serializer};

#[cfg(feature = "serde")]
use crate::serde::de::{Deserialize, Deserializer, Error as DeError, Visitor};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// The unit of bytes.
pub enum ByteUnit {
    /// 1 B = 1 byte
    B,
    /// 1 Kb = 125 bytes (10<sup>3</sup> / 8)
    Kb,
    /// 1 KB = 1000 bytes (10<sup>3</sup>)
    KB,
    /// 1 KiB = 1024 bytes (2<sup>10</sup>)
    KiB,
    /// 1 Kb = 125000 bytes (10<sup>6</sup> / 8)
    Mb,
    /// 1 MB = 1000000 bytes (10<sup>6</sup>)
    MB,
    /// 1 MiB = 1048576 bytes (2<sup>20</sup>)
    MiB,
    /// 1 Kb = 125000000 bytes (10<sup>9</sup> / 8)
    Gb,
    /// 1 GB = 1000000000 bytes (10<sup>9</sup>)
    GB,
    /// 1 GiB = 1073741824 bytes (2<sup>30</sup>)
    GiB,
    /// 1 Kb = 125000000000 bytes (10<sup>12</sup> / 8)
    Tb,
    /// 1 TB = 1000000000000 bytes (10<sup>12</sup>)
    TB,
    /// 1 TiB = 1099511627776 bytes (2<sup>40</sup>)
    TiB,
    /// 1 Kb = 125000000000000 bytes (10<sup>15</sup> / 8)
    Pb,
    /// 1 PB = 1000000000000000 bytes (10<sup>15</sup>)
    PB,
    /// 1 PiB = 1125899906842624 bytes (2<sup>50</sup>)
    PiB,
    #[cfg(feature = "u128")]
    /// 1 Kb = 125000000000000000 bytes (10<sup>18</sup> / 8)
    Eb,
    #[cfg(feature = "u128")]
    /// 1 EB = 1000000000000000000 bytes (10<sup>18</sup>)
    EB,
    #[cfg(feature = "u128")]
    /// 1 EiB = 1152921504606846976 bytes (2<sup>60</sup>)
    EiB,
    #[cfg(feature = "u128")]
    /// 1 Kb = 125000000000000000000 bytes (10<sup>21</sup> / 8)
    Zb,
    #[cfg(feature = "u128")]
    /// 1 ZB = 1000000000000000000000 bytes (10<sup>21</sup>)
    ZB,
    #[cfg(feature = "u128")]
    /// 1 ZiB = 1180591620717411303424 bytes (2<sup>70</sup>)
    ZiB,
}

impl ByteUnit {
    /// Get an instance of `ByteUnit` from a string slice.
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::ByteUnit;
    ///
    /// assert_eq!(ByteUnit::B, ByteUnit::from_str("").unwrap());
    /// assert_eq!(ByteUnit::B, ByteUnit::from_str("B").unwrap());
    /// assert_eq!(ByteUnit::KB, ByteUnit::from_str("K").unwrap());
    /// assert_eq!(ByteUnit::Kb, ByteUnit::from_str("Kb").unwrap());
    /// assert_eq!(ByteUnit::KiB, ByteUnit::from_str("KiB").unwrap());
    /// assert_eq!(ByteUnit::MB, ByteUnit::from_str("MB").unwrap());
    /// assert_eq!(ByteUnit::Mb, ByteUnit::from_str("Mb").unwrap());
    /// assert_eq!(ByteUnit::MiB, ByteUnit::from_str("miB").unwrap());
    /// assert_eq!(ByteUnit::GB, ByteUnit::from_str("GB").unwrap());
    /// assert_eq!(ByteUnit::Gb, ByteUnit::from_str("Gb").unwrap());
    /// assert_eq!(ByteUnit::GiB, ByteUnit::from_str("GiB").unwrap());
    /// assert_eq!(ByteUnit::TB, ByteUnit::from_str("TB").unwrap());
    /// assert_eq!(ByteUnit::Tb, ByteUnit::from_str("Tb").unwrap());
    /// assert_eq!(ByteUnit::TiB, ByteUnit::from_str("TIB").unwrap());
    /// assert_eq!(ByteUnit::PB, ByteUnit::from_str("PB").unwrap());
    /// assert_eq!(ByteUnit::Pb, ByteUnit::from_str("Pb").unwrap());
    /// assert_eq!(ByteUnit::PiB, ByteUnit::from_str("PiB").unwrap());
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(unit: S) -> Result<ByteUnit, UnitIncorrectError> {
        let s = unit.as_ref().trim();

        let mut bytes = s.bytes();

        read_xib(bytes.next(), bytes)
    }

    /// Use string slice to represent this `ByteUnit`.
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::ByteUnit;
    ///
    /// assert_eq!("B", ByteUnit::B.as_str());
    /// assert_eq!("KB", ByteUnit::KB.as_str());
    /// assert_eq!("Kb", ByteUnit::Kb.as_str());
    /// assert_eq!("KiB", ByteUnit::KiB.as_str());
    /// assert_eq!("MB", ByteUnit::MB.as_str());
    /// assert_eq!("Mb", ByteUnit::Mb.as_str());
    /// assert_eq!("MiB", ByteUnit::MiB.as_str());
    /// assert_eq!("GB", ByteUnit::GB.as_str());
    /// assert_eq!("Gb", ByteUnit::Gb.as_str());
    /// assert_eq!("GiB", ByteUnit::GiB.as_str());
    /// assert_eq!("TB", ByteUnit::TB.as_str());
    /// assert_eq!("Tb", ByteUnit::Tb.as_str());
    /// assert_eq!("TiB", ByteUnit::TiB.as_str());
    /// assert_eq!("PB", ByteUnit::PB.as_str());
    /// assert_eq!("Pb", ByteUnit::Pb.as_str());
    /// assert_eq!("PiB", ByteUnit::PiB.as_str());
    /// ```
    #[inline]
    pub fn as_str(self) -> &'static str {
        match self {
            ByteUnit::B => "B",
            ByteUnit::Kb => "Kb",
            ByteUnit::KB => "KB",
            ByteUnit::KiB => "KiB",
            ByteUnit::Mb => "Mb",
            ByteUnit::MB => "MB",
            ByteUnit::MiB => "MiB",
            ByteUnit::Gb => "Gb",
            ByteUnit::GB => "GB",
            ByteUnit::GiB => "GiB",
            ByteUnit::Tb => "Tb",
            ByteUnit::TB => "TB",
            ByteUnit::TiB => "TiB",
            ByteUnit::Pb => "Pb",
            ByteUnit::PB => "PB",
            ByteUnit::PiB => "PiB",
            #[cfg(feature = "u128")]
            ByteUnit::Eb => "Eb",
            #[cfg(feature = "u128")]
            ByteUnit::EB => "EB",
            #[cfg(feature = "u128")]
            ByteUnit::EiB => "EiB",
            #[cfg(feature = "u128")]
            ByteUnit::Zb => "Zb",
            #[cfg(feature = "u128")]
            ByteUnit::ZB => "ZB",
            #[cfg(feature = "u128")]
            ByteUnit::ZiB => "ZiB",
        }
    }

    /// Get bytes represented by this `ByteUnit`.
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::ByteUnit;
    ///
    /// assert_eq!(125000000000000000000, ByteUnit::Zb.get_unit_bytes());
    /// assert_eq!(1000000000000000000000, ByteUnit::ZB.get_unit_bytes());
    /// assert_eq!(1152921504606846976, ByteUnit::EiB.get_unit_bytes());
    /// ```
    #[cfg(feature = "u128")]
    #[inline]
    pub fn get_unit_bytes(self) -> u128 {
        match self {
            ByteUnit::B => 1,
            ByteUnit::Kb => n_kb_bits!(),
            ByteUnit::KB => n_kb_bytes!(),
            ByteUnit::KiB => n_kib_bytes!(),
            ByteUnit::Mb => n_mb_bits!(),
            ByteUnit::MB => n_mb_bytes!(),
            ByteUnit::MiB => n_mib_bytes!(),
            ByteUnit::Gb => n_gb_bits!(),
            ByteUnit::GB => n_gb_bytes!(),
            ByteUnit::GiB => n_gib_bytes!(),
            ByteUnit::Tb => n_tb_bits!(),
            ByteUnit::TB => n_tb_bytes!(),
            ByteUnit::TiB => n_tib_bytes!(),
            ByteUnit::Pb => n_pb_bits!(),
            ByteUnit::PB => n_pb_bytes!(),
            ByteUnit::PiB => n_pib_bytes!(),
            ByteUnit::Eb => n_eb_bits!(),
            ByteUnit::EB => n_eb_bytes!(),
            ByteUnit::EiB => n_eib_bytes!(),
            ByteUnit::Zb => n_zb_bits!(),
            ByteUnit::ZB => n_zb_bytes!(),
            ByteUnit::ZiB => n_zib_bytes!(),
        }
    }

    /// Get bytes represented by this `ByteUnit`.
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::ByteUnit;
    ///
    /// assert_eq!(125, ByteUnit::Kb.get_unit_bytes());
    /// assert_eq!(1024, ByteUnit::KiB.get_unit_bytes());
    /// assert_eq!(1000000000, ByteUnit::GB.get_unit_bytes());
    /// ```
    #[cfg(not(feature = "u128"))]
    #[inline]
    pub fn get_unit_bytes(self) -> u64 {
        match self {
            ByteUnit::B => 1,
            ByteUnit::Kb => n_kb_bits!(),
            ByteUnit::KB => n_kb_bytes!(),
            ByteUnit::KiB => n_kib_bytes!(),
            ByteUnit::Mb => n_mb_bits!(),
            ByteUnit::MB => n_mb_bytes!(),
            ByteUnit::MiB => n_mib_bytes!(),
            ByteUnit::Gb => n_gb_bits!(),
            ByteUnit::GB => n_gb_bytes!(),
            ByteUnit::GiB => n_gib_bytes!(),
            ByteUnit::Tb => n_tb_bits!(),
            ByteUnit::TB => n_tb_bytes!(),
            ByteUnit::TiB => n_tib_bytes!(),
            ByteUnit::Pb => n_pb_bits!(),
            ByteUnit::PB => n_pb_bytes!(),
            ByteUnit::PiB => n_pib_bytes!(),
        }
    }
}

impl Display for ByteUnit {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for ByteUnit {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for ByteUnit {
    type Error = UnitIncorrectError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        ByteUnit::from_str(s)
    }
}

impl FromStr for ByteUnit {
    type Err = UnitIncorrectError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ByteUnit::from_str(s)
    }
}

#[inline]
pub(crate) fn get_char_from_bytes(e: u8, mut bytes: Bytes) -> char {
    let width = unsafe { utf8_width::get_width_assume_valid(e) };

    let mut char_bytes = [e; 4];

    if width > 1 {
        for e in char_bytes[1..].iter_mut().take(width - 1) {
            *e = bytes.next().unwrap();
        }
    }

    let char_str = unsafe { from_utf8_unchecked(&char_bytes[..width]) };

    char::from_str(char_str).unwrap()
}

pub(crate) fn read_xib(e: Option<u8>, mut bytes: Bytes) -> Result<ByteUnit, UnitIncorrectError> {
    match e {
        Some(e) => {
            match e {
                b'B' => {
                    match bytes.next() {
                        Some(e) => {
                            Err(UnitIncorrectError {
                                character: get_char_from_bytes(e, bytes),
                                expected_characters: &[],
                                also_expect_no_character: false,
                            })
                        }
                        None => Ok(ByteUnit::B),
                    }
                }
                b'K' | b'k' => {
                    match read_postfix(bytes)? {
                        ByteUnitPostfix::It => Ok(ByteUnit::Kb),
                        ByteUnitPostfix::Yte => Ok(ByteUnit::KB),
                        ByteUnitPostfix::Ibit => Ok(ByteUnit::KiB),
                    }
                }
                b'M' | b'm' => {
                    match read_postfix(bytes)? {
                        ByteUnitPostfix::It => Ok(ByteUnit::Mb),
                        ByteUnitPostfix::Yte => Ok(ByteUnit::MB),
                        ByteUnitPostfix::Ibit => Ok(ByteUnit::MiB),
                    }
                }
                b'G' | b'g' => {
                    match read_postfix(bytes)? {
                        ByteUnitPostfix::It => Ok(ByteUnit::Gb),
                        ByteUnitPostfix::Yte => Ok(ByteUnit::GB),
                        ByteUnitPostfix::Ibit => Ok(ByteUnit::GiB),
                    }
                }
                b'T' | b't' => {
                    match read_postfix(bytes)? {
                        ByteUnitPostfix::It => Ok(ByteUnit::Tb),
                        ByteUnitPostfix::Yte => Ok(ByteUnit::TB),
                        ByteUnitPostfix::Ibit => Ok(ByteUnit::TiB),
                    }
                }
                b'P' | b'p' => {
                    match read_postfix(bytes)? {
                        ByteUnitPostfix::It => Ok(ByteUnit::Pb),
                        ByteUnitPostfix::Yte => Ok(ByteUnit::PB),
                        ByteUnitPostfix::Ibit => Ok(ByteUnit::PiB),
                    }
                }
                #[cfg(feature = "u128")]
                b'E' | b'e' => {
                    match read_postfix(bytes)? {
                        ByteUnitPostfix::It => Ok(ByteUnit::Eb),
                        ByteUnitPostfix::Yte => Ok(ByteUnit::EB),
                        ByteUnitPostfix::Ibit => Ok(ByteUnit::EiB),
                    }
                }
                #[cfg(feature = "u128")]
                b'Z' | b'z' => {
                    match read_postfix(bytes)? {
                        ByteUnitPostfix::It => Ok(ByteUnit::Zb),
                        ByteUnitPostfix::Yte => Ok(ByteUnit::ZB),
                        ByteUnitPostfix::Ibit => Ok(ByteUnit::ZiB),
                    }
                }
                _ => {
                    #[cfg(feature = "u128")]
                    {
                        Err(UnitIncorrectError {
                            character: get_char_from_bytes(e, bytes),
                            expected_characters: &['B', 'K', 'k', 'M', 'm', 'G', 'g', 'T', 't', 'P', 'p', 'E', 'e', 'Z', 'z'],
                            also_expect_no_character: true,
                        })
                    }
                    #[cfg(not(feature = "u128"))]
                    {
                        Err(UnitIncorrectError {
                            character: get_char_from_bytes(e, bytes),
                            expected_characters: &['B', 'K', 'k', 'M', 'm', 'G', 'g', 'T', 't', 'P', 'p'],
                            also_expect_no_character: true,
                        })
                    }
                }
            }
        }
        None => Ok(ByteUnit::B),
    }
}

enum ByteUnitPostfix {
    It,
    Yte,
    Ibit,
}

fn read_postfix(mut bytes: Bytes) -> Result<ByteUnitPostfix, UnitIncorrectError> {
    match bytes.next() {
        Some(e) => {
            match e {
                b'I' | b'i' => {
                    match bytes.next() {
                        Some(e) => {
                            match e {
                                b'B' => Ok(ByteUnitPostfix::Ibit),
                                _ => {
                                    Err(UnitIncorrectError {
                                        character: get_char_from_bytes(e, bytes),
                                        expected_characters: &['B'],
                                        also_expect_no_character: false,
                                    })
                                }
                            }
                        }
                        None => Ok(ByteUnitPostfix::Ibit),
                    }
                }
                b'B' => {
                    match bytes.next() {
                        Some(e) => {
                            Err(UnitIncorrectError {
                                character: get_char_from_bytes(e, bytes),
                                expected_characters: &[],
                                also_expect_no_character: false,
                            })
                        }
                        None => Ok(ByteUnitPostfix::Yte),
                    }
                }
                b'b' => {
                    match bytes.next() {
                        Some(e) => {
                            Err(UnitIncorrectError {
                                character: get_char_from_bytes(e, bytes),
                                expected_characters: &[],
                                also_expect_no_character: false,
                            })
                        }
                        None => Ok(ByteUnitPostfix::It),
                    }
                }
                _ => {
                    Err(UnitIncorrectError {
                        character: get_char_from_bytes(e, bytes),
                        expected_characters: &['B', 'i'],
                        also_expect_no_character: false,
                    })
                }
            }
        }
        None => Ok(ByteUnitPostfix::Yte),
    }
}

#[cfg(feature = "serde")]
impl Serialize for ByteUnit {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for ByteUnit {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct ByteUnitVisitor;

        impl<'de> Visitor<'de> for ByteUnitVisitor {
            type Value = ByteUnit;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                f.write_str("a byte unit such as \"B\", \"KB\" or \"MiB\"")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: DeError, {
                ByteUnit::from_str(v).map_err(DeError::custom)
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: DeError, {
                ByteUnit::from_str(v.as_str()).map_err(DeError::custom)
            }
        }

        deserializer.deserialize_any(ByteUnitVisitor)
    }
}
