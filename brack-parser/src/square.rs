use anyhow::{bail, Result};
use brack_tokenizer::tokens::{Location, Token};

use crate::{
    cst::{new_square, new_square_bracket_close, new_square_bracket_open},
    expr, newline,
    parser::Parser,
};

// square_bracket_open (expr | newline)* square_bracket_close?
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    let mut result = new_square();

    let bracket_open_location = if let Some(token) = tokens.first() {
        token.get_location()
    } else {
        bail!("Expected square bracket open token, found none");
    };

    let (cst, mut tokens) = parse_square_bracket_open(tokens)?;
    result.add(cst);

    loop {
        if let Ok((cst, new_tokens)) = expr::parse(tokens) {
            result.add(cst);
            tokens = new_tokens;
        } else if let Ok((cst, new_tokens)) = newline::parse(tokens) {
            result.add(cst);
            tokens = new_tokens;
        } else {
            break;
        }
    }

    let bracket_close_location = if let Some(token) = tokens.first() {
        token.get_location()
    } else {
        bail!("Expected even at worst EOF, found none");
    };

    let tokens = if let Ok((cst, tokens)) = parse_square_bracket_close(tokens) {
        result.add(cst);
        tokens
    } else {
        tokens
    };

    result.set_location(Location {
        start: bracket_open_location.start,
        end: bracket_close_location.end,
    });

    Ok((result, tokens))
}

// square_bracket_open
fn parse_square_bracket_open<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::SquareBracketOpen(location) => {
                return Ok((new_square_bracket_open(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected square bracket open token, found {:?}", token),
        }
    }
    bail!("Expected square bracket open token, found none");
}

// square_bracket_close
fn parse_square_bracket_close<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::SquareBracketClose(location) => {
                return Ok((new_square_bracket_close(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected square bracket close token, found {:?}", token),
        }
    }
    bail!("Expected square bracket close token, found none");
}
