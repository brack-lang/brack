use anyhow::{bail, Result};
use brack_tokenizer::tokens::{Location, Token};

use crate::{bracket_close, bracket_open, cst::new_bracket, expr, newline, parser::Parser};

// bracket_open (expr | newline)* bracket_close?
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    let mut result = new_bracket();

    let bracket_open_location = if let Some(token) = tokens.first() {
        token.get_location()
    } else {
        bail!("Expected bracket open token, found none");
    };

    let (cst, mut tokens) = bracket_open::parse(tokens)?;
    result.add(cst);

    loop {
        if let Ok((cst, new_tokens)) = expr::parse(tokens) {
            result.add(cst);
            tokens = new_tokens;
        } else if let Ok((cst, new_tokens)) = newline::parse(tokens) {
            result.add(cst);
            tokens = new_tokens;
        } else {
            break;
        }
    }

    let bracket_close_location = if let Some(token) = tokens.first() {
        token.get_location()
    } else {
        bail!("Expected even at worst EOF, found none");
    };

    let tokens = if let Ok((cst, tokens)) = bracket_close::parse(tokens) {
        result.add(cst);
        tokens
    } else {
        tokens
    };

    result.set_location(Location {
        start: bracket_open_location.start,
        end: bracket_close_location.end,
    });

    Ok((result, tokens))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{
        matches_kind, new_angle_bracket_close, new_angle_bracket_open, new_bracket, new_dot,
        new_expr, new_ident, new_module, new_text, new_whitespace,
    };

    #[test]
    fn test_bracket_parse_valid_angle_bracket() -> Result<()> {
        let tokens = vec![
            Token::AngleBracketOpen(mock_location()),
            Token::Module("std".to_string(), mock_location()),
            Token::Dot(mock_location()),
            Token::Ident("*".to_string(), mock_location()),
            Token::WhiteSpace(mock_location()),
            Token::Text("Hello!".to_string(), mock_location()),
            Token::AngleBracketClose(mock_location()),
        ];
        let (cst, tokens) = super::parse(&tokens)?;
        assert!(matches_kind(&cst, &new_bracket()));
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
        Ok(())
    }

    #[test]
    fn test_bracket_parse_invalid_angle_bracket() -> Result<()> {
        let tokens = vec![
            Token::AngleBracketOpen(mock_location()),
            Token::Module("std".to_string(), mock_location()),
            Token::Dot(mock_location()),
            Token::Ident("*".to_string(), mock_location()),
            Token::WhiteSpace(mock_location()),
            Token::Text("Hello!".to_string(), mock_location()),
            Token::EOF(mock_location()),
        ];
        let (cst, tokens) = super::parse(&tokens)?;
        assert!(matches_kind(&cst, &new_bracket()));
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
        assert_eq!(tokens.len(), 1);
        Ok(())
    }
}
