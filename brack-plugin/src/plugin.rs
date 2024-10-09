use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use extism::Plugin;
use extism_convert::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    TInline,
    TOption(Box<Type>),
    TBlock,
    TArray(Box<Type>),
    TInlineCmd(String),
    TBlockCmd(String),
    TAST,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub command_name: String,
    pub call_name: String,
    pub argument_types: Vec<(String, Type)>,
    pub return_type: Type,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Value {
    Text(String),
    TextArray(Vec<String>),
    TextOption(Option<String>),
}

pub type ModuleName = String;
pub type CommandName = String;
pub type Plugins = HashMap<ModuleName, (Plugin, PluginMetaDataMap)>;
pub type PluginMetaDataMap = HashMap<(CommandName, Type), Metadata>;

#[derive(Debug, Clone, Default)]
pub struct FeatureFlug {
    pub document_hook: bool,
    pub stmt_hook: bool,
    pub expr_hook: bool,
}

pub fn new_plugins<M: AsRef<str>, P: AsRef<Path>>(
    plutin_path_map: HashMap<M, (P, FeatureFlug)>,
) -> Result<Plugins> {
    let mut result = HashMap::new();
    let mut document_hook_count = 0;
    let mut stmt_hook_count = 0;
    let mut expr_hook_count = 0;
    for (name, (path, flag)) in plutin_path_map {
        let name = name.as_ref().to_string();
        let path = path.as_ref().to_path_buf();
        let mut extism_plugin = Plugin::new(fs::read(&path)?, [], true)?;
        let Json(metadatas) = extism_plugin.call::<(), Json<Vec<Metadata>>>("get_metadata", ())?;
        let mut metadata_map = HashMap::new();
        let mut exists_document_hook = false;
        let mut exists_stmt_hook = false;
        let mut exists_expr_hook = false;
        for metadata in metadatas {
            let command_name = metadata.command_name.clone();
            let return_type = metadata.return_type.clone();
            if command_name == "document" && return_type == Type::TBlock {
                exists_document_hook = true;
                document_hook_count += 1;
                let mut document_plugin_metadata_map = HashMap::new();
                document_plugin_metadata_map.insert(
                    (command_name.clone(), return_type.clone()),
                    metadata.clone(),
                );
                result.insert("_document_hook".to_string(), (Plugin::new(fs::read(&path)?, [], true)?, document_plugin_metadata_map));
                continue
            } else if command_name == "stmt" && return_type == Type::TBlock {
                exists_stmt_hook = true;
                stmt_hook_count += 1;
                let mut stmt_plugin_metadata_map = HashMap::new();
                stmt_plugin_metadata_map.insert(
                    (command_name.clone(), return_type.clone()),
                    metadata.clone(),
                );
                result.insert("_stmt_hook".to_string(), (Plugin::new(fs::read(&path)?, [], true)?, stmt_plugin_metadata_map));
                continue
            } else if command_name == "expr" && return_type == Type::TInline {
                exists_expr_hook = true;
                expr_hook_count += 1;
                let mut expr_plugin_metadata_map = HashMap::new();
                expr_plugin_metadata_map.insert(
                    (command_name.clone(), return_type.clone()),
                    metadata.clone(),
                );
                result.insert("_expr_hook".to_string(), (Plugin::new(fs::read(&path)?, [], true)?, expr_plugin_metadata_map));
                continue
            }
            metadata_map.insert(
                (command_name, return_type),
                metadata,
            );
        }
        if flag.document_hook && !exists_document_hook {
            return Err(anyhow::anyhow!("Document hook is not found"));
        }
        if flag.stmt_hook && !exists_stmt_hook {
            return Err(anyhow::anyhow!("Stmt hook is not found"));
        }
        if flag.expr_hook && !exists_expr_hook {
            return Err(anyhow::anyhow!("Expr hook is not found"));
        }
        result.insert(name, (extism_plugin, metadata_map));
    }
    if document_hook_count > 1 {
        return Err(anyhow::anyhow!("Only one document hook is allowed"));
    }
    if stmt_hook_count > 1 {
        return Err(anyhow::anyhow!("Only one stmt hook is allowed"));
    }
    if expr_hook_count > 1 {
        return Err(anyhow::anyhow!("Only one expr hook is allowed"));
    }
    Ok(result)
}

pub fn arg_counter(arg_types: &Vec<Type>) -> (usize, usize) {
    let mut min = 0;
    let mut max = 0;
    for arg_type in arg_types {
        match arg_type {
            Type::TOption(_) => {
                min += 0;
                max += 1;
            }
            Type::TArray(_) => {
                min += 0;
                max = usize::MAX;
            }
            _ => {
                min += 1;
                max += 1;
            }
        }
    }
    (min, max)
}
