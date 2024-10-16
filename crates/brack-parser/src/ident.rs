use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_ident, parser::Parser};

// ident
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Ident(text, location) => {
                return Ok((new_ident(text.clone(), location.clone()), &tokens[1..]));
            }
            token => bail!("Expected ident token, found {:?}", token),
        }
    }
    bail!("Expected ident token, found none");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{matches_kind, new_ident};

    #[test]
    fn test_ident_parse_only_ident() -> Result<()> {
        let tokens = vec![Token::Ident("foo".to_string(), mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(
            &cst,
            &new_ident("foo".to_string(), mock_location())
        ));
        Ok(())
    }

    #[test]
    fn test_ident_parse_failure() {
        let tokens = vec![Token::Dot(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_err());
    }
}
