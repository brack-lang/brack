use crate::{dispatch::dispatch, tokenizer::Tokenizer, tokens::Token, utils::separate};

pub fn tokenize(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();

    let (_, tail) = separate(&s);
    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        escaped: Some(true),
        ..Default::default()
    };
    dispatch(&t.merge(&t2))
}
