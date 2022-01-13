use std::str::FromStr;

use crate::error::ParseError;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct XRef {
    //
}

impl FromStr for XRef {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
