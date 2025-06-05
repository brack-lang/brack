use crate::parser::Parser;
use brack_common::cst::{
    new_angle_bracket_close, new_curly_bracket_close, new_square_bracket_close,
};
use brack_common::tokens::Token;

// angle_bracket_close | square_bracket_close | curly_bracket_close
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::AngleBracketClose(location) => {
                return Some((new_angle_bracket_close(location.clone()), &tokens[1..]));
            }
            Token::SquareBracketClose(location) => {
                return Some((new_square_bracket_close(location.clone()), &tokens[1..]));
            }
            Token::CurlyBracketClose(location) => {
                return Some((new_curly_bracket_close(location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use brack_common::cst::{
        matches_kind, new_angle_bracket_close, new_curly_bracket_close, new_square_bracket_close,
    };
    use brack_common::location::mock_location;
    use brack_common::tokens::Token;

    #[test]
    fn test_bracket_close_parse_only_angle_bracket_close() {
        let tokens = vec![Token::AngleBracketClose(mock_location())];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(
                &cst,
                &new_angle_bracket_close(mock_location())
            ));
        } else {
            panic!("Expected to parse an angle bracket close token");
        }
    }

    #[test]
    fn test_bracket_close_parse_only_square_bracket_close() {
        let tokens = vec![Token::SquareBracketClose(mock_location())];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(
                &cst,
                &new_square_bracket_close(mock_location())
            ));
        } else {
            panic!("Expected to parse a square bracket close token");
        }
    }

    #[test]
    fn test_bracket_close_parse_only_curly_bracket_close() {
        let tokens = vec![Token::CurlyBracketClose(mock_location())];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(
                &cst,
                &new_curly_bracket_close(mock_location())
            ));
        } else {
            panic!("Expected to parse a curly bracket close token");
        }
    }

    #[test]
    fn test_bracket_close_parse_failures() {
        let tokens = vec![];
        assert!(super::parse(&tokens).is_none());

        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        assert!(super::parse(&tokens).is_none());

        let tokens = vec![Token::SquareBracketOpen(mock_location())];
        assert!(super::parse(&tokens).is_none());

        let tokens = vec![Token::CurlyBracketOpen(mock_location())];
        assert!(super::parse(&tokens).is_none());
    }
}
