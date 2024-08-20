use crate::document::Document;
use crate::plugin::Plugin;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub document: Document,
    pub plugins: Option<HashMap<String, Plugin>>,
}
