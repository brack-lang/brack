use crate::{
    tokenizer::Tokenizer,
    tokens::{Token, TokenData},
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
            TokenData {
                line: t.token_start_line.unwrap_or_default(),
                column: t.token_start_column.unwrap_or_default() + space_count,
            },
        ));
        return tokens;
    }

    tokens.push(Token::Text(
        pool.to_string(),
        TokenData {
            line: t.token_start_line.unwrap_or_default(),
            column: t.token_start_column.unwrap_or_default(),
        },
    ));
    tokens
}
