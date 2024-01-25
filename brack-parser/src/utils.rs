use brack_tokenizer::tokens::{Token, Location, LocationData};

pub fn check_text(tokens: &Vec<Token>) -> bool {
    matches!(tokens.first(), Some(Token::Text(_, _)))
}

pub fn check_eof(tokens: &Vec<Token>) -> bool {
    matches!(tokens.first(), Some(Token::EOF(_)))
}

pub fn consume_by_kind(tokens: &[Token], kind: Token) -> (bool, Vec<Token>) {
    let (head, tail) = tokens
        .split_first()
        .unwrap_or((&Token::EOF(Location {
            start: LocationData { line: 0, character: 0 },
            end: LocationData { line: 0, character: 0 },
        }), &[]));
    if matches_kind(head, &kind) {
        (true, tail.to_vec())
    } else {
        (false, tokens.to_vec())
    }
}

pub fn matches_kind(token: &Token, kind: &Token) -> bool {
    use Token::*;
    match (token, kind) {
        (Empty(_), Empty(_)) => true,
        (Text(_, _), Text(_, _)) => true,
        (Ident(_, _), Ident(_, _)) => true,
        (NewLine(_), NewLine(_)) => true,
        (Dot(_), Dot(_)) => true,
        (AngleBracketOpen(_), AngleBracketOpen(_)) => true,
        (AngleBracketClose(_), AngleBracketClose(_)) => true,
        (SquareBracketOpen(_), SquareBracketOpen(_)) => true,
        (SquareBracketClose(_), SquareBracketClose(_)) => true,
        (CurlyBracketOpen(_), CurlyBracketOpen(_)) => true,
        (CurlyBracketClose(_), CurlyBracketClose(_)) => true,
        (Comma(_), Comma(_)) => true,
        (EOF(_), EOF(_)) => true,
        _ => false,
    }
}
