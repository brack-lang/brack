use anyhow::Result;
use brack_tokenizer::tokens::Token;

use crate::{ast::AST, ident, error::ParserError, arguments};

// ident (expr ("," expr)*)?
pub fn parse(tokens: &Vec<Token>) -> Result<(Vec<AST>, Vec<Token>)> {
    let mut new_tokens = tokens.clone();
    let mut result = vec![];

    match ident::parse(&new_tokens) {
        Ok((ast, tokens)) => {
            result.push(ast);
            new_tokens = tokens;
        }
        Err(e) => return Err(e),
    }

    if let Token::CurlyBracketOpen(_) = new_tokens.first().unwrap() {
        return Err(anyhow::anyhow!(ParserError::new(
            "Curly Brackets is not allowed in Square Brackets or Angle Brackets.".to_string(),
            new_tokens.first().unwrap().clone(),
        )));
    }

    if let Ok((asts, tokens)) = arguments::parse(&new_tokens) {
        for ast in asts {
            result.push(ast);
        }
        new_tokens = tokens;
    }

    Ok((result, new_tokens))
}
