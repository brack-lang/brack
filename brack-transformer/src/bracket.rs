use brack_parser::cst::CST;

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
            | CST::Comma(_)
            | CST::Dot(_)
            | CST::AngleBracketOpen(_)
            | CST::AngleBracketClose(_)
            | CST::CurlyBracketOpen(_)
            | CST::CurlyBracketClose(_)
            | CST::SquareBracketOpen(_)
            | CST::SquareBracketClose(_) => (),
            _ => new_csts.push(cst.clone()),
        }
    }
    new_csts
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
