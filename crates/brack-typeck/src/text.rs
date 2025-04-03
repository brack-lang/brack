use anyhow::Result;
use brack_transformer::ast::AST;

use crate::{errors::LoweringError, op_code::OpCode};

pub(crate) fn lowering(ast: &AST) -> Result<Vec<OpCode>, LoweringError> {
    let AST::Text(_) = ast else {
        return Err(LoweringError::Panic {
            message: format!("Text must be a text but found {:?}", ast),
            location: ast.location().clone(),
        });
    };

    let mut result = vec![];

    let text_value = ast.value().ok_or_else(|| LoweringError::Panic {
        message: "Text value is missing".to_string(),
        location: ast.location().clone(),
    })?;

    result.push(OpCode::Push(text_value.clone()));

    Ok(result)
}
