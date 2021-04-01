//! Transforms raw Rust 2018 code into Lexemes.

use std::fmt;

use super::lexeme::{Lexeme,LexemeKind};
use super::detect::character::detect_character;
use super::detect::comment::detect_comment;
use super::detect::identifier::detect_identifier;
use super::detect::number::detect_number;
use super::detect::punctuation::detect_punctuation;
use super::detect::string::detect_string;
use super::detect::whitespace::detect_whitespace;

///
pub struct LexemizeResult {
    ///
    pub end_pos: usize,
    ///
    pub lexemes: Vec<Lexeme>,
}

impl fmt::Display for LexemizeResult {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Lexemes found: {}\n", self.lexemes.len())?;
        for lexeme in &self.lexemes {
            fmt.write_str(&lexeme.to_string())?;
            fmt.write_str("\n")?;
        }
        write!(fmt, "EndOfInput       {: >4}  <EOI>", self.end_pos)
        //                              |||
        //                              ||+-- target width is four characters
        //                              |+--- align right
        //                              +---- fill with spaces
    }
}

/// An array which associates the `detect_*()` functions with `LexemeKind`s.
/// 
/// Note that a `String` can start with an `"r"` character, so `detect_string()`
/// is placed before `detect_identifier()`.
pub const DETECTORS_AND_KINDS: [(
    fn (&str, usize) -> usize,
    LexemeKind,
); 7] = [
    (detect_character,   LexemeKind::Character),
    (detect_comment,     LexemeKind::Comment),
    (detect_string,      LexemeKind::String),
    (detect_identifier,  LexemeKind::Identifier),
    (detect_number,      LexemeKind::Number),
    (detect_punctuation, LexemeKind::Punctuation),
    (detect_whitespace,  LexemeKind::Whitespace),
];

/// Transforms a Rust 2018 program into a vector of `Lexemes`.
/// 
/// The primary purpose of `lexemize()` is to quickly divide Rust code into
/// three basic sections — comments, strings, and everything else.
/// 
/// The ‘everything else’ section is then divided into literals, punctuation,
/// whitespace and identifiers. Anything left over is marked as ‘xtraneous’.
/// 
/// Any input string can be lexemized, so this function never returns any kind
/// of error. Checking `orig` for semantic correctness should be done later on,
/// during tokenization and parsing.
/// 
/// ### Arguments
/// * `orig` The original Rust code, assumed to conform to the 2018 edition
/// 
/// ### Returns
/// `lexemize()` returns a [`LexemizeResult`] object.
pub fn lexemize(
    orig: &str
) -> LexemizeResult {
    // Initialise `len`, and some mutable variables.
    let len = orig.len();
    let mut pos = 0;
    let mut xtra_pos = 0;
    let mut result = LexemizeResult {
        end_pos: 0,
        lexemes: vec![],
    };

    // Loop until we reach the last character of the input string.
    'outer: while pos < len {
        // Only try to detect a Lexeme if this is the start of a character.
        if orig.is_char_boundary(pos) {
            // Step through the array of `detect_*()` functions, and their
            // associated `LexemeKinds`.
            for (detector, kind) in DETECTORS_AND_KINDS.iter() {
                // Possibly add one or two Lexemes to `result`.
                let next_pos = detect(
                    *detector,
                    *kind,
                    orig,
                    pos,
                    xtra_pos,
                    &mut result
                );
                // If a Lexeme has been detected at this character position,
                // `detect()` will have returned the character position of the
                // end of that Lexeme.
                if next_pos != pos {
                    pos = next_pos;
                    xtra_pos = pos;
                    continue 'outer;
                }
            }
            // Anything else is an unidentifiable character, which will be
            // picked up by the `xtra_pos != pos` conditional in `detect()`.
        }

        // Step forward one byte.
        pos += 1;
    }

    // If there are unidentifiable characters at the end of `orig`, add a final 
    // `Xtraneous` Lexeme before returning `result`.
    if xtra_pos != pos {
        result.lexemes.push(Lexeme {
            kind: LexemeKind::Xtraneous,
            pos: xtra_pos,
            snippet: orig[xtra_pos..pos].to_string(),
        });
    }

    result.end_pos = pos;
    result
}

fn detect(
    detector: fn (&str, usize) -> usize,
    kind: LexemeKind,
    orig: &str,
    pos: usize,
    xtra_pos: usize,
    result: &mut LexemizeResult,
) -> usize {
    // If the passed-in `detector()` does not detect the Lexeme, it will return
    // the same char-position as `pos`. In that case, just return `pos`.
    let next_pos = detector(orig, pos);
    if next_pos == pos { return pos }

    // If any ‘Xtraneous’ characters precede this Lexeme, record them before
    // recording this Lexeme.
    if xtra_pos != pos {
        result.lexemes.push(Lexeme {
            kind: LexemeKind::Xtraneous,
            pos: xtra_pos,
            snippet: orig[xtra_pos..pos].to_string(),
        });
    }
    result.lexemes.push(Lexeme {
        kind,
        pos,
        snippet: orig[pos..next_pos].to_string(),
    });

    // Tell `lexemize()` the character position of the end of the Lexeme.
    next_pos
}



#[cfg(test)]
mod tests {
    use super::{LexemizeResult,lexemize};
    use super::super::lexeme::{Lexeme,LexemeKind};

    #[test]
    fn lexemize_result_to_string_as_expected() {
        let result = LexemizeResult {
            end_pos: 123,
            lexemes: vec![
                Lexeme {
                    kind: LexemeKind::Comment,
                    pos: 0,
                    snippet: "/* This is a comment */".into(),
                },
                Lexeme {
                    kind: LexemeKind::Number,
                    pos: 23,
                    snippet: "44.4".into(),
                },
            ],
        };
        assert_eq!(result.to_string(),
            "Lexemes found: 2\n\
             Comment             0  /* This is a comment */\n\
             Number             23  44.4\n\
             EndOfInput        123  <EOI>"
        );
    }

    #[test]
    fn lexemize_all_lexemes() {
        // Empty string.
        assert_eq!(lexemize("").to_string(),
            "Lexemes found: 0\n\
             EndOfInput          0  <EOI>");
    }

    #[test]
    fn lexemize_characters() {
        // Three Characters.
        assert_eq!(lexemize("'Z''\\t''\\0'").to_string(),
            "Lexemes found: 3\n\
             Character           0  'Z'\n\
             Character           3  '\\t'\n\
             Character           7  '\\0'\n\
             EndOfInput         11  <EOI>"
        );
    }

    #[test]
    fn lexemize_comments() {
        // Three Comments.
        assert_eq!(lexemize("/**A/*A'*/*///B\n//C").to_string(),
            "Lexemes found: 4\n\
             Comment             0  /**A/*A'*/*/\n\
             Comment            12  //B\n\
             Whitespace         15  <NL>\n\
             Comment            16  //C\n\
             EndOfInput         19  <EOI>"
        );
    }

    #[test]
    fn lexemize_identifiers() {
        // Three Identifiers.
        assert_eq!(lexemize("abc;_D,__12").to_string(),
            "Lexemes found: 5\n\
             Identifier          0  abc\n\
             Punctuation         3  ;\n\
             Identifier          4  _D\n\
             Punctuation         6  ,\n\
             Identifier          7  __12\n\
             EndOfInput         11  <EOI>"
        );
    }

    #[test]
    fn lexemize_numbers() {
        // Three Numbers.
        assert_eq!(lexemize("0b1001_0011 0x__01aB__ 1_2.3_4E+_5_").to_string(),
            "Lexemes found: 5\n\
             Number              0  0b1001_0011\n\
             Whitespace         11   \n\
             Number             12  0x__01aB__\n\
             Whitespace         22   \n\
             Number             23  1_2.3_4E+_5_\n\
             EndOfInput         35  <EOI>"
        );
    }

    #[test]
    fn lexemize_punctuations() {
        // Three Punctuations.
        assert_eq!(lexemize(";*=>>=").to_string(),
            "Lexemes found: 3\n\
             Punctuation         0  ;\n\
             Punctuation         1  *=\n\
             Punctuation         3  >>=\n\
             EndOfInput          6  <EOI>"
        );
    }

    #[test]
    fn lexemize_strings() {
        // Three Strings.
        assert_eq!(lexemize("\"\"\"ok\"r##\"\\\"\"##").to_string(),
            "Lexemes found: 3\n\
             String              0  \"\"\n\
             String              2  \"ok\"\n\
             String              6  r##\"\\\"\"##\n\
             EndOfInput         15  <EOI>"
      );
    }

    #[test]
    fn lexemize_whitespace() {
        // Three Whitespace.
        assert_eq!(lexemize("\t\ta \n\nb\r ").to_string(),
            "Lexemes found: 5\n\
             Whitespace          0  \t\t\n\
             Identifier          2  a\n\
             Whitespace          3   <NL><NL>\n\
             Identifier          6  b\n\
             Whitespace          7  \r \n\
             EndOfInput          9  <EOI>"
      );
    }

    #[test]
    fn lexemize_xtraneous() {
        // Mixture.
        assert_eq!(lexemize("~¶ €").to_string(),
            "Lexemes found: 3\n\
             Xtraneous           0  ~¶\n\
             Whitespace          3   \n\
             Xtraneous           4  €\n\
             EndOfInput          7  <EOI>"
        );
        // Non-ascii.
        assert_eq!(lexemize("~`\\").to_string(),
            "Lexemes found: 1\n\
             Xtraneous           0  ~`\\\n\
             EndOfInput          3  <EOI>"
        );
        // Ascii.
        assert_eq!(lexemize("é¢€±").to_string(),
            "Lexemes found: 1\n\
             Xtraneous           0  é¢€±\n\
             EndOfInput          9  <EOI>"
        );
    }
}
