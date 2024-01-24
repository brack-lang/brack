use crate::{
    dispatch::dispatch,
    tokenizer::Tokenizer,
    tokens::{Location, LocationData, Token},
    utils::separate,
};

pub fn tokenize(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let line = t.line.unwrap_or_default();
    let (_, tail) = separate(&s);
    let pool = t.pool.clone().unwrap_or_default();
    let mut new_tokens = t.tokens.clone().unwrap_or_default();

    if !pool.trim().is_empty() {
        new_tokens.push(Token::Text(
            pool,
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
    }

    new_tokens.push(Token::NewLine(Location {
        start: LocationData {
            line: t.token_start_line.unwrap_or_default(),
            character: t.token_start_column.unwrap_or_default(),
        },
        end: LocationData {
            line: t.line.unwrap_or_default(),
            character: t.column.unwrap_or_default(),
        },
    }));

    let t2 = Tokenizer {
        line: Some(line + 1),
        column: Some(1),
        token_start_line: Some(line + 1),
        token_start_column: Some(1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        ..Default::default()
    };
    dispatch(&t.merge(&t2))
}
