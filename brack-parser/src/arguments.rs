use anyhow::Result;
use brack_tokenizer::tokens::{Token, mock_token_data};

use crate::{utils::consume_by_kind, ast::AST, expr};

// expr ("," expr)*
pub fn parse(tokens: &Vec<Token>) -> Result<(Vec<AST>, Vec<Token>)> {
    let mut new_tokens = tokens.clone();
    let mut result = vec![];

    match expr::parse(&new_tokens) {
        Ok((ast, tokens)) => {
            new_tokens = tokens;
            result.push(ast);
        }
        Err(e) => return Err(e),
    }

    // ("," expr)*
    while new_tokens.len() > 0 {
        let (consumed, new_tokens_from_comma) =
            consume_by_kind(&new_tokens, Token::Comma(mock_token_data()));
        if !consumed {
            break;
        }
        new_tokens = new_tokens_from_comma;

        match expr::parse(&new_tokens) {
            Ok((ast, tokens)) => {
                new_tokens = tokens;
                result.push(ast);
            }
            Err(e) => return Err(e),
        }
    }

    Ok((result, new_tokens))
}
