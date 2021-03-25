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
    // If the current char is the last in `raw`, it does not begin a comment.
    let len = raw.len();
    if len < pos + 2 { return pos }
    // If the current char is not a forward slash, it does not begin a comment.
    if &raw[pos..pos+1] != "/" { return pos }
    // If the next char is:
    match &raw[pos+1..pos+2] {
        // Also a forward slash, `pos` begins an inline comment.
        "/" => identify_inline_comment(raw, pos, len),
        // An asterisk, `pos` begins a multiline comment.
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
    // Step through each char, from `pos` to the end of the raw input code.
    // `len-1` saves a nanosecond or two, but also prevents `raw[i..i+1]` from
    // panicking when `raw` ends in an asterisk.
    for i in pos+2..len-1 {
        // If this char is an asterisk, and the next is a forward slash:
        if &raw[i..i+1] == "*" && &raw[i+1..i+2] == "/" {
            // Advance to the end of the "*/".
            return i + 2
        }
    }
    // "*/" was not found, so this is not a multiline comment.
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
    fn identify_comment_multiline_to_end() {
        let raw = "abc/*ok*/";
        assert_eq!(identify(raw, 2), 2); // c/*ok*/
        assert_eq!(identify(raw, 3), 9); // /*ok*/ advance to the end
        assert_eq!(identify(raw, 4), 4); // *ok*/
    }
  
    #[test]
    fn identify_comment_minimal() {
        let raw = "//";
        assert_eq!(identify(raw, 0), 2); // //
        assert_eq!(identify(raw, 1), 1); // /
        let raw = "/**/";
        assert_eq!(identify(raw, 0), 4); // /**/
        assert_eq!(identify(raw, 1), 1); // **/
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