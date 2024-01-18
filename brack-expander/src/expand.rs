use anyhow::Result;
use brack_parser::ast::AST;
use brack_plugin::plugin::{Plugins, PluginMacroArgument};
use extism::convert::Json;

fn expand_angle(overall_ast: &AST, ast: &AST, plugins: &mut Plugins) -> Result<AST> {
    let mut module_name = String::from("");
    let mut ident_name = String::from("");

    for child in ast.children() {
        let res = match child {
            AST::Identifier(ast) => {
                let mut iter = ast.children.iter();
                let module_name_ast = iter.next().ok_or_else(|| {
                    anyhow::anyhow!("Failed to retrieve module or identifier name from AST.")
                })?;
                let ident_name_ast = iter.next().ok_or_else(|| {
                    anyhow::anyhow!("Failed to retrieve module or identifier name from AST.")
                })?;
                match (module_name_ast, ident_name_ast) {
                    (AST::Text(left), AST::Text(right)) => {
                        (left.value.clone(), right.value.clone())
                    }
                    _ => anyhow::bail!("Expected module name and identifier name as text nodes."),
                }
            }
            _ => anyhow::bail!("Angle must be expanded by the macro expander."),
        };
        module_name = res.0;
        ident_name = res.1;
        break;
    }

    let plugin_argument = PluginMacroArgument {
        ast: overall_ast.clone(),
        uuid: ast.id(),
    };
    let plugin = plugins.get_mut(&module_name).ok_or_else(|| {
        anyhow::anyhow!("Module '{}' not found during angle expansion.", module_name)
    })?;
    let res =
        plugin.call::<Json<PluginMacroArgument>, Json<AST>>(&ident_name, Json(plugin_argument))?;

    Ok(res.0)
}

fn expand_other(overall_ast: &AST, ast: &AST, plugins: &mut Plugins) -> Result<AST> {
    let mut children = vec![];
    for child in ast.children() {
        match child {
            AST::Angle(_) => children.push(expand_angle(overall_ast, &child, plugins)?),
            _ => children.push(expand_other(overall_ast, &child, plugins)?),
        }
    }
    Ok(ast.clone())
}

pub fn expander(ast: &AST, plugins: &mut Plugins) -> Result<AST> {
    let overall_ast = ast.clone();
    Ok(expand_other(&overall_ast, ast, plugins)?)
}
