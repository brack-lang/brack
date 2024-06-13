use anyhow::Result;
use brack_tokenizer::tokens::{mock_location, Token};

use crate::{
    ast::{new_ident, new_text},
    error::ParserError,
    parser::Parser,
    utils::consume_by_kind,
};

// text "." text
pub fn parse(tokens: &Vec<Token>) -> Result<Parser, ParserError> {
    let mut result = new_ident(vec![]);

    let new_tokens = if let Token::Module(i, _) = tokens.first().ok_or_else(|| {
        ParserError::new_parse_termination_error(
            "Could not parse ident.".to_string(),
            tokens.first().unwrap().clone(),
        )
    })? {
        result.add(new_text(i.to_string())).map_err(|e| {
            ParserError::new_document_error(e.to_string(), tokens.first().unwrap().clone())
        })?;
        tokens[1..].to_vec()
    } else {
        return Err(ParserError::new_parse_termination_error(
            "Could not parse ident.".to_string(),
            tokens.first().unwrap().clone(),
        ));
    };

    let (consumed, new_tokens_from_dot) = consume_by_kind(&new_tokens, Token::Dot(mock_location()));
    if !consumed {
        return Err(ParserError::new_parse_termination_error(
            "Dot is not found.".to_string(),
            new_tokens.first().unwrap().clone(),
        ));
    }
    let new_tokens = new_tokens_from_dot;

    let new_tokens = if let Token::Ident(i, _) = new_tokens.first().ok_or_else(|| {
        ParserError::new_document_error(
            "Could not parse ident.".to_string(),
            new_tokens.first().unwrap().clone(),
        )
    })? {
        result.add(new_text(i.to_string())).map_err(|e| {
            ParserError::new_document_error(e.to_string(), new_tokens.first().unwrap().clone())
        })?;
        (new_tokens.clone())[1..].to_vec()
    } else {
        return Err(ParserError::new_document_error(
            "Could not parse ident.".to_string(),
            new_tokens.first().unwrap().clone(),
        ));
    };

    Ok((result, new_tokens))
}
