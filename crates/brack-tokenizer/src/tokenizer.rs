use crate::tokens::Token;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tokenizer {
    pub uri: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub token_start_line: Option<usize>,
    pub token_start_column: Option<usize>,
    pub untreated: Option<String>,
    pub pool: Option<String>,
    pub tokens: Option<Vec<Token>>,
    pub angle_nest_count: Option<i32>,
    pub square_nest_count: Option<i32>,
    pub curly_nest_count: Option<i32>,
    pub looking_for_identifier: Option<bool>,
}

impl Tokenizer {
    pub fn merge(&self, other: &Tokenizer) -> Tokenizer {
        Tokenizer {
            uri: other.uri.clone(),
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
    use crate::tokens::{Location, LocationData};

    #[test]
    fn test_merge() {
        use super::*;
        let a = Tokenizer {
            uri: "a".to_string(),
            line: Some(1),
            column: Some(1),
            token_start_line: Some(1),
            token_start_column: Some(1),
            untreated: Some("a".to_string()),
            pool: Some("a".to_string()),
            tokens: Some(vec![Token::Text(
                "a".to_string(),
                Location {
                    start: LocationData {
                        line: 1,
                        character: 1,
                    },
                    end: LocationData {
                        line: 1,
                        character: 1,
                    },
                },
            )]),
            angle_nest_count: Some(1),
            square_nest_count: Some(1),
            curly_nest_count: Some(1),
            looking_for_identifier: Some(true),
        };
        let b = Tokenizer {
            uri: "b".to_string(),
            line: Some(2),
            column: Some(2),
            token_start_line: Some(2),
            token_start_column: Some(2),
            untreated: Some("b".to_string()),
            pool: Some("b".to_string()),
            tokens: Some(vec![Token::Text(
                "b".to_string(),
                Location {
                    start: LocationData {
                        line: 2,
                        character: 2,
                    },
                    end: LocationData {
                        line: 2,
                        character: 2,
                    },
                },
            )]),
            angle_nest_count: Some(2),
            square_nest_count: Some(2),
            curly_nest_count: Some(2),
            looking_for_identifier: Some(false),
        };
        let c = a.merge(&b);
        let res = Tokenizer {
            uri: "b".to_string(),
            line: Some(2),
            column: Some(2),
            token_start_line: Some(2),
            token_start_column: Some(2),
            untreated: Some("b".to_string()),
            pool: Some("b".to_string()),
            tokens: Some(vec![Token::Text(
                "b".to_string(),
                Location {
                    start: LocationData {
                        line: 2,
                        character: 2,
                    },
                    end: LocationData {
                        line: 2,
                        character: 2,
                    },
                },
            )]),
            angle_nest_count: Some(2),
            square_nest_count: Some(2),
            curly_nest_count: Some(2),
            looking_for_identifier: Some(false),
        };
        assert_eq!(c, res);
    }
}
