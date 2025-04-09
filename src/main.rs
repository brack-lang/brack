use clap::Parser;
use cli::{Cli, SubCommands};
use std::env;

mod cli;
mod logger;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let raw_command = env::args().collect::<Vec<_>>().join(" ");
    match cli.subcommand.clone() {
        SubCommands::Create {
            path,
            plugin,
            document,
        } => cli.create(raw_command, path, plugin, document),
        SubCommands::Build => cli.build(raw_command).await,
        SubCommands::Clean { dry_run } => cli.clean(raw_command, dry_run),
        SubCommands::Plugin(plugin) => cli.plugin(&plugin),
        SubCommands::Channel(channel) => cli.channel(raw_command, &channel).await,
        SubCommands::LanguageServer => cli.language_server(),
    }
}
