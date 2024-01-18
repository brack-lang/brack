use anyhow::Result;
use brack_tokenizer::tokens::Token;

use crate::{stmt, ast::{AST, new_document}};

pub fn parse(tokens: &Vec<Token>) -> Result<AST> {
    let mut new_tokens = tokens.clone();
    let mut result = new_document();

    while new_tokens.len() > 0 {
        match stmt::parse(&new_tokens) {
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
    use brack_tokenizer::tokens::{Token, mock_token_data};

    use crate::ast::{new_document_with_children, new_stmt_with_children, new_expr_with_children, new_text, assert_ast_eq, new_square_with_children, new_ident, new_curly_with_children, new_angle_with_children};

    use super::parse;

    #[test]
    fn test_parse_no_commands() -> Result<()> {
        let tokens = vec![
            Token::Text("Hello, World!".to_string(), mock_token_data()),
            Token::EOF(mock_token_data()),
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
            Token::Text("Hello, ".to_string(), mock_token_data()),
            Token::SquareBracketOpen(mock_token_data()),
            Token::Module("std".to_string(), mock_token_data()),
            Token::Dot(mock_token_data()),
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
            Token::CurlyBracketOpen(mock_token_data()),
            Token::Module("std".to_string(), mock_token_data()),
            Token::Dot(mock_token_data()),
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
            Token::Text("Hello, ".to_string(), mock_token_data()),
            Token::AngleBracketOpen(mock_token_data()),
            Token::Module("std".to_string(), mock_token_data()),
            Token::Dot(mock_token_data()),
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
            Token::Text("Hello, ".to_string(), mock_token_data()),
            Token::SquareBracketOpen(mock_token_data()),
            Token::Module("std".to_string(), mock_token_data()),
            Token::Dot(mock_token_data()),
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
            Token::Text("Hello, ".to_string(), mock_token_data()),
            Token::SquareBracketOpen(mock_token_data()),
            Token::Module("std".to_string(), mock_token_data()),
            Token::Dot(mock_token_data()),
            Token::Ident("*".to_string(), mock_token_data()),
            Token::SquareBracketOpen(mock_token_data()),
            Token::Module("std".to_string(), mock_token_data()),
            Token::Dot(mock_token_data()),
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
            Token::Text("Hello,".to_string(), mock_token_data()),
            Token::NewLine(mock_token_data()),
            Token::Text("World,".to_string(), mock_token_data()),
            Token::NewLine(mock_token_data()),
            Token::CurlyBracketOpen(mock_token_data()),
            Token::Module("std".to_string(), mock_token_data()),
            Token::Dot(mock_token_data()),
            Token::Ident("**".to_string(), mock_token_data()),
            Token::Text("Contact".to_string(), mock_token_data()),
            Token::CurlyBracketClose(mock_token_data()),
            Token::NewLine(mock_token_data()),
            Token::SquareBracketOpen(mock_token_data()),
            Token::Module("std".to_string(), mock_token_data()),
            Token::Dot(mock_token_data()),
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
