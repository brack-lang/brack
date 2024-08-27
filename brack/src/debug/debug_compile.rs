use anyhow::Result;
use brack::sub_commands::SubCommands;
use regex::Regex;
use std::collections::HashMap;
use tokio;

pub fn run_compile(subcommand: SubCommands) -> Result<()> {
    let mut pathes = HashMap::new();

    let args = match subcommand {
        SubCommands::Compile {
            plugins_dir_path,
            backend,
            filename,
        } => (plugins_dir_path, backend, filename),
        _ => unreachable!(),
    };

    let plugins_dir_path = match args.0 {
        Some(path) => path,
        None => match std::env::var("BRACK_PLUGINS_PATH") {
            Ok(path) => path,
            Err(_) => String::new(),
        },
    };

    let pattern = Regex::new(r"(?<module_name>[[:alpha:]]+)_[[:alnum:]]+.wasm").unwrap();
    let entries = std::fs::read_dir(plugins_dir_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let capture = pattern.captures(
            path.to_str()
                .ok_or_else(|| anyhow::anyhow!("Could not convert file name to string."))?,
        );
        if let Some(capture) = capture {
            let module_name = capture.name("module_name").unwrap().as_str();
            pathes.insert(module_name.to_string(), path);
        }
    }

    let mut plugins = brack_plugin::plugin::new_plugins(pathes)?;

    if !args.2.ends_with(".[]") {
        anyhow::bail!("Filename must end with .[]");
    }

    let tokenized = brack_tokenizer::tokenize::tokenize(&args.2)?;
    let parsed = brack_parser::parse::parse(&tokenized)?;
    let expanded = brack_expander::expand::expander(&parsed, &mut plugins)?;
    let gen = brack_codegen::generate::generate(&expanded, &mut plugins)?;
    println!("{}", gen);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = SubCommands::Compile {
        plugins_dir_path: Some("plugins".to_string()),
        backend: "html".to_string(),
        filename: "docs/main.[]".to_string(),
    };
    run_compile(args)?;

    Ok(())
}
