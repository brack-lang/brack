use brack_common::cst::new_expr;
use brack_common::location::Location;
use brack_common::tokens::Token;

use crate::{bracket, comma, dot, escaped, ident, modules, parser::Parser, text, whitespace};

// (escaped | module | ident | bracket | dot | comma | whitespace | text)+
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    let mut tokens = tokens;
    let mut expr = new_expr();

    loop {
        if let Some((cst, new_tokens)) = escaped::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Some((cst, new_tokens)) = modules::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Some((cst, new_tokens)) = ident::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Some((cst, new_tokens)) = bracket::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Some((cst, new_tokens)) = dot::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Some((cst, new_tokens)) = comma::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Some((cst, new_tokens)) = whitespace::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Some((cst, new_tokens)) = text::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else {
            break;
        }
    }

    if expr.children().is_empty() {
        return None;
    }

    expr.set_location(Location {
        start: expr.children().first().unwrap().location().start,
        end: expr.children().last().unwrap().location().end,
    });
    Some((expr, tokens))
}
