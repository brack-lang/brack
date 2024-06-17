use anyhow::Result;
use brack_tokenizer::tokens::{mock_location, Token};

use crate::error::{DocumentError, ParseTerminationError, ParserError};
use crate::{ast::new_square, parser::Parser, surrounded, utils::consume_by_kind};

// "[" ident (expr ("," expr)*)? "]"
pub fn parse(tokens: &Vec<Token>) -> Result<Parser, ParserError> {
    let (mut consumed, mut new_tokens) =
        consume_by_kind(&tokens, Token::SquareBracketOpen(mock_location()));
    if !consumed {
        return Err(ParseTerminationError::SquareNotOpened(
            new_tokens.first().unwrap().get_location(),
        )
        .into());
    }
    let mut result = new_square();

    match surrounded::parse(&new_tokens) {
        Ok((asts, tokens)) => {
            for ast in asts {
                result.add(ast).unwrap();
            }
            new_tokens = tokens;
        }
        Err(e) => return Err(e),
    }

    (consumed, new_tokens) =
        consume_by_kind(&new_tokens, Token::SquareBracketClose(mock_location()));
    if !consumed {
        return Err(
            DocumentError::SquareNotClosed(new_tokens.first().unwrap().get_location()).into(),
        );
    }

    Ok((result, new_tokens))
}
