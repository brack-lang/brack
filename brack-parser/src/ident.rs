use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_ident, parser::Parser};

// ident
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Text(text, location) => {
                return Ok((new_ident(text.clone(), location.clone()), tokens));
            }
            token => bail!("Expected ident token, found {:?}", token),
        }
    }
    bail!("Expected ident token, found none");
}

