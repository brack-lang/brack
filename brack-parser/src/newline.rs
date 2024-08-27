use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_newline, parser::Parser};

// newline = '\n'
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::NewLine(location) => {
                return Ok((new_newline(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected newline, found {:?}", token),
        }
    }
    bail!("Expected newline, found none");
}
