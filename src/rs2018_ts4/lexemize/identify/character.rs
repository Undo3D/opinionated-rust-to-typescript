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
            pos +
                if len >= pos + 4
                && &raw[pos+3..pos+4] == "'"
                { 4 } else { 0 },
        // ...lowercase x, signifying a 7-bit char code:
        "x" =>
            // Advance 6 places if the chars after that are 0-7 and 0-9A-Fa-f.
            pos +
                if len >= pos + 6
                && raw[pos+3..pos+4].chars().all(|c| c >= '0' && c <= '7')
                && raw[pos+4..pos+5].chars().all(|c| c.is_ascii_hexdigit())
                && &raw[pos+5..pos+6] == "'"
                { 6 } else { 0 },
        // ...lowercase u, signifying a unicode char code:
        "u" =>
            // Advance to the position after the closing single-quote, if valid.
            pos + identify_unicode_char_length(raw, pos, len),
        // Otherwise `pos` does not begin a char.
        _ => pos
    }
}

// 24-bit Unicode character code, 1 to 6 digits, eg '\u{f}' to '\u{10abCD}'.
fn identify_unicode_char_length(raw: &str, pos: usize, len: usize) -> usize {
    // If `raw` is not even long enough for the shortest form, '\u{0}',
    // or if the "'\u" is not followed by an open curly bracket, return zero.
    if len < pos + 7 || &raw[pos+3..pos+4] != "{" { return 0 }
    // Initialise variables which will be modified by the loop, below.
    let mut found_closing_curly_bracket = false;
    let mut codepoint = "".to_string();
    // Loop through the characters after "'\u{", to a maximum "'\u{123456}".
    for i in 4..11 {
        // If `raw` is too short to read `&raw[pos+i..pos+i+1]`, return zero.
        if len <= pos + i { return 0 }
        let c = &raw[pos+i..pos+i+1];
        if c == "}" { found_closing_curly_bracket = true; break }
        // If the current character is 0-9A-Fa-f, append it to `codepoint`.
        if c.chars().all(|c| c.is_ascii_hexdigit()) {
            codepoint.push_str(c)
        } else {
            return 0
        }
    }
    // Guard against an overlong unicode escape. Must have at most 6 hex digits.
    if ! found_closing_curly_bracket { return 0 }
    // Get the position of the character which should be a closing single-quote.
    let l = codepoint.len() + 5;
    // If `raw` is too short to read `&raw[pos+l..pos+l+1]`, or if that char
    // is not a single-quote, return zero.
    if len <= pos + l || &raw[pos+l..pos+l+1] != "'" { return 0 }
    // Parse the codepoint into a number.
    match u32::from_str_radix(&codepoint, 16) {
        Err(_) => 0, // unreachable
        // Unicode escapes must be at most 10FFFF. If it’s not above that,
        // return the position after the closing single-quote.
        Ok(value) => if value > 0x10FFFF { 0 } else { l + 1 },
    }
}


#[cfg(test)]
mod tests {
    use super::identify_character as identify;

    #[test]
    fn identify_character_basic() {
        let raw = "abcde'f'ghi";
        assert_eq!(identify(&raw, 4), 4); // e'f
        assert_eq!(identify(&raw, 5), 8); // 'f' advance three places
        assert_eq!(identify(&raw, 6), 6); // f'g
        assert_eq!(identify(&raw, 7), 7); // 'gh
    }

    #[test]
    fn identify_character_backslashed() {
        let raw = " -'\\n'- ";
        assert_eq!(identify(&raw, 1), 1); // -'\n
        assert_eq!(identify(&raw, 2), 6); // '\n' advance four places
        assert_eq!(identify(&raw, 3), 3); // \n'-
        assert_eq!(identify("'\\r'", 0), 4); // '\r'
        assert_eq!(identify("'\\t' ", 0), 4); // '\t'
        assert_eq!(identify("'\\\\'", 0), 4); // '\\'
        assert_eq!(identify(" '\\0'", 1), 5); // '\0'
        assert_eq!(identify("'\\\"'", 0), 4); // '\"'
        assert_eq!(identify("'\\''", 0), 4); // '\''
    }

    #[test]
    fn identify_character_7_bit_char_code() {
        let raw = "'\\x4A'";
        assert_eq!(identify(&raw, 0), 6); // '\x4A' advance to end
        assert_eq!(identify(&raw, 1), 1); // \x4A'
        assert_eq!(identify(&raw, 5), 5); // '
        let raw = " - '\\x0f' - ";
        assert_eq!(identify(&raw, 3), 9); // '\x0f' advance 6 places
    }

    #[test]
    fn identify_character_unicode() {
        assert_eq!(identify("'\\u{0}'", 0), 7); // '\u{0}'
        assert_eq!(identify(" '\\u{C}'", 1), 8); // '\u{C}'
        assert_eq!(identify("- '\\u{f}'", 2), 9); // '\u{f}'
        assert_eq!(identify("'\\u{00}'", 0), 8); // '\u{00}'
        assert_eq!(identify(" '\\u{bD}'", 1), 9); // '\u{bD}'
        assert_eq!(identify("'\\u{1cF}'", 0), 9); // '\u{1cF}'
        assert_eq!(identify("'\\u{fFfF}'", 0), 10); // '\u{fFfF}'
        assert_eq!(identify(" '\\u{00000}'", 1), 12); // '\u{00000}'
        assert_eq!(identify("'\\u{100abC}'", 0), 12); // '\u{100abC}'
        assert_eq!(identify(" - '\\u{10FFFF}'", 3), 15); // maximum
        let raw = "'\\u{30aF}'";
        assert_eq!(identify(&raw, 0), 10); // '\u{30aF}' advance to end
        assert_eq!(identify(&raw, 1), 1); // \u{30aF}'
        assert_eq!(identify(&raw, 2), 2); // u{30aF}'
    }

    #[test]
    fn identify_character_not_backslashed() {
        assert_eq!(identify("'\\' ", 0), 0); // '\' no char after the \
        assert_eq!(identify(" '\\\\", 1), 1); // '\\ has no end quote
        assert_eq!(identify("'\\q'", 0), 0); // '\q' no such backslash
        assert_eq!(identify(" '\\x'", 1), 1); // '\x' would start 7-bit
        assert_eq!(identify("'\\u'", 0), 0); // '\x' would start unicode
    }

    #[test]
    fn identify_character_not_7_bit_char_code() {
        assert_eq!(identify("'\\x3' - ", 0), 0); // '\x3' has no 2nd digit
        assert_eq!(identify("'\\x3f - ", 0), 0); // '\x3f has no end quote
        assert_eq!(identify("'\\x0G'", 0), 0); // '\x0G' is not valid
        assert_eq!(identify("'\\x81'", 0), 0); // '\x81' is out of range
    }

    #[test]
    fn identify_character_not_unicode() {
        assert_eq!(identify("'\\u{", 0), 0); // raw too short
        assert_eq!(identify("'\\u[0]'", 0), 0); // square not curly
        assert_eq!(identify("'\\u{abcde", 0), 0); // raw too short
        assert_eq!(identify("'\\u{12i4}'", 0), 0); // not a hex digit
        assert_eq!(identify("'\\u{100abCd}'", 0), 0); // too long
        assert_eq!(identify("'\\u{1234}", 0), 0); // raw too short
        assert_eq!(identify("'\\u{1234} ", 0), 0); // no closing quote
        assert_eq!(identify("'\\u{110000}'", 0), 0); // too high
    }

    #[test]
    fn identify_character_near_end_does_not_panic() {
        assert_eq!(identify("'a", 0), 0); // 'a
        assert_eq!(identify("'\\", 0), 0); // '\
        assert_eq!(identify("'\\n", 0), 0); // '\n
        assert_eq!(identify("'\\x", 0), 0); // '\x
        assert_eq!(identify("'\\x4", 0), 0); // '\x4
        assert_eq!(identify("'\\x7f", 0), 0); // '\x7f
        assert_eq!(identify("'\\u", 0), 0); // '\u
        assert_eq!(identify("'\\u{", 0), 0); // '\u{
        assert_eq!(identify("'\\u{0", 0), 0); // '\u{0
        assert_eq!(identify("'\\u{0}", 0), 0); // '\u{0}
        assert_eq!(identify("'\\u{30aF", 0), 0); // '\u{30aF
        assert_eq!(identify("'\\u{30Af}", 0), 0); // '\u{30Af}
    }
}
