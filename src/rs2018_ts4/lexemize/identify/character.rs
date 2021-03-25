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
            return pos +
                if len >= pos + 4
                && &raw[pos+3..pos+4] == "'"
                { 4 } else { 0 },
        // ...lowercase x, signifying a 7-bit char code:
        "x" =>
            // Advance six places if the chars after that are 0-7 and 0-9A-F.
            return pos +
                if len >= pos + 5
                && raw[pos+3..pos+4].chars().all(|c| c >= '0' && c <= '7')
                && raw[pos+4..pos+5].chars().all(|c| c.is_ascii_hexdigit())
                && &raw[pos+5..pos+6] == "'"
                { 6 } else { 0 },
        // Otherwise `pos` does not begin a char.
        _ => return pos
    }
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
    fn identify_character_backslashed() {
        let raw = " -'\\n'- ";
        assert_eq!(identify_character(&raw, 1), 1); // -'\n
        assert_eq!(identify_character(&raw, 2), 6); // '\n' advance four places
        assert_eq!(identify_character(&raw, 3), 3); // \n'-
        assert_eq!(identify_character("'\\r'", 0), 4); // '\r'
        assert_eq!(identify_character("'\\t' ", 0), 4); // '\t'
        assert_eq!(identify_character("'\\\\'", 0), 4); // '\\'
        assert_eq!(identify_character(" '\\0'", 1), 5); // '\0'
        assert_eq!(identify_character("'\\\"'", 0), 4); // '\"'
        assert_eq!(identify_character("'\\''", 0), 4); // '\''
    }

    #[test]
    fn identify_character_7_bit_char_code() {
        let raw = "'\\x4A'";
        assert_eq!(identify_character(&raw, 0), 6); // '\x4A' advance to end
        assert_eq!(identify_character(&raw, 1), 1); // \x4A'
        assert_eq!(identify_character(&raw, 5), 5); // '
        let raw = " - '\\x0f' - ";
        assert_eq!(identify_character(&raw, 3), 9); // '\x0f' advance 6 places
    }

    #[test]
    fn identify_character_not_backslashed() {
        assert_eq!(identify_character("'\\' ", 0), 0); // '\' no char after the \
        assert_eq!(identify_character(" '\\\\", 1), 1); // '\\ has no end quote
        assert_eq!(identify_character("'\\q'", 0), 0); // '\q' no such backslash
        assert_eq!(identify_character(" '\\x'", 1), 1); // '\x' would start 7-bit
        assert_eq!(identify_character("'\\u'", 0), 0); // '\x' would start unicode
    }

    #[test]
    fn identify_character_not_a_7_bit_char_code() {
        assert_eq!(identify_character("'\\x3' - ", 0), 0); // '\x3' has no 2nd digit
        assert_eq!(identify_character("'\\x3f - ", 0), 0); // '\x3f has no end quote
        assert_eq!(identify_character("'\\x0G'", 0), 0); // '\x0G' is not valid
        assert_eq!(identify_character("'\\x81'", 0), 0); // '\x81' is out of range
    }

    #[test]
    fn identify_character_near_end_does_not_panic() {
        assert_eq!(identify_character("'a", 0), 0); // 'a
        assert_eq!(identify_character("'\\", 0), 0); // '\
        assert_eq!(identify_character("'\\n", 0), 0); // '\n
        assert_eq!(identify_character("'\\x", 0), 0); // '\x
        assert_eq!(identify_character("'\\x4", 0), 0); // '\x4
    }
}
