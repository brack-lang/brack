use core::fmt;
use std::{collections::HashMap, fs, path::Path};

use brack_common::{
    errors::PluginError,
    plugins::{CommandType, Plugin, Signature, Type, Value, ValueVec},
};
use extism::convert::Json;
use serde::{Deserialize, Serialize};

pub struct WasmPluginV1<IR: Serialize + for<'de> Deserialize<'de> + fmt::Display + Clone> {
    name: String,
    extism_plugin: extism::Plugin,
    signature_table: HashMap<(String, CommandType), Vec<Signature>>,
    _marker: std::marker::PhantomData<IR>,
}

impl<T: Serialize + for<'de> Deserialize<'de> + fmt::Display + Clone> Plugin for WasmPluginV1<T> {
    type IR = T;

    fn call(&mut self, signature: &Signature, args: &ValueVec<T>) -> Result<Value<T>, PluginError> {
        let signatures = self.get_signature(&signature.command_name, &signature.command_type)?;
        let signature = self.match_signature(&signatures, &signature.command_name, args)?;
        let result = self
            .extism_plugin
            .call::<Json<ValueVec<T>>, Json<Value<T>>>(&signature.callee, Json(args.clone()))
            .map_err(|_| PluginError::PluginCallError {
                name: self.name.clone(),
                signature: signature.clone(),
                args: args.to_string(),
            })?;
        let Json(result) = result;
        Ok(result)
    }
}

impl<T: Serialize + for<'de> Deserialize<'de> + fmt::Display + Clone> WasmPluginV1<T> {
    pub fn new<P: AsRef<Path>>(name: &str, path: &P) -> Result<Self, PluginError> {
        let file = match fs::read(path) {
            Ok(file) => file,
            Err(source) => {
                return Err(PluginError::PluginReadError {
                    name: name.to_string(),
                    source: source.to_string(),
                });
            }
        };
        let mut extism_plugin = match extism::Plugin::new(file, [], true) {
            Ok(plugin) => plugin,
            Err(source) => {
                return Err(PluginError::PluginCreateError {
                    name: name.to_string(),
                    source: source.to_string(),
                });
            }
        };
        let Json(signatures_from_plugin) = extism_plugin
            .call::<(), Json<Vec<Signature>>>("get_signatures", ())
            .map_err(|_| PluginError::PluginCallError {
                name: name.to_string(),
                signature: Signature {
                    command_name: "get_signatures".to_string(),
                    args: vec![],
                    return_type: Type::Invalid, // `get_signatures` is special and doesn't have a return type
                    command_type: CommandType::InlineCommand, // dummy
                    callee: "get_signatures".to_string(),
                },
                args: "()".to_string(),
            })?;
        let signature_table =
            signatures_from_plugin
                .iter()
                .fold(HashMap::new(), |mut acc, signature| {
                    acc.entry((
                        signature.command_name.clone(),
                        signature.command_type.clone(),
                    ))
                    .or_insert_with(Vec::new)
                    .push(signature.clone());
                    acc
                });
        Ok(Self {
            name: name.to_string(),
            extism_plugin,
            signature_table,
            _marker: std::marker::PhantomData,
        })
    }

    fn get_signature(
        &self,
        command_name: &str,
        command_type: &CommandType,
    ) -> Result<Vec<Signature>, PluginError> {
        let signatures = self
            .signature_table
            .get(&(command_name.to_string(), command_type.clone()))
            .ok_or_else(|| PluginError::SignatureNotFound {
                name: self.name.clone(),
                command_name: command_name.to_string(),
                command_type: command_type.clone(),
            })?;
        if signatures.is_empty() {
            return Err(PluginError::SignatureNotFound {
                name: self.name.clone(),
                command_name: command_name.to_string(),
                command_type: command_type.clone(),
            });
        }
        Ok(signatures.clone())
    }

    fn match_signature(
        &self,
        signatures: &[Signature],
        command_name: &str,
        args: &ValueVec<T>,
    ) -> Result<Signature, PluginError> {
        for signature in signatures {
            let signature_args_type = signature
                .args
                .iter()
                .map(|(_, typ)| typ.clone())
                .collect::<Vec<_>>();
            let args_type = args.0.iter().map(|arg| arg.to_type()).collect::<Vec<_>>();
            if signature_args_type == args_type {
                return Ok(signature.clone());
            }
        }
        Err(PluginError::SignatureNotMatched {
            name: self.name.clone(),
            command_name: command_name.to_string(),
            args: args.0.iter().map(|arg| arg.to_string()).collect(),
        })
    }
}
