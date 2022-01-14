use std::str::FromStr;

use crate::{composition::Composition, error::ParseError, XRef};

/// A full Ideographic Destruction Sequence item
#[derive(Default, Debug, PartialEq, Eq)]
pub struct IDS {
    pub(crate) literal: char,
    pub(crate) compositions: Vec<Composition>,
    pub(crate) xrefs: Vec<XRef>,
}

/// Parse an `IDS` from a full string
impl FromStr for IDS {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('\t');
        let _utf8_code = split.next().ok_or(ParseError::InvalidIDS)?;

        let literal = split
            .next()
            .ok_or(ParseError::InvalidIDS)?
            .chars()
            .next()
            .unwrap();

        let mut compositions = vec![];
        let mut xrefs = vec![];

        while let Some(part) = split.next() {
            if part.starts_with('^') {
                let composition = Composition::from_str(part)?;
                compositions.push(composition);
            } else if part.starts_with('*') {
                if let Some(xref) = XRef::from_str(&part[1..]).ok() {
                    xrefs.push(xref);
                }
            }
        }

        Ok(IDS {
            literal,
            compositions,
            xrefs,
        })
    }
}
