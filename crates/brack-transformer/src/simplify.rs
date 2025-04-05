use brack_common::cst::CST;
use brack_common::transformer_errors::TransformError;

use crate::{angle, backslash, curly, document, expr, square, stmt};

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
