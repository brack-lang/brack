use anyhow::Result;
use brack_tokenizer::tokens::Token;

use crate::{parser::Parser, utils::check_text, ast::new_text, angle, square, error::ParserError};

// text | square | angle
pub fn parse(tokens: &Vec<Token>) -> Result<Parser> {
    if check_text(&tokens) && tokens.len() > 0 {
        if let Token::Text(t, _) = tokens.first().unwrap() {
            return Ok((new_text(t.to_string()), tokens[1..].to_vec()));
        }
        unreachable!()
    }
    if let Ok(parser) = angle::parse(tokens) {
        return Ok(parser);
    }
    if let Ok(parser) = square::parse(tokens) {
        return Ok(parser);
    }
    Err(anyhow::anyhow!(ParserError::new(
        "Could not parse expr_component.".to_string(),
        tokens.first().unwrap().clone(),
    )))
}
