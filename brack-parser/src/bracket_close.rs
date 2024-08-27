use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{
    cst::{new_angle_bracket_close, new_curly_bracket_close, new_square_bracket_close},
    parser::Parser,
};

// angle_bracket_close | square_bracket_close | curly_bracket_close
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::AngleBracketClose(location) => {
                return Ok((new_angle_bracket_close(location.clone()), &tokens[1..]));
            }
            Token::SquareBracketClose(location) => {
                return Ok((new_square_bracket_close(location.clone()), &tokens[1..]));
            }
            Token::CurlyBracketClose(location) => {
                return Ok((new_curly_bracket_close(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected bracket close token, found {:?}", token),
        }
    }
    bail!("Expected bracket close token, found none");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{matches_kind, new_angle_bracket_close, new_curly_bracket_close, new_square_bracket_close};

    #[test]
    fn test_bracket_close_parse_only_angle_bracket_close() -> Result<()> {
        let tokens = vec![Token::AngleBracketClose(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_angle_bracket_close(mock_location())));
        Ok(())
    }

    #[test]
    fn test_bracket_close_parse_only_square_bracket_close() -> Result<()> {
        let tokens = vec![Token::SquareBracketClose(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_square_bracket_close(mock_location())));
        Ok(())
    }

    #[test]
    fn test_bracket_close_parse_only_curly_bracket_close() -> Result<()> {
        let tokens = vec![Token::CurlyBracketClose(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_curly_bracket_close(mock_location())));
        Ok(())
    }

    #[test]
    fn test_bracket_close_parse_failures() {
        let tokens = vec![];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::SquareBracketOpen(mock_location())];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::CurlyBracketOpen(mock_location())];
        assert!(super::parse(&tokens).is_err());
    }
}
