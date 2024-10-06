use crate::cst::CST;
use brack_tokenizer::tokens::Token;

pub type Parser<'a> = (CST, &'a [Token]);
