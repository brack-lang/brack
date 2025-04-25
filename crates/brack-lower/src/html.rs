use std::collections::HashMap;

use brack_common::{
    ast::{InnerNode, LeafNode, AST},
    html::{Html, HtmlTag},
    logger::Logger,
    plugins::Plugins,
};

pub enum LowerError {
    RemainingMacros,
    DocumentNotFound,
    MustNotBeTopLevelNode,
}

pub fn lowering<L: Logger>(
    ast: AST,
    plugins: &mut Plugins<Html>,
    logger: &L,
) -> (Option<Html>, Vec<LowerError>) {
    let (ast, mut errors) = expand(ast, plugins, logger);
    match ast {
        AST::Document(_) => (),
        _ => {
            let err = LowerError::DocumentNotFound;
            errors.push(err);
        }
    }
    let (html, new_errors) = match ast {
        AST::Stmt(stmt) => lowering_stmt(stmt, plugins, logger),
        AST::Expr(expr) => lowering_expr(expr, plugins, logger),
        AST::Angle(_) => {
            let err = LowerError::RemainingMacros;
            (None, vec![err])
        }
        AST::Curly(curly) => lowering_curly(curly, plugins, logger),
        AST::Square(square) => lowering_square(square, plugins, logger),
        AST::Text(text) => lowering_text(text, plugins, logger),
        _ => {
            let err = LowerError::MustNotBeTopLevelNode;
            (None, vec![err])
        }
    };
    errors.extend(new_errors);
    (html, errors)
}

fn expand<L: Logger>(
    ast: AST,
    _plugins: &mut Plugins<Html>,
    _logger: &L,
) -> (AST, Vec<LowerError>) {
    return (ast, vec![]);
}

fn lowering_stmt<L: Logger>(
    stmt: InnerNode,
    plugins: &mut Plugins<Html>,
    logger: &L,
) -> (Option<Html>, Vec<LowerError>) {
    let mut html = Html {
        tag: HtmlTag::Div,
        attributes: HashMap::new(),
        children: vec![],
    };
    let mut errors = vec![];
    for child in stmt.children {
        let (child_html, child_errors) = lowering(child, plugins, logger);
        if let Some(child_html) = child_html {
            html.children.push(child_html);
        }
        errors.extend(child_errors);
    }
    (Some(html), errors)
}

fn lowering_expr<L: Logger>(
    expr: InnerNode,
    plugins: &mut Plugins<Html>,
    logger: &L,
) -> (Option<Html>, Vec<LowerError>) {
    let mut html = Html {
        tag: HtmlTag::Span,
        attributes: HashMap::new(),
        children: vec![],
    };
    let mut errors = vec![];
    for child in expr.children {
        let (child_html, child_errors) = lowering(child, plugins, logger);
        if let Some(child_html) = child_html {
            html.children.push(child_html);
        }
        errors.extend(child_errors);
    }
    (Some(html), errors)
}

fn lowering_curly<L: Logger>(
    curly: InnerNode,
    plugins: &mut Plugins<Html>,
    logger: &L,
) -> (Option<Html>, Vec<LowerError>) {
    lowering(curly.children[0].clone(), plugins, logger)
}

fn lowering_square<L: Logger>(
    square: InnerNode,
    plugins: &mut Plugins<Html>,
    logger: &L,
) -> (Option<Html>, Vec<LowerError>) {
    lowering(square.children[0].clone(), plugins, logger)
}

fn lowering_text<L: Logger>(
    text: LeafNode,
    plugins: &mut Plugins<Html>,
    logger: &L,
) -> (Option<Html>, Vec<LowerError>) {
    lowering(AST::Text(text), plugins, logger)
}
