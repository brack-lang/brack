use crate::{expr_or_close, newline, parser::Parser};
use brack_common::cst::new_stmt;
use brack_common::tokens::Token;

// expr_or_close (newline expr_or_close)*
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    let mut stmt = new_stmt();
    let (cst, mut tokens) = expr_or_close::parse(tokens)?;
    stmt.add(cst);

    loop {
        if let Some((cst1, new_tokens)) = newline::parse(tokens) {
            if let Some((cst2, new_tokens)) = expr_or_close::parse(new_tokens) {
                stmt.add(cst1);
                stmt.add(cst2);
                tokens = new_tokens;
                continue;
            }
        }
        break;
    }

    Some((stmt, tokens))
}
