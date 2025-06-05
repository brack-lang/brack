use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Value {
    Text(String),
    TextArray(Vec<String>),
    TextOption(Option<String>),
}