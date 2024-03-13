use anyhow::Result;
use brack_sdk_rs::ast::AST;
use brack_tokenizer::tokens::{mock_location, Token};

use crate::{
    ast::{new_ident, new_text},
    error::ParserError,
    utils::consume_by_kind,
};

// text "." text
pub fn parse(tokens: &Vec<Token>) -> Result<(AST, Vec<Token>)> {
    let mut result = new_ident(vec![]);

    let new_tokens = if let Token::Module(i, _) = tokens.first().ok_or_else(|| {
        anyhow::anyhow!(ParserError::new(
            "Could not parse ident.".to_string(),
            tokens.first().unwrap().clone(),
        ))
    })? {
        result.add(new_text(i.to_string()))?;
        tokens[1..].to_vec()
    } else {
        return Err(anyhow::anyhow!(ParserError::new(
            "Could not parse ident.".to_string(),
            tokens.first().unwrap().clone(),
        )));
    };

    let (consumed, new_tokens_from_dot) = consume_by_kind(&new_tokens, Token::Dot(mock_location()));
    if !consumed {
        return Err(anyhow::anyhow!(ParserError::new(
            "".to_string(),
            new_tokens.first().unwrap().clone(),
        )));
    }
    let new_tokens = new_tokens_from_dot;

    let new_tokens = if let Token::Ident(i, _) = new_tokens.first().ok_or_else(|| {
        anyhow::anyhow!(ParserError::new(
            "Could not parse ident.".to_string(),
            new_tokens.first().unwrap().clone(),
        ))
    })? {
        result.add(new_text(i.to_string()))?;
        (new_tokens.clone())[1..].to_vec()
    } else {
        return Err(anyhow::anyhow!(ParserError::new(
            "Could not parse ident.".to_string(),
            new_tokens.first().unwrap().clone(),
        )));
    };

    Ok((result, new_tokens))
}
