use anyhow::Result;

use crate::{
    ast::{
        new_angle, new_curly, new_document, new_expr, new_ident, new_square, new_stmt, new_text,
        AST,
    },
    tokens::{Token, mock_token_data},
};

type Parser = (AST, Vec<Token>);

fn separate(tokens: &Vec<Token>) -> (Token, Vec<Token>) {
    if tokens.len() == 0 {
        return (Token::Empty(mock_token_data()), vec![]);
    }
    if tokens.len() == 1 {
        return (tokens[0].clone(), vec![]);
    }
    (tokens[0].clone(), tokens[1..].to_vec())
}

fn check_text(tokens: &Vec<Token>) -> bool {
    matches!(tokens.first(), Some(Token::Text(_, _)))
}

fn check_ident(tokens: &Vec<Token>) -> bool {
    matches!(tokens.first(), Some(Token::Ident(_, _)))
}

fn check_eof(tokens: &Vec<Token>) -> bool {
    matches!(tokens.first(), Some(Token::EOF(_)))
}

fn consume_by_kind(tokens: &Vec<Token>, kind: Token) -> (bool, Vec<Token>) {
    let (head, tail) = separate(tokens);
    if head == kind {
        return (true, tail);
    }
    (false, tokens.to_vec())
}

// (curly | expr ("\n" expr)*) ("\n"+ | "\n"* EOF)
fn parse_stmt(tokens: &Vec<Token>) -> Result<Parser> {
    let new_tokens = tokens.clone();
    let mut result = new_stmt();

    let mut new_tokens = if let Ok((ast, tokens)) = parse_curly(&new_tokens) {
        result.add(ast)?;
        tokens
    } else if let Ok((asts, tokens)) = parse_expr_seq(&new_tokens) {
        for ast in asts {
            result.add(ast)?;
        }
        tokens
    } else {
        anyhow::bail!("Failed to parse curly or expr_seq.");
    };

    let mut newline_count = 0;
    loop {
        let (consumed, new_tokens_from_newline) = consume_by_kind(&new_tokens, Token::NewLine(mock_token_data()));
        if !consumed {
            break;
        }
        newline_count += 1;
        new_tokens = new_tokens_from_newline;
    }

    if check_eof(&new_tokens) {
        new_tokens = new_tokens[1..].to_vec();
    } else if newline_count == 0 {
        anyhow::bail!("Expected at least one newline after statement.");
    }

    Ok((result, new_tokens))
}

// expr ("\n" expr)*
fn parse_expr_seq(tokens: &Vec<Token>) -> Result<(Vec<AST>, Vec<Token>)> {
    let mut new_tokens = tokens.clone();
    let mut result = vec![];

    match parse_expr(&new_tokens) {
        Ok((ast, tokens)) => {
            new_tokens = tokens;
            result.push(ast);
        }
        Err(e) => return Err(e),
    }

    // ("\n" expr)*
    {
        let mut tokens = new_tokens.clone();
        let mut succeeded_parse_expr = true;
        while tokens.len() > 0 {
            let (consumed, new_tokens_from_newline) = consume_by_kind(&tokens, Token::NewLine(mock_token_data()));
            if !consumed {
                succeeded_parse_expr = false;
                break;
            }
            tokens = new_tokens_from_newline;

            match parse_expr(&tokens) {
                Ok((ast, tokens)) => {
                    new_tokens = tokens;
                    result.push(ast);
                }
                Err(_) => {
                    succeeded_parse_expr = false;
                    break;
                }
            }
        }
        if succeeded_parse_expr {
            new_tokens = tokens;
        }
    }

    Ok((result, new_tokens))
}

// (text | square | angle)+
fn parse_expr(tokens: &Vec<Token>) -> Result<Parser> {
    let mut new_tokens = tokens.clone();
    let mut result = new_expr();

    match parse_expr_component(&new_tokens) {
        Ok((ast, tokens)) => {
            new_tokens = tokens;
            result.add(ast)?;
        }
        Err(e) => return Err(e),
    }

    while let Ok((ast, tokens)) = parse_expr_component(&new_tokens) {
        new_tokens = tokens;
        result.add(ast)?;
    }

    Ok((result, new_tokens))
}

// text | square | angle
fn parse_expr_component(tokens: &Vec<Token>) -> Result<Parser> {
    if check_text(&tokens) && tokens.len() > 0 {
        if let Token::Text(t, _) = tokens.first().unwrap() {
            return Ok((new_text(t.to_string()), tokens[1..].to_vec()));
        }
        unreachable!()
    }
    if let Ok(parser) = parse_angle(tokens) {
        return Ok(parser);
    }
    if let Ok(parser) = parse_square(tokens) {
        return Ok(parser);
    }
    anyhow::bail!("could not parse expr_component");
}

// "<" ident (expr ("," expr)*)? ">"
fn parse_angle(tokens: &Vec<Token>) -> Result<Parser> {
    let (mut consumed, mut new_tokens) = consume_by_kind(&tokens, Token::AngleBracketOpen(mock_token_data()));
    if !consumed {
        anyhow::bail!("Angle Brackets is not opened.")
    }
    let mut result = new_angle();

    match parse_surrounded(&new_tokens) {
        Ok((asts, tokens)) => {
            new_tokens = tokens;
            for ast in asts {
                result.add(ast)?;
            }
        }
        Err(e) => return Err(e),
    }

    (consumed, new_tokens) = consume_by_kind(&new_tokens, Token::AngleBracketClose(mock_token_data()));
    if !consumed {
        anyhow::bail!("Angle Brackets is not closed.")
    }

    Ok((result, new_tokens))
}

// "{" ident (expr ("," expr)*)? "}"
fn parse_curly(tokens: &Vec<Token>) -> Result<Parser> {
    let (mut consumed, mut new_tokens) = consume_by_kind(&tokens, Token::CurlyBracketOpen(mock_token_data()));
    if !consumed {
        anyhow::bail!("Curly Brackets is not opened.")
    }
    let mut result = new_curly();

    match parse_surrounded(&new_tokens) {
        Ok((asts, tokens)) => {
            new_tokens = tokens;
            for ast in asts {
                result.add(ast)?;
            }
        }
        Err(e) => return Err(e),
    }

    (consumed, new_tokens) = consume_by_kind(&new_tokens, Token::CurlyBracketClose(mock_token_data()));
    if !consumed {
        anyhow::bail!("Curly Brackets is not closed.")
    }

    Ok((result, new_tokens))
}

// "[" ident (expr ("," expr)*)? "]"
fn parse_square(tokens: &Vec<Token>) -> Result<Parser> {
    let (mut consumed, mut new_tokens) = consume_by_kind(&tokens, Token::SquareBracketOpen(mock_token_data()));
    if !consumed {
        anyhow::bail!("Square Brackets is not opened.")
    }
    let mut result = new_square();

    match parse_surrounded(&new_tokens) {
        Ok((asts, tokens)) => {
            new_tokens = tokens;
            for ast in asts {
                result.add(ast)?;
            }
        }
        Err(e) => return Err(e),
    }

    (consumed, new_tokens) = consume_by_kind(&new_tokens, Token::SquareBracketClose(mock_token_data()));
    if !consumed {
        anyhow::bail!("Square Brackets is not closed.")
    }

    Ok((result, new_tokens))
}

// ident (expr ("," expr)*)?
fn parse_surrounded(tokens: &Vec<Token>) -> Result<(Vec<AST>, Vec<Token>)> {
    let mut new_tokens = tokens.clone();
    let mut result = vec![];

    if check_ident(&new_tokens) && new_tokens.len() > 0 {
        if let Token::Ident(i, _) = new_tokens.first().unwrap() {
            result.push(new_ident(i.to_string()));
            new_tokens = (new_tokens.clone())[1..].to_vec()
        }
    } else {
        anyhow::bail!("Could not parse ident.");
    }

    if let Ok((asts, tokens)) = parse_arguments(&new_tokens) {
        for ast in asts {
            result.push(ast);
        }
        new_tokens = tokens;
    }

    Ok((result, new_tokens))
}

// expr ("," expr)*
fn parse_arguments(tokens: &Vec<Token>) -> Result<(Vec<AST>, Vec<Token>)> {
    let mut new_tokens = tokens.clone();
    let mut result = vec![];

    match parse_expr(&new_tokens) {
        Ok((ast, tokens)) => {
            new_tokens = tokens;
            result.push(ast);
        }
        Err(e) => return Err(e),
    }

    // ("," expr)*
    while new_tokens.len() > 0 {
        let (consumed, new_tokens_from_comma) = consume_by_kind(&new_tokens, Token::Comma(mock_token_data()));
        if !consumed {
            break;
        }
        new_tokens = new_tokens_from_comma;

        match parse_expr(&new_tokens) {
            Ok((ast, tokens)) => {
                new_tokens = tokens;
                result.push(ast);
            }
            Err(e) => return Err(e),
        }
    }

    Ok((result, new_tokens))
}

pub fn parse(tokens: &Vec<Token>) -> Result<AST> {
    let mut new_tokens = tokens.clone();
    let mut result = new_document();

    while new_tokens.len() > 0 {
        match parse_stmt(&new_tokens) {
            Ok((ast, tokens)) => {
                new_tokens = tokens;
                result.add(ast)?;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::{
        ast::{
            assert_ast_eq, new_angle_with_children, new_curly_with_children,
            new_document_with_children, new_expr_with_children, new_ident,
            new_square_with_children, new_stmt_with_children, new_text,
        },
        tokens::{Token, mock_token_data},
    };

    use super::parse;

    #[test]
    fn test_parse_no_commands() -> Result<()> {
        let tokens = vec![Token::Text("Hello, World!".to_string(), mock_token_data()), Token::EOF(mock_token_data())];
        let parsed = parse(&tokens)?;
        let expected =
            new_document_with_children(vec![new_stmt_with_children(vec![new_expr_with_children(
                vec![new_text("Hello, World!".to_string())],
            )])]);
        assert_ast_eq(&parsed, &expected);
        Ok(())
    }

    #[test]
    fn test_parse_commands_with_an_argument_includes_square_brackets() -> Result<()> {
        let tokens = vec![
            Token::Text("Hello, ".to_string(), mock_token_data()),
            Token::SquareBracketOpen(mock_token_data()),
            Token::Ident("*".to_string(), mock_token_data()),
            Token::Text("World!".to_string(), mock_token_data()),
            Token::SquareBracketClose(mock_token_data()),
            Token::EOF(mock_token_data()),
        ];
        let parsed = parse(&tokens)?;
        let expected =
            new_document_with_children(vec![new_stmt_with_children(vec![new_expr_with_children(
                vec![
                    new_text("Hello, ".to_string()),
                    new_square_with_children(vec![
                        new_ident("*".to_string()),
                        new_expr_with_children(vec![new_text("World!".to_string())]),
                    ]),
                ],
            )])]);
        assert_ast_eq(&parsed, &expected);
        Ok(())
    }

    #[test]
    fn test_parse_commands_with_an_argument_includes_curly_brackets() -> Result<()> {
        let tokens = vec![
            Token::CurlyBracketOpen(mock_token_data()),
            Token::Ident("*".to_string(), mock_token_data()),
            Token::Text("Heading".to_string(), mock_token_data()),
            Token::CurlyBracketClose(mock_token_data()),
            Token::NewLine(mock_token_data()),
            Token::Text("Hello, World!".to_string(), mock_token_data()),
            Token::EOF(mock_token_data()),
        ];
        let parsed = parse(&tokens)?;
        let expected = new_document_with_children(vec![
            new_stmt_with_children(vec![new_curly_with_children(vec![
                new_ident("*".to_string()),
                new_expr_with_children(vec![new_text("Heading".to_string())]),
            ])]),
            new_stmt_with_children(vec![new_expr_with_children(vec![new_text(
                "Hello, World!".to_string(),
            )])]),
        ]);
        assert_ast_eq(&parsed, &expected);
        Ok(())
    }

    #[test]
    fn test_parse_commands_with_an_argument_includes_angle_brackets() -> Result<()> {
        let tokens = vec![
            Token::Text("Hello, ".to_string(), mock_token_data()),
            Token::AngleBracketOpen(mock_token_data()),
            Token::Ident("*".to_string(), mock_token_data()),
            Token::Text("World!".to_string(), mock_token_data()),
            Token::AngleBracketClose(mock_token_data()),
            Token::EOF(mock_token_data()),
        ];
        let parsed = parse(&tokens)?;
        let expected =
            new_document_with_children(vec![new_stmt_with_children(vec![new_expr_with_children(
                vec![
                    new_text("Hello, ".to_string()),
                    new_angle_with_children(vec![
                        new_ident("*".to_string()),
                        new_expr_with_children(vec![new_text("World!".to_string())]),
                    ]),
                ],
            )])]);
        assert_ast_eq(&parsed, &expected);
        Ok(())
    }

    #[test]
    fn test_parse_commands_with_two_arguments_includes_square_brackets() -> Result<()> {
        let tokens = vec![
            Token::Text("Hello, ".to_string(), mock_token_data()),
            Token::SquareBracketOpen(mock_token_data()),
            Token::Ident("@".to_string(), mock_token_data()),
            Token::Text("World!".to_string(), mock_token_data()),
            Token::Comma(mock_token_data()),
            Token::Text("https://example.com/".to_string(), mock_token_data()),
            Token::SquareBracketClose(mock_token_data()),
            Token::EOF(mock_token_data()),
        ];
        let parsed = parse(&tokens)?;
        let expected =
            new_document_with_children(vec![new_stmt_with_children(vec![new_expr_with_children(
                vec![
                    new_text("Hello, ".to_string()),
                    new_square_with_children(vec![
                        new_ident("@".to_string()),
                        new_expr_with_children(vec![new_text("World!".to_string())]),
                        new_expr_with_children(vec![new_text("https://example.com/".to_string())]),
                    ]),
                ],
            )])]);
        assert_ast_eq(&parsed, &expected);
        Ok(())
    }

    #[test]
    fn test_parse_nesting_commands() -> Result<()> {
        let tokens = vec![
            Token::Text("Hello, ".to_string(), mock_token_data()),
            Token::SquareBracketOpen(mock_token_data()),
            Token::Ident("*".to_string(), mock_token_data()),
            Token::SquareBracketOpen(mock_token_data()),
            Token::Ident("@".to_string(), mock_token_data()),
            Token::Text("World!".to_string(), mock_token_data()),
            Token::Comma(mock_token_data()),
            Token::Text("https://example.com/".to_string(), mock_token_data()),
            Token::SquareBracketClose(mock_token_data()),
            Token::SquareBracketClose(mock_token_data()),
            Token::EOF(mock_token_data()),
        ];
        let parsed = parse(&tokens)?;
        let expected =
            new_document_with_children(vec![new_stmt_with_children(vec![new_expr_with_children(
                vec![
                    new_text("Hello, ".to_string()),
                    new_square_with_children(vec![
                        new_ident("*".to_string()),
                        new_expr_with_children(vec![new_square_with_children(vec![
                            new_ident("@".to_string()),
                            new_expr_with_children(vec![new_text("World!".to_string())]),
                            new_expr_with_children(vec![new_text(
                                "https://example.com/".to_string(),
                            )]),
                        ])]),
                    ]),
                ],
            )])]);
        assert_ast_eq(&parsed, &expected);
        Ok(())
    }

    #[test]
    fn test_parse_newlines() -> Result<()> {
        let tokens = vec![
            Token::Text("Hello,".to_string(), mock_token_data()),
            Token::NewLine(mock_token_data()),
            Token::Text("World,".to_string(), mock_token_data()),
            Token::NewLine(mock_token_data()),
            Token::CurlyBracketOpen(mock_token_data()),
            Token::Ident("**".to_string(), mock_token_data()),
            Token::Text("Contact".to_string(), mock_token_data()),
            Token::CurlyBracketClose(mock_token_data()),
            Token::NewLine(mock_token_data()),
            Token::SquareBracketOpen(mock_token_data()),
            Token::Ident("@".to_string(), mock_token_data()),
            Token::Text("My website".to_string(), mock_token_data()),
            Token::Comma(mock_token_data()),
            Token::Text("https://example.com/".to_string(), mock_token_data()),
            Token::SquareBracketClose(mock_token_data()),
            Token::NewLine(mock_token_data()),
            Token::NewLine(mock_token_data()),
            Token::Text("2023.12.28".to_string(), mock_token_data()),
            Token::NewLine(mock_token_data()),
            Token::EOF(mock_token_data()),
        ];
        let parsed = parse(&tokens)?;
        let expected = new_document_with_children(vec![
            new_stmt_with_children(vec![
                new_expr_with_children(vec![new_text("Hello,".to_string())]),
                new_expr_with_children(vec![new_text("World,".to_string())]),
            ]),
            new_stmt_with_children(vec![new_curly_with_children(vec![
                new_ident("**".to_string()),
                new_expr_with_children(vec![new_text("Contact".to_string())]),
            ])]),
            new_stmt_with_children(vec![new_expr_with_children(vec![
                new_square_with_children(vec![
                    new_ident("@".to_string()),
                    new_expr_with_children(vec![new_text("My website".to_string())]),
                    new_expr_with_children(vec![new_text("https://example.com/".to_string())]),
                ]),
            ])]),
            new_stmt_with_children(vec![new_expr_with_children(vec![new_text(
                "2023.12.28".to_string(),
            )])]),
        ]);
        assert_ast_eq(&parsed, &expected);
        Ok(())
    }
}
