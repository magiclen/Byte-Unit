use core::str::FromStr;

use super::Bit;
use crate::{ExceededBoundsError, ParseError, TryFromIntError};

impl TryFrom<u128> for Bit {
    type Error = ExceededBoundsError;

    #[inline]
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Bit::from_u128(value).ok_or(ExceededBoundsError)
    }
}

impl From<u64> for Bit {
    #[inline]
    fn from(value: u64) -> Self {
        Bit::from_u64(value)
    }
}

impl From<u32> for Bit {
    #[inline]
    fn from(value: u32) -> Self {
        Bit::from_u64(value as u64)
    }
}

impl From<u16> for Bit {
    #[inline]
    fn from(value: u16) -> Self {
        Bit::from_u64(value as u64)
    }
}

impl From<u8> for Bit {
    #[inline]
    fn from(value: u8) -> Self {
        Bit::from_u64(value as u64)
    }
}

impl From<usize> for Bit {
    #[inline]
    fn from(value: usize) -> Self {
        #[cfg(target_pointer_width = "128")]
        {
            Bit::from_u128(value as u128).unwrap()
        }

        #[cfg(not(target_pointer_width = "128"))]
        {
            Bit::from_u64(value as u64)
        }
    }
}

impl TryFrom<i128> for Bit {
    type Error = ExceededBoundsError;

    #[inline]
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        Bit::from_i128(value).ok_or(ExceededBoundsError)
    }
}

impl TryFrom<i64> for Bit {
    type Error = ExceededBoundsError;

    #[inline]
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Bit::from_i64(value).ok_or(ExceededBoundsError)
    }
}

impl TryFrom<i32> for Bit {
    type Error = ExceededBoundsError;

    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Bit::from_i64(value as i64).ok_or(ExceededBoundsError)
    }
}

impl TryFrom<i16> for Bit {
    type Error = ExceededBoundsError;

    #[inline]
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Bit::from_i64(value as i64).ok_or(ExceededBoundsError)
    }
}

impl TryFrom<i8> for Bit {
    type Error = ExceededBoundsError;

    #[inline]
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Bit::from_i64(value as i64).ok_or(ExceededBoundsError)
    }
}

impl TryFrom<isize> for Bit {
    type Error = ExceededBoundsError;

    #[inline]
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        #[cfg(target_pointer_width = "128")]
        {
            Bit::from_i128(value as i128).ok_or(ExceededBoundsError)
        }

        #[cfg(not(target_pointer_width = "128"))]
        {
            Bit::from_i64(value as i64).ok_or(ExceededBoundsError)
        }
    }
}

impl TryFrom<f64> for Bit {
    type Error = ExceededBoundsError;

    #[inline]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Bit::from_f64(value).ok_or(ExceededBoundsError)
    }
}

impl TryFrom<f32> for Bit {
    type Error = ExceededBoundsError;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Bit::from_f32(value).ok_or(ExceededBoundsError)
    }
}

impl From<Bit> for u128 {
    #[inline]
    fn from(bit: Bit) -> Self {
        bit.as_u128()
    }
}

impl From<Bit> for u64 {
    #[inline]
    fn from(bit: Bit) -> Self {
        bit.as_u64()
    }
}

impl TryFrom<Bit> for u32 {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(bit: Bit) -> Result<Self, Self::Error> {
        u32::try_from(bit.as_u64())
    }
}

impl TryFrom<Bit> for u16 {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(bit: Bit) -> Result<Self, Self::Error> {
        u16::try_from(bit.as_u64())
    }
}

impl TryFrom<Bit> for u8 {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(bit: Bit) -> Result<Self, Self::Error> {
        u8::try_from(bit.as_u64())
    }
}

impl TryFrom<Bit> for usize {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(bit: Bit) -> Result<Self, Self::Error> {
        #[cfg(target_pointer_width = "128")]
        {
            usize::try_from(bit.as_u128())
        }

        #[cfg(not(target_pointer_width = "128"))]
        {
            usize::try_from(bit.as_u64())
        }
    }
}

impl FromStr for Bit {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Bit::parse_str(s)
    }
}
