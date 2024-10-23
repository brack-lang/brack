use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
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
    let start = match location1.start.line.cmp(&location2.start.line) {
        Ordering::Less => location1.start.clone(),
        Ordering::Equal => match location1.start.character.cmp(&location2.start.character) {
            Ordering::Less => location1.start.clone(),
            _ => location2.start.clone(),
        },
        Ordering::Greater => location2.start.clone(),
    };

    let end = match location1.end.line.cmp(&location2.end.line) {
        Ordering::Less => location2.end.clone(),
        Ordering::Equal => match location1.end.line.cmp(&location2.end.character) {
            Ordering::Greater => location1.end.clone(),
            _ => location2.end.clone(),
        },
        Ordering::Greater => location1.end.clone(),
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
