use brack_parser::cst::CST;

use crate::{ast::AST, error::TransformError, simplify};

pub fn transform(cst: &CST) -> (AST, Vec<TransformError>) {
    let (_cst, _errors) = simplify::simplify(&cst);
    todo!()
}
