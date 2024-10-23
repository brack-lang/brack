use anyhow::Result;
use brack_tokenizer::tokens::{Location, Token};

use crate::{
    bracket, comma, cst::new_expr, dot, escaped, ident, modules, parser::Parser, text, whitespace,
};

// (escaped | module | ident | bracket | dot | comma | whitespace | text)+
pub fn parse(tokens: &[Token]) -> Result<Parser> {
    let mut tokens = tokens;
    let mut expr = new_expr();

    loop {
        if let Ok((cst, new_tokens)) = escaped::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Ok((cst, new_tokens)) = modules::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Ok((cst, new_tokens)) = ident::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Ok((cst, new_tokens)) = bracket::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Ok((cst, new_tokens)) = dot::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Ok((cst, new_tokens)) = comma::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Ok((cst, new_tokens)) = whitespace::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else if let Ok((cst, new_tokens)) = text::parse(tokens) {
            expr.add(cst);
            tokens = new_tokens;
        } else {
            break;
        }
    }

    if expr.children().is_empty() {
        return Err(anyhow::anyhow!(
            "Expected escaped, module, ident, bracket, dot, comma, whitespace, or text, found none"
        ));
    }

    expr.set_location(Location {
        start: expr.children().first().unwrap().location().start,
        end: expr.children().last().unwrap().location().end,
    });
    Ok((expr, tokens))
}
