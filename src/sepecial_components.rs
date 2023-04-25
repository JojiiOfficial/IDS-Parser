/// Converts one of those unencoded special characters
pub fn conv_special(nr: u8) -> Option<char> {
    SPECIAL_IDS.get(nr as usize).map(|c| *c)
}

pub const SPECIAL_IDS: &[char] = &[
    '\u{F2A5}', '\u{F0CF}', '\u{F2A7}', '\u{F313}', '\u{F2A9}', '\u{F2AA}', '\u{F4F3}', '\u{F2AB}',
    '\u{F2AC}', '\u{F2A8}', '\u{F2AD}', '\u{F2AE}', '\u{F29F}', '\u{F2B0}', '\u{F2B1}', '\u{F31A}',
    '\u{F2B3}', '\u{F2B4}', '\u{F2B5}', '\u{F5EE}', '\u{F101}', '\u{F2E6}', '\u{F2E9}', '\u{F2F2}',
    '\u{F2F4}', '\u{F2BA}', '\u{F2BB}', '\u{F2BC}', '\u{F2BD}', '\u{F2F5}', '\u{F2BF}', '\u{F5FD}',
    '\u{F2C1}', '\u{F2B2}', '\u{F2C3}', '\u{F2C4}', '\u{F2B6}', '\u{F2B9}', '\u{F2C7}', '\u{F2C8}',
    '\u{F2C6}', '\u{F2D4}', '\u{F2A4}', '\u{F2C9}', '\u{F5FB}', '\u{EC30}', '\u{F2CA}', '\u{F5FC}',
    '\u{F2D2}', '\u{F319}', '\u{F2CB}', '\u{F2E8}', '\u{F2E7}', '\u{F2FA}', '\u{F2D0}', '\u{F2CF}',
    '\u{F2CE}', '\u{F2CD}', '\u{F13C}', '\u{F5F6}', '\u{F5F0}', '\u{F13D}', '\u{F2BE}', '\u{F314}',
    '\u{F318}', '\u{F31B}', '\u{F2F7}', '\u{F5F4}', '\u{F5F5}', '\u{F2D1}', '\u{F5FE}', '\u{F5FF}',
    '\u{ECE0}', '\u{ECE1}', '\u{ECE2}', '\u{ECE3}', '\u{ECE4}', '\u{ECE5}', '\u{F5E1}', '\u{ECE6}',
    '\u{ECE7}', '\u{F2E3}', '\u{ECE9}', '\u{ECEA}', '\u{ECEB}', '\u{ECEC}', '\u{ECED}', '\u{ECEE}',
    '\u{F5F7}', '\u{F5F8}', '\u{F5F9}', '\u{F5FA}', '\u{ECEF}', '\u{ECF0}', '\u{ECF1}', '\u{ECF2}',
    '\u{ECF3}', '\u{F5DE}', '\u{ECF5}', '\u{F3BE}', '\u{ECF6}', '\u{ECF7}', '\u{ECF8}', '\u{ECF9}',
    '\u{ECFA}', '\u{ECFB}', '\u{ECFC}', '\u{ECFD}', '\u{ECFE}', '\u{F29E}', '\u{EF31}', '\u{EF32}',
    '\u{EF33}', '\u{EF34}', '\u{EF35}', '\u{EF36}', '\u{EF37}', '\u{ECFF}', '\u{F2E5}', '\u{F2CC}',
    '\u{F5DF}',
];

#[inline]
pub fn is_special(c: &char) -> bool {
    SPECIAL_IDS.contains(c)
}
