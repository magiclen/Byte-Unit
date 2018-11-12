use std::fmt::{self, Display, Formatter};

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
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            ByteUnit::B => f.write_str("B"),
            ByteUnit::KB => f.write_str("KB"),
            ByteUnit::KiB => f.write_str("KiB"),
            ByteUnit::MB => f.write_str("MB"),
            ByteUnit::MiB => f.write_str("MiB"),
            ByteUnit::GB => f.write_str("GB"),
            ByteUnit::GiB => f.write_str("GiB"),
            ByteUnit::TB => f.write_str("TB"),
            ByteUnit::TiB => f.write_str("TiB"),
            ByteUnit::PB => f.write_str("PB"),
            ByteUnit::PiB => f.write_str("PiB"),
        }
    }
}