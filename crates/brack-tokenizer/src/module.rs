use crate::{
    dispatch::dispatch,
    tokenizer::Tokenizer,
    tokens::{Location, LocationData, Token},
    utils::separate,
};
use anyhow::Result;

pub fn tokenize(t: &Tokenizer) -> Result<Vec<Token>> {
    let s = t
        .untreated
        .clone()
        .ok_or_else(|| anyhow::anyhow!("`t.untreated` is not set"))?;
    let (head, tail) = separate(&s);
    let mut pool = t
        .pool
        .clone()
        .ok_or_else(|| anyhow::anyhow!("`t.pool` is not set"))?;
    pool.push_str(&head);

    let mut tokens = t
        .tokens
        .clone()
        .ok_or_else(|| anyhow::anyhow!("`t.tokens` is not set"))?;
    let token_start_line = t
        .token_start_line
        .ok_or_else(|| anyhow::anyhow!("`t.token_start_line` is not set"))?;
    let token_start_column = t
        .token_start_column
        .ok_or_else(|| anyhow::anyhow!("`t.token_start_column` is not set"))?;
    let line = t
        .line
        .ok_or_else(|| anyhow::anyhow!("`t.line` is not set"))?;
    let column = t
        .column
        .ok_or_else(|| anyhow::anyhow!("`t.column` is not set"))?;
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
