use anyhow::Result;
use brack_plugin::{plugins::Plugins, types::{arg_counter, Type}, value::Value};
use brack_transformer::ast::AST;

use crate::{expr, square, text};

pub(crate) fn generate(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    match ast {
        AST::Curly(_) => (),
        _ => anyhow::bail!("Curly must be a curly"),
    };
    let mut arguments = vec![];
    let module = ast
        .children()
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("Curly must contain module"))?;
    let ident = ast
        .children()
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Curly must contain identifier"))?;
    for (_, child) in ast.children().iter().skip(2).enumerate() {
        let res = match child {
            AST::Expr(_) => expr::generate(&child, plugins)?,
            AST::Curly(_) => generate(&child, plugins)?,
            AST::Square(_) => square::generate(&child, plugins)?,
            AST::Text(_) => text::generate(&child, plugins)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            ast => anyhow::bail!("Curly cannot contain the following node\n{}", ast),
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

    let arg_types = plugins.argument_types(&module_name, &ident_name)?;

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

    let text = plugins.call_block_command(&module_name, &ident_name, args)?;
    Ok(text)
}
