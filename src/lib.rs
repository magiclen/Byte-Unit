//! # Byte Unit
//! A library for interaction with units of bytes.

extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

#[macro_export]
/// Convert n KB to bytes.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_kb_bytes!(4);
///
/// assert_eq!(result, 4000u128);
/// ```
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_kb_bytes!(2.5, f64);
///
/// assert_eq!(result, 2500u128);
/// ```
macro_rules! n_kb_bytes {
    () => {1000u128};
    ( $x:expr ) => {$x as u128 * 1000u128};
    ( $x:expr, $t:ty ) => {($x * (1000000 as $t)) as u128 / 1000u128};
}

#[macro_export]
/// Convert n KiB to bytes.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_kib_bytes!(4);
///
/// assert_eq!(result, 4096u128);
/// ```
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_kib_bytes!(2.5, f64);
///
/// assert_eq!(result, 2560u128);
/// ```
macro_rules! n_kib_bytes {
    () => {1024u128};
    ( $x:expr ) => {$x as u128 * 1024u128};
    ( $x:expr, $t:ty ) => {($x * (1048576 as $t)) as u128 / 1024u128};
}

#[macro_export]
/// Convert n MB to bytes.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_mb_bytes!(4);
///
/// assert_eq!(result, 4000000u128);
/// ```
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_mb_bytes!(2.5, f64);
///
/// assert_eq!(result, 2500000u128);
/// ```
macro_rules! n_mb_bytes {
    () => {1000000u128};
    ( $x:expr ) => {$x as u128 * 1000000u128};
    ( $x:expr, $t:ty ) => {($x * (1000000 as $t)) as u128};
}

#[macro_export]
/// Convert n MiB to bytes.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_mib_bytes!(4);
///
/// assert_eq!(result, 4194304u128);
/// ```
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_mib_bytes!(2.5, f64);
///
/// assert_eq!(result, 2621440u128);
/// ```
macro_rules! n_mib_bytes {
    () => {1048576u128};
    ( $x:expr ) => {$x as u128 * 1048576u128};
    ( $x:expr, $t:ty ) => {($x * (1048576 as $t)) as u128};
}

#[macro_export]
/// Convert n GB to bytes.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_gb_bytes!(4);
///
/// assert_eq!(result, 4000000000u128);
/// ```
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_gb_bytes!(2.5, f64);
///
/// assert_eq!(result, 2500000000u128);
/// ```
macro_rules! n_gb_bytes {
    () => {1000000000u128};
    ( $x:expr ) => {$x as u128 * 1000000000u128};
    ( $x:expr, $t:ty ) => {($x * (1000000 as $t)) as u128 * 1000u128};
}

#[macro_export]
/// Convert n GiB to bytes.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_gib_bytes!(4);
///
/// assert_eq!(result, 4294967296u128);
/// ```
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_gib_bytes!(2.5, f64);
///
/// assert_eq!(result, 2684354560u128);
/// ```
macro_rules! n_gib_bytes {
    () => {1073741824u128};
    ( $x:expr ) => {$x as u128 * 1073741824u128};
    ( $x:expr, $t:ty ) => {($x * (1048576 as $t)) as u128 * 1024u128};
}

#[macro_export]
/// Convert n TB to bytes.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_tb_bytes!(4);
///
/// assert_eq!(result, 4000000000000u128);
/// ```
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_tb_bytes!(2.5, f64);
///
/// assert_eq!(result, 2500000000000u128);
/// ```
macro_rules! n_tb_bytes {
    () => {1000000000000u128};
    ( $x:expr ) => {$x as u128 * 1000000000000u128};
    ( $x:expr, $t:ty ) => {($x * (1000000 as $t)) as u128 * 1000000u128};
}

#[macro_export]
/// Convert n TiB to bytes.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_tib_bytes!(4);
///
/// assert_eq!(result, 4398046511104u128);
/// ```
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_tib_bytes!(2.5, f64);
///
/// assert_eq!(result, 2748779069440u128);
/// ```
macro_rules! n_tib_bytes {
    () => {1099511627776u128};
    ( $x:expr ) => {$x as u128 * 1099511627776u128};
    ( $x:expr, $t:ty ) => {($x * (1048576 as $t)) as u128 * 1048576u128};
}

#[macro_export]
/// Convert n PB to bytes.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_pb_bytes!(4);
///
/// assert_eq!(result, 4000000000000000u128);
/// ```
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_pb_bytes!(2.5, f64);
///
/// assert_eq!(result, 2500000000000000u128);
/// ```
macro_rules! n_pb_bytes {
    () => {1000000000000000u128};
    ( $x:expr ) => {$x as u128 * 1000000000000000u128};
    ( $x:expr, $t:ty ) => {($x * (1000000 as $t)) as u128 * 1000000000u128};
}

#[macro_export]
/// Convert n PiB to bytes.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_pib_bytes!(4);
///
/// assert_eq!(result, 4503599627370496u128);
/// ```
///
/// ```
/// #[macro_use] extern crate byte_unit;
///
/// let result = n_pib_bytes!(2.5, f64);
///
/// assert_eq!(result, 2814749767106560u128);
/// ```
macro_rules! n_pib_bytes {
    () => {1125899906842624u128};
    ( $x:expr ) => {$x as u128 * 1125899906842624u128};
    ( $x:expr, $t:ty ) => {($x * (1048576 as $t)) as u128 * 1073741824u128};
}

lazy_static! {
    static ref BYTE_RE: Regex = {
        Regex::new(r"^(\d+(\.\d+)?)[\s]*(\S{1,3})?$").unwrap()
    };
}

#[derive(Debug, PartialEq, Clone)]
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

impl ToString for ByteUnit {
    /// Get the unit string.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::ByteUnit;
    ///
    /// let result = ByteUnit::KB.to_string();
    ///
    /// assert_eq!(result, "KB");
    /// ```
    fn to_string(&self) -> String {
        match self {
            ByteUnit::B => String::from("B"),
            ByteUnit::KB => String::from("KB"),
            ByteUnit::KiB => String::from("KiB"),
            ByteUnit::MB => String::from("MB"),
            ByteUnit::MiB => String::from("MiB"),
            ByteUnit::GB => String::from("GB"),
            ByteUnit::GiB => String::from("GiB"),
            ByteUnit::TB => String::from("TB"),
            ByteUnit::TiB => String::from("TiB"),
            ByteUnit::PB => String::from("PB"),
            ByteUnit::PiB => String::from("PiB"),
        }
    }
}

#[derive(Debug)]
/// Different error types for Byte.
pub enum ByteError {
    /// The value used for creating a `Byte` object is incorrect. (`from_unit`, `from_string`)
    ValueIncorrect,
    /// The unit used for creating a `Byte` object is incorrect. (`from_string`)
    UnitIncorrect,
    /// The string used for creating a `Byte` object is incorrect. (`from_string`)
    ParseError,
}

#[derive(Debug)]
struct ByteInner {
    bytes: u128
}

impl PartialEq for ByteInner {
    fn eq(&self, other: &ByteInner) -> bool {
        self.bytes == other.bytes
    }
}

#[derive(Debug)]
/// Represent the n-bytes data. Use associated functions: `from_unit`, `from_bytes`, `from_string`, to create the instance.
pub struct Byte(ByteInner);

impl PartialEq for Byte {
    /// Deal with the logical numeric equivalent.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let byte1 = Byte::from_unit(1024f64, ByteUnit::KiB).unwrap();
    /// let byte2 = Byte::from_unit(1024f64, ByteUnit::KiB).unwrap();
    ///
    /// assert_eq!(byte1, byte2);
    /// ```
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let byte1 = Byte::from_unit(1024f64, ByteUnit::KiB).unwrap();
    /// let byte2 = Byte::from_unit(1f64, ByteUnit::MiB).unwrap();
    ///
    /// assert_eq!(byte1, byte2);
    /// ```
    fn eq(&self, other: &Byte) -> bool {
        self.0 == other.0
    }
}

impl ToString for Byte {
    /// Format the `Byte` object to bytes as string.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let byte = Byte::from_unit(1500f64, ByteUnit::KB).unwrap();
    ///
    /// let result = byte.to_string();
    ///
    /// assert_eq!(result, "1500000");
    /// ```
    fn to_string(&self) -> String {
        format!("{}", self.0.bytes)
    }
}

impl Byte {
    /// Create a new `Byte` object from a specified value and a unit. **Accuracy** should be taken care of.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let result = Byte::from_unit(1500f64, ByteUnit::KB).unwrap();
    ///
    /// assert_eq!(result.get_bytes(), 1500000u128);
    /// ```
    pub fn from_unit(value: f64, unit: ByteUnit) -> Result<Byte, ByteError> {
        if value < 0f64 {
            return Err(ByteError::ValueIncorrect);
        }

        let bytes = get_bytes(value, &unit);

        Ok(Byte(ByteInner {
            bytes
        }))
    }

    /// Create a new `Byte` object from bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let result = Byte::from_bytes(1500000u128);
    ///
    /// assert_eq!(result.get_bytes(), 1500000u128);
    /// ```
    pub fn from_bytes(bytes: u128) -> Byte {
        Byte(ByteInner {
            bytes
        })
    }

    /// Create a new `Byte` object from string. **Accuracy** should be taken care of.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let result = Byte::from_string("123KiB").unwrap();
    ///
    /// assert_eq!(result, Byte::from_unit(123f64, ByteUnit::KiB).unwrap());
    /// ```
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let result = Byte::from_string("50.84 MB").unwrap();
    ///
    /// assert_eq!(result, Byte::from_unit(50.84f64, ByteUnit::MB).unwrap());
    /// ```
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let result = Byte::from_string("8 b").unwrap(); // 8 bits
    ///
    /// assert_eq!(result.get_bytes(), 1u128);
    /// ```
    pub fn from_string<S: AsRef<str>>(string: S) -> Result<Byte, ByteError> {
        let string = string.as_ref();

        let string = string.trim();

        let regex = &*BYTE_RE;

        let captures = regex.captures(string);

        match captures {
            Some(captures) => {
                let unit = match captures.get(3) {
                    Some(v) => v.as_str(),
                    None => "B"
                };

                let value = match captures[1].parse::<f64>() {
                    Ok(v) => {
                        match unit {
                            "b" => {
                                if v.floor() != v || v % 8f64 != 0f64 {
                                    return Err(ByteError::ValueIncorrect);
                                }

                                v / 8f64
                            }
                            _ => v
                        }
                    }
                    Err(_) => return Err(ByteError::ParseError)
                };

                let unit = match unit.to_uppercase().as_str() {
                    "B" => ByteUnit::B,
                    "KB" | "K" => ByteUnit::KB,
                    "KIB" => ByteUnit::KiB,
                    "MB" | "M" => ByteUnit::MB,
                    "MIB" => ByteUnit::MiB,
                    "GB" | "G" => ByteUnit::GB,
                    "GIB" => ByteUnit::GiB,
                    "TB" | "T" => ByteUnit::TB,
                    "TIB" => ByteUnit::TiB,
                    "PB" | "P" => ByteUnit::PB,
                    "PIB" => ByteUnit::PiB,
                    _ => return Err(ByteError::UnitIncorrect)
                };

                Byte::from_unit(value, unit)
            }
            None => Err(ByteError::ParseError)
        }
    }

    /// Get bytes represented by a `Byte` object.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::Byte;
    ///
    /// let byte = Byte::from_string("123KiB").unwrap();
    ///
    /// let result = byte.get_bytes();
    ///
    /// assert_eq!(result, 125952);
    /// ```
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::Byte;
    ///
    /// let byte = Byte::from_string("50.84 MB").unwrap();
    ///
    /// let result = byte.get_bytes();
    ///
    /// assert_eq!(result, 50840000);
    /// ```
    pub fn get_bytes(&self) -> u128 {
        self.0.bytes
    }

    /// Find the appropriate unit and value for `Byte` object. **Accuracy** should be taken care of.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::Byte;
    ///
    /// let byte = Byte::from_string("123KiB").unwrap();
    ///
    /// let adjusted_byte = byte.get_appropriate_unit(false);
    ///
    /// assert_eq!(adjusted_byte.to_string(), "125.95 KB");
    /// ```
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::Byte;
    ///
    /// let byte = Byte::from_string("50.84 MB").unwrap();
    ///
    /// let adjusted_byte = byte.get_appropriate_unit(true);
    ///
    /// assert_eq!(adjusted_byte.to_string(), "48.48 MiB");
    /// ```
    pub fn get_appropriate_unit(&self, binary_multiples: bool) -> AdjustedByte {
        let bytes = self.0.bytes;

        if binary_multiples {
            if bytes > n_pib_bytes!() {
                AdjustedByte(AdjustedByteInner {
                    value: bytes as f64 / n_pib_bytes!() as f64,
                    unit: ByteUnit::PiB,
                })
            } else if bytes > n_tib_bytes!() {
                AdjustedByte(AdjustedByteInner {
                    value: bytes as f64 / n_tib_bytes!() as f64,
                    unit: ByteUnit::TiB,
                })
            } else if bytes > n_gib_bytes!() {
                AdjustedByte(AdjustedByteInner {
                    value: bytes as f64 / n_gib_bytes!() as f64,
                    unit: ByteUnit::GiB,
                })
            } else if bytes > n_mib_bytes!() {
                AdjustedByte(AdjustedByteInner {
                    value: bytes as f64 / n_mib_bytes!() as f64,
                    unit: ByteUnit::MiB,
                })
            } else if bytes > n_kib_bytes!() {
                AdjustedByte(AdjustedByteInner {
                    value: bytes as f64 / n_kib_bytes!() as f64,
                    unit: ByteUnit::KiB,
                })
            } else {
                AdjustedByte(AdjustedByteInner {
                    value: bytes as f64,
                    unit: ByteUnit::B,
                })
            }
        } else {
            if bytes > n_pb_bytes!() {
                AdjustedByte(AdjustedByteInner {
                    value: bytes as f64 / n_pb_bytes!() as f64,
                    unit: ByteUnit::PB,
                })
            } else if bytes > n_tb_bytes!() {
                AdjustedByte(AdjustedByteInner {
                    value: bytes as f64 / n_tb_bytes!() as f64,
                    unit: ByteUnit::TB,
                })
            } else if bytes > n_gb_bytes!() {
                AdjustedByte(AdjustedByteInner {
                    value: bytes as f64 / n_gb_bytes!() as f64,
                    unit: ByteUnit::GB,
                })
            } else if bytes > n_mb_bytes!() {
                AdjustedByte(AdjustedByteInner {
                    value: bytes as f64 / n_mb_bytes!() as f64,
                    unit: ByteUnit::MB,
                })
            } else if bytes > n_kb_bytes!() {
                AdjustedByte(AdjustedByteInner {
                    value: bytes as f64 / n_kb_bytes!() as f64,
                    unit: ByteUnit::KB,
                })
            } else {
                AdjustedByte(AdjustedByteInner {
                    value: bytes as f64,
                    unit: ByteUnit::B,
                })
            }
        }
    }

    /// Adjust the unit and value for `Byte` object. **Accuracy** should be taken care of.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let byte = Byte::from_string("123KiB").unwrap();
    ///
    /// let adjusted_byte = byte.get_adjusted_unit(ByteUnit::KB);
    ///
    /// assert_eq!(adjusted_byte.to_string(), "125.95 KB");
    /// ```
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let byte = Byte::from_string("50.84 MB").unwrap();
    ///
    /// let adjusted_byte = byte.get_adjusted_unit(ByteUnit::MiB);
    ///
    /// assert_eq!(adjusted_byte.to_string(), "48.48 MiB");
    /// ```
    pub fn get_adjusted_unit(&self, unit: ByteUnit) -> AdjustedByte {
        let bytes = self.0.bytes;

        let value = match unit {
            ByteUnit::B => bytes as f64,
            ByteUnit::KB => bytes as f64 / n_kb_bytes!() as f64,
            ByteUnit::KiB => bytes as f64 / n_kib_bytes!() as f64,
            ByteUnit::MB => bytes as f64 / n_mb_bytes!() as f64,
            ByteUnit::MiB => bytes as f64 / n_mib_bytes!() as f64,
            ByteUnit::GB => bytes as f64 / n_gb_bytes!() as f64,
            ByteUnit::GiB => bytes as f64 / n_gib_bytes!() as f64,
            ByteUnit::TB => bytes as f64 / n_tb_bytes!() as f64,
            ByteUnit::TiB => bytes as f64 / n_tib_bytes!() as f64,
            ByteUnit::PB => bytes as f64 / n_pb_bytes!() as f64,
            ByteUnit::PiB => bytes as f64 / n_pib_bytes!() as f64,
        };

        AdjustedByte(AdjustedByteInner {
            value,
            unit,
        })
    }
}

#[derive(Debug)]
struct AdjustedByteInner {
    value: f64,
    unit: ByteUnit,
}

impl PartialEq for AdjustedByteInner {
    fn eq(&self, other: &AdjustedByteInner) -> bool {
        self.value == other.value && self.unit == other.unit
    }
}

#[derive(Debug)]
/// Generated from the `get_appropriate_unit` and `get_adjusted_unit` methods of a `Byte` object.
pub struct AdjustedByte(AdjustedByteInner);

impl PartialEq for AdjustedByte {
    /// Deal with the logical numeric equivalent.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let byte1 = Byte::from_unit(1024f64, ByteUnit::KiB).unwrap();
    /// let byte2 = Byte::from_unit(1024f64, ByteUnit::KiB).unwrap();
    ///
    /// assert_eq!(byte1.get_appropriate_unit(false), byte2.get_appropriate_unit(true));
    /// ```
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let byte1 = Byte::from_unit(1024f64, ByteUnit::KiB).unwrap();
    /// let byte2 = Byte::from_unit(1f64, ByteUnit::MiB).unwrap();
    ///
    /// assert_eq!(byte1.get_appropriate_unit(true), byte2.get_appropriate_unit(false));
    /// ```
    fn eq(&self, other: &AdjustedByte) -> bool {
        let s = &self.0;
        let o = &other.0;

        if s == o {
            return true;
        }

        let self_value = get_bytes(s.value, &s.unit);

        let other_value = get_bytes(o.value, &o.unit);

        self_value == other_value
    }
}

impl ToString for AdjustedByte {
    /// Format the `AdjustedByte` object to string. The number of fractional digits is 2.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let byte = Byte::from_unit(1500f64, ByteUnit::KB).unwrap();
    ///
    /// let result = byte.get_appropriate_unit(false).to_string();
    ///
    /// assert_eq!(result, "1.50 MB");
    /// ```
    fn to_string(&self) -> String {
        self.format(2)
    }
}

impl AdjustedByte {
    /// Format the `AdjustedByte` object to string.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate byte_unit;
    ///
    /// use byte_unit::{Byte, ByteUnit};
    ///
    /// let byte = Byte::from_unit(1555f64, ByteUnit::KB).unwrap();
    ///
    /// let result = byte.get_appropriate_unit(false).format(3);
    ///
    /// assert_eq!(result, "1.555 MB");
    /// ```
    pub fn format(&self, fractional_digits: usize) -> String {
        format!("{:.*} {}", fractional_digits, self.0.value, self.0.unit.to_string())
    }

    pub fn get_value(&self) -> f64 {
        self.0.value
    }

    pub fn get_unit(&self) -> &ByteUnit {
        &self.0.unit
    }
}

fn get_bytes(value: f64, unit: &ByteUnit) -> u128 {
    match unit {
        ByteUnit::B => value as u128,
        ByteUnit::KB => n_kb_bytes!(value, f64),
        ByteUnit::KiB => n_kib_bytes!(value, f64),
        ByteUnit::MB => n_mb_bytes!(value, f64),
        ByteUnit::MiB => n_mib_bytes!(value, f64),
        ByteUnit::GB => n_gb_bytes!(value, f64),
        ByteUnit::GiB => n_gib_bytes!(value, f64),
        ByteUnit::TB => n_tb_bytes!(value, f64),
        ByteUnit::TiB => n_gib_bytes!(value, f64),
        ByteUnit::PB => n_pb_bytes!(value, f64),
        ByteUnit::PiB => n_pib_bytes!(value, f64)
    }
}

// TODO -----Test START-----

#[cfg(test)]
mod tests {
    // use super::*;
}

// TODO -----Test END-----