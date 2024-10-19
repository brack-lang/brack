use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_dot, parser::Parser};

// dot = '.'
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Dot(location) => {
                return Ok((new_dot(location.clone()), &tokens[1..]));
            }
            token => bail!("Expected dot token, found {:?}", token),
        }
    }
    bail!("Expected dot, found none");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{matches_kind, new_dot};

    #[test]
    fn test_dot_parse_only_dot() -> Result<()> {
        let tokens = vec![Token::Dot(mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&cst, &new_dot(mock_location())));
        Ok(())
    }

    #[test]
    fn test_dot_parse_failures() {
        let tokens = vec![];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        assert!(super::parse(&tokens).is_err());
    }
}
