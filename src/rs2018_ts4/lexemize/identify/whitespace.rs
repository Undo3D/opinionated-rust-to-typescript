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
/// @TODO document what this function returns
pub fn identify_whitespace(raw: &str, pos: usize) -> usize {
    // If the current char is past the last char in `raw`, bail out!
    let len = raw.len();
    if pos >= len { return pos }
    // Step through each char, from `pos` to the end of the input code.
    for i in pos..len {
        let c = &raw[i..i+1];
        if c != "\u{0009}" // horizontal tab, "\t"
        && c != "\u{000A}" // line feed, "\n"
        && c != "\u{000B}" // vertical tab
        && c != "\u{000C}" // form feed
        && c != "\u{000D}" // carriage return, "\r"
        && c != "\u{0020}" // space, " "
        && c != "\u{0085}" // next line
        && c != "\u{200E}" // left-to-right mark
        && c != "\u{200F}" // right-to-left mark
        && c != "\u{2028}" // line separator
        && c != "\u{2029}" // paragraph separator
        {
            // Advance to the character after the final whitespace.
            return i
        }
    }
    // Advance to the end of the input code.
    len
}


#[cfg(test)]
mod tests {
    use super::identify_whitespace as identify;

    #[test]
    fn identify_whitespace_typical() {
        let raw = "abc \t\nxyz";
        assert_eq!(identify(raw, 2), 2); // c
        assert_eq!(identify(raw, 3), 6); // <SP><TB><NL> advance three spaces
        assert_eq!(identify(raw, 4), 6); // <TB><NL> advance two spaces
        assert_eq!(identify(raw, 5), 6); // <NL> advance one space
        assert_eq!(identify(raw, 6), 6); // xyz
    }

    #[test]
    fn identify_whitespace_exhaustive() {
        //doc.rust-lang.org/reference/whitespace.html
        assert_eq!(identify("\0",       0), 0); // null is not whitespace
        assert_eq!(identify("\u{0009}", 0), 1); // horizontal tab to U+2029
        assert_eq!(identify("\u{000A}", 0), 1); // line feed to U+2029
        assert_eq!(identify("\u{000B}", 0), 1); // vertical tab to U+2029
        assert_eq!(identify("\u{000C}", 0), 1); // form feed to U+2029
        assert_eq!(identify("\u{000D}", 0), 1); // carriage return to U+2029
        assert_eq!(identify("\u{0020}", 0), 1); // space to U+2029
        // assert_eq!(identify("\u{0085}", 0), 1); // next line to U+2029
        // assert_eq!(identify("\u{200E}", 0), 1); // left-to-right to U+2029
        // assert_eq!(identify("\u{200F}", 0), 1); // right-to-left to U+2029
        // assert_eq!(identify("\u{2028}", 0), 1); // line separator to U+2029
        // assert_eq!(identify("\u{2029}", 0), 1); // just paragraph separator
        // assert_eq!(identify("\u{00A0}", 0), 0); // NBSP is not whitespace
    }

    #[test]
    fn identify_whitespace_ends_with_newline() {
        let raw = "xyz. \n";
        assert_eq!(identify(raw, 2), 2); // z. <NL>
        assert_eq!(identify(raw, 3), 3); // . <NL>
        assert_eq!(identify(raw, 4), 6); //  <NL> advance to eoi
        assert_eq!(identify(raw, 5), 6); // <NL> advance to eoi
    }

}
