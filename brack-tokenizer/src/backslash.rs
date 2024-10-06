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
    let (_, tail) = separate(&s);
    let mut tokens = t
        .tokens
        .clone()
        .ok_or_else(|| anyhow::anyhow!("`t.tokens` is not set"))?;
    let line = t
        .line
        .ok_or_else(|| anyhow::anyhow!("`t.line` is not set"))?;
    let column = t
        .column
        .ok_or_else(|| anyhow::anyhow!("`t.column` is not set"))?;

    tokens.push(Token::BackSlash(Location {
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
        tokens: Some(tokens),
        ..Default::default()
    };
    dispatch(&t.merge(&t2))
}
