use anyhow::Result;
use std::{fs::File, io::Read, path::Path};

use crate::{dispatch::dispatch, tokenizer::Tokenizer, tokens::Token};

pub fn tokenize<P: AsRef<Path>>(path: P) -> Result<Vec<Token>> {
    let mut file = File::open(&path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let t = Tokenizer {
        line: Some(1),
        column: Some(1),
        token_start_line: Some(1),
        token_start_column: Some(1),
        untreated: Some(text),
        ..Default::default()
    };
    Ok(dispatch(&t))
}

#[cfg(test)]
mod tests {
    use super::tokenize;
    use crate::tokens::{Location, LocationData, Token};
    use anyhow::Result;

    #[test]
    fn test_split_no_commands() -> Result<()> {
        let pwd = std::env::current_dir()?;
        let uri = pwd
            .join("text/split_no_commands.[]")
            .to_string_lossy()
            .to_string();
        let tokens = tokenize(uri.clone())?;
        assert_eq!(
            tokens,
            vec![
                Token::Text(
                    "Hello, World!".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 1,
                        },
                        end: LocationData {
                            line: 1,
                            character: 14,
                        }
                    },
                ),
                Token::EOF(Location {
                    start: LocationData {
                        line: 1,
                        character: 14,
                    },
                    end: LocationData {
                        line: 1,
                        character: 14,
                    }
                }),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_split_commands_with_an_argument_includes_square_brackets() -> Result<()> {
        let pwd = std::env::current_dir()?;
        let uri = pwd
            .join("text/split_commands_with_an_argument_includes_square_brackets.[]")
            .to_string_lossy()
            .to_string();
        let tokens = tokenize(uri.clone())?;
        assert_eq!(
            tokens,
            vec![
                Token::Text(
                    "Hello, ".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 1,
                        },
                        end: LocationData {
                            line: 1,
                            character: 8,
                        },
                    },
                ),
                Token::SquareBracketOpen(Location {
                    start: LocationData {
                        line: 1,
                        character: 8,
                    },
                    end: LocationData {
                        line: 1,
                        character: 9,
                    },
                }),
                Token::Module(
                    "std".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 9,
                        },
                        end: LocationData {
                            line: 1,
                            character: 12,
                        },
                    }
                ),
                Token::Dot(Location {
                    start: LocationData {
                        line: 1,
                        character: 12,
                    },
                    end: LocationData {
                        line: 1,
                        character: 13,
                    },
                }),
                Token::Ident(
                    "*".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 13,
                        },
                        end: LocationData {
                            line: 1,
                            character: 14,
                        },
                    }
                ),
                Token::Text(
                    "World!".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 14,
                        },
                        end: LocationData {
                            line: 1,
                            character: 20,
                        },
                    }
                ),
                Token::SquareBracketClose(Location {
                    start: LocationData {
                        line: 1,
                        character: 20,
                    },
                    end: LocationData {
                        line: 1,
                        character: 21,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 1,
                        character: 21,
                    },
                    end: LocationData {
                        line: 1,
                        character: 21,
                    },
                }),
            ]
        );
        Ok(())
    }

    //     #[test]
    //     fn test_split_commands_with_an_argument_includes_curly_brackets() {
    //         let tokens = tokenize("Hello, {std.* World!}");
    //         assert_eq!(
    //             tokens,
    //             vec![
    //                 Token::Text("Hello, ".to_string(), TokenData { line: 1, column: 1 }),
    //                 Token::CurlyBracketOpen(TokenData { line: 1, column: 8 }),
    //                 Token::Module("std".to_string(), TokenData { line: 1, column: 9 }),
    //                 Token::Dot(TokenData {
    //                     line: 1,
    //                     column: 12,
    //                 }),
    //                 Token::Ident(
    //                     "*".to_string(),
    //                     TokenData {
    //                         line: 1,
    //                         column: 13
    //                     }
    //                 ),
    //                 Token::Text(
    //                     "World!".to_string(),
    //                     TokenData {
    //                         line: 1,
    //                         column: 15,
    //                     }
    //                 ),
    //                 Token::CurlyBracketClose(TokenData {
    //                     line: 1,
    //                     column: 21,
    //                 }),
    //                 Token::EOF(TokenData {
    //                     line: 1,
    //                     column: 22,
    //                 }),
    //             ]
    //         );
    //     }

    //     #[test]
    //     fn test_split_commands_with_an_argument_includes_angle_brackets() {
    //         let tokens = tokenize("Hello, <* World!>");
    //         assert_eq!(
    //             tokens,
    //             vec![
    //                 Token::Text("Hello, ".to_string(), TokenData { line: 1, column: 1 }),
    //                 Token::AngleBracketOpen(TokenData { line: 1, column: 8 }),
    //                 Token::Ident("*".to_string(), TokenData { line: 1, column: 9 }),
    //                 Token::Text(
    //                     "World!".to_string(),
    //                     TokenData {
    //                         line: 1,
    //                         column: 11,
    //                     }
    //                 ),
    //                 Token::AngleBracketClose(TokenData {
    //                     line: 1,
    //                     column: 17,
    //                 }),
    //                 Token::EOF(TokenData {
    //                     line: 1,
    //                     column: 18,
    //                 }),
    //             ]
    //         );
    //     }

    //     #[test]
    //     fn test_split_commands_with_two_arguments_includes_square_brackets() {
    //         let tokens = tokenize("Hello, [std.@ World!, https://example.com/]");
    //         assert_eq!(
    //             tokens,
    //             vec![
    //                 Token::Text("Hello, ".to_string(), TokenData { line: 1, column: 1 }),
    //                 Token::SquareBracketOpen(TokenData { line: 1, column: 8 }),
    //                 Token::Module("std".to_string(), TokenData { line: 1, column: 9 }),
    //                 Token::Dot(TokenData {
    //                     line: 1,
    //                     column: 12,
    //                 }),
    //                 Token::Ident(
    //                     "@".to_string(),
    //                     TokenData {
    //                         line: 1,
    //                         column: 13
    //                     }
    //                 ),
    //                 Token::Text(
    //                     "World!".to_string(),
    //                     TokenData {
    //                         line: 1,
    //                         column: 15,
    //                     }
    //                 ),
    //                 Token::Comma(TokenData {
    //                     line: 1,
    //                     column: 21,
    //                 }),
    //                 Token::Text(
    //                     "https://example.com/".to_string(),
    //                     TokenData {
    //                         line: 1,
    //                         column: 23,
    //                     }
    //                 ),
    //                 Token::SquareBracketClose(TokenData {
    //                     line: 1,
    //                     column: 43,
    //                 }),
    //                 Token::EOF(TokenData {
    //                     line: 1,
    //                     column: 44,
    //                 }),
    //             ]
    //         );
    //     }

    //     #[test]
    //     fn test_split_nesting_commands() {
    //         let tokens = tokenize("Hello, [std.* [std.@ World!, https://example.com/]]");
    //         assert_eq!(
    //             tokens,
    //             vec![
    //                 Token::Text("Hello, ".to_string(), TokenData { line: 1, column: 1 }),
    //                 Token::SquareBracketOpen(TokenData { line: 1, column: 8 }),
    //                 Token::Module("std".to_string(), TokenData { line: 1, column: 9 }),
    //                 Token::Dot(TokenData {
    //                     line: 1,
    //                     column: 12,
    //                 }),
    //                 Token::Ident(
    //                     "*".to_string(),
    //                     TokenData {
    //                         line: 1,
    //                         column: 13
    //                     }
    //                 ),
    //                 Token::SquareBracketOpen(TokenData {
    //                     line: 1,
    //                     column: 15,
    //                 }),
    //                 Token::Module(
    //                     "std".to_string(),
    //                     TokenData {
    //                         line: 1,
    //                         column: 16,
    //                     }
    //                 ),
    //                 Token::Dot(TokenData {
    //                     line: 1,
    //                     column: 19,
    //                 }),
    //                 Token::Ident(
    //                     "@".to_string(),
    //                     TokenData {
    //                         line: 1,
    //                         column: 20,
    //                     }
    //                 ),
    //                 Token::Text(
    //                     "World!".to_string(),
    //                     TokenData {
    //                         line: 1,
    //                         column: 22,
    //                     }
    //                 ),
    //                 Token::Comma(TokenData {
    //                     line: 1,
    //                     column: 28,
    //                 }),
    //                 Token::Text(
    //                     "https://example.com/".to_string(),
    //                     TokenData {
    //                         line: 1,
    //                         column: 30,
    //                     }
    //                 ),
    //                 Token::SquareBracketClose(TokenData {
    //                     line: 1,
    //                     column: 50,
    //                 }),
    //                 Token::SquareBracketClose(TokenData {
    //                     line: 1,
    //                     column: 51,
    //                 }),
    //                 Token::EOF(TokenData {
    //                     line: 1,
    //                     column: 52,
    //                 }),
    //             ]
    //         );
    //     }

    //     #[test]
    //     fn test_split_newlines() {
    //         let tokens = tokenize(
    //             "Hello,\nWorld,\n{std.** Contact}\n[std.@ My website, https://example.com/]\n\n2023.12.28\n",
    //         );
    //         assert_eq!(
    //             tokens,
    //             vec![
    //                 Token::Text("Hello,".to_string(), TokenData { line: 1, column: 1 }),
    //                 Token::NewLine(TokenData { line: 1, column: 7 }),
    //                 Token::Text("World,".to_string(), TokenData { line: 2, column: 1 }),
    //                 Token::NewLine(TokenData { line: 2, column: 7 }),
    //                 Token::CurlyBracketOpen(TokenData { line: 3, column: 1 }),
    //                 Token::Module("std".to_string(), TokenData { line: 3, column: 2 }),
    //                 Token::Dot(TokenData { line: 3, column: 5 }),
    //                 Token::Ident("**".to_string(), TokenData { line: 3, column: 6 }),
    //                 Token::Text("Contact".to_string(), TokenData { line: 3, column: 9 }),
    //                 Token::CurlyBracketClose(TokenData {
    //                     line: 3,
    //                     column: 16,
    //                 }),
    //                 Token::NewLine(TokenData {
    //                     line: 3,
    //                     column: 17,
    //                 }),
    //                 Token::SquareBracketOpen(TokenData { line: 4, column: 1 }),
    //                 Token::Module("std".to_string(), TokenData { line: 4, column: 2 }),
    //                 Token::Dot(TokenData { line: 4, column: 5 }),
    //                 Token::Ident("@".to_string(), TokenData { line: 4, column: 6 }),
    //                 Token::Text("My website".to_string(), TokenData { line: 4, column: 8 }),
    //                 Token::Comma(TokenData {
    //                     line: 4,
    //                     column: 18,
    //                 }),
    //                 Token::Text(
    //                     "https://example.com/".to_string(),
    //                     TokenData {
    //                         line: 4,
    //                         column: 20,
    //                     }
    //                 ),
    //                 Token::SquareBracketClose(TokenData {
    //                     line: 4,
    //                     column: 40,
    //                 }),
    //                 Token::NewLine(TokenData {
    //                     line: 4,
    //                     column: 41,
    //                 }),
    //                 Token::NewLine(TokenData { line: 5, column: 1 }),
    //                 Token::Text("2023.12.28".to_string(), TokenData { line: 6, column: 1 }),
    //                 Token::NewLine(TokenData {
    //                     line: 6,
    //                     column: 11,
    //                 }),
    //                 Token::EOF(TokenData { line: 7, column: 1 }),
    //             ]
    //         );
    //     }
    // }
}
