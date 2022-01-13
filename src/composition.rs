use std::str::FromStr;

use crate::{destr_form::DestructionForm, error::ParseError, origin::Origin};

/// A single composition of the format "^⿳亠口冋$(GHJKTV)"
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Composition {
    pub(crate) reg_origins: Vec<Origin>,
    pub(crate) data: Vec<CompositionPart>,
}

/// A single part of the full composition
#[derive(Debug, PartialEq, Eq)]
pub enum CompositionPart {
    Destructive(DestructionForm),
    UnencodedComponent(u8),
    Radical(char),
}

/// Converts an character of a composition into a `CompositionPart`
impl TryFrom<char> for CompositionPart {
    type Error = ParseError;

    #[inline]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        if let Ok(destructive_form) = DestructionForm::try_from(value) {
            return Ok(CompositionPart::Destructive(destructive_form));
        }
        Ok(CompositionPart::Radical(value))
    }
}

/// Converts a composition string to a `Composition`. Format of the composition string: ^⿳亠口冋$(GHJKTV)
impl FromStr for Composition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.chars();
        let first = s_iter.next().ok_or(ParseError::InvalidComposition)?;
        if first != '^' {
            return Err(ParseError::InvalidComposition);
        }

        let mut parts = vec![];

        while let Some(part) = s_iter.next() {
            if part == '$' {
                break;
            }

            if part == '{' {
                let mut unenc_nr = String::new();
                while let Some(pnr) = s_iter.next() {
                    if pnr == '}' {
                        break;
                    }
                    unenc_nr.push(pnr);
                }
                let unenc_nr: u8 = unenc_nr
                    .parse()
                    .map_err(|_| ParseError::UnexpectedCharacter)?;
                parts.push(resolve_unencoded_comp(unenc_nr));
                continue;
            }

            parts.push(CompositionPart::try_from(part)?);
        }

        let mut origins = vec![];

        // should now be left in s_iter: (GHJKTV)
        if let Some(next) = s_iter.next() {
            if next == '(' {
                while let Some(n) = s_iter.next() {
                    if n == ')' {
                        break;
                    }
                    origins.push(Origin::try_from(n)?);
                }
            }
        }

        Ok(Composition {
            data: parts,
            reg_origins: origins,
        })
    }
}

#[inline]
fn resolve_unencoded_comp(nr: u8) -> CompositionPart {
    CompositionPart::UnencodedComponent(nr)
}