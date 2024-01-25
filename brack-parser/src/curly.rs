use anyhow::Result;
use brack_tokenizer::tokens::{Token, mock_location};

use crate::{parser::Parser, utils::consume_by_kind, error::ParserError, ast::new_curly, surrounded};

// "{" ident (expr ("," expr)*)? "}"
pub fn parse(tokens: &Vec<Token>) -> Result<Parser> {
    let (mut consumed, mut new_tokens) =
        consume_by_kind(&tokens, Token::CurlyBracketOpen(mock_location()));
    if !consumed {
        return Err(anyhow::anyhow!(ParserError::new(
            "Curly Brackets is not opened.".to_string(),
            tokens.first().unwrap().clone(),
        )));
    }
    let mut result = new_curly();

    match surrounded::parse(&new_tokens) {
        Ok((asts, tokens)) => {
            new_tokens = tokens;
            for ast in asts {
                result.add(ast)?;
            }
        }
        Err(e) => return Err(e),
    }

    (consumed, new_tokens) =
        consume_by_kind(&new_tokens, Token::CurlyBracketClose(mock_location()));
    if !consumed {
        return Err(anyhow::anyhow!(ParserError::new(
            "Curly Brackets is not closed.".to_string(),
            new_tokens.first().unwrap().clone(),
        )));
    }

    Ok((result, new_tokens))
}
