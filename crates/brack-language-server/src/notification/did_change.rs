use anyhow::Result;
use lsp_types::DidChangeTextDocumentParams;

use crate::server::Server;

impl Server {
    pub(crate) async fn handle_text_document_did_change(
        &self,
        _params: DidChangeTextDocumentParams,
    ) -> Result<()> {
        Ok(())
    }
}
