use crate::document::Document;
use crate::plugin::PluginSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub document: Document,
    pub plugins: Option<HashMap<String, PluginSchema>>,
}
