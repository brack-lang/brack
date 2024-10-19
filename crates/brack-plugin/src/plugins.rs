use std::{collections::HashMap};

use anyhow::Result;
use brack_transformer::ast::AST;
use extism::{FromBytes, ToBytes};
use extism_convert::Json;

use crate::{plugin::Plugin, types::Type, value::Value};

pub struct Plugins {
    pub name_to_plugin: HashMap<String, Plugin>,
    document_hook_plugin_name: Option<String>,
    stmt_hook_plugin_name: Option<String>,
    expr_hook_plugin_name: Option<String>,
    text_hook_plugin_name: Option<String>,
}

impl Plugins {
    pub fn new(plugins: Vec<Plugin>) -> Result<Self> {
        let mut name_to_plugin = HashMap::new();
        let mut document_hook_plugin_name = None;
        let mut stmt_hook_plugin_name = None;
        let mut expr_hook_plugin_name = None;
        let mut text_hook_plugin_name = None;

        for plugin in plugins {
            let name = plugin.name.clone();
            if plugin.feature_flag.document_hook {
                if document_hook_plugin_name.is_some() {
                    return Err(anyhow::anyhow!("only one document hook is allowed"));
                }
                document_hook_plugin_name = Some(name.clone());
            }
            if plugin.feature_flag.stmt_hook {
                if stmt_hook_plugin_name.is_some() {
                    return Err(anyhow::anyhow!("only one stmt hook is allowed"));
                }
                stmt_hook_plugin_name = Some(name.clone());
            }
            if plugin.feature_flag.expr_hook {
                if expr_hook_plugin_name.is_some() {
                    return Err(anyhow::anyhow!("only one expr hook is allowed"));
                }
                expr_hook_plugin_name = Some(name.clone());
            }
            if plugin.feature_flag.text_hook {
                if text_hook_plugin_name.is_some() {
                    return Err(anyhow::anyhow!("only one text hook is allowed"));
                }
                text_hook_plugin_name = Some(name.clone());
            }
            name_to_plugin.insert(name, plugin);
        }

        Ok(Self {
            name_to_plugin,
            document_hook_plugin_name,
            stmt_hook_plugin_name,
            expr_hook_plugin_name,
            text_hook_plugin_name,
        })
    }

    pub fn argument_types(&self, module_name: &str, command_name: &str, typ: Type) -> Result<Vec<(String, Type)>> {
        let plugin = self
            .name_to_plugin
            .get(module_name)
            .ok_or_else(|| anyhow::anyhow!("plugin not found: {}", module_name))?;
        let metadata = plugin
            .signature_to_metadata
            .get(&(command_name.to_string(), typ))
            .ok_or_else(|| anyhow::anyhow!("command not found: {}", command_name))?;
        Ok(metadata.argument_types.clone())
    }

    fn call<T: for<'a> ToBytes<'a>, U: for<'a> FromBytes<'a>>(
        &mut self,
        plugin_name: &str,
        command_name: &str,
        return_type: Type,
        args: T,
    ) -> Result<U> {
        let plugin = self
            .name_to_plugin
            .get_mut(plugin_name)
            .ok_or_else(|| anyhow::anyhow!("plugin not found: {}", plugin_name))?;
        let result = plugin.call(command_name, return_type, args)?;
        Ok(result)
    }

    pub fn call_inline_command(
        &mut self,
        plugin_name: &str,
        command_name: &str,
        args: Vec<Value>,
    ) -> Result<String> {
        let result = self.call::<Json<Vec<Value>>, String>(
            plugin_name,
            command_name,
            Type::TInline,
            Json(args),
        )?;
        Ok(result)
    }

    pub fn call_block_command(
        &mut self,
        plugin_name: &str,
        command_name: &str,
        args: Vec<Value>,
    ) -> Result<String> {
        let result = self.call::<Json<Vec<Value>>, String>(
            plugin_name,
            command_name,
            Type::TBlock,
            Json(args),
        )?;
        Ok(result)
    }

    pub fn call_macro_command(
        &mut self,
        plugin_name: &str,
        command_name: &str,
        ast: AST,
        id: String,
    ) -> Result<AST> {
        let result = self.call::<Json<(AST, String)>, Json<AST>>(
            plugin_name,
            command_name,
            Type::TAST,
            Json((ast, id)),
        )?;
        let Json(ast) = result;
        Ok(ast)
    }

    pub fn call_document_hook(&mut self, args: Vec<Value>) -> Result<Option<String>> {
        let document_hook_plugin_name = self.document_hook_plugin_name.clone();
        if let Some(plugin_name) = document_hook_plugin_name {
            let result = self.call_block_command(&plugin_name, "document", args)?;
            return Ok(Some(result));
        }
        Ok(None)
    }

    pub fn call_stmt_hook(&mut self, args: Vec<Value>) -> Result<Option<String>> {
        let stmt_hook_plugin_name = self.stmt_hook_plugin_name.clone();
        if let Some(plugin_name) = stmt_hook_plugin_name {
            let result = self.call_block_command(&plugin_name, "stmt", args)?;
            return Ok(Some(result));
        }
        Ok(None)
    }

    pub fn call_expr_hook(&mut self, args: Vec<Value>) -> Result<Option<String>> {
        let expr_hook_plugin_name = self.expr_hook_plugin_name.clone();
        if let Some(plugin_name) = expr_hook_plugin_name {
            let result = self.call_inline_command(&plugin_name, "expr", args)?;
            return Ok(Some(result));
        }
        Ok(None)
    }

    pub fn call_text_hook(&mut self, args: Vec<Value>) -> Result<Option<String>> {
        let text_hook_plugin_name = self.text_hook_plugin_name.clone();
        if let Some(plugin_name) = text_hook_plugin_name {
            let result = self.call_inline_command(&plugin_name, "text", args)?;
            return Ok(Some(result));
        }
        Ok(None)
    }
}
