use std::env;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let tokens = brack_tokenizer::tokenize::tokenize(&filename).unwrap();
    dbg!(&tokens);
    let ast = brack_parser::parse::parse(&tokens).unwrap();
    dbg!(ast);
}
