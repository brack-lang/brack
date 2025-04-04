use brack_common::cst::CST;
use brack_common::tokens::Token;

pub type Parser<'a> = (CST, &'a [Token]);
