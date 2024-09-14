use crate::{
    angle_bracket_close, angle_bracket_open, backslash, comma, curly_bracket_close,
    curly_bracket_open, dot, identifier, module, newline, square_bracket_close,
    square_bracket_open, text,
    tokenizer::Tokenizer,
    tokens::{Location, LocationData, Token},
    utils::separate,
    whitespace,
};
use anyhow::Result;

pub fn dispatch(t: &Tokenizer) -> Result<Vec<Token>> {
    let s = t
        .untreated
        .clone()
        .ok_or_else(|| anyhow::anyhow!("`t.untreated` is not set"))?;
    let pool = t
        .pool
        .clone()
        .ok_or_else(|| anyhow::anyhow!("`t.pool` is not set"))?;
    let column = t
        .column
        .ok_or_else(|| anyhow::anyhow!("`t.column` is not set"))?;

    let (head, tail) = separate(&s);

    if head == "\0" {
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
        tokens.push(Token::EOF(Location {
            start: LocationData {
                line,
                character: column,
            },
            end: LocationData {
                line,
                character: column,
            },
        }));
        return Ok(tokens);
    }

    let (head2, _) = separate(&tail);

    let angle_c = t
        .angle_nest_count
        .ok_or_else(|| anyhow::anyhow!("`t.angle_nest_count` is not set"))?;
    let curly_c = t
        .curly_nest_count
        .ok_or_else(|| anyhow::anyhow!("`t.curly_nest_count` is not set"))?;
    let square_c = t
        .square_nest_count
        .ok_or_else(|| anyhow::anyhow!("`t.square_nest_count` is not set"))?;
    let look_for_ident = t
        .looking_for_identifier
        .ok_or_else(|| anyhow::anyhow!("`t.looking_for_identifier` is not set"))?;
    let nested = (angle_c + curly_c + square_c) > 0;
    match (&head[..], &head2[..]) {
        ("\\", _) => backslash::tokenize(t),
        (_, ">") if look_for_ident => identifier::tokenize(t),
        (_, "}") if look_for_ident => identifier::tokenize(t),
        (_, "]") if look_for_ident => identifier::tokenize(t),
        ("<", _) => angle_bracket_open::tokenize(t),
        (">", _) => angle_bracket_close::tokenize(t),
        ("{", _) => curly_bracket_open::tokenize(t),
        ("}", _) => curly_bracket_close::tokenize(t),
        ("[", _) => square_bracket_open::tokenize(t),
        ("]", _) => square_bracket_close::tokenize(t),
        (".", _) if nested => dot::tokenize(t),
        (",", _) if nested => comma::tokenize(t),
        (" ", _) if nested => whitespace::tokenize(t),
        ("\n", _) => newline::tokenize(t),
        (_, " ") if look_for_ident => identifier::tokenize(t),
        (_, ".") if look_for_ident => module::tokenize(t),
        (_, "<") | (_, "{") | (_, "[") | (_, "\n") | (_, "\0") => text::tokenize(t),
        (_, " ") if nested => text::tokenize(t),
        (_, ",") if nested => text::tokenize(t),
        (_, ".") if nested => text::tokenize(t),
        (_, ">") => text::tokenize(t),
        (_, "]") => text::tokenize(t),
        (_, "}") => text::tokenize(t),
        _ => {
            let t2 = Tokenizer {
                column: Some(column + 1),
                untreated: Some(tail),
                pool: Some(format!("{}{}", pool, head)),
                ..Default::default()
            };
            dispatch(&t.merge(&t2))
        }
    }
}
