use brack_common::cst::{new_angle, new_angle_bracket_close, new_angle_bracket_open};
use brack_common::location::Location;
use brack_common::tokens::Token;

use crate::{expr, newline, parser::Parser};

// angle_bracket_open (expr | newline)* angle_bracket_close?
pub fn parse(tokens: &[Token]) -> Option<Parser> {
    let mut result = new_angle();

    let bracket_open_location = if let Some(token) = tokens.first() {
        token.get_location()
    } else {
        return None;
    };

    let (cst, mut tokens) = parse_angle_bracket_open(tokens)?;
    result.add(cst);

    loop {
        if let Some((cst, new_tokens)) = expr::parse(tokens) {
            result.add(cst);
            tokens = new_tokens;
        } else if let Some((cst, new_tokens)) = newline::parse(tokens) {
            result.add(cst);
            tokens = new_tokens;
        } else {
            break;
        }
    }

    let bracket_close_location = if let Some(token) = tokens.first() {
        token.get_location()
    } else {
        return None;
    };

    let tokens = if let Some((cst, tokens)) = parse_angle_bracket_close(tokens) {
        result.add(cst);
        tokens
    } else {
        tokens
    };

    result.set_location(Location {
        start: bracket_open_location.start,
        end: bracket_close_location.end,
    });

    Some((result, tokens))
}

// angle_bracket_open
fn parse_angle_bracket_open(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::AngleBracketOpen(location) => {
                return Some((new_angle_bracket_open(location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}

// angle_bracket_close
fn parse_angle_bracket_close(tokens: &[Token]) -> Option<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::AngleBracketClose(location) => {
                return Some((new_angle_bracket_close(location.clone()), &tokens[1..]));
            }
            _ => return None,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use brack_common::cst::{
        matches_kind, new_angle, new_angle_bracket_close, new_angle_bracket_open, new_dot,
        new_expr, new_ident, new_module, new_text, new_whitespace,
    };
    use brack_common::location::mock_location;
    use brack_common::tokens::Token;

    #[test]
    fn test_bracket_parse_valid_angle_bracket() {
        let tokens = vec![
            Token::AngleBracketOpen(mock_location()),
            Token::Module("std".to_string(), mock_location()),
            Token::Dot(mock_location()),
            Token::Ident("*".to_string(), mock_location()),
            Token::WhiteSpace(mock_location()),
            Token::Text("Hello!".to_string(), mock_location()),
            Token::AngleBracketClose(mock_location()),
        ];
        if let Some((cst, tokens)) = super::parse(&tokens) {
            assert!(matches_kind(&cst, &new_angle()));
            assert!(matches_kind(
                &cst.children()[0],
                &new_angle_bracket_open(mock_location())
            ));
            assert!(matches_kind(&cst.children()[1], &new_expr()));
            assert!(matches_kind(
                &cst.children()[1].children()[0],
                &new_module("std".to_string(), mock_location())
            ));
            assert!(matches_kind(
                &cst.children()[1].children()[1],
                &new_dot(mock_location())
            ));
            assert!(matches_kind(
                &cst.children()[1].children()[2],
                &new_ident("*".to_string(), mock_location())
            ));
            assert!(matches_kind(
                &cst.children()[1].children()[3],
                &new_whitespace(mock_location())
            ));
            assert!(matches_kind(
                &cst.children()[1].children()[4],
                &new_text("Hello!".to_string(), mock_location())
            ));
            assert!(matches_kind(
                &cst.children()[2],
                &new_angle_bracket_close(mock_location())
            ));
            assert_eq!(tokens.len(), 0);
        } else {
            panic!("Expected to parse an angle bracket");
        }
    }

    #[test]
    fn test_bracket_parse_invalid_angle_bracket() {
        let tokens = vec![
            Token::AngleBracketOpen(mock_location()),
            Token::Module("std".to_string(), mock_location()),
            Token::Dot(mock_location()),
            Token::Ident("*".to_string(), mock_location()),
            Token::WhiteSpace(mock_location()),
            Token::Text("Hello!".to_string(), mock_location()),
            Token::EOF(mock_location()),
        ];
        if let Some((cst, _)) = super::parse(&tokens) {
            assert!(matches_kind(&cst, &new_angle()));
            assert!(matches_kind(
                &cst.children()[0],
                &new_angle_bracket_open(mock_location())
            ));
            assert!(matches_kind(&cst.children()[1], &new_expr()));
            assert!(matches_kind(
                &cst.children()[1].children()[0],
                &new_module("std".to_string(), mock_location())
            ));
            assert!(matches_kind(
                &cst.children()[1].children()[1],
                &new_dot(mock_location())
            ));
            assert!(matches_kind(
                &cst.children()[1].children()[2],
                &new_ident("*".to_string(), mock_location())
            ));
            assert!(matches_kind(
                &cst.children()[1].children()[3],
                &new_whitespace(mock_location())
            ));
            assert!(matches_kind(
                &cst.children()[1].children()[4],
                &new_text("Hello!".to_string(), mock_location())
            ));
        } else {
            panic!("Expected to parse an angle bracket");
        }
    }
}
