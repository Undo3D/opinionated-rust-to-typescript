//! Identifies an identifier, like `String` or `foo_bar`.

/// Identifies an identifier, like `String` or `foo_bar`.
/// 
/// ### Arguments
/// * `raw` The original Rust code, assumed to conform to the 2018 edition
/// * `pos` The character position in `raw` to look at
/// 
/// ### Returns
/// @TODO document what this function returns
pub fn identify_identifier(raw: &str, pos: usize) -> usize {
    // If the current char is past the last char in `raw`, bail out!
    let len = raw.len();
    if pos >= len { return pos }
    // If the current char is not [_a-zA-Z], it does not begin an identifier.
    let c = &raw[pos..pos+1];
    let starts_u = c == "_"; // true if the current char is an underscore
    if ! starts_u && ! c.chars().all(char::is_alphabetic) { return pos }
    // If the current char is the last in the input code:
    if len == pos + 1 {
        // A lone "_" is not an identifier, but anything ascii-alphabetic is.
        return if starts_u { pos } else { len }
    }
    // If the next char is not an underscore, letter or digit:
    let c = &raw[pos+1..pos+2];
    if c != "_" && ! c.chars().all(char::is_alphanumeric) {
        // A lone "_" is not an identifier. Else, advance after the first char.
        return if starts_u { pos } else { pos + 1 }
    }
    // Step through each char, from `pos` to the end of the input code.
    for i in pos+2..len-1 {
        let c = &raw[i..i+1];
        // If this char is not an underscore, letter or digit, advance to here.
        if c != "_" && ! c.chars().all(char::is_alphanumeric) { return i }
    }
    // The last char in the input code is a valid identifier.
    len
}


#[cfg(test)]
mod tests {
    use super::identify_identifier as identify;
    
    #[test]
    fn identify_identifier_short() {
        let raw = "abc^_def,G_h__1_; _123e+__ X2 Y Z";
        assert_eq!(identify(raw, 0), 3);   // abc
        assert_eq!(identify(raw, 1), 3);   // bc
        assert_eq!(identify(raw, 2), 3);   // c
        assert_eq!(identify(raw, 3), 3);   // ^ is invalid in identifiers
        assert_eq!(identify(raw, 4), 8);   // _def
        assert_eq!(identify(raw, 8), 8);   // , is invalid in identifiers
        assert_eq!(identify(raw, 9), 16);  // G_h__1_
        assert_eq!(identify(raw, 18), 23); // _123e
        assert_eq!(identify(raw, 24), 26); // __
        assert_eq!(identify(raw, 27), 29); // X2
        assert_eq!(identify(raw, 30), 31); // Y
        assert_eq!(identify(raw, 32), 33); // Z
    }

    #[test]
    fn identify_identifier_short_invalid() {
        // Here, each lone "_" exercises a different conditional branch.
        let raw = "_ 2X _";
        assert_eq!(identify(raw, 0), 0); // _ cannot be the only char
        assert_eq!(identify(raw, 2), 2); // 2X is not a valid identifier
        assert_eq!(identify(raw, 5), 5); // _ cannot be the only char
    }

}