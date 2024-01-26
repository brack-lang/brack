use anyhow::Result;
use brack_tokenizer::tokens::Token;

use crate::{ast::new_expr, expr_component, parser::Parser};

// (text | square | angle)+
pub fn parse(tokens: &Vec<Token>) -> Result<Parser> {
    let mut new_tokens = tokens.clone();
    let mut result = new_expr();

    match expr_component::parse(&new_tokens) {
        Ok((ast, tokens)) => {
            new_tokens = tokens;
            result.add(ast)?;
        }
        Err(e) => return Err(e),
    }

    while let Ok((ast, tokens)) = expr_component::parse(&new_tokens) {
        new_tokens = tokens;
        result.add(ast)?;
    }

    Ok((result, new_tokens))
}
