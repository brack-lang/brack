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
