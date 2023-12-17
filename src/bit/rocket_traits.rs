use std::str::FromStr;

use rocket::{
    form::{self, FromFormField, ValueField},
    request::FromParam,
};

use super::Bit;
use crate::ParseError;

impl<'r> FromParam<'r> for Bit {
    type Error = ParseError;

    #[inline]
    fn from_param(v: &'r str) -> Result<Self, Self::Error> {
        Self::from_str(v)
    }
}

impl<'r> FromFormField<'r> for Bit {
    #[inline]
    fn from_value(v: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::from_str(v.value).map_err(form::Error::custom)?)
    }
}
