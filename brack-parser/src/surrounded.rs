use crate::ast::AST;
use anyhow::Result;
use brack_tokenizer::tokens::Token;

use crate::{arguments, error::ParserError, ident};

// ident (expr ("," expr)*)?
pub fn parse(tokens: &Vec<Token>) -> Result<(Vec<AST>, Vec<Token>), ParserError> {
    let mut new_tokens = tokens.clone();
    let mut result = vec![];

    match ident::parse(&new_tokens) {
        Ok((ast, tokens)) => {
            result.push(ast);
            new_tokens = tokens;
        }
        Err(e) => return Err(e),
    }

    if let Ok((asts, tokens)) = arguments::parse(&new_tokens) {
        for ast in asts {
            result.push(ast);
        }
        new_tokens = tokens;
    }

    Ok((result, new_tokens))
}
