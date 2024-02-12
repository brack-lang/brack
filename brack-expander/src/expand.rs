use anyhow::Result;
use brack_parser::ast::AST;
use brack_plugin::plugin::Plugins2;
use brack_sdk_rs::Type;
use extism::convert::Json;

fn expand_angle(overall_ast: &AST, ast: &AST, plugins: &mut Plugins2) -> Result<AST> {
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

    let (plugin, plugin_metadata_map) = plugins.get_mut(&module_name).ok_or_else(|| anyhow::anyhow!("Module {} not found", module_name))?;
    let plugin_metadata = plugin_metadata_map.get(&(ident_name.clone(), Type::TAST)).ok_or_else(|| anyhow::anyhow!("Command {} not found", ident_name))?;
    let Json(res) =
        plugin.call::<Json<(AST, String)>, Json<AST>>(&plugin_metadata.call_name, Json((overall_ast.clone(), ast.id())))?;

    Ok(res)
}

fn expand_other(overall_ast: &AST, ast: &AST, plugins: &mut Plugins2) -> Result<AST> {
    let mut children = vec![];
    match ast {
        AST::Text(_) => {
            return Ok(ast.clone());
        }
        _ => {}
    }
    for child in ast.children() {
        match child {
            AST::Angle(_) => children.push(expand_angle(overall_ast, &child, plugins)?),
            _ => children.push(expand_other(overall_ast, &child, plugins)?),
        }
    }
    Ok(ast.clone())
}

pub fn expander(ast: &AST, plugins: &mut Plugins2) -> Result<AST> {
    let overall_ast = ast.clone();
    Ok(expand_other(&overall_ast, ast, plugins)?)
}
