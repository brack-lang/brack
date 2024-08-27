use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_backslash, parser::Parser};

// backslash = '\\'
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::BackSlash(location) => {
                return Ok((new_backslash(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected backslash, found {:?}", token),
        }
    }
    bail!("Expected backslash, found none");
}

