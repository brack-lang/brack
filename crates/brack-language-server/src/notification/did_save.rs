use anyhow::Result;
use brack_parser::parse::parse;
use brack_tokenizer::tokenize::tokenize;
use brack_transformer::transform::transform;
use lsp_types::{Diagnostic, DidSaveTextDocumentParams};
use std::fs::read_to_string;

use crate::{server::Server, utils::to_url};

impl Server {
    pub(crate) async fn handle_text_document_did_save(
        &self,
        param: DidSaveTextDocumentParams,
    ) -> Result<()> {
        let path = to_url(param.text_document.uri)?
            .to_file_path()
            .map_err(|e| anyhow::anyhow!("Failed to convert URI to file path: {:?}", e))?;
        let path_str = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;

        let file = read_to_string(path_str)?;
        let tokens = tokenize(&file);
        let cst = parse(&tokens);
        let (_, errors) = transform(&cst);

        if errors.is_empty() {
            let diagnostics: Vec<Diagnostic> = vec![];
            return self.send_publish_diagnostics(path_str, &diagnostics).await;
        }

        let mut diagnostics = vec![];
        for error in errors {
            let location = error.get_location();
            let message = error.get_message();
            let diagnostic = Diagnostic {
                range: lsp_types::Range {
                    start: lsp_types::Position {
                        line: location.start.line as u32,
                        character: location.start.character as u32,
                    },
                    end: lsp_types::Position {
                        line: location.end.line as u32,
                        character: location.end.character as u32,
                    },
                },
                message,
                ..Default::default()
            };
            diagnostics.push(diagnostic);
        }
        self.send_publish_diagnostics(path_str, &diagnostics).await
    }
}
