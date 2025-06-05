use brack_common::tokens::Token;

use crate::{angle, curly, parser::Parser, square};

// angle | curly | square
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    if let Some((cst, new_tokens)) = angle::parse(tokens) {
        return Some((cst, new_tokens));
    } else if let Some((cst, new_tokens)) = curly::parse(tokens) {
        return Some((cst, new_tokens));
    } else if let Some((cst, new_tokens)) = square::parse(tokens) {
        return Some((cst, new_tokens));
    }
    None
}
