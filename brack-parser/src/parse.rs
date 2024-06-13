use anyhow::Result;
use brack_sdk_rs::ast::AST;
use brack_tokenizer::tokens::Token;

use crate::{ast::new_document, error::ParserError, stmt};

pub fn parse(tokens: &Vec<Token>) -> Result<AST, ParserError> {
    let mut new_tokens = tokens.clone();
    let mut result = new_document();

    while new_tokens.len() > 0 {
        match stmt::parse(&new_tokens) {
            Ok((ast, tokens)) => {
                result.add(ast).map_err(|e| {
                    ParserError::new_document_error(e.to_string(), new_tokens[0].clone())
                })?;
                new_tokens = tokens;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::ast::{
        assert_ast_eq, new_angle_with_children, new_curly_with_children,
        new_document_with_children, new_expr_with_children, new_ident, new_square_with_children,
        new_stmt_with_children, new_text,
    };

    use super::parse;

    #[test]
    fn test_parse_no_commands() -> Result<()> {
        let tokens = vec![
            Token::Text("Hello, World!".to_string(), mock_location()),
            Token::EOF(mock_location()),
        ];
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
            Token::Text("Hello, ".to_string(), mock_location()),
            Token::SquareBracketOpen(mock_location()),
            Token::Module("std".to_string(), mock_location()),
            Token::Dot(mock_location()),
            Token::Ident("*".to_string(), mock_location()),
            Token::Text("World!".to_string(), mock_location()),
            Token::SquareBracketClose(mock_location()),
            Token::EOF(mock_location()),
        ];
        let parsed = parse(&tokens)?;
        let expected =
            new_document_with_children(vec![new_stmt_with_children(vec![new_expr_with_children(
                vec![
                    new_text("Hello, ".to_string()),
                    new_square_with_children(vec![
                        new_ident(vec![new_text("std".to_string()), new_text("*".to_string())]),
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
            Token::CurlyBracketOpen(mock_location()),
            Token::Module("std".to_string(), mock_location()),
            Token::Dot(mock_location()),
            Token::Ident("*".to_string(), mock_location()),
            Token::Text("Heading".to_string(), mock_location()),
            Token::CurlyBracketClose(mock_location()),
            Token::NewLine(mock_location()),
            Token::Text("Hello, World!".to_string(), mock_location()),
            Token::EOF(mock_location()),
        ];
        let parsed = parse(&tokens)?;
        let expected = new_document_with_children(vec![
            new_stmt_with_children(vec![new_curly_with_children(vec![
                new_ident(vec![new_text("std".to_string()), new_text("*".to_string())]),
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
            Token::Text("Hello, ".to_string(), mock_location()),
            Token::AngleBracketOpen(mock_location()),
            Token::Module("std".to_string(), mock_location()),
            Token::Dot(mock_location()),
            Token::Ident("*".to_string(), mock_location()),
            Token::Text("World!".to_string(), mock_location()),
            Token::AngleBracketClose(mock_location()),
            Token::EOF(mock_location()),
        ];
        let parsed = parse(&tokens)?;
        let expected =
            new_document_with_children(vec![new_stmt_with_children(vec![new_expr_with_children(
                vec![
                    new_text("Hello, ".to_string()),
                    new_angle_with_children(vec![
                        new_ident(vec![new_text("std".to_string()), new_text("*".to_string())]),
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
            Token::Text("Hello, ".to_string(), mock_location()),
            Token::SquareBracketOpen(mock_location()),
            Token::Module("std".to_string(), mock_location()),
            Token::Dot(mock_location()),
            Token::Ident("@".to_string(), mock_location()),
            Token::Text("World!".to_string(), mock_location()),
            Token::Comma(mock_location()),
            Token::Text("https://example.com/".to_string(), mock_location()),
            Token::SquareBracketClose(mock_location()),
            Token::EOF(mock_location()),
        ];
        let parsed = parse(&tokens)?;
        let expected =
            new_document_with_children(vec![new_stmt_with_children(vec![new_expr_with_children(
                vec![
                    new_text("Hello, ".to_string()),
                    new_square_with_children(vec![
                        new_ident(vec![new_text("std".to_string()), new_text("@".to_string())]),
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
            Token::Text("Hello, ".to_string(), mock_location()),
            Token::SquareBracketOpen(mock_location()),
            Token::Module("std".to_string(), mock_location()),
            Token::Dot(mock_location()),
            Token::Ident("*".to_string(), mock_location()),
            Token::SquareBracketOpen(mock_location()),
            Token::Module("std".to_string(), mock_location()),
            Token::Dot(mock_location()),
            Token::Ident("@".to_string(), mock_location()),
            Token::Text("World!".to_string(), mock_location()),
            Token::Comma(mock_location()),
            Token::Text("https://example.com/".to_string(), mock_location()),
            Token::SquareBracketClose(mock_location()),
            Token::SquareBracketClose(mock_location()),
            Token::EOF(mock_location()),
        ];
        let parsed = parse(&tokens)?;
        let expected =
            new_document_with_children(vec![new_stmt_with_children(vec![new_expr_with_children(
                vec![
                    new_text("Hello, ".to_string()),
                    new_square_with_children(vec![
                        new_ident(vec![new_text("std".to_string()), new_text("*".to_string())]),
                        new_expr_with_children(vec![new_square_with_children(vec![
                            new_ident(vec![new_text("std".to_string()), new_text("@".to_string())]),
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
            Token::Text("Hello,".to_string(), mock_location()),
            Token::NewLine(mock_location()),
            Token::Text("World,".to_string(), mock_location()),
            Token::NewLine(mock_location()),
            Token::CurlyBracketOpen(mock_location()),
            Token::Module("std".to_string(), mock_location()),
            Token::Dot(mock_location()),
            Token::Ident("**".to_string(), mock_location()),
            Token::Text("Contact".to_string(), mock_location()),
            Token::CurlyBracketClose(mock_location()),
            Token::NewLine(mock_location()),
            Token::SquareBracketOpen(mock_location()),
            Token::Module("std".to_string(), mock_location()),
            Token::Dot(mock_location()),
            Token::Ident("@".to_string(), mock_location()),
            Token::Text("My website".to_string(), mock_location()),
            Token::Comma(mock_location()),
            Token::Text("https://example.com/".to_string(), mock_location()),
            Token::SquareBracketClose(mock_location()),
            Token::NewLine(mock_location()),
            Token::NewLine(mock_location()),
            Token::Text("2023.12.28".to_string(), mock_location()),
            Token::NewLine(mock_location()),
            Token::EOF(mock_location()),
        ];
        let parsed = parse(&tokens)?;
        let expected = new_document_with_children(vec![
            new_stmt_with_children(vec![
                new_expr_with_children(vec![new_text("Hello,".to_string())]),
                new_expr_with_children(vec![new_text("World,".to_string())]),
            ]),
            new_stmt_with_children(vec![new_curly_with_children(vec![
                new_ident(vec![
                    new_text("std".to_string()),
                    new_text("**".to_string()),
                ]),
                new_expr_with_children(vec![new_text("Contact".to_string())]),
            ])]),
            new_stmt_with_children(vec![new_expr_with_children(vec![
                new_square_with_children(vec![
                    new_ident(vec![new_text("std".to_string()), new_text("@".to_string())]),
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
