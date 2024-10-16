use anyhow::{bail, Result};
use brack_tokenizer::tokens::Token;

use crate::{cst::new_text, parser::Parser};

// text
pub fn parse<'a>(tokens: &'a [Token]) -> Result<Parser> {
    if let Some(token) = tokens.first() {
        match token {
            Token::Text(text, location) => {
                return Ok((new_text(text.clone(), location.clone()), &tokens[1..]));
            }
            token => bail!("Expected text token, found {:?}", token),
        }
    }
    bail!("Expected text token, found none");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{matches_kind, new_text};

    #[test]
    fn test_text_parse_only_text() -> Result<()> {
        let tokens = vec![Token::Text("text".to_string(), mock_location())];
        let (cst, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(
            &cst,
            &new_text("text".to_string(), mock_location())
        ));
        Ok(())
    }

    #[test]
    fn test_text_parse_failure() {
        let tokens = vec![Token::Dot(mock_location())];
        let result = super::parse(&tokens);
        assert!(result.is_err());
    }
}
