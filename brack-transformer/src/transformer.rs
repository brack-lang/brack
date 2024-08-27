use brack_parser::cst::CST;

use crate::error::ParserError;

pub type Transformer = (CST, Vec<ParserError>);
