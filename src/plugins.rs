use std::{collections::HashMap, path::Path};

use anyhow::Result;
use extism::{Manifest, Plugin, Wasm};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ast::AST;

pub type Plugins = HashMap<String, Plugin>;

#[derive(Debug, Serialize, Deserialize)]
pub enum PluginArgument {
    Arg0,
    Arg1(String),
    Arg2(String, String),
    Arg3(String, String, String),
    Arg4(String, String, String, String),
    Arg5(String, String, String, String, String),
    Arg6(String, String, String, String, String, String),
    Arg7(String, String, String, String, String, String, String),
    Arg8(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginMacroArgument {
    pub ast: AST,
    pub uuid: Uuid,
}

impl PluginArgument {
    pub fn new(args: Vec<String>) -> Self {
        match args.len() {
            0 => PluginArgument::Arg0,
            1 => PluginArgument::Arg1(args[0].clone()),
            2 => PluginArgument::Arg2(args[0].clone(), args[1].clone()),
            3 => PluginArgument::Arg3(args[0].clone(), args[1].clone(), args[2].clone()),
            4 => PluginArgument::Arg4(
                args[0].clone(),
                args[1].clone(),
                args[2].clone(),
                args[3].clone(),
            ),
            5 => PluginArgument::Arg5(
                args[0].clone(),
                args[1].clone(),
                args[2].clone(),
                args[3].clone(),
                args[4].clone(),
            ),
            6 => PluginArgument::Arg6(
                args[0].clone(),
                args[1].clone(),
                args[2].clone(),
                args[3].clone(),
                args[4].clone(),
                args[5].clone(),
            ),
            7 => PluginArgument::Arg7(
                args[0].clone(),
                args[1].clone(),
                args[2].clone(),
                args[3].clone(),
                args[4].clone(),
                args[5].clone(),
                args[6].clone(),
            ),
            8 => PluginArgument::Arg8(
                args[0].clone(),
                args[1].clone(),
                args[2].clone(),
                args[3].clone(),
                args[4].clone(),
                args[5].clone(),
                args[6].clone(),
                args[7].clone(),
            ),
            _ => panic!("Too many arguments"),
        }
    }
}

pub fn new_plugins<P: AsRef<Path>>(pathes: Vec<P>) -> Result<Plugins> {
    let mut hash = HashMap::new();
    for path in pathes {
        let wasm = Wasm::file(&path);
        let manifest = Manifest::new([wasm]);
        let plugin = Plugin::new(&manifest, [], true)?;

        let file_stem = path
            .as_ref()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let parts: Vec<&str> = file_stem.split('.').collect();
        let name = parts.get(0).map_or("", |s| *s).to_string();

        hash.insert(name, plugin);
    }
    Ok(hash)
}
