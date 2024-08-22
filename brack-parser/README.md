# brack-parser
It generates a concrete syntax tree (CST) from tokens of the Brack language.
The CST has nodes that are not related to semantic analysis and code generation, for example whitespace (` `), newline (`\n`), period (`.`), and comma (`,`), and so on.
These nodes are useful for implementing formatters, especially snippet expanders.
You can refer to `./brack.cst.ebnf` for concrete syntax.

## Example
```rs
let tokens = tokenize(code)?;
let cst = parse(tokens)?;
```

## CST
CST is defined below.

```rs
pub enum CST {
    Document(InnerNode),
    Stmt(InnerNode),
    Expr(InnerNode),
    Bracket(InnerNode),
    AngleBracketOpen(LeafNode),
    AngleBracketClose(LeafNode),
    SquareBracketOpen(LeafNode),
    SquareBracketClose(LeafNode),
    CurlyBracketOpen(LeafNode),
    CurlyBracketClose(LeafNode),
    Module(LeafNode),
    Ident(LeafNode),
    Text(LeafNode),
    Whitespace(LeafNode),
    Newline(LeafNode),
    Dot(LeafNode),
    Comma(LeafNode),
    EOF(LeafNode),
}
```
