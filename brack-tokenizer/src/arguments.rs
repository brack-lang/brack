use crate::{
    dispatch::dispatch,
    tokenizer::Tokenizer,
    tokens::{Location, LocationData, Token},
    utils::{separate, take_text_token_from_pool},
};

pub fn tokenize(t: &Tokenizer) -> Vec<Token> {
    let mut t = t.clone();

    let s = t.untreated.clone().unwrap_or_default();
    let (_, tail) = separate(&s);
    
    let mut tokens = t.tokens.clone().unwrap_or_default();
    let took_text_token = take_text_token_from_pool(&t, true);
    if let Some((t2, text_token)) = took_text_token {
        tokens.push(text_token);
        t = t2;
    }

    tokens.push(Token::Comma(Location {
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
        ..Default::default()
    };
    dispatch(&t.merge(&t2))
}
