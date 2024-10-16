use anyhow::Result;
use brack_tokenizer::tokens::Token;

use crate::{
    cst::{new_document, CST},
    eof, newline, stmt,
};

// (stmt newline newline+)* stmt? newline* EOF
pub fn parse<'a>(tokens: &'a [Token]) -> Result<CST> {
    let mut tokens = tokens;
    let mut cst = new_document();

    loop {
        let mut csts = vec![];
        let mut tokens1 = tokens;

        if let Ok((cst1, new_tokens)) = stmt::parse(tokens1) {
            tokens1 = new_tokens;
            csts.push(cst1);
        } else {
            break;
        }

        if let Ok((cst2, new_tokens)) = newline::parse(tokens1) {
            tokens1 = new_tokens;
            csts.push(cst2);
        } else {
            break;
        }

        loop {
            if let Ok((cst3, new_tokens)) = newline::parse(tokens1) {
                tokens1 = new_tokens;
                csts.push(cst3);
            } else {
                break;
            }
        }

        tokens = tokens1;
        for cst1 in csts {
            cst.add(cst1);
        }
    }

    if let Ok((cst1, new_tokens)) = stmt::parse(tokens) {
        cst.add(cst1);
        tokens = new_tokens;
    }

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
