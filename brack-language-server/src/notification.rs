use crate::server::Server;
use anyhow::Result;
use lsp_types::{
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
};
use serde::Deserialize;
use serde_json::Value;

pub mod did_change;
pub mod did_open;
pub mod did_save;
pub mod initialized;

impl Server {
    pub(crate) async fn handle_notification(&mut self, msg: &Value, method: &str) -> Result<()> {
        match method {
            "initialized" => {
                self.log_message("Brack Language Server has been initialized!")
                    .await?;
                Ok(())
            }
            "textDocument/didOpen" => {
                let params = DidOpenTextDocumentParams::deserialize(msg["params"].clone())?;
                self.handle_text_document_did_open(params).await
            }
            "textDocument/didChange" => {
                let params = DidChangeTextDocumentParams::deserialize(msg["params"].clone())?;
                self.handle_text_document_did_change(params).await
            }
            "textDocument/didSave" => {
                let params = DidSaveTextDocumentParams::deserialize(msg["params"].clone())?;
                self.handle_text_document_did_save(params).await
            }
            _ => Ok(()),
        }
    }
}
