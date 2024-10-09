use anyhow::Result;
use brack_plugin::plugin::{Plugins, Value};
use brack_transformer::ast::AST;
use extism::convert::Json;

use crate::{curly, expr, square, stmt, text};

pub fn generate(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    match ast {
        AST::Document(_) => (),
        _ => anyhow::bail!("Document must be a document"),
    };
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Stmt(_) => stmt::generate(&child, plugins)?,
            AST::Expr(_) => expr::generate(&child, plugins)?,
            AST::Curly(_) => curly::generate(&child, plugins)?,
            AST::Square(_) => square::generate(&child, plugins)?,
            AST::Text(_) => text::generate(&child)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            ast => anyhow::bail!("Document cannot contain the following node\n{}", ast),
        };
        result.push_str(&res);
    }

    let plugin = plugins.get_mut("_document_hook");
    if let Some((plugin, _)) = plugin {
        return Ok(
            plugin.call::<Json<Vec<Value>>, String>("document", Json(vec![Value::Text(result)]))?
        );
    }
    Ok(result)
}
