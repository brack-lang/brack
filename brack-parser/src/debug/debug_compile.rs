use std::env;

static FILE_PATH: &str = "sample-text/no_commands.[]";

fn main() {
    let tokens = brack_tokenizer::tokenize::tokenize(&FILE_PATH).unwrap();
    let ast = brack_parser::parse::parse(&tokens).unwrap();
    dbg!(ast);
}
