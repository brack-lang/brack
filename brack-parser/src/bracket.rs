use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{angle, curly, parser::Parser, square};

// angle | curly | square
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Ok((cst, new_tokens)) = angle::parse(tokens) {
        return Ok((cst, new_tokens));
    } else if let Ok((cst, new_tokens)) = curly::parse(tokens) {
        return Ok((cst, new_tokens));
    } else if let Ok((cst, new_tokens)) = square::parse(tokens) {
        return Ok((cst, new_tokens));
    }
    bail!("Expected angle, curly, or square bracket, found none");
}
