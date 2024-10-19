use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_whitespace, parser::Parser};

// whitespace
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::WhiteSpace(location) => {
                return Ok((new_whitespace(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected whitespace token, found {:?}", token),
        }
    }
    bail!("Expected whitespace token, found none");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{matches_kind, new_whitespace};

    #[test]
    fn test_whitespace_parse_only_whitespace() -> Result<()> {
        let tokens = vec![Token::WhiteSpace(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_whitespace(mock_location())));
        Ok(())
    }

    #[test]
    fn test_whitespace_parse_failure() -> Result<()> {
        let tokens = vec![Token::Dot(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_err());
        Ok(())
    }
}
