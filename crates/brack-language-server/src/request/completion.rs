use anyhow::Result;
use brack_plugin::{metadata::Metadata, plugin::Plugin, plugins::Plugins, types::Type};
use lsp_types::{CompletionItem, CompletionParams, CompletionResponse, InsertTextFormat};

use crate::server::Server;

fn build_completion_item(
    module_name: &str,
    name: &str,
    typ: &Type,
    command_metadata: &Metadata,
) -> CompletionItem {
    let insert_text = Some(match typ {
        Type::TInline => format!("{}.{} $0]", module_name, name),
        Type::TBlock => format!("{}.{} $0}}", module_name, name),
        Type::TAST => format!("{}.{} $0>", module_name, name),
        _ => panic!("Invalid type"),
    });
    CompletionItem {
        label: format!("{}.{}", module_name, name),
        detail: Some(format!(
            "Argument Types: {:?}\nReturn Type: {:?}",
            command_metadata.argument_types, command_metadata.return_type
        )),
        insert_text,
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..CompletionItem::default()
    }
}

impl Server {
    pub(crate) async fn handle_completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>> {
        if self.project.is_none() {
            // BLS doesn't support single-file mode now.
            return Ok(None);
        }
        let project = self.project.as_ref().unwrap();
        let mut completion_items = vec![];
        let mut plugin_vec = vec![];
        for (name, (path, feature_flag)) in &project.plugins_metadata {
            let plugin = Plugin::new(name, path, feature_flag.clone())?;
            plugin_vec.push(plugin);
        }
        let plugins = Plugins::new(plugin_vec)?;
        let start = params
            .context
            .ok_or_else(|| anyhow::anyhow!("No context"))?
            .trigger_character;
        if start.is_none() {
            return Ok(None);
        }
        let start = start.unwrap();
        for plugin in plugins.name_to_plugin.values() {
            for ((name, typ), command_metadata) in plugin.signature_to_metadata.iter() {
                if start == String::from("[") && matches!(typ, Type::TInline) {
                    completion_items.push(build_completion_item(
                        &plugin.name,
                        &name,
                        &typ,
                        &command_metadata,
                    ));
                } else if start == String::from("{") && matches!(typ, Type::TBlock) {
                    completion_items.push(build_completion_item(
                        &plugin.name,
                        &name,
                        &typ,
                        &command_metadata,
                    ));
                }
            }
        }
        Ok(Some(CompletionResponse::Array(completion_items)))
    }
}
