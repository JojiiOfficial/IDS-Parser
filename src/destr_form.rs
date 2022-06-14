use serde::{Deserialize, Serialize};

use crate::error::ParseError;

/// Describing in which form radicals get combined with each other
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
