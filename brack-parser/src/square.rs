use anyhow::Result;
use brack_tokenizer::tokens::{mock_location, Token};

use crate::{
    ast::new_square, error::ParserError, parser::Parser, surrounded, utils::consume_by_kind,
};

// "[" ident (expr ("," expr)*)? "]"
pub fn parse(tokens: &Vec<Token>) -> Result<Parser, ParserError> {
    let (mut consumed, mut new_tokens) =
        consume_by_kind(&tokens, Token::SquareBracketOpen(mock_location()));
    if !consumed {
        return Err(ParserError::new(
            "Square Brackets is not opened.".to_string(),
            tokens.first().unwrap().clone(),
        ));
    }
    let mut result = new_square();

    match surrounded::parse(&new_tokens) {
        Ok((asts, tokens)) => {
            for ast in asts {
                result
                    .add(ast)
                    .map_err(|e| ParserError::new(e.to_string(), tokens[0].clone()))?;
            }
            new_tokens = tokens;
        }
        Err(e) => return Err(e),
    }

    (consumed, new_tokens) =
        consume_by_kind(&new_tokens, Token::SquareBracketClose(mock_location()));
    if !consumed {
        return Err(ParserError::new(
            "Square Brackets is not closed.".to_string(),
            tokens.first().unwrap().clone(),
        ));
    }

    Ok((result, new_tokens))
}
