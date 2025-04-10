use std::{collections::HashMap, path::Path};

use brack_common::{
    errors::{ProjectDebug, ProjectError, ProjectInfo},
    logger::Logger,
};
use serde::{Deserialize, Serialize};

use crate::{
    manifest::License,
    utils::{create_dir, toml_to_string, write, DEFAULT_PROJECT_TARGET_PATH},
};

pub struct ChannelProvider {
    name: String,
    url: String,
}

pub struct Channels {
    channels: HashMap<String, ChannelProvider>,
}

impl Channels {
    pub fn new() -> Self {
        Channels {
            channels: HashMap::new(),
        }
    }

    pub fn ensure_channel_not_exists(&self, name: &str) -> Result<(), ProjectError> {
        if self.channels.contains_key(name) {
            return Err(ProjectError::ChannelAlreadyExists {
                name: name.to_string(),
            });
        }
        Ok(())
    }
}

pub type Channel = HashMap<String, PluginProvider>;

#[derive(Serialize, Deserialize, Clone)]
pub struct PluginProvider {
    name: String,
    url: String,
    method: PluginFetchMethod,
    licenses: Vec<License>,
    targets: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PluginFetchMethod {
    Git,
}

pub fn check_valid_channel_name(name: &str) -> Result<(), ProjectError> {
    if name.is_empty() {
        return Err(ProjectError::InvalidChannelName {
            name: name.to_string(),
        });
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ProjectError::InvalidChannelName {
            name: name.to_string(),
        });
    }
    Ok(())
}

pub fn check_valid_channel_url(url: &str) -> Result<(), ProjectError> {
    if url.is_empty() {
        return Err(ProjectError::InvalidChannelUrl {
            url: url.to_string(),
        });
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(ProjectError::InvalidChannelUrl {
            url: url.to_string(),
        });
    }
    Ok(())
}

impl ChannelProvider {
    pub fn new(name: &str, url: &str) -> Result<Self, ProjectError> {
        check_valid_channel_name(name)?;
        check_valid_channel_url(url)?;
        Ok(ChannelProvider {
            name: name.to_string(),
            url: url.to_string(),
        })
    }

    pub async fn fetch<L: Logger>(&self, logger: &L) -> Result<Channel, ProjectError> {
        logger.info(
            &ProjectInfo::FetchingChannel {
                name: self.name.clone(),
                url: self.url.clone(),
            }
            .into(),
        );
        let response = reqwest::get(self.url.clone()).await.map_err(|source| {
            ProjectError::FailedToFetchChannel {
                url: self.url.clone(),
                source,
            }
        })?;
        logger.debug(
            &ProjectDebug::ReceivedResponse {
                url: self.url.clone(),
            }
            .into(),
        );
        if !response.status().is_success() {
            let status = response.status();
            let reason = status.canonical_reason().unwrap_or("Unknown reqwest error");
            return Err(ProjectError::ReceivedHttpNonSuccessStatus {
                url: self.url.clone(),
                status: status.as_u16(),
                reason: reason.to_string(),
            });
        }
        let downloaded_data = response.bytes().await.map_err(|source| {
            ProjectError::FailedToConvertResponseToBytes {
                url: self.url.clone(),
                source,
            }
        })?;
        logger.debug(
            &ProjectDebug::ConvertingResponseToBytes {
                url: self.url.clone(),
            }
            .into(),
        );
        let downloaded_data_str = std::str::from_utf8(&downloaded_data)
            .map_err(|source| ProjectError::FailedToConvertBytesToStr { source })?;
        logger.debug(
            &ProjectDebug::ConvertingBytesToStr {
                url: self.url.clone(),
            }
            .into(),
        );
        let channel: Channel = toml::from_str(downloaded_data_str)
            .map_err(|source| ProjectError::FailedToDeserializeToChannel { source })?;
        logger.debug(&ProjectDebug::DeserializingToChannel.into());
        logger.info(
            &ProjectInfo::FinishedFetchingChannel {
                name: self.name.clone(),
            }
            .into(),
        );
        Ok(channel)
    }

    pub fn write<P: AsRef<Path>, L: Logger>(
        &self,
        project_root: &P,
        channel: &Channel,
        logger: &L,
    ) -> Result<(), ProjectError> {
        let path = project_root
            .as_ref()
            .join(DEFAULT_PROJECT_TARGET_PATH)
            .join(".channels")
            .join(&self.name);
        create_dir(&path, logger)?;
        let path = path.join("Channel.toml");
        if path.exists() {
            return Err(ProjectError::ChannelAlreadyExists {
                name: self.name.clone(),
            });
        }
        let content = toml_to_string(channel, logger)?;
        write(&path, &content, logger)?;
        Ok(())
    }
}

// fn get_brack_progress_bar() -> ProgressBar {
//     let pb = ProgressBar::new(0);
//     pb.set_style(
//         ProgressStyle::default_bar()
//             .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>3}/{len:3} {msg}")
//             .unwrap()
//             .progress_chars("=>-"),
//     );
//     pb
// }

// async fn download_plugin_from_channel(channel: Channel, version: String) -> Result<()> {
//     Ok(())
// }

// async fn download_plugin_from_local(
//     path: String,
//     feature_flags: Option<Vec<String>>,
// ) -> Result<()> {
//     Ok(())
// }

// async fn download_plugin_from_github(owner: String, name: String, tag: String) -> Result<()> {
//     let url = format!(
//         "https://github.com/{}/{}/releases/download/{}/{}.tar.gz",
//         owner, name, tag, name
//     );
//     let pb = get_brack_progress_bar();
//     pb.set_message(format!("Downloading plugin {}", name));
//     let response = reqwest::get(&url).await?;
//     if !response.status().is_success() {
//         pb.finish_and_clear();
//         anyhow::bail!(
//             "Failed to download plugin from {}.\nStatus: {} - {}",
//             url,
//             response.status().as_str(),
//             response
//                 .status()
//                 .canonical_reason()
//                 .unwrap_or("Unknown error")
//         );
//     }
//     if let Some(size) = response.content_length() {
//         pb.set_length(size);
//     }
//     let mut stream = response.bytes_stream();
//     let mut downloaded_data = Vec::new();
//     while let Some(chunk) = stream.next().await {
//         let chunk = chunk?;
//         pb.inc(chunk.len() as u64);
//         downloaded_data.extend_from_slice(&chunk);
//     }
//     pb.finish_with_message(format!("Downloaded plugin {}", name));
//     let mut tar = tar::Archive::new(std::io::Cursor::new(downloaded_data));
//     tar.unpack("plugins")?;
//     Ok(())
// }

// async fn download_plugins(manifest: &Manifest, channels: &Channels) -> Result<Plugins> {
//     let mut dependencies = vec![];
//     if let Some(deps) = manifest.dependencies.clone() {
//         for (name, dep) in deps {
//             match dep {
//                 Dependency::Version(version) => {}
//                 Dependency::Channel {
//                     channel,
//                     version,
//                     feature_flags,
//                 } => {
//                     let channel_name = match channel {
//                         Some(channel) => channel,
//                         None => String::from("default"),
//                     };
//                     let channel = channels
//                         .get(&channel_name)
//                         .ok_or_else(|| anyhow::anyhow!("Channel {} not found", channel_name))?;
//                     let channel = channel.clone();
//                     download_plugin_from_channel(channel, version).await?
//                 }
//                 Dependency::GitHub {
//                     owner,
//                     name,
//                     tag,
//                     feature_flags,
//                 } => download_plugin_from_github(owner, name, tag).await?,
//                 Dependency::Local {
//                     path,
//                     feature_flags,
//                 } => download_plugin_from_local(path, feature_flags).await?,
//             }
//         }
//     }
//     Plugins::new(dependencies)
// }
