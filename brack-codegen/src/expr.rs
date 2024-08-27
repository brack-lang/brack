use anyhow::Result;
use brack_transformer::ast::AST;
use brack_plugin::plugin::Plugins;

use crate::{identifier, square, text};

pub fn generate(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Square(_) => square::generate(&child, plugins)?,
            AST::Identifier(_) => identifier::generate(&child)?,
            AST::Text(_) => text::generate(&child)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            _ => anyhow::bail!("Expr cannot contain Document, Stmt and Expr"),
        };
        result.push_str(&res);
    }
    Ok(result)
}
