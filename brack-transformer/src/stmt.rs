use brack_parser::cst::{InnerNode, CST};
use uuid::Uuid;

use crate::{error::TransformError, simplify};

pub fn simplify(cst: &CST) -> (CST, Vec<TransformError>) {
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

    (
        CST::Stmt(InnerNode {
            id: Uuid::new_v4().to_string(),
            children: csts,
            location: node.location.clone(),
        }),
        errors,
    )
}
