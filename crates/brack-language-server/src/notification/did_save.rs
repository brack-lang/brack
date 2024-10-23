use anyhow::Result;
use brack_parser::parse::parse;
use brack_tokenizer::tokenize::tokenize;
use brack_transformer::transform::transform;
use lsp_types::{Diagnostic, DidSaveTextDocumentParams};

use crate::server::Server;

impl Server {
    pub(crate) async fn handle_text_document_did_save(
        &self,
        param: DidSaveTextDocumentParams,
    ) -> Result<()> {
        let file_path = param
            .text_document
            .uri
            .to_file_path()
            .map_err(|_| anyhow::anyhow!("Invalid file path"))?;
        let uri = file_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;

        let tokens = match tokenize(uri) {
            Ok(tokens) => tokens,
            Err(e) => return self.log_message(&format!("Tokenize failed: {}", e)).await,
        };
        let cst = parse(&tokens)?;
        let (_, errors) = transform(&cst);

        if errors.is_empty() {
            let diagnostics: Vec<Diagnostic> = vec![];
            return self.send_publish_diagnostics(uri, &diagnostics).await;
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
        self.send_publish_diagnostics(uri, &diagnostics).await
    }
}
