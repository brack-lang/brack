use std::str::from_utf8;

use anyhow::Result;
use brack_project_manager::project::Project;
use lsp_types::{ClientCapabilities, Diagnostic};
use serde::Serialize;
use serde_json::{from_str, json, Value};
use tokio::io::{stdin, stdout, AsyncReadExt, AsyncWriteExt};

#[derive(Default)]
pub struct Server {
    pub(crate) client_capabilities: ClientCapabilities,
    pub(crate) project: Option<Project>,
}

impl Server {
    pub(crate) async fn send_stdout<T: ?Sized + Serialize>(&self, message: &T) -> Result<()> {
        let msg = serde_json::to_string(message)?;
        let mut output = stdout();
        output
            .write_all(format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg).as_bytes())
            .await?;
        output.flush().await?;
        Ok(())
    }

    pub(crate) async fn send_message(&self, msg: &str) -> Result<()> {
        let mut output = stdout();
        output
            .write_all(format!("Content-Length: {}\r\n\r\n{}", msg.len(), msg).as_bytes())
            .await?;
        output.flush().await?;
        Ok(())
    }

    pub(crate) async fn log_message(&self, message: &str) -> Result<()> {
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

    pub(crate) async fn send_error_response(
        &self,
        id: Option<i64>,
        code: i32,
        message: &str,
    ) -> Result<()> {
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

    pub(crate) async fn send_invalid_request_response(&self) -> Result<()> {
        self.send_error_response(None, -32600, "received an invalid request")
            .await
    }

    pub(crate) async fn send_method_not_found_response(&self, id: i64, method: &str) -> Result<()> {
        self.send_error_response(Some(id), -32601, &format!("{} is not supported", method))
            .await
    }

    #[allow(dead_code)]
    pub(crate) async fn send_parse_error_response(&self) -> Result<()> {
        self.send_error_response(None, -32700, "received an invalid JSON")
            .await
    }

    pub(crate) async fn send_publish_diagnostics(
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

    pub(crate) async fn dispatch(&mut self, msg: Value) -> Result<()> {
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
