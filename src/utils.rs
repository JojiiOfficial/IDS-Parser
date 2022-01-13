/// Converts an String encoded utf8-string (eg: `U+9AD8`) to the representing character
#[inline]
pub fn utf_code_to_char(code: &str) -> Option<char> {
    char::from_u32(u32::from_str_radix(code.split('+').nth(1)?, 16).ok()?)
}
