use crate::logger::CliLogLevel;
use crate::logger::Logger;
use anstyle::{AnsiColor, Color, Style};
use brack_project::project::Project;
use clap::{builder, ArgGroup, Parser, Subcommand};
use std::path::Path;
use std::process::exit;

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true
)]
#[command(styles=get_styles())]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommands,

    #[arg(long, value_enum, global = true, default_value_t = CliLogLevel::Info)]
    pub log_level: CliLogLevel,
}

#[derive(Subcommand, Clone)]
pub enum SubCommands {
    /// Create a new project
    #[command(group(
        ArgGroup::new("create_mode")
            .required(false)
            .args(&["plugin", "document"])
    ))]
    Create {
        /// Specify the project path, if not specified, the current directory is used
        path: String,

        /// Create a new plugin project
        #[clap(long, default_value_t = false)]
        plugin: bool,

        /// Create a new document project
        #[clap(long, default_value_t = false)]
        document: bool,
    },

    /// Build the project
    Build,

    /// Clean the project
    #[command(group(
        ArgGroup::new("clean_mode")
            .required(false)
            .args(&["plugins_only", "output_only"])
    ))]
    Clean {
        #[clap(short, long, default_value_t = false)]
        dry_run: bool,
    },

    /// Manage plugins
    #[clap(subcommand)]
    Plugin(Plugin),

    /// Manage channels
    #[clap(subcommand)]
    Channel(Channel),

    /// Start the language server
    LanguageServer,
}

#[derive(Subcommand, Clone)]
pub enum Plugin {
    /// Add a new plugin
    Add { schema: String },
    /// Remove a plugin
    Remove { name: String },
    /// Update a plugin, if no name is specified, all plugins are updated
    Update { name: Option<String> },
    /// List all plugins
    List,
}

#[derive(Subcommand, Clone)]
pub enum Channel {
    /// Add a new channel
    Add { name: String, url: String },
    /// Remove a channel
    Remove { name: String },
    /// Update a channel, if no name is specified, all channels are updated
    Update { name: Option<String> },
    /// List all channels
    List,
}

impl Cli {
    pub fn create<P: AsRef<Path>>(
        &self,
        raw_command: String,
        path: P,
        plugin: bool,
        _document: bool,
    ) {
        let mut project = Project::new();
        let logger = Logger {
            raw_command,
            cli_log_level: self.log_level.clone(),
            path: None,
        };
        if plugin {
            // project.create_plugin(path)?;
        } else {
            project
                .create_document(&logger, path)
                .map_err(|_| {
                    exit(1);
                })
                .unwrap();
        }
    }

    pub async fn build(self, raw_command: String) {
        let mut logger = Logger {
            raw_command,
            cli_log_level: self.log_level.clone(),
            path: None,
        };
        let project = Project::new_with_manifest(&logger, Path::new("."))
            .map_err(|_| {
                exit(1);
            })
            .unwrap();
        project
            .build(&mut logger)
            .await
            .map_err(|_| {
                exit(1);
            })
            .unwrap();
    }

    pub fn clean(self, raw_command: String, dry_run: bool) {
        let logger = Logger {
            raw_command,
            cli_log_level: self.log_level.clone(),
            path: None,
        };
        let project = Project::new_with_manifest(&logger, Path::new("."))
            .map_err(|_| {
                exit(1);
            })
            .unwrap();
        project
            .clean(&logger, dry_run)
            .map_err(|_| {
                exit(1);
            })
            .unwrap();
    }

    pub fn plugin(self, plugin: &Plugin) {
        match plugin {
            Plugin::Add { .. } => self.plugin_add(),
            Plugin::Remove { .. } => self.plugin_remove(),
            Plugin::Update { .. } => self.plugin_update(),
            Plugin::List { .. } => self.plugin_list(),
        }
    }

    fn plugin_add(self) {}

    fn plugin_remove(self) {}

    fn plugin_update(self) {}

    fn plugin_list(self) {}

    pub async fn channel(self, raw_command: String, channel: &Channel) {
        match channel {
            Channel::Add { name, url } => self.channel_add(raw_command, name, url).await,
            Channel::Remove { name } => self.channel_remove(raw_command, name),
            Channel::Update { name } => self.channel_update(raw_command, name.clone()).await,
            Channel::List => self.channel_list(raw_command).await,
        }
    }

    async fn channel_add(self, raw_command: String, name: &str, url: &str) {
        let logger = Logger {
            raw_command,
            cli_log_level: self.log_level.clone(),
            path: None,
        };
        let mut project = Project::new_with_manifest(&logger, Path::new("."))
            .map_err(|_| {
                exit(1);
            })
            .unwrap();
        project
            .add_channel(&logger, name, url)
            .await
            .map_err(|_| {
                exit(1);
            })
            .unwrap();
    }

    fn channel_remove(self, raw_command: String, name: &str) {
        let logger = Logger {
            raw_command,
            cli_log_level: self.log_level.clone(),
            path: None,
        };
        let mut project = Project::new_with_manifest(&logger, Path::new("."))
            .map_err(|_| {
                exit(1);
            })
            .unwrap();
        project
            .remove_channel(&logger, name)
            .map_err(|_| {
                exit(1);
            })
            .unwrap();
    }

    async fn channel_update(self, raw_command: String, name: Option<String>) {
        let logger = Logger {
            raw_command,
            cli_log_level: self.log_level.clone(),
            path: None,
        };
        let project = Project::new_with_manifest(&logger, Path::new("."))
            .map_err(|_| {
                exit(1);
            })
            .unwrap();
        project
            .update_channel(&logger, name)
            .await
            .map_err(|_| {
                exit(1);
            })
            .unwrap();
    }

    async fn channel_list(self, raw_command: String) {
        let logger = Logger {
            raw_command,
            cli_log_level: self.log_level.clone(),
            path: None,
        };
        let project = Project::new_with_manifest(&logger, Path::new("."))
            .map_err(|_| {
                exit(1);
            })
            .unwrap();
        project
            .list_channels(&logger)
            .await
            .map_err(|_| {
                exit(1);
            })
            .unwrap();
    }

    pub fn language_server(self) {}
}

fn get_styles() -> builder::Styles {
    builder::Styles::styled()
        .usage(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
        )
        .header(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
        )
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .invalid(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .error(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .valid(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::White))))
}
