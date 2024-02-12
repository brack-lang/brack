use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BrackType {
    TInline,
    TOption(Box<BrackType>),
    TBlock,
    TArray(Box<BrackType>),
    TInlineCmd(String),
    TBlockCmd(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrackMetaData {
    pub command_name: String,
    pub call_name: String,
    pub argument_types: Vec<(String, BrackType)>,
    pub return_type: BrackType,
}
