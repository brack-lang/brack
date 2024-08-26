# brack-parser
It generates a concrete syntax tree (CST) from tokens of the Brack language.
The CST has nodes that are not related to semantic analysis and code generation, for example whitespace (` `), newline (`\n`), period (`.`), and comma (`,`), and so on.
These nodes are useful for implementing formatters, especially snippet expanders.

## Syntax
You can also refer to `./brack.cst.ebnf` for concrete syntax.

```ebnf
document := (stmt newline newline+)* stmt newline* EOF
stmt := expr_or_close (newline expr_or_close)*
expr_or_close := expr | bracket_close
escaped := backslash | (dot | comma | bracket_open | bracket_close | backslash | .)
expr := (escaped | module | ident | bracket | dot | comma | whitespace | text)*
bracket := bracket_open (expr | newline)* bracket_close?
bracket_open := angle_bracket_open | square_bracket_open | curly_bracket_open
bracket_close := angle_bracket_close | square_bracket_close | curly_bracket_close
angle_bracket_open := "<"
angle_bracket_close := ">"
square_bracket_open := "["
square_bracket_close := "]"
curly_bracket_open := "{"
curly_bracket_close := "}"
module := text
ident := text
text := [^.]+
whitespace := " "
newline := "\n"
dot = "."
comma := ","
backslash := "\"
```

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

