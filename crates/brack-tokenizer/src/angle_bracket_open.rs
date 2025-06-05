use crate::{dispatch::dispatch, tokenizer::Tokenizer, utils::separate};
use brack_common::location::{Location, LocationData};
use brack_common::tokens::Token;

pub fn tokenize(t: &Tokenizer) -> Vec<Token> {
    let s = t
        .untreated
        .clone()
        .unwrap_or_else(|| panic!("`Tokenizer.untreated` is not set"));
    let (_, tail) = separate(&s);

    let mut tokens = t
        .tokens
        .clone()
        .unwrap_or_else(|| panic!("`Tokenizer.tokens` is not set"));
    let line = t
        .line
        .unwrap_or_else(|| panic!("`Tokenizer.line` is not set"));
    let column = t
        .column
        .unwrap_or_else(|| panic!("`Tokenizer.column` is not set"));
    let angle_nest_count = t
        .angle_nest_count
        .unwrap_or_else(|| panic!("`Tokenizer.angle_nest_count` is not set"));

    tokens.push(Token::AngleBracketOpen(Location {
        start: LocationData {
            line,
            character: column,
        },
        end: LocationData {
            line,
            character: column + 1,
        },
    }));

    let t2 = Tokenizer {
        column: Some(column + 1),
        token_start_column: Some(column + 1),
        untreated: Some(tail),
        pool: Some(String::new()),
        tokens: Some(tokens),
        angle_nest_count: Some(angle_nest_count + 1),
        looking_for_identifier: Some(true),
        ..Default::default()
    };
    dispatch(&t.merge(&t2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenize::tokenize;
    use anyhow::Result;
    use brack_common::tokens::Token::{AngleBracketOpen, EOF};

    #[test]
    fn test_angle_bracket_open() -> Result<()> {
        let input = "<";
        let expected_output = vec![
            AngleBracketOpen(Location {
                start: LocationData {
                    line: 0,
                    character: 0,
                },
                end: LocationData {
                    line: 0,
                    character: 1,
                },
            }),
            EOF(Location {
                start: LocationData {
                    line: 0,
                    character: 1,
                },
                end: LocationData {
                    line: 0,
                    character: 1,
                },
            }),
        ];
        let actual_output = tokenize(input);
        assert_eq!(expected_output, actual_output);
        Ok(())
    }
}
