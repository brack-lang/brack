use std::collections::HashMap;

use anyhow::Result;
use brack_plugin::{feature_flag::FeatureFlag, plugin::Plugin, plugins::Plugins};
use regex::Regex;
use std::path::Path;

pub fn compile<P1: AsRef<Path>, P2: AsRef<Path>>(path: P1, plugins_dir_path: P2) -> Result<String> {
    let path = path.as_ref();
    let plugins_dir_path = plugins_dir_path.as_ref();
    let pattern = Regex::new(r"(?<module_name>[[:alpha:]]+)_[[:alnum:]]+.wasm").unwrap();
    let entries = std::fs::read_dir(plugins_dir_path)?;
    let mut paths = HashMap::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let capture = pattern.captures(path.to_str().ok_or_else(|| {
            anyhow::anyhow!("Could not convert file name to string: {}", path.display())
        })?);
        if let Some(capture) = capture {
            let module_name = capture.name("module_name").unwrap().as_str();
            paths.insert(module_name.to_string(), (path, FeatureFlag::default()));
        }
    }

    let mut plugin_vec = vec![];
    for (name, (path, feature_flag)) in paths {
        let plugin = Plugin::new(&name, path, feature_flag)?;
        plugin_vec.push(plugin);
    }
    let mut plugins = Plugins::new(plugin_vec)?;

    let tokens = brack_tokenizer::tokenize::tokenize(path)?;
    let cst = brack_parser::parse::parse(&tokens)?;
    let (ast, _errors) = brack_transformer::transform::transform(&cst);
    let expanded_ast = brack_expander::expand::expander(&ast, &mut plugins)?;
    let gen = brack_codegen::generate::generate(&expanded_ast, &mut plugins)?;
    Ok(gen)
}
