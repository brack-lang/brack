use anyhow::Result;
use brack_plugin::{plugins::Plugins, types::Type};
use brack_transformer::ast::AST;

use crate::errors::LoweringError;
use crate::op_code::{self, OpCode};
use crate::{square, text};

pub(crate) fn lowering(ast: &AST, plugins: &Plugins) -> Result<Vec<OpCode>, LoweringError> {
    let AST::Expr(_) = ast else {
        return Err(LoweringError::Panic {
            message: format!("Expr must be an expr but found {:?}", ast),
            location: ast.location().clone(),
        });
    };
    let mut result = vec![];
    for child in ast.children() {
        let res = match child {
            AST::Square(_) => square::lowering(child, &plugins)?,
            AST::Text(_) => text::lowering(child, &plugins)?,
            _ => {
                return Err(LoweringError::Panic {
                    message: format!("Expr must contain square or text but found {:?}", child),
                    location: child.location().clone(),
                })
            }
        };
        result.extend(res);
    }

    result.push(op_code::OpCode::Join(ast.children().len()));

    if let Some(expr_hook_plugin_name) = plugins.expr_hook_plugin_name.clone() {
        result.push(OpCode::Call {
            plugin_name: expr_hook_plugin_name,
            function_name: "expr".to_string(),
            return_type: Type::TInline,
        });
    }
    Ok(result)
}
