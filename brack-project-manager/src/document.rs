use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub name: String,
    pub version: String,
    pub backend: String,
}
