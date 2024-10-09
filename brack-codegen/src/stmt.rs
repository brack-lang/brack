use anyhow::Result;
use brack_plugin::plugin::{Plugins, Value};
use brack_transformer::ast::AST;
use extism::convert::Json;

use crate::{curly, expr, square, text};

pub fn generate(ast: &AST, plugins: &mut Plugins) -> Result<String> {
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

    let plugin = plugins.get_mut("_stmt_hook");
    if let Some((plugin, _)) = plugin {
        return Ok(
            plugin.call::<Json<Vec<Value>>, String>("stmt", Json(vec![Value::Text(result)]))?
        );
    }

    Ok(result)
}
