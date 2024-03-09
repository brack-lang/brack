use std::str::from_utf8;

use anyhow::{anyhow, Result};
use lsp_types::{ClientCapabilities, Diagnostic, Position, Range};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, Value};
use tokio::io::{stdin, stdout, AsyncReadExt, AsyncWriteExt};

pub struct LanguageServer {
    client_capabilities: ClientCapabilities,
}

impl LanguageServer {
    pub fn new() -> Self {
        Self {
            client_capabilities: ClientCapabilities::default(),
        }
    }

    async fn send_message(&self, msg: &str) -> Result<()> {
        let mut output = stdout();
        output
            .write_all(format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg).as_bytes())
            .await?;
        output.flush().await?;
        Ok(())
    }

    async fn log_message(&self, message: &str) -> Result<()> {
        let response = json!({
            "jsonrpc": "2.0",
            "method": "window/logMessage",
            "params": {
                "type": 3,
                "message": message
            }
        })
        .to_string();
        self.send_message(&response).await
    }

    async fn send_error_response(&self, id: Option<i64>, code: i32, message: &str) -> Result<()> {
        let response = json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": code,
                "message": message,
            }
        })
        .to_string();
        self.send_message(&response).await
    }

    async fn send_invalid_request_response(&self) -> Result<()> {
        self.send_error_response(None, -32600, "received an invalid request")
            .await
    }

    async fn send_method_not_found_response(&self, id: i64, method: &str) -> Result<()> {
        self.send_error_response(Some(id), -32601, &format!("{} is not supported", method))
            .await
    }

    #[allow(dead_code)]
    async fn send_parse_error_response(&self) -> Result<()> {
        self.send_error_response(None, -32700, "received an invalid JSON")
            .await
    }

    async fn send_publish_diagnostics(
        &self,
        uri: &str,
        diagnostics: &Vec<Diagnostic>,
    ) -> Result<()> {
        // check client_capabilities.text_document.publish_diagnostics
        if self
            .client_capabilities
            .text_document
            .as_ref()
            .and_then(|td| td.publish_diagnostics.as_ref())
            .is_none()
        {
            return Ok(());
        }

        let response = json!({
            "jsonrpc": "2.0",
            "method": "textDocument/publishDiagnostics",
            "params": {
                "uri": uri,
                "diagnostics": json!(diagnostics),
            }
        })
        .to_string();
        self.send_message(&response).await
    }

    async fn handle_request(&mut self, msg: &Value, id: i64, method: &str) -> Result<()> {
        match method {
            "initialize" => {
                self.log_message("Brack Language Server is initializing...")
                    .await?;
                self.client_capabilities = serde_json::from_value(
                    msg.get("params")
                        .ok_or_else(|| anyhow!("No params"))?
                        .get("capabilities")
                        .ok_or_else(|| anyhow!("No capabilities"))?
                        .clone(),
                )?;

                // for debugging
                // self.log_message(&format!(
                //     "Client capabilities : {:?}",
                //     self.client_capabilities
                // ))
                // .await?;

                let response = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "capabilities": {
                        "textDocumentSync": 1,
                    } }
                })
                .to_string();
                self.send_message(&response).await
            }
            _ => self.send_method_not_found_response(id, method).await,
        }
    }

    async fn handle_response(&self, _: &Value, _: i64) -> Result<()> {
        Ok(())
    }

    async fn handle_notification_text_document_did_open(&self, msg: &Value) -> Result<()> {
        let text_document = msg
            .get("params")
            .ok_or_else(|| anyhow!("No params"))?
            .get("textDocument")
            .ok_or_else(|| anyhow!("No textDocument"))?;
        let uri = text_document
            .get("uri")
            .and_then(|uri| uri.as_str())
            .ok_or_else(|| anyhow!("No uri"))?;
        let _ = text_document
            .get("text")
            .and_then(|text| text.as_str())
            .ok_or_else(|| anyhow!("No text"))?;
        self.log_message(&format!("Did open {}", uri)).await?;

        Ok(())
    }

    async fn handle_notification_text_document_did_change(&self, msg: &Value) -> Result<()> {
        let uri = msg
            .get("params")
            .ok_or_else(|| anyhow!("No params"))?
            .get("textDocument")
            .ok_or_else(|| anyhow!("No textDocument"))?
            .get("uri")
            .and_then(|uri| uri.as_str())
            .ok_or_else(|| anyhow!("No uri"))?;
        let index = msg
            .get("params")
            .ok_or_else(|| anyhow!("No params"))?
            .get("contentChanges")
            .and_then(|content_changes| content_changes.as_array())
            .and_then(|content_changes| content_changes.len().checked_sub(1))
            .ok_or_else(|| anyhow!("No contentChanges"))?;
        let _ = msg
            .get("params")
            .ok_or_else(|| anyhow!("No params"))?
            .get("contentChanges")
            .and_then(|content_changes| content_changes.get(index))
            .and_then(|content_change| content_change.get("text"))
            .and_then(|text| text.as_str())
            .ok_or_else(|| anyhow!("No text"))?;
        self.log_message(&format!("Did change {}", uri)).await?;

        let diagnostics = vec![Diagnostic {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 3,
                },
            },
            message: "This is a test diagnostic message 2!".to_string(),
            ..Default::default()
        }];
        self.log_message(&format!("{:?}", diagnostics)).await?;
        self.send_publish_diagnostics(uri, &diagnostics).await
    }

    async fn handle_notification(&mut self, msg: &Value, method: &str) -> Result<()> {
        match method {
            "initialized" => {
                self.log_message("Brack Language Server has been initialized!")
                    .await?;
                Ok(())
            }
            "textDocument/didOpen" => self.handle_notification_text_document_did_open(msg).await,
            "textDocument/didChange" => {
                self.handle_notification_text_document_did_change(msg).await
            }
            _ => Ok(()),
        }
    }

    async fn dispatch(&mut self, msg: Value) -> Result<()> {
        match (
            msg.get("id").and_then(|i| i.as_i64()),
            msg.get("method").and_then(|m| m.as_str()),
        ) {
            (Some(id), Some(method)) => self.handle_request(&msg, id, method).await,
            (Some(id), None) => self.handle_response(&msg, id).await,
            (None, Some(method)) => self.handle_notification(&msg, method).await,
            _ => self.send_invalid_request_response().await,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut stdin = stdin();
        let mut buffer = Vec::new();

        loop {
            let mut tmp_buffer = [0; 1024];

            let chunk = stdin.read(&mut tmp_buffer).await?;

            if chunk == 0 {
                break;
            }
            buffer.extend_from_slice(&tmp_buffer[..chunk]);

            let buffer_string = from_utf8(&buffer)?;
            if !buffer_string.contains("\r\n\r\n") {
                continue;
            }

            let splitted_buffer = buffer_string.split("\r\n\r\n").collect::<Vec<&str>>();
            let header_string = splitted_buffer[0];

            let mut content_length = -1;
            let header_length = header_string.len() + 4;
            for line in header_string.split("\r\n") {
                let splitted_line = line.split(": ").collect::<Vec<&str>>();
                let key = splitted_line[0];
                let value = splitted_line[1];
                if key == "Content-Length" {
                    content_length = value.parse::<i32>()?;
                }
            }

            if content_length == -1 {
                continue;
            }
            let total_length = header_length + content_length as usize;

            if buffer.len() < total_length {
                continue;
            }

            let msg: Value = from_str(&buffer_string[header_length..total_length])?;
            self.dispatch(msg).await?;
            buffer.drain(0..total_length);
        }

        Ok(())
    }
}
