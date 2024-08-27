use anyhow::{bail, Result};
use brack_tokenizer::tokens::{Location, Token};

use crate::{bracket_close, bracket_open, cst::new_bracket, expr, newline, parser::Parser};

// bracket_open (expr | newline)* bracket_close?
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    let mut result = new_bracket();

    let bracket_open_location = if let Some(token) = tokens.first() {
        token.get_location()
    } else {
        bail!("Expected bracket open token, found none");
    };

    let (cst, mut tokens) = bracket_open::parse(tokens)?;
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

    let tokens = if let Ok((cst, tokens)) = bracket_close::parse(tokens) {
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
