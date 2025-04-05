use brack_common::cst::{new_backslash, new_text};
use brack_common::tokens::Token;

use crate::parser::Parser;

// backslash (dot | comma | bracket_open | bracket_close | backslash | .)
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::BackSlash(location) => {
                let mut tokens = &tokens[1..];
                if let Some(token) = tokens.first() {
                    let escaped_node = match token {
                        Token::BackSlash(location) => {
                            tokens = &tokens[1..];
                            Some(new_text("\\".to_string(), location.clone()))
                        }
                        Token::AngleBracketOpen(location) => {
                            tokens = &tokens[1..];
                            Some(new_text("<".to_string(), location.clone()))
                        }
                        Token::AngleBracketClose(location) => {
                            tokens = &tokens[1..];
                            Some(new_text(">".to_string(), location.clone()))
                        }
                        Token::CurlyBracketOpen(location) => {
                            tokens = &tokens[1..];
                            Some(new_text("{".to_string(), location.clone()))
                        }
                        Token::CurlyBracketClose(location) => {
                            tokens = &tokens[1..];
                            Some(new_text("}".to_string(), location.clone()))
                        }
                        Token::SquareBracketOpen(location) => {
                            tokens = &tokens[1..];
                            Some(new_text("[".to_string(), location.clone()))
                        }
                        Token::SquareBracketClose(location) => {
                            tokens = &tokens[1..];
                            Some(new_text("]".to_string(), location.clone()))
                        }
                        Token::Dot(location) => {
                            tokens = &tokens[1..];
                            Some(new_text(".".to_string(), location.clone()))
                        }
                        Token::Comma(location) => {
                            tokens = &tokens[1..];
                            Some(new_text(",".to_string(), location.clone()))
                        }
                        _ => None,
                    };
                    let children = match escaped_node {
                        Some(node) => vec![node],
                        _ => vec![],
                    };
                    let mut inner = new_backslash(location.clone());
                    for child in children {
                        inner.add(child);
                    }
                    return Some((inner, tokens));
                }
                return Some((new_backslash(location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use brack_common::cst::{matches_kind, new_backslash, new_text};
    use brack_common::location::mock_location;
    use brack_common::tokens::Token;

    #[test]
    fn test_escaped_parse_valid_dot() {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::Dot(mock_location()),
        ];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(&cst, &new_backslash(mock_location())));
            assert!(matches_kind(
                &cst.children()[0],
                &new_text(String::from("."), mock_location())
            ));
        } else {
            panic!("Expected to parse a backslash token");
        }
    }

    #[test]
    fn test_escaped_parse_valid_comma() {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::Comma(mock_location()),
        ];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(&cst, &new_backslash(mock_location())));
            assert!(matches_kind(
                &cst.children()[0],
                &new_text(String::from(","), mock_location())
            ));
        } else {
            panic!("Expected to parse a backslash token");
        }
    }

    #[test]
    fn test_escaped_parse_valid_bracket_open() {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::AngleBracketOpen(mock_location()),
        ];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(&cst, &new_backslash(mock_location())));
            assert!(matches_kind(
                &cst.children()[0],
                &new_text(String::from("<"), mock_location())
            ));
        } else {
            panic!("Expected to parse a backslash token");
        }
    }

    #[test]
    fn test_escaped_parse_valid_bracket_close() {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::AngleBracketClose(mock_location()),
        ];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(&cst, &new_backslash(mock_location())));
            assert!(matches_kind(
                &cst.children()[0],
                &new_text(String::from(">"), mock_location())
            ));
        } else {
            panic!("Expected to parse a backslash token");
        }
    }

    #[test]
    fn test_escaped_parse_valid_backslash() {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::BackSlash(mock_location()),
        ];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(&cst, &new_backslash(mock_location())));
            assert!(matches_kind(
                &cst.children()[0],
                &new_text(String::from("\\"), mock_location())
            ));
        } else {
            panic!("Expected to parse a backslash token");
        }
    }

    #[test]
    fn test_escaped_parse_valid_text() {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::Text("Hello!".to_string(), mock_location()),
        ];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 1);
            assert!(matches_kind(&cst, &new_backslash(mock_location())));
            assert_eq!(cst.children().len(), 0);
        } else {
            panic!("Expected to parse a backslash token");
        }
    }

    #[test]
    fn test_escaped_parse_failures() {
        let tokens = vec![];
        assert!(super::parse(&tokens).is_none());

        let tokens = vec![Token::Dot(mock_location())];
        assert!(super::parse(&tokens).is_none());

        let tokens = vec![Token::Comma(mock_location())];
        assert!(super::parse(&tokens).is_none());

        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        assert!(super::parse(&tokens).is_none());

        let tokens = vec![Token::AngleBracketClose(mock_location())];
        assert!(super::parse(&tokens).is_none());

        let tokens = vec![Token::Text("Hello!".to_string(), mock_location())];
        assert!(super::parse(&tokens).is_none());
    }
}
