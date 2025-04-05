use brack_common::cst::new_module;
use brack_common::tokens::Token;

use crate::parser::Parser;

// text
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Module(text, location) => {
                return Some((new_module(text.clone(), location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use brack_common::cst::{matches_kind, new_module};
    use brack_common::location::mock_location;
    use brack_common::tokens::Token;

    #[test]
    fn test_module_parse_only_module() {
        let tokens = vec![Token::Module("module".to_string(), mock_location())];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert_eq!(tokens.len(), 0);
            assert!(matches_kind(
                &cst,
                &new_module("module".to_string(), mock_location())
            ));
        } else {
            panic!("Expected to parse a module token");
        }
    }

    #[test]
    fn test_module_parse_failure() {
        let tokens = vec![Token::Dot(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_none());
    }
}
