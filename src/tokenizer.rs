use crate::tokens::{Token, TokenData, Tokenizer};

fn separate(s: &str) -> (char, String) {
    if s == "" {
        return ('\0', String::new());
    }
    if s.len() == 1 {
        return (s.chars().next().unwrap(), String::new());
    }
    return (s.chars().next().unwrap(), s[1..].to_string());
}

fn update_tokens(t: &Tokenizer, strip: bool) -> Vec<Token> {
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

fn tokenize_escape(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let pool = t.pool.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();

    let (head, tail) = separate(&s);
    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(format!("{}{}", pool, head)),
        escaped: Some(false),
        ..Default::default()
    };
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_back_slash(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();

    let (_, tail) = separate(&s);
    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        escaped: Some(true),
        ..Default::default()
    };
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_angle_bracket_open(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();

    let (_, tail) = separate(&s);
    let mut new_tokens = update_tokens(t, false);
    new_tokens.push(Token::AngleBracketOpen(TokenData {
        line: t.token_start_line.unwrap_or_default(),
        column,
    }));

    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        angle_nest_count: Some(t.angle_nest_count.unwrap_or_default() + 1),
        looking_for_identifier: Some(true),
        ..Default::default()
    };
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_angle_bracket_close(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();
    let (_, tail) = separate(&s);

    let mut new_tokens = update_tokens(t, true);
    new_tokens.push(Token::AngleBracketClose(TokenData {
        line: t.token_start_line.unwrap_or_default(),
        column,
    }));

    let mut t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        angle_nest_count: Some(t.angle_nest_count.unwrap_or_default() - 1),
        ..Default::default()
    };
    if t.looking_for_identifier.unwrap_or_default() {
        t2.looking_for_identifier = Some(false)
    }
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_curly_bracket_open(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();

    let (_, tail) = separate(&s);
    let mut new_tokens = update_tokens(t, false);
    new_tokens.push(Token::CurlyBracketOpen(TokenData {
        line: t.token_start_line.unwrap_or_default(),
        column,
    }));

    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        curly_nest_count: Some(t.curly_nest_count.unwrap_or_default() + 1),
        looking_for_identifier: Some(true),
        ..Default::default()
    };
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_curly_bracket_close(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();
    let (_, tail) = separate(&s);

    let mut new_tokens = update_tokens(t, true);
    new_tokens.push(Token::CurlyBracketClose(TokenData {
        line: t.token_start_line.unwrap_or_default(),
        column,
    }));

    let mut t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        curly_nest_count: Some(t.curly_nest_count.unwrap_or_default() - 1),
        ..Default::default()
    };
    if t.looking_for_identifier.unwrap_or_default() {
        t2.looking_for_identifier = Some(false)
    }
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_square_bracket_open(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();

    let (_, tail) = separate(&s);
    let mut new_tokens = update_tokens(t, false);
    new_tokens.push(Token::SquareBracketOpen(TokenData {
        line: t.token_start_line.unwrap_or_default(),
        column,
    }));

    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        square_nest_count: Some(t.square_nest_count.unwrap_or_default() + 1),
        looking_for_identifier: Some(true),
        ..Default::default()
    };
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_square_bracket_close(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();
    let (_, tail) = separate(&s);

    let mut new_tokens = update_tokens(t, true);
    new_tokens.push(Token::SquareBracketClose(TokenData {
        line: t.token_start_line.unwrap_or_default(),
        column,
    }));

    let mut t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        square_nest_count: Some(t.square_nest_count.unwrap_or_default() - 1),
        ..Default::default()
    };
    if t.looking_for_identifier.unwrap_or_default() {
        t2.looking_for_identifier = Some(false)
    }
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_arguments(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();
    let (_, tail) = separate(&s);
    let mut new_tokens = update_tokens(t, true);
    new_tokens.push(Token::Comma(TokenData {
        line: t.token_start_line.unwrap_or_default(),
        column,
    }));
    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        ..Default::default()
    };
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_identifier(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();
    let (_, tail) = separate(&s);
    let mut new_tokens = t.tokens.clone().unwrap_or_default();
    new_tokens.push(Token::Ident(
        t.pool.clone().unwrap_or_default(),
        TokenData {
            line: t.token_start_line.unwrap_or_default(),
            column: t.token_start_column.unwrap_or_default(),
        },
    ));
    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        looking_for_identifier: Some(false),
        ..Default::default()
    };
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_newline(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let line = t.line.unwrap_or_default();
    let (_, tail) = separate(&s);
    let pool = t.pool.clone().unwrap_or_default();
    let mut new_tokens = t.tokens.clone().unwrap_or_default();

    if !pool.trim().is_empty() {
        new_tokens.push(Token::Text(
            pool,
            TokenData {
                line: t.token_start_line.unwrap_or_default(),
                column: t.token_start_column.unwrap_or_default(),
            },
        ));
    }

    new_tokens.push(Token::NewLine(TokenData {
        line: t.token_start_line.unwrap_or_default(),
        column: t.column.unwrap_or_default(),
    }));

    let t2 = Tokenizer {
        line: Some(line + 1),
        column: Some(1),
        token_start_line: Some(line + 1),
        token_start_column: Some(1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        ..Default::default()
    };
    inner_tokenize(&t.merge(&t2))
}

fn inner_tokenize(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let pool = t.pool.clone().unwrap_or_default();
    let column = t.column.unwrap_or_default();

    let (head, tail) = separate(&s);

    if head == '\0' {
        let mut updated = update_tokens(t, false);
        updated.push(Token::EOF(TokenData {
            line: t.line.unwrap_or_default(),
            column,
        }));
        return updated;
    }

    if t.escaped.unwrap_or_default() {
        return tokenize_escape(t);
    }

    let angle_c = t.angle_nest_count.unwrap_or_default();
    let curly_c = t.curly_nest_count.unwrap_or_default();
    let square_c = t.square_nest_count.unwrap_or_default();
    let look_for_ident = t.looking_for_identifier.unwrap_or_default();
    match head {
        '\\' => tokenize_back_slash(t),
        '<' => tokenize_angle_bracket_open(t),
        '>' if angle_c > 0 => tokenize_angle_bracket_close(t),
        '{' => tokenize_curly_bracket_open(t),
        '}' if curly_c > 0 => tokenize_curly_bracket_close(t),
        '[' => tokenize_square_bracket_open(t),
        ']' if square_c > 0 => tokenize_square_bracket_close(t),
        ',' if (angle_c + curly_c + square_c) > 0 => tokenize_arguments(t),
        ' ' if look_for_ident => tokenize_identifier(t),
        '\n' => tokenize_newline(t),
        _ => {
            let t2 = Tokenizer {
                column: Some(column + 1),
                untreated: Some(tail),
                pool: Some(format!("{}{}", pool, head)),
                ..Default::default()
            };

            inner_tokenize(&t.merge(&t2))
        }
    }
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let t = Tokenizer {
        line: Some(1),
        column: Some(1),
        token_start_line: Some(1),
        token_start_column: Some(1),
        untreated: Some(s.to_string()),
        ..Default::default()
    };
    inner_tokenize(&t)
}

#[cfg(test)]
mod tests {
    use crate::tokens::{Token, TokenData};

    use super::tokenize;

    #[test]
    fn test_split_no_commands() {
        let tokens = tokenize("Hello, World!");
        assert_eq!(
            tokens,
            vec![
                Token::Text(
                    "Hello, World!".to_string(),
                    TokenData { line: 1, column: 1 }
                ),
                Token::EOF(TokenData {
                    line: 1,
                    column: 14,
                }),
            ]
        );
    }

    #[test]
    fn test_split_commands_with_an_argument_includes_square_brackets() {
        let tokens = tokenize("Hello, [* World!]");
        assert_eq!(
            tokens,
            vec![
                Token::Text("Hello, ".to_string(), TokenData { line: 1, column: 1 }),
                Token::SquareBracketOpen(TokenData { line: 1, column: 8 }),
                Token::Ident("*".to_string(), TokenData { line: 1, column: 9 }),
                Token::Text(
                    "World!".to_string(),
                    TokenData {
                        line: 1,
                        column: 11,
                    }
                ),
                Token::SquareBracketClose(TokenData {
                    line: 1,
                    column: 17,
                }),
                Token::EOF(TokenData {
                    line: 1,
                    column: 18,
                }),
            ]
        );
    }

    #[test]
    fn test_split_commands_with_an_argument_includes_curly_brackets() {
        let tokens = tokenize("Hello, {* World!}");
        assert_eq!(
            tokens,
            vec![
                Token::Text("Hello, ".to_string(), TokenData { line: 1, column: 1 }),
                Token::CurlyBracketOpen(TokenData { line: 1, column: 8 }),
                Token::Ident("*".to_string(), TokenData { line: 1, column: 9 }),
                Token::Text(
                    "World!".to_string(),
                    TokenData {
                        line: 1,
                        column: 11,
                    }
                ),
                Token::CurlyBracketClose(TokenData {
                    line: 1,
                    column: 17,
                }),
                Token::EOF(TokenData {
                    line: 1,
                    column: 18,
                }),
            ]
        );
    }

    #[test]
    fn test_split_commands_with_an_argument_includes_angle_brackets() {
        let tokens = tokenize("Hello, <* World!>");
        assert_eq!(
            tokens,
            vec![
                Token::Text("Hello, ".to_string(), TokenData { line: 1, column: 1 }),
                Token::AngleBracketOpen(TokenData { line: 1, column: 8 }),
                Token::Ident("*".to_string(), TokenData { line: 1, column: 9 }),
                Token::Text(
                    "World!".to_string(),
                    TokenData {
                        line: 1,
                        column: 11,
                    }
                ),
                Token::AngleBracketClose(TokenData {
                    line: 1,
                    column: 17,
                }),
                Token::EOF(TokenData {
                    line: 1,
                    column: 18,
                }),
            ]
        );
    }

    #[test]
    fn test_split_commands_with_two_arguments_includes_square_brackets() {
        let tokens = tokenize("Hello, [@ World!, https://example.com/]");
        assert_eq!(
            tokens,
            vec![
                Token::Text("Hello, ".to_string(), TokenData { line: 1, column: 1 }),
                Token::SquareBracketOpen(TokenData { line: 1, column: 8 }),
                Token::Ident("@".to_string(), TokenData { line: 1, column: 9 }),
                Token::Text(
                    "World!".to_string(),
                    TokenData {
                        line: 1,
                        column: 11,
                    }
                ),
                Token::Comma(TokenData {
                    line: 1,
                    column: 17,
                }),
                Token::Text(
                    "https://example.com/".to_string(),
                    TokenData {
                        line: 1,
                        column: 19,
                    }
                ),
                Token::SquareBracketClose(TokenData {
                    line: 1,
                    column: 39,
                }),
                Token::EOF(TokenData {
                    line: 1,
                    column: 40,
                }),
            ]
        );
    }

    #[test]
    fn test_split_nesting_commands() {
        let tokens = tokenize("Hello, [* [@ World!, https://example.com/]]");
        assert_eq!(
            tokens,
            vec![
                Token::Text("Hello, ".to_string(), TokenData { line: 1, column: 1 }),
                Token::SquareBracketOpen(TokenData { line: 1, column: 8 }),
                Token::Ident("*".to_string(), TokenData { line: 1, column: 9 }),
                Token::SquareBracketOpen(TokenData {
                    line: 1,
                    column: 11,
                }),
                Token::Ident(
                    "@".to_string(),
                    TokenData {
                        line: 1,
                        column: 12,
                    }
                ),
                Token::Text(
                    "World!".to_string(),
                    TokenData {
                        line: 1,
                        column: 14,
                    }
                ),
                Token::Comma(TokenData {
                    line: 1,
                    column: 20,
                }),
                Token::Text(
                    "https://example.com/".to_string(),
                    TokenData {
                        line: 1,
                        column: 22,
                    }
                ),
                Token::SquareBracketClose(TokenData {
                    line: 1,
                    column: 42,
                }),
                Token::SquareBracketClose(TokenData {
                    line: 1,
                    column: 43,
                }),
                Token::EOF(TokenData {
                    line: 1,
                    column: 44,
                }),
            ]
        );
    }

    #[test]
    fn test_split_newlines() {
        let tokens = tokenize(
            "Hello,\nWorld,\n{** Contact}\n[@ My website, https://example.com/]\n\n2023.12.28\n",
        );
        assert_eq!(
            tokens,
            vec![
                Token::Text("Hello,".to_string(), TokenData { line: 1, column: 1 }),
                Token::NewLine(TokenData { line: 1, column: 7 }),
                Token::Text("World,".to_string(), TokenData { line: 2, column: 1 }),
                Token::NewLine(TokenData { line: 2, column: 7 }),
                Token::CurlyBracketOpen(TokenData { line: 3, column: 1 }),
                Token::Ident("**".to_string(), TokenData { line: 3, column: 2 }),
                Token::Text("Contact".to_string(), TokenData { line: 3, column: 5 }),
                Token::CurlyBracketClose(TokenData {
                    line: 3,
                    column: 12,
                }),
                Token::NewLine(TokenData {
                    line: 3,
                    column: 13,
                }),
                Token::SquareBracketOpen(TokenData { line: 4, column: 1 }),
                Token::Ident("@".to_string(), TokenData { line: 4, column: 2 }),
                Token::Text("My website".to_string(), TokenData { line: 4, column: 4 }),
                Token::Comma(TokenData {
                    line: 4,
                    column: 14,
                }),
                Token::Text(
                    "https://example.com/".to_string(),
                    TokenData {
                        line: 4,
                        column: 16,
                    }
                ),
                Token::SquareBracketClose(TokenData {
                    line: 4,
                    column: 36,
                }),
                Token::NewLine(TokenData {
                    line: 4,
                    column: 37,
                }),
                Token::NewLine(TokenData { line: 5, column: 1 }),
                Token::Text("2023.12.28".to_string(), TokenData { line: 6, column: 1 }),
                Token::NewLine(TokenData {
                    line: 6,
                    column: 11,
                }),
                Token::EOF(TokenData { line: 7, column: 1 }),
            ]
        );
    }
}
