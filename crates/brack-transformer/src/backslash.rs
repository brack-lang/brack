use brack_common::cst::{new_invalid, CST};
use brack_common::errors::TransformingError;

pub fn simplify(cst: &CST) -> (CST, Vec<TransformingError>) {
    let node = match cst {
        CST::BackSlash(node) => node,
        _ => panic!("Cannot pass non-back-slash node to backslash::simplify"),
    };
    let mut errors = vec![];

    if node.children.is_empty() {
        errors.push(TransformingError::InvalidBackslash(node.location.clone()));
        return (new_invalid(node.location.clone()), errors);
    }

    (cst.children()[0].clone(), errors)
}
