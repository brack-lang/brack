use crate::config::Config;
use crate::plugin::Plugin;
use anyhow::Result;
use brack_plugin::plugin::PluginMetaDataMap;
use futures::future::join_all;
use reqwest;
use std::fs;
use tokio::task::{self, JoinHandle};

pub struct Project {
    pub config: Config,
    pub plugins_metadata: PluginMetaDataMap,
}

impl Project {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            plugins_metadata: Default::default(),
        }
    }

    pub fn update_config_from_brack_toml(&mut self) -> Result<()> {
        let config: Config = toml::from_str(&fs::read_to_string("Brack.toml")?)?;
        self.config = config;
        Ok(())
    }

    pub async fn download_plugins_using_config(&mut self) -> Result<()> {
        self.plugins_metadata = Default::default();
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
                    let path = format!("plugins/{}.wasm", plugin.hash_sha256());
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
            let path = result??;
            println!("Downloaded plugin to {}", path);
        }

        Ok(())
    }
}
