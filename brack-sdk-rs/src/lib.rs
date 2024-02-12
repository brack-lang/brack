pub mod ast;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    TInline,
    TOption(Box<Type>),
    TBlock,
    TArray(Box<Type>),
    TInlineCmd(String),
    TBlockCmd(String),
    TAST,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaData {
    pub command_name: String,
    pub call_name: String,
    pub argument_types: Vec<(String, Type)>,
    pub return_type: Type,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Value {
    Text(String),
    TextArray(Vec<String>),
    TextOption(Option<String>),
}
