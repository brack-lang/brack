use std::fs::read_dir;

use anyhow::Result;
use brack::{codegen::generate, parser::parse, plugins::new_plugins, tokenizer::tokenize};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    plugins_dir_path: String,

    #[arg(short, long)]
    backend: String,

    #[arg(short, long)]
    filename: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut pathes = vec![];

    let entries = read_dir(args.plugins_dir_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!(""))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!(""))?;
        if name.ends_with(format!(".{}.wasm", args.backend).as_str()) {
            pathes.push(path);
        }
    }

    let mut plugins = new_plugins(pathes)?;

    if !args.filename.ends_with(".[]") {
        anyhow::bail!("Filename must end with .[]");
    }

    let code = std::fs::read_to_string(args.filename)?;
    let tokenized = tokenize(&code);
    let parsed = parse(&tokenized)?;
    let gen = generate(&parsed, &mut plugins)?;
    println!("{}", gen);
    Ok(())
}
