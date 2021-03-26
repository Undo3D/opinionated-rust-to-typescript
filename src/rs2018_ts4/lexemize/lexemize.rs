//! Transforms a Rust 2018 program into lexemes.

use std::fmt;

use super::lexeme::{Lexeme,LexemeKind};
use super::identify::character::identify_character;
use super::identify::comment::identify_comment;
use super::identify::identifier::identify_identifier;
use super::identify::number::identify_number;
use super::identify::punctuation::identify_punctuation;
use super::identify::string::identify_string;
use super::identify::whitespace::identify_whitespace;

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

/// Transforms a Rust 2018 program into `Lexemes`.
/// 
/// The primary purpose of `lexemize()` is to quickly divide Rust code into
/// three basic sections — comments, strings, and everything else.
/// 
/// The ‘everything else’ section is then divided into literals, punctuation,
/// whitespace and identifiers. Anything left over is marked as ‘xtraneous’.
/// 
/// Any input string can be lexemized, so this function never returns any kind
/// of error. Checking `raw` for semantic correctness should be done later on,
/// during tokenization and parsing.
/// 
/// ### Arguments
/// * `raw` The original Rust code, assumed to conform to the 2018 edition
/// 
/// ### Returns
/// @TODO document what this function returns
pub fn lexemize(
    raw: &str
) -> LexemizeResult {
    // Initialise `len`, `pos`, and the output object.
    let len = raw.len();
    let mut pos = 0;
    let mut result = LexemizeResult {
        end_pos: 0,
        lexemes: vec![],
    };
  
    // Loop until we reach the last character of the original Rust code.
    while pos < len {

        // Deal with a literal char, if one begins here.
        let next_pos = identify_character(raw, pos);
        if next_pos != pos {
            result.lexemes.push(Lexeme {
                kind: LexemeKind::Character,
                pos,
                snippet: raw[pos..next_pos].to_string(),
            });
            pos = next_pos;
            continue;
        }

        // Deal with an inline or multiline comment, if one begins here.
        let next_pos = identify_comment(raw, pos);
        if next_pos != pos {
            result.lexemes.push(Lexeme {
                kind: LexemeKind::Comment,
                pos,
                snippet: raw[pos..next_pos].to_string(),
            });
            pos = next_pos;
            continue;
        }
    
        // Deal with a literal string, if one begins here.
        let next_pos = identify_string(raw, pos);
        if next_pos != pos {
            result.lexemes.push(Lexeme {
                kind: LexemeKind::String,
                pos,
                snippet: raw[pos..next_pos].to_string(),
            });
            pos = next_pos;
            continue;
        }

        // Deal with an identifier, if one begins here.
        let next_pos = identify_identifier(raw, pos);
        if next_pos != pos {
            result.lexemes.push(Lexeme {
                kind: LexemeKind::Identifier,
                pos,
                snippet: raw[pos..next_pos].to_string(),
            });
            pos = next_pos;
            continue;
        }

        // Deal with a literal number, if one begins here.
        let next_pos = identify_number(raw, pos);
        if next_pos != pos {
            result.lexemes.push(Lexeme {
                kind: LexemeKind::Number,
                pos,
                snippet: raw[pos..next_pos].to_string(),
            });
            pos = next_pos;
            continue;
        }
    
        // Deal with punctuation, if a sequence of 1, 2 or 3 begins here.
        let next_pos = identify_punctuation(raw, pos);
        if next_pos != pos {
            result.lexemes.push(Lexeme {
                kind: LexemeKind::Punctuation,
                pos,
                snippet: raw[pos..next_pos].to_string(),
            });
            pos = next_pos;
            continue;
        }

        // Deal with whitespace, if whitespace begins here.
        let next_pos = identify_whitespace(raw, pos);
        if next_pos != pos {
            result.lexemes.push(Lexeme {
                kind: LexemeKind::Whitespace,
                pos,
                snippet: raw[pos..next_pos].to_string(),
            });
            pos = next_pos;
            continue;
        }
    
        pos += 1;
    }

    result.end_pos = pos;
    result
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
    fn lexemize_empty_input() {
        let raw = "";
        assert_eq!(lexemize(raw).to_string(),
            "Lexemes found: 0\n\
             EndOfInput          0  <EOI>");
    }

    #[test]
    fn lexemize_three_characters() {
        let raw = "'Z''\\t''\\0'";
        assert_eq!(lexemize(raw).to_string(),
            "Lexemes found: 3\n\
             Character           0  'Z'\n\
             Character           3  '\\t'\n\
             Character           7  '\\0'\n\
             EndOfInput         11  <EOI>"
        );
    }

    #[test]
    fn lexemize_three_comments() {
        let raw = "/**A/*A'*/*///B\n//C";
        assert_eq!(lexemize(raw).to_string(),
            "Lexemes found: 4\n\
             Comment             0  /**A/*A'*/*/\n\
             Comment            12  //B\n\
             Whitespace         15  <NL>\n\
             Comment            16  //C\n\
             EndOfInput         19  <EOI>"
        );
    }

    #[test]
    fn three_identifiers() {
        let raw = "abc;_D,__12";
        assert_eq!(lexemize(raw).to_string(),
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
    fn three_numbers() {
        let raw = "0b1001_0011 0x__01aB__ 1_2.3_4E+_5_";
        assert_eq!(lexemize(raw).to_string(),
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
    fn three_punctuations() {
        let raw = ";*=>>=";
        assert_eq!(lexemize(raw).to_string(),
            "Lexemes found: 3\n\
             Punctuation         0  ;\n\
             Punctuation         1  *=\n\
             Punctuation         3  >>=\n\
             EndOfInput          6  <EOI>"
        );
    }

    #[test]
    fn three_strings() {
        let raw = "\"\"\"ok\"r##\"\\\"\"##";
        assert_eq!(lexemize(raw).to_string(),
            "Lexemes found: 3\n\
             String              0  \"\"\n\
             String              2  \"ok\"\n\
             String              6  r##\"\\\"\"##\n\
             EndOfInput         15  <EOI>"
      );
    }

    #[test]
    fn three_whitespace() {
        let raw = "\t\ta \n\nb\r ";
        assert_eq!(lexemize(raw).to_string(),
            "Lexemes found: 5\n\
             Whitespace          0  \t\t\n\
             Identifier          2  a\n\
             Whitespace          3   <NL><NL>\n\
             Identifier          6  b\n\
             Whitespace          7  \r \n\
             EndOfInput          9  <EOI>"
      );
    }

}
