use anyhow::Result;
use lsp_types::{CompletionParams, InitializeParams};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{result::BLSResult, server::Server};

pub mod completion;

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

                let parms: InitializeParams =
                    serde_json::from_value(msg.get("params").unwrap().clone())?;
                self.client_capabilities = parms.capabilities;

                let response = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "capabilities": {
                        "textDocumentSync": 1,
                        "completionProvider": {
                            "triggerCharacters": ["[", "{", "<", " ", "."],
                            "resolveProvider": false,
                        }
                    } }
                })
                .to_string();

                self.send_message(&response).await
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
