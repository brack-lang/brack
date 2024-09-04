use anyhow::Result;
use brack_transformer::ast::AST;

use crate::text;

pub fn generate(ast: &AST) -> Result<String> {
    let mut result = vec![];
    for child in ast.children() {
        match child {
            AST::Text(_) => {
                result.push(text::generate(&child)?);
            }
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            _ => anyhow::bail!(
                "Ident cannot contain Document, Stmt, Expr, Curly, Square and Ident"
            ),
        }
    }
    Ok(result.join(" "))
}
