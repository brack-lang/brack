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

    tokens.push(Token::AngleBracketClose(Location {
        start: LocationData {
            line: t.line.unwrap_or_default(),
            character: t.column.unwrap_or_default(),
        },
        end: LocationData {
            line: t.line.unwrap_or_default(),
            character: t.column.unwrap_or_default() + 1,
        },
    }));

    let column = t.column.unwrap_or_default();
    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(tokens),
        angle_nest_count: Some(t.angle_nest_count.unwrap_or_default() - 1),
        ..Default::default()
    };
    dispatch(&t.merge(&t2))
}
