use std::fs::read_dir;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[clap(arg_required_else_help = true)]
    Compile {
        #[clap(short, long)]
        plugins_dir_path: Option<String>,

        #[clap(short, long)]
        backend: String,

        #[clap(short, long)]
        filename: String,
    },
    LanguageServer,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

fn run_compile(subcommand: SubCommands) -> Result<()> {
    let mut pathes = vec![];

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

    let entries = read_dir(plugins_dir_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!(""))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!(""))?;
        if name.ends_with(format!(".{}.wasm", args.1).as_str()) {
            pathes.push(path);
        }
    }

    let mut plugins = brack_plugin::plugin::new_plugins(pathes)?;

    if !args.2.ends_with(".[]") {
        anyhow::bail!("Filename must end with .[]");
    }

    let code = std::fs::read_to_string(args.2)?;
    let tokenized = brack_tokenizer::tokenize::tokenize(&code);
    let parsed = brack_parser::parse::parse(&tokenized)?;
    let expanded = brack_expander::expand::expander(&parsed, &mut plugins)?;
    let gen = brack_codegen::generate::generate(&expanded, &mut plugins)?;
    println!("{}", gen);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.subcommand {
        SubCommands::Compile { .. } => run_compile(args.subcommand)?,
        SubCommands::LanguageServer => brack_language_server::server::run().await?,
    }
    Ok(())
}