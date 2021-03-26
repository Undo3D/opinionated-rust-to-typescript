//! Identifies a multiline or inline comment.

/// Identifies a multiline or inline comment.
/// 
/// ### Arguments
/// * `raw` The original Rust code, assumed to conform to the 2018 edition
/// * `pos` The character position in `raw` to look at
/// 
/// ### Returns
/// @TODO document what this function returns
pub fn identify_comment(raw: &str, pos: usize) -> usize {
    // If the current char is the last or second-from-last in `raw`, it does not
    // begin a comment. Also, bail out for invalid `pos`.
    let len = raw.len();
    if len < pos + 2 { return pos }
    // If the current char is not a forward slash, it does not begin a comment.
    if &raw[pos..pos+1] != "/" { return pos }
    // If the next char is:
    match &raw[pos+1..pos+2] {
        // Also a forward slash, `pos` could begin an inline comment.
        "/" => identify_inline_comment(raw, pos, len),
        // An asterisk, `pos` could begin a multiline comment.
        "*" => identify_multiline_comment(raw, pos, len),
        // Anything else, `pos` does not begin a comment.
        _ => pos,
    }
}

fn identify_inline_comment(raw: &str, pos: usize, len: usize) -> usize {
    // Step through each char, from `pos` to the end of the raw input code.
    for i in pos+2..len-1 {
        // If this char is a newline:
        if &raw[i..i+1] == "\n" { //@TODO maybe recognise Windows style "\r\n"?
            // Advance to the start of the newline.
            return i
        }
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
    // `len-1` saves a nanosecond or two, but also prevents `raw[i..i+1]` from
    // panicking when `raw` ends in an asterisk.
    while i < len-1 {
        // If this char is an asterisk, and the next is a forward slash:
        if &raw[i..i+1] == "*" && &raw[i+1..i+2] == "/" {
            // If the depth is zero (so we are at the outermost nesting level):
            if depth == 0 {
                // Advance to the end of the "*/".
                return i + 2
            // Otherwise we are some way inside a nested multiline comment:
            } else {
                // Decrement the nesting-depth.
                depth -= 1;
                // Skip the forward slash (avoids confusion in "/*/* */* */").
                i += 1;
            }
        // If this char is a forward slash, and the next is an asterisk:
        } else if &raw[i..i+1] == "/" && &raw[i+1..i+2] == "*" {
            // Increment the nesting-depth.
            depth += 1;
            // Skip the asterisk (avoids confusion in "/*/*/ */ */").
            i += 1;
        }
        // Step forward.
        i += 1;
    }
    // The outermost "*/" was not found, so this is not a multiline comment.
    pos
}


#[cfg(test)]
mod tests {
    use super::identify_comment as identify;

    #[test]
    fn identify_comment_inline_with_newline() {
        let raw = "abc//ok\nxyz";
        assert_eq!(identify(raw, 2), 2); // c//o
        assert_eq!(identify(raw, 3), 7); // //ok advance four places
        assert_eq!(identify(raw, 4), 4); // /ok<NL>
    }
  
    #[test]
    fn identify_comment_inline_without_newline() {
        let raw = "abc//okxyz";
        assert_eq!(identify(raw, 2), 2);  // c//o
        assert_eq!(identify(raw, 3), 10); // //okxyz advance to the end
        assert_eq!(identify(raw, 4), 4);  // /okxyz
    }
  
    #[test]
    fn identify_comment_inline_with_windows_line_ending() {
        // The carriage return, '\r ', is treated like any other character.
        let raw = "abc//ok\r\nxyz";
        assert_eq!(identify(raw, 2), 2); // c//ok
        assert_eq!(identify(raw, 3), 8); // //ok<CR> advance five places
        assert_eq!(identify(raw, 4), 4); // /ok<CR><NL>
    }
  
    #[test]
    fn identify_comment_multiline_with_newline() {
        let raw = "abc/*ok\n*/z";
        assert_eq!(identify(raw, 2), 2);  // c/*ok<NL>*
        assert_eq!(identify(raw, 3), 10); // /*ok<NL>*/ adv. seven places
        assert_eq!(identify(raw, 4), 4);  // *ok<NL>*/z
    }

    #[test]
    fn identify_comment_multiline_doc() {
        assert_eq!(identify("/** Here's a doc */", 0), 19);
        assert_eq!(identify("/**A/*A*/*/", 0), 11);
        // assert_eq!(identify("/**A/*A'*/*/", 0), 12);
    }

    #[test]
    fn identify_comment_multiline_nested_basic() {
        let raw = "/* outer /* inner */ outer */";
        assert_eq!(identify(raw, 0), 29); // does not end after ...inner */
        assert_eq!(identify(raw, 9), 20); // just catched /* inner */
    }

    #[test]
    fn identify_comment_multiline_nested_complex() {
        let raw = "pre-/* 0 /* 1 */ 0 /* 2 /* 3 */ 2 */ 0 */-post";
        assert_eq!(identify(raw, 3), 3);  // -/* 0
        assert_eq!(identify(raw, 4), 41); // /* 0 ... 0 */
        assert_eq!(identify(raw, 5), 5);  // * 0
        assert_eq!(identify(raw, 9), 16); // /* 1 */
        assert_eq!(identify(raw, 19), 36); // /* 2 /* 3 */ 2 */
    }

    #[test]
    fn identify_comment_multiline_nested_edge_cases() {
        // These edge cases are dealt with correctly by stepping forward an
        // extra position after finding a nested "/*" or "*/".
        let raw = "/*/*/ */ */";
        assert_eq!(identify(raw, 0), 11); // /*/*/ */ */ edge case is the 3rd /
        assert_eq!(identify(raw, 1), 1);  // */*/ */ */
        assert_eq!(identify(raw, 2), 8);  // /*/ */
        let raw = "/*/* */* */";
        assert_eq!(identify(raw, 0), 11); // /*/* */* */ edge case is the 4th *
        assert_eq!(identify(raw, 1), 1);  // */* */* */
        assert_eq!(identify(raw, 2), 7);  // /* */
    }

    #[test]
    fn identify_comment_multiline_nested_invalid() {
        let raw = "/* outer /* inner */ missing trailing slash *";
        assert_eq!(identify(raw, 0), 0);
    }

    #[test]
    fn identify_comment_multiline_to_end() {
        let raw = "abc/*ok*/";
        assert_eq!(identify(raw, 2), 2); // c/*ok*/
        assert_eq!(identify(raw, 3), 9); // /*ok*/ advance to the end
        assert_eq!(identify(raw, 4), 4); // *ok*/
    }
  
    #[test]
    fn identify_comment_minimal() {
        let raw = "//";
        assert_eq!(identify(raw, 0), 2);  // //
        assert_eq!(identify(raw, 1), 1);  // /
        let raw = "/**/";
        assert_eq!(identify(raw, 0), 4);  // /**/
        assert_eq!(identify(raw, 1), 1);  // **/
    }

    #[test]
    fn identify_comment_multiline_without_end() {
        let raw = "abc/*nope*";
        assert_eq!(identify(raw, 2), 2); // c/*nope*
        assert_eq!(identify(raw, 3), 3); // /*nope* malformed
        assert_eq!(identify(raw, 4), 4); // *nope*
    }
  
    #[test]
    fn identify_comment_trailing_slash() {
        assert_eq!(identify("xyz/", 3), 3); // should not panic
    }

    #[test]
    fn identify_comment_invalid_pos_doesnt_panic() {
        assert_eq!(identify("abc", 2), 2); // 2 is before "c", so in range
        assert_eq!(identify("abc", 3), 3); // 3 is after "c", so incorrect
        assert_eq!(identify("abc", 4), 4); // 4 is out of range
        assert_eq!(identify("abc", 100), 100); // 100 is way out of range
    }
  
}
