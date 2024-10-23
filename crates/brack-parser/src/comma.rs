use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_comma, parser::Parser};

pub fn parse(tokens: &[Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Comma(location) => {
                return Ok((new_comma(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected comma token, found {:?}", token),
        }
    }
    bail!("Expected comma, found none");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{matches_kind, new_comma};

    #[test]
    fn test_comma_parse_only_comma() -> Result<()> {
        let tokens = vec![Token::Comma(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_comma(mock_location())));
        Ok(())
    }

    #[test]
    fn test_comma_parse_failure() {
        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_err());
    }
}
