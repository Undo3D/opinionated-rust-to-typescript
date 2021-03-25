//! Transforms a Rust 2018 program into lexemes.

use std::fmt;

use super::lexeme::{Lexeme,LexemeKind};
use super::identify::character::identify_character;

///
pub struct LexemizeResult {
    ///
    pub end_column: usize,
    ///
    pub end_line_number: usize,
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

/// Transforms a Rust 2018 program into lexemes.
/// 
/// ### Arguments
/// * `raw` The original Rust code, assumed to conform to the 2018 edition
/// 
/// ### Returns
/// @TODO document what this function returns
pub fn lexemize(
    raw: &str
) -> LexemizeResult {
    let len = raw.len();
    // Initialise the output object.
    let mut result = LexemizeResult {
        end_column: 0,
        end_line_number: 0,
        end_pos: 0,
        lexemes: vec![],
    };
    // Initialise variables which will be increased during the loop, below.
    let mut column = 0;
    let mut line_number = 0;
    let mut pos = 0;
  
    // Loop until we reach the last character of the original Rust code.
    while pos < len {

        let next_pos = identify_character(raw, pos);
        if next_pos != pos {
            result.lexemes.push(Lexeme {
                column,
                kind: LexemeKind::Character,
                line_number,
                pos,
                snippet: raw[pos..next_pos].to_string(),
            });
            pos = next_pos;
            continue;
        }
    
        if &raw[pos..pos+1] == "\n" {
            column = 0;
            line_number += 1;
        } else {
            column += 1;
        }
        pos += 1;
    }

    result.end_column = column;
    result.end_line_number = line_number;
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
            end_column: 5,
            end_line_number: 20,
            end_pos: 123,
            lexemes: vec![
                Lexeme {
                    column: 0,
                    kind: LexemeKind::Comment,
                    line_number: 0,
                    pos: 0,
                    snippet: "/* This is a comment */".into(),
                },
                Lexeme {
                    column: 23,
                    kind: LexemeKind::Number,
                    line_number: 0,
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

}
