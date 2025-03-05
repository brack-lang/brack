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

        /// 1: token, 2: cst, 3: ast, 4: expanded ast, 5: final output
        #[clap(long, default_value_t = 5)]
        output_level: u8,

        /// Output as JSON. This flag can not be used with output level 5.
        #[clap(long)]
        json: bool,
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
