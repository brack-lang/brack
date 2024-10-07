use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SubCommands {
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
    Version,
}
