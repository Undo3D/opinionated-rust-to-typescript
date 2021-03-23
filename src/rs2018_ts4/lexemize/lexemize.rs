//! Transforms a Rust 2018 program into lexemes.

use super::lexeme::{Lexeme,LexemeKind,Lexemes};

/// Transforms a Rust 2018 program into lexemes.
/// 
/// ### Arguments
/// * `raw` The original Rust code, assumed to conform to the 2018 edition
/// 
/// ### Returns
/// @TODO document what this function returns
pub fn lexemize(
    raw: &str
) -> Lexemes {
    let mut lexemes = Lexemes {
        end_column: 0,
        end_line_number: 0,
        end_pos: 0,
        lexemes: vec![],
    };
    let len = raw.len();
    let mut column = 0;
    let mut line_number = 0;
    let mut pos = 0;
  
    // Loop until we reach the last character of the original Rust code.
    while pos < len {
        lexemes.lexemes.push(Lexeme {
            column,
            kind: LexemeKind::Character,
            line_number,
            pos,
            snippet: "ok",
        });
        if &raw[pos..pos+1] == "\n" {
            column = 0;
            line_number += 1;
        } else {
            column += 1;
        }
        pos += 1;
    }

    lexemes.end_column = column;
    lexemes.end_line_number = line_number;
    lexemes.end_pos = pos;

    lexemes
}

#[cfg(test)]
mod tests {
//   use super::super::lexeme::Lexeme;
  use super::lexemize;

  #[test]
  fn empty_input() {
    let raw = "";
    assert_eq!(lexemize(raw).to_string(), "");
  }
}
