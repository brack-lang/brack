use crate::{
    dispatch::dispatch,
    tokenizer::Tokenizer,
    tokens::{Location, LocationData, Token},
    utils::separate,
};

pub fn tokenize(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let (_, tail) = separate(&s);

    let mut tokens = t.tokens.clone().unwrap_or_default();
    tokens.push(Token::Ident(
        t.pool.clone().unwrap_or_default(),
        Location {
            start: LocationData {
                line: t.token_start_line.unwrap_or_default(),
                character: t.token_start_column.unwrap_or_default(),
            },
            end: LocationData {
                line: t.line.unwrap_or_default(),
                character: t.column.unwrap_or_default(),
            },
        },
    ));

    let column = t.column.unwrap_or_default();
    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(tokens),
        looking_for_identifier: Some(false),
        ..Default::default()
    };
    dispatch(&t.merge(&t2))
}
