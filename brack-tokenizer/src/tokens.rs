#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Empty(Location),
    Text(String, Location),
    Module(String, Location),
    Ident(String, Location),
    NewLine(Location),
    Dot(Location),
    AngleBracketOpen(Location),
    AngleBracketClose(Location),
    SquareBracketOpen(Location),
    SquareBracketClose(Location),
    CurlyBracketOpen(Location),
    CurlyBracketClose(Location),
    Comma(Location),
    EOF(Location),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocationData {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub start: LocationData,
    pub end: LocationData,
}

pub fn mock_location() -> Location {
    Location {
        start: LocationData {
            line: 0,
            character: 0,
        },
        end: LocationData {
            line: 0,
            character: 0,
        },
    }
}

impl Token {
    pub fn get_location(&self) -> Location {
        match self {
            Token::Empty(location) => location.clone(),
            Token::Text(_, location) => location.clone(),
            Token::Module(_, location) => location.clone(),
            Token::Ident(_, location) => location.clone(),
            Token::NewLine(location) => location.clone(),
            Token::Dot(location) => location.clone(),
            Token::AngleBracketOpen(location) => location.clone(),
            Token::AngleBracketClose(location) => location.clone(),
            Token::SquareBracketOpen(location) => location.clone(),
            Token::SquareBracketClose(location) => location.clone(),
            Token::CurlyBracketOpen(location) => location.clone(),
            Token::CurlyBracketClose(location) => location.clone(),
            Token::Comma(location) => location.clone(),
            Token::EOF(location) => location.clone(),
        }
    }
}
