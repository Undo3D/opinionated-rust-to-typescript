//! Identifies a multiline or inline comment.

/// Identifies a multiline or inline comment.
/// 
/// ### Arguments
/// * `raw` The original Rust code, assumed to conform to the 2018 edition
/// * `pos` The character position in `raw` to look at
/// 
/// ### Returns
/// If `pos` begins a valid looking comment, `identify_comment()` returns
/// the character position after the comment ends.  
/// Otherwise, `identify_comment()` just returns the `pos` argument.
pub fn identify_comment(raw: &str, pos: usize) -> usize {
    // If the current char is the last or second-from-last in `raw`, it does not
    // begin a comment.
    let len = raw.len();
    if len < pos + 2 { return pos }
    // If the current char is not a forward slash, it does not begin a comment.
    if get_aot(raw, pos) != "/" { return pos }
    // If the next char is:
    match get_aot(raw, pos+1) {
        // Also a forward slash, `pos` could begin an inline comment.
        "/" => identify_inline_comment(raw, pos, len),
        // An asterisk, `pos` could begin a multiline comment.
        "*" => identify_multiline_comment(raw, pos, len),
        // Anything else, `pos` does not begin a comment.
        _ => pos,
    }
}

// Returns the ascii character at a position, or tilde if invalid or non-ascii.
fn get_aot(raw: &str, pos: usize) -> &str { raw.get(pos..pos+1).unwrap_or("~") }

fn identify_inline_comment(raw: &str, pos: usize, len: usize) -> usize {
    // Step through each char, from `pos + 2` to the end of the input code.
    let mut i = pos + 2;
    while i < len - 1 {
        // Get this character, even if it’s non-ascii.
        let mut j = i + 1;
        while !raw.is_char_boundary(j) { j += 1 }
        // If this char is a newline:
        if &raw[i..j] == "\n" { //@TODO maybe recognise Windows style "\r\n"?
            // Advance to the start of the newline.
            return i
        }
        // Step forward, ready for the next iteration.
        i = j;
    }
    // No newline was found, so advance to the end of the input code.
    len
}

fn identify_multiline_comment(raw: &str, pos: usize, len: usize) -> usize {
    // Track how deep into a nested multiline comment we are.
    let mut depth = 0;
    // Slightly hacky way to to skip forward while looping.
    let mut i = pos + 2;
    // Step through each char, from `pos` to the end of the raw input code.
    while i < len {
        // Get this character, even if it’s non-ascii.
        let mut j = i + 1;
        while !raw.is_char_boundary(j) { j += 1 }
        let c0 = &raw[i..j];
        // Get the next character, or tilde if it’s non-ascii.
        let c1 = get_aot(raw, j);
        // If this char is an asterisk, and the next is a forward slash:
        if c0 == "*" && c1 == "/" {
            // If the depth is zero (so we are at the outermost nesting level):
            if depth == 0 {
                // Advance to the end of the "*/".
                return i + 2
            // Otherwise we are some way inside a nested multiline comment:
            } else {
                // Decrement the nesting-depth.
                depth -= 1;
                // Skip the forward slash (avoids confusion in "/*/* */* */").
                j += 1;
            }
        // If this char is a forward slash, and the next is an asterisk:
        } else if c0 == "/" && c1 == "*" {
            // Increment the nesting-depth.
            depth += 1;
            // Skip the asterisk (avoids confusion in "/*/*/ */ */").
            j += 1;
        }
        // Step forward, ready for the next iteration.
        i = j;
    }
    // The outermost "*/" was not found, so this is not a multiline comment.
    pos
}


#[cfg(test)]
mod tests {
    use super::identify_comment as identify;

    #[test]
    fn identify_comment_inline() {
        // With newline.
        let raw = "abc//ok\nxyz";
        assert_eq!(identify(raw, 2), 2); // c//o
        assert_eq!(identify(raw, 3), 7); // //ok advance four places
        assert_eq!(identify(raw, 4), 4); // /ok<NL>
        // Without newline.
        let raw = "abc//okxyz";
        assert_eq!(identify(raw, 2), 2);  // c//o
        assert_eq!(identify(raw, 3), 10); // //okxyz advance to the end
        assert_eq!(identify(raw, 4), 4);  // /okxyz
        // With Windows line ending. The carriage return, '\r ', is treated like
        // any other character.
        let raw = "abc//ok\r\nxyz";
        assert_eq!(identify(raw, 2), 2); // c//ok
        assert_eq!(identify(raw, 3), 8); // //ok<CR> advance five places
        assert_eq!(identify(raw, 4), 4); // /ok<CR><NL>
        // Non-ascii.
        assert_eq!(identify("//€", 0), 5); // 3-byte non-ascii directly after //
        assert_eq!(identify("//abcd€", 0), 9); // 3-byte non-ascii after //abcd
    }

    #[test]
    fn identify_comment_multiline_basic() {
        // Contains newline.
        let raw = "abc/*ok\n*/z";
        assert_eq!(identify(raw, 2), 2);  // c/*ok<NL>*
        assert_eq!(identify(raw, 3), 10); // /*ok<NL>*/ adv. seven places
        assert_eq!(identify(raw, 4), 4);  // *ok<NL>*/z
        // Doc.
        assert_eq!(identify("/** Here's a doc */", 0), 19);
        assert_eq!(identify("/**A/*A*/*/", 0), 11);
        assert_eq!(identify("/**A/*A'*/*/", 0), 12);
        // To end of `raw`.
        let raw = "abc/*ok*/";
        assert_eq!(identify(raw, 2), 2); // c/*ok*/
        assert_eq!(identify(raw, 3), 9); // /*ok*/ advance to the end
        assert_eq!(identify(raw, 4), 4); // *ok*/
        // Minimal.
        let raw = "//";
        assert_eq!(identify(raw, 0), 2);  // //
        assert_eq!(identify(raw, 1), 1);  // /
        let raw = "//\n";
        assert_eq!(identify(raw, 0), 3);  // //<NL>
        assert_eq!(identify(raw, 1), 1);  // /<NL>
        let raw = "/**/";
        assert_eq!(identify(raw, 0), 4);  // /**/
        assert_eq!(identify(raw, 1), 1);  // **/
        // Without end.
        let raw = "abc/*nope*";
        assert_eq!(identify(raw, 2), 2); // c/*nope*
        assert_eq!(identify(raw, 3), 3); // /*nope* malformed
        assert_eq!(identify(raw, 4), 4); // *nope*
    }
  
    #[test]
    fn identify_comment_multiline_nested() {
        // Single nesting.
        let raw = "/* outer /* inner */ outer */";
        assert_eq!(identify(raw, 0), 29); // does not end after ...inner */
        assert_eq!(identify(raw, 9), 20); // just catched /* inner */
        // Complex nesting.
        let raw = "pre-/* 0 /* 1 */ 0 /* 2 /* 3 */ 2 */ 0 */-post";
        assert_eq!(identify(raw, 3), 3);  // -/* 0
        assert_eq!(identify(raw, 4), 41); // /* 0 ... 0 */
        assert_eq!(identify(raw, 5), 5);  // * 0
        assert_eq!(identify(raw, 9), 16); // /* 1 */
        assert_eq!(identify(raw, 19), 36); // /* 2 /* 3 */ 2 */
        // `identify_comment()`’s loop deals with these edge cases correctly, by
        // stepping forward one extra pos after finding a nested "/*" or "*/".
        let raw = "/*/*/ */ */";
        assert_eq!(identify(raw, 0), 11); // /*/*/ */ */ edge case is the 3rd /
        assert_eq!(identify(raw, 1), 1);  // */*/ */ */
        assert_eq!(identify(raw, 2), 8);  // /*/ */
        let raw = "/*/* */* */";
        assert_eq!(identify(raw, 0), 11); // /*/* */* */ edge case is the 4th *
        assert_eq!(identify(raw, 1), 1);  // */* */* */
        assert_eq!(identify(raw, 2), 7);  // /* */
        // Invalid nesting.
        let raw = "/* outer /* inner */ missing trailing slash *";
        assert_eq!(identify(raw, 0), 0);
    }

    #[test]
    fn identify_comment_will_not_panic() {
        // Near the end of `raw`.
        assert_eq!(identify("", 0), 0); // empty string
        assert_eq!(identify("/", 0), 0); // /
        assert_eq!(identify("xyz/", 3), 3); // /
        assert_eq!(identify("*", 0), 0); // *
        assert_eq!(identify("//", 0), 2); // //
        assert_eq!(identify("//\n", 0), 3); // //<NL>
        assert_eq!(identify("//abc", 0), 5); // //abc
        assert_eq!(identify("//abc\n", 0), 6); // //abc<NL>
        assert_eq!(identify("/*", 0), 0); // /*
        assert_eq!(identify("*/", 0), 0); // */
        assert_eq!(identify("/**/", 0), 4); // /**/
        assert_eq!(identify("/*abc", 0), 0); // /*abc
        assert_eq!(identify("/*abc*", 0), 0); // /*abc*
        assert_eq!(identify("/*abc*/", 0), 7); // /*abc*/
        assert_eq!(identify("/*abc*/\n", 0), 7); // /*abc*/<NL>
        assert_eq!(identify("/*abc\n*/", 0), 8); // /*abc<NL>*/
        // Invalid `pos`.
        assert_eq!(identify("abc", 2), 2); // 2 is before "c", so in range
        assert_eq!(identify("abc", 3), 3); // 3 is after "c", so incorrect
        assert_eq!(identify("abc", 4), 4); // 4 is out of range
        assert_eq!(identify("abc", 100), 100); // 100 is way out of range
        // Non-ascii.
        assert_eq!(identify("€", 1), 1); // part way through the three eurobytes
        assert_eq!(identify("/€", 0), 0); // non-ascii after /
        assert_eq!(identify("/*€", 0), 0); // non-ascii after /*
    }
  
}
