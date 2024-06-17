use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use brack_sdk_rs::{MetaData, Type};
use extism::Plugin;
use extism_convert::Json;

pub type ModuleName = String;
pub type CommandName = String;
pub type Plugins = HashMap<ModuleName, (Plugin, PluginMetaDataMap)>;
pub type PluginMetaDataMap = HashMap<(CommandName, Type), MetaData>;

pub fn new_plugins<P: AsRef<Path>>(pathes: Vec<P>) -> Result<Plugins> {
    let mut result = HashMap::new();
    for path in pathes {
        let mut extism_plugin = Plugin::new(fs::read(&path)?, [], true)?;
        let Json(metadatas) = extism_plugin.call::<(), Json<Vec<MetaData>>>("get_metadata", ())?;
        let mut metadata_map = HashMap::new();
        for metadata in metadatas {
            metadata_map.insert(
                (metadata.command_name.clone(), metadata.return_type.clone()),
                metadata,
            );
        }
        let file_stem = path
            .as_ref()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let parts: Vec<&str> = file_stem.split('.').collect();
        let name = parts.get(0).map_or("", |s| *s).to_string();
        result.insert(name, (extism_plugin, metadata_map));
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
