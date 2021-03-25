//! Identifies a `char` literal.

/// Identifies a `char` literal.
/// 
/// ### Arguments
/// * `raw` The original Rust code, assumed to conform to the 2018 edition
/// * `pos` The character position in `raw` to look at
/// 
/// ### Returns
/// @TODO document what this function returns
pub fn identify_character(raw: &str, pos: usize) -> usize {
    let len = raw.len();
    // Avoid panicking, if there would not be enough room for a char.
    if len < pos + 3 { return pos }
    // If the current char is not a single-quote, then it does not begin a char.
    if &raw[pos..pos+1] != "'" { return pos }
    // If this is a simple non-backslashed char, advance three places.
    if &raw[pos+1..pos+2] != "\\" && &raw[pos+2..pos+3] == "'" { return pos + 3 }
    // If the char after the backslash is...
    match &raw[pos+2..pos+3] {
        // ...one of Rust’s simple backslashed chars:
        "n" | "r" | "t" | "\\" | "0" | "\"" | "'" =>
            // Advance four places if the char after that is a single-quote.
            return pos + if &raw[pos+3..pos+4] == "'" { 4 } else { 0 },
        // Otherwise `pos` does not begin a char.
        _ => return pos
    }
    //@TODO 7-bit character code (exactly 2 digits, up to 0x7F), eg '\x41'
    //@TODO 24-bit Unicode character code (up to 6 digits), eg '\u{7FFF}'
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identify_character_f() {
        let raw = "abcde'f'ghi";
        assert_eq!(identify_character(&raw, 4), 4); // e'f
        assert_eq!(identify_character(&raw, 5), 8); // 'f' advance three places
        assert_eq!(identify_character(&raw, 6), 6); // f'g
        assert_eq!(identify_character(&raw, 7), 7); // 'gh
    }

    #[test]
    fn identify_character_newline() {
        let raw = " -'\\n'- ";
        assert_eq!(identify_character(&raw, 1), 1); // -'\n
        assert_eq!(identify_character(&raw, 2), 6); // '\n' advance four places
        assert_eq!(identify_character(&raw, 3), 3); // \n'-
    }

    #[test]
    fn identify_character_backslashed_z() {
        let raw = " -'\\z'- ";
        assert_eq!(identify_character(&raw, 1), 1); // -'\z
        assert_eq!(identify_character(&raw, 2), 2); // '\z' no such char
        assert_eq!(identify_character(&raw, 3), 3); // \z'-
    }

    #[test]
    fn identify_character_near_end() {
        let raw = "xy'z";
        assert_eq!(identify_character(&raw, 1), 1); // y'z
        assert_eq!(identify_character(&raw, 2), 2); // 'z
    }
}
