use anyhow::Result;
use brack_tokenizer::tokens::Token;

use crate::{
    ast::granteed_safe_add, ast::new_expr, error::ParserError, expr_component, parser::Parser,
};

// (text | square | angle)+
pub fn parse(tokens: &Vec<Token>) -> Result<Parser, ParserError> {
    let mut new_tokens = tokens.clone();
    let mut result = new_expr();

    match expr_component::parse(&new_tokens) {
        Ok((ast, tokens)) => {
            granteed_safe_add(&mut result, ast);
            new_tokens = tokens;
        }
        Err(e) => return Err(e),
    }

    loop {
        match expr_component::parse(&new_tokens) {
            Ok((ast, tokens)) => {
                granteed_safe_add(&mut result, ast);
                new_tokens = tokens;
            }
            Err(ParserError::DocumentError(e)) => return Err(e.into()),
            _ => break,
        }
    }

    Ok((result, new_tokens))
}
