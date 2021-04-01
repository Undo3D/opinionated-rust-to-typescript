//! Identifies whitespace.

/// Identifies whitespace.
/// 
/// Rust uses Pattern_White_Space, and treats it all the same.
/// There is some debate on whether to simplify things, in future:
/// internals.rust-lang.org/t/do-we-need-unicode-whitespace/9876
/// 
/// ### Arguments
/// * `raw` The original Rust code, assumed to conform to the 2018 edition
/// * `pos` The character position in `raw` to look at
/// 
/// ### Returns
/// If `pos` begins a sequence of whitespace characters, `identify_whitespace()`
/// returns the character position after that sequence ends.  
/// Otherwise, `identify_whitespace()` just returns the `pos` argument.
pub fn identify_whitespace(raw: &str, pos: usize) -> usize {
    // If the current char is past the last char in `raw`, or `pos` is not on
    // a character boundary, bail out! The char boundary test avoids a potential
    // panic if `&raw[i..j]` is reached, below.
    let len = raw.len();
    if pos >= len || !raw.is_char_boundary(pos) { return pos }
    // Step through each byte-position, from `pos` to the end of the input code.
    let mut i = pos;
    while i < len {
        // Get the current character if it’s ascii, or get "~" if it’s not.
        let c = get_aot(raw, i);
        // Jump to the next char if this is ascii whitespace.
        if c == " "        // U+0020  UTF-8 20        "Space"
        || c == "\n"       // U+000A  UTF-8 0A        "New Line" or "Line Feed"
        || c == "\t"       // U+0009  UTF-8 09        "Horizontal Tabulation"
        || c == "\r"       // U+000D  UTF-8 0D        "Carriage Return"
        || c == "\u{000B}" // U+000B  UTF-8 0B        "Vertical Tabulation"
        || c == "\u{000C}" // U+000C  UTF-8 0C        "Form Feed"
        { i += 1; continue }
        // End the loop if this is ascii non-whitespace.
        if c != "~" { return i }
        // End the loop if there is no next byte.
        if i >= len - 1 { return i }
        // Get the next character.
        let mut j = i + 1;
        while !raw.is_char_boundary(j) { j += 1 }
        let c = &raw[i..j];
        // End the loop if we encountered a literal tilde.
        if c == "~" { return i }
        // Jump to the next char if this is non-ascii Pattern_White_Space.
        if c == "\u{0085}" // U+0085  UTF-8 C2 85     "Next Line"
        || c == "\u{200E}" // U+200E  UTF-8 E2 80 8E  "Left-To-Right Mark"
        || c == "\u{200F}" // U+200F  UTF-8 E2 80 8F  "Right-To-Left Mark"
        || c == "\u{2028}" // U+2028  UTF-8 E2 80 A8  "Line Separator"
        || c == "\u{2029}" // U+2029  UTF-8 E2 80 A9  "Paragraph Separator"
        { i = j; continue }
        // End the loop if we encountered anything else.
        return i;
    }
    // Advance to the end of the input code.
    len
}

// Returns the ascii character at a position, or tilde if invalid or non-ascii.
fn get_aot(raw: &str, pos: usize) -> &str { raw.get(pos..pos+1).unwrap_or("~") }


#[cfg(test)]
mod tests {
    use super::identify_whitespace as identify;

    #[test]
    fn identify_whitespace_correct() {
        // Typical.
        let raw = "~abc \t\nxyz~";
        assert_eq!(identify(raw, 3), 3); // c
        assert_eq!(identify(raw, 4), 7); // <SP><TB><NL> advance three spaces
        assert_eq!(identify(raw, 5), 7); // <TB><NL> advance two spaces
        assert_eq!(identify(raw, 6), 7); // <NL> advance one space
        assert_eq!(identify(raw, 7), 7); // xyz~

        // Exhaustive.
        //doc.rust-lang.org/reference/whitespace.html
        assert_eq!(identify("\u{0000}", 0), 0); // null is not whitespace
        assert_eq!(identify("\u{0009}", 0), 1); // horizontal tab
        assert_eq!(identify("\u{000A}", 0), 1); // line feed
        assert_eq!(identify("\u{000B}", 0), 1); // vertical tab
        assert_eq!(identify("\u{000C}", 0), 1); // form feed
        assert_eq!(identify("\u{000D}", 0), 1); // carriage return
        assert_eq!(identify("\u{0020}", 0), 1); // space
        assert_eq!(identify("\u{0085}", 0), 2); // next line
        assert_eq!(identify("\u{00A0}", 0), 0); // NBSP is not whitespace
        assert_eq!(identify("\u{200E}", 0), 3); // left-to-right
        assert_eq!(identify("\u{200F}", 0), 3); // right-to-left
        assert_eq!(identify("\u{2028}", 0), 3); // line separator
        assert_eq!(identify("\u{2029}", 0), 3); // just paragraph separator
        let raw = "\u{0000}\u{0009}\u{000A}\u{000B}\u{000C}\u{000D}\u{0020}\u{0085}";
        assert_eq!(identify(&raw, 0), 0); // null is not whitespace
        assert_eq!(identify(&raw, 1), 9); // "next line" is two bytes
        let raw = "\u{00A0}\u{200E}\u{200F}\u{2028}\u{2029}";
        assert_eq!(identify(&raw, 0), 0); // NBSP is not whitespace
        assert_eq!(identify(&raw, 2), 14); // 2 + (4 * 3)

        // Ends with newline.
        let raw = "xyz~ \n";
        assert_eq!(identify(raw, 2), 2); // z~ <NL>
        assert_eq!(identify(raw, 3), 3); // ~ <NL>
        assert_eq!(identify(raw, 4), 6); //  <NL> advance to eoi
        assert_eq!(identify(raw, 5), 6); // <NL> advance to eoi
    }

    #[test]
    fn identify_whitespace_will_not_panic() {
        // Near the end of `raw` input code.
        assert_eq!(identify("", 0), 0); // empty string
        assert_eq!(identify("~", 0), 0); // ~
        assert_eq!(identify("\n", 0), 1); // <NL>
        // Invalid `pos`.
        assert_eq!(identify("abc", 2), 2); // 2 is before "c", so in range
        assert_eq!(identify("abc", 3), 3); // 3 is after "c", so incorrect
        assert_eq!(identify("abc", 4), 4); // 4 is out of range
        assert_eq!(identify("abc", 100), 100); // 100 is way out of range
        let raw = "\u{00A0}\u{200E}\u{200F}\u{2028}\u{2029}";
        identify(&raw, 1); // `pos` is halfway through the two NBSP bytes
        // Non-ascii.
        assert_eq!(identify("€", 1), 1); // part way through the three eurobytes
        assert_eq!(identify(" €", 0), 1); // non-ascii after space
        assert_eq!(identify("\u{2029}€", 0), 3); // non-ascii after U+2029
    }

}
