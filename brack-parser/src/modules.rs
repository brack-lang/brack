use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_text, parser::Parser};

// text
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Text(text, location) => {
                return Ok((new_text(text.clone(), location.clone()), tokens));
            }
            token => bail!("Expected text token, found {:?}", token),
        }
    }
    bail!("Expected text token, found none");
}

