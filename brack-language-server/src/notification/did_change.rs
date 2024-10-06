use anyhow::Result;
use lsp_types::DidChangeTextDocumentParams;

use crate::server::Server;

impl Server {
    pub(crate) async fn handle_text_document_did_change(
        &self,
        params: DidChangeTextDocumentParams,
    ) -> Result<()> {
        let file_path = params
            .text_document
            .uri
            .to_file_path()
            .map_err(|_| anyhow::anyhow!("Invalid file path"))?;
        let uri = file_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;
        self.log_message(&format!("Did change: {}", uri)).await?;
        Ok(())
    }
}
