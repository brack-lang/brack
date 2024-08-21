use crate::config::Config;
use crate::plugin::Plugin;
use anyhow::Result;
use bytes::Bytes;
use futures::future::join_all;
use reqwest;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::task::{self, JoinHandle};

pub struct Project {
    pub config: Config,
    pub plugins_metadata: HashMap<String, PathBuf>,
}

impl Project {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            plugins_metadata: Default::default(),
        }
    }

    pub fn update_config_from_brack_toml(&mut self) -> Result<()> {
        let config: Config = toml::from_str(&std::fs::read_to_string("Brack.toml")?)?;
        self.config = config;
        Ok(())
    }

    pub fn clear_plugins(&mut self) -> Result<()> {
        std::fs::remove_dir_all("plugins")?;
        std::fs::create_dir("plugins")?;
        self.plugins_metadata = Default::default();
        Ok(())
    }

    pub async fn download_plugins_using_config(&mut self) -> Result<()> {
        if let Some(plugins) = self.config.plugins.clone() {
            let mut tasks = vec![];
            for (name, plugin) in plugins {
                let path = PathBuf::from(&format!("plugins/{}.wasm", plugin.hash_sha256()));
                if path.exists() {
                    self.plugins_metadata.insert(name, path.into());
                    continue;
                }
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
                        let task: JoinHandle<Result<(String, PathBuf, Bytes)>> =
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
                                Ok((name, path, bytes))
                            });
                        tasks.push(task);
                    }
                }
            }

            let results = join_all(tasks).await;
            for result in results {
                let (name, path, bytes) = result??;
                std::fs::write(&path, &bytes)?;
                self.plugins_metadata.insert(name, Path::new(&path).into());
            }
        }

        Ok(())
    }
}
