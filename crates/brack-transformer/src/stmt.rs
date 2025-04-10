use brack_common::cst::{new_stmt, CST};
use brack_common::errors::TransformingError;

use crate::{simplify, utils::remove_elements_not_included_ast};

pub fn simplify(cst: &CST) -> (CST, Vec<TransformingError>) {
    let node = match cst {
        CST::Stmt(node) => node,
        _ => panic!("Cannot pass non-stmt node to stmt::simplify"),
    };
    let mut errors = vec![];
    let mut csts = vec![];

    for child in node.children.clone() {
        let (cst, mut node_errors) = simplify::simplify(&child);
        csts.push(cst);
        errors.append(&mut node_errors);
    }

    csts = remove_elements_not_included_ast(&csts);

    let mut stmt = new_stmt();
    for child in csts.clone() {
        stmt.add(child);
    }
    stmt.set_location(node.location.clone());
    (stmt, errors)
}
