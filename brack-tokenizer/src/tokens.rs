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
pub struct LocationRangeData {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocationRange {
    pub start: LocationRangeData,
    pub end: LocationRangeData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub uri: String,
    pub range: LocationRange,
}

pub fn mock_location() -> Location {
    Location {
        uri: String::from("/example/path/to"),
        range: LocationRange {
            start: LocationRangeData {
                line: 0,
                character: 0
            },
            end: LocationRangeData {
                line: 0,
                character: 0
            },
        },
    }
}
