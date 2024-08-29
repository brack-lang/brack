use brack_parser::cst::CST;
use uuid::Uuid;

use crate::{
    ast::{InnerNode, AST},
    error::TransformError,
    transform,
};

pub fn transform(cst: &CST) -> (CST, Vec<TransformError>) {
    let node = match cst {
        CST::Document(node) => node,
        _ => panic!("Cannot pass non-document node to transform_document"),
    };
    let mut errors = vec![];
    let mut asts = vec![];
    for child in node.children.clone() {
        let (ast, mut node_errors) = transform::transform(&child);
        asts.push(ast);
        errors.append(&mut node_errors);
    }

    // ignored 対象のノードを全て削除する

    (
        CST::Document(InnerNode {
            id: Uuid::new_v4().to_string(),
            children: asts,
            location: node.location.clone(),
        }),
        errors,
    )
}
