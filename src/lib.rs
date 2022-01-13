pub mod composition;
pub mod destr_form;
pub mod error;
pub mod ids;
pub mod origin;
pub mod utils;
pub mod xref;

pub use composition::{Composition, CompositionPart};
pub use destr_form::DestructionForm;
pub use ids::IDS;
pub use origin::Origin;
pub use xref::XRef;

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{
        composition::{Composition, CompositionPart},
        destr_form::DestructionForm,
        ids::IDS,
        origin::Origin,
    };

    use super::*;

    #[test]
    fn test_conv_utf_to_char() {
        let inp = &["U+9AD8"];
        let exp = &['高'];

        for (i, code) in inp.iter().enumerate() {
            assert_eq!(utils::utf_code_to_char(code), Some(exp[i]));
        }
    }

    #[test]
    fn test_dec_ids_full() {
        let input = "U+9AD8	高	^⿳亠口冋$(GHJKTV)";
        let parsed = IDS::from_str(input);

        assert!(parsed.is_ok());
        assert_eq!(
            parsed.unwrap(),
            IDS {
                literal: '高',
                // TODO
                xrefs: vec![],
                compositions: vec![Composition {
                    data: vec![
                        CompositionPart::Destructive(DestructionForm::Horizontally3),
                        CompositionPart::Radical('亠'),
                        CompositionPart::Radical('口'),
                        CompositionPart::Radical('冋'),
                    ],
                    reg_origins: vec![
                        Origin::China,
                        Origin::HongKong,
                        Origin::Japan,
                        Origin::SouthKorea,
                        Origin::Taiwan,
                        Origin::Vietnam,
                    ]
                }]
            }
        )
    }

    #[test]
    fn test_dec_ids_full_2_compositions() {
        let input = "U+4E12	丒	^⿱刃一$(GT)	^⿱⿹𠃌㐅一$(J)";
        let parsed = IDS::from_str(input);
        assert!(parsed.is_ok());
        // TODO: add full test
    }

    #[test]
    fn test_dec_ids_full_xref() {
        let input = "U+4E8C	二	^⿱一一$(GHJKTV)	*U+4E8C≠U+2011E≠U+2011F≠U+20120";
        let parsed = IDS::from_str(input);
        assert!(parsed.is_ok());
        // TODO: add full test
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
