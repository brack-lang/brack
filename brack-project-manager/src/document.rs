use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub name: String,
    pub version: String,
    pub backend: String,
    pub authors: Vec<String>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            version: "0.1.0".to_string(),
            backend: "".to_string(),
            authors: vec!["your name <your email>".to_string()],
        }
    }
}
