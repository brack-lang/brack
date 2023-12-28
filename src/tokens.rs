pub enum Token {
    Empty,
    Text(String),
    NewLine,
    AngleBracketOpen,
    AngleBracketClose,
    SquareBracketOpen,
    SquareBracketClose,
    CurlyBracketOpen,
    CurlyBracketClose,
    Comma,
}

pub struct Tokenizer {
    untreated: Option<String>,
    pool: Option<String>,
    tokens: Option<Vec<Token>>,
    escaped: Option<bool>,
    angle_nest_count: Option<i32>,
    square_nest_count: Option<i32>,
    curly_nest_count: Option<i32>,
    looking_for_identifier: Option<bool>,
}

impl Tokenizer {
    pub fn merge(self, other: Tokenizer) -> Tokenizer {
        Tokenizer {
            untreated: match self.untreated {
                Some(untreated) => Some(untreated),
                None => other.untreated,
            },
            pool: match self.pool {
                Some(pool) => Some(pool),
                None => other.pool,
            },
            tokens: match self.tokens {
                Some(tokens) => Some(tokens),
                None => other.tokens,
            },
            escaped: match self.escaped {
                Some(escaped) => Some(escaped),
                None => other.escaped,
            },
            angle_nest_count: match self.angle_nest_count {
                Some(angle_nest_count) => Some(angle_nest_count),
                None => other.angle_nest_count,
            },
            square_nest_count: match self.square_nest_count {
                Some(square_nest_count) => Some(square_nest_count),
                None => other.square_nest_count,
            },
            curly_nest_count: match self.curly_nest_count {
                Some(curly_nest_count) => Some(curly_nest_count),
                None => other.curly_nest_count,
            },
            looking_for_identifier: match self.looking_for_identifier {
                Some(looking_for_identifier) => Some(looking_for_identifier),
                None => other.looking_for_identifier,
            },
        }
    }
}
