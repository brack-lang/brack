use brack_common::tokens::Token;
use brack_common::cst::new_comma;

use crate::parser::Parser;

pub fn parse(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Comma(location) => {
                return Some((new_comma(location.clone()), &tokens[1..]));
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
    use brack_common::cst::{matches_kind, new_comma};

    #[test]
    fn test_comma_parse_only_comma() {
        let tokens = vec![Token::Comma(mock_location())];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(&cst, &new_comma(mock_location())));
        } else {
            panic!("Expected to parse a comma token");
        }
    }

    #[test]
    fn test_comma_parse_failure() {
        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_none());
    }
}
