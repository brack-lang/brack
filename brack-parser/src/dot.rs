use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_dot, parser::Parser};

// dot = '.'
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Dot(location) => {
                return Ok((new_dot(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected dot token, found {:?}", token),
        }
    }
    bail!("Expected dot, found none");
}
