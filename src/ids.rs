use std::str::FromStr;

use crate::{composition::Composition, error::ParseError, xref::XRef};

/// A full Ideographic Destruction Sequence item
#[derive(Default, Debug, PartialEq, Eq)]
pub struct IDS {
    literal: char,
    compositions: Vec<Composition>,
    xrefs: Vec<XRef>,
}

/// Parse an `IDS` from a full string
impl FromStr for IDS {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
