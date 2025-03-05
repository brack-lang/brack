use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_eof, parser::Parser};

// EOF
pub fn parse(tokens: &[Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::EOF(location) => {
                return Ok((new_eof(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected eof token, found {:?}", token),
        }
    }
    bail!("Expected eof, found none");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{matches_kind, new_eof};

    #[test]
    fn test_eof_parse_only_eof() -> Result<()> {
        let tokens = vec![Token::EOF(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_eof(mock_location())));
        Ok(())
    }

    #[test]
    fn test_eof_parse_failure() {
        let tokens = vec![];
        let result = super::parse(&tokens);
        assert!(result.is_err());

        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_err());
    }
}
