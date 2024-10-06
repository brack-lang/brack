use anyhow::Result;
use brack_plugin::plugin::Plugins;
use brack_sdk_rs::Type;
use brack_transformer::ast::AST;
use extism::convert::Json;

fn expand_angle(overall_ast: &AST, ast: &AST, plugins: &mut Plugins) -> Result<AST> {
    let mut module_name = String::from("");
    let mut ident_name = String::from("");

    for child in ast.children() {
        match child {
            AST::Module(node) => {
                module_name = node
                    .clone()
                    .value
                    .ok_or_else(|| anyhow::anyhow!("No value found"))?
            }
            AST::Ident(node) => {
                ident_name = node
                    .clone()
                    .value
                    .ok_or_else(|| anyhow::anyhow!("No value found"))?
            }
            _ => (),
        }
    }

    let (plugin, plugin_metadata_map) = plugins
        .get_mut(&module_name)
        .ok_or_else(|| anyhow::anyhow!("Module {} not found", module_name))?;
    let plugin_metadata = plugin_metadata_map
        .get(&(ident_name.clone(), Type::TAST))
        .ok_or_else(|| anyhow::anyhow!("Command {} not found", ident_name))?;
    let Json(res) = plugin.call::<Json<(AST, String)>, Json<AST>>(
        &plugin_metadata.call_name,
        Json((overall_ast.clone(), ast.id())),
    )?;

    Ok(res)
}

fn expand_other(overall_ast: &AST, ast: &AST, plugins: &mut Plugins) -> Result<AST> {
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

pub fn expander(ast: &AST, plugins: &mut Plugins) -> Result<AST> {
    let overall_ast = ast.clone();
    Ok(expand_other(&overall_ast, ast, plugins)?)
}
