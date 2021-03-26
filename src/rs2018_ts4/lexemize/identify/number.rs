//! Identifies a number, like `12.34` or `0b100100`.

/// Identifies a number, like `12.34` or `0b100100`.
/// 
/// ### Arguments
/// * `raw` The original Rust code, assumed to conform to the 2018 edition
/// * `pos` The character position in `raw` to look at
/// 
/// ### Returns
/// @TODO document what this function returns
pub fn identify_number(raw: &str, pos: usize) -> usize {
    // If the current char is past the last char in `raw`, bail out!
    let len = raw.len();
    if pos >= len { return pos }
    let c = &raw[pos..pos+1];
    // If the current char is not a digit, then it does not begin a char.
    if c < "0" || c > "9" { return pos }
    // If the digit is the input code’s last character, we’re finished.
    if len == pos + 1 { return len }
    // If the digit at `pos` is not zero, this is a decimal number:
    if c != "0" { return identify_number_decimal(raw, pos, len) }
    // If the digit is zero, and the next char is "b", "x" or "o":
    match &raw[pos+1..pos+2] {
        // Use the binary, hex or octal identifier, as appropriate.
        "b" => identify_number_binary(raw, pos, len),
        "x" => identify_number_hex(raw, pos, len),
        "o" => identify_number_octal(raw, pos, len),
        // Otherwise, this is a decimal number which starts with a zero.
        _ => identify_number_decimal(raw, pos, len),
    }
}

fn identify_number_binary(raw: &str, pos: usize, len: usize) -> usize {
    let mut has_digit = false; // binary literals must have at least one digit
    for i in pos+2..len { // +2, because we already found "0b"
        let c = &raw[i..i+1];
        // If the character is an underscore, do nothing.
        if c == "_" {
        // Otherwise, if this char is a binary digit:
        } else if c == "0" || c == "1" {
            has_digit = true;
        // Otherwise, if this is a digit (can only be 2 to 9, here) or a dot:
        } else if (c >= "0" && c <= "9") || c == "." {
            // Reject the whole of 0b101021, don’t just accept the 0b1010 part.
            // And reject the whole of 0b11.1, don’t just accept the 0b11 part.
            return pos
        } else {
            // Advance to the character after the binary number.
            return if has_digit { i } else { pos }
        }
    }
    // We’ve reached the end of the input string.
    if has_digit { len } else { pos }
}

fn identify_number_decimal(raw: &str, pos: usize, len: usize) -> usize {
    let mut has_dot = false; // decimal literals may have one "."
    let mut has_e = false; // decimal literals may have one "e" or "E"
    let mut pos_dot = 0; // helps detect invalid numbers like "1._2"
    let mut pos_e = 0; // helps detect invalid numbers like "10E2+3" and "10E"
    let mut pos_eu = 0; // helps detect invalid numbers like "10E_"
    let mut pos_s = 0; // helps detect numbers with invalid signs, like "10E+"

    for i in pos+1..len { // +1, because we already found a digit, 0 to 9
        let c = &raw[i..i+1];

        // If the character is an underscore:
        if c == "_" {
            // Reject a number like "1._2", where the "." is followed by "_".
            if has_dot && pos_dot == i { return pos }
            // Guard against a dangling underscore, eg "7.5e_".
            if has_e && pos_e == i { pos_eu = i + 1 }

        // If the previous char was "e" or "E" and this is a "+" or "-":
        } else if has_e && pos_e == i && (c == "+" || c == "-") {
            // Guard against a dangling plus or minus sign, eg "7.5e-".
            pos_s = i + 1

        // If we haven’t found a decimal point yet, and this char is a dot:
        } else if ! has_dot && c == "." {
            // Reject a number like "1e2.3", where the exponent contains a dot.
            if has_e { return pos }
            // Else, record that a dot was found, and the position after it.
            // We are being verbose by setting two variables here, but hopefully
            // it makes the code clearer, and perhaps run a little faster.
            has_dot = true;
            pos_dot = i + 1;

        // If we haven’t found an exponent marker yet, and this is "e" or "E":
        } else if ! has_e && (c == "e" || c == "E") {
            // Record that an "e" or "E" was found, and the position after it.
            has_e = true;
            pos_e = i + 1;

        // Otherwise, if this char is not a digit:
        } else if c < "0" || c > "9" {
            // We’ve reached a char which can’t be part of a valid number.
            // Numbers can’t end "e", "E", "+", "-", "e_" or "E_".
            return if i == pos_e || i == pos_s || i == pos_eu { pos } else { i }
        }
    }

    // We’ve reached the end of the input string.
    // Numbers can’t end "e", "E", "+", "-", "e_" or "E_".
    if len == pos_e || len == pos_s || len == pos_eu { pos } else { len }
}

fn identify_number_hex(raw: &str, pos: usize, len: usize) -> usize {
    let mut has_digit = false; // hex literals must have at least one digit
    for i in pos+2..len { // +2, because we already found "0x"
        let c = &raw[i..i+1];
        // If the character is an underscore, do nothing.
        if c == "_" {
        // Otherwise, if this char is a hex digit 0-9A-Fa-f:
        } else if c.chars().all(|c| c.is_ascii_hexdigit()) {
            has_digit = true;
        // Otherwise, if this char is a point:
        } else if c == "." {
            // Reject the whole of 0xAB.C, don’t just accept the 0xAB part.
            return pos
        } else {
            // Advance to the character after the hex number.
            return if has_digit { i } else { pos }
        }
    }
    // We’ve reached the end of the input string.
    if has_digit { len } else { pos }
}

fn identify_number_octal(raw: &str, pos: usize, len: usize) -> usize {
    let mut has_digit = false; // octal literals must have at least one digit
    for i in pos+2..len { // +2, because we already found "0o"
        let c = &raw[i..i+1];
        // If the character is an underscore, do nothing.
        if c == "_" {
        // Otherwise, if this char is an digit 0-7:
        } else if c >= "0" && c <= "7" {
            has_digit = true;
        // Otherwise, if this char is a point:
        } else if c == "." {
            // Reject the whole of 0o56.7, don’t just accept the 0o56 part.
            return pos
        } else {
            // Advance to the character after the octal number.
            return if has_digit { i } else { pos }
        }
    }
    // We’ve reached the end of the input string.
    if has_digit { len } else { pos }
}


#[cfg(test)]
mod tests {
    use super::identify_number as identify;

    #[test]
    fn identify_number_binary() {
        let raw = "0b01 0b0_0_ 0b1A 0b__1_";
        assert_eq!(identify(raw, 0), 4); // 0b01
        assert_eq!(identify(raw, 1), 1); // b01
        assert_eq!(identify(raw, 2), 4); // 01 is recognised as decimal
        assert_eq!(identify(raw, 5), 11); // 0b0_0_
        assert_eq!(identify(raw, 12), 15); // the 0b1 part is accepted
        assert_eq!(identify(raw, 17), 23); // 0b__1_
    }

    #[test]
    fn identify_number_binary_invalid() {
        let raw = "0b12 0b11.1 0b 0B11 0b___";
        assert_eq!(identify(raw, 0), 0); // 0b12 is not a valid number
        assert_eq!(identify(raw, 2), 4); // 12 is recognised as decimal
        assert_eq!(identify(raw, 5), 5); // 0b11.1 is not a valid number
        assert_eq!(identify(raw, 7), 11); // 11.1
        assert_eq!(identify(raw, 12), 12); // 0b is not a valid number
        assert_eq!(identify(raw, 15), 16); // 0B11 is not valid, but 0 is
        assert_eq!(identify(raw, 20), 20); // 0b___ is not a valid number
    }

    #[test]
    fn identify_number_decimal_integer() {
        let raw = "7 0 3";
        assert_eq!(identify(raw, 0), 1); // 7
        assert_eq!(identify(raw, 1), 1); // space
        assert_eq!(identify(raw, 2), 3); // 0
        assert_eq!(identify(raw, 3), 3); // space
        assert_eq!(identify(raw, 4), 5); // 3
        let raw = "765 012 10";
        assert_eq!(identify(raw, 0), 3); // 765
        assert_eq!(identify(raw, 1), 3); // 65 no ‘lookbehind’ happens!
        assert_eq!(identify(raw, 2), 3); // 5
        assert_eq!(identify(raw, 3), 3); // space
        assert_eq!(identify(raw, 4), 7); // 012
        assert_eq!(identify(raw, 7), 7); // space
        assert_eq!(identify(raw, 8), 10); // 10
        assert_eq!(identify(raw, 9), 10); // 0
    }

    #[test]
    fn identify_number_decimal_underscores() {
        let raw = "7_5 012___ 3_4_. 0_0.0_00__0_";
        assert_eq!(identify(raw, 0), 3); // 7_5
        assert_eq!(identify(raw, 1), 1); // _5 can’t start numbers that way!
        assert_eq!(identify(raw, 2), 3); // 5
        assert_eq!(identify(raw, 4), 10); // 012___
        assert_eq!(identify(raw, 11), 16); // 3_4_.
        assert_eq!(identify(raw, 17), 29); // 0_0.0_00__0_
    }

    #[test]
    fn identify_number_float_no_exponent() {
        let raw = "7.5 0.12 34. 00.0__0_00";
        assert_eq!(identify(raw, 0), 3); // 7.5
        assert_eq!(identify(raw, 1), 1); // .5 is not a valid number
        assert_eq!(identify(raw, 2), 3); // 5
        assert_eq!(identify(raw, 3), 3); // space
        assert_eq!(identify(raw, 4), 8); // 0.12
        assert_eq!(identify(raw, 9), 12); // 34. is valid
        assert_eq!(identify(raw, 13), 23); // 00.0__0_00
        // Here, each "123." exercises a different conditional branch.
        let raw = "123. 123.";
        assert_eq!(identify(raw, 0), 4); // 123. part way through input
        assert_eq!(identify(raw, 5), 9); // 123. reaches end of input
    }

    #[test]
    fn identify_number_float_no_exponent_invalid() {
        let raw = "1.2.3 .12 0..1";
        assert_eq!(identify(raw, 0), 3); // 1.2
        assert_eq!(identify(raw, 1), 1); // .2 is not a valid number
        assert_eq!(identify(raw, 2), 5); // 2.3
        assert_eq!(identify(raw, 5), 5); // space
        assert_eq!(identify(raw, 6), 6); // .12 is not a valid number
        assert_eq!(identify(raw, 7), 9); // 12
        assert_eq!(identify(raw, 10), 12); // 0.
        assert_eq!(identify(raw, 11), 11); // ..
        assert_eq!(identify(raw, 12), 12); // .1
        assert_eq!(identify(raw, 13), 14); // 1
    }

    #[test]
    fn identify_number_float_with_exponent() {
        let raw = "0e0 9E9 1e+2 4E-3 8E1+2 54.32E+10";
        assert_eq!(identify(raw, 0), 3);   // 0e0 is 0
        assert_eq!(identify(raw, 4), 7);   // 9E9 is 9000000000
        assert_eq!(identify(raw, 8), 12);  // 1e+2 is 100
        assert_eq!(identify(raw, 13), 17); // 4E-3 is 0.004
        assert_eq!(identify(raw, 18), 21); // the 8E1 part is accepted
        assert_eq!(identify(raw, 24), 33); // 54.32E+10 is 543200000000
        let raw = "4_3.21e+10 43_.21e+10 43.2_1e+10 43.21_e+10 43.21e+_10 43.21e+1_0 43.21e+10_";
        assert_eq!(identify(raw, 0), 10);  // 4_3.21e+10 is ok .js
        assert_eq!(identify(raw, 11), 21); // 43_.21e+10 is invalid in .js
        assert_eq!(identify(raw, 22), 32); // 43.2_1e+10 is ok .js
        assert_eq!(identify(raw, 33), 43); // 43.21_e+10 is invalid in .js
        assert_eq!(identify(raw, 44), 54); // 43.21e+_10 is invalid in .js
        assert_eq!(identify(raw, 55), 65); // 43.21e+1_0 is ok .js
        assert_eq!(identify(raw, 66), 76); // 43.21e+10_ is invalid in .js
        assert_eq!(identify("43.21e_10", 0), 9); // 43.21e_10 is invalid in .js
    }

    #[test]
    fn identify_number_float_with_exponent_invalid() {
        let raw = "10e 9E+ 1e2. 4E+-3 8Ee12 1+1 54.32E";
        assert_eq!(identify(raw, 0), 0);   // 10e has no exponent value
        assert_eq!(identify(raw, 4), 4);   // 9E+ has no exponent value
        assert_eq!(identify(raw, 8), 8);   // 1e2. exponent value contains "."
        assert_eq!(identify(raw, 13), 13); // 4E+-3 has "+" and "-"
        assert_eq!(identify(raw, 19), 19); // 8Ee12 has an extra "e"
        assert_eq!(identify(raw, 21), 21); // e12 has no digit at start
        assert_eq!(identify(raw, 25), 26); // 1+1 perhaps you meant 1e+1
        assert_eq!(identify(raw, 29), 29); // 54.32E has no exponent value
        // The last character of a string is an edge case which needs its own test.
        assert_eq!(identify("54.32e-", 0), 0); // 54.32e- has no exponent value
        // Here, each "43.21e_" exercises a different conditional branch.
        let raw = "43._21e+10 43.21e_+10 43.21e_+ 43.21e_ 43.21e_";
        assert_eq!(identify(raw, 0), 0);   // 43._21e+10
        assert_eq!(identify(raw, 11), 11); // 43.21e_+10
        assert_eq!(identify(raw, 22), 22); // 43.21e_+
        assert_eq!(identify(raw, 31), 31); // 43.21e_ part way through input
        assert_eq!(identify(raw, 39), 39); // 43.21e_ reaches end of input
    }

    #[test]
    fn identify_number_hex() {
        let raw = "0x09 0xA_b_ 0xAG 0x__C_";
        assert_eq!(identify(raw, 0), 4);   // 0x09
        assert_eq!(identify(raw, 1), 1);   // x09
        assert_eq!(identify(raw, 2), 4);   // 09 is recognised as decimal
        assert_eq!(identify(raw, 5), 11);  // 0xA_b_ mixed case is ok
        assert_eq!(identify(raw, 12), 15); // the 0xA part is accepted
        assert_eq!(identify(raw, 17), 23); // 0x__C_
    }

    #[test]
    fn identify_number_hex_invalid() {
        let raw = "0xGA 0xab.c 0x 0XAB 0x___";
        assert_eq!(identify(raw, 0), 0);   // 0xGA is not a valid number
        assert_eq!(identify(raw, 5), 5);   // 0xab.c is not a valid number
        assert_eq!(identify(raw, 7), 7);   // ab.c is valid, but not a number
        assert_eq!(identify(raw, 12), 12); // 0x is not a valid number
        assert_eq!(identify(raw, 15), 16); // 0XAB is not valid, but 0 is
        assert_eq!(identify(raw, 20), 20); // 0x___ is not a valid number
    }

    #[test]
    fn identify_number_octal() {
        let raw = "0o07 0o7_3_ 0o7a 0o__5_";
        assert_eq!(identify(raw, 0), 4);   // 0o07
        assert_eq!(identify(raw, 1), 1);   // o07
        assert_eq!(identify(raw, 2), 4);   // 07 is recognised as decimal
        assert_eq!(identify(raw, 5), 11);  // 0o7_3_
        assert_eq!(identify(raw, 12), 15); // the 0o7 part is accepted
        assert_eq!(identify(raw, 17), 23); // 0o__5_
    }

    #[test]
    fn identify_number_octal_invalid() {
        let raw = "0oa7 0o56.7 0o 0O34 0o___";
        assert_eq!(identify(raw, 0), 0);   // 0oa7 is not a valid number
        assert_eq!(identify(raw, 5), 5);   // 0o56.7 is not a valid number
        assert_eq!(identify(raw, 7), 11);  // 56.7 is recognised as decimal
        assert_eq!(identify(raw, 12), 12); // 0o is not a valid number
        assert_eq!(identify(raw, 15), 16); // 0O34 is not valid, but 0 is
        assert_eq!(identify(raw, 20), 20); // 0o___ is not a valid number
    }

    #[test]
    fn identify_number_too_large() {
        // These numbers are larger than u128, so Rust won’t parse them.
        // However, identify_number() is just a scanner, and not that smart!
        // let _nope: u128 = 1234567890123456789012345678901234567890;
        let raw = "1234567890123456789012345678901234567890";
        assert_eq!(identify(raw, 0), 40);
        // let _nope: u128 = 0b1_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let raw = "0b1_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000";
        assert_eq!(identify(raw, 0), 147);
        // let _nope: u128 = 0o12345671234567123456712345671234567123456712;
        let raw = "0o12345671234567123456712345671234567123456712";
        assert_eq!(identify(raw, 0), 46);
        // let _nope: u128 = 0x1234567890abcdefABCDEF1234567890a;
        let raw = "0x1234567890abcdefABCDEF1234567890a";
        assert_eq!(identify(raw, 0), 35); // we also test 0-9A-Za-z here
    }
}
