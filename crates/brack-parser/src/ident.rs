use brack_common::cst::new_ident;
use brack_common::tokens::Token;

use crate::parser::Parser;

// ident
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Ident(text, location) => {
                return Some((new_ident(text.clone(), location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use brack_common::cst::{matches_kind, new_ident};
    use brack_common::location::mock_location;
    use brack_common::tokens::Token;

    #[test]
    fn test_ident_parse_only_ident() {
        let tokens = vec![Token::Ident("foo".to_string(), mock_location())];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(
                &cst,
                &new_ident("foo".to_string(), mock_location())
            ));
        } else {
            panic!("Expected to parse an identifier");
        }
    }

    #[test]
    fn test_ident_parse_failure() {
        let tokens = vec![Token::Dot(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_none());
    }
}
