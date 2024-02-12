use anyhow::Result;
use brack_parser::ast::AST;
use brack_plugin::plugin::Plugins2;

use crate::{curly, expr, identifier, square, stmt, text};

pub fn generate(ast: &AST, plugins: &mut Plugins2) -> Result<String> {
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Stmt(_) => stmt::generate(&child, plugins)?,
            AST::Expr(_) => expr::generate(&child, plugins)?,
            AST::Curly(_) => curly::generate(&child, plugins)?,
            AST::Square(_) => square::generate(&child, plugins)?,
            AST::Identifier(_) => identifier::generate(&child)?,
            AST::Text(_) => text::generate(&child)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            _ => anyhow::bail!("Document cannot contain Document"),
        };
        result.push_str(&res);
    }
    Ok(result)
}
