use crate::{dispatch::dispatch, tokenizer::Tokenizer, utils::separate};
use brack_common::location::{Location, LocationData};
use brack_common::tokens::Token;

pub fn tokenize(t: &Tokenizer) -> Vec<Token> {
    let s = t
        .untreated
        .clone()
        .unwrap_or_else(|| panic!("`Tokenizer.untreated` is not set"));
    let (_, tail) = separate(&s);

    let mut tokens = t
        .tokens
        .clone()
        .unwrap_or_else(|| panic!("`Tokenizer.tokens` is not set"));
    let line = t
        .line
        .unwrap_or_else(|| panic!("`Tokenizer.line` is not set"));
    let column = t
        .column
        .unwrap_or_else(|| panic!("`Tokenizer.column` is not set"));
    let square_nest_count = t
        .square_nest_count
        .unwrap_or_else(|| panic!("`Tokenizer.square_nest_count` is not set"));

    tokens.push(Token::SquareBracketOpen(Location {
        start: LocationData {
            line,
            character: column,
        },
        end: LocationData {
            line,
            character: column + 1,
        },
    }));

    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(tokens),
        square_nest_count: Some(square_nest_count + 1),
        looking_for_identifier: Some(true),
        ..Default::default()
    };
    dispatch(&t.merge(&t2))
}
