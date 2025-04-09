use brack_common::cst::new_eof;
use brack_common::tokens::Token;

use crate::parser::Parser;

// EOF
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::EOF(location) => {
                return Some((new_eof(location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use brack_common::cst::{matches_kind, new_eof};
    use brack_common::location::mock_location;
    use brack_common::tokens::Token;

    #[test]
    fn test_eof_parse_only_eof() {
        let tokens = vec![Token::EOF(mock_location())];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(&cst, &new_eof(mock_location())));
        } else {
            panic!("Expected to parse an EOF token");
        }
    }

    #[test]
    fn test_eof_parse_failure() {
        let tokens = vec![];
        let result = super::parse(&tokens);
        assert!(result.is_none());

        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_none());
    }
}
