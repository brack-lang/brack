use crate::{
    angle_bracket_close, angle_bracket_open, backslash, comma, curly_bracket_close,
    curly_bracket_open, dot, escape, identifier, module, newline, square_bracket_close,
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
        tokens.push(Token::EOF(Location {
            start: LocationData {
                line: t.line.unwrap_or_default(),
                character: t.column.unwrap_or_default(),
            },
            end: LocationData {
                line: t.line.unwrap_or_default(),
                character: t.column.unwrap_or_default(),
            },
        }));
        return Ok(tokens);
    }

    if t.escaped.unwrap_or_default() {
        return escape::tokenize(t);
    }

    let (head2, _) = separate(&tail);

    let angle_c = t.angle_nest_count.unwrap_or_default();
    let curly_c = t.curly_nest_count.unwrap_or_default();
    let square_c = t.square_nest_count.unwrap_or_default();
    let look_for_ident = t.looking_for_identifier.unwrap_or_default();
    let nested = (angle_c + curly_c + square_c) > 0;
    match (&head[..], &head2[..]) {
        ("\\", _) => backslash::tokenize(t),
        ("<", _) => angle_bracket_open::tokenize(t),
        (">", _) if angle_c > 0 => angle_bracket_close::tokenize(t),
        ("{", _) => curly_bracket_open::tokenize(t),
        ("}", _) if curly_c > 0 => curly_bracket_close::tokenize(t),
        ("[", _) => square_bracket_open::tokenize(t),
        ("]", _) if square_c > 0 => square_bracket_close::tokenize(t),
        (".", _) if look_for_ident => dot::tokenize(t),
        (",", _) if nested => comma::tokenize(t),
        (" ", _) if nested => whitespace::tokenize(t),
        ("\n", _) => newline::tokenize(t),
        (_, " ") if look_for_ident => identifier::tokenize(t),
        (_, ".") if look_for_ident => module::tokenize(t),
        (_, "<") | (_, "{") | (_, "[") | (_, "\n") | (_, "\0") => text::tokenize(t),
        (_, " ") if nested => text::tokenize(t),
        (_, ",") if nested => text::tokenize(t),
        (_, ">") if angle_c > 0 => text::tokenize(t),
        (_, "]") if square_c > 0 => text::tokenize(t),
        (_, "}") if curly_c > 0 => text::tokenize(t),
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
