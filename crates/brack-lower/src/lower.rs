use anyhow::Result;
use brack_plugin::{plugins::Plugins, types::Type};
use brack_transformer::ast::AST;

use crate::op_code::OpCode;
use crate::errors::LoweringError;
use crate::{curly, expr, square, stmt, text};

pub fn lowering(ast: &AST, plugins: Plugins) -> Result<Vec<OpCode>, LoweringError> {
    match ast {
        AST::Document(_) => (),
        _ => panic!("Document must be a document"),
    };
    let mut result = vec![];
    for child in ast.children() {
        let res = match child {
            AST::Stmt(_) => stmt::lowering(child, &plugins)?,
            AST::Expr(_) => expr::lowering(child, &plugins)?,
            AST::Curly(_) => curly::lowering(child, &plugins)?,
            AST::Square(_) => square::lowering(child, &plugins)?,
            AST::Text(_) => text::lowering(child, &plugins)?,
            AST::Angle(_) => panic!("Angle must be expanded by the macro expander."),
            ast => panic!("Document cannot contain the following node\n{}", ast),
        };
        result.extend(res);
    }

    result.push(OpCode::Join(ast.children().len()));

    if let Some(document_hook_plugin_name) = plugins.document_hook_plugin_name.clone() {
        result.push(OpCode::Call{
            plugin_name: document_hook_plugin_name,
            function_name: "document".to_string(),
            return_type: Type::TBlock,
        });
    }
    Ok(result)
}
