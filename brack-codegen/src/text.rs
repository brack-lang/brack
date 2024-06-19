use anyhow::Result;
use brack_parser::ast::AST;

pub fn generate(ast: &AST) -> Result<String> {
    Ok(ast.value())
}
