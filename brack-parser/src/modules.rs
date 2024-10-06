use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_module, parser::Parser};

// text
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Module(text, location) => {
                return Ok((new_module(text.clone(), location.clone()), &tokens[1..]));
            }
            token => bail!("Expected module token, found {:?}", token),
        }
    }
    bail!("Expected module token, found none");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{matches_kind, new_module};

    #[test]
    fn test_module_parse_only_module() -> Result<()> {
        let tokens = vec![Token::Module("module".to_string(), mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(
            &cst,
            &new_module("module".to_string(), mock_location())
        ));
        Ok(())
    }

    #[test]
    fn test_module_parse_failure() -> Result<()> {
        let tokens = vec![Token::Dot(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_err());
        Ok(())
    }
}
