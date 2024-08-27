use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_eof, parser::Parser};

// EOF
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::EOF(location) => {
                return Ok((new_eof(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected eof token, found {:?}", token),
        }
    }
    bail!("Expected eof, found none");
}

