use anyhow::anyhow;
use anyhow::Result;
use brack_tokenizer::tokens::Token;

use crate::comma;
use crate::dot;
use crate::text;
use crate::{backslash, bracket_close, bracket_open, cst::CST};

// backslash (dot | comma | bracket_open | bracket_close | backslash | .)
pub fn parse<'a>(tokens: &'a [Token]) -> Result<(Vec<CST>, &'a [Token])> {
    let mut result = vec![];
    let (cst, mut tokens) = backslash::parse(tokens)?;
    result.push(cst);
    if let Ok((cst, new_tokens)) = dot::parse(tokens) {
        result.push(cst);
        tokens = new_tokens;
    } else if let Ok((cst, new_tokens)) = comma::parse(tokens) {
        result.push(cst);
        tokens = new_tokens;
    } else if let Ok((cst, new_tokens)) = bracket_open::parse(tokens) {
        result.push(cst);
        tokens = new_tokens;
    } else if let Ok((cst, new_tokens)) = bracket_close::parse(tokens) {
        result.push(cst);
        tokens = new_tokens;
    } else if let Ok((cst, new_tokens)) = text::parse(tokens) {
        result.push(cst);
        tokens = new_tokens;
    } else {
        return Err(anyhow!(
            "Expected dot, comma, bracket open, bracket close, backslash, or text, found none"
        ));
    }
    Ok((result, tokens))
}
