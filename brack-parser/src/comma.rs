use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_comma, parser::Parser};

pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Comma(location) => {
                return Ok((new_comma(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected comma token, found {:?}", token),
        }
    }
    bail!("Expected comma, found none");
}
