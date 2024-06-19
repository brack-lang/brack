use anyhow::Result;
use brack_tokenizer::tokens::{mock_location, Token};

use crate::error::{DocumentError, ParseTerminationError, ParserError};
use crate::{
    ast::{granteed_safe_add, new_ident, new_text},
    parser::Parser,
    utils::consume_by_kind,
};

// text "." text
pub fn parse(tokens: &Vec<Token>) -> Result<Parser, ParserError> {
    let mut result = new_ident(vec![]);

    let new_tokens = if let Token::Module(i, _) = tokens
        .first()
        .ok_or_else(|| ParseTerminationError::TokenNotFound(mock_location()))?
    {
        granteed_safe_add(&mut result, new_text(i.to_string()));
        tokens[1..].to_vec()
    } else {
        return Err(
            ParseTerminationError::MokuleNotFound(tokens.first().unwrap().get_location()).into(),
        );
    };

    let (consumed, new_tokens_from_dot) = consume_by_kind(&new_tokens, Token::Dot(mock_location()));
    if !consumed {
        return Err(
            ParseTerminationError::DotNotFound(new_tokens.first().unwrap().get_location()).into(),
        );
    }
    let new_tokens = new_tokens_from_dot;

    let new_tokens = if let Token::Ident(i, _) = new_tokens
        .first()
        .ok_or_else(|| DocumentError::IdentifierNotFound(mock_location()))?
    {
        granteed_safe_add(&mut result, new_text(i.to_string()));
        (new_tokens.clone())[1..].to_vec()
    } else {
        return Err(
            DocumentError::IdentifierNotFound(new_tokens.first().unwrap().get_location()).into(),
        );
    };

    Ok((result, new_tokens))
}
