use std::convert::From;
use std::fmt::{self, Display, Formatter};

use brack_tokenizer::tokens::{Location, Token};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct DocumentError {
    message: String,
    location: Location,
}

#[derive(Error, Debug)]
pub struct ParseTerminationError {
    message: String,
    location: Location,
}

#[derive(Error, Debug)]
pub enum ParserError {
    DocumentError(DocumentError),
    ParseTerminationError(ParseTerminationError),
}

impl Display for DocumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

impl DocumentError {
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
        Self { message, location }
    }
}

impl Display for ParseTerminationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

impl ParseTerminationError {
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
        Self { message, location }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DocumentError(e) => write!(f, "{}", e),
            Self::ParseTerminationError(e) => write!(f, "{}", e),
        }
    }
}

impl ParserError {
    pub fn new_document_error(message: String, token: Token) -> Self {
        Self::DocumentError(DocumentError::new(message, token))
    }

    pub fn new_parse_termination_error(message: String, token: Token) -> Self {
        Self::ParseTerminationError(ParseTerminationError::new(message, token))
    }

    pub fn get_location(&self) -> Location {
        match self {
            Self::DocumentError(e) => e.location.clone(),
            Self::ParseTerminationError(e) => e.location.clone(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            Self::DocumentError(e) => e.message.clone(),
            Self::ParseTerminationError(e) => e.message.clone(),
        }
    }
}

impl From<DocumentError> for ParserError {
    fn from(e: DocumentError) -> Self {
        Self::DocumentError(e)
    }
}

impl From<ParseTerminationError> for ParserError {
    fn from(e: ParseTerminationError) -> Self {
        Self::ParseTerminationError(e)
    }
}
