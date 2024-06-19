struct Document {
    //何か
}

struct Stmt {
    //何か
}

struct Square {
    cmd: Identifier,
    args: Vec<Expr>,
}

enum AST {
    Document(Document),
    Stmt(Stmt),
    Expr(Expr),
    Text(Text),
}
