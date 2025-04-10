// use std::collections::HashMap;

// use brack_common::{
//     ast::{InnerNode, AST},
//     ir::{Html, HtmlTag},
//     logger::Logger,
// };

// pub enum LowerError {
//     RemainingMacros,
//     DocumentNotFound,
//     MustNotBeTopLevelNode,
// }

// pub fn lowering<L: Logger>(ast: AST, logger: &L) -> (Html, Vec<LowerError>) {
//     let (ast, mut errors) = expand(ast);
//     match ast {
//         AST::Document(_) => (),
//         _ => {
//             let err = LowerError::DocumentNotFound;
//             errors.push(err);
//         }
//     }
//     let (html, new_errors) = match ast {
//         AST::Stmt(stmt) => lowering_stmt(stmt, logger),
//         AST::Expr(expr) => lowering_expr(expr, logger),
//         AST::Angle(angle) => {
//             let err = LowerError::RemainingMacros;
//             errors.push(err);
//         }
//         AST::Curly(curly) => lowering_curly(curly),
//         AST::Square(square) => lowering_square(square),
//         AST::Text(text) => lowering_text(text),
//         _ => {
//             let err = LowerError::MustNotBeTopLevelNode;
//             errors.push(err);
//         }
//     };
//     errors.extend(new_errors);
//     (html, errors)
// }

// fn expand(ast: AST) -> (AST, Vec<LowerError>) {
//     return (ast, vec![]);
// }

// fn lowering_stmt<L: Logger>(stmt: InnerNode, logger: &L) -> (Html, Vec<LowerError>) {
//     let mut html = Html {
//         tag: HtmlTag::Div,
//         attributes: HashMap::new(),
//         children: vec![],
//     };
//     let mut errors = vec![];
//     for child in stmt.children {
//         let (child_html, child_errors) = lowering(child, logger);
//         html.children.push(child_html);
//         errors.extend(child_errors);
//     }
//     (html, errors)
// }

// fn lowering_expr<L: Logger>(expr: InnerNode, logger: &L) -> (Html, Vec<LowerError>) {
//     let mut html = Html {
//         tag: HtmlTag::Span,
//         attributes: HashMap::new(),
//         children: vec![],
//     };
//     let mut errors = vec![];
//     for child in expr.children {
//         let (child_html, child_errors) = lowering(child, logger);
//         html.children.push(child_html);
//         errors.extend(child_errors);
//     }
//     (html, errors)
// }

// fn lowering_curly<L: Logger>(curly: InnerNode, logger: &L) -> (Html, Vec<LowerError>) {}

// fn lowering_square<L: Logger>(square: InnerNode, logger: &L) -> (Html, Vec<LowerError>) {}

// fn lowering_text<L: Logger>(text: InnerNode, logger: &L) -> (Html, Vec<LowerError>) {}
