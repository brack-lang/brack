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
                            character: 15,
                        },
                        end: LocationData {
                            line: 1,
                            character: 21,
                        },
                    }
                ),
                Token::SquareBracketClose(Location {
                    start: LocationData {
                        line: 1,
                        character: 21,
                    },
                    end: LocationData {
                        line: 1,
                        character: 22,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 1,
                        character: 22,
                    },
                    end: LocationData {
                        line: 1,
                        character: 22,
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
                            line: 1,
                            character: 1,
                        },
                        end: LocationData {
                            line: 1,
                            character: 8,
                        },
                    }
                ),
                Token::CurlyBracketOpen(Location {
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
                            character: 15,
                        },
                        end: LocationData {
                            line: 1,
                            character: 21,
                        },
                    }
                ),
                Token::CurlyBracketClose(Location {
                    start: LocationData {
                        line: 1,
                        character: 21,
                    },
                    end: LocationData {
                        line: 1,
                        character: 22,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 1,
                        character: 22,
                    },
                    end: LocationData {
                        line: 1,
                        character: 22,
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
                            line: 1,
                            character: 1,
                        },
                        end: LocationData {
                            line: 1,
                            character: 8,
                        },
                    }
                ),
                Token::AngleBracketOpen(Location {
                    start: LocationData {
                        line: 1,
                        character: 8,
                    },
                    end: LocationData {
                        line: 1,
                        character: 9,
                    },
                }),
                Token::Ident(
                    "*".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 9,
                        },
                        end: LocationData {
                            line: 1,
                            character: 10,
                        },
                    }
                ),
                Token::Text(
                    "World!".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 11,
                        },
                        end: LocationData {
                            line: 1,
                            character: 17,
                        },
                    }
                ),
                Token::AngleBracketClose(Location {
                    start: LocationData {
                        line: 1,
                        character: 17,
                    },
                    end: LocationData {
                        line: 1,
                        character: 18,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 1,
                        character: 18,
                    },
                    end: LocationData {
                        line: 1,
                        character: 18,
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
                            line: 1,
                            character: 1,
                        },
                        end: LocationData {
                            line: 1,
                            character: 8,
                        },
                    }
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
                    "@".to_string(),
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
                            character: 15,
                        },
                        end: LocationData {
                            line: 1,
                            character: 21,
                        },
                    }
                ),
                Token::Comma(Location {
                    start: LocationData {
                        line: 1,
                        character: 21,
                    },
                    end: LocationData {
                        line: 1,
                        character: 22,
                    },
                }),
                Token::Text(
                    "https://example.com/".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 23,
                        },
                        end: LocationData {
                            line: 1,
                            character: 43,
                        },
                    }
                ),
                Token::SquareBracketClose(Location {
                    start: LocationData {
                        line: 1,
                        character: 43,
                    },
                    end: LocationData {
                        line: 1,
                        character: 44,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 1,
                        character: 44,
                    },
                    end: LocationData {
                        line: 1,
                        character: 44,
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
                            line: 1,
                            character: 1,
                        },
                        end: LocationData {
                            line: 1,
                            character: 8,
                        },
                    }
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
                Token::SquareBracketOpen(Location {
                    start: LocationData {
                        line: 1,
                        character: 15,
                    },
                    end: LocationData {
                        line: 1,
                        character: 16,
                    },
                }),
                Token::Module(
                    "std".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 16,
                        },
                        end: LocationData {
                            line: 1,
                            character: 19,
                        },
                    }
                ),
                Token::Dot(Location {
                    start: LocationData {
                        line: 1,
                        character: 19,
                    },
                    end: LocationData {
                        line: 1,
                        character: 20,
                    },
                }),
                Token::Ident(
                    "@".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 20,
                        },
                        end: LocationData {
                            line: 1,
                            character: 21,
                        },
                    }
                ),
                Token::Text(
                    "World!".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 22,
                        },
                        end: LocationData {
                            line: 1,
                            character: 28,
                        },
                    }
                ),
                Token::Comma(Location {
                    start: LocationData {
                        line: 1,
                        character: 28,
                    },
                    end: LocationData {
                        line: 1,
                        character: 29,
                    },
                }),
                Token::Text(
                    "https://example.com/".to_string(),
                    Location {
                        start: LocationData {
                            line: 1,
                            character: 30,
                        },
                        end: LocationData {
                            line: 1,
                            character: 50,
                        },
                    }
                ),
                Token::SquareBracketClose(Location {
                    start: LocationData {
                        line: 1,
                        character: 50,
                    },
                    end: LocationData {
                        line: 1,
                        character: 51,
                    },
                }),
                Token::SquareBracketClose(Location {
                    start: LocationData {
                        line: 1,
                        character: 51,
                    },
                    end: LocationData {
                        line: 1,
                        character: 52,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 1,
                        character: 52,
                    },
                    end: LocationData {
                        line: 1,
                        character: 52,
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
                            line: 1,
                            character: 1,
                        },
                        end: LocationData {
                            line: 1,
                            character: 7,
                        },
                    }
                ),
                Token::NewLine(Location {
                    start: LocationData {
                        line: 1,
                        character: 7,
                    },
                    end: LocationData {
                        line: 1,
                        character: 8,
                    },
                }),
                Token::Text(
                    "World,".to_string(),
                    Location {
                        start: LocationData {
                            line: 2,
                            character: 1,
                        },
                        end: LocationData {
                            line: 2,
                            character: 7,
                        },
                    }
                ),
                Token::NewLine(Location {
                    start: LocationData {
                        line: 2,
                        character: 7,
                    },
                    end: LocationData {
                        line: 2,
                        character: 8,
                    },
                }),
                Token::CurlyBracketOpen(Location {
                    start: LocationData {
                        line: 3,
                        character: 1,
                    },
                    end: LocationData {
                        line: 3,
                        character: 2,
                    },
                }),
                Token::Module(
                    "std".to_string(),
                    Location {
                        start: LocationData {
                            line: 3,
                            character: 2,
                        },
                        end: LocationData {
                            line: 3,
                            character: 5,
                        },
                    }
                ),
                Token::Dot(Location {
                    start: LocationData {
                        line: 3,
                        character: 5,
                    },
                    end: LocationData {
                        line: 3,
                        character: 6,
                    },
                }),
                Token::Ident(
                    "**".to_string(),
                    Location {
                        start: LocationData {
                            line: 3,
                            character: 6,
                        },
                        end: LocationData {
                            line: 3,
                            character: 8,
                        },
                    }
                ),
                Token::Text(
                    "Contact".to_string(),
                    Location {
                        start: LocationData {
                            line: 3,
                            character: 9,
                        },
                        end: LocationData {
                            line: 3,
                            character: 16,
                        },
                    }
                ),
                Token::CurlyBracketClose(Location {
                    start: LocationData {
                        line: 3,
                        character: 16,
                    },
                    end: LocationData {
                        line: 3,
                        character: 17,
                    },
                }),
                Token::NewLine(Location {
                    start: LocationData {
                        line: 3,
                        character: 17,
                    },
                    end: LocationData {
                        line: 3,
                        character: 18,
                    },
                }),
                Token::SquareBracketOpen(Location {
                    start: LocationData {
                        line: 4,
                        character: 1,
                    },
                    end: LocationData {
                        line: 4,
                        character: 2,
                    },
                }),
                Token::Module(
                    "std".to_string(),
                    Location {
                        start: LocationData {
                            line: 4,
                            character: 2,
                        },
                        end: LocationData {
                            line: 4,
                            character: 5,
                        },
                    }
                ),
                Token::Dot(Location {
                    start: LocationData {
                        line: 4,
                        character: 5,
                    },
                    end: LocationData {
                        line: 4,
                        character: 6,
                    },
                }),
                Token::Ident(
                    "@".to_string(),
                    Location {
                        start: LocationData {
                            line: 4,
                            character: 6,
                        },
                        end: LocationData {
                            line: 4,
                            character: 7,
                        },
                    }
                ),
                Token::Text(
                    "My website".to_string(),
                    Location {
                        start: LocationData {
                            line: 4,
                            character: 8,
                        },
                        end: LocationData {
                            line: 4,
                            character: 18,
                        },
                    }
                ),
                Token::Comma(Location {
                    start: LocationData {
                        line: 4,
                        character: 18,
                    },
                    end: LocationData {
                        line: 4,
                        character: 19,
                    },
                }),
                Token::Text(
                    "https://example.com/".to_string(),
                    Location {
                        start: LocationData {
                            line: 4,
                            character: 20,
                        },
                        end: LocationData {
                            line: 4,
                            character: 40,
                        },
                    }
                ),
                Token::SquareBracketClose(Location {
                    start: LocationData {
                        line: 4,
                        character: 40,
                    },
                    end: LocationData {
                        line: 4,
                        character: 41,
                    },
                }),
                Token::NewLine(Location {
                    start: LocationData {
                        line: 4,
                        character: 41,
                    },
                    end: LocationData {
                        line: 4,
                        character: 42,
                    },
                }),
                Token::NewLine(Location {
                    start: LocationData {
                        line: 5,
                        character: 1,
                    },
                    end: LocationData {
                        line: 5,
                        character: 2,
                    },
                }),
                Token::Text(
                    "2023.12.28".to_string(),
                    Location {
                        start: LocationData {
                            line: 6,
                            character: 1,
                        },
                        end: LocationData {
                            line: 6,
                            character: 11,
                        },
                    }
                ),
                Token::NewLine(Location {
                    start: LocationData {
                        line: 6,
                        character: 11,
                    },
                    end: LocationData {
                        line: 6,
                        character: 12,
                    },
                }),
                Token::EOF(Location {
                    start: LocationData {
                        line: 7,
                        character: 1,
                    },
                    end: LocationData {
                        line: 7,
                        character: 1,
                    },
                }),
            ]
        );
        Ok(())
    }
}
