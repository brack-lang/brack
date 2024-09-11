use std::fmt::{self, Display, Formatter};

use brack_tokenizer::tokens::Location;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {
    AngleNotOpened(Location),
    AngleNotClosed(Location),
    CurlyNotOpened(Location),
    CurlyNotClosed(Location),
    SquareNotOpened(Location),
    SquareNotClosed(Location),
    MismatchedBracket(Location),
    ModuleNotFound(Location),
    IdentifierNotFound(Location),
    DotNotFound(Location),
    CommaNotFound(Location),
    UnexpectedDot(Location),
    UnexpectedComma(Location),
    InvalidBackslash(Location),
}

impl TransformError {
    pub fn get_location(&self) -> Location {
        match self {
            Self::AngleNotOpened(location) => location.clone(),
            Self::AngleNotClosed(location) => location.clone(),
            Self::CurlyNotOpened(location) => location.clone(),
            Self::CurlyNotClosed(location) => location.clone(),
            Self::SquareNotOpened(location) => location.clone(),
            Self::SquareNotClosed(location) => location.clone(),
            Self::MismatchedBracket(location) => location.clone(),
            Self::ModuleNotFound(location) => location.clone(),
            Self::IdentifierNotFound(location) => location.clone(),
            Self::DotNotFound(location) => location.clone(),
            Self::CommaNotFound(location) => location.clone(),
            Self::UnexpectedDot(location) => location.clone(),
            Self::UnexpectedComma(location) => location.clone(),
            Self::InvalidBackslash(location) => location.clone(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            Self::AngleNotOpened(_) => "Angle bracket not opened".to_string(),
            Self::AngleNotClosed(_) => "Angle bracket not closed".to_string(),
            Self::CurlyNotOpened(_) => "Curly bracket not opened".to_string(),
            Self::CurlyNotClosed(_) => "Curly bracket not closed".to_string(),
            Self::SquareNotOpened(_) => "Square bracket not opened".to_string(),
            Self::SquareNotClosed(_) => "Square bracket not closed".to_string(),
            Self::MismatchedBracket(_) => "Mismatched bracket".to_string(),
            Self::ModuleNotFound(_) => "Need module".to_string(),
            Self::IdentifierNotFound(_) => "Need identifier after module".to_string(),
            Self::DotNotFound(_) => "Need dot after module".to_string(),
            Self::CommaNotFound(_) => "Need comma after module".to_string(),
            Self::UnexpectedDot(_) => "Unexpected dot".to_string(),
            Self::UnexpectedComma(_) => "Unexpected comma".to_string(),
            Self::InvalidBackslash(_) => {
                "Backslash must be followed by dot, comma, backslash, or bracket".to_string()
            }
        }
    }
}

impl Display for TransformError {
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
