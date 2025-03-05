use anyhow::Result;
use brack_plugin::plugins::Plugins;
use brack_transformer::ast::AST;

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

    let new_ast =
        plugins.call_macro_command(&module_name, &ident_name, overall_ast.clone(), ast.id())?;
    Ok(new_ast)
}

fn expand_other(overall_ast: &AST, ast: &AST, plugins: &mut Plugins) -> Result<AST> {
    let mut children = vec![];
    match ast {
        AST::Text(_) => {
            return Ok(ast.clone());
        }
        AST::Module(_) => {
            return Ok(ast.clone());
        }
        AST::Ident(_) => {
            return Ok(ast.clone());
        }
        _ => {}
    }
    for child in ast.children() {
        match child {
            AST::Angle(_) => children.push(expand_angle(overall_ast, child, plugins)?),
            _ => children.push(expand_other(overall_ast, child, plugins)?),
        }
    }
    Ok(ast.clone())
}

pub fn expander(ast: &AST, plugins: &mut Plugins) -> Result<AST> {
    let overall_ast = ast.clone();
    expand_other(&overall_ast, ast, plugins)
}
