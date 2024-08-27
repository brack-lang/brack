use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{
    cst::{new_angle_bracket_close, new_curly_bracket_close, new_square_bracket_close},
    parser::Parser,
};

// angle_bracket_close | square_bracket_close | curly_bracket_close
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::AngleBracketClose(location) => {
                return Ok((new_angle_bracket_close(location.clone()), &tokens[1..]));
            }
            Token::SquareBracketClose(location) => {
                return Ok((new_square_bracket_close(location.clone()), &tokens[1..]));
            }
            Token::CurlyBracketClose(location) => {
                return Ok((new_curly_bracket_close(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected bracket close token, found {:?}", token),
        }
    }
    bail!("Expected bracket close token, found none");
}
