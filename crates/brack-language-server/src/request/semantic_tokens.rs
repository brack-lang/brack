use anyhow::Result;
use brack_tokenizer::{tokenize::tokenize, tokens::Token};
use lsp_types::{SemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensParams};

use crate::server::Server;

fn token_kind_to_type(token: &Token) -> u32 {
    let typ = match token {
        Token::Module(_, _) => SemanticTokenType::NAMESPACE,
        Token::Ident(_, _) => SemanticTokenType::VARIABLE,
        Token::AngleBracketOpen(_) | Token::AngleBracketClose(_) => SemanticTokenType::MACRO,
        Token::CurlyBracketOpen(_) | Token::CurlyBracketClose(_) => SemanticTokenType::METHOD,
        Token::SquareBracketOpen(_) | Token::SquareBracketClose(_) => SemanticTokenType::FUNCTION,
        _ => return 100 // no decoration
    };
    token_type_as_u32(typ)
}

fn token_type_as_u32(token_type: SemanticTokenType) -> u32 {
    match token_type.as_str() {
        "namespace" => 0,
        "type" => 1,
        "class" => 2,
        "enum" => 3,
        "interface" => 4,
        "struct" => 5,
        "typeParameter" => 6,
        "parameter" => 7,
        "variable" => 8,
        "property" => 9,
        "enumMember" => 10,
        "event" => 11,
        "function" => 12,
        "method" => 13,
        "macro" => 14,
        "keyword" => 15,
        "modifier" => 16,
        "comment" => 17,
        "string" => 18,
        "number" => 19,
        "regexp" => 20,
        "operator" => 21,
        _ => 8,
    }
}

fn separate(tokens: &Vec<Token>) -> Vec<SemanticToken> {
    let mut semantic_tokens = Vec::new();
    let mut prev_line = 0;
    let mut prev_char = 0;

    for token in tokens {
        let location = token.get_location();
        let delta_line = location.start.line as u32 - prev_line;
        let delta_start = if delta_line == 0 {
            location.start.character as u32 - prev_char
        } else {
            location.start.character as u32
        };

        semantic_tokens.push(SemanticToken {
            delta_line,
            delta_start,
            length: (location.end.character - location.start.character) as u32,
            token_type: token_kind_to_type(&token),
            token_modifiers_bitset: 0,
        });

        prev_line = location.start.line as u32;
        prev_char = location.start.character as u32;
    }

    semantic_tokens
}

impl Server {
    pub(crate) async fn handle_semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokens>> {
        let file_path = params.text_document.uri.to_file_path().map_err(|_| {
            anyhow::anyhow!(
                "Failed to convert URI to file path: {:?}",
                params.text_document.uri
            )
        })?;
        let uri = file_path.to_str().ok_or_else(|| {
            anyhow::anyhow!("Failed to convert file path to string: {:?}", file_path)
        })?;

        let tokens = match tokenize(uri) {
            Ok(tokens) => tokens,
            Err(e) => {
                self.log_message(&format!("Failed to tokenize file: {:?}", e))
                    .await?;
                return Ok(None);
            }
        };

        let separated = separate(&tokens);
        Ok(Some(SemanticTokens {
            result_id: None,
            data: separated,
        }))
    }
}
