use brack_common::tokens::Token;

use crate::{bracket_close, expr, parser::Parser};

// expr | bracket_close
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    if let Some((cst, new_tokens)) = expr::parse(tokens) {
        return Some((cst, new_tokens));
    } else if let Some((cst, new_tokens)) = bracket_close::parse(tokens) {
        return Some((cst, new_tokens));
    }
    None
}
