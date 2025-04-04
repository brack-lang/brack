use brack_common::tokens::Token;
use brack_common::cst::new_text;

use crate::parser::Parser;

// text
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Text(text, location) => {
                return Some((new_text(text.clone(), location.clone()), &tokens[1..]));
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
    use brack_common::cst::{matches_kind, new_text};

    #[test]
    fn test_text_parse_only_text() {
        let tokens = vec![Token::Text("text".to_string(), mock_location())];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(
                &cst,
                &new_text("text".to_string(), mock_location())
            ));
        } else {
            panic!("Expected to parse a text token");
        }
    }

    #[test]
    fn test_text_parse_failure() {
        let tokens = vec![Token::Dot(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_none());
    }
}
