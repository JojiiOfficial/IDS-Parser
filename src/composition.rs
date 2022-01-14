use std::str::FromStr;

use crate::{destr_form::DestructionForm, error::ParseError, origin::Origin};

/// A single composition of the format "^⿳亠口冋$(GHJKTV)"
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Composition {
    pub reg_origins: Vec<Origin>,
    pub data: Vec<CompositionPart>,
}

/// A single part of the full composition
#[derive(Debug, PartialEq, Eq)]
pub enum CompositionPart {
    Destructive(DestructionForm),
    Radical(char),
    Modifier(Modifier),
    UnencodedComponent(u8),
}

impl CompositionPart {
    /// Returns `true` if the composition part is [`Destructive`].
    ///
    /// [`Destructive`]: CompositionPart::Destructive
    #[inline]
    pub fn is_destructive(&self) -> bool {
        matches!(self, Self::Destructive(..))
    }

    #[inline]
    pub fn as_destructive(&self) -> Option<&DestructionForm> {
        if let Self::Destructive(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the composition part is [`Radical`].
    ///
    /// [`Radical`]: CompositionPart::Radical
    #[inline]
    pub fn is_radical(&self) -> bool {
        matches!(self, Self::Radical(..))
    }

    #[inline]
    pub fn as_radical(&self) -> Option<&char> {
        if let Self::Radical(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the composition part is [`Modifier`].
    ///
    /// [`Modifier`]: CompositionPart::Modifier
    #[inline]
    pub fn is_modifier(&self) -> bool {
        matches!(self, Self::Modifier(..))
    }

    #[inline]
    pub fn as_modifier(&self) -> Option<&Modifier> {
        if let Self::Modifier(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the composition part is [`UnencodedComponent`].
    ///
    /// [`UnencodedComponent`]: CompositionPart::UnencodedComponent
    #[inline]
    pub fn is_unencoded_component(&self) -> bool {
        matches!(self, Self::UnencodedComponent(..))
    }

    pub fn as_unencoded_component(&self) -> Option<&u8> {
        if let Self::UnencodedComponent(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

/// A modifier for another component
#[derive(Debug, PartialEq, Eq)]
pub enum Modifier {
    UnrepresntableCompontent,
    IdeographicVariation,
    Mirror,
    Rotation,
    Subtraction,
}

/// Converts an character of a composition into a `CompositionPart`
impl TryFrom<char> for CompositionPart {
    type Error = ParseError;

    #[inline]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        if let Ok(destructive_form) = DestructionForm::try_from(value) {
            return Ok(CompositionPart::Destructive(destructive_form));
        }
        if let Ok(modifier) = Modifier::try_from(value) {
            return Ok(CompositionPart::Modifier(modifier));
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
                    if n == '[' || n == ']' {
                        continue;
                    }
                    if let Ok(origin) = Origin::try_from(n) {
                        origins.push(origin);
                    }
                }
            }
        }

        Ok(Composition {
            data: parts,
            reg_origins: origins,
        })
    }
}

impl TryFrom<char> for Modifier {
    type Error = ();

    #[inline]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '？' => Modifier::UnrepresntableCompontent,
            '〾' => Modifier::IdeographicVariation,
            '↔' => Modifier::Mirror,
            '↷' => Modifier::Rotation,
            '⊖' => Modifier::Subtraction,
            _ => return Err(()),
        })
    }
}

impl Modifier {
    /// Returns `true` if `c` is a `Modifier`
    #[inline]
    pub fn is_modifier(c: char) -> bool {
        Modifier::try_from(c).is_ok()
    }
}

#[inline]
fn resolve_unencoded_comp(nr: u8) -> CompositionPart {
    CompositionPart::UnencodedComponent(nr)
}

impl Composition {
    /// Returns all radicals from the composition
    #[inline]
    pub fn get_radicals(&self) -> Vec<char> {
        self.data
            .iter()
            .filter_map(|i| i.as_radical())
            .copied()
            .collect()
    }
}
