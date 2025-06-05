use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
};

use crate::{feature_flag::FeatureFlag, metadata::Metadata, types::Type};
use anyhow::Result;
use extism::{FromBytes, Plugin as ExtismPlugin, ToBytes};
use extism_convert::Json;

#[derive(Debug)]
pub struct Plugin {
    pub name: String,
    pub(crate) extism_plugin: ExtismPlugin,
    pub signature_to_metadata: HashMap<(String, Type), Metadata>,
    pub(crate) feature_flag: FeatureFlag,
}

impl Plugin {
    pub fn new<P: AsRef<Path>>(
        name: &str,
        wasm_bin_path: P,
        feature_flag: FeatureFlag,
    ) -> Result<Self> {
        let wasm_bin = fs::read(wasm_bin_path)?;
        let mut extism_plugin = ExtismPlugin::new(wasm_bin, [], true)?;
        let Json(metadatas) = extism_plugin.call::<(), Json<Vec<Metadata>>>("get_metadata", ())?;
        let mut signature_to_metadata = HashMap::new();

        let mut exists_document_hook = false;
        let mut exists_stmt_hook = false;
        let mut exists_expr_hook = false;
        let mut exists_text_hook = false;

        for metadata in metadatas {
            let command_name = metadata.command_name.clone();
            let return_type = metadata.return_type.clone();
            if command_name == "document" && feature_flag.document_hook {
                if return_type != Type::TBlock {
                    return Err(anyhow::anyhow!("document hook must return TBlock"));
                }
                exists_document_hook = true;
            }
            if command_name == "stmt" && feature_flag.stmt_hook {
                if return_type != Type::TBlock {
                    return Err(anyhow::anyhow!("stmt hook must return TBlock"));
                }
                exists_stmt_hook = true;
            }
            if command_name == "expr" && feature_flag.expr_hook {
                if return_type != Type::TInline {
                    return Err(anyhow::anyhow!("expr hook must return TInline"));
                }
                exists_expr_hook = true;
            }
            if command_name == "text" && feature_flag.text_hook {
                if return_type != Type::TInline {
                    return Err(anyhow::anyhow!("text hook must return TInline"));
                }
                exists_text_hook = true;
            }
            signature_to_metadata.insert((command_name, return_type), metadata);
        }

        if feature_flag.document_hook && !exists_document_hook {
            return Err(anyhow::anyhow!("document hook not found"));
        }
        if feature_flag.stmt_hook && !exists_stmt_hook {
            return Err(anyhow::anyhow!("stmt hook not found"));
        }
        if feature_flag.expr_hook && !exists_expr_hook {
            return Err(anyhow::anyhow!("expr hook not found"));
        }
        if feature_flag.text_hook && !exists_text_hook {
            return Err(anyhow::anyhow!("text hook not found"));
        }

        Ok(Self {
            name: name.to_string(),
            extism_plugin,
            signature_to_metadata,
            feature_flag,
        })
    }

    pub(crate) fn call<T: for<'a> ToBytes<'a>, U: for<'a> FromBytes<'a>>(
        &mut self,
        command_name: &str,
        return_type: Type,
        args: T,
    ) -> Result<U> {
        let metadata = self
            .signature_to_metadata
            .get(&(command_name.to_string(), return_type))
            .ok_or_else(|| anyhow::anyhow!("metadata not found: {}", command_name))?;
        let result = self
            .extism_plugin
            .call::<T, U>(metadata.call_name.clone(), args)?;
        Ok(result)
    }
}
