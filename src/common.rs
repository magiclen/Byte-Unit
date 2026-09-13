use core::str::Bytes;

#[cfg(any(feature = "byte", feature = "bit"))]
use rust_decimal::prelude::*;

/// # Safety
/// Make sure the input is valid on your own.
pub(crate) unsafe fn get_char_from_bytes(e: u8, mut bytes: Bytes) -> char {
    let width = unsafe { utf8_width::get_width_assume_valid(e) };

    let mut char_bytes = [0; 4];

    char_bytes[0] = e;

    if width > 1 {
        for e in char_bytes[1..].iter_mut().take(width - 1) {
            *e = bytes.next().unwrap();
        }
    }

    let s = unsafe { core::str::from_utf8_unchecked(&char_bytes[..width]) };

    unsafe { s.chars().next().unwrap_unchecked() }
}

#[cfg(any(feature = "byte", feature = "bit"))]
#[cfg(feature = "std")]
#[inline]
pub(crate) fn ceil_f64(v: f64) -> f64 {
    v.ceil()
}

#[cfg(any(feature = "byte", feature = "bit"))]
#[cfg(not(feature = "std"))]
#[inline]
pub(crate) fn ceil_f64(v: f64) -> f64 {
    debug_assert!(v >= 0.0);

    // Values at or above this limit have no fractional bits.
    if v >= (1u64 << 52) as f64 || v == 0.0 {
        return v;
    }

    let trunc = v as u64 as f64;

    if trunc < v { trunc + 1.0 } else { trunc }
}

#[cfg(any(feature = "byte", feature = "bit"))]
#[cfg(feature = "std")]
#[inline]
pub(crate) fn ceil_f32(v: f32) -> f32 {
    debug_assert!(v >= 0.0);

    v.ceil()
}

#[cfg(any(feature = "byte", feature = "bit"))]
#[cfg(not(feature = "std"))]
#[inline]
pub(crate) fn ceil_f32(v: f32) -> f32 {
    debug_assert!(v >= 0.0);

    // Values at or above this limit have no fractional bits.
    if v >= (1u64 << 23) as f32 || v == 0.0 {
        return v;
    }

    let trunc = v as u32 as f32;

    if trunc < v { trunc + 1.0 } else { trunc }
}

#[cfg(any(feature = "byte", feature = "bit"))]
#[cfg(feature = "std")]
#[inline]
pub fn round_fractional_part_f64(value: f64, mut precision: usize) -> f64 {
    if precision > 16 {
        precision = 16;
    } else if precision == 0 {
        return value.round();
    }

    let scale = 10f64.powi(precision as i32);

    (value * scale).round() / scale
}

#[cfg(any(feature = "byte", feature = "bit"))]
#[cfg(not(feature = "std"))]
pub fn round_fractional_part_f64(value: f64, mut precision: usize) -> f64 {
    debug_assert!(value >= 0.0);

    precision = precision.min(16);

    let scale = 10u64.pow(precision as u32) as f64;
    let scaled = value * scale;

    // Large floating-point values already represent whole numbers.
    let rounded = if scaled >= (1u64 << 52) as f64 {
        scaled
    } else {
        let trunc = scaled as u64 as f64;

        if scaled - trunc >= 0.5 { trunc + 1.0 } else { trunc }
    };

    rounded / scale
}

#[cfg(any(feature = "byte", feature = "bit"))]
#[inline]
pub(crate) fn is_zero_remainder_decimal(
    a: Decimal,
    b: Decimal,
    precision: usize,
) -> Option<Decimal> {
    debug_assert!(a.is_sign_positive() && b.is_sign_positive());
    debug_assert!(b > Decimal::ZERO);
    debug_assert!(precision <= 28);

    let quotient = a / b;

    let quotient_round = {
        if precision == 0 {
            quotient.round()
        } else {
            let trunc = quotient.trunc();
            let mut fract = quotient.fract();

            let scale = Decimal::from(10u128.pow(precision as u32));

            fract = (fract * scale).round() / scale;

            trunc + fract
        }
    };

    if b * quotient_round == a { Some(quotient_round) } else { None }
}
