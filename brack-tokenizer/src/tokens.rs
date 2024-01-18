#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Empty(TokenData),
    Text(String, TokenData),
    Module(String, TokenData),
    Ident(String, TokenData),
    NewLine(TokenData),
    Dot(TokenData),
    AngleBracketOpen(TokenData),
    AngleBracketClose(TokenData),
    SquareBracketOpen(TokenData),
    SquareBracketClose(TokenData),
    CurlyBracketOpen(TokenData),
    CurlyBracketClose(TokenData),
    Comma(TokenData),
    EOF(TokenData),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenData {
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn line(&self) -> usize {
        match self {
            Token::Empty(data) => data.line,
            Token::Text(_, data) => data.line,
            Token::Module(_, data) => data.line,
            Token::Ident(_, data) => data.line,
            Token::NewLine(data) => data.line,
            Token::Dot(data) => data.line,
            Token::AngleBracketOpen(data) => data.line,
            Token::AngleBracketClose(data) => data.line,
            Token::SquareBracketOpen(data) => data.line,
            Token::SquareBracketClose(data) => data.line,
            Token::CurlyBracketOpen(data) => data.line,
            Token::CurlyBracketClose(data) => data.line,
            Token::Comma(data) => data.line,
            Token::EOF(data) => data.line,
        }
    }

    pub fn column(&self) -> usize {
        match self {
            Token::Empty(data) => data.column,
            Token::Text(_, data) => data.column,
            Token::Module(_, data) => data.column,
            Token::Ident(_, data) => data.column,
            Token::NewLine(data) => data.column,
            Token::Dot(data) => data.column,
            Token::AngleBracketOpen(data) => data.column,
            Token::AngleBracketClose(data) => data.column,
            Token::SquareBracketOpen(data) => data.column,
            Token::SquareBracketClose(data) => data.column,
            Token::CurlyBracketOpen(data) => data.column,
            Token::CurlyBracketClose(data) => data.column,
            Token::Comma(data) => data.column,
            Token::EOF(data) => data.column,
        }
    }
}

pub fn mock_token_data() -> TokenData {
    TokenData { line: 0, column: 0 }
}
