use crate::config::Config;
use crate::plugin::Plugin;
use anyhow::Result;
use bytes::Bytes;
use futures::future::join_all;
use reqwest;
use std::{collections::HashMap, fs, hash::Hash, path::Path};
use tokio::task::{self, JoinHandle};

pub struct Project {
    pub config: Config,
    pub plugins_metadata: HashMap<String, Box<Path>>,
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

        if let Some(plugins) = self.config.plugins.clone() {
            let mut tasks = vec![];
            for (name, plugin) in plugins {
                let hash = plugin.hash_sha256();
                match plugin {
                    Plugin::GitHub {
                        owner,
                        repo,
                        version,
                    } => {
                        let url = format!(
                            "https://github.com/{}/{}/releases/download/{}/{}.{}.wasm",
                            owner, repo, version, name, self.config.document.backend
                        );
                        let task: JoinHandle<Result<(String, String, Bytes)>> =
                            task::spawn(async move {
                                let response = reqwest::get(&url).await?;
                                if !response.status().is_success() {
                                    anyhow::bail!(
                                        "Failed to download plugin from {}.\nStatus: {} - {}",
                                        url,
                                        response.status().as_str(),
                                        response
                                            .status()
                                            .canonical_reason()
                                            .unwrap_or("Unknown error")
                                    );
                                }
                                let bytes = response.bytes().await?;
                                Ok((name, hash, bytes))
                            });
                        tasks.push(task);
                    }
                }
            }

            let results = join_all(tasks).await;
            for result in results {
                let (name, hash, bytes) = result??;
                let path = format!("plugins/{}.wasm", hash);
                std::fs::write(&path, &bytes)?;
                self.plugins_metadata.insert(name, Path::new(&path).into());
            }
        }

        Ok(())
    }
}
