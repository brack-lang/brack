use brack_parser::cst::CST;

use crate::{angle, backslash, curly, document, error::TransformError, expr, square, stmt};

pub fn simplify(cst: &CST) -> (CST, Vec<TransformError>) {
    match cst {
        CST::Document(_) => document::simplify(&cst),
        CST::Stmt(_) => stmt::simplify(&cst),
        CST::Expr(_) => expr::simplify(&cst),
        CST::Angle(_) => angle::simplify(&cst),
        CST::Curly(_) => curly::simplify(&cst),
        CST::Square(_) => square::simplify(&cst),
        CST::BackSlash(_) => backslash::simplify(&cst),
        node => (node.clone(), vec![]),
    }
}

// #[cfg(test)]
// mod tests {
//     use anyhow::Result;
//     use brack_parser::parse::parse;
//     use brack_tokenizer::tokenize::tokenize;

//     #[test]
//     fn test_simplify_square_1() -> Result<()> {
//         let tokens = tokenize("../brack-tokenizer/test/multiple_errors.[]")?;
//         let cst = parse(&tokens)?;
//         let (_output, errors) = super::simplify(&cst);
//         Err(anyhow::anyhow!(errors.iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n")))
//     }
// }
