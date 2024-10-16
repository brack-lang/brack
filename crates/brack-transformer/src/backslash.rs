use brack_parser::cst::{new_invalid, CST};

use crate::error::TransformError;

pub fn simplify(cst: &CST) -> (CST, Vec<TransformError>) {
    let node = match cst {
        CST::BackSlash(node) => node,
        _ => panic!("Cannot pass non-back-slash node to backslash::simplify"),
    };
    let mut errors = vec![];

    if node.children.is_empty() {
        errors.push(TransformError::InvalidBackslash(node.location.clone()));
        return (new_invalid(node.location.clone()), errors);
    }

    (cst.children()[0].clone(), errors)
}
