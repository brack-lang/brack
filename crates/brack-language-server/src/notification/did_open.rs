use crate::{logger::Logger, server::Server};
use anyhow::Result;
use brack_project::project::Project;
use lsp_types::DidOpenTextDocumentParams;
use std::path::Path;

impl Server {
    pub(crate) async fn handle_text_document_did_open(
        &mut self,
        params: DidOpenTextDocumentParams,
    ) -> Result<()> {
        let file_path_str = params.text_document.uri.as_str();
        let file_path = Path::new(file_path_str);

        // root/docs/file.[] -> root
        let root = file_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;

        let logger = Logger {};
        let project = Project::new_with_manifest(&logger, root)
            .map_err(|e| anyhow::anyhow!("Failed to create project: {:?}", e))?;
        self.project = Some(project);

        Ok(())
    }
}
