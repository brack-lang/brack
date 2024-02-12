use anyhow::Result;
use brack_plugin::plugin::{arg_counter, Plugins2};
use brack_sdk_rs::{ast::AST, Type, Value};
use extism::convert::Json;

use crate::{expr, identifier, square, text};

pub fn generate(ast: &AST, plugins: &mut Plugins2) -> Result<String> {
    let mut module_name = String::from("");
    let mut ident_name = String::from("");

    let mut arguments = vec![];
    for (i, child) in ast.children().iter().enumerate() {
        let res = match child {
            AST::Expr(_) => expr::generate(&child, plugins)?,
            AST::Curly(_) => generate(&child, plugins)?,
            AST::Square(_) => square::generate(&child, plugins)?,
            AST::Identifier(_) => identifier::generate(&child)?,
            AST::Text(_) => text::generate(&child)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            _ => anyhow::bail!("Curly cannot contain Document, Stmt, Expr and Curly"),
        };
        if i == 0 {
            let (module, ident) = match res.split_once(" ") {
                Some((module, ident)) => (module, ident),
                None => anyhow::bail!("Curly must contain module and identifier"),
            };
            module_name = module.to_string();
            ident_name = ident.to_string();
        } else {
            arguments.push(res);
        }
    }

    let (plugin, plugin_metadata_map) = plugins.get_mut(&module_name).ok_or_else(|| anyhow::anyhow!("Module {} not found", module_name))?;
    let plugin_metadata = plugin_metadata_map.get(&(ident_name.clone(), Type::TBlock)).ok_or_else(|| anyhow::anyhow!("Command {} not found", ident_name))?;
    let arg_types = &plugin_metadata.argument_types;
    
    let (min, max) = arg_counter(&arg_types.iter().map(|(_, t)| t).cloned().collect::<Vec<_>>());

    if arguments.len() < min {
        // TODO: show the signature of the command 
        anyhow::bail!("{} requires at least {} arguments", ident_name, min);
    }
    if arguments.len() > max {
        // TODO: show the signature of the command
        anyhow::bail!("{} requires at most {} arguments", ident_name, max);
    }

    let mut args = vec![];
    for (i, (_, t)) in arg_types.iter().enumerate() {
        let arg = match t {
            Type::TOption(_) => {
                if i < arguments.len() {
                    Value::TextOption(Some(arguments[i].clone()))
                } else {
                    Value::TextOption(None)
                }
            }
            Type::TArray(_) => {
                Value::TextArray(arguments[i..].iter().map(|s| s.clone()).collect())
            },
            _ => {
                Value::Text(arguments[i].clone())
            }
        };
        args.push(arg);
    }

    Ok(plugin.call::<Json<Vec<Value>>, String>(&ident_name, Json(args))?)
}
