use anyhow::Result;
use brack_plugin::{plugins::Plugins, types::Type};
use brack_transformer::ast::AST;

use crate::errors::LoweringError;
use crate::op_code::{self, OpCode};
use crate::{curly, square, text};

pub(crate) fn lowering(ast: &AST, plugins: &Plugins) -> Result<Vec<OpCode>, LoweringError> {
    match ast {
        AST::Expr(_) => (),
        _ => panic!("Expr must be an expr"),
    };
    let mut result = vec![];
    for child in ast.children() {
        let res = match child {
            AST::Expr(_) => lowering(child, &plugins)?,
            AST::Curly(_) => curly::lowering(child, &plugins)?,
            AST::Square(_) => square::lowering(child, &plugins)?,
            AST::Text(_) => text::lowering(child, &plugins)?,
            AST::Angle(_) => panic!("Angle must be expanded by the macro expander."),
            ast => panic!("Expr cannot contain the following node\n{}", ast),
        };
        result.extend(res);
    }

    result.push(op_code::OpCode::Join(ast.children().len()));

    if let Some(expr_hook_plugin_name) = plugins.expr_hook_plugin_name.clone() {
        result.push(OpCode::Call{
            plugin_name: expr_hook_plugin_name,
            function_name: "expr".to_string(),
            return_type: Type::TInline,
        });
    }
    Ok(result)
}
