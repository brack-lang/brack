use core::fmt;

use serde::{Deserialize, Serialize};

use crate::html::Html;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IR {
    Html(Html),
}

impl fmt::Display for IR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IR::Html(html) => write!(f, "{}", html),
        }
    }
}

#[derive(Clone)]
pub enum IRKind {
    Html,
}
