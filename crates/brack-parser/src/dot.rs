use brack_common::tokens::Token;
use brack_common::cst::new_dot;

use crate::parser::Parser;

// dot = '.'
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Dot(location) => {
                return Some((new_dot(location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use brack_common::tokens::Token;
    use brack_common::location::mock_location;
    use brack_common::cst::{matches_kind, new_dot};

    #[test]
    fn test_dot_parse_only_dot() {
        let tokens = vec![Token::Dot(mock_location())];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(&cst, &new_dot(mock_location())));
        } else {
            panic!("Expected to parse a dot token");
        }
    }

    #[test]
    fn test_dot_parse_failures() {
        let tokens = vec![];
        assert!(super::parse(&tokens).is_none());

        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        assert!(super::parse(&tokens).is_none());
    }
}
