/// Converts one of those unencoded special characters
pub fn conv_special(nr: u8) -> Option<char> {
    // SPECIAL_IDS.get(nr as usize - 1).copied()
    let e = match nr {
        01 => '\u{F2A5}',
        02 => '\u{F2A6}',
        03 => '\u{F2A7}',
        04 => '\u{F313}',
        05 => '\u{F2A9}',
        06 => '\u{F2AA}',
        07 => '\u{F4F3}',
        08 => '\u{F2AB}',
        09 => '\u{F2AC}',
        10 => '\u{F2A8}',
        11 => '\u{F2AD}',
        12 => '\u{F2AE}',
        13 => '\u{F29F}',
        14 => '\u{F2B0}',
        15 => '\u{F2B1}',
        16 => '\u{F31A}',
        17 => '\u{F2B3}',
        18 => '\u{F2B4}',
        19 => '\u{F2B5}',
        20 => '\u{F315}',
        21 => '\u{F101}',
        22 => '\u{F2E6}',
        23 => '\u{F2E9}',
        24 => '\u{F2F2}',
        25 => '\u{F2F4}',
        26 => '\u{F2BA}',
        27 => '\u{F2BB}',
        28 => '\u{F2BC}',
        29 => '\u{F2BD}',
        30 => '\u{F2F5}',
        31 => '\u{F2BF}',
        32 => '\u{F2C0}',
        33 => '\u{F2C1}',
        35 => '\u{F2C3}',
        36 => '\u{F2C4}',
        39 => '\u{F2C7}',
        40 => '\u{F2C8}',
        41 => '\u{F2D3}',
        42 => '\u{F2D4}',
        43 => '\u{F2A4}',
        47 => '\u{F2CA}',
        49 => '\u{F2D2}',
        50 => '\u{F319}',
        51 => '\u{F2CB}',
        52 => '\u{F2E8}',
        53 => '\u{F2E7}',
        54 => '\u{F2FA}',
        55 => '\u{F2D0}',
        56 => '\u{F2CF}',
        57 => '\u{F2CE}',
        58 => '\u{F2CD}',
        59 => '\u{F13C}',
        60 => '\u{F5F6}',
        61 => '\u{F5F0}',
        62 => '\u{F13D}',
        63 => '\u{F2BE}',
        67 => '\u{F2F7}',
        68 => '\u{F5F4}',
        69 => '\u{F5F5}',
        70 => '\u{F2D1}',
        _ => return None,
    };
    Some(e)
}

/// Sorted list of special ids.
pub const SPECIAL_IDS: &[char] = &[
    '\u{f101}', '\u{f13c}', '\u{f13d}', '\u{f29f}', '\u{f2a4}', '\u{f2a5}', '\u{f2a6}', '\u{f2a7}',
    '\u{f2a8}', '\u{f2a9}', '\u{f2aa}', '\u{f2ab}', '\u{f2ac}', '\u{f2ad}', '\u{f2ae}', '\u{f2b0}',
    '\u{f2b1}', '\u{f2b3}', '\u{f2b4}', '\u{f2b5}', '\u{f2ba}', '\u{f2bb}', '\u{f2bc}', '\u{f2bd}',
    '\u{f2be}', '\u{f2bf}', '\u{f2c0}', '\u{f2c1}', '\u{f2c3}', '\u{f2c4}', '\u{f2c7}', '\u{f2c8}',
    '\u{f2ca}', '\u{f2cb}', '\u{f2cd}', '\u{f2ce}', '\u{f2cf}', '\u{f2d0}', '\u{f2d1}', '\u{f2d2}',
    '\u{f2d3}', '\u{f2d4}', '\u{f2e6}', '\u{f2e7}', '\u{f2e8}', '\u{f2e9}', '\u{f2f2}', '\u{f2f4}',
    '\u{f2f5}', '\u{f2f7}', '\u{f2fa}', '\u{f313}', '\u{f315}', '\u{f319}', '\u{f31a}', '\u{f4f3}',
    '\u{f5f0}', '\u{f5f4}', '\u{f5f5}', '\u{f5f6}',
];

#[inline]
pub fn is_special(c: &char) -> bool {
    // its sorted
    SPECIAL_IDS.binary_search(c).is_ok()
}
