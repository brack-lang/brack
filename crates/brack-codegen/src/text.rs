use anyhow::Result;
use brack_plugin::{plugins::Plugins, value::Value};
use brack_transformer::ast::AST;

pub(crate) fn generate(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    let result = ast
        .value()
        .ok_or_else(|| anyhow::anyhow!("No value found"))?
        .to_string();
    let hook_result = plugins.call_text_hook(vec![Value::Text(result.clone())])?;
    match hook_result {
        Some(result) => Ok(result),
        None => Ok(result),
    }
}
