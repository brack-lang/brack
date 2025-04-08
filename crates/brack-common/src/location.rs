use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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

impl LocationData {
    pub fn to_usize(&self, source: &str) -> usize {
        let lines: Vec<&str> = source.lines().collect();
        let mut offset = 0;

        for i in 0..self.line {
            offset += lines[i].len() + 1;
        }

        offset + self.character
    }
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
