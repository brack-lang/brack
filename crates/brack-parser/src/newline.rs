use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_newline, parser::Parser};

// newline = '\n'
pub fn parse(tokens: &[Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::NewLine(location) => {
                return Ok((new_newline(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected newline, found {:?}", token),
        }
    }
    bail!("Expected newline, found none");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{matches_kind, new_newline};

    #[test]
    fn test_newline_parse_only_newline() -> Result<()> {
        let tokens = vec![Token::NewLine(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_newline(mock_location())));
        Ok(())
    }

    #[test]
    fn test_newline_parse_failure() {
        let tokens = vec![Token::Dot(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_err());
    }
}
