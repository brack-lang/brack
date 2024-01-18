use crate::tokens::Token;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tokenizer {
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub token_start_line: Option<usize>,
    pub token_start_column: Option<usize>,
    pub untreated: Option<String>,
    pub pool: Option<String>,
    pub tokens: Option<Vec<Token>>,
    pub escaped: Option<bool>,
    pub angle_nest_count: Option<i32>,
    pub square_nest_count: Option<i32>,
    pub curly_nest_count: Option<i32>,
    pub looking_for_identifier: Option<bool>,
}

impl Tokenizer {
    pub fn merge(&self, other: &Tokenizer) -> Tokenizer {
        Tokenizer {
            line: match other.line {
                Some(s) => Some(s),
                None => self.line,
            },
            column: match other.column {
                Some(s) => Some(s),
                None => self.column,
            },
            token_start_line: match other.token_start_line {
                Some(s) => Some(s),
                None => self.token_start_line,
            },
            token_start_column: match other.token_start_column {
                Some(s) => Some(s),
                None => self.token_start_column,
            },
            untreated: match &other.untreated {
                Some(s) => Some(s.clone()),
                None => self.untreated.clone(),
            },
            pool: match &other.pool {
                Some(s) => Some(s.clone()),
                None => self.pool.clone(),
            },
            tokens: match &other.tokens {
                Some(s) => Some(s.clone()),
                None => self.tokens.clone(),
            },
            escaped: match other.escaped {
                Some(s) => Some(s),
                None => self.escaped,
            },
            angle_nest_count: match other.angle_nest_count {
                Some(s) => Some(s),
                None => self.angle_nest_count,
            },
            square_nest_count: match other.square_nest_count {
                Some(s) => Some(s),
                None => self.square_nest_count,
            },
            curly_nest_count: match other.curly_nest_count {
                Some(s) => Some(s),
                None => self.curly_nest_count,
            },
            looking_for_identifier: match other.looking_for_identifier {
                Some(s) => Some(s),
                None => self.looking_for_identifier,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::TokenData;

    #[test]
    fn test_merge() {
        use super::*;
        let a = Tokenizer {
            line: Some(1),
            column: Some(1),
            token_start_line: Some(1),
            token_start_column: Some(1),
            untreated: Some("a".to_string()),
            pool: Some("a".to_string()),
            tokens: Some(vec![Token::Text(
                "a".to_string(),
                TokenData { line: 1, column: 1 },
            )]),
            escaped: Some(true),
            angle_nest_count: Some(1),
            square_nest_count: Some(1),
            curly_nest_count: Some(1),
            looking_for_identifier: Some(true),
        };
        let b = Tokenizer {
            line: Some(2),
            column: Some(2),
            token_start_line: Some(2),
            token_start_column: Some(2),
            untreated: Some("b".to_string()),
            pool: Some("b".to_string()),
            tokens: Some(vec![Token::Text(
                "b".to_string(),
                TokenData { line: 2, column: 2 },
            )]),
            escaped: Some(false),
            angle_nest_count: Some(2),
            square_nest_count: Some(2),
            curly_nest_count: Some(2),
            looking_for_identifier: Some(false),
        };
        let c = a.merge(&b);
        let res = Tokenizer {
            line: Some(2),
            column: Some(2),
            token_start_line: Some(2),
            token_start_column: Some(2),
            untreated: Some("b".to_string()),
            pool: Some("b".to_string()),
            tokens: Some(vec![Token::Text(
                "b".to_string(),
                TokenData { line: 2, column: 2 },
            )]),
            escaped: Some(false),
            angle_nest_count: Some(2),
            square_nest_count: Some(2),
            curly_nest_count: Some(2),
            looking_for_identifier: Some(false),
        };
        assert_eq!(c, res);
    }
}
