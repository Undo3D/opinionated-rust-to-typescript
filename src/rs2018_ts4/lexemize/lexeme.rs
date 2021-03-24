//! Enums and structs used by `lexemize()` (`LexemeKind`, `Lexeme`, `Lexemes`).

use std::fmt;

/// The edition of Rust that the input code is written in.
#[derive(Clone,Copy,PartialEq)]
pub enum LexemeKind {
    /// 
    Character,
    /// 
    Comment,
    /// 
    Identifier,
    /// 
    Number,
    /// 
    Punctuation,
    /// 
    String,
    /// 
    Whitespace,
    /// 
    Xtraneous,
}

impl LexemeKind {
    /// @TODO impl fmt::Display for LexemeKind
    pub fn to_string(&self) -> &str {
        match self {
            Self::Character   => "Character",
            Self::Comment     => "Comment",
            Self::Identifier  => "Identifier",
            Self::Number      => "Number",
            Self::Punctuation => "Punctuation",
            Self::String      => "String",
            Self::Whitespace  => "Whitespace",
            Self::Xtraneous   => "Xtraneous",
        }
    }
}

///
pub struct Lexemes {
    ///
    pub end_column: usize,
    ///
    pub end_line_number: usize,
    ///
    pub end_pos: usize,
    ///
    pub lexemes: Vec<Lexeme>,
}

impl fmt::Display for Lexemes {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut str = "";
        for lexeme in &self.lexemes {
            fmt.write_str(str)?;
            fmt.write_str(&lexeme.to_string())?;
            str = "\n";
        }
        Ok(())
    }
}

///
pub struct Lexeme {
    /// The position that the Lexeme starts, relative to the start of the line.
    /// Zero indexed.
    pub column: usize,
    /// Category of the Lexeme.
    pub kind: LexemeKind,
    /// The line number that contains the Lexeme’s start position.
    /// Zero indexed.
    pub line_number: usize,
    /// The position that the Lexeme starts, relative to the start of `raw`.
    /// Zero indexed.
    pub pos: usize,
    /// 
    pub snippet: &'static str,
}

impl fmt::Display for Lexeme {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let kind = self.kind.to_string();
        // snippet = snippet.replace("\n", "<NL>");
        write!(fmt, "{: <16} {: >4}  {}", kind, self.pos, self.snippet)
        //                  |||
        //                  ||+-- target width is four characters
        //                  |+--- align right
        //                  +---- fill with spaces
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn lexeme_kind_to_string_as_expected() {
        assert_eq!(LexemeKind::Character.to_string(),   "Character");
        assert_eq!(LexemeKind::Comment.to_string(),     "Comment");
        assert_eq!(LexemeKind::Identifier.to_string(),  "Identifier");
        assert_eq!(LexemeKind::Number.to_string(),      "Number");
        assert_eq!(LexemeKind::Punctuation.to_string(), "Punctuation");
        assert_eq!(LexemeKind::String.to_string(),      "String");
        assert_eq!(LexemeKind::Whitespace.to_string(),  "Whitespace");
        assert_eq!(LexemeKind::Xtraneous.to_string(),   "Xtraneous");
    }

    #[test]
    fn lexeme_to_string_as_expected() {
        let lexeme = Lexeme {
            column: 22,
            kind: LexemeKind::Character,
            line_number: 10,
            pos: 123,
            snippet: "yup",
        };
        assert_eq!(lexeme.to_string(), "Character         123  yup");
    }

    #[test]
    fn lexemes_to_string_as_expected() {
        let lexemes = Lexemes {
            end_column: 5,
            end_line_number: 20,
            end_pos: 123,
            lexemes: vec![
                Lexeme {
                    column: 0,
                    kind: LexemeKind::Comment,
                    line_number: 0,
                    pos: 0,
                    snippet: "/* This is a comment */",
                },
                Lexeme {
                    column: 23,
                    kind: LexemeKind::Number,
                    line_number: 0,
                    pos: 23,
                    snippet: "44.4",
                },
            ],
        };
        assert_eq!(lexemes.to_string(),
            "Comment             0  /* This is a comment */\n\
             Number             23  44.4"
        );
    }
}
