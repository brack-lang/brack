use anyhow::Result;
use brack_plugin::plugin::{arg_counter, Plugins, Type, Value};
use brack_transformer::ast::AST;
use extism::convert::Json;

use crate::{curly, expr, text};

pub fn generate(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    match ast {
        AST::Square(_) => (),
        _ => anyhow::bail!("Square must be a square"),
    };
    let mut arguments = vec![];
    let module = ast
        .children()
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("Square must contain module and identifier"))?;
    let ident = ast
        .children()
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Square must contain module and identifier"))?;
    for (_, child) in ast.children().iter().skip(2).enumerate() {
        let res = match child {
            AST::Expr(_) => expr::generate(&child, plugins)?,
            AST::Curly(_) => curly::generate(&child, plugins)?,
            AST::Square(_) => generate(&child, plugins)?,
            AST::Text(_) => text::generate(&child)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            ast => anyhow::bail!("Square cannot contain the following node\n{}", ast),
        };
        arguments.push(res);
    }

    let module_name = match module {
        AST::Module(module) => module.value.clone(),
        _ => anyhow::bail!("Module must be a module"),
    };
    let module_name = match module_name {
        Some(module_name) => module_name,
        None => anyhow::bail!("Module name must be a string"),
    };

    let ident_name = match ident {
        AST::Ident(ident) => ident.value.clone(),
        _ => anyhow::bail!("Identifier must be an identifier"),
    };
    let ident_name = match ident_name {
        Some(ident_name) => ident_name,
        None => anyhow::bail!("Identifier name must be a string"),
    };

    let (plugin, plugin_metadata_map) = plugins
        .get_mut(&module_name)
        .ok_or_else(|| anyhow::anyhow!("Module {} not found", module_name))?;
    let plugin_metadata = plugin_metadata_map
        .get(&(ident_name.clone(), Type::TInline))
        .ok_or_else(|| anyhow::anyhow!("Command {} not found", ident_name))?;
    let arg_types = &plugin_metadata.argument_types;

    let (min, max) = arg_counter(
        &arg_types
            .iter()
            .map(|(_, t)| t)
            .cloned()
            .collect::<Vec<_>>(),
    );

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
            Type::TArray(_) => Value::TextArray(arguments[i..].iter().map(|s| s.clone()).collect()),
            _ => Value::Text(arguments[i].clone()),
        };
        args.push(arg);
    }

    Ok(plugin.call::<Json<Vec<Value>>, String>(&plugin_metadata.call_name, Json(args))?)
}
