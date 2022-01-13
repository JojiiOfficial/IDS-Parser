use std::fmt::Display;

/// Error occurring during parsing process
#[derive(Debug)]
pub enum ParseError {
    InvalidOrigin(char),
    InvalidDestructiveForm(char),
    InvalidIDS,
    InvalidComposition,
    UnexpectedCharacter,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for ParseError {}
