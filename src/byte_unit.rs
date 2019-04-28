use crate::regex::Regex;
use crate::ByteError;

use std::fmt::{self, Display, Formatter};

lazy_static! {
    static ref BYTE_UNIT_RE: Regex = {
        Regex::new(r"^(?i)((([ptgmk])(i)?)?b?)$").unwrap()
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// The unit of bytes.
pub enum ByteUnit {
    /// 1 B = 1 byte
    B,
    /// 1 KB = 1000 bytes
    KB,
    /// 1 KiB = 1024 bytes
    KiB,
    /// 1 MB = 1000000 bytes
    MB,
    /// 1 MiB = 1048576 bytes
    MiB,
    /// 1 GB = 1000000000 bytes
    GB,
    /// 1 GiB = 1073741824 bytes
    GiB,
    /// 1 TB = 1000000000000 bytes
    TB,
    /// 1 TiB = 1099511627776 bytes
    TiB,
    /// 1 PB = 1000000000000000 bytes
    PB,
    /// 1 PiB = 1125899906842624 bytes
    PiB,
}

impl Display for ByteUnit {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.as_ref())
    }
}

impl AsRef<str> for ByteUnit {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
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
    /// assert_eq!(ByteUnit::B, ByteUnit::from_str("b").unwrap());
    /// assert_eq!(ByteUnit::B, ByteUnit::from_str("B").unwrap());
    /// assert_eq!(ByteUnit::KB, ByteUnit::from_str("k").unwrap());
    /// assert_eq!(ByteUnit::KB, ByteUnit::from_str("K").unwrap());
    /// assert_eq!(ByteUnit::KiB, ByteUnit::from_str("Kib").unwrap());
    /// assert_eq!(ByteUnit::MB, ByteUnit::from_str("mb").unwrap());
    /// assert_eq!(ByteUnit::MiB, ByteUnit::from_str("mib").unwrap());
    /// assert_eq!(ByteUnit::GB, ByteUnit::from_str("GB").unwrap());
    /// assert_eq!(ByteUnit::GiB, ByteUnit::from_str("GiB").unwrap());
    /// assert_eq!(ByteUnit::TB, ByteUnit::from_str("TB").unwrap());
    /// assert_eq!(ByteUnit::TiB, ByteUnit::from_str("TIB").unwrap());
    /// assert_eq!(ByteUnit::PB, ByteUnit::from_str("PB").unwrap());
    /// assert_eq!(ByteUnit::PiB, ByteUnit::from_str("PiB").unwrap());
    /// ```
    pub fn from_str<S: AsRef<str>>(unit: S) -> Result<ByteUnit, ByteError> {
        let captures = BYTE_UNIT_RE.captures(unit.as_ref()).ok_or(ByteError::UnitIncorrect)?;

        match captures.get(1) {
            Some(_) => {
                match captures.get(3) {
                    Some(m) => {
                        let u: String = m.as_str().to_lowercase();

                        match captures.get(4) {
                            Some(_) => {
                                match u.as_str() {
                                    "k" => Ok(ByteUnit::KiB),
                                    "m" => Ok(ByteUnit::MiB),
                                    "g" => Ok(ByteUnit::GiB),
                                    "t" => Ok(ByteUnit::TiB),
                                    "p" => Ok(ByteUnit::PiB),
                                    _ => unreachable!()
                                }
                            }
                            None => {
                                match u.as_str() {
                                    "k" => Ok(ByteUnit::KB),
                                    "m" => Ok(ByteUnit::MB),
                                    "g" => Ok(ByteUnit::GB),
                                    "t" => Ok(ByteUnit::TB),
                                    "p" => Ok(ByteUnit::PB),
                                    _ => unreachable!()
                                }
                            }
                        }
                    }
                    None => {
                        Ok(ByteUnit::B)
                    }
                }
            }
            None => Ok(ByteUnit::B)
        }
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
    /// assert_eq!("KiB", ByteUnit::KiB.as_str());
    /// assert_eq!("MB", ByteUnit::MB.as_str());
    /// assert_eq!("MiB", ByteUnit::MiB.as_str());
    /// assert_eq!("GB", ByteUnit::GB.as_str());
    /// assert_eq!("GiB", ByteUnit::GiB.as_str());
    /// assert_eq!("TB", ByteUnit::TB.as_str());
    /// assert_eq!("TiB", ByteUnit::TiB.as_str());
    /// assert_eq!("PB", ByteUnit::PB.as_str());
    /// assert_eq!("PiB", ByteUnit::PiB.as_str());
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            ByteUnit::B => "B",
            ByteUnit::KB => "KB",
            ByteUnit::KiB => "KiB",
            ByteUnit::MB => "MB",
            ByteUnit::MiB => "MiB",
            ByteUnit::GB => "GB",
            ByteUnit::GiB => "GiB",
            ByteUnit::TB => "TB",
            ByteUnit::TiB => "TiB",
            ByteUnit::PB => "PB",
            ByteUnit::PiB => "PiB",
        }
    }
}