use anyhow::Result;
use brack_transformer::ast::AST;

pub fn generate(ast: &AST) -> Result<String> {
    Ok(ast.value())
}
