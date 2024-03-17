use crate::{
    tokenizer::Tokenizer,
    tokens::{Location, LocationData, Token},
};

use unicode_segmentation::UnicodeSegmentation;

pub fn separate(s: &str) -> (String, String) {
    let graphemes = s.graphemes(true);
    match graphemes.count() {
        0 => ('\0'.to_string(), String::new()),
        1 => (s.graphemes(true).nth(0).unwrap().to_string(), String::new()),
        _ => {
            let mut graphemes = s.graphemes(true);
            let head = graphemes.next().unwrap().to_string();
            let tail = graphemes.collect::<String>();
            (head, tail)
        }
    }
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
        return Some((
            t.merge(&t2),
            Token::Text(
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
            ),
        ));
    }

    let t2 = Tokenizer {
        token_start_column: Some(t.column.unwrap_or_default()),
        ..Default::default()
    };
    return Some((
        t.merge(&t2),
        Token::Text(
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
        ),
    ));
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
