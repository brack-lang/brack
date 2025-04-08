use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<Author>>,
    pub licenses: Option<Vec<License>>,
    pub dependencies: Option<HashMap<String, Dependency>>,
    pub channels: Option<HashMap<String, String>>,
    pub document: Option<DocumentSettings>,
    pub plugin: Option<PluginSettings>,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum License {
    Apache20,
    MIT,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Dependency {
    Version(String),
    Channel {
        channel: Option<String>,
        version: String,
        feature_flags: Option<Vec<String>>,
    },
    GitHub {
        owner: String,
        name: String,
        tag: String,
        feature_flags: Option<Vec<String>>,
    },
    Local {
        path: String,
        feature_flags: Option<Vec<String>>,
    },
}

#[derive(Serialize, Deserialize)]
pub enum OutputLevel {
    Token,
    Cst,
    Ast,
    ExpandedAst,
    IR,
    FinalOutput,
}

#[derive(Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    Text,
}

#[derive(Serialize, Deserialize)]
pub struct DocumentSettings {
    pub backend: String,
    pub extension: Option<String>,
    pub output_level: Option<OutputLevel>,
    pub output_format: Option<OutputFormat>,
    pub src: String,
}

#[derive(Serialize, Deserialize)]
pub enum CommandType {
    Inline,
    Block,
    AST,
}

#[derive(Serialize, Deserialize)]
pub struct CommandParam {
    pub name: String,

    #[serde(rename = "type")]
    pub type_: CommandType,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub callee: String,
    pub params: Vec<CommandParam>,
    pub return_type: Option<CommandType>,
}

#[derive(Serialize, Deserialize)]
pub struct PluginSettings {
    pub support_backends: Vec<String>,
    pub inlines: Option<HashMap<String, Command>>,
    pub blocks: Option<HashMap<String, Command>>,
    pub macros: Option<HashMap<String, Command>>,
}
