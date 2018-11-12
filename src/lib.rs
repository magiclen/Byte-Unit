/*!
# Byte Unit

A library for interaction with units of bytes.

The units are **B** for 1 byte, **KB** for 1000 bytes, **KiB** for 1024 bytes, **MB** for 1000000 bytes, **MiB** for 1048576 bytes, etc, and up to **PiB** which is 1125899906842624 bytes.

The data type for storing the size of bytes is `u128`, so don't worry about the overflow problem.

## Usage

### Macros

There are `n_*_bytes` macros can be used. The star `*` means the unit. For example, `n_gb_bytes` can be used to get a **n-GB** value in bytes.

```rust
#[macro_use] extern crate byte_unit;

let result = n_gb_bytes!(4);

assert_eq!(4000000000u128, result);
```

You may need to assign a primitive type if the `n` is not an integer.

```rust
#[macro_use] extern crate byte_unit;

let result = n_gb_bytes!(2.5, f64);

assert_eq!(2500000000u128, result);
```

### Byte

The `Byte` structure can be used for representing a size of bytes.

The `from_string` associated function can parse any **SIZE** string and return a `Byte` instance in common usage. The format of a **SIZE** string is like "123", "123KiB" or "50.84 MB".

```rust
extern crate byte_unit;

use byte_unit::Byte;

let result = Byte::from_string("50.84 MB").unwrap();

assert_eq!(50840000u128, result.get_bytes());
```

You can also use the `from_bytes` and `from_unit` associated functions to create a `Byte` instance.

```rust
extern crate byte_unit;

use byte_unit::Byte;

let result = Byte::from_bytes(1500000u128);

assert_eq!(1500000u128, result.get_bytes());
```

```rust
extern crate byte_unit;

use byte_unit::{Byte, ByteUnit};

let result = Byte::from_unit(1500f64, ByteUnit::KB).unwrap();

assert_eq!(1500000u128, result.get_bytes());
```

### AdjustedByte

To change the unit of a `Byte` instance, you can use the `get_adjusted_unit` method.

```rust
extern crate byte_unit;

use byte_unit::{Byte, ByteUnit};

let byte = Byte::from_string("123KiB").unwrap();

let adjusted_byte = byte.get_adjusted_unit(ByteUnit::KB);

assert_eq!("125.95 KB", adjusted_byte.to_string());
```

To change the unit of a `Byte` instance automatically and appropriately, you can use the `get_appropriate_unit` method.

```rust
extern crate byte_unit;

use byte_unit::Byte;

let byte = Byte::from_bytes(1500000u128);

let adjusted_byte = byte.get_appropriate_unit(false);

assert_eq!("1.50 MB", adjusted_byte.to_string());
```

```rust
extern crate byte_unit;

use byte_unit::Byte;

let byte = Byte::from_bytes(1500000u128);

let adjusted_byte = byte.get_appropriate_unit(true);

assert_eq!("1.43 MiB", adjusted_byte.to_string());
```

The number of fractional digits created by the `to_string` method of a `AdjustedByte` instance is always 2.

To change the number of fractional digits in the formatted string, you can use the `format` method instead.

```rust
extern crate byte_unit;

use byte_unit::Byte;

let byte = Byte::from_bytes(1500000u128);

let adjusted_byte = byte.get_appropriate_unit(false);

assert_eq!("1.5 MB", adjusted_byte.format(1));
```
*/

extern crate regex;

#[macro_use]
extern crate lazy_static;

mod macros;
mod byte_unit;

pub use byte_unit::ByteUnit;

use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem::transmute;

use regex::Regex;

lazy_static! {
    static ref BYTE_RE: Regex = {
        Regex::new(r"^(\d+(\.\d+)?)[\s]*(\S{1,3})?$").unwrap()
    };
}

#[derive(Debug, PartialEq, Eq)]
/// Different error types for Byte.
pub enum ByteError {
    /// The value used for creating a `Byte` object is incorrect. (`from_unit`, `from_string`)
    ValueIncorrect,
    /// The unit used for creating a `Byte` object is incorrect. (`from_string`)
    UnitIncorrect,
    /// The string used for creating a `Byte` object is incorrect. (`from_string`)
    ParseError,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
/// Represent the n-bytes data. Use associated functions: `from_unit`, `from_bytes`, `from_string`, to create the instance.
pub struct Byte {
    bytes: u128
}

impl Display for Byte {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{}", self.bytes))
    }
}

impl Byte {
    /// Create a new `Byte` object from a specified value and a unit. **Accuracy** should be taken care of.
    ///
    /// ## Examples
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

        let bytes = get_bytes(value, unit);

        Ok(Byte {
            bytes
        })
    }

    /// Create a new `Byte` object from bytes.
    ///
    /// ## Examples
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
        Byte {
            bytes
        }
    }

    /// Create a new `Byte` object from string. **Accuracy** should be taken care of.
    ///
    /// ## Examples
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
    /// ## Examples
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
        self.bytes
    }

    /// Find the appropriate unit and value for `Byte` object. **Accuracy** should be taken care of.
    ///
    /// ## Examples
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
        let bytes = self.bytes;

        if binary_multiples {
            if bytes > n_pib_bytes!() {
                AdjustedByte {
                    value: bytes as f64 / n_pib_bytes!() as f64,
                    unit: ByteUnit::PiB,
                }
            } else if bytes > n_tib_bytes!() {
                AdjustedByte {
                    value: bytes as f64 / n_tib_bytes!() as f64,
                    unit: ByteUnit::TiB,
                }
            } else if bytes > n_gib_bytes!() {
                AdjustedByte {
                    value: bytes as f64 / n_gib_bytes!() as f64,
                    unit: ByteUnit::GiB,
                }
            } else if bytes > n_mib_bytes!() {
                AdjustedByte {
                    value: bytes as f64 / n_mib_bytes!() as f64,
                    unit: ByteUnit::MiB,
                }
            } else if bytes > n_kib_bytes!() {
                AdjustedByte {
                    value: bytes as f64 / n_kib_bytes!() as f64,
                    unit: ByteUnit::KiB,
                }
            } else {
                AdjustedByte {
                    value: bytes as f64,
                    unit: ByteUnit::B,
                }
            }
        } else {
            if bytes > n_pb_bytes!() {
                AdjustedByte {
                    value: bytes as f64 / n_pb_bytes!() as f64,
                    unit: ByteUnit::PB,
                }
            } else if bytes > n_tb_bytes!() {
                AdjustedByte {
                    value: bytes as f64 / n_tb_bytes!() as f64,
                    unit: ByteUnit::TB,
                }
            } else if bytes > n_gb_bytes!() {
                AdjustedByte {
                    value: bytes as f64 / n_gb_bytes!() as f64,
                    unit: ByteUnit::GB,
                }
            } else if bytes > n_mb_bytes!() {
                AdjustedByte {
                    value: bytes as f64 / n_mb_bytes!() as f64,
                    unit: ByteUnit::MB,
                }
            } else if bytes > n_kb_bytes!() {
                AdjustedByte {
                    value: bytes as f64 / n_kb_bytes!() as f64,
                    unit: ByteUnit::KB,
                }
            } else {
                AdjustedByte {
                    value: bytes as f64,
                    unit: ByteUnit::B,
                }
            }
        }
    }

    /// Adjust the unit and value for `Byte` object. **Accuracy** should be taken care of.
    ///
    /// ## Examples
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
        let bytes = self.bytes;

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

        AdjustedByte {
            value,
            unit,
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// Generated from the `get_appropriate_unit` and `get_adjusted_unit` methods of a `Byte` object.
pub struct AdjustedByte {
    value: f64,
    unit: ByteUnit,
}

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
        if self.value == other.value && self.unit == other.unit {
            return true;
        }

        let self_value = get_bytes(self.value, self.unit);

        let other_value = get_bytes(other.value, other.unit);

        self_value == other_value
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
        format!("{:.*} {}", fractional_digits, self.value, self.unit.to_string())
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn get_unit(&self) -> ByteUnit {
        self.unit
    }
}

impl Display for AdjustedByte {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{:.2} ", self.value))?;
        Display::fmt(&self.unit, f)
    }
}

impl Eq for AdjustedByte {}

impl Hash for AdjustedByte {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bytes: [u8; 8] = unsafe { transmute(self.value) };
        state.write(&bytes);
        self.unit.hash(state);
    }
}

fn get_bytes(value: f64, unit: ByteUnit) -> u128 {
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