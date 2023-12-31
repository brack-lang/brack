use anyhow::Result;
use extism::convert::Json;

use crate::{
    ast::AST,
    plugins::{PluginArgument, Plugins},
};

fn generate_document(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Stmt(_) => generate_stmt(&child, plugins)?,
            AST::Expr(_) => generate_expr(&child, plugins)?,
            AST::Curly(_) => generate_curly(&child, plugins)?,
            AST::Square(_) => generate_square(&child, plugins)?,
            AST::Identifier(_) => generate_identifier(&child)?,
            AST::Text(_) => generate_text(&child)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            _ => anyhow::bail!("Document cannot contain Document"),
        };
        result.push_str(&res);
    }
    Ok(result)
}

fn generate_stmt(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Expr(_) => generate_expr(&child, plugins)?,
            AST::Curly(_) => generate_curly(&child, plugins)?,
            AST::Square(_) => generate_square(&child, plugins)?,
            AST::Identifier(_) => generate_identifier(&child)?,
            AST::Text(_) => generate_text(&child)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            _ => anyhow::bail!("Stmt cannot contain Document and Stmt"),
        };
        result.push_str(&res);
    }
    Ok(result + "\n")
}

fn generate_expr(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Square(_) => generate_square(&child, plugins)?,
            AST::Identifier(_) => generate_identifier(&child)?,
            AST::Text(_) => generate_text(&child)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            _ => anyhow::bail!("Expr cannot contain Document, Stmt and Expr"),
        };
        result.push_str(&res);
    }
    Ok(result)
}

fn generate_curly(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    let mut module_name = String::from("");
    let mut ident_name = String::from("");

    let mut arguments = vec![];
    for (i, child) in ast.children().iter().enumerate() {
        let res = match child {
            AST::Expr(_) => generate_expr(&child, plugins)?,
            AST::Curly(_) => generate_curly(&child, plugins)?,
            AST::Square(_) => generate_square(&child, plugins)?,
            AST::Identifier(_) => generate_identifier(&child)?,
            AST::Text(_) => generate_text(&child)?,
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

    let plugin_argument = PluginArgument::new(arguments);
    let plugin = plugins
        .get_mut(&module_name)
        .ok_or_else(|| anyhow::anyhow!("Module {} not found", module_name))?;
    let res = plugin.call::<Json<PluginArgument>, &str>(&ident_name, Json(plugin_argument))?;

    Ok(format!("{}", res))
}

fn generate_square(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    let mut module_name = String::from("");
    let mut ident_name = String::from("");

    let mut arguments = vec![];
    for (i, child) in ast.children().iter().enumerate() {
        let res = match child {
            AST::Expr(_) => generate_expr(&child, plugins)?,
            AST::Curly(_) => generate_curly(&child, plugins)?,
            AST::Square(_) => generate_square(&child, plugins)?,
            AST::Identifier(_) => generate_identifier(&child)?,
            AST::Text(_) => generate_text(&child)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            _ => anyhow::bail!("Square cannot contain Document, Stmt, Expr and Square"),
        };
        if i == 0 {
            let (module, ident) = match res.split_once(" ") {
                Some((module, ident)) => (module, ident),
                None => anyhow::bail!("Square must contain module and identifier"),
            };
            module_name = module.to_string();
            ident_name = ident.to_string();
        } else {
            arguments.push(res);
        }
    }

    let plugin_argument = PluginArgument::new(arguments);
    let plugin = plugins
        .get_mut(&module_name)
        .ok_or_else(|| anyhow::anyhow!("Module {} not found", module_name))?;
    let res = plugin.call::<Json<PluginArgument>, &str>(&ident_name, Json(plugin_argument))?;

    Ok(format!("{}", res))
}

fn generate_identifier(ast: &AST) -> Result<String> {
    let mut result = vec![];
    for child in ast.children() {
        match child {
            AST::Text(_) => {
                result.push(generate_text(&child)?);
            }
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            _ => anyhow::bail!(
                "Identifier cannot contain Document, Stmt, Expr, Curly, Square and Identifier"
            ),
        }
    }
    Ok(result.join(" "))
}

fn generate_text(ast: &AST) -> Result<String> {
    Ok(ast.value())
}

pub fn generate(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    generate_document(ast, plugins)
}
