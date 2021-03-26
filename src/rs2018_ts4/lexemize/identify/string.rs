//! Identifies a string literal, like `"Hello \"Rust\""` or `r#"Hello "Rust""#`.

/// Identifies a string literal, like `"Hello \"Rust\""` or `r#"Hello "Rust""#`.
/// 
/// ### Arguments
/// * `raw` The original Rust code, assumed to conform to the 2018 edition
/// * `pos` The character position in `raw` to look at
/// 
/// ### Returns
/// @TODO document what this function returns
pub fn identify_string(raw: &str, pos: usize) -> usize {
    // If the current char is the last in `raw`, it does not begin a string.
    let len = raw.len();
    if len < pos + 1 { return pos }

    // If the current char is:
    match &raw[pos..pos+1] {
        // A double quote, `pos` could begin a regular string.
        "\"" => identify_regular_string(raw, pos, len),
        // A lowercase "r", `pos` could begin a raw string.
        "r" => identify_raw_string(raw, pos, len),
        // Anything else, `pos` does not begin a string.
        _ => pos,
    }
}

fn identify_regular_string(raw: &str, pos: usize, len: usize) -> usize {
    // Slightly hacky way to to skip forward while looping.
    let mut i = pos + 1;
    // Step through each char, from `pos` to the end of the raw input code.
    // `len-1` saves a nanosecond or two, but also prevents `raw[i..i+1]` from
    // panicking at the end of the input.
    while i < len-1 {
        let c = &raw[i..i+1];
        // If this char is a backslash:
        if c == "\\" {
            // Ignore the next char.
            i += 1
        // If this char is a double quote:
        } else if c == "\"" {
            // Advance to the end of the double quote.
            return i + 1
        }
        // Step forward.
        i += 1;
    }
    // The closing double quote was not found, so this is not a string.
    pos
}

// doc.rust-lang.org/reference/tokens.html#raw-string-literals
fn identify_raw_string(raw: &str, pos: usize, len: usize) -> usize {
    // Slightly hacky way to to skip forward while looping.
    let mut i = pos + 1;
    // Keep track of the number of leading hashes.
    let mut hashes = 0;
    // Keep track of finding the opening and closing double quotes.
    let mut found_opening_dq = false;
    let mut found_closing_dq = false;

    // Step through each char, from `pos` to the end of the raw input code.
    // `len-1` saves a nanosecond or two, but also prevents `raw[i..i+1]` from
    // panicking at the end of the input.
    while i < len {
        let c = &raw[i..i+1];

        // If we have not found the opening double quote yet:
        if ! found_opening_dq {
            // If this is the opening double quote, note that it’s been found.
            if c == "\"" {
                found_opening_dq = true
            // Otherwise, if this is a leading hash, increment the tally.
            } else if c == "#" {
                hashes += 1
            // Anything else is not valid for the start of a raw string.
            } else {
                return pos
            }

        // Otherwise, if we have already found the closing double quote:
        } else if found_closing_dq {
            // If we are not expecting any more hashes:
            if hashes == 0 {
                // This is the end of a valid raw string.
                return i
            // Otherwise, if this is a trailing hash, decrement the tally.
            } else if c == "#" {
                hashes -= 1
            // Anything else is not valid for the end of a raw string.
            } else {
                return pos
            }

        // Otherwise we are inside the main part of the string:
        } else {
            // If this char is a backslash:
            if c == "\\" {
                // Ignore the next char.
                i += 1
            // If this char is a double quote:
            } else if c == "\"" {
                // Note that the closing double quote has been found.
                found_closing_dq = true
            }
        }

        // Step forward.
        i += 1;
    }

    // Reached the end of the `raw` input string. Any leading hashes should have
    // been balanced by trailing hashes.
    if hashes == 0 { i } else { pos }
}


#[cfg(test)]
mod tests {
    use super::identify_string as identify;
    

    #[test]
    fn identify_string_typical() {
        let raw = "abc\"ok\"xyz";
        assert_eq!(identify(raw, 2), 2); // c"ok
        assert_eq!(identify(raw, 3), 7); // "ok" advance four places
        assert_eq!(identify(raw, 4), 4); // ok"x
    }

    #[test]
    fn identify_string_basic_raw() {
        assert_eq!(identify("-r\"ok\"-", 1), 6);
        assert_eq!(identify("r#\"ok\"#", 0), 7);
        assert_eq!(identify("abcr###\"ok\"###xyz", 3), 14);
    }

    #[test]
    fn identify_string_escaped_double_quote() {
        let raw = "a\"b\\\"c\"d";
        assert_eq!(identify(raw, 0), 0); // a"b\"c
        assert_eq!(identify(raw, 1), 7); // "b\"c" advance six places
        assert_eq!(identify(raw, 2), 2); // b\"c"d
        assert_eq!(identify(raw, 3), 3); // \"c"d
        assert_eq!(identify(raw, 4), 7); // "c"d no ‘lookbehind’ happens!
    }

    #[test]
    fn identify_string_escapes() {
        // Valid escapes, regular string.
        let raw = r#"a"\0\\\\\"\\\n"z"#;
        assert_eq!(identify(raw, 0),  0);  // a"\0\\\\\"\\\n"
        assert_eq!(identify(raw, 1),  15); // "\0\\\\\"\\\n"z
        assert_eq!(identify(raw, 2),  2);  // \0\\\\\"\\\n"z
        assert_eq!(identify(raw, 9),  15); // "\\\n"z no ‘lookbehind’s!
        assert_eq!(identify(raw, 14), 14); // "z not a string, has no end
        // Invalid escapes, regular string.
        assert_eq!(identify("\\a\\b\\c", 0), 0); // \a\b\c
        // Valid escapes, raw string.
        assert_eq!(identify("r\"\\0\\n\\t\"", 0), 9); // r"\0\n\t"
        // Invalid escapes, raw string.
        assert_eq!(identify("r#\"\\X\\Y\\Z\"#", 0), 11); // r#"\X\Y\Z"#
    }

    #[test]
    fn identify_string_invalid_raw() {
        assert_eq!(identify("r##X#\" X in leading hashes \"###", 0), 0);
        assert_eq!(identify("r###\" X in trailing hashes \"##X#", 0), 0);
        assert_eq!(identify("r###\" too few trailing hashes \"##", 0), 0);
        assert_eq!(identify("-r###\" no trailing hashes \"-", 1), 1);
    }

}