use anyhow::bail;
use anyhow::Result;
use brack_tokenizer::tokens::Token;
use uuid::Uuid;

use crate::cst::new_backslash;
use crate::cst::new_text;
use crate::cst::InnerNode;
use crate::cst::CST;
use crate::parser::Parser;

// backslash (dot | comma | bracket_open | bracket_close | backslash | .)
pub fn parse(tokens: &[Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::BackSlash(location) => {
                let mut tokens = &tokens[1..];
                if let Some(token) = tokens.first() {
                    let escaped_node = match token {
                        Token::BackSlash(location) => {
                            tokens = &tokens[1..];
                            Ok(new_text("\\".to_string(), location.clone()))
                        }
                        Token::AngleBracketOpen(location) => {
                            tokens = &tokens[1..];
                            Ok(new_text("<".to_string(), location.clone()))
                        }
                        Token::AngleBracketClose(location) => {
                            tokens = &tokens[1..];
                            Ok(new_text(">".to_string(), location.clone()))
                        }
                        Token::CurlyBracketOpen(location) => {
                            tokens = &tokens[1..];
                            Ok(new_text("{".to_string(), location.clone()))
                        }
                        Token::CurlyBracketClose(location) => {
                            tokens = &tokens[1..];
                            Ok(new_text("}".to_string(), location.clone()))
                        }
                        Token::SquareBracketOpen(location) => {
                            tokens = &tokens[1..];
                            Ok(new_text("[".to_string(), location.clone()))
                        }
                        Token::SquareBracketClose(location) => {
                            tokens = &tokens[1..];
                            Ok(new_text("]".to_string(), location.clone()))
                        }
                        Token::Dot(location) => {
                            tokens = &tokens[1..];
                            Ok(new_text(".".to_string(), location.clone()))
                        }
                        Token::Comma(location) => {
                            tokens = &tokens[1..];
                            Ok(new_text(",".to_string(), location.clone()))
                        }
                        _ => Err(()),
                    };
                    let children = match escaped_node {
                        Ok(node) => vec![node],
                        Err(_) => vec![],
                    };
                    return Ok((
                        CST::BackSlash(InnerNode {
                            id: Uuid::new_v4().to_string(),
                            location: location.clone(),
                            children: children.clone(),
                        }),
                        tokens,
                    ));
                }
                return Ok((new_backslash(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected backslash, found {:?}", token),
        }
    }
    bail!("Expected backslash, found none");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{matches_kind, new_backslash, new_text};

    #[test]
    fn test_escaped_parse_valid_dot() -> Result<()> {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::Dot(mock_location()),
        ];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_backslash(mock_location())));
        assert!(matches_kind(
            &cst.children()[0],
            &new_text(String::from("."), mock_location())
        ));
        Ok(())
    }

    #[test]
    fn test_escaped_parse_valid_comma() -> Result<()> {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::Comma(mock_location()),
        ];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_backslash(mock_location())));
        assert!(matches_kind(
            &cst.children()[0],
            &new_text(String::from(","), mock_location())
        ));
        Ok(())
    }

    #[test]
    fn test_escaped_parse_valid_bracket_open() -> Result<()> {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::AngleBracketOpen(mock_location()),
        ];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_backslash(mock_location())));
        assert!(matches_kind(
            &cst.children()[0],
            &new_text(String::from("<"), mock_location())
        ));
        Ok(())
    }

    #[test]
    fn test_escaped_parse_valid_bracket_close() -> Result<()> {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::AngleBracketClose(mock_location()),
        ];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_backslash(mock_location())));
        assert!(matches_kind(
            &cst.children()[0],
            &new_text(String::from(">"), mock_location())
        ));
        Ok(())
    }

    #[test]
    fn test_escaped_parse_valid_backslash() -> Result<()> {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::BackSlash(mock_location()),
        ];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_backslash(mock_location())));
        assert!(matches_kind(
            &cst.children()[0],
            &new_text(String::from("\\"), mock_location())
        ));
        Ok(())
    }

    #[test]
    fn test_escaped_parse_valid_text() -> Result<()> {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::Text("Hello!".to_string(), mock_location()),
        ];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 1);
        assert!(matches_kind(&cst, &new_backslash(mock_location())));
        assert_eq!(cst.children().len(), 0);
        Ok(())
    }

    #[test]
    fn test_escaped_parse_failures() {
        let tokens = vec![];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::Dot(mock_location())];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::Comma(mock_location())];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::AngleBracketClose(mock_location())];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::Text("Hello!".to_string(), mock_location())];
        assert!(super::parse(&tokens).is_err());
    }
}
