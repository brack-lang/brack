use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    Empty(Location),
    Text(String, Location),
    Module(String, Location),
    Ident(String, Location),
    NewLine(Location),
    WhiteSpace(Location),
    Dot(Location),
    BackSlash(Location),
    AngleBracketOpen(Location),
    AngleBracketClose(Location),
    SquareBracketOpen(Location),
    SquareBracketClose(Location),
    CurlyBracketOpen(Location),
    CurlyBracketClose(Location),
    Comma(Location),
    EOF(Location),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationData {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

pub fn merge_location(location1: &Location, location2: &Location) -> Location {
    let start = if location1.start.line < location2.start.line {
        location1.start.clone()
    } else if location1.start.line == location2.start.line {
        if location1.start.character < location2.start.character {
            location1.start.clone()
        } else {
            location2.start.clone()
        }
    } else {
        location2.start.clone()
    };
    let end = if location1.end.line > location2.end.line {
        location1.end.clone()
    } else if location1.end.line == location2.end.line {
        if location1.end.character > location2.end.character {
            location1.end.clone()
        } else {
            location2.end.clone()
        }
    } else {
        location2.end.clone()
    };

    Location { start, end }
}

impl Token {
    pub fn get_location(&self) -> Location {
        match self {
            Token::Empty(location) => location.clone(),
            Token::Text(_, location) => location.clone(),
            Token::Module(_, location) => location.clone(),
            Token::Ident(_, location) => location.clone(),
            Token::NewLine(location) => location.clone(),
            Token::WhiteSpace(location) => location.clone(),
            Token::Dot(location) => location.clone(),
            Token::BackSlash(location) => location.clone(),
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

    pub fn set_location(&mut self, location: Location) {
        match self {
            Token::Empty(l) => *l = location,
            Token::Text(_, l) => *l = location,
            Token::Module(_, l) => *l = location,
            Token::Ident(_, l) => *l = location,
            Token::NewLine(l) => *l = location,
            Token::WhiteSpace(l) => *l = location,
            Token::Dot(l) => *l = location,
            Token::BackSlash(l) => *l = location,
            Token::AngleBracketOpen(l) => *l = location,
            Token::AngleBracketClose(l) => *l = location,
            Token::SquareBracketOpen(l) => *l = location,
            Token::SquareBracketClose(l) => *l = location,
            Token::CurlyBracketOpen(l) => *l = location,
            Token::CurlyBracketClose(l) => *l = location,
            Token::Comma(l) => *l = location,
            Token::EOF(l) => *l = location,
        }
    }
}
