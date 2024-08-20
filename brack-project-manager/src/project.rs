use crate::add_plugin::{Config, Plugin};
use anyhow::Result;
use brack_plugin::plugin::Plugins;
use futures::future::join_all;
use reqwest;
use std::fs;
use std::path::Path;
use tokio::task::{self, JoinHandle};

pub struct Project {
    pub config: Config,
    pub plugins: Plugins,
}

impl Project {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            plugins: Default::default(),
        }
    }

    pub async fn load_brack_toml(&mut self) -> Result<()> {
        if !Path::new("Brack.toml").exists() {
            anyhow::bail!("Brack.toml is not found.");
        }

        self.config = toml::from_str(&std::fs::read_to_string("Brack.toml")?)?;
        self.plugins = Default::default();
        std::fs::remove_dir_all("plugins")?;
        std::fs::create_dir("plugins")?;

        let mut tasks = vec![];

        for (name, plugin) in self.config.plugins.as_ref().unwrap_or(&Default::default()) {
            match plugin {
                Plugin::GitHub {
                    owner,
                    repo,
                    version,
                } => {
                    let url = format!(
                        "https://github.com/{}/{}/releases/download/{}/{}.wasm",
                        owner, repo, version, name
                    );
                    let path = format!("plugins/{}.wasm", name);
                    let task: JoinHandle<Result<String>> = task::spawn(async move {
                        let response = reqwest::get(&url).await?;
                        let bytes = response.bytes().await?;
                        fs::write(&path, bytes)?;
                        Ok(path)
                    });
                    tasks.push(task);
                }
            }
        }

        let results = join_all(tasks).await;
        for result in results {
            result??;
        }

        Ok(())
    }
}
