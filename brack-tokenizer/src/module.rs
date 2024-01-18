use crate::{
    dispatch::dispatch,
    tokenizer::Tokenizer,
    tokens::{Token, TokenData},
    utils::separate,
};

pub fn tokenize(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();
    let (_, tail) = separate(&s);
    let mut new_tokens = t.tokens.clone().unwrap_or_default();
    new_tokens.push(Token::Module(
        t.pool.clone().unwrap_or_default(),
        TokenData {
            line: t.token_start_line.unwrap_or_default(),
            column: t.token_start_column.unwrap_or_default(),
        },
    ));
    new_tokens.push(Token::Dot(TokenData {
        line: t.token_start_line.unwrap_or_default(),
        column,
    }));
    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        ..Default::default()
    };
    dispatch(&t.merge(&t2))
}
