use brack_common::cst::new_whitespace;
use brack_common::tokens::Token;

use crate::parser::Parser;

// whitespace
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::WhiteSpace(location) => {
                return Some((new_whitespace(location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use brack_common::cst::{matches_kind, new_whitespace};
    use brack_common::location::mock_location;
    use brack_common::tokens::Token;

    #[test]
    fn test_whitespace_parse_only_whitespace() {
        let tokens = vec![Token::WhiteSpace(mock_location())];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(&cst, &new_whitespace(mock_location())));
        } else {
            panic!("Expected to parse a whitespace token");
        }
    }

    #[test]
    fn test_whitespace_parse_failure() {
        let tokens = vec![Token::Dot(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_none());
    }
}
