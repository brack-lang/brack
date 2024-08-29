use brack_parser::cst::{matches_kind, InnerNode, CST};
use brack_tokenizer::tokens::merge_location;

use crate::{error::TransformError, transform};

fn flatten_expr(cst: &CST) -> Vec<CST> {
    match cst {
        CST::Expr(node) => {
            let mut result = vec![];
            for child in node.clone().children {
                result.append(&mut flatten_expr(&child));
            }
            return result;
        }
        node => vec![node.clone()],
    }
}

pub fn transform(cst: &CST) -> (CST, Vec<TransformError>) {
    let node = match cst {
        CST::Bracket(node) => node,
        _ => panic!("Cannot pass non-bracket node to bracket::trasnform"),
    };
    let mut errors = vec![];
    let mut csts = vec![];
    for child in node.children.clone() {
        let (cst, mut node_errors) = transform::transform(&child);
        csts.push(cst);
        errors.append(&mut node_errors);
    }

    csts = flatten_expr(&CST::Bracket(InnerNode {
        id: node.id.clone(),
        children: csts,
        location: node.location.clone(),
    }));

    match (csts[0].clone(), csts[csts.len() - 1].clone()) {
        (CST::AngleBracketOpen(_), CST::AngleBracketClose(_)) => (),
        (CST::CurlyBracketOpen(_), CST::CurlyBracketClose(_)) => (),
        (CST::SquareBracketOpen(_), CST::SquareBracketClose(_)) => (),
        (CST::AngleBracketOpen(left), CST::CurlyBracketClose(right))
        | (CST::AngleBracketOpen(left), CST::SquareBracketClose(right))
        | (CST::CurlyBracketOpen(left), CST::AngleBracketClose(right))
        | (CST::CurlyBracketOpen(left), CST::SquareBracketClose(right))
        | (CST::SquareBracketOpen(left), CST::AngleBracketClose(right))
        | (CST::SquareBracketOpen(left), CST::CurlyBracketClose(right)) => errors.push(
            TransformError::MismatchedBracket(merge_location(&left.location, &right.location)),
        ),
        (CST::AngleBracketOpen(left), right) => errors.push(TransformError::AngleNotClosed(
            merge_location(&left.location, &right.location()),
        )),
        (CST::CurlyBracketOpen(left), right) => errors.push(TransformError::CurlyNotClosed(
            merge_location(&left.location, &right.location()),
        )),
        (CST::SquareBracketOpen(left), right) => errors.push(TransformError::SquareNotClosed(
            merge_location(&left.location, &right.location()),
        )),
        _ => panic!("Maybe cst parser is broken because CST::Bracket mush have bracket-open node first.")
    }

    (
        CST::Bracket(InnerNode {
            id: node.id.clone(),
            children: csts,
            location: node.location.clone(),
        }),
        errors,
    )
}
