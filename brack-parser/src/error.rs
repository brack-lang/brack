use std::fmt::{Display, Formatter, self};

use brack_tokenizer::tokens::Token;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct ParserError {
    message: String,
    line: usize,
    column: usize,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl ParserError {
    pub fn new(message: String, token: Token) -> Self {
        let token_data = match token {
            Token::Empty(data) => data,
            Token::Text(_, data) => data,
            Token::Module(_, data) => data,
            Token::Ident(_, data) => data,
            Token::NewLine(data) => data,
            Token::Dot(data) => data,
            Token::AngleBracketOpen(data) => data,
            Token::AngleBracketClose(data) => data,
            Token::SquareBracketOpen(data) => data,
            Token::SquareBracketClose(data) => data,
            Token::CurlyBracketOpen(data) => data,
            Token::CurlyBracketClose(data) => data,
            Token::Comma(data) => data,
            Token::EOF(data) => data,
        };
        Self {
            message,
            line: token_data.line,
            column: token_data.column,
        }
    }
}
