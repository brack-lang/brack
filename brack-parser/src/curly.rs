use anyhow::{bail, Result};
use brack_tokenizer::tokens::{Location, Token};

use crate::{
    cst::{new_curly, new_curly_bracket_close, new_curly_bracket_open},
    expr, newline,
    parser::Parser,
};

// curly_bracket_open (expr | newline)* curly_bracket_close?
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    let mut result = new_curly();

    let bracket_open_location = if let Some(token) = tokens.first() {
        token.get_location()
    } else {
        bail!("Expected curly bracket open token, found none");
    };

    let (cst, mut tokens) = parse_curly_bracket_open(tokens)?;
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

    let tokens = if let Ok((cst, tokens)) = parse_curly_bracket_close(tokens) {
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

// curly_bracket_open
fn parse_curly_bracket_open<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::CurlyBracketOpen(location) => {
                return Ok((new_curly_bracket_open(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected curly bracket open token, found {:?}", token),
        }
    }
    bail!("Expected curly bracket open token, found none");
}

// curly_bracket_close
fn parse_curly_bracket_close<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::CurlyBracketClose(location) => {
                return Ok((new_curly_bracket_close(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected curly bracket close token, found {:?}", token),
        }
    }
    bail!("Expected curly bracket close token, found none");
}
