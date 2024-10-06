use crate::server::Server;
use anyhow::Result;
use brack_project_manager::project::Project;
use lsp_types::DidOpenTextDocumentParams;

impl Server {
    pub(crate) async fn handle_text_document_did_open(
        &mut self,
        params: DidOpenTextDocumentParams,
    ) -> Result<()> {
        let file_path = params
            .text_document
            .uri
            .to_file_path()
            .map_err(|_| anyhow::anyhow!("Invalid file path"))?;
        let uri = file_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;
        
        // root/docs/file.[] -> root
        let root = file_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;

        let mut project = Project::new(root);
        if let Ok(_) = project.load_brack_toml() {
            project.download_plugins_using_config().await?;
            self.project = Some(project);
        }

        self.log_message(&format!("Did open: {}", uri)).await?;
        Ok(())
    }
}
