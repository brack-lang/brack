use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_backslash, parser::Parser};

// backslash = '\\'
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::BackSlash(location) => {
                return Ok((new_backslash(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected backslash, found {:?}", token),
        }
    }
    bail!("Expected backslash, found none");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{matches_kind, new_backslash};

    #[test]
    fn test_backslash_parse_only_backslash() -> Result<()> {
        let tokens = vec![Token::BackSlash(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert!(matches_kind(&cst, &new_backslash(mock_location())));
        assert_eq!(tokens.len(), 0);
        Ok(())
    }

    #[test]
    fn test_backslash_parse_backslash_and_rest() -> Result<()> {
        let tokens = vec![Token::BackSlash(mock_location()), Token::SquareBracketOpen(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert!(matches_kind(&cst, &new_backslash(mock_location())));
        assert_eq!(tokens, vec![Token::SquareBracketOpen(mock_location())]);
        Ok(())
    }

    #[test]
    fn test_backslash_parse_failure() {
        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_err());
    }
}
