use anyhow::Result;
use brack_transformer::ast::AST;

pub fn generate(ast: &AST) -> Result<String> {
    Ok(ast
        .value()
        .ok_or_else(|| anyhow::anyhow!("No value found"))?
        .to_string())
}
