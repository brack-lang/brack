use crate::{
    tokenizer::Tokenizer,
    tokens::{Location, LocationData, Token},
};

pub fn separate(s: &str) -> (char, String) {
    if s == "" {
        return ('\0', String::new());
    }
    if s.len() == 1 {
        return (s.chars().next().unwrap(), String::new());
    }
    return (s.chars().next().unwrap(), s[1..].to_string());
}

pub fn take_text_token_from_pool(t: &Tokenizer, strip: bool) -> Option<(Tokenizer, Token)> {
    let pool = t.pool.clone().unwrap_or_default();

    if &pool == "" {
        return None;        
    }

    if strip {
        let mut space_count = 0;
        for c in pool.chars() {
            if c == ' ' {
                space_count += 1;
            } else {
                break;
            }
        }
        let t2 = Tokenizer {
            column: Some(t.column.unwrap_or_default()),
            token_start_column: Some(t.token_start_column.unwrap_or_default() + space_count),
            ..Default::default()
        };
        return Some((t.merge(&t2), Token::Text(
            pool.trim().to_string(),
            Location {
                start: LocationData {
                    line: t.token_start_line.unwrap_or_default(),
                    character: t.token_start_column.unwrap_or_default() + space_count,
                },
                end: LocationData {
                    line: t.line.unwrap_or_default(),
                    character: t.column.unwrap_or_default(),
                },
            },
        )));
    }

    let t2 = Tokenizer {
        token_start_column: Some(t.column.unwrap_or_default()),
        ..Default::default()
    };
    return Some((t.merge(&t2), Token::Text(
        pool.to_string(),
        Location {
            start: LocationData {
                line: t.token_start_line.unwrap_or_default(),
                character: t.token_start_column.unwrap_or_default(),
            },
            end: LocationData {
                line: t.line.unwrap_or_default(),
                character: t.column.unwrap_or_default(),
            },
        },
    )));
}

pub fn update_tokens(t: &Tokenizer, strip: bool) -> Vec<Token> {
    let mut tokens = t.tokens.clone().unwrap_or_default();
    let pool = t.pool.clone().unwrap_or_default();

    if &pool == "" {
        return tokens;
    }

    if strip {
        let mut space_count = 0;
        for c in pool.chars() {
            if c == ' ' {
                space_count += 1;
            } else {
                break;
            }
        }
        tokens.push(Token::Text(
            pool.trim().to_string(),
            Location {
                start: LocationData {
                    line: t.token_start_line.unwrap_or_default() + space_count,
                    character: t.token_start_column.unwrap_or_default(),
                },
                end: LocationData {
                    line: t.line.unwrap_or_default(),
                    character: t.column.unwrap_or_default(),
                },
            },
        ));
        return tokens;
    }

    tokens.push(Token::Text(
        pool.to_string(),
        Location {
            start: LocationData {
                line: t.token_start_line.unwrap_or_default(),
                character: t.token_start_column.unwrap_or_default(),
            },
            end: LocationData {
                line: t.line.unwrap_or_default(),
                character: t.column.unwrap_or_default(),
            },
        },
    ));
    tokens
}
