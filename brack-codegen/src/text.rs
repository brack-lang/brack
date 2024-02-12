use anyhow::Result;
use brack_sdk_rs::ast::AST;

pub fn generate(ast: &AST) -> Result<String> {
    Ok(ast.value())
}
