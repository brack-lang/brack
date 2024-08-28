use brack_parser::cst::CST;
use uuid::Uuid;

use crate::{ast::{LeafNode, AST}, document, error::TransformError};

pub fn transform(cst: &CST) -> (AST, Vec<TransformError>) {
    match cst {
        CST::Document(_) => document::transform(&cst),
        CST::Stmt(InnerNode) => {}
        CST::Expr(InnerNode) => {}
        CST::Bracket(InnerNode) => {}
        CST::Module(LeafNode) => {}
        CST::Ident(LeafNode) => {}
        CST::Text(LeafNode) => {}
        CST::BackSlash(LeafNode) => {}
        CST::AngleBracketOpen(node)
        | CST::AngleBracketClose(node)
        | CST::SquareBracketOpen(node)
        | CST::SquareBracketClose(node)
        | CST::CurlyBracketOpen(node)
        | CST::CurlyBracketClose(node)
        | CST::Dot(node)
        | CST::Comma(node)
        | CST::Whitespace(node)
        | CST::Newline(node)
        | CST::EOF(node) => {
            (AST::Invalid(LeafNode {
                id: Uuid::new_v4().to_string(),
                value: node.value,
                location: node.location,
            }), vec![])
        }
    }
}
