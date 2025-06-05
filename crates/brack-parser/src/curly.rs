use brack_common::cst::{new_curly, new_curly_bracket_close, new_curly_bracket_open};
use brack_common::location::Location;
use brack_common::tokens::Token;

use crate::{expr, newline, parser::Parser};

// curly_bracket_open (expr | newline)* curly_bracket_close?
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    let mut result = new_curly();
    let mut tokens = tokens;

    let bracket_open_location = if let Some(token) = tokens.first() {
        token.get_location()
    } else {
        return None;
    };

    if let Some((cst, new_tokens)) = parse_curly_bracket_open(tokens) {
        result.add(cst);
        tokens = new_tokens;
    } else {
        return None;
    }

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

    let tokens = if let Some((cst, tokens)) = parse_curly_bracket_close(tokens) {
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

// curly_bracket_open
fn parse_curly_bracket_open(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::CurlyBracketOpen(location) => {
                return Some((new_curly_bracket_open(location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}

// curly_bracket_close
fn parse_curly_bracket_close(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::CurlyBracketClose(location) => {
                return Some((new_curly_bracket_close(location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}
