use std::fmt::{self, Display, Formatter};

use brack_tokenizer::tokens::{Location, mock_location};
use brack_plugin::{plugins::Plugins, metadata::Metadata, types::Type::*};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoweringError {
    PluginNotFound(Location),
    CommandNotFound(Location),
    MissingArgument(Location),
    TooManyArguments(Location),
}

impl LoweringError {
    pub fn get_location(&self) -> Location {
        match self {
            Self::PluginNotFound(location) => location.clone(),
            Self::CommandNotFound(location) => location.clone(),
            Self::MissingArgument(location) => location.clone(),
            Self::TooManyArguments(location) => location.clone(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            Self::PluginNotFound(_) => "Plugin not found".to_string(),
            Self::CommandNotFound(_) => "Command not found".to_string(),
            Self::MissingArgument(_) => "Missing argument".to_string(),
            Self::TooManyArguments(_) => "Too many arguments".to_string(),
        }
    }
}

impl Display for LoweringError {
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

