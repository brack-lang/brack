use crate::{dispatch::dispatch, tokenizer::Tokenizer, utils::separate};
use brack_common::location::{Location, LocationData};
use brack_common::tokens::Token;

pub fn tokenize(t: &Tokenizer) -> Vec<Token> {
    let s = t
        .untreated
        .clone()
        .unwrap_or_else(|| panic!("`Tokenizer.untreated` is not set"));
    let (head, tail) = separate(&s);
    let mut pool = t
        .pool
        .clone()
        .unwrap_or_else(|| panic!("`Tokenizer.pool` is not set"));
    pool.push_str(&head);

    let mut tokens = t
        .tokens
        .clone()
        .unwrap_or_else(|| panic!("`Tokenizer.tokens` is not set"));
    let token_start_line = t
        .token_start_line
        .unwrap_or_else(|| panic!("`Tokenizer.token_start_line` is not set"));
    let token_start_column = t
        .token_start_column
        .unwrap_or_else(|| panic!("`Tokenizer.token_start_column` is not set"));
    let line = t
        .line
        .unwrap_or_else(|| panic!("`Tokenizer.line` is not set"));
    let column = t
        .column
        .unwrap_or_else(|| panic!("`Tokenizer.column` is not set"));
    tokens.push(Token::Module(
        pool,
        Location {
            start: LocationData {
                line: token_start_line,
                character: token_start_column,
            },
            end: LocationData {
                line,
                character: column + 1,
            },
        },
    ));

    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(tokens),
        ..Default::default()
    };
    dispatch(&t.merge(&t2))
}
