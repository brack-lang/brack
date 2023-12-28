#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Empty,
    Text(String),
    Ident(String),
    NewLine,
    AngleBracketOpen,
    AngleBracketClose,
    SquareBracketOpen,
    SquareBracketClose,
    CurlyBracketOpen,
    CurlyBracketClose,
    Comma,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tokenizer {
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
