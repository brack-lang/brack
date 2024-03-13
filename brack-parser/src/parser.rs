use brack_sdk_rs::ast::AST;
use brack_tokenizer::tokens::Token;

pub type Parser = (AST, Vec<Token>);
