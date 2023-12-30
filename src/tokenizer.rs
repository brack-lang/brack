use crate::tokens::{Token, Tokenizer};

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
        tokens.push(Token::Text(pool.trim().to_string()));
        return tokens;
    }

    tokens.push(Token::Text(pool.to_string()));
    tokens
}

fn tokenize_escape(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let pool = t.pool.clone().unwrap_or_default();

    let (head, tail) = separate(&s);
    let t2 = Tokenizer {
        untreated: Some(tail),
        pool: Some(format!("{}{}", pool, head)),
        escaped: Some(false),
        ..Default::default()
    };
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_back_slash(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();

    let (_, tail) = separate(&s);
    let t2 = Tokenizer {
        untreated: Some(tail),
        escaped: Some(true),
        ..Default::default()
    };
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_angle_bracket_open(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();

    let (_, tail) = separate(&s);
    let mut new_tokens = update_tokens(t, false);
    new_tokens.push(Token::AngleBracketOpen);

    let t2 = Tokenizer {
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
    let (_, tail) = separate(&s);

    let mut new_tokens = update_tokens(t, true);
    new_tokens.push(Token::AngleBracketClose);

    let mut t2 = Tokenizer {
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

    let (_, tail) = separate(&s);
    let mut new_tokens = update_tokens(t, false);
    new_tokens.push(Token::CurlyBracketOpen);

    let t2 = Tokenizer {
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
    let (_, tail) = separate(&s);

    let mut new_tokens = update_tokens(t, true);
    new_tokens.push(Token::CurlyBracketClose);

    let mut t2 = Tokenizer {
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

    let (_, tail) = separate(&s);
    let mut new_tokens = update_tokens(t, false);
    new_tokens.push(Token::SquareBracketOpen);

    let t2 = Tokenizer {
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
    let (_, tail) = separate(&s);

    let mut new_tokens = update_tokens(t, true);
    new_tokens.push(Token::SquareBracketClose);

    let mut t2 = Tokenizer {
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
    let (_, tail) = separate(&s);
    let mut new_tokens = update_tokens(t, true);
    new_tokens.push(Token::Comma);
    let t2 = Tokenizer {
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(new_tokens),
        ..Default::default()
    };
    inner_tokenize(&t.merge(&t2))
}

fn tokenize_identifier(t: &Tokenizer) -> Vec<Token> {
    let s = t.untreated.clone().unwrap_or_default();
    let (_, tail) = separate(&s);
    let mut new_tokens = t.tokens.clone().unwrap_or_default();
    new_tokens.push(Token::Ident(t.pool.clone().unwrap_or_default()));
    let t2 = Tokenizer {
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
    let (_, tail) = separate(&s);
    let pool = t.pool.clone().unwrap_or_default();
    let mut new_tokens = t.tokens.clone().unwrap_or_default();

    if !pool.trim().is_empty() {
        new_tokens.push(Token::Text(pool));
    }

    new_tokens.push(Token::NewLine);

    let t2 = Tokenizer {
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

    let (head, tail) = separate(&s);

    if head == '\0' {
        let mut updated = update_tokens(t, false);
        updated.push(Token::EOF);
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
        untreated: Some(s.to_string()),
        ..Default::default()
    };
    inner_tokenize(&t)
}

#[cfg(test)]
mod tests {
    use crate::tokens::Token;

    use super::tokenize;

    #[test]
    fn test_split_no_commands() {
        let tokens = tokenize("Hello, World!");
        assert_eq!(
            tokens,
            vec![Token::Text("Hello, World!".to_string()), Token::EOF,]
        );
    }

    #[test]
    fn test_split_commands_with_an_argument_includes_square_brackets() {
        let tokens = tokenize("Hello, [* World!]");
        assert_eq!(
            tokens,
            vec![
                Token::Text("Hello, ".to_string()),
                Token::SquareBracketOpen,
                Token::Ident("*".to_string()),
                Token::Text("World!".to_string()),
                Token::SquareBracketClose,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_split_commands_with_an_argument_includes_curly_brackets() {
        let tokens = tokenize("Hello, {* World!}");
        assert_eq!(
            tokens,
            vec![
                Token::Text("Hello, ".to_string()),
                Token::CurlyBracketOpen,
                Token::Ident("*".to_string()),
                Token::Text("World!".to_string()),
                Token::CurlyBracketClose,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_split_commands_with_an_argument_includes_angle_brackets() {
        let tokens = tokenize("Hello, <* World!>");
        assert_eq!(
            tokens,
            vec![
                Token::Text("Hello, ".to_string()),
                Token::AngleBracketOpen,
                Token::Ident("*".to_string()),
                Token::Text("World!".to_string()),
                Token::AngleBracketClose,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_split_commands_with_two_arguments_includes_square_brackets() {
        let tokens = tokenize("Hello, [@ World!, https://example.com/]");
        assert_eq!(
            tokens,
            vec![
                Token::Text("Hello, ".to_string()),
                Token::SquareBracketOpen,
                Token::Ident("@".to_string()),
                Token::Text("World!".to_string()),
                Token::Comma,
                Token::Text("https://example.com/".to_string()),
                Token::SquareBracketClose,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_split_nesting_commands() {
        let tokens = tokenize("Hello, [* [@ World!, https://example.com/]]");
        assert_eq!(
            tokens,
            vec![
                Token::Text("Hello, ".to_string()),
                Token::SquareBracketOpen,
                Token::Ident("*".to_string()),
                Token::SquareBracketOpen,
                Token::Ident("@".to_string()),
                Token::Text("World!".to_string()),
                Token::Comma,
                Token::Text("https://example.com/".to_string()),
                Token::SquareBracketClose,
                Token::SquareBracketClose,
                Token::EOF,
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
                Token::Text("Hello,".to_string()),
                Token::NewLine,
                Token::Text("World,".to_string()),
                Token::NewLine,
                Token::CurlyBracketOpen,
                Token::Ident("**".to_string()),
                Token::Text("Contact".to_string()),
                Token::CurlyBracketClose,
                Token::NewLine,
                Token::SquareBracketOpen,
                Token::Ident("@".to_string()),
                Token::Text("My website".to_string()),
                Token::Comma,
                Token::Text("https://example.com/".to_string()),
                Token::SquareBracketClose,
                Token::NewLine,
                Token::NewLine,
                Token::Text("2023.12.28".to_string()),
                Token::NewLine,
                Token::EOF,
            ]
        );
    }
}
