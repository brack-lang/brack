use brack_common::cst::{new_document, CST};
use brack_common::errors::TransformingError;

use crate::{simplify, utils::remove_elements_not_included_ast};

pub fn simplify(cst: &CST) -> (CST, Vec<TransformingError>) {
    let node = match cst {
        CST::Document(node) => node,
        _ => panic!("Cannot pass non-document node to document::simplify"),
    };
    let mut errors = vec![];
    let mut csts = vec![];

    for child in node.children.clone() {
        let (cst, mut node_errors) = simplify::simplify(&child);
        csts.push(cst);
        errors.append(&mut node_errors);
    }

    csts = remove_elements_not_included_ast(&csts);

    let mut document = new_document();
    document.set_location(node.location.clone());
    for child in csts {
        document.add(child);
    }
    (document, errors)
}
