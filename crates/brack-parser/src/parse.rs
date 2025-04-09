use brack_common::cst::{new_document, CST};
use brack_common::tokens::Token;

use crate::{eof, newline, stmt};

// (stmt newline newline+)* stmt? newline* EOF
pub fn parse(tokens: &[Token]) -> CST {
    let mut tokens = tokens;
    let mut cst = new_document();

    loop {
        let mut csts = vec![];
        let mut tokens1 = tokens;

        if let Some((cst1, new_tokens)) = stmt::parse(tokens1) {
            tokens1 = new_tokens;
            csts.push(cst1);
        } else {
            break;
        }

        if let Some((cst2, new_tokens)) = newline::parse(tokens1) {
            tokens1 = new_tokens;
            csts.push(cst2);
        } else {
            break;
        }

        while let Some((cst3, new_tokens)) = newline::parse(tokens1) {
            tokens1 = new_tokens;
            csts.push(cst3);
        }

        tokens = tokens1;
        for cst1 in csts {
            cst.add(cst1);
        }
    }

    if let Some((cst1, new_tokens)) = stmt::parse(tokens) {
        cst.add(cst1);
        tokens = new_tokens;
    }

    while let Some((cst1, new_tokens)) = newline::parse(tokens) {
        cst.add(cst1);
        tokens = new_tokens;
    }

    if let Some((cst1, _)) = eof::parse(tokens) {
        cst.add(cst1);
    } else {
        panic!("EOF not found");
    }

    cst
}
