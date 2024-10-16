use anyhow::Result;
use brack_plugin::{plugins::Plugins, value::Value};
use brack_transformer::ast::AST;

use crate::{curly, expr, square, text};

pub(crate) fn generate(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    match ast {
        AST::Stmt(_) => (),
        _ => anyhow::bail!("Stmt must be a stmt"),
    };
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Expr(_) => expr::generate(&child, plugins)?,
            AST::Curly(_) => curly::generate(&child, plugins)?,
            AST::Square(_) => square::generate(&child, plugins)?,
            AST::Text(_) => text::generate(&child)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            ast => anyhow::bail!("Stmt cannot contain the following node\n{}", ast),
        };
        result.push_str(&res);
    }

    let hook_result = plugins.call_stmt_hook(vec![Value::Text(result.clone())])?;
    match hook_result {
        Some(result) => Ok(result),
        None => Ok(result),
    }
}
