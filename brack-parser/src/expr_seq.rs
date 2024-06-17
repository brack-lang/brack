use anyhow::Result;
use brack_sdk_rs::ast::AST;
use brack_tokenizer::tokens::{mock_location, Token};

use crate::{error::ParserError, expr, utils::consume_by_kind};

// expr ("\n" expr)*
pub fn parse(tokens: &Vec<Token>) -> Result<(Vec<AST>, Vec<Token>), ParserError> {
    let mut new_tokens = tokens.clone();
    let mut result = vec![];

    match expr::parse(&new_tokens) {
        Ok((ast, tokens)) => {
            new_tokens = tokens;
            result.push(ast);
        }
        Err(e) => return Err(e),
    }

    // ("\n" expr)*
    {
        let mut tokens = new_tokens.clone();
        let mut succeeded_parse_expr = true;
        while tokens.len() > 0 {
            let (consumed, new_tokens_from_newline) =
                consume_by_kind(&tokens, Token::NewLine(mock_location()));
            if !consumed {
                succeeded_parse_expr = false;
                break;
            }
            tokens = new_tokens_from_newline;

            match expr::parse(&tokens) {
                Ok((ast, tokens)) => {
                    new_tokens = tokens;
                    result.push(ast);
                }
                Err(ParserError::DocumentError(e)) => return Err(e.into()),
                Err(_) => {
                    succeeded_parse_expr = false;
                    break;
                }
            }
        }
        if succeeded_parse_expr {
            new_tokens = tokens;
        }
    }

    Ok((result, new_tokens))
}
