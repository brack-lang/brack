use anyhow::Result;
use brack_tokenizer::tokens::Token;

use crate::error::{ParseTerminationError, ParserError};
use crate::{angle, ast::new_text, parser::Parser, square, utils::check_text};

// text | square | angle
pub fn parse(tokens: &Vec<Token>) -> Result<Parser, ParserError> {
    if check_text(&tokens) && tokens.len() > 0 {
        if let Token::Text(t, location) = tokens.first().unwrap() {
            return Ok((
                new_text(t.to_string(), location.clone()),
                tokens[1..].to_vec(),
            ));
        }
        unreachable!()
    }
    match square::parse(tokens) {
        Ok(parser) => return Ok(parser),
        Err(ParserError::DocumentError(e)) => return Err(e.into()),
        _ => {}
    }
    match angle::parse(tokens) {
        Ok(parser) => return Ok(parser),
        Err(ParserError::DocumentError(e)) => return Err(e.into()),
        _ => {
            return Err(ParseTerminationError::ExprComponentNotFound(
                tokens.first().unwrap().get_location(),
            )
            .into())
        }
    }
}
