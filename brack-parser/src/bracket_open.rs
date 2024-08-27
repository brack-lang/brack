use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{
    cst::{new_angle_bracket_open, new_curly_bracket_open, new_square_bracket_open},
    parser::Parser,
};

// angle_bracket_open | square_bracket_open | curly_bracket_open
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::AngleBracketOpen(location) => {
                return Ok((new_angle_bracket_open(location.clone()), &tokens[1..]));
            }
            Token::SquareBracketOpen(location) => {
                return Ok((new_square_bracket_open(location.clone()), &tokens[1..]));
            }
            Token::CurlyBracketOpen(location) => {
                return Ok((new_curly_bracket_open(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected bracket open token, found {:?}", token),
        }
    }
    bail!("Expected bracket open token, found none");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{
        matches_kind, new_angle_bracket_open, new_curly_bracket_open, new_square_bracket_open,
    };

    #[test]
    fn test_bracket_open_parse_only_angle_bracket_open() -> Result<()> {
        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_angle_bracket_open(mock_location())));
        Ok(())
    }

    #[test]
    fn test_bracket_open_parse_only_square_bracket_open() -> Result<()> {
        let tokens = vec![Token::SquareBracketOpen(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(
            &cst,
            &new_square_bracket_open(mock_location())
        ));
        Ok(())
    }

    #[test]
    fn test_bracket_open_parse_only_curly_bracket_open() -> Result<()> {
        let tokens = vec![Token::CurlyBracketOpen(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_curly_bracket_open(mock_location())));
        Ok(())
    }

    #[test]
    fn test_bracket_open_parse_failure() {
        let tokens = vec![];
        let result = super::parse(&tokens);
        assert!(result.is_err());

        let tokens = vec![Token::AngleBracketClose(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_err());

        let tokens = vec![Token::SquareBracketClose(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_err());

        let tokens = vec![Token::CurlyBracketClose(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_err());
    }
}
