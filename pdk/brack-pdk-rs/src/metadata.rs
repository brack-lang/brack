use crate::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub command_name: String,
    pub call_name: String,
    pub argument_types: Vec<(String, Type)>,
    pub return_type: Type,
}

impl Metadata {
    pub fn new(command_name: String, call_name: String, argument_types: Vec<(String, Type)>, return_type: Type) -> Metadata {
        Metadata {
            command_name,
            call_name,
            argument_types,
            return_type,
        }
    }
}
