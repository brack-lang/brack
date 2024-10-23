use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{bracket_close, expr, parser::Parser};

// expr | bracket_close
pub fn parse(tokens: &[Token]) -> Result<Parser> {
    if let Ok((cst, new_tokens)) = expr::parse(tokens) {
        return Ok((cst, new_tokens));
    } else if let Ok((cst, new_tokens)) = bracket_close::parse(tokens) {
        return Ok((cst, new_tokens));
    }
    bail!("Expected expr or bracket close, found none");
}
