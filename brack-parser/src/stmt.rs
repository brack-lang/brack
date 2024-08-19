use anyhow::Result;
use brack_tokenizer::tokens::{mock_location, Token};

use crate::error::{DocumentError, ParserError};
use crate::{
    ast::new_stmt,
    curly, expr_seq,
    parser::Parser,
    utils::{check_eof, consume_by_kind},
};

// (curly | expr ("\n" expr)*) ("\n"+ | "\n"* EOF)
pub fn parse(tokens: &Vec<Token>) -> Result<Parser, ParserError> {
    let new_tokens = tokens.clone();
    let mut result = new_stmt();

    let mut new_tokens = match curly::parse(&new_tokens) {
        Ok((ast, tokens)) => {
            result.add(ast);
            tokens
        }
        Err(ParserError::DocumentError(curly_err)) => return Err(curly_err.into()),
        Err(ParserError::ParseTerminationError(_)) => match expr_seq::parse(&new_tokens) {
            Ok((asts, tokens)) => {
                for ast in asts {
                    result.add(ast);
                }
                tokens
            }
            Err(e) => return Err(e),
        },
    };

    let mut newline_count = 0;
    loop {
        let (consumed, new_tokens_from_newline) =
            consume_by_kind(&new_tokens, Token::NewLine(mock_location()));
        if !consumed {
            break;
        }
        newline_count += 1;
        new_tokens = new_tokens_from_newline;
    }

    if check_eof(&new_tokens) {
        new_tokens = new_tokens[1..].to_vec();
    } else if newline_count == 0 {
        return Err(DocumentError::NeedNewLine(new_tokens.first().unwrap().get_location()).into());
    }

    Ok((result, new_tokens))
}
