use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{
    cst::{new_angle_bracket_open, new_curly_bracket_open, new_square_bracket_open},
    parser::Parser,
};

// angle_bracket_open | square_bracket_open | curly_bracket_open
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::AngleBracketOpen(location) => {
                return Ok((new_angle_bracket_open(location.clone()), tokens));
            }
            Token::SquareBracketOpen(location) => {
                return Ok((new_square_bracket_open(location.clone()), tokens));
            }
            Token::CurlyBracketOpen(location) => {
                return Ok((new_curly_bracket_open(location.clone()), tokens));
            }
            token => bail!("Expected bracket open token, found {:?}", token),
        }
    }
    bail!("Expected bracket open token, found none");
}
