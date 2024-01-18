use anyhow::Result;
use brack_tokenizer::tokens::{Token, mock_token_data};

use crate::{parser::Parser, ast::new_stmt, utils::{consume_by_kind, check_eof}, error::ParserError, curly, expr_seq};

// (curly | expr ("\n" expr)*) ("\n"+ | "\n"* EOF)
pub fn parse(tokens: &Vec<Token>) -> Result<Parser> {
    let new_tokens = tokens.clone();
    let mut result = new_stmt();

    let mut new_tokens = match curly::parse(&new_tokens) {
        Ok((ast, tokens)) => {
            result.add(ast)?;
            tokens
        }
        Err(curry_err) => match expr_seq::parse(&new_tokens) {
            Ok((asts, tokens)) => {
                for ast in asts {
                    result.add(ast)?;
                }
                tokens
            }
            Err(expr_seq_err) => {
                if let Token::CurlyBracketOpen(_) = new_tokens.first().unwrap() {
                    return Err(curry_err);
                }
                return Err(expr_seq_err);
            }
        },
    };

    let mut newline_count = 0;
    loop {
        let (consumed, new_tokens_from_newline) =
            consume_by_kind(&new_tokens, Token::NewLine(mock_token_data()));
        if !consumed {
            break;
        }
        newline_count += 1;
        new_tokens = new_tokens_from_newline;
    }

    if check_eof(&new_tokens) {
        new_tokens = new_tokens[1..].to_vec();
    } else if newline_count == 0 {
        return Err(anyhow::anyhow!(ParserError::new(
            "Expected at least one newline after statement.".to_string(),
            new_tokens.first().unwrap().clone(),
        )));
    }

    Ok((result, new_tokens))
}
