use brack_tokenizer::tokens::Token;

use crate::ast::AST;

pub type Parser = (AST, Vec<Token>);
