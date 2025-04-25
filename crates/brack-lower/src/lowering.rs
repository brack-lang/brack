pub trait Lower {
    type Output;

    fn tycheck_macro() -> AST;
    fn expand() -> AST;
    fn tycheck() -> AST;
}
