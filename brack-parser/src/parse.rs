use anyhow::Result;
use brack_tokenizer::tokens::Token;

use crate::{
    cst::{new_document, CST},
    eof, newline, stmt,
};

// (stmt newline newline+)* stmt newline* EOF
pub fn parse<'a>(tokens: &'a [Token]) -> Result<CST> {
    let mut tokens = tokens;
    let mut cst = new_document();

    loop {
        if let Ok((cst1, new_tokens)) = stmt::parse(tokens) {
            cst.add(cst1);
            tokens = new_tokens;
        } else {
            break;
        }

        if let Ok((cst2, new_tokens)) = newline::parse(tokens) {
            cst.add(cst2);
            tokens = new_tokens;
        } else {
            break;
        }

        loop {
            if let Ok((cst3, new_tokens)) = newline::parse(tokens) {
                cst.add(cst3);
                tokens = new_tokens;
            } else {
                break;
            }
        }
    }

    let (cst1, mut tokens) = stmt::parse(tokens)?;
    cst.add(cst1);

    loop {
        if let Ok((cst1, new_tokens)) = newline::parse(tokens) {
            cst.add(cst1);
            tokens = new_tokens;
        } else {
            break;
        }
    }

    let (cst1, _) = eof::parse(tokens)?;
    cst.add(cst1);

    Ok(cst)
}
