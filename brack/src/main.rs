use std::collections::HashMap;

use anyhow::Result;
use brack::sub_commands::SubCommands;
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

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

    let tokens = brack_tokenizer::tokenize::tokenize(&args.2)?;
    let cst = brack_parser::parse::parse(&tokens)?;
    let (ast, _errors) = brack_transformer::transform::transform(&cst);
    let expanded_ast = brack_expander::expand::expander(&ast, &mut plugins)?;
    let gen = brack_codegen::generate::generate(&expanded_ast, &mut plugins)?;
    println!("{}", gen);
    Ok(())
}

fn new_project(name: &str) -> Result<()> {
    std::fs::create_dir(name)?;
    std::fs::create_dir(format!("{}/docs", name))?;
    std::fs::write(format!("{}/docs/main.[]", name), "")?;
    std::fs::write(
        format!("{}/Brack.toml", name),
        format!(
            r#"[document]
name = "{}"
version = "0.1.0"
backend = ""
authors = ["your name <your email>"]

[plugins]
"#,
            name
        ),
    )?;
    std::fs::write(
        format!("{}/.gitignore", name),
        r#"plugins
out
"#,
    )?;
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
            let mut language_server = brack_language_server::server::Server::new();
            language_server.run().await?;
        }
        SubCommands::New { name } => new_project(&name)?,
        SubCommands::Add { schema } => brack_project_manager::plugin::add_plugin(&schema).await?,
        SubCommands::Version => {
            let version = match std::env::var("APP_VERSION") {
                Ok(version) => version,
                // FIXME: The following hard-coded version for Nix packaging now.
                //       We should remove this when we have a better way to handle it.
                Err(_) => "0.1.0".to_string(),
            };
            println!("Brack {}", version);
        }
    }
    Ok(())
}
