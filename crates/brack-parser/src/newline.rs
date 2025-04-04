use brack_common::tokens::Token;
use brack_common::cst::new_newline;

use crate::parser::Parser;

// newline = '\n'
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::NewLine(location) => {
                return Some((new_newline(location.clone()), &tokens[1..]));
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
    use brack_common::cst::{matches_kind, new_newline};

    #[test]
    fn test_newline_parse_only_newline() {
        let tokens = vec![Token::NewLine(mock_location())];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(&cst, &new_newline(mock_location())));
        } else {
            panic!("Expected to parse a newline token");
        }
    }

    #[test]
    fn test_newline_parse_failure() {
        let tokens = vec![Token::Dot(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_none());
    }
}
