use core::{
    fmt::{self, Formatter},
    str::FromStr,
};

use serde::{
    self, Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, Visitor},
};

use super::{AdjustedBit, Bit};

impl Serialize for AdjustedBit {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, {
        // Rounding can move a valid adjusted value above the supported range.
        if self.value >= Bit::MAX.get_adjusted_unit(self.unit).get_value() {
            return if serializer.is_human_readable() {
                serializer.collect_str(&format_args!("{:#}", Bit::MAX))
            } else {
                serializer.collect_str(&format_args!("{:-#}", Bit::MAX))
            };
        }

        if serializer.is_human_readable() {
            serializer.collect_str(&format_args!("{:#}", self))
        } else {
            serializer.collect_str(&format_args!("{:-#}", self))
        }
    }
}

impl<'de> Deserialize<'de> for AdjustedBit {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = AdjustedBit;

            #[inline]
            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a string such as \"123\", \"123Kib\", \"50.84 Mb\"")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: DeError, {
                AdjustedBit::from_str(v).map_err(DeError::custom)
            }
        }

        deserializer.deserialize_str(MyVisitor)
    }
}
