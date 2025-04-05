use brack_common::cst::{new_expr, CST};
use brack_common::transformer_errors::TransformError;

use crate::simplify;

pub fn simplify(cst: &CST) -> (CST, Vec<TransformError>) {
    let node = match cst {
        CST::Expr(node) => node,
        _ => panic!("Cannot pass non-expr node to expr::simplify"),
    };
    let mut errors = vec![];
    let mut csts = vec![];

    for child in node.children.clone() {
        let (cst, mut node_errors) = simplify::simplify(&child);
        csts.push(cst);
        errors.append(&mut node_errors);
    }

    let mut expr = new_expr();
    for child in csts {
        expr.add(child);
    }
    expr.set_location(node.location.clone());
    (expr, errors)
}
