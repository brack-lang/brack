use crate::{cst::new_stmt, expr_or_close, newline, parser::Parser};
use anyhow::Result;
use brack_tokenizer::tokens::Token;

// expr_or_close (newline expr_or_close)*
pub fn parse(tokens: &[Token]) -> Result<Parser> {
    let mut stmt = new_stmt();
    let (cst, mut tokens) = expr_or_close::parse(tokens)?;
    stmt.add(cst);

    loop {
        if let Ok((cst1, new_tokens)) = newline::parse(tokens) {
            if let Ok((cst2, new_tokens)) = expr_or_close::parse(new_tokens) {
                stmt.add(cst1);
                stmt.add(cst2);
                tokens = new_tokens;
                continue;
            }
        }
        break;
    }

    Ok((stmt, tokens))
}
