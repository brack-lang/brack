use anyhow::Result;
use std::{fs::File, io::Read, path::Path};

use crate::{dispatch::dispatch, tokenizer::Tokenizer, tokens::Token};

pub fn tokenize<P: AsRef<Path>>(path: P) -> Result<Vec<Token>> {
    let mut file = File::open(&path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let t = Tokenizer {
        line: Some(0),
        column: Some(0),
        token_start_line: Some(0),
        token_start_column: Some(0),
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
                            line: 0,
                            character: 0,
                        },
                        end: LocationData {
                            line: 0,
                            character: 13,
                        }
                    },
                ),
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
                            line: 0,
                            character: 0,
                        },
                        end: LocationData {
                            line: 0,
                            character: 7,
                        },
                    },
                ),
                Token::SquareBracketOpen(Location {
                    start: LocationData {
                        line: 0,
                        character: 7,
                    },
                    end: LocationData {
                        line: 0,
                        character: 8,
                    },
                }),
                Token::Module(
                    "std".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 8,
                        },
                        end: LocationData {
                            line: 0,
                            character: 11,
                        },
                    }
                ),
                Token::Dot(Location {
                    start: LocationData {
                        line: 0,
                        character: 11,
                    },
                    end: LocationData {
                        line: 0,
                        character: 12,
                    },
                }),
                Token::Ident(
                    "*".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 12,
                        },
                        end: LocationData {
                            line: 0,
                            character: 13,
                        },
                    }
                ),
                Token::Text(
                    "World!".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 14,
                        },
                        end: LocationData {
                            line: 0,
                            character: 20,
                        },
                    }
                ),
                Token::SquareBracketClose(Location {
                    start: LocationData {
                        line: 0,
                        character: 20,
                    },
                    end: LocationData {
                        line: 0,
                        character: 21,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 0,
                        character: 21,
                    },
                    end: LocationData {
                        line: 0,
                        character: 21,
                    },
                }),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_split_commands_with_an_argument_includes_curly_brackets() -> Result<()> {
        let pwd = std::env::current_dir()?;
        let uri = pwd
            .join("text/split_commands_with_an_argument_includes_curly_brackets.[]")
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
                            line: 0,
                            character: 0,
                        },
                        end: LocationData {
                            line: 0,
                            character: 7,
                        },
                    }
                ),
                Token::CurlyBracketOpen(Location {
                    start: LocationData {
                        line: 0,
                        character: 7,
                    },
                    end: LocationData {
                        line: 0,
                        character: 8,
                    },
                }),
                Token::Module(
                    "std".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 8,
                        },
                        end: LocationData {
                            line: 0,
                            character: 11,
                        },
                    }
                ),
                Token::Dot(Location {
                    start: LocationData {
                        line: 0,
                        character: 11,
                    },
                    end: LocationData {
                        line: 0,
                        character: 12,
                    },
                }),
                Token::Ident(
                    "*".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 12,
                        },
                        end: LocationData {
                            line: 0,
                            character: 13,
                        },
                    }
                ),
                Token::Text(
                    "World!".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 14,
                        },
                        end: LocationData {
                            line: 0,
                            character: 20,
                        },
                    }
                ),
                Token::CurlyBracketClose(Location {
                    start: LocationData {
                        line: 0,
                        character: 20,
                    },
                    end: LocationData {
                        line: 0,
                        character: 21,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 0,
                        character: 21,
                    },
                    end: LocationData {
                        line: 0,
                        character: 21,
                    },
                }),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_split_commands_with_an_argument_includes_angle_brackets() -> Result<()> {
        let pwd = std::env::current_dir()?;
        let uri = pwd
            .join("text/split_commands_with_an_argument_includes_angle_brackets.[]")
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
                            line: 0,
                            character: 0,
                        },
                        end: LocationData {
                            line: 0,
                            character: 7,
                        },
                    }
                ),
                Token::AngleBracketOpen(Location {
                    start: LocationData {
                        line: 0,
                        character: 7,
                    },
                    end: LocationData {
                        line: 0,
                        character: 8,
                    },
                }),
                Token::Ident(
                    "*".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 8,
                        },
                        end: LocationData {
                            line: 0,
                            character: 9,
                        },
                    }
                ),
                Token::Text(
                    "World!".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 10,
                        },
                        end: LocationData {
                            line: 0,
                            character: 16,
                        },
                    }
                ),
                Token::AngleBracketClose(Location {
                    start: LocationData {
                        line: 0,
                        character: 16,
                    },
                    end: LocationData {
                        line: 0,
                        character: 17,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 0,
                        character: 17,
                    },
                    end: LocationData {
                        line: 0,
                        character: 17,
                    },
                }),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_split_commands_with_two_arguments_includes_square_brackets() -> Result<()> {
        let pwd = std::env::current_dir()?;
        let uri = pwd
            .join("text/split_commands_with_two_arguments_includes_square_brackets.[]")
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
                            line: 0,
                            character: 0,
                        },
                        end: LocationData {
                            line: 0,
                            character: 7,
                        },
                    }
                ),
                Token::SquareBracketOpen(Location {
                    start: LocationData {
                        line: 0,
                        character: 7,
                    },
                    end: LocationData {
                        line: 0,
                        character: 8,
                    },
                }),
                Token::Module(
                    "std".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 8,
                        },
                        end: LocationData {
                            line: 0,
                            character: 11,
                        },
                    }
                ),
                Token::Dot(Location {
                    start: LocationData {
                        line: 0,
                        character: 11,
                    },
                    end: LocationData {
                        line: 0,
                        character: 12,
                    },
                }),
                Token::Ident(
                    "@".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 12,
                        },
                        end: LocationData {
                            line: 0,
                            character: 13,
                        },
                    }
                ),
                Token::Text(
                    "World!".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 14,
                        },
                        end: LocationData {
                            line: 0,
                            character: 20,
                        },
                    }
                ),
                Token::Comma(Location {
                    start: LocationData {
                        line: 0,
                        character: 20,
                    },
                    end: LocationData {
                        line: 0,
                        character: 21,
                    },
                }),
                Token::Text(
                    "https://example.com/".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 22,
                        },
                        end: LocationData {
                            line: 0,
                            character: 42,
                        },
                    }
                ),
                Token::SquareBracketClose(Location {
                    start: LocationData {
                        line: 0,
                        character: 42,
                    },
                    end: LocationData {
                        line: 0,
                        character: 43,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 0,
                        character: 43,
                    },
                    end: LocationData {
                        line: 0,
                        character: 43,
                    },
                }),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_split_nesting_commands() -> Result<()> {
        let pwd = std::env::current_dir()?;
        let uri = pwd
            .join("text/split_nesting_commands.[]")
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
                            line: 0,
                            character: 0,
                        },
                        end: LocationData {
                            line: 0,
                            character: 7,
                        },
                    }
                ),
                Token::SquareBracketOpen(Location {
                    start: LocationData {
                        line: 0,
                        character: 7,
                    },
                    end: LocationData {
                        line: 0,
                        character: 8,
                    },
                }),
                Token::Module(
                    "std".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 8,
                        },
                        end: LocationData {
                            line: 0,
                            character: 11,
                        },
                    }
                ),
                Token::Dot(Location {
                    start: LocationData {
                        line: 0,
                        character: 11,
                    },
                    end: LocationData {
                        line: 0,
                        character: 12,
                    },
                }),
                Token::Ident(
                    "*".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 12,
                        },
                        end: LocationData {
                            line: 0,
                            character: 13,
                        },
                    }
                ),
                Token::SquareBracketOpen(Location {
                    start: LocationData {
                        line: 0,
                        character: 14,
                    },
                    end: LocationData {
                        line: 0,
                        character: 15,
                    },
                }),
                Token::Module(
                    "std".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 15,
                        },
                        end: LocationData {
                            line: 0,
                            character: 18,
                        },
                    }
                ),
                Token::Dot(Location {
                    start: LocationData {
                        line: 0,
                        character: 18,
                    },
                    end: LocationData {
                        line: 0,
                        character: 19,
                    },
                }),
                Token::Ident(
                    "@".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 19,
                        },
                        end: LocationData {
                            line: 0,
                            character: 20,
                        },
                    }
                ),
                Token::Text(
                    "World!".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 21,
                        },
                        end: LocationData {
                            line: 0,
                            character: 27,
                        },
                    }
                ),
                Token::Comma(Location {
                    start: LocationData {
                        line: 0,
                        character: 27,
                    },
                    end: LocationData {
                        line: 0,
                        character: 28,
                    },
                }),
                Token::Text(
                    "https://example.com/".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 29,
                        },
                        end: LocationData {
                            line: 0,
                            character: 49,
                        },
                    }
                ),
                Token::SquareBracketClose(Location {
                    start: LocationData {
                        line: 0,
                        character: 49,
                    },
                    end: LocationData {
                        line: 0,
                        character: 50,
                    },
                }),
                Token::SquareBracketClose(Location {
                    start: LocationData {
                        line: 0,
                        character: 50,
                    },
                    end: LocationData {
                        line: 0,
                        character: 51,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 0,
                        character: 51,
                    },
                    end: LocationData {
                        line: 0,
                        character: 51,
                    },
                }),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_split_newlines() -> Result<()> {
        let pwd = std::env::current_dir()?;
        let uri = pwd
            .join("text/split_newlines.[]")
            .to_string_lossy()
            .to_string();
        let tokens = tokenize(uri.clone())?;

        assert_eq!(
            tokens,
            vec![
                Token::Text(
                    "Hello,".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 0,
                        },
                        end: LocationData {
                            line: 0,
                            character: 6,
                        },
                    }
                ),
                Token::NewLine(Location {
                    start: LocationData {
                        line: 0,
                        character: 6,
                    },
                    end: LocationData {
                        line: 0,
                        character: 7,
                    },
                }),
                Token::Text(
                    "World,".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 0,
                        },
                        end: LocationData {
                            line: 1,
                            character: 6,
                        },
                    }
                ),
                Token::NewLine(Location {
                    start: LocationData {
                        line: 1,
                        character: 6,
                    },
                    end: LocationData {
                        line: 1,
                        character: 7,
                    },
                }),
                Token::CurlyBracketOpen(Location {
                    start: LocationData {
                        line: 2,
                        character: 0,
                    },
                    end: LocationData {
                        line: 2,
                        character: 1,
                    },
                }),
                Token::Module(
                    "std".to_string(),
                    Location {
                        start: LocationData {
                            line: 2,
                            character: 1,
                        },
                        end: LocationData {
                            line: 2,
                            character: 4,
                        },
                    }
                ),
                Token::Dot(Location {
                    start: LocationData {
                        line: 2,
                        character: 4,
                    },
                    end: LocationData {
                        line: 2,
                        character: 5,
                    },
                }),
                Token::Ident(
                    "**".to_string(),
                    Location {
                        start: LocationData {
                            line: 2,
                            character: 5,
                        },
                        end: LocationData {
                            line: 2,
                            character: 7,
                        },
                    }
                ),
                Token::Text(
                    "Contact".to_string(),
                    Location {
                        start: LocationData {
                            line: 2,
                            character: 8,
                        },
                        end: LocationData {
                            line: 2,
                            character: 15,
                        },
                    }
                ),
                Token::CurlyBracketClose(Location {
                    start: LocationData {
                        line: 2,
                        character: 15,
                    },
                    end: LocationData {
                        line: 2,
                        character: 16,
                    },
                }),
                Token::NewLine(Location {
                    start: LocationData {
                        line: 2,
                        character: 16,
                    },
                    end: LocationData {
                        line: 2,
                        character: 17,
                    },
                }),
                Token::SquareBracketOpen(Location {
                    start: LocationData {
                        line: 3,
                        character: 0,
                    },
                    end: LocationData {
                        line: 3,
                        character: 1,
                    },
                }),
                Token::Module(
                    "std".to_string(),
                    Location {
                        start: LocationData {
                            line: 3,
                            character: 1,
                        },
                        end: LocationData {
                            line: 3,
                            character: 4,
                        },
                    }
                ),
                Token::Dot(Location {
                    start: LocationData {
                        line: 3,
                        character: 4,
                    },
                    end: LocationData {
                        line: 3,
                        character: 5,
                    },
                }),
                Token::Ident(
                    "@".to_string(),
                    Location {
                        start: LocationData {
                            line: 3,
                            character: 5,
                        },
                        end: LocationData {
                            line: 3,
                            character: 6,
                        },
                    }
                ),
                Token::Text(
                    "My website".to_string(),
                    Location {
                        start: LocationData {
                            line: 3,
                            character: 7,
                        },
                        end: LocationData {
                            line: 3,
                            character: 17,
                        },
                    }
                ),
                Token::Comma(Location {
                    start: LocationData {
                        line: 3,
                        character: 17,
                    },
                    end: LocationData {
                        line: 3,
                        character: 18,
                    },
                }),
                Token::Text(
                    "https://example.com/".to_string(),
                    Location {
                        start: LocationData {
                            line: 3,
                            character: 19,
                        },
                        end: LocationData {
                            line: 3,
                            character: 39,
                        },
                    }
                ),
                Token::SquareBracketClose(Location {
                    start: LocationData {
                        line: 3,
                        character: 39,
                    },
                    end: LocationData {
                        line: 3,
                        character: 40,
                    },
                }),
                Token::NewLine(Location {
                    start: LocationData {
                        line: 3,
                        character: 40,
                    },
                    end: LocationData {
                        line: 3,
                        character: 41,
                    },
                }),
                Token::NewLine(Location {
                    start: LocationData {
                        line: 4,
                        character: 0,
                    },
                    end: LocationData {
                        line: 4,
                        character: 1,
                    },
                }),
                Token::Text(
                    "2023.12.28".to_string(),
                    Location {
                        start: LocationData {
                            line: 5,
                            character: 0,
                        },
                        end: LocationData {
                            line: 5,
                            character: 10,
                        },
                    }
                ),
                Token::NewLine(Location {
                    start: LocationData {
                        line: 5,
                        character: 10,
                    },
                    end: LocationData {
                        line: 5,
                        character: 11,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 6,
                        character: 0,
                    },
                    end: LocationData {
                        line: 6,
                        character: 0,
                    },
                }),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_split_japanese_and_emoji() -> Result<()> {
        let pwd = std::env::current_dir()?;
        let uri = pwd
            .join("text/split_japanese_and_emoji.[]")
            .to_string_lossy()
            .to_string();
        let tokens = tokenize(uri.clone())?;
        assert_eq!(
            tokens,
            vec![
                Token::Text(
                    "こんにちは！🇯🇵".to_string(),
                    Location {
                        start: LocationData {
                            line: 0,
                            character: 0,
                        },
                        end: LocationData {
                            line: 0,
                            character: 7,
                        },
                    }
                ),
                Token::EOF(Location {
                    start: LocationData {
                        line: 0,
                        character: 7,
                    },
                    end: LocationData {
                        line: 0,
                        character: 7,
                    },
                }),
            ]
        );
        Ok(())
    }
}
