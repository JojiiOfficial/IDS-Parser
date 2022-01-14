use std::str::FromStr;

use crate::{error::ParseError, utils, Origin};

#[derive(Debug, PartialEq, Eq)]
pub struct XRef {
    pub ref_type: RefType,
    pub left: XRefItem,
    pub right: XRefItem,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct XRefItem {
    pub literal: char,
    pub src_identifier: Option<Origin>,
}

/// Type of cross reference
#[derive(Debug, PartialEq, Eq)]
pub enum RefType {
    /// Glyphs are the sams for different characters
    Full,
    /// Deliberately not unified characters
    FullNoCognate,
    /// Characters are unifiable variants
    UnifiableVariants,
}

impl FromStr for XRef {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ref_type: Option<RefType> = None;
        let mut ref_char = ' ';
        for next in s.chars() {
            if let Ok(rt) = RefType::try_from(next) {
                ref_type = Some(rt);
                ref_char = next;
                break;
            }
        }

        if ref_type.is_none() {
            return Err(ParseError::InvalidXRef)?;
        }

        let mut s_iter = s.split(ref_char);
        let left_str = s_iter.next().ok_or(ParseError::InvalidXRef)?;
        let right_str = s_iter.next().ok_or(ParseError::InvalidXRef)?;

        let left = XRefItem::from_str(left_str)?;
        let right = XRefItem::from_str(right_str)?;

        Ok(XRef {
            ref_type: ref_type.unwrap(),
            left,
            right,
        })
    }
}

impl TryFrom<char> for RefType {
    type Error = ParseError;

    #[inline]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '=' => RefType::Full,
            '≠' => RefType::FullNoCognate,
            '≡' => RefType::UnifiableVariants,
            _ => return Err(ParseError::InvalidRefType(value)),
        })
    }
}

impl FromStr for XRefItem {
    type Err = ParseError;

    // Takes something like 'U+5098(V)'
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('(');

        let utf8_str = parts.next().ok_or(ParseError::InvalidXRef)?;
        let character = utils::utf_code_to_char(utf8_str).ok_or(ParseError::InvalidXRef)?;

        let mut origin = None;

        if let Some(next) = parts.next() {
            if next.len() > 1 && next.ends_with(')') {
                let origin_char = next.chars().next().unwrap();
                origin = Some(Origin::try_from(origin_char)?);
            }
        }

        Ok(XRefItem {
            literal: character,
            src_identifier: origin,
        })
    }
}
