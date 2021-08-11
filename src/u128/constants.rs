/// Kb
pub const KILOBIT: u128 = 125;
/// KB
pub const KILOBYTE: u128 = 1_000;
/// KiB
pub const KIBIBYTE: u128 = 1 << 10;
/// Mb
pub const MEGABIT: u128 = 125_000;
/// MB
pub const MEGABYTE: u128 = 1_000_000;
/// MiB
pub const MEBIBYTE: u128 = 1 << 20;
/// Gb
pub const GIGABIT: u128 = 125_000_000;
/// GB
pub const GIGABYTE: u128 = 1_000_000_000;
/// GiB
pub const GIBIBYTE: u128 = 1 << 30;
/// Tb
pub const TERABIT: u128 = 125_000_000_000;
/// TB
pub const TERABYTE: u128 = 1_000_000_000_000;
/// TiB
pub const TEBIBYTE: u128 = 1 << 40;
/// Pb
pub const PETABIT: u128 = 125_000_000_000_000;
/// PB
pub const PETABYTE: u128 = 1_000_000_000_000_000;
/// PiB
pub const PEBIBYTE: u128 = 1 << 50;
/// Eb
pub const EXABIT: u128 = 125_000_000_000_000_000;
/// EB
pub const EXABYTE: u128 = 1_000_000_000_000_000_000;
/// EiB
pub const EXBIBYTE: u128 = 1 << 60;
/// Zb
pub const ZETTABIT: u128 = 125_000_000_000_000_000_000;
/// ZB
pub const ZETTABYTE: u128 = 1_000_000_000_000_000_000_000;
/// ZiB
pub const ZEBIBYTE: u128 = 1 << 70;

/// Convert n Kb to bits.
#[inline]
pub const fn n_kb_bits(bits: u128) -> u128 {
    bits * KILOBIT
}

/// Convert n KB to bytes.
#[inline]
pub const fn n_kb_bytes(bytes: u128) -> u128 {
    bytes * KILOBYTE
}

/// Convert n KiB to bytes.
#[inline]
pub const fn n_kib_bytes(bytes: u128) -> u128 {
    bytes * KIBIBYTE
}

/// Convert n Mb to bits.
#[inline]
pub const fn n_mb_bits(bits: u128) -> u128 {
    bits * MEGABIT
}

/// Convert n MB to bytes.
#[inline]
pub const fn n_mb_bytes(bytes: u128) -> u128 {
    bytes * MEGABYTE
}

/// Convert n MiB to bytes.
#[inline]
pub const fn n_mib_bytes(bytes: u128) -> u128 {
    bytes * MEBIBYTE
}

/// Convert n Gb to bits.
#[inline]
pub const fn n_gb_bits(bits: u128) -> u128 {
    bits * GIGABIT
}

/// Convert n GB to bytes.
#[inline]
pub const fn n_gb_bytes(bytes: u128) -> u128 {
    bytes * GIGABYTE
}

/// Convert n GiB to bytes.
#[inline]
pub const fn n_gib_bytes(bytes: u128) -> u128 {
    bytes * GIBIBYTE
}

/// Convert n Tb to bits.
#[inline]
pub const fn n_tb_bits(bits: u128) -> u128 {
    bits * TERABIT
}

/// Convert n TB to bytes.
#[inline]
pub const fn n_tb_bytes(bytes: u128) -> u128 {
    bytes * TERABYTE
}

/// Convert n TiB to bytes.
#[inline]
pub const fn n_tib_bytes(bytes: u128) -> u128 {
    bytes * TEBIBYTE
}

/// Convert n Pb to bits.
#[inline]
pub const fn n_pb_bits(bits: u128) -> u128 {
    bits * PETABIT
}

/// Convert n PB to bytes.
#[inline]
pub const fn n_pb_bytes(bytes: u128) -> u128 {
    bytes * PETABYTE
}

/// Convert n PiB to bytes.
#[inline]
pub const fn n_pib_bytes(bytes: u128) -> u128 {
    bytes * PEBIBYTE
}

/// Convert n Eb to bits.
#[inline]
pub const fn n_eb_bits(bits: u128) -> u128 {
    bits * EXABIT
}

/// Convert n EB to bytes.
#[inline]
pub const fn n_eb_bytes(bytes: u128) -> u128 {
    bytes * EXABYTE
}

/// Convert n EiB to bytes.
#[inline]
pub const fn n_eib_bytes(bytes: u128) -> u128 {
    bytes * EXBIBYTE
}

/// Convert n Zb to bits.
#[inline]
pub const fn n_zb_bits(bits: u128) -> u128 {
    bits * ZETTABIT
}

/// Convert n ZB to bytes.
#[inline]
pub const fn n_zb_bytes(bytes: u128) -> u128 {
    bytes * ZETTABYTE
}

/// Convert n ZiB to bytes.
#[inline]
pub const fn n_zib_bytes(bytes: u128) -> u128 {
    bytes * ZEBIBYTE
}
