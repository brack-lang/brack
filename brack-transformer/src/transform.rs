use brack_parser::cst::CST;
use uuid::Uuid;

use crate::{
    ast::{LeafNode, AST},
    document,
    error::TransformError,
};

pub fn transform(cst: &CST) -> (CST, Vec<TransformError>) {
    match cst {
        CST::Document(_) => document::transform(&cst),
        CST::Stmt(InnerNode) => {}
        CST::Expr(InnerNode) => {}
        CST::Bracket(InnerNode) => {}
        CST::BackSlash(LeafNode) => {}
        node => (node.clone(), vec![])
    }
}
