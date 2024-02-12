use std::{fs::read_dir, path::Path};

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
    Build,
    LanguageServer,
    New {
        #[clap(short, long)]
        name: String,
    },
    Add {
        schema: String,
    },
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
            .ok_or_else(|| anyhow::anyhow!("Could not get file name from path."))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Could not convert file name to string."))?;
        if name.ends_with(format!(".{}.wasm", args.1).as_str()) {
            pathes.push(path);
        }
    }

    let mut plugins = brack_plugin::plugin::new_plugins2(pathes)?;

    if !args.2.ends_with(".[]") {
        anyhow::bail!("Filename must end with .[]");
    }

    let tokenized = brack_tokenizer::tokenize::tokenize(args.2)?;
    let parsed = brack_parser::parse::parse(&tokenized)?;
    let expanded = brack_expander::expand::expander(&parsed, &mut plugins)?;
    let gen = brack_codegen::generate::generate(&expanded, &mut plugins)?;
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

fn build() -> Result<()> {
    if !Path::new("Brack.toml").exists() {
        anyhow::bail!("Brack.toml is not found.");
    }

    let config: brack_plugin_manager::add_plugin::Config =
        toml::from_str(&std::fs::read_to_string("Brack.toml")?)?;
    let backend = config.document.backend;

    let entries = read_dir("plugins")?;
    let mut pathes = vec![];
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Could not get file name from path."))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Could not convert file name to string."))?;
        if name.ends_with(format!(".{}.wasm", backend).as_str()) {
            pathes.push(path);
        }
    }

    let mut plugins = brack_plugin::plugin::new_plugins2(pathes)?;

    let entries = read_dir("docs")?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Could not get file name from path."))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Could not convert file name to string."))?;
        if name.ends_with(".[]") {
            let tokenized = brack_tokenizer::tokenize::tokenize(&path.to_str().unwrap())?;
            let parsed = brack_parser::parse::parse(&tokenized)?;
            let expanded = brack_expander::expand::expander(&parsed, &mut plugins)?;
            let gen = brack_codegen::generate::generate(&expanded, &mut plugins)?;
            std::fs::create_dir_all("out")?;
            std::fs::write(
                format!("out/{}.{}", name.trim_end_matches(".[]"), backend),
                gen,
            )?;
        }
    }

    println!("Build succeeded.");
    for out in std::fs::read_dir("out")? {
        let out = out?;
        let path = out.path();
        let name = path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Could not get file name from path."))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Could not convert file name to string."))?;
        println!("  - ./out/{}", name);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.subcommand {
        SubCommands::Build => build()?,
        SubCommands::Compile { .. } => run_compile(args.subcommand)?,
        SubCommands::LanguageServer => brack_language_server::server::run().await?,
        SubCommands::New { name } => new_project(&name)?,
        SubCommands::Add { schema } => {
            brack_plugin_manager::add_plugin::add_plugin(&schema).await?
        }
    }
    Ok(())
}
