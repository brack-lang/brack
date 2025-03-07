use anyhow::Result;
use brack_plugin::plugins::Plugins;
use brack_transformer::ast::AST;

use crate::{errors::LoweringError, op_code::OpCode};

pub(crate) fn lowering(ast: &AST, plugins: &Plugins) -> Result<Vec<OpCode>, LoweringError> {
    return Ok(vec![]);
}
