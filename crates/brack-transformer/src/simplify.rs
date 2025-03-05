use brack_parser::cst::CST;

use crate::{angle, backslash, curly, document, error::TransformError, expr, square, stmt};

pub fn simplify(cst: &CST) -> (CST, Vec<TransformError>) {
    match cst {
        CST::Document(_) => document::simplify(cst),
        CST::Stmt(_) => stmt::simplify(cst),
        CST::Expr(_) => expr::simplify(cst),
        CST::Angle(_) => angle::simplify(cst),
        CST::Curly(_) => curly::simplify(cst),
        CST::Square(_) => square::simplify(cst),
        CST::BackSlash(_) => backslash::simplify(cst),
        node => (node.clone(), vec![]),
    }
}
