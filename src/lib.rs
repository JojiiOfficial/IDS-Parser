pub mod composition;
pub mod destr_form;
pub mod error;
pub mod ids;
pub mod origin;
pub mod sepecial_components;
pub mod utils;
pub mod xref;

pub use composition::{Composition, CompositionPart};
pub use destr_form::DestructionForm;
pub use ids::IDS;
pub use origin::Origin;
pub use xref::XRef;

const MAPPINGS: &[(char, char)] = &[
    ('牛', '牜'),
    ('玉', '𤣩'),
    ('竹', '𥫗'),
    ('艸', '艹'),
    //('肉', '月'),
    ('糸', '糹'),
    ('言', '訁'),
    ('金', '釒'),
    ('食', '飠'),
    ('孑', '子'),
    ('⺶', '羊'),
];

/// Maps radicals to the respective radical/glyph used in the ids dataset
#[inline]
pub fn map_special_form(inp: char) -> char {
    MAPPINGS
        .iter()
        .find(|i| i.1 == inp)
        .map(|i| i.0)
        .unwrap_or(inp)
}

#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        str::FromStr,
    };

    use crate::{
        composition::{Composition, CompositionPart},
        destr_form::DestructionForm,
        ids::IDS,
        origin::Origin,
        xref::{RefType, XRefItem},
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
    fn test_all() {
        let reader = BufReader::new(File::open("./IDS.TXT").unwrap());
        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();
            if line.starts_with('#') || line.starts_with('\u{feff}') {
                continue;
            }

            let parsed = IDS::from_str(&line);
            if let Err(err) = parsed {
                panic!("{line:?}: {err}");
            }
        }
    }

    #[test]
    fn test_dec_ids_full() {
        let input = "U+9AD8	高	^⿳亠口冋$(GHJKTV)	*U+507D≡U+50DE";
        let parsed = IDS::from_str(input);

        assert!(parsed.is_ok());
        assert_eq!(
            parsed.unwrap(),
            IDS {
                literal: '高',
                xrefs: vec![XRef {
                    ref_type: RefType::UnifiableVariants,
                    left: XRefItem {
                        literal: '偽',
                        src_identifier: None,
                    },
                    right: XRefItem {
                        literal: '僞',
                        src_identifier: None,
                    },
                }],
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
    fn test_xref_item() {
        let input = "U+5098";
        let xref_item = XRefItem::from_str(input);
        assert!(xref_item.is_ok());
        assert_eq!(
            xref_item.unwrap(),
            XRefItem {
                literal: '傘',
                src_identifier: None,
            }
        );
    }

    #[test]
    fn test_xref_item_origin() {
        let input = "U+5098(V)";
        let xref_item = XRefItem::from_str(input);
        assert!(xref_item.is_ok());
        assert_eq!(
            xref_item.unwrap(),
            XRefItem {
                literal: '傘',
                src_identifier: Some(Origin::Vietnam)
            }
        );
    }

    #[test]
    fn test_special_unencoded2() {
        let input = "U+8C61	象	^⿱{02}𧰨$(GHJKTV)";
        let parsed = IDS::from_str(input).unwrap();
        let e = parsed
            .comp_by_lang(Origin::Japan)
            .unwrap()
            .get_radicals()
            .collect::<Vec<_>>();
        assert!(e.contains(&'𧰨'));
        assert!(e.contains(&'\u{F2A6}'));
    }

    #[test]
    fn test_special_unencoded() {
        let input = "U+9E7F	鹿	^⿸{55}比$(GHJKTV)";
        let parsed = IDS::from_str(input);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();
        let e = parsed
            .comp_by_lang(Origin::Japan)
            .unwrap()
            .get_radicals()
            .collect::<Vec<_>>();
        assert!(e.contains(&'\u{F2D0}'));
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
