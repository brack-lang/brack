use std::collections::HashMap;

use anyhow::Result;
use brack::{new_project::new_project, sub_commands::SubCommands};
use brack_plugin::{feature_flag::FeatureFlag, plugin::Plugin, plugins::Plugins};
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

pub fn run_compile(subcommand: SubCommands) -> Result<()> {
    let mut pathes = HashMap::new();

    let (plugins_dir_path, _, filename, output_level, json) = match subcommand {
        SubCommands::Compile {
            plugins_dir_path,
            backend,
            filename,
            output_level,
            json,
        } => (plugins_dir_path, backend, filename, output_level, json),
        _ => unreachable!(),
    };

    let plugins_dir_path = match plugins_dir_path {
        Some(path) => path,
        None => std::env::var("BRACK_PLUGINS_PATH").unwrap_or_default(),
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
            pathes.insert(module_name.to_string(), (path, FeatureFlag::default()));
        }
    }

    let mut plugin_vec = vec![];
    for (name, (path, feature_flag)) in pathes {
        let plugin = Plugin::new(&name, path, feature_flag)?;
        plugin_vec.push(plugin);
    }
    let mut plugins = Plugins::new(plugin_vec)?;

    if !filename.ends_with(".[]") {
        anyhow::bail!("Filename must end with .[]");
    }

    match output_level {
        1 => {
            let tokens = brack_tokenizer::tokenize::tokenize(&filename)?;
            if json {
                let json = serde_json::to_string(&tokens)?;
                println!("{}", json);
            } else {
                for token in tokens {
                    println!("{:?}", token);
                }
            }
        }
        2 => {
            let tokens = brack_tokenizer::tokenize::tokenize(&filename)?;
            let cst = brack_parser::parse::parse(&tokens)?;
            if json {
                let json = serde_json::to_string(&cst)?;
                println!("{}", json);
            } else {
                println!("{:?}", cst);
            }
        }
        3 => {
            let tokens = brack_tokenizer::tokenize::tokenize(&filename)?;
            let cst = brack_parser::parse::parse(&tokens)?;
            let (ast, _errors) = brack_transformer::transform::transform(&cst);
            if json {
                let json = serde_json::to_string(&ast)?;
                println!("{}", json);
            } else if !_errors.is_empty() {
                for error in _errors {
                    println!("{:?}", error);
                }
            } else {
                println!("{:?}", ast);
            }
        }
        4 => {
            let tokens = brack_tokenizer::tokenize::tokenize(&filename)?;
            let cst = brack_parser::parse::parse(&tokens)?;
            let (ast, _errors) = brack_transformer::transform::transform(&cst);
            let expanded_ast = brack_expander::expand::expander(&ast, &mut plugins)?;
            if json {
                let json = serde_json::to_string(&expanded_ast)?;
                println!("{}", json);
            } else {
                println!("{:?}", expanded_ast);
            }
        }
        5 => {
            if json {
                anyhow::bail!("Cannot output JSON at output level 5.")
            }
            let tokens = brack_tokenizer::tokenize::tokenize(&filename)?;
            let cst = brack_parser::parse::parse(&tokens)?;
            let (ast, _errors) = brack_transformer::transform::transform(&cst);
            let expanded_ast = brack_expander::expand::expander(&ast, &mut plugins)?;
            let gen = brack_codegen::generate::generate(&expanded_ast, &mut plugins)?;
            println!("{}", gen);
        }
        _ => anyhow::bail!("Invalid output level."),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.subcommand {
        SubCommands::Build => {
            let mut project = brack_project_manager::project::Project::new(".");
            project.load_brack_toml()?;
            project.download_plugins_using_config().await?;
            project.build()?;
        }
        SubCommands::Compile { .. } => run_compile(args.subcommand)?,
        SubCommands::LanguageServer => {
            let mut language_server = brack_language_server::server::Server::default();
            language_server.run().await?;
        }
        SubCommands::New { name } => new_project(&name)?,
        SubCommands::Add { schema } => brack_project_manager::plugin::add_plugin(&schema).await?,
        SubCommands::Version => {
            let version = match std::env::var("APP_VERSION") {
                Ok(version) => version,
                // FIXME: The following hard-coded version for Nix packaging now.
                //       We should remove this when we have a better way to handle it.
                Err(_) => "0.2.0".to_string(),
            };
            println!("Brack {}", version);
        }
    }
    Ok(())
}
