use anyhow::Result;
use brack_plugin::plugin::Plugins;
use brack_transformer::ast::AST;

use crate::{curly, square, text};

pub fn generate(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    match ast {
        AST::Expr(_) => (),
        _ => anyhow::bail!("Expr must be an expr"),
    };
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Curly(_) => curly::generate(&child, plugins)?,
            AST::Square(_) => square::generate(&child, plugins)?,
            AST::Text(_) => text::generate(&child)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            AST::Expr(_) => generate(&child, plugins)?,
            ast => anyhow::bail!("Expr cannot contain the following node\n{}", ast),
        };
        result.push_str(&res);
    }
    Ok(result)
}
