use brack_common::tokens::Token;
use brack_common::location::Location;
use brack_common::cst::{new_square, new_square_bracket_close, new_square_bracket_open};

use crate::{
    expr, newline,
    parser::Parser,
};

// square_bracket_open (expr | newline)* square_bracket_close?
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    let mut result = new_square();

    let bracket_open_location = if let Some(token) = tokens.first() {
        token.get_location()
    } else {
        return None;
    };

    let (cst, mut tokens) = parse_square_bracket_open(tokens)?;
    result.add(cst);

    loop {
        if let Some((cst, new_tokens)) = expr::parse(tokens) {
            result.add(cst);
            tokens = new_tokens;
        } else if let Some((cst, new_tokens)) = newline::parse(tokens) {
            result.add(cst);
            tokens = new_tokens;
        } else {
            break;
        }
    }

    let bracket_close_location = if let Some(token) = tokens.first() {
        token.get_location()
    } else {
        return None;
    };

    let tokens = if let Some((cst, tokens)) = parse_square_bracket_close(tokens) {
        result.add(cst);
        tokens
    } else {
        tokens
    };

    result.set_location(Location {
        start: bracket_open_location.start,
        end: bracket_close_location.end,
    });

    Some((result, tokens))
}

// square_bracket_open
fn parse_square_bracket_open(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::SquareBracketOpen(location) => {
                return Some((new_square_bracket_open(location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}

// square_bracket_close
fn parse_square_bracket_close(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::SquareBracketClose(location) => {
                return Some((new_square_bracket_close(location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}
