use std::fmt::{self, Display, Formatter};

use brack_tokenizer::tokens::Token;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct ParserError {
    message: String,
    location: brack_tokenizer::tokens::Location,
    // line: usize,
    // column: usize,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // write!(
        //     f,
        //     "Error at line {}, column {}: {}",
        //     self.line, self.column, self.message
        // )
        write!(
            f,
            "Error at line {}, column {} to line {}, column {}: {}",
            self.location.start.line,
            self.location.start.character,
            self.location.end.line,
            self.location.end.character,
            self.message
        )
    }
}

impl ParserError {
    pub fn new(message: String, token: Token) -> Self {
        let location = match token {
            Token::Empty(location) => location,
            Token::Text(_, location) => location,
            Token::Module(_, location) => location,
            Token::Ident(_, location) => location,
            Token::NewLine(location) => location,
            Token::Dot(location) => location,
            Token::AngleBracketOpen(location) => location,
            Token::AngleBracketClose(location) => location,
            Token::SquareBracketOpen(location) => location,
            Token::SquareBracketClose(location) => location,
            Token::CurlyBracketOpen(location) => location,
            Token::CurlyBracketClose(location) => location,
            Token::Comma(location) => location,
            Token::EOF(location) => location,
        };
        Self {
            message,
            location,
            // line: location.start.line,
            // column: location.start.character,
        }
    }
}
