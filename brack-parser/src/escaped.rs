use anyhow::anyhow;
use anyhow::Result;
use brack_tokenizer::tokens::Token;

use crate::comma;
use crate::dot;
use crate::text;
use crate::{backslash, bracket_close, bracket_open, cst::CST};

// backslash (dot | comma | bracket_open | bracket_close | backslash | .)
pub fn parse<'a>(tokens: &'a [Token]) -> Result<(Vec<CST>, &'a [Token])> {
    let mut result = vec![];
    let (cst, mut tokens) = backslash::parse(tokens)?;
    result.push(cst);
    if let Ok((cst, new_tokens)) = dot::parse(tokens) {
        result.push(cst);
        tokens = new_tokens;
    } else if let Ok((cst, new_tokens)) = comma::parse(tokens) {
        result.push(cst);
        tokens = new_tokens;
    } else if let Ok((cst, new_tokens)) = bracket_open::parse(tokens) {
        result.push(cst);
        tokens = new_tokens;
    } else if let Ok((cst, new_tokens)) = bracket_close::parse(tokens) {
        result.push(cst);
        tokens = new_tokens;
    } else if let Ok((cst, new_tokens)) = text::parse(tokens) {
        result.push(cst);
        tokens = new_tokens;
    } else if let Ok((cst, new_tokens)) = backslash::parse(tokens) {
        result.push(cst);
        tokens = new_tokens;
    } else {
        return Err(anyhow!(
            "Expected dot, comma, bracket open, bracket close, backslash, or text, found none"
        ));
    }
    Ok((result, tokens))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use brack_tokenizer::tokens::{mock_location, Token};

    use crate::cst::{
        matches_kind, new_angle_bracket_close, new_angle_bracket_open, new_backslash, new_comma,
        new_dot, new_text,
    };

    #[test]
    fn test_escaped_parse_valid_dot() -> Result<()> {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::Dot(mock_location()),
        ];
        let (csts, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&csts[0], &new_backslash(mock_location())));
        assert!(matches_kind(&csts[1], &new_dot(mock_location())));
        Ok(())
    }

    #[test]
    fn test_escaped_parse_valid_comma() -> Result<()> {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::Comma(mock_location()),
        ];
        let (csts, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&csts[0], &new_backslash(mock_location())));
        assert!(matches_kind(&csts[1], &new_comma(mock_location())));
        Ok(())
    }

    #[test]
    fn test_escaped_parse_valid_bracket_open() -> Result<()> {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::AngleBracketOpen(mock_location()),
        ];
        let (csts, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&csts[0], &new_backslash(mock_location())));
        assert!(matches_kind(
            &csts[1],
            &new_angle_bracket_open(mock_location())
        ));
        Ok(())
    }

    #[test]
    fn test_escaped_parse_valid_bracket_close() -> Result<()> {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::AngleBracketClose(mock_location()),
        ];
        let (csts, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&csts[0], &new_backslash(mock_location())));
        assert!(matches_kind(
            &csts[1],
            &new_angle_bracket_close(mock_location())
        ));
        Ok(())
    }

    #[test]
    fn test_escaped_parse_valid_backslash() -> Result<()> {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::BackSlash(mock_location()),
        ];
        let (csts, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&csts[0], &new_backslash(mock_location())));
        assert!(matches_kind(&csts[1], &new_backslash(mock_location())));
        Ok(())
    }

    #[test]
    fn test_escaped_parse_valid_text() -> Result<()> {
        let tokens = vec![
            Token::BackSlash(mock_location()),
            Token::Text("Hello!".to_string(), mock_location()),
        ];
        let (csts, tokens) = super::parse(&tokens)?;
        assert_eq!(tokens.len(), 0);
        assert!(matches_kind(&csts[0], &new_backslash(mock_location())));
        assert!(matches_kind(
            &csts[1],
            &new_text("Hello!".to_string(), mock_location())
        ));
        Ok(())
    }

    #[test]
    fn test_escaped_parse_failures() {
        let tokens = vec![];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::Dot(mock_location())];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::Comma(mock_location())];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::AngleBracketOpen(mock_location())];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::AngleBracketClose(mock_location())];
        assert!(super::parse(&tokens).is_err());

        let tokens = vec![Token::Text("Hello!".to_string(), mock_location())];
        assert!(super::parse(&tokens).is_err());
    }
}
