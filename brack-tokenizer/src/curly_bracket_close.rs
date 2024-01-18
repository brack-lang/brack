use crate::{
    dispatch::dispatch,
    tokenizer::Tokenizer,
    tokens::{Token, TokenData},
    utils::{separate, update_tokens},
};

pub fn tokenize(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();
    let (_, tail) = separate(&s);

    let mut new_tokens = update_tokens(t, true);
    new_tokens.push(Token::CurlyBracketClose(TokenData {
        line: t.token_start_line.unwrap_or_default(),
        column,
    }));

    let mut t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        curly_nest_count: Some(t.curly_nest_count.unwrap_or_default() - 1),
        ..Default::default()
    };
    if t.looking_for_identifier.unwrap_or_default() {
        t2.looking_for_identifier = Some(false)
    }
    dispatch(&t.merge(&t2))
}
