use crate::{dispatch::dispatch, tokenizer::Tokenizer};
use brack_common::tokens::Token;

pub fn tokenize(text: &str) -> Vec<Token> {
    let t = Tokenizer {
        tokens: Some(vec![]),
        line: Some(0),
        column: Some(0),
        pool: Some(String::new()),
        token_start_line: Some(0),
        token_start_column: Some(0),
        untreated: Some(text.to_string()),
        curly_nest_count: Some(0),
        square_nest_count: Some(0),
        angle_nest_count: Some(0),
        looking_for_identifier: Some(false),
        ..Default::default()
    };
    dispatch(&t)
}

#[cfg(test)]
mod tests {
    use super::tokenize;
    use brack_common::location::{Location, LocationData};
    use brack_common::tokens::Token;
    use pretty_assertions::assert_eq;


    macro_rules! test_tokenize {
        {$name:ident, $document:expr, $expected:expr} => {
            #[test]
            fn $name(){
                let tokens = tokenize($document);
                assert_eq!(tokens, $expected);
            }
        };
    }

    test_tokenize! {
        v01,
        "",
        vec![
            Token::EOF(Location {
                start: LocationData {
                    line: 0,
                    character: 0,
                },
                end: LocationData {
                    line: 0,
                    character: 0,
                }
            }),
        ]
    }

    test_tokenize! {
        v02,
        "\n",
        vec![
            Token::NewLine(Location {
                start: LocationData {
                    line: 0,
                    character: 0,
                },
                end: LocationData {
                    line: 0,
                    character: 1,
                }
            }),
            Token::EOF(Location {
                start: LocationData {
                    line: 1,
                    character: 0,
                },
                end: LocationData { 
                    line: 1,
                    character: 0,
                }
            }),
        ]
    }

    test_tokenize! {
        v03,
        "plain text only",
        vec![
            Token::Text("plain text only".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 0,
                },
                end: LocationData {
                    line: 0,
                    character: 15
                }
            }),
            Token::EOF(Location {
                start: LocationData {
                    line: 0,
                    character: 15,
                },
                end: LocationData {
                    line: 0,
                    character: 15,
                }
            }),
        ]
    }

    test_tokenize! {
        v04,
        "\
first stmt

second stmt

third stmt",
        vec![
            Token::Text("first stmt".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 0,
                },
                end: LocationData {
                    line: 0,
                    character: 10,
                }
            }),
            Token::NewLine(Location {
                start: LocationData {
                    line: 0,
                    character: 10,
                },
                end: LocationData {
                    line: 0,
                    character: 11,
                }
            }),
            Token::NewLine(Location {
                start: LocationData {
                    line: 1,
                    character: 0,
                },
                end: LocationData {
                    line: 1,
                    character: 1,
                }
            }),
            Token::Text("second stmt".to_string(), Location {
                start: LocationData {
                    line: 2,
                    character: 0,
                },
                end: LocationData {
                    line: 2,
                    character: 11,
                }
            }),
            Token::NewLine(Location {
                start: LocationData {
                    line: 2,
                    character: 11,
                },
                end: LocationData {
                    line: 2,
                    character: 12,
                }
            }),
            Token::NewLine(Location {
                start: LocationData {
                    line: 3,
                    character: 0,
                },
                end: LocationData {
                    line: 3,
                    character: 1,
                }
            }),
            Token::Text("third stmt".to_string(), Location {
                start: LocationData {
                    line: 4,
                    character: 0,
                },
                end: LocationData {
                    line: 4,
                    character: 10,
                }
            }),
            Token::EOF(Location {
                start: LocationData {
                    line: 4,
                    character: 10,
                },
                end: LocationData {
                    line: 4,
                    character: 10,
                }
            }),

        ]
    }
    test_tokenize! {
        v05,
        "\
first stmt

second stmt

third stmt",
        vec![
            Token::Text("first stmt".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 0,
                },
                end: LocationData {
                    line: 0,
                    character: 10,
                }
            }),
            Token::NewLine(Location {
                start: LocationData {
                    line: 0,
                    character: 10,
                },
                end: LocationData {
                    line: 0,
                    character: 11,
                }
            }),
            Token::NewLine(Location {
                start: LocationData {
                    line: 1,
                    character: 0,
                },
                end: LocationData {
                    line: 1,
                    character: 1,
                }
            }),
            Token::Text("second stmt".to_string(), Location {
                start: LocationData {
                    line: 2,
                    character: 0,
                },
                end: LocationData {
                    line: 2,
                    character: 11,
                }
            }),
            Token::NewLine(Location {
                start: LocationData {
                    line: 2,
                    character: 11,
                },
                end: LocationData {
                    line: 2,
                    character: 12,
                }
            }),
            Token::NewLine(Location {
                start: LocationData {
                    line: 3,
                    character: 0,
                },
                end: LocationData {
                    line: 3,
                    character: 1,
                }
            }),
            Token::Text("third stmt".to_string(), Location {
                start: LocationData {
                    line: 4,
                    character: 0,
                },
                end: LocationData {
                    line: 4,
                    character: 10,
                }
            }),
            Token::EOF(Location {
                start: LocationData {
                    line: 4,
                    character: 10,
                },
                end: LocationData {
                    line: 4,
                    character: 10,
                }
            }),

        ]
    }

    test_tokenize! {
        v06,
        "<print>",
        vec![
            Token::AngleBracketOpen(Location {
                start: LocationData {
                    line: 0,
                    character: 0,
                },
                end: LocationData {
                    line: 0,
                    character: 1,
                }
            }),
            Token::Ident("print".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 1,
                },
                end: LocationData {
                    line: 0,
                    character: 6,
                }
            }),
            Token::AngleBracketClose(Location {
                start: LocationData {
                    line: 0,
                    character: 6,
                },
                end: LocationData {
                    line: 0,
                    character: 7,
                }
            }),
            Token::EOF(Location {
                start: LocationData {
                    line: 0,
                    character: 7,
                },
                end: LocationData {
                    line: 0,
                    character: 7,
                }
            }),
        ]
    }

    test_tokenize! {
        v07,
        "<sum 1, 2, 3>",
        vec![
            Token::AngleBracketOpen(Location {
                start: LocationData {
                    line: 0,
                    character: 0,
                },
                end: LocationData {
                    line: 0,
                    character: 1,
                }
            }),
            Token::Ident("sum".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 1,
                },
                end: LocationData {
                    line: 0,
                    character: 4,
                }
            }),
            Token::WhiteSpace(Location {
                start: LocationData {
                    line: 0,
                    character: 4,
                },
                end: LocationData {
                    line: 0,
                    character: 5,
                }
            }),
            Token::Text("1".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 5,
                },
                end: LocationData {
                    line: 0,
                    character: 6,
                }
            }),
            Token::Comma(Location {
                start: LocationData {
                    line: 0,
                    character: 6,
                },
                end: LocationData {
                    line: 0,
                    character: 7,
                }
            }),
            Token::WhiteSpace(Location {
                start: LocationData {
                    line: 0,
                    character: 7,
                },
                end: LocationData {
                    line: 0,
                    character: 8,
                }
            }),
            Token::Text("2".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 8,
                },
                end: LocationData {
                    line: 0,
                    character: 9,
                }
            }),
            Token::Comma(Location {
                start: LocationData {
                    line: 0,
                    character: 9,
                },
                end: LocationData {
                    line: 0,
                    character: 10,
                }
            }),
            Token::WhiteSpace(Location {
                start: LocationData {
                    line: 0,
                    character: 10,
                },
                end: LocationData {
                    line: 0,
                    character: 11
                }
            }),
            Token::Text("3".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 11
                },
                end :LocationData{
                   line :0 ,
                   character: 12
            }
            }),
            Token::AngleBracketClose(Location {
                start: LocationData {
                    line: 0,
                    character: 12,
                },
                end: LocationData {
                    line: 0,
                    character: 13,
                }
            }),
            Token::EOF(Location {
                start: LocationData {
                    line: 0,
                    character: 13,
                },
                end: LocationData {
                    line: 0,
                    character: 13,
                }
            }),
        ]
    }

    test_tokenize! {
        v08,
        "[plot, x=time, y=value]",
        vec![
            Token::SquareBracketOpen(Location {
                start: LocationData {
                    line: 0,
                    character: 0,
                },
                end: LocationData {
                    line: 0,
                    character: 1,
                }
            }),
            Token::Ident("plot".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 1,
                },
                end: LocationData {
                    line: 0,
                    character: 5,
                }
            }),
            Token::Comma(Location {
                start: LocationData {
                    line: 0,
                    character: 5,
                },
                end: LocationData {
                    line: 0,
                    character: 6,
                }
            }),
            Token::WhiteSpace(Location {
                start: LocationData {
                    line: 0,
                    character: 6,
                },
                end: LocationData {
                    line: 0,
                    character: 7,
                }
            }),
            Token::Ident("x".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 7,
                },
                end: LocationData {
                    line: 0,
                    character: 8,
                }
            }),
            Token::Equals(Location {
                start: LocationData {
                    line: 0,
                    character: 8,
                },
                end: LocationData {
                    line: 0,
                    character: 9,
                }
            }),
            Token::Text("time".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 9,
                },
                end: LocationData {
                    line: 0,
                    character: 13,
                }
            }),
            Token::Comma(Location {
                start: LocationData {
                    line: 0,
                    character: 13,
                },
                end: LocationData {
                    line: 0,
                    character: 14,
                }
            }),
            Token::WhiteSpace(Location {
                start: LocationData {
                    line: 0,
                    character: 14,
                },
                end: LocationData {
                    line: 0,
                    character: 15,
                }
            }),
            Token::Ident("y".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 15,
                },
                end: LocationData {
                    line: 0,
                    character: 16,
                }
            }),
            Token::Equals(Location {
                start: LocationData {
                    line: 0,
                    character: 16,
                },
                end: LocationData {
                    line: 0,
                    character: 17,
                }
            }),
            Token::Text("value".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 17,
                },
                end: LocationData {
                    line: 0,
                    character: 22,
                }
            }),
            Token::SquareBracketClose(Location {
                start: LocationData {
                    line: 0,
                    character: 22,
                },
                end: LocationData {
                    line: 0,
                    character: 23,
                }
            }),
            Token::EOF(Location {
                start: LocationData {
                    line: 0,
                    character: 23,
                },
                end: LocationData {
                    line: 0,
                    character: 23,
                }
            }),
        ]
    }

    test_tokenize! {
        v09,
        "{macro, <include, \"file.txt\"> }",
        vec! [
            Token::CurlyBracketOpen(Location {
                start: LocationData {
                    line: 0,
                    character: 0,
                },
                end: LocationData {
                    line: 0,
                    character: 1,
                }
            }),
            Token::Ident("macro".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 1,
                },
                end: LocationData {
                    line: 0,
                    character: 6,
                }
            }),
            Token::Comma(Location {
                start: LocationData {
                    line: 0,
                    character: 6,
                },
                end: LocationData {
                    line: 0,
                    character: 7,
                }
            }),
            Token::WhiteSpace(Location {
                start: LocationData {
                    line: 0,
                    character: 7,
                },
                end: LocationData {
                    line: 0,
                    character: 8,
                }
            }),
            Token::AngleBracketOpen(Location {
                start: LocationData {
                    line: 0,
                    character: 8,
                },
                end: LocationData {
                    line: 0,
                    character: 9,
                }
            }),
            Token::Ident("include".to_string(), Location {
                start: LocationData {
                    line: 0,
                    character: 9,
                },
                end: LocationData {
                    line: 0,
                    character: 16,
                }
            }),
            Token::Comma(Location {
                start: LocationData {
                    line: 0,
                    character: 16,
                },
                end: LocationData {
                    line: 0,
                    character: 17,
                }
            }),
            Token::WhiteSpace(Location {
                start: LocationData {
                    line: 0,
                    character: 17
                },
                end :LocationData{
                   line :0 ,
                   character :18
            }
            }),
            Token::DoubleQuote(Location{
               start :LocationData{
                  line :0 ,
                  character :18
               },
               end :LocationData{
                  line :0 ,
                  character :19
               }
            }),
            Token::String("file.txt".to_string(), Location{
               start :LocationData{
                  line :0 ,
                  character :19
               },
               end :LocationData{
                  line :0 ,
                  character :28
               }
            }),
            Token::DoubleQuote(Location{
               start :LocationData{
                  line :0 ,
                  character :28
               },
               end :LocationData{
                  line :0 ,
                  character :29
               }
            }),
            Token::AngleBracketClose(Location {
                start: LocationData {
                    line: 0,
                    character: 29,
                },
                end: LocationData {
                    line: 0,
                    character: 30,
                }
            }),
            Token::WhiteSpace(Location {
                start: LocationData {
                    line: 0,
                    character: 30,
                },
                end: LocationData {
                    line: 0,
                    character: 31,
                }
            }),
            Token::CurlyBracketClose(Location {
                start: LocationData {
                    line: 0,
                    character: 31,
                },
                end: LocationData {
                    line: 0,
                    character: 32,
                }
            }),
            Token::EOF(Location {
                start: LocationData {
                    line: 0,
                    character: 32,
                },
                end: LocationData {
                    line: 0,
                    character: 32,
                }
            }),
        ]
    }

}
