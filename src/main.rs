use std::str::FromStr;

fn main() {
    let c = '\u{50E7}';
    println!("Hello, world!: {}", c);
}

/// A full Ideographic Destruction Sequence item
#[derive(Default, Debug, PartialEq, Eq)]
pub struct IDS {
    literal: char,
    compositions: Vec<Composition>,
    xrefs: Vec<XRef>,
}

/// A single composition of the format "^⿳亠口冋$(GHJKTV)"
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Composition {
    reg_origins: Vec<Origin>,
    data: Vec<CompositionPart>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompositionPart {
    Destructive(DestructionForm),
    UnencodedComponent(u8),
    Radical(char),
}

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

#[derive(Debug, PartialEq, Eq)]
pub enum DestructionForm {
    Vertically,
    Horizontally,
    Vertically3,
    Horizontally3,
    BoxInner,
    BoxOpenBottom,
    BoxOpenTop,
    BoxOpenRight,
    BoxOpenLeft,
    BoxOpenBottomRight,
    BoxOpenBottomLeft,
    BoxOpenTopRight,
    Diagonal,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Origin {
    China,
    HongKong,
    Japan,
    SouthKorea,
    Macau,
    NordKorea,
    Sat,
    Taiwan,
    UK,
    Unicode,
    Vietnam,
    UCS2003,
    Alternative,
    UnifiableVariant,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct XRef {
    //
}

impl FromStr for IDS {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl TryFrom<char> for DestructionForm {
    type Error = ParseError;
    #[inline]
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '⿰' => DestructionForm::Vertically,
            '⿱' => DestructionForm::Horizontally,
            '⿲' => DestructionForm::Vertically3,
            '⿳' => DestructionForm::Horizontally3,
            '⿴' => DestructionForm::BoxInner,
            '⿵' => DestructionForm::BoxOpenBottom,
            '⿶' => DestructionForm::BoxOpenTop,
            '⿷' => DestructionForm::BoxOpenRight,
            '⿸' => DestructionForm::BoxOpenBottomRight,
            '⿹' => DestructionForm::BoxOpenBottomLeft,
            '⿺' => DestructionForm::BoxOpenTopRight,
            '⿻' => DestructionForm::Diagonal,
            _ => return Err(ParseError::InvalidDestructiveForm(c)),
        })
    }
}

impl TryFrom<char> for Origin {
    type Error = ParseError;
    #[inline]
    fn try_from(c: char) -> Result<Origin, Self::Error> {
        Ok(match c.to_ascii_uppercase() {
            'G' => Origin::China,
            'H' => Origin::HongKong,
            'J' => Origin::Japan,
            'K' => Origin::SouthKorea,
            'M' => Origin::Macau,
            'P' => Origin::NordKorea,
            'S' => Origin::Sat,
            'T' => Origin::Taiwan,
            'B' => Origin::UK,
            'U' => Origin::Unicode,
            'V' => Origin::Vietnam,
            'X' => Origin::Alternative,
            'Z' => Origin::UnifiableVariant,
            _ => return Err(ParseError::InvalidOrigin(c)),
        })
    }
}

impl FromStr for Composition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // expected: ^⿳亠口冋$(GHJKTV)
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

/// Converts an String encoded utf8-string (eg: `U+9AD8`) to the representing character
#[inline]
pub fn utf_code_to_char(code: &str) -> Option<char> {
    char::from_u32(u32::from_str_radix(code.split('+').nth(1)?, 16).ok()?)
}

#[derive(Debug)]
pub enum ParseError {
    InvalidOrigin(char),
    InvalidDestructiveForm(char),
    InvalidComposition,
    UnexpectedCharacter,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_conv_utf_to_char() {
        let inp = &["U+9AD8"];
        let exp = &['高'];

        for (i, code) in inp.iter().enumerate() {
            assert_eq!(utf_code_to_char(code), Some(exp[i]));
        }
    }

    #[test]
    fn test_dec_ids_full() {
        let input = "U+9AD8	高	^⿳亠口冋$(GHJKTV)";
        let parsed = IDS::from_str(input);

        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), IDS::default())
    }

    #[test]
    fn test_dec_composition1() {
        let input = "^⿳亠口冋$(GHJKTV)";
        let parsed = Composition::from_str(&input);

        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();
        assert_eq!(
            parsed.reg_origins,
            vec![
                Origin::China,
                Origin::HongKong,
                Origin::Japan,
                Origin::SouthKorea,
                Origin::Taiwan,
                Origin::Vietnam,
            ]
        );

        assert_eq!(
            parsed.data,
            vec![
                CompositionPart::Destructive(DestructionForm::Horizontally3),
                CompositionPart::Radical('亠'),
                CompositionPart::Radical('口'),
                CompositionPart::Radical('冋'),
            ]
        );
    }
}
