use anyhow::Result;
use brack_plugin::{plugins::Plugins, value::Value};
use brack_transformer::ast::AST;

use crate::{curly, expr, square, stmt, text};

pub fn generate(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    match ast {
        AST::Document(_) => (),
        _ => anyhow::bail!("Document must be a document"),
    };
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Stmt(_) => stmt::generate(child, plugins)?,
            AST::Expr(_) => expr::generate(child, plugins)?,
            AST::Curly(_) => curly::generate(child, plugins)?,
            AST::Square(_) => square::generate(child, plugins)?,
            AST::Text(_) => text::generate(child, plugins)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            ast => anyhow::bail!("Document cannot contain the following node\n{}", ast),
        };
        result.push_str(&res);
    }

    let hook_result = plugins.call_document_hook(vec![Value::Text(result.clone())])?;
    match hook_result {
        Some(result) => Ok(result),
        None => Ok(result),
    }
}
