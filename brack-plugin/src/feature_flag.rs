use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FeatureFlag {
    pub document_hook: bool,
    pub stmt_hook: bool,
    pub expr_hook: bool,
}
