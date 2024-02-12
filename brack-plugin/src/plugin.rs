use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use brack_sdk_rs::{MetaData, Type};
use extism::{Manifest, Plugin, Wasm};
use extism_convert::{Json, Protobuf};
use modsurfer_plugins::MODSURFER_WASM;
use modsurfer_proto::api::Module;
use protobuf::MessageField;

pub type Plugins = HashMap<String, Plugin>;

pub type ModuleName = String;
pub type CommandName = String;
pub type Plugins2 = HashMap<ModuleName, (Plugin, PluginMetaDataMap)>;
pub type PluginMetaDataMap = HashMap<(CommandName, Type), MetaData>;

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

pub fn get_metadata_functions<P: AsRef<Path>>(
    modsurfer: &mut Plugin,
    path: P,
) -> Result<Vec<String>> {
    let wasm = fs::read(path)?;
    let Protobuf(data) =
        modsurfer.call::<Vec<u8>, Protobuf<Module>>("parse_module", wasm.clone())?;
    let mut all_functions = vec![];
    for export in data.exports {
        if let MessageField(Some(f)) = export.func {
            all_functions.push(f.name);
        }
    }
    let result = all_functions
        .iter()
        .filter(|&x| x.starts_with("metadata_"))
        .cloned()
        .collect::<Vec<_>>();
    Ok(result)
}

pub fn new_plugins2<P: AsRef<Path>>(pathes: Vec<P>) -> Result<Plugins2> {
    let mut result = HashMap::new();
    let mut modsurfer = Plugin::new(MODSURFER_WASM, [], false)?;
    for path in pathes {
        let metadata_functions = get_metadata_functions(&mut modsurfer, &path)?;
        let mut plugin = Plugin::new(fs::read(&path)?, [], false)?;
        let mut metadata_map = HashMap::new();
        for metadata_function in metadata_functions {
            let Json(metadata) =
                plugin.call::<(), extism_convert::Json<MetaData>>(&metadata_function, ())?;
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
        result.insert(name, (plugin, metadata_map));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;
    use brack_sdk_rs::{Type, Value};
    use extism_convert::Json;

    #[test]
    fn test1() {
        let mut plugins2 = new_plugins2(vec!["./test.html.wasm"]).unwrap();
        let (plugin, metadata_map) = plugins2.get_mut("test").unwrap();

        match metadata_map.get(&("@".to_string(), Type::TInline)) {
            Some(metadata) => {
                match plugin.call::<Json<Vec<Value>>, String>(
                    metadata.call_name.clone(),
                    Json(vec![
                        Value::Text("https://github.com/momeemt".to_string()),
                        Value::TextOption(Some("GitHub".to_string())),
                    ]),
                ) {
                    Ok(processed) => assert_eq!(
                        processed,
                        "<a href=\"https://github.com/momeemt\">GitHub</a>"
                    ),
                    Err(e) => panic!("{:?}", e),
                }
            }
            _ => panic!("No metadata found for @"),
        }
    }
}
