use std::convert::From;
use std::fmt::{self, Display, Formatter};

use brack_tokenizer::tokens::Location;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DocumentError {
    AngleNotClosed(Location),
    CurlyNotClosed(Location),
    SquareNotClosed(Location),
    DotNotFound(Location),
    ModuleNotFound(Location),
    IdentifierNotFound(Location),
    NeedNewLine(Location),
    ExprComponentNotFound(Location),
}

#[derive(Error, Debug)]
pub enum ParseTerminationError {
    AngleNotOpened(Location),
    CurlyNotOpened(Location),
    SquareNotOpened(Location),
    ExprComponentNotFound(Location),
    TokenNotFound(Location),
    MokuleNotFound(Location),
    DotNotFound(Location),
}

#[derive(Error, Debug)]
pub enum ParserError {
    DocumentError(DocumentError),
    ParseTerminationError(ParseTerminationError),
}

impl DocumentError {
    pub fn get_location(&self) -> Location {
        match self {
            Self::AngleNotClosed(location) => location.clone(),
            Self::CurlyNotClosed(location) => location.clone(),
            Self::SquareNotClosed(location) => location.clone(),
            Self::DotNotFound(location) => location.clone(),
            Self::ModuleNotFound(location) => location.clone(),
            Self::IdentifierNotFound(location) => location.clone(),
            Self::NeedNewLine(location) => location.clone(),
            Self::ExprComponentNotFound(location) => location.clone(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            Self::AngleNotClosed(_) => "Angle bracket not closed".to_string(),
            Self::CurlyNotClosed(_) => "Curly bracket not closed".to_string(),
            Self::SquareNotClosed(_) => "Square bracket not closed".to_string(),
            Self::DotNotFound(_) => "Need dot after module".to_string(),
            Self::ModuleNotFound(_) => "Need module".to_string(),
            Self::IdentifierNotFound(_) => "Need identifier after module".to_string(),
            Self::NeedNewLine(_) => "Need new line".to_string(),
            Self::ExprComponentNotFound(_) => "Need `text` or `angle` or `square`".to_string(),
        }
    }
}

impl Display for DocumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let location = self.get_location();
        let message = self.get_message();
        write!(
            f,
            "Error at line {}, column {} to line {}, column {}: {}",
            location.start.line,
            location.start.character,
            location.end.line,
            location.end.character,
            message
        )
    }
}

impl ParseTerminationError {
    pub fn get_location(&self) -> Location {
        match self {
            Self::AngleNotOpened(location) => location.clone(),
            Self::CurlyNotOpened(location) => location.clone(),
            Self::SquareNotOpened(location) => location.clone(),
            Self::ExprComponentNotFound(location) => location.clone(),
            Self::TokenNotFound(location) => location.clone(),
            Self::MokuleNotFound(location) => location.clone(),
            Self::DotNotFound(location) => location.clone(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            Self::AngleNotOpened(_) => "Angle bracket not opened".to_string(),
            Self::CurlyNotOpened(_) => "Curly bracket not opened".to_string(),
            Self::SquareNotOpened(_) => "Square bracket not opened".to_string(),
            Self::ExprComponentNotFound(_) => "Need `text` or `angle` or `square`".to_string(),
            Self::TokenNotFound(_) => "Token not found".to_string(),
            Self::MokuleNotFound(_) => "Module not found".to_string(),
            Self::DotNotFound(_) => "Dot not found".to_string(),
        }
    }
}

impl Display for ParseTerminationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let location = self.get_location();
        let message = self.get_message();
        write!(
            f,
            "Error at line {}, column {} to line {}, column {}: {}",
            location.start.line,
            location.start.character,
            location.end.line,
            location.end.character,
            message
        )
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
    pub fn get_location(&self) -> Location {
        match self {
            Self::DocumentError(e) => e.get_location(),
            Self::ParseTerminationError(e) => e.get_location(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            Self::DocumentError(e) => e.get_message(),
            Self::ParseTerminationError(e) => e.get_message(),
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
