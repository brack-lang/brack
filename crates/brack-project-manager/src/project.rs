use crate::config::Config;
use crate::plugin::PluginSchema;
use anyhow::Result;
use brack_plugin::{feature_flag::FeatureFlag, plugin::Plugin, plugins::Plugins};
use bytes::Bytes;
use futures::future::join_all;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::task::{self, JoinHandle};

#[derive(Debug)]
pub struct Project {
    pub config: Config,
    pub plugins_metadata: HashMap<String, (PathBuf, FeatureFlag)>,
    pub root: PathBuf,
}

fn get_task_download_plugin_from_github(
    owner: &str,
    repo: &str,
    version: &str,
    name: &str,
    backend: &str,
    dest_path: PathBuf,
    flag: FeatureFlag,
) -> JoinHandle<Result<(String, PathBuf, Bytes, FeatureFlag)>> {
    let url = format!(
        "https://github.com/{}/{}/releases/download/{}/{}.{}.wasm",
        owner, repo, version, name, backend
    );
    let name = String::from(name);
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>3}/{len:3} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message(format!("Downloading {}", name));
    let task: JoinHandle<Result<(String, PathBuf, Bytes, FeatureFlag)>> = task::spawn(async move {
        let response = reqwest::get(&url).await?;
        if !response.status().is_success() {
            pb.finish_and_clear();
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
        if let Some(size) = response.content_length() {
            pb.set_length(size);
        }

        let mut stream = response.bytes_stream();
        let mut downloaded_data = Vec::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            pb.inc(chunk.len() as u64);
            downloaded_data.extend_from_slice(&chunk);
            // sleep
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }

        pb.finish_with_message(format!("Downloaded {}", name));
        Ok((name, dest_path, Bytes::from(downloaded_data), flag))
    });
    task
}

fn get_task_download_plugin_from_local(
    path: &str,
    dest_path: PathBuf,
) -> JoinHandle<Result<(String, PathBuf, Bytes, FeatureFlag)>> {
    let path = String::from(path);
    let task: JoinHandle<Result<(String, PathBuf, Bytes, FeatureFlag)>> = task::spawn(async move {
        let bytes = std::fs::read(&path)?;
        Ok((path, dest_path, bytes.into(), FeatureFlag::default()))
    });
    task
}

impl Project {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            config: Default::default(),
            plugins_metadata: Default::default(),
            root: path.as_ref().to_path_buf(),
        }
    }

    pub fn load_brack_toml(&mut self) -> Result<()> {
        let config: Config =
            toml::from_str(&std::fs::read_to_string(self.root.join("Brack.toml"))?)?;
        self.config = config;
        Ok(())
    }

    pub fn load_brack_toml_with_config(&mut self, config: Config) {
        self.config = config;
    }

    pub fn clear_plugins(&mut self) -> Result<()> {
        std::fs::remove_dir_all("plugins")?;
        std::fs::create_dir("plugins")?;
        self.plugins_metadata = Default::default();
        Ok(())
    }

    pub async fn download_plugins_using_config(&mut self) -> Result<()> {
        let plugins = match self.config.plugins.clone() {
            Some(plugins) => plugins,
            None => return Ok(()),
        };
        std::fs::create_dir("plugins")?;
        let mut tasks = vec![];
        for (name, ref plugin) in plugins {
            let dest_path = PathBuf::from(&format!("plugins/{}.wasm", plugin.hash_sha256()));
            let hook = match plugin {
                PluginSchema::GitHub { hook, .. } => hook,
                PluginSchema::Local { hook, .. } => hook,
            };
            let flag = FeatureFlag {
                document_hook: hook.document.unwrap_or_default(),
                stmt_hook: hook.stmt.unwrap_or_default(),
                expr_hook: hook.expr.unwrap_or_default(),
                text_hook: hook.text.unwrap_or_default(),
            };
            if dest_path.exists() {
                self.plugins_metadata.insert(name, (dest_path, flag));
                continue;
            }
            match plugin {
                PluginSchema::GitHub {
                    owner,
                    repo,
                    version,
                    ..
                } => {
                    tasks.push(get_task_download_plugin_from_github(
                        owner,
                        repo,
                        version,
                        &name,
                        &self.config.document.backend,
                        dest_path,
                        flag,
                    ));
                }
                PluginSchema::Local { path, .. } => {
                    tasks.push(get_task_download_plugin_from_local(path, dest_path));
                }
            }
        }

        let results = join_all(tasks).await;
        for result in results {
            let (name, path, bytes, flag) = result??;
            std::fs::write(&path, &bytes)?;
            self.plugins_metadata.insert(name, (path, flag));
        }
        Ok(())
    }

    pub fn build(&self) -> Result<()> {
        let mut plugin_vec = vec![];
        for (name, (path, feature_flag)) in self.plugins_metadata.clone() {
            plugin_vec.push(Plugin::new(&name, path, feature_flag)?);
        }
        let mut plugins = Plugins::new(plugin_vec)?;

        let mut output_paths = vec![];
        let entries = walkdir::WalkDir::new("docs")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());
        for entry in entries {
            let path = entry.path();
            let relative_path = path.strip_prefix("docs")?;
            let file_stem = path
                .file_stem()
                .ok_or_else(|| anyhow::anyhow!("Could not get file name from path."))?
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Could not convert file name to string."))?;
            if path.extension() == Some("[]".as_ref()) {
                let tokenized = brack_tokenizer::tokenize::tokenize(path.to_str().unwrap())?;
                let parsed = brack_parser::parse::parse(&tokenized)?;
                let (ast, _errors) = brack_transformer::transform::transform(&parsed);
                let expanded = brack_expander::expand::expander(&ast, &mut plugins)?;
                let gen = brack_codegen::generate::generate(&expanded, &mut plugins)?;
                let output_dir = std::path::Path::new("out")
                    .join(relative_path.parent().unwrap_or(std::path::Path::new("")));
                std::fs::create_dir_all(&output_dir)?;
                let output_path =
                    output_dir.join(format!("{}.{}", file_stem, self.config.document.extension));
                std::fs::write(&output_path, gen)?;
                output_paths.push(output_path.to_string_lossy().to_string());
            }
        }

        println!("Build succeeded.");
        for output_path in output_paths {
            println!("  - {}", output_path);
        }
        Ok(())
    }
}
