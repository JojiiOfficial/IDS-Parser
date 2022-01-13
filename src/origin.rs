use crate::error::ParseError;

/// What origin a radical composition is from
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

// Parse an origin from its prefix code
impl TryFrom<char> for Origin {
    type Error = ParseError;

    /// Parse an origin from its prefix code
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
