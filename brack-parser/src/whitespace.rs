use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_whitespace, parser::Parser};

// whitespace
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::WhiteSpace(location) => {
                return Ok((new_whitespace(location.clone()), tokens));
            }
            token => bail!("Expected whitespace token, found {:?}", token),
        }
    }
    bail!("Expected whitespace token, found none");
}

