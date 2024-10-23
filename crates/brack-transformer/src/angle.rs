use brack_parser::cst::{InnerNode, CST};
use brack_tokenizer::tokens::merge_location;

use crate::{
    error::TransformError,
    simplify,
    utils::{
        check_if_dot, check_if_ident_or_angle_bracket, check_if_module_or_angle_bracket,
        check_unexpected_dot, check_valid_arguments, remove_elements_not_included_ast,
        remove_whitespaces_and_newlines,
    },
};

fn check_if_the_first_and_last_node_are_brackets(csts: &[CST]) -> Vec<TransformError> {
    let mut errors = vec![];
    match (csts[0].clone(), csts[csts.len() - 1].clone()) {
        (CST::AngleBracketOpen(_), CST::AngleBracketClose(_)) => (),
        (CST::AngleBracketOpen(left), CST::CurlyBracketClose(right))
        | (CST::AngleBracketOpen(left), CST::SquareBracketClose(right)) => errors.push(
            TransformError::MismatchedBracket(merge_location(&left.location, &right.location)),
        ),
        (CST::AngleBracketOpen(left), right) => errors.push(TransformError::AngleNotClosed(
            merge_location(&left.location, &right.location()),
        )),
        _ => panic!(
            "Maybe cst parser is broken because CST::Angle must have bracket-open node first."
        ),
    }
    errors
}

pub fn simplify(cst: &CST) -> (CST, Vec<TransformError>) {
    let node = match cst {
        CST::Angle(node) => node,
        _ => panic!("Cannot pass non-angle-bracket node to angle::simplify"),
    };
    let mut errors = vec![];
    let mut csts = vec![];
    for child in node.children.clone() {
        let (cst, mut node_errors) = simplify::simplify(&child);
        let nodes = match cst {
            CST::Expr(node) => node.children.clone(),
            node => vec![node],
        };
        csts.append(&mut nodes.clone());
        errors.append(&mut node_errors);
    }

    errors.append(&mut check_if_the_first_and_last_node_are_brackets(&csts));
    errors.append(&mut check_if_module_or_angle_bracket(&csts));
    errors.append(&mut check_if_dot(&csts));
    errors.append(&mut check_if_ident_or_angle_bracket(&csts));
    errors.append(&mut check_unexpected_dot(&csts));
    let csts = remove_whitespaces_and_newlines(&csts);
    let (csts, mut new_errors) = check_valid_arguments(&csts);
    errors.append(&mut new_errors);
    let csts = remove_elements_not_included_ast(&csts);

    (
        CST::Angle(InnerNode {
            id: node.id.clone(),
            children: csts,
            location: node.location.clone(),
        }),
        errors,
    )
}
