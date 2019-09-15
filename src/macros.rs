/// Convert n KB to bytes.
///
/// ## Examples
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_kb_bytes!(4);
///
/// assert_eq!(result, 4000u128);
/// ```
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_kb_bytes!(2.5, f64);
///
/// assert_eq!(result, 2500u128);
/// ```
#[macro_export]
macro_rules! n_kb_bytes {
    () => {
        1000u128
    };
    ($x:expr) => {
        $x as u128 * 1000u128
    };
    ($x:expr, $t:ty) => {
        ($x * (1000000 as $t)) as u128 / 1000u128
    };
}

/// Convert n KiB to bytes.
///
/// ## Examples
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_kib_bytes!(4);
///
/// assert_eq!(result, 4096u128);
/// ```
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_kib_bytes!(2.5, f64);
///
/// assert_eq!(result, 2560u128);
/// ```
#[macro_export]
macro_rules! n_kib_bytes {
    () => {
        1024u128
    };
    ($x:expr) => {
        $x as u128 * 1024u128
    };
    ($x:expr, $t:ty) => {
        ($x * (1048576 as $t)) as u128 / 1024u128
    };
}

/// Convert n MB to bytes.
///
/// ## Examples
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_mb_bytes!(4);
///
/// assert_eq!(result, 4000000u128);
/// ```
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_mb_bytes!(2.5, f64);
///
/// assert_eq!(result, 2500000u128);
/// ```
#[macro_export]
macro_rules! n_mb_bytes {
    () => {
        1000000u128
    };
    ($x:expr) => {
        $x as u128 * 1000000u128
    };
    ($x:expr, $t:ty) => {
        ($x * (1000000 as $t)) as u128
    };
}

/// Convert n MiB to bytes.
///
/// ## Examples
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_mib_bytes!(4);
///
/// assert_eq!(result, 4194304u128);
/// ```
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_mib_bytes!(2.5, f64);
///
/// assert_eq!(result, 2621440u128);
/// ```
#[macro_export]
macro_rules! n_mib_bytes {
    () => {
        1048576u128
    };
    ($x:expr) => {
        $x as u128 * 1048576u128
    };
    ($x:expr, $t:ty) => {
        ($x * (1048576 as $t)) as u128
    };
}

/// Convert n GB to bytes.
///
/// ## Examples
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_gb_bytes!(4);
///
/// assert_eq!(result, 4000000000u128);
/// ```
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_gb_bytes!(2.5, f64);
///
/// assert_eq!(result, 2500000000u128);
/// ```
#[macro_export]
macro_rules! n_gb_bytes {
    () => {
        1000000000u128
    };
    ($x:expr) => {
        $x as u128 * 1000000000u128
    };
    ($x:expr, $t:ty) => {
        ($x * (1000000 as $t)) as u128 * 1000u128
    };
}

/// Convert n GiB to bytes.
///
/// ## Examples
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_gib_bytes!(4);
///
/// assert_eq!(result, 4294967296u128);
/// ```
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_gib_bytes!(2.5, f64);
///
/// assert_eq!(result, 2684354560u128);
/// ```
#[macro_export]
macro_rules! n_gib_bytes {
    () => {
        1073741824u128
    };
    ($x:expr) => {
        $x as u128 * 1073741824u128
    };
    ($x:expr, $t:ty) => {
        ($x * (1048576 as $t)) as u128 * 1024u128
    };
}

/// Convert n TB to bytes.
///
/// ## Examples
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_tb_bytes!(4);
///
/// assert_eq!(result, 4000000000000u128);
/// ```
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_tb_bytes!(2.5, f64);
///
/// assert_eq!(result, 2500000000000u128);
/// ```
#[macro_export]
macro_rules! n_tb_bytes {
    () => {
        1000000000000u128
    };
    ($x:expr) => {
        $x as u128 * 1000000000000u128
    };
    ($x:expr, $t:ty) => {
        ($x * (1000000 as $t)) as u128 * 1000000u128
    };
}

/// Convert n TiB to bytes.
///
/// ## Examples
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_tib_bytes!(4);
///
/// assert_eq!(result, 4398046511104u128);
/// ```
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_tib_bytes!(2.5, f64);
///
/// assert_eq!(result, 2748779069440u128);
/// ```
#[macro_export]
macro_rules! n_tib_bytes {
    () => {
        1099511627776u128
    };
    ($x:expr) => {
        $x as u128 * 1099511627776u128
    };
    ($x:expr, $t:ty) => {
        ($x * (1048576 as $t)) as u128 * 1048576u128
    };
}

/// Convert n PB to bytes.
///
/// ## Examples
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_pb_bytes!(4);
///
/// assert_eq!(result, 4000000000000000u128);
/// ```
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_pb_bytes!(2.5, f64);
///
/// assert_eq!(result, 2500000000000000u128);
/// ```
#[macro_export]
macro_rules! n_pb_bytes {
    () => {
        1000000000000000u128
    };
    ($x:expr) => {
        $x as u128 * 1000000000000000u128
    };
    ($x:expr, $t:ty) => {
        ($x * (1000000 as $t)) as u128 * 1000000000u128
    };
}

/// Convert n PiB to bytes.
///
/// ## Examples
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_pib_bytes!(4);
///
/// assert_eq!(result, 4503599627370496u128);
/// ```
///
/// ```
/// extern crate byte_unit;
///
/// let result = byte_unit::n_pib_bytes!(2.5, f64);
///
/// assert_eq!(result, 2814749767106560u128);
/// ```
#[macro_export]
macro_rules! n_pib_bytes {
    () => {
        1125899906842624u128
    };
    ($x:expr) => {
        $x as u128 * 1125899906842624u128
    };
    ($x:expr, $t:ty) => {
        ($x * (1048576 as $t)) as u128 * 1073741824u128
    };
}
