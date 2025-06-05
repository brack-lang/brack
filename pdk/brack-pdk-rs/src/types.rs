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