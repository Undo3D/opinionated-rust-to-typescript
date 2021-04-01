//! Identifies a char literal, like `'A'` or `\u{03aB}`.

/// Identifies a char literal, like `'A'` or `\u{03aB}`.
/// 
/// @TODO `b` prefix, eg `b'A'`
/// 
/// ### Arguments
/// * `raw` The original Rust code, assumed to conform to the 2018 edition
/// * `pos` The character position in `raw` to look at
/// 
/// ### Returns
/// If `pos` begins a valid looking char literal, `identify_character()`
/// returns the character position after the closing single quote.  
/// Otherwise, `identify_character()` just returns the `pos` argument.
pub fn identify_character(raw: &str, pos: usize) -> usize {
    // Avoid panicking, if there would not be enough room for a char.
    let len = raw.len();
    if len < pos + 3 { return pos } // pos + ' + A + '
    // If the current char is not a single-quote, then it does not begin a char.
    let c0 = get_aot(raw, pos);
    if c0 != "'" { return pos }
    // Get the next char, even if it’s not ascii.
    let mut c1_end = pos + 2;
    while !raw.is_char_boundary(c1_end) { c1_end += 1 }
    // Avoid panicking, if there would not be enough room for a char.
    if len < c1_end + 1 { return pos }
    let c1 = &raw[pos+1..c1_end];
    // If the next char is not a backslash:
    if c1 != "\\" {
        return
            // If `c1` is a single quote:
            if c1 == "'"
                // We have found the string "''", which is not a valid char.
                { pos }
            // Otherwise, if the char directly after `c1` is not a single quote:
            else if get_aot(raw, c1_end) != "'"
                // We have probably found a label, like "'static".
                { pos }
            // Otherwise, this is a valid char literal, like "'A'" or "'±'".
            else { c1_end + 1 }
    }

    // Now we know `c1` is a backslash, if the char after it is...
    match get_aot(raw, pos+2) {
        // ...one of Rust’s simple backslashable chars:
        "n" | "r" | "t" | "\\" | "0" | "\"" | "'" =>
            // Advance four places if the char after that is a single-quote.
            pos +
                if len >= pos + 4
                && get_aot(raw, pos+3) == "'"
                { 4 } else { 0 },
        // ...lowercase x, signifying a 7-bit char code:
        "x" =>
            // Advance 6 places if the chars after that are 0-7 and 0-9A-Fa-f.
            pos +
                if len >= pos + 6
                && get_aot(raw, pos+3).chars().all(|c| c >= '0' && c <= '7')
                && get_aot(raw, pos+4).chars().all(|c| c.is_ascii_hexdigit())
                && get_aot(raw, pos+5) == "'"
                { 6 } else { 0 },
        // ...lowercase u, signifying a unicode char code:
        "u" =>
            // Advance to the position after the closing single-quote, if valid.
            pos + identify_unicode_char_length(raw, pos, len),
        // ...anything else:
        _ =>
            // `pos` does not begin a char.
            pos
    }
}

// Returns the ascii character at a position, or tilde if invalid or non-ascii.
fn get_aot(raw: &str, pos: usize) -> &str { raw.get(pos..pos+1).unwrap_or("~") }

// 24-bit Unicode character code, 1 to 6 digits, eg '\u{f}' to '\u{10abCD}'.
fn identify_unicode_char_length(raw: &str, pos: usize, len: usize) -> usize {
    // If `raw` is not even long enough for the shortest form, '\u{0}',
    // or if the "'\u" is not followed by an open curly bracket, return zero.
    if len < pos + 7 || get_aot(raw, pos+3) != "{" { return 0 }
    // Initialise variables which will be modified by the loop, below.
    let mut found_closing_curly_bracket = false;
    let mut codepoint = "".to_string();
    // Loop through the characters after "'\u{", to a maximum "'\u{123456}".
    for i in 4..11 {
        let c = get_aot(raw, pos+i);
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
    // If that char is not a single-quote, return zero.
    if get_aot(raw, pos+l) != "'" { return 0 }
    // Parse the codepoint into a number.
    match u32::from_str_radix(&codepoint, 16) {
        // This error conditional is actually unreachable, because we used
        // `is_ascii_hexdigit()`, above.
        Err(_) => 0,
        // Unicode escapes must be at most 10FFFF. If it’s not above that,
        // return the position after the closing single-quote.
        Ok(value) => if value > 0x10FFFF { 0 } else { l + 1 },
    }
}


#[cfg(test)]
mod tests {
    use super::identify_character as identify;

    #[test]
    fn get_ascii_or_tilde() {
        // Test the logic of `get_aot()`.
        let raw = "abcd€f";
        assert_eq!(raw.get(0..0+1).unwrap_or("~"), "a");
        assert_eq!(raw.get(1..1+1).unwrap_or("~"), "b");
        assert_eq!(raw.get(4..4+1).unwrap_or("~"), "~"); // start of €
        assert_eq!(raw.get(5..5+1).unwrap_or("~"), "~"); // middle of €
        assert_eq!(raw.get(7..7+1).unwrap_or("~"), "f");
        assert_eq!(raw.get(8..8+1).unwrap_or("~"), "~"); // right on the end
        assert_eq!(raw.get(9..9+1).unwrap_or("~"), "~"); // past the end
    }

    #[test]
    fn identify_character_correct() {
        // Simple ascii char in the middle of other ascii text.
        let raw = "abcde'f'ghi";
        assert_eq!(identify(&raw, 4), 4); // e'f
        assert_eq!(identify(&raw, 5), 8); // 'f' advance three places
        assert_eq!(identify(&raw, 6), 6); // f'g
        assert_eq!(identify(&raw, 7), 7); // 'gh
        // Non-ascii chars in the middle of other non-ascii text.
        // //en.wikipedia.org/wiki/Thousand_Character_Classic
        let raw = "±'±'∆'∆'\u{10FFFF}'\u{10FFFF}'";
        assert_eq!(identify(&raw, 0), 0); // ± is 2 bytes wide
        assert_eq!(identify(&raw, 2), 6); // '±' advance four places
        assert_eq!(identify(&raw, 6), 6); // ∆ is 3 bytes wide
        assert_eq!(identify(&raw, 9), 14); // '∆' advance five places
        assert_eq!(identify(&raw, 14), 14); // \u{10FFFF} is 4 bytes wide
        assert_eq!(identify(&raw, 18), 24); // '\u{10FFFF}' advance five places
        // Simple backslash.
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
        // 7-bit '\x00'.
        let raw = "'\\x4A'";
        assert_eq!(identify(&raw, 0), 6); // '\x4A' advance to end
        assert_eq!(identify(&raw, 1), 1); // \x4A'
        assert_eq!(identify(&raw, 5), 5); // '
        let raw = " - '\\x0f' - ";
        assert_eq!(identify(&raw, 3), 9); // '\x0f' advance 6 places
        // Unicode '\u{0}'.
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
        assert_eq!(identify("'\\u{123}'€", 0), 9); // '\u{123}'
        let raw = "'\\u{30aF}'";
        assert_eq!(identify(&raw, 0), 10); // '\u{30aF}' advance to end
        assert_eq!(identify(&raw, 1), 1); // \u{30aF}'
        assert_eq!(identify(&raw, 2), 2); // u{30aF}'
    }

    #[test]
    fn identify_character_incorrect() {
        // Empty.
        assert_eq!(identify("'' ", 0), 0); // '' missing char
        // Incorrect simple backslash.
        assert_eq!(identify("'\\' ", 0), 0); // '\' no char after the \
        assert_eq!(identify(" '\\\\", 1), 1); // '\\ has no end quote
        assert_eq!(identify("'\\q'", 0), 0); // '\q' no such backslash
        assert_eq!(identify("'\\~'", 0), 0); // '\~' no such backslash
        assert_eq!(identify(" '\\x'", 1), 1); // '\x' would start 7-bit
        assert_eq!(identify("'\\u'", 0), 0); // '\x' would start unicode
        // Incorrect 7-bit '\x00'.
        assert_eq!(identify("'\\x3' - ", 0), 0); // '\x3' has no 2nd digit
        assert_eq!(identify("'\\x3f - ", 0), 0); // '\x3f has no end quote
        assert_eq!(identify("'\\x0G'", 0), 0); // '\x0G' is not valid
        assert_eq!(identify("'\\x81'", 0), 0); // '\x81' is out of range
        // Incorrect Unicode '\u{0}'.
        assert_eq!(identify("'\\uxyz", 0), 0); // missing {0}
        assert_eq!(identify("'\\u{xyz", 0), 0); // missing 0}
        assert_eq!(identify("'\\u{0xyz", 0), 0); // missing }
        assert_eq!(identify("'\\u", 0), 0); // at end, missing {0}
        assert_eq!(identify("'\\u{", 0), 0); // at end, missing 0}
        assert_eq!(identify("'\\u{0", 0), 0); // at end, missing }
        assert_eq!(identify("'\\u[0]'", 0), 0); // square not curly
        assert_eq!(identify("'\\u{abcde", 0), 0); // raw too short
        assert_eq!(identify("'\\u{12i4}'", 0), 0); // not a hex digit
        assert_eq!(identify("'\\u{100abCd}'", 0), 0); // too long
        assert_eq!(identify("'\\u{1234}", 0), 0); // raw too short
        assert_eq!(identify("'\\u{1234} ", 0), 0); // no closing quote
        assert_eq!(identify("'\\u{110000}'", 0), 0); // too high
    }

    #[test]
    fn identify_character_will_not_panic() {
        // Near the end of `raw`.
        assert_eq!(identify("", 0), 0); // empty string
        assert_eq!(identify("'", 0), 0); // '
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
        // Invalid `pos`.
        assert_eq!(identify("abc", 2), 2); // 2 is before "c", so in range
        assert_eq!(identify("abc", 3), 3); // 3 is after "c", so incorrect
        assert_eq!(identify("abc", 4), 4); // 4 is out of range
        assert_eq!(identify("abc", 100), 100); // 100 is way out of range
        // Non-ascii.
        assert_eq!(identify("€", 1), 1); // part way through the three eurobytes
        assert_eq!(identify("'€", 0), 0); // non-ascii after '
        assert_eq!(identify("'\\€", 0), 0); // non-ascii after '\
        assert_eq!(identify("'\\u€'", 0), 0); // non-ascii after '\u
        assert_eq!(identify("'\\u{€'", 0), 0); // non-ascii after '\u{
        assert_eq!(identify("'\\u{123€'", 0), 0); // non-ascii after '\u{123
        assert_eq!(identify("'\\u{123}€'", 0), 0); // non-ascii after '\u{123}
    }

}
