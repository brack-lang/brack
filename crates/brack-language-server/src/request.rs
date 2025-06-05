use anyhow::Result;
use lsp_types::{
    CompletionOptions, CompletionParams, InitializeParams, InitializeResult, SemanticTokenModifier,
    SemanticTokenType, SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions,
    SemanticTokensParams, SemanticTokensServerCapabilities, ServerCapabilities, ServerInfo,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};
use serde::Deserialize;
use serde_json::Value;

use crate::{result::BLSResult, server::Server};

pub mod completion;
pub mod semantic_tokens;

impl Server {
    pub(crate) async fn handle_request(
        &mut self,
        msg: &Value,
        id: i64,
        method: &str,
    ) -> Result<()> {
        match method {
            "initialize" => {
                self.log_message("Brack Language Server is initializing...")
                    .await?;

                let params = InitializeParams::deserialize(msg["params"].clone())?;
                self.client_capabilities = params.capabilities;

                let response = InitializeResult {
                    capabilities: ServerCapabilities {
                        text_document_sync: Some(TextDocumentSyncCapability::Kind(
                            TextDocumentSyncKind::FULL,
                        )),
                        completion_provider: Some(CompletionOptions {
                            resolve_provider: Some(false),
                            trigger_characters: Some(vec![
                                String::from("["),
                                String::from("{"),
                                String::from("<"),
                                String::from(" "),
                                String::from("."),
                            ]),
                            ..Default::default()
                        }),
                        semantic_tokens_provider: Some(
                            SemanticTokensServerCapabilities::SemanticTokensOptions(
                                SemanticTokensOptions {
                                    range: Some(false),
                                    full: Some(SemanticTokensFullOptions::Bool(true)),
                                    legend: SemanticTokensLegend {
                                        token_types: vec![
                                            SemanticTokenType::NAMESPACE,
                                            SemanticTokenType::TYPE,
                                            SemanticTokenType::CLASS,
                                            SemanticTokenType::ENUM,
                                            SemanticTokenType::INTERFACE,
                                            SemanticTokenType::STRUCT,
                                            SemanticTokenType::TYPE_PARAMETER,
                                            SemanticTokenType::PARAMETER,
                                            SemanticTokenType::VARIABLE,
                                            SemanticTokenType::PROPERTY,
                                            SemanticTokenType::ENUM_MEMBER,
                                            SemanticTokenType::EVENT,
                                            SemanticTokenType::FUNCTION,
                                            SemanticTokenType::METHOD,
                                            SemanticTokenType::MACRO,
                                            SemanticTokenType::KEYWORD,
                                            SemanticTokenType::MODIFIER,
                                            SemanticTokenType::COMMENT,
                                            SemanticTokenType::STRING,
                                            SemanticTokenType::NUMBER,
                                            SemanticTokenType::REGEXP,
                                            SemanticTokenType::OPERATOR,
                                            SemanticTokenType::DECORATOR,
                                        ],
                                        token_modifiers: vec![
                                            SemanticTokenModifier::DECLARATION,
                                            SemanticTokenModifier::DEFINITION,
                                            SemanticTokenModifier::READONLY,
                                            SemanticTokenModifier::STATIC,
                                            SemanticTokenModifier::ABSTRACT,
                                            SemanticTokenModifier::DEPRECATED,
                                            SemanticTokenModifier::ASYNC,
                                            SemanticTokenModifier::MODIFICATION,
                                            SemanticTokenModifier::DOCUMENTATION,
                                            SemanticTokenModifier::DEFAULT_LIBRARY,
                                        ],
                                    },
                                    ..Default::default()
                                },
                            ),
                        ),
                        ..Default::default()
                    },
                    server_info: Some(ServerInfo {
                        name: "Brack Language Server".to_string(),
                        version: Some("0.1.0".to_string()),
                    }),
                };

                let result = BLSResult::new(id, response);
                self.send_stdout(&result).await
            }
            "textDocument/semanticTokens/full" => {
                let params = SemanticTokensParams::deserialize(msg["params"].clone())?;
                match self.handle_semantic_tokens_full(params).await {
                    Ok(result) => match result {
                        Some(response) => {
                            let result = BLSResult::new(id, response);
                            self.send_stdout(&result).await?;
                            Ok(())
                        }
                        None => Ok(()),
                    },
                    Err(_) => {
                        self.send_error_response(Some(id), -32000, "semantic tokens failed")
                            .await
                    }
                }
            }
            "textDocument/completion" => {
                let params = CompletionParams::deserialize(msg["params"].clone())?;
                match self.handle_completion(params).await {
                    Ok(result) => match result {
                        Some(response) => {
                            let result = BLSResult::new(id, response);
                            self.send_stdout(&result).await?;
                            Ok(())
                        }
                        None => Ok(()),
                    },
                    Err(_) => {
                        self.send_error_response(Some(id), -32000, "completion failed")
                            .await
                    }
                }
            }
            _ => self.send_method_not_found_response(id, method).await,
        }
    }
}
