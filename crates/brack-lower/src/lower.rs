use anyhow::Result;
use brack_plugin::{plugins::Plugins, types::Type};
use brack_transformer::ast::AST;

use crate::errors::LoweringError;
use crate::op_code::OpCode;
use crate::{curly, expr, square, stmt, text};

pub fn lowering(ast: &AST, plugins: Plugins) -> Result<Vec<OpCode>, LoweringError> {
    let AST::Document(_) = ast else {
        return Err(LoweringError::Panic {
            message: format!("Document must be a document but found {:?}", ast),
            location: ast.location().clone(),
        });
    };
    let mut result = vec![];
    for child in ast.children() {
        let res = match child {
            AST::Stmt(_) => stmt::lowering(child, &plugins)?,
            _ => return Err(LoweringError::Panic {
                message: format!("Document must contain stmt but found {:?}", child),
                location: child.location().clone(),
            }),
        };
        result.extend(res);
    }

    result.push(OpCode::Join(ast.children().len()));

    if let Some(document_hook_plugin_name) = plugins.document_hook_plugin_name.clone() {
        result.push(OpCode::Call {
            plugin_name: document_hook_plugin_name,
            function_name: "document".to_string(),
            return_type: Type::TBlock,
        });
    }
    Ok(result)
}
