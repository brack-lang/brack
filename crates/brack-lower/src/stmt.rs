use anyhow::Result;
use brack_plugin::{plugins::Plugins, types::Type};
use brack_transformer::ast::AST;

use crate::errors::LoweringError;
use crate::op_code::{self, OpCode};
use crate::{curly, expr, square, text};

pub(crate) fn lowering(ast: &AST, plugins: &Plugins) -> Result<Vec<OpCode>, LoweringError> {
    let AST::Stmt(_) = ast else {
        return Err(LoweringError::Panic {
            message: format!("Stmt must be a stmt but found {:?}", ast),
            location: ast.location().clone(),
        });
    };
    let mut result = vec![];
    for child in ast.children() {
        let res = match child {
            AST::Expr(_) => expr::lowering(child, &plugins)?,
            AST::Curly(_) => curly::lowering(child, &plugins)?,
            _ => return Err(LoweringError::Panic {
                message: format!("Stmt must contain expr or curly but found {:?}", child),
                location: child.location().clone(),
            }),
        };
        result.extend(res);
    }

    if let Some(stmt_hook_plugin_name) = plugins.stmt_hook_plugin_name.clone() {
        result.push(OpCode::Call {
            plugin_name: stmt_hook_plugin_name,
            function_name: "stmt".to_string(),
            return_type: Type::TBlock,
        });
    }
    Ok(result)
}
