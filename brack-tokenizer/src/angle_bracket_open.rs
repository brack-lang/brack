use crate::{
    dispatch::dispatch,
    tokenizer::Tokenizer,
    tokens::{Location, LocationData, Token},
    utils::separate,
};
use anyhow::Result;

pub fn tokenize(t: &Tokenizer) -> Result<Vec<Token>> {
    let s = t
        .untreated
        .clone()
        .ok_or_else(|| anyhow::anyhow!("`t.untreated` is not set"))?;
    let (_, tail) = separate(&s);

    let mut tokens = t
        .tokens
        .clone()
        .ok_or_else(|| anyhow::anyhow!("`t.tokens` is not set"))?;
    let line = t
        .line
        .ok_or_else(|| anyhow::anyhow!("`t.line` is not set"))?;
    let column = t
        .column
        .ok_or_else(|| anyhow::anyhow!("`t.column` is not set"))?;
    let angle_nest_count = t
        .angle_nest_count
        .ok_or_else(|| anyhow::anyhow!("`t.angle_nest_count` is not set"))?;

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
    use crate::{tokenize::tokenize_str, tokens::Token::*};

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
        let actual_output = tokenize_str(input)?;
        assert_eq!(expected_output, actual_output);
        Ok(())
    }
}
