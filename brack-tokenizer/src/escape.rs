use crate::{dispatch::dispatch, tokenizer::Tokenizer, tokens::Token, utils::separate};
use anyhow::Result;

pub fn tokenize(t: &Tokenizer) -> Result<Vec<Token>> {
    let s = t
        .untreated
        .clone()
        .ok_or_else(|| anyhow::anyhow!("`t.untreated` is not set"))?;
    let mut pool = t
        .pool
        .clone()
        .ok_or_else(|| anyhow::anyhow!("`t.pool` is not set"))?;
    let column = t
        .column
        .ok_or_else(|| anyhow::anyhow!("`t.column` is not set"))?;

    let (head, tail) = separate(&s);
    pool.push_str(&head);

    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(pool),
        escaped: Some(false),
        ..Default::default()
    };
    dispatch(&t.merge(&t2))
}
