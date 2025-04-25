use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Html {
    pub tag: HtmlTag,
    pub attributes: Vec<HashMap<String, String>>,
    pub children: Vec<Html>,
}

impl fmt::Display for Html {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} ", self.tag)?;
        for attr in self.attributes.clone() {
            for (key, value) in attr {
                write!(f, "{}=\"{}\" ", key, value)?;
            }
        }
        write!(f, ">")?;
        for child in &self.children {
            write!(f, "{}", child)?;
        }
        write!(f, "</{}>", self.tag)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HtmlTag {
    Fragment(String),
    Div,
    Span,
    P,
    B,
}

impl fmt::Display for HtmlTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlTag::Fragment(_) => write!(f, ""),
            HtmlTag::Div => write!(f, "div"),
            HtmlTag::Span => write!(f, "span"),
            HtmlTag::P => write!(f, "p"),
            HtmlTag::B => write!(f, "b"),
        }
    }
}

