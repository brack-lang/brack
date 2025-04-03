use brack_plugin::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OpCode {
    ToArray(usize),
    ToOption(Option<()>),
    Join(usize),
    Call {
        plugin_name: String,
        function_name: String,
        return_type: Type,
    },
    Push(String),
}
