use brack_parser::cst::CST;

use crate::{
    ast::{
        new_angle, new_curly, new_document, new_expr, new_ident, new_invalid, new_module,
        new_square, new_stmt, new_text, AST,
    },
    error::TransformError,
    simplify,
};

pub fn transform(cst: &CST) -> (AST, Vec<TransformError>) {
    let (cst, errors) = simplify::simplify(cst);

    fn aux(cst: &CST) -> AST {
        match cst {
            CST::Document(node) => {
                let mut children = vec![];
                for child in node.children.clone() {
                    children.push(aux(&child));
                }
                new_document(children, node.location.clone())
            }
            CST::Stmt(node) => {
                let mut children = vec![];
                for child in node.children.clone() {
                    children.push(aux(&child));
                }
                new_stmt(children, node.location.clone())
            }
            CST::Expr(node) => {
                let mut children = vec![];
                for child in node.children.clone() {
                    children.push(aux(&child));
                }
                new_expr(children, node.location.clone())
            }
            CST::Angle(node) => {
                let mut children = vec![];
                for child in node.children.clone() {
                    children.push(aux(&child));
                }
                new_angle(children, node.location.clone())
            }
            CST::Curly(node) => {
                let mut children = vec![];
                for child in node.children.clone() {
                    children.push(aux(&child));
                }
                new_curly(children, node.location.clone())
            }
            CST::Square(node) => {
                let mut children = vec![];
                for child in node.children.clone() {
                    children.push(aux(&child));
                }
                new_square(children, node.location.clone())
            }
            CST::Ident(node) => new_ident(node.value.clone(), node.location.clone()),
            CST::Module(node) => new_module(node.value.clone(), node.location.clone()),
            CST::Invalid(node) => new_invalid(node.location.clone()),
            CST::Text(node) => new_text(node.value.clone(), node.location.clone()),
            node => panic!("Cannot pass non-ast node to transform::aux: {:?}", node),
        }
    }

    (aux(&cst), errors)
}

// #[cfg(test)]
// mod tests {
//     use anyhow::Result;
//     use brack_parser::parse::parse;
//     use brack_tokenizer::tokenize::tokenize;

//     #[test]
//     fn test_simplify_square_1() -> Result<()> {
//         let tokens = tokenize("../brack-tokenizer/test/multiple_errors.[]")?;
//         println!("{:?}", tokens);
//         let cst = parse(&tokens)?;
//         println!("{}", cst);
//         let (ast, errors) = super::transform(&cst);
//         println!("{}", ast);

//         Err(anyhow::anyhow!(errors
//             .iter()
//             .map(|e| e.to_string())
//             .collect::<Vec<String>>()
//             .join("\n")))
//     }
// }
