use brack_parser::cst::{new_expr, CST};

use crate::error::TransformError;

pub fn check_if_module_or_angle_bracket(csts: &Vec<CST>) -> Vec<TransformError> {
    if csts.len() < 2 {
        return vec![];
    }
    let cst = csts[1].clone();
    match cst {
        CST::Module(_) => vec![],
        CST::Angle(_) => vec![],
        _ => vec![TransformError::ModuleNotFound(cst.location())],
    }
}

pub fn check_if_dot(csts: &Vec<CST>) -> Vec<TransformError> {
    if csts.len() < 3 {
        return vec![];
    }
    let cst = csts[2].clone();
    match cst {
        CST::Dot(_) => vec![],
        _ => vec![TransformError::DotNotFound(cst.location())],
    }
}

pub fn check_if_ident_or_angle_bracket(csts: &Vec<CST>) -> Vec<TransformError> {
    if csts.len() < 4 {
        return vec![];
    }
    let cst = csts[3].clone();
    match cst {
        CST::Ident(_) => vec![],
        CST::Angle(_) => vec![],
        _ => vec![TransformError::IdentifierNotFound(cst.location())],
    }
}

pub fn remove_elements_not_included_ast(csts: &Vec<CST>) -> Vec<CST> {
    let mut new_csts = vec![];
    for cst in csts {
        match cst {
            CST::Whitespace(_)
            | CST::Newline(_)
            | CST::Comma(_)
            | CST::Dot(_)
            | CST::AngleBracketOpen(_)
            | CST::AngleBracketClose(_)
            | CST::CurlyBracketOpen(_)
            | CST::CurlyBracketClose(_)
            | CST::SquareBracketOpen(_)
            | CST::SquareBracketClose(_)
            | CST::EOF(_) => (),
            _ => new_csts.push(cst.clone()),
        }
    }
    new_csts
}

pub fn check_valid_arguments(csts: &Vec<CST>) -> (Vec<CST>, Vec<TransformError>) {
    if csts.len() < 4 {
        return (csts.clone(), vec![]);
    }
    let mut errors = vec![];
    let mut new_csts = csts[0..4].to_vec(); // [AngleBracketOpen, Module, Dot, Ident
    let mut expr = new_expr();
    let mut previous_comma = false;
    for i in 4..csts.len() {
        match csts[i].clone() {
            CST::Comma(_) => {
                if expr.children().is_empty() {
                    errors.push(TransformError::UnexpectedComma(csts[i].location()));
                    continue;
                }
                new_csts.push(expr);
                expr = new_expr();
                previous_comma = true;
            }
            CST::AngleBracketClose(_) | CST::CurlyBracketClose(_) | CST::SquareBracketClose(_) => {
                if !expr.children().is_empty() {
                    new_csts.push(expr.clone());
                } else if previous_comma {
                    errors.push(TransformError::UnexpectedComma(csts[i - 1].location()));
                }
                expr = new_expr();
                new_csts.push(csts[i].clone());
                break;
            }
            _ => {
                expr.add(csts[i].clone());
                previous_comma = false;
            }
        }
    }
    if !expr.children().is_empty() {
        new_csts.push(expr);
    }
    (new_csts, errors)
}

pub fn check_unexpected_dot(csts: &Vec<CST>) -> Vec<TransformError> {
    let mut errors = vec![];
    for i in 3..csts.len() {
        let cst = csts[i].clone();
        match cst {
            CST::Dot(_) => errors.push(TransformError::UnexpectedDot(cst.location())),
            _ => (),
        }
    }
    errors
}

pub fn remove_whitespaces_and_newlines(csts: &Vec<CST>) -> Vec<CST> {
    let mut result = vec![];
    for cst in csts {
        match cst {
            CST::Whitespace(_) | CST::Newline(_) => (),
            _ => result.push(cst.clone()),
        }
    }
    result
}
