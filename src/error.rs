use std::fmt::Display;

/// Error occurring during parsing process
#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    InvalidOrigin(char),
    InvalidDestructiveForm(char),
    InvalidRefType(char),
    InvalidIDS,
    InvalidXRef,
    InvalidComposition,
    UnexpectedCharacter,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for ParseError {}
